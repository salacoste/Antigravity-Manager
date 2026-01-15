# EPIC-028: Model-Specific Rate Limiting

**Epic ID**: EPIC-028
**Priority**: 🔴 P0 CRITICAL
**Impact**: HIGH - 3-5x account utilization improvement
**Effort**: Medium (3-5 days)
**Risk**: Medium - Core TokenManager changes
**Status**: 📋 PLANNING
**Team**: Backend/Rust Team
**Created**: 2026-01-15
**Target Completion**: Week 2 (2026-01-26)

---

## 📊 Executive Summary

### Problem Statement

**Current Critical Issue**: Antigravity-Manager implements **account-level rate limiting**, meaning when a single model hits a 429 error, the entire account is blocked, even though other models on the same account may have abundant available quota.

**Business Impact**:
- Account utilization drops by 60-80% when multiple models are used
- Unnecessary account switching increases latency
- Reduced prompt cache hit rate due to account churn
- Poor user experience with "all accounts rate-limited" errors

### Solution Overview

Implement **model-specific rate limiting** to track rate limits per model per account, allowing continued use of available models on rate-limited accounts.

**Expected Benefits**:
- 3-5x improvement in account utilization efficiency
- Reduced 429 error rate from 15-20% to <5%
- Improved prompt cache hit rate (same account = better cache)
- Better user experience with fewer "exhausted" errors

---

## 🎯 Epic Objectives

### Primary Objectives

1. **Implement Model-Specific Rate Limit Tracking**
   - Add `model_rate_limits: DashMap<String, ModelRateLimit>` to `ProxyToken`
   - Track rate limits per model ID per account
   - Automatic expiration of rate limits

2. **Update Account Selection Logic**
   - Check availability per model, not per account
   - Continue using available models on rate-limited accounts
   - Smart wait-time decision logic (≤2min wait vs switch)

3. **Enhance Rate Limit Marking**
   - Mark specific models as rate-limited (not entire account)
   - Parse exact reset time from API responses
   - Support concurrent model usage on same account

4. **Maintain Backward Compatibility**
   - Preserve existing API contracts
   - No breaking changes for existing configurations
   - Graceful degradation for edge cases

---

## 🔍 Current Implementation Analysis

### Existing Code (src-tauri/src/proxy/token_manager.rs)

```rust
// CURRENT STRUCTURE - Global rate limiting
pub struct ProxyToken {
    pub account_id: String,
    pub access_token: String,
    pub rate_limit_until: Option<Instant>,  // ❌ Global for entire account
}

impl TokenManager {
    pub fn get_token(&self) -> Result<ProxyToken, String> {
        // Picks next account without checking model-specific limits
    }

    pub fn mark_rate_limited(&self, account_id: &str, duration: Duration) {
        // ❌ Blocks entire account
    }
}
```

### Problem Scenarios

**Scenario 1: Sequential Model Usage**
```
User Request 1: claude-sonnet-4-5-thinking → Account A (429 ❌)
User Request 2: gemini-3-flash → Account A (blocked ❌)
                                    ^^^ Should be AVAILABLE!
```

**Scenario 2: Concurrent Requests**
```
Request 1: gemini-3-flash on Account A (success ✅)
Request 2: claude-sonnet-4-5-thinking on Account A (429 ❌)
Result: Account A marked as rate_limited = true
Request 3: gemini-3-flash on Account A (blocked ❌)
                                    ^^^ Should be AVAILABLE!
```

**Scenario 3: Uneven Quota Usage**
```
Account A:
  - claude-sonnet-4-5-thinking: 0% remaining (429)
  - gemini-3-flash: 100% remaining
  - gemini-3-pro-high: 85% remaining

Current behavior: Account A blocked entirely ❌
Desired behavior: Only sonnet blocked, others available ✅
```

---

## 🏗️ Technical Specification

### New Data Structures

#### 1. Enhanced ProxyToken

```rust
use dashmap::DashMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ModelRateLimit {
    pub is_rate_limited: bool,
    pub reset_time_ms: Option<u64>,  // Unix timestamp
    pub reset_ms: u64,                // Duration until reset
    pub last_429_at: Option<Instant>, // When the 429 occurred
}

pub struct ProxyToken {
    pub account_id: String,
    pub access_token: String,

    // ✅ NEW: Model-specific rate limits
    pub model_rate_limits: Arc<DashMap<String, ModelRateLimit>>,

    // Keep for backward compatibility
    pub rate_limit_until: Option<Instant>,  // Deprecated but preserved
}
```

#### 2. Rate Limit State

```rust
impl ModelRateLimit {
    pub fn new(reset_ms: u64) -> Self {
        Self {
            is_rate_limited: true,
            reset_time_ms: Some(chrono::Utc::now().timestamp_millis() + reset_ms as u64),
            reset_ms,
            last_429_at: Some(Instant::now()),
        }
    }

    pub fn is_expired(&self) -> bool {
        if !self.is_rate_limited {
            return true;  // Not rate limited = available
        }

        if let Some(reset_time) = self.reset_time_ms {
            let now = chrono::Utc::now().timestamp_millis();
            return now >= reset_time;
        }

        false
    }

    pub fn get_wait_ms(&self) -> Option<u64> {
        if let Some(reset_time) = self.reset_time_ms {
            let now = chrono::Utc::now().timestamp_millis();
            if reset_time > now {
                return Some((reset_time - now) as u64);
            }
        }
        None
    }
}
```

### Core API Changes

#### 3. Enhanced TokenManager

```rust
impl TokenManager {
    // ✅ NEW: Check availability for specific model
    pub fn is_account_available_for_model(
        &self,
        account_id: &str,
        model_id: &str,
    ) -> bool {
        if let Some(token) = self.get_token_by_id(account_id) {
            // Check model-specific limit
            if let Some(limit) = token.model_rate_limits.get(model_id) {
                if !limit.is_expired() {
                    return false;  // Still rate-limited for this model
                }
            }
            return true;  // Available for this model
        }
        false
    }

    // ✅ NEW: Get available accounts for specific model
    pub fn get_available_accounts_for_model(
        &self,
        model_id: &str,
    ) -> Vec<ProxyToken> {
        self.tokens
            .iter()
            .filter(|entry| {
                let (_, token) = entry.pair();
                self.is_account_available_for_model(&token.account_id, model_id)
            })
            .map(|entry| entry.value().clone())
            .collect()
    }

    // ✅ NEW: Mark specific model as rate-limited
    pub fn mark_model_rate_limited(
        &self,
        account_id: &str,
        model_id: &str,
        reset_ms: u64,
    ) {
        if let Some(token) = self.get_token_by_id(account_id) {
            let limit = ModelRateLimit::new(reset_ms);
            token.model_rate_limits.insert(model_id.to_string(), limit);

            tracing::warn!(
                "[TokenManager] Model-specific rate limit: {} (model: {}) - reset in {}",
                account_id,
                model_id,
                format_duration(reset_ms)
            );
        }
    }

    // ✅ NEW: Clear expired rate limits (cleanup task)
    pub fn clear_expired_model_limits(&self) -> usize {
        let mut cleared = 0;

        for token in self.tokens.iter() {
            let mut limits_to_remove = Vec::new();

            for entry in token.model_rate_limits.iter() {
                let (model_id, limit) = entry.pair();
                if limit.is_expired() {
                    limits_to_remove.push(model_id.clone());
                }
            }

            for model_id in limits_to_remove {
                token.model_rate_limits.remove(&model_id);
                cleared += 1;
            }
        }

        if cleared > 0 {
            tracing::info!("[TokenManager] Cleared {} expired model rate limits", cleared);
        }

        cleared
    }

    // ✅ ENHANCED: Get token with model awareness
    pub fn get_token_for_model(
        &self,
        model_id: &str,
    ) -> Result<ProxyToken, String> {
        // Try current sticky account first
        if let Some(token) = self.try_current_sticky_for_model(model_id)? {
            return Ok(token);
        }

        // Get available accounts for this specific model
        let available = self.get_available_accounts_for_model(model_id);

        if available.is_empty() {
            // Check if we should wait for reset
            if let Some(wait_info) = self.should_wait_for_model(model_id) {
                const MAX_WAIT_MS: u64 = 120_000; // 2 minutes

                if wait_info.wait_ms <= MAX_WAIT_MS {
                    tracing::info!(
                        "[TokenManager] All accounts rate-limited for {}, waiting {}ms",
                        model_id,
                        wait_info.wait_ms
                    );
                    return Err(format!("All accounts rate-limited. Wait {} or use different model.",
                        format_duration(wait_info.wait_ms)));
                }
            }

            return Err("All accounts rate-limited for this model".to_string());
        }

        // Pick next available account
        Ok(available[0].clone())
    }

    // ✅ NEW: Check if we should wait for rate limit reset
    fn should_wait_for_model(&self, model_id: &str) -> Option<WaitInfo> {
        let mut min_wait_ms = None;

        for token in self.tokens.iter() {
            if let Some(limit) = token.model_rate_limits.get(model_id) {
                if let Some(wait_ms) = limit.get_wait_ms() {
                    match min_wait_ms {
                        None => min_wait_ms = Some(wait_ms),
                        Some(current) if wait_ms < current => min_wait_ms = Some(wait_ms),
                        _ => {}
                    }
                }
            }
        }

        min_wait_ms.map(|wait_ms| WaitInfo { wait_ms })
    }
}

#[derive(Debug, Clone)]
pub struct WaitInfo {
    pub wait_ms: u64,
}
```

---

## 📋 Implementation Plan

### Phase 1: Data Structure Updates (Day 1)

**Tasks**:
1. Add `ModelRateLimit` struct to `src-tauri/src/proxy/token_manager.rs`
2. Update `ProxyToken` struct with `model_rate_limits: DashMap`
3. Implement `ModelRateLimit` methods (is_expired, get_wait_ms)
4. Update `TokenManager::new()` to initialize DashMap
5. Add backward compatibility layer for `rate_limit_until`

**Acceptance Criteria**:
- ✅ Code compiles without errors
- ✅ All existing tests pass
- ✅ New structures are thread-safe (Arc + DashMap)
- ✅ Backward compatibility preserved

**Estimated Time**: 4-6 hours

---

### Phase 2: Core Logic Implementation (Day 2-3)

**Tasks**:

#### 2.1 Model Availability Checking
```rust
// Implement in src-tauri/src/proxy/token_manager.rs

pub fn is_account_available_for_model(&self, account_id: &str, model_id: &str) -> bool
pub fn get_available_accounts_for_model(&self, model_id: &str) -> Vec<ProxyToken>
```

#### 2.2 Model-Specific Rate Limit Marking
```rust
// Implement in src-tauri/src/proxy/token_manager.rs

pub fn mark_model_rate_limited(&self, account_id: &str, model_id: &str, reset_ms: u64)
```

#### 2.3 Enhanced Token Selection
```rust
// Update in src-tauri/src/proxy/token_manager.rs

pub fn get_token_for_model(&self, model_id: &str) -> Result<ProxyToken, String>
```

**Acceptance Criteria**:
- ✅ Model availability check works correctly
- ✅ Rate limit marking is model-specific
- ✅ Token selection prefers available models
- ✅ Unit tests cover all new functions

**Estimated Time**: 8-12 hours

---

### Phase 3: Handler Integration (Day 3-4)

**Tasks**:

#### 3.1 Update Claude Handler
```rust
// File: src-tauri/src/proxy/handlers/claude.rs

// In request processing loop:
loop {
    let token = match token_manager.get_token_for_model(&model) {
        Ok(t) => t,
        Err(e) => {
            // Check if we should wait
            if e.contains("Wait") {
                // Extract wait time and return error to client
                return Err(e);
            }
            continue;  // Try next model/account
        }
    };

    match make_request(token, &request).await {
        Ok(response) => break,
        Err(ApiError::RateLimitError(reset_ms)) => {
            // ✅ Mark specific model as rate-limited
            token_manager.mark_model_rate_limited(&token.account_id, &model, reset_ms);
            continue;
        }
        Err(e) => return Err(e),
    }
}
```

#### 3.2 Update OpenAI Handler
```rust
// File: src-tauri/src/proxy/handlers/openai.rs

// Same pattern as Claude handler
```

#### 3.3 Update Gemini Handler
```rust
// File: src-tauri/src/proxy/handlers/gemini.rs

// Same pattern as Claude handler
```

**Acceptance Criteria**:
- ✅ All handlers use model-specific rate limiting
- ✅ 429 errors mark specific model, not entire account
- ✅ Requests for available models succeed on rate-limited accounts
- ✅ Integration tests pass

**Estimated Time**: 6-8 hours

---

### Phase 4: Cleanup and Optimization (Day 4)

**Tasks**:

#### 4.1 Expired Rate Limit Cleanup
```rust
// Implement background task in src-tauri/src/proxy/server.rs

// Spawn cleanup task
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        token_manager.clear_expired_model_limits();
    }
});
```

#### 4.2 Monitoring and Logging
```rust
// Add structured logging

tracing::info!(
    model = %model_id,
    account = %account_id,
    rate_limited_models = ?limited_models,
    available_models = ?available_models,
    "[TokenManager] Account status"
);
```

#### 4.3 Metrics Collection
```rust
// Track model-specific rate limit statistics

pub struct RateLimitMetrics {
    pub total_429s: AtomicU64,
    pub model_specific_429s: DashMap<String, AtomicU64>,
    pub avoided_blockades: AtomicU64,  // Requests that succeeded due to model-specific tracking
}
```

**Acceptance Criteria**:
- ✅ Expired limits auto-clear every 60 seconds
- ✅ Structured logging provides visibility
- ✅ Metrics show improvement (avoided_blockades counter)
- ✅ No memory leaks from DashMap

**Estimated Time**: 4-6 hours

---

### Phase 5: Testing (Day 5)

**Tasks**:

#### 5.1 Unit Tests
```rust
// File: src-tauri/src/proxy/token_manager_tests.rs

#[tokio::test]
async fn test_model_specific_rate_limiting() {
    // Test 1: Mark one model as rate-limited
    // Test 2: Verify other models still available
    // Test 3: Verify expiration
    // Test 4: Verify cleanup
}

#[tokio::test]
async fn test_concurrent_model_usage() {
    // Test concurrent requests for different models
    // Verify no race conditions
}

#[tokio::test]
async fn test_wait_time_logic() {
    // Test wait time calculation
    // Test MAX_WAIT_MS threshold
}
```

#### 5.2 Integration Tests
```rust
// File: tests/integration/model_rate_limiting.rs

#[tokio::test]
async fn test_claude_429_doesnt_block_gemini() {
    // 1. Make Claude request → force 429
    // 2. Immediately make Gemini request
    // 3. Verify Gemini succeeds
}

#[tokio::test]
async fn test_multiple_accounts_model_limits() {
    // Test across multiple accounts
    // Verify proper load balancing
}
```

**Acceptance Criteria**:
- ✅ All unit tests pass (100%)
- ✅ All integration tests pass (100%)
- ✅ Code coverage ≥80% for new code
- ✅ No regression in existing tests

**Estimated Time**: 6-8 hours

---

## ✅ Acceptance Criteria

### Functional Requirements

- [ ] **FR1**: System tracks rate limits per model per account
- [ ] **FR2**: When Model A hits 429, Model B on same account remains available
- [ ] **FR3**: Rate limit marking is model-specific, not account-wide
- [ ] **FR4**: Expired rate limits auto-clear within 60 seconds
- [ ] **FR5**: Wait time logic considers ≤2min threshold

### Non-Functional Requirements

- [ ] **NFR1**: Thread-safe concurrent access (Arc + DashMap)
- [ ] **NFR2**: Memory usage increases <20% per account
- [ ] **NFR3**: Latency impact <5ms per request
- [ ] **NFR4**: Backward compatibility with existing config
- [ ] **NFR5**: No breaking changes to API contracts

### Quality Requirements

- [ ] **QR1**: Unit test coverage ≥80% for new code
- [ ] **QR2**: Integration tests cover main scenarios
- [ ] **QR3**: Code follows Rust best practices
- [ ] **QR4**: Documentation updated (CLAUDE.md, comments)
- [ ] **QR5**: All existing tests still pass

---

## 🎯 Success Metrics

### Quantitative Metrics

| Metric | Before | After | Target | Measurement |
|--------|--------|-------|--------|-------------|
| **Account Utilization** | 40-60% | 80-95% | +2-3x | Request success rate per account |
| **429 Error Rate** | 15-20% | <5% | -75% | 429 errors / total requests |
| **Concurrent Model Success** | N/A | 100% | ✅ | Gemini succeeds when Claude 429 |
| **Avg 429 Recovery Time** | 60s | ≤10s | -83% | Time to successful retry |
| **Prompt Cache Hit Rate** | 30-40% | 50-60% | +50% | cache_read_input_tokens |

### Qualitative Metrics

- **Stability**: No account-wide false positives
- **Efficiency**: Better use of available quota
- **User Experience**: Fewer "all accounts exhausted" errors
- **Observability**: Clear logging of model-specific limits

---

## ⚠️ Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **DashMap memory leaks** | Low | High | Periodic cleanup task, tests for memory |
| **Race conditions** | Medium | High | Arc + DashMap for thread-safety, extensive testing |
| **Backward compatibility** | Low | Medium | Preserve old fields, gradual migration |
| **Performance degradation** | Low | Medium | Benchmark before/after, optimize hot paths |

### Operational Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Extended downtime during deployment** | Low | High | Blue-green deployment, quick rollback |
| **Account ban rate increase** | Low | High | Gradual rollout (10% → 50% → 100%) |
| **Configuration complexity** | Medium | Medium | Clear defaults, documentation |

---

## 📊 Implementation Timeline

### Week 1 (Jan 15-19)

```
Day 1 (Mon): Data structures + backward compatibility
Day 2 (Tue): Core logic implementation
Day 3 (Wed): Handler integration (Claude)
Day 4 (Thu): Handler integration (OpenAI + Gemini) + cleanup
Day 5 (Fri): Testing + validation
```

### Week 2 (Jan 22-26)

```
Day 1 (Mon): Code review + refinements
Day 2 (Tue): Integration testing
Day 3 (Wed): Performance testing
Day 4 (Thu): Documentation updates
Day 5 (Fri): Deployment preparation + staging tests
```

---

## 🔄 Rollout Strategy

### Phase 1: Canary (10% traffic)

**Duration**: 1 day
**Monitoring**:
- Model-specific 429 rate
- Account utilization per model
- Error rate comparison (before/after)

**Rollback Criteria**:
- Error rate increase >20%
- Latency increase >100ms
- Account ban rate spike

### Phase 2: Partial Rollout (50% traffic)

**Duration**: 2 days
**Monitoring**: Same as Phase 1

### Phase 3: Full Rollout (100% traffic)

**Duration**: Ongoing
**Monitoring**: Continuous

---

## 📚 Related Documentation

### Alternative Proxy Reference
- `alternative_proxy_app/docs/ARCHITECTURE_ANALYSIS.md` (Lines 99-114, 656-761)
- `alternative_proxy_app/docs/DEEP_TECHNICAL_ANALYSIS.md` (Lines 654-761)
- `alternative_proxy_app/src/account-manager/rate-limits.js` (Reference implementation)

### Current Implementation
- `src-tauri/src/proxy/token_manager.rs` (Main changes needed)
- `src-tauri/src/proxy/handlers/claude.rs` (Handler integration)
- `src-tauri/src/proxy/handlers/openai.rs` (Handler integration)
- `src-tauri/src/proxy/handlers/gemini.rs` (Handler integration)

---

## 🧪 Testing Strategy

### Unit Tests

**File**: `src-tauri/src/proxy/token_manager_tests.rs`

```rust
#[tokio::test]
async fn test_single_model_rate_limit() {
    let manager = TokenManager::new();

    // Add account
    manager.add_account(test_account());

    // Mark model as rate-limited
    manager.mark_model_rate_limited("test@example.com", "claude-sonnet-4-5-thinking", 60000);

    // Verify model is rate-limited
    assert!(!manager.is_account_available_for_model("test@example.com", "claude-sonnet-4-5-thinking"));

    // Verify other models are still available
    assert!(manager.is_account_available_for_model("test@example.com", "gemini-3-flash"));
}

#[tokio::test]
async fn test_rate_limit_expiration() {
    // Test automatic expiration after reset time
}

#[tokio::test]
async fn test_concurrent_model_requests() {
    // Test thread safety with concurrent requests
}
```

### Integration Tests

**File**: `tests/integration/model_specific_rate_limiting.rs`

```rust
#[tokio::test]
async fn test_claude_429_allows_gemini() {
    // Real account test
    // Force 429 on Claude model
    // Immediately request Gemini model
    // Verify Gemini succeeds
}
```

---

## 📝 Checklist

### Pre-Implementation

- [ ] Review this epic with tech lead
- [ ] Approve implementation timeline
- [ ] Allocate development resources
- [ ] Set up staging environment
- [ ] Prepare rollback plan

### Implementation (Days 1-5)

- [ ] Day 1: Data structures + backward compatibility
- [ ] Day 2: Core logic implementation
- [ ] Day 3: Handler integration
- [ ] Day 4: Cleanup + optimization
- [ ] Day 5: Testing + validation

### Pre-Deployment

- [ ] All tests pass (unit + integration)
- [ ] Code review approved
- [ ] Documentation updated
- [ ] Performance benchmarks acceptable
- [ ] Staging tests successful

### Deployment

- [ ] Canary deployment (10%)
- [ ] Monitor metrics for 24h
- [ ] Partial rollout (50%)
- [ ] Monitor metrics for 48h
- [ ] Full rollout (100%)
- [ ] Continuous monitoring

### Post-Deployment

- [ ] Monitor for 1 week
- [ ] Collect success metrics
- [ ] Address any issues
- [ ] Document lessons learned
- [ ] Update runbooks

---

## 🚀 Next Steps

After this epic is complete:

1. **EPIC-029**: Thinking Recovery Mechanism (builds on model-specific tracking)
2. **EPIC-030**: Signature Cache with Model Family (complements this epic)
3. **EPIC-031**: Enhanced Sticky Session (leverages model-specific availability)

These epics form a foundation for **cross-model conversation stability** and **intelligent account selection**.

---

**Author**: Amelia (Dev Agent)
**Date**: 2026-01-15
**Status**: Ready for Implementation
**Version**: 1.0
