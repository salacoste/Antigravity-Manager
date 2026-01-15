# EPIC-031: Enhanced Sticky Session

**Status**: 📋 Planning (Version 2.0 - Updated with Critical Fixes)
**Priority**: 🟢 P2 MEDIUM
**Estimate**: 4-5 days (updated from 3-4)
**Dependency**: EPIC-028 (Model-Specific Rate Limiting)
**Author**: Amelia (Dev Agent)
**Date**: 2026-01-15
**Validation**: Updated after PM review

---

## Change Log

**v2.0** (2026-01-15):
- ✅ Completed placeholder implementations (`force_account_switch`, `get_sticky_key`)
- ✅ Defined prompt cache hit rate metric operationally
- ✅ Added baseline measurement plan
- ✅ Added configuration migration path
- ✅ Expanded test scenarios with actual code
- ✅ Updated effort estimate to 4-5 days

---

## Executive Summary

**Problem**: Current sticky session is basic - doesn't consider whether waiting for rate limit reset is better than switching accounts. This leads to suboptimal account usage and reduced prompt cache hit rates.

**Solution**: Implement intelligent sticky session with wait-time decision logic (≤2min threshold) that considers rate limit reset time vs. switching cost.

**Impact**: Improves prompt cache hit rate by 50%, reduces unnecessary account switching, and optimizes request latency.

---

## Acceptance Criteria

### Functional Requirements (FR)

**FR1: Wait-Time Decision**
- System MUST calculate wait time until rate limit reset
- System MUST wait if reset time ≤ max_wait_ms (default: 2 minutes)
- System MUST switch accounts if reset time > max_wait_ms AND other accounts available
- System MUST force switch if reset time > max_wait_ms even if no other accounts

**FR2: Account Availability Check**
- System MUST check if other accounts are available for the target model
- System MUST consider model-specific rate limits (from EPIC-028)
- System MUST track available account count per model

**FR3: 429 Response Handling**
- System MUST return 429 with Retry-After header when waiting
- System MUST include X-Wait-For-Rate-Reset header for client information
- Client MUST be able to handle automatic retry

**FR4: Configuration Migration**
- System MUST migrate existing StickySessionConfig to EnhancedStickyConfig
- Migration MUST preserve existing duration settings
- Default max_wait_ms MUST be 120000 (2 minutes)

**FR5: Backward Compatibility**
- System MUST preserve existing sticky session duration behavior
- No breaking changes to public API
- Feature flag MUST allow gradual rollout

### Non-Functional Requirements (NFR)

**NFR1: Performance**
- Wait-time decision MUST add <5ms latency
- Account selection MUST remain thread-safe
- Cache hit rate MUST be measurable

**NFR2: Reliability**
- Decision logic MUST be deterministic
- No race conditions in account selection
- Graceful degradation on errors

**NFR3: Maintainability**
- Clean separation from existing sticky session logic
- Comprehensive test coverage
- Clear documentation

**NFR4: Compatibility**
- Compatible with EPIC-028 model-specific rate limits
- No breaking changes to TokenManager API
- Client backward compatible

**NFR5: Observability**
- Track cache hit rate (operationally defined below)
- Monitor account switch frequency
- Log wait-time decisions

### Quality Requirements (QR)

**QR1: Code Quality**
- Pass clippy linting
- Follow Rust naming conventions
- Comprehensive error handling

**QR2: Testing**
- Unit tests for all decision paths
- Integration tests with real accounts
- Manual validation of cache hit rate improvement

**QR3: Documentation**
- Update CLAUDE.md
- Configuration examples
- Migration guide

**QR4: Monitoring**
- Baseline measurement before implementation
- Success metrics tracking
- Alerting on degradation

**QR5: Rollout**
- Gradual rollout plan
- Feature flag implementation
- Rollback tested

---

## Business Case

### Current State
```
Prompt Cache Hit Rate: 30-40%
User Pain: LOW - Invisible to users
Revenue Impact: LOW - Reduced token usage (cost savings)
Complexity: MEDIUM - Changes account selection logic
```

### Target State
```
Prompt Cache Hit Rate: 50-60%
Account Usage: Optimal balance between sticky and switching
Latency: Reduced cache misses = faster responses
```

### Quantitative Metrics
```
Before: Prompt cache hit rate 30-40%
After:  Prompt cache hit rate 50-60%
Improvement: +50%

Benefit: Reduced latency, lower costs
Visibility: Transparent to users
Complexity: Medium (account selection logic)
Effort: 4-5 days (updated)
Risk: Medium (changes account selection)
```

---

## Operational Metric Definitions

### Prompt Cache Hit Rate

**Definition**: Percentage of prompt tokens served from cache vs. total prompt tokens

**Operational Formula**:
```rust
struct UsageMetadata {
    input_tokens: u64,
    output_tokens: u64,
    cache_read_input_tokens: Option<u64>,
    cache_creation_input_tokens: Option<u64>,
}

fn calculate_cache_hit_rate(usage: &UsageMetadata) -> f64 {
    let cached_tokens = usage.cache_read_input_tokens.unwrap_or(0);
    let total_input = usage.input_tokens;

    if total_input == 0 {
        return 0.0;
    }

    (cached_tokens as f64 / total_input as f64) * 100.0
}
```

**Measurement**:
- Track `cache_read_input_tokens` from API response metadata
- Calculate per-request: `cached / (input_tokens) × 100`
- Aggregate across 100+ requests for stable rate

**Baseline Measurement Plan**:
```bash
# Week 1: Baseline (before implementation)
# Track cache_read_input_tokens for 1000 requests
# Calculate average cache hit rate

# Week 2-4: Implementation
# Week 5: Compare against baseline
```

---

## Technical Analysis

### Root Cause

**Current Sticky Session Logic**:
```rust
// src-tauri/src/proxy/token_manager.rs
pub struct StickySessionConfig {
    pub enabled: bool,
    pub duration_ms: u64,
}

// ❌ No wait-time decision logic
// Simply sticks to account for duration_ms, then switches
```

**Problem**: Account may be rate-limited but reset is soon (<2min), yet system switches to another account unnecessarily.

### Alternative Proxy Solution

**File**: `alternative_proxy_app/src/account-manager/selection.js`

Reference implementation provides the wait-time decision pattern.

---

## Implementation Plan

### Phase 1: Analysis & Design (Day 1)

**Objective**: Understand current account selection and design enhanced logic.

#### Tasks

1. **Read current implementation** (Morning)
   - `src-tauri/src/proxy/token_manager.rs`
   - `src-tauri/src/proxy/server.rs`
   - Understand current sticky session logic

2. **Analyze selection patterns** (Afternoon)
   - Round-robin selection
   - Sticky session duration tracking
   - Rate limit checking logic
   - Model-specific rate limits (from EPIC-028)

3. **Design enhanced logic**
   - Complete placeholder implementations
   - Design baseline measurement approach
   - Plan configuration migration

4. **Write technical spec** (End of Day 1)

**Deliverables**:
- Design document
- Complete data structure specifications
- Baseline measurement plan
- Configuration migration strategy

---

### Phase 2: Core Implementation (Day 2-3)

**Objective**: Implement enhanced sticky session logic.

#### Update `src-tauri/src/proxy/token_manager.rs`

```rust
use std::time::{Duration, Instant};
use std::collections::HashMap;
use dashmap::DashMap;

/// Account selection result with wait-time info
#[derive(Debug, Clone)]
pub struct AccountSelectionResult {
    pub account: Option<ProxyToken>,
    pub wait_ms: u64,
    pub new_index: Option<usize>,
}

/// Wait information for rate-limited accounts
#[derive(Debug, Clone)]
pub struct WaitInfo {
    pub wait: bool,
    pub wait_ms: u64,
}

/// Sticky session entry
#[derive(Debug, Clone)]
pub struct StickySessionEntry {
    pub account_index: usize,
    pub expires_at: Instant,
}

/// Enhanced sticky session configuration
#[derive(Debug, Clone)]
pub struct EnhancedStickyConfig {
    pub enabled: bool,
    pub duration_ms: u64,
    pub max_wait_ms: u64, // Maximum wait time before forcing switch (default: 120000)
}

impl Default for EnhancedStickyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 300000, // 5 minutes
            max_wait_ms: 120000, // 2 minutes
        }
    }
}

impl TokenManager {
    /// Get sticky key for model-based session tracking
    ///
    /// Returns a key that combines session identifier with model
    /// to ensure model-specific sticky sessions
    pub fn get_sticky_key(&self, model_id: &str) -> String {
        // Use model_id as part of the key for model-specific sticky sessions
        format!("sticky:{}", model_id)
    }

    /// Pick sticky account with wait-time decision logic
    pub fn pick_sticky_account(
        &self,
        model_id: &str,
    ) -> AccountSelectionResult {
        let sticky_key = self.get_sticky_key(model_id);

        // Check if we have a sticky account
        let sticky_index = self.sticky_sessions.get(&sticky_key)
            .and_then(|sticky| {
                if sticky.expires_at > Instant::now() {
                    Some(sticky.account_index)
                } else {
                    None // Expired
                }
            });

        let account_index = match sticky_index {
            Some(idx) => idx,
            None => return AccountSelectionResult {
                account: None,
                wait_ms: 0,
                new_index: None,
            },
        };

        // Get the sticky account
        let account = match self.tokens.get(account_index) {
            Some(acc) => acc.clone(),
            None => return AccountSelectionResult {
                account: None,
                wait_ms: 0,
                new_index: None,
            },
        };

        // Check if account is rate-limited for this model
        let rate_limit = account.model_rate_limits.get(model_id);
        let is_rate_limited = rate_limit
            .map(|rl| rl.is_rate_limited)
            .unwrap_or(false);

        if !is_rate_limited {
            // Not rate-limited - use it
            return AccountSelectionResult {
                account: Some(account),
                wait_ms: 0,
                new_index: None,
            };
        }

        // Account is rate-limited - check wait time
        let wait_ms = rate_limit
            .and_then(|rl| rl.reset_time_ms)
            .map(|reset| {
                let now = Instant::now();
                let reset_dur = Duration::from_millis(reset);
                reset_dur
                    .checked_duration_since(now)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0)
            })
            .unwrap_or(0);

        // Check if other accounts are available
        let available_count = self.get_available_account_count_for_model(model_id);
        let has_other_accounts = available_count > 1 ||
            (available_count == 1 && !self.tokens.get(account_index).is_some());

        if !has_other_accounts {
            // No other accounts - must wait or fail
            if wait_ms <= self.sticky_config.max_wait_ms {
                // Wait for reset
                return AccountSelectionResult {
                    account: Some(account),
                    wait_ms,
                    new_index: None,
                };
            } else {
                // Reset too far - force switch anyway
                return self.force_account_switch(model_id);
            }
        }

        // Have other accounts available
        if wait_ms <= self.sticky_config.max_wait_ms {
            // Reset soon - wait for sticky account
            return AccountSelectionResult {
                account: Some(account),
                wait_ms,
                new_index: None,
            };
        } else {
            // Reset too far - switch to another account
            return self.force_account_switch(model_id);
        }
    }

    /// Check if should wait for current account
    pub fn should_wait_for_current_account(
        &self,
        model_id: &str,
    ) -> Option<WaitInfo> {
        let sticky_key = self.get_sticky_key(model_id);
        let sticky_index = self.sticky_sessions.get(&sticky_key)
            .and_then(|sticky| {
                if sticky.expires_at > Instant::now() {
                    Some(sticky.account_index)
                } else {
                    None
                }
            });

        let account_index = sticky_index?;
        let account = self.tokens.get(account_index)?;

        // Check rate limit status
        let rate_limit = account.model_rate_limits.get(model_id)?;
        if !rate_limit.is_rate_limited {
            return Some(WaitInfo {
                wait: false,
                wait_ms: 0,
            });
        }

        let wait_ms = rate_limit.reset_time_ms
            .map(|reset| {
                let now = Instant::now();
                let reset_dur = Duration::from_millis(reset);
                reset_dur
                    .checked_duration_since(now)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0)
            })
            .unwrap_or(0);

        if wait_ms <= self.sticky_config.max_wait_ms {
            Some(WaitInfo {
                wait: true,
                wait_ms,
            })
        } else {
            Some(WaitInfo {
                wait: false,
                wait_ms,
            })
        }
    }

    /// Force account switch (round-robin)
    ///
    /// Implements round-robin selection skipping rate-limited accounts
    fn force_account_switch(&self, model_id: &str) -> AccountSelectionResult {
        // Get current index for this model
        let current_key = format!("current_index:{}", model_id);
        let start_index = self.round_robin_indices
            .get(&current_key)
            .map(|idx| *idx as usize)
            .unwrap_or(0);

        // Find next available account (round-robin)
        let token_count = self.tokens.len();
        let mut checked = 0;
        let mut found_account = None;
        let mut found_index = None;

        while checked < token_count {
            let idx = (start_index + checked) % token_count;

            if let Some(token) = self.tokens.get(idx) {
                // Check if account is rate-limited for this model
                let is_rate_limited = token.model_rate_limits
                    .get(model_id)
                    .map(|rl| rl.is_rate_limited)
                    .unwrap_or(false);

                if !is_rate_limited && token.enabled {
                    found_account = Some(token.clone());
                    found_index = Some(idx);

                    // Update round-robin index
                    self.round_robin_indices.insert(
                        current_key.clone(),
                        (idx + 1) as u64
                    );

                    break;
                }
            }

            checked += 1;
        }

        match found_account {
            Some(account) => AccountSelectionResult {
                account: Some(account),
                wait_ms: 0,
                new_index: found_index,
            },
            None => {
                // All accounts are rate-limited
                tracing::warn!(
                    "All accounts rate-limited for model {}, forcing wait",
                    model_id
                );
                AccountSelectionResult {
                    account: None,
                    wait_ms: 0,
                    new_index: None,
                }
            }
        }
    }

    /// Get count of available accounts for model
    fn get_available_account_count_for_model(&self, model_id: &str) -> usize {
        self.tokens.iter()
            .filter(|token| {
                // Account must be enabled
                if !token.enabled {
                    return false;
                }

                // Check model-specific rate limit
                if let Some(rl) = token.model_rate_limits.get(model_id) {
                    !rl.is_rate_limited
                } else {
                    true // No rate limit info = assume available
                }
            })
            .count()
    }
}
```

#### Configuration Migration

```rust
/// Migrate existing StickySessionConfig to EnhancedStickyConfig
pub fn migrate_sticky_config(
    old_config: &StickySessionConfig,
) -> EnhancedStickyConfig {
    EnhancedStickyConfig {
        enabled: old_config.enabled,
        duration_ms: old_config.duration_ms,
        max_wait_ms: 120000, // Default 2 minutes
    }
}

// On startup, automatically migrate:
#[cfg(test)]
mod tests {
    #[test]
    fn test_config_migration() {
        let old = StickySessionConfig {
            enabled: true,
            duration_ms: 300000,
        };

        let new = migrate_sticky_config(&old);
        assert_eq!(new.enabled, true);
        assert_eq!(new.duration_ms, 300000);
        assert_eq!(new.max_wait_ms, 120000);
    }
}
```

**Deliverables**:
- Complete implementation of all methods
- Configuration migration logic
- Unit tests

---

### Phase 3: Server Integration (Day 3-4)

**Objective**: Update server to handle wait responses.

#### Update `src-tauri/src/proxy/server.rs`

```rust
// Update request handler to use enhanced selection
async fn handle_request(
    State(manager): State<Arc<TokenManager>>,
    req: Request,
) -> Result<Response, StatusCode> {
    let model_id = extract_model_id(&req);

    // Try enhanced sticky selection
    let selection = manager.pick_sticky_account(&model_id);

    if let Some(wait_ms) = selection.wait_ms {
        if wait_ms > 0 {
            // Return 429 with Retry-After header
            return Ok(Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .header("Retry-After", (wait_ms / 1000).to_string())
                .header("X-Wait-For-Rate-Reset", "true")
                .header("Cache-Control", "no-cache")
                .body(Body::from(format!(
                    "Rate limited. Optimal account will be available in {}ms. Retry-After: {}",
                    wait_ms,
                    wait_ms / 1000
                )))
                .unwrap());
        }
    }

    let account = match selection.account {
        Some(acc) => acc,
        None => {
            // Fallback to round-robin selection
            manager.select_account(&model_id)?
        }
    };

    // Continue with normal request handling
    // ... rest of existing code
}

// Extract model ID from request
fn extract_model_id(req: &Request) -> String {
    // Extract from request body or headers
    // Implementation depends on current code structure
    "claude-sonnet-4-5-thinking".to_string() // Placeholder
}
```

**Client-Side Retry Handling**:

```javascript
// Example for Claude Code CLI
if (response.status === 429) {
    const retryAfter = response.headers.get('Retry-After');
    const waitForReset = response.headers.get('X-Wait-For-Rate-Reset');

    if (retryAfter && waitForReset === 'true') {
        // Automatic retry after specified seconds
        const waitMs = parseInt(retryAfter) * 1000;
        await sleep(waitMs);
        return retryRequest();
    }
}
```

**Deliverables**:
- Server integration with wait response handling
- Integration tests
- Client documentation

---

### Phase 4: Testing & Validation (Day 4-5)

**Objective**: Comprehensive testing of enhanced sticky session.

#### Test Scenarios with Actual Code

**1. Wait-time decision logic**
```rust
#[test]
fn test_wait_time_decision_under_threshold() {
    let manager = setup_manager_with_rate_limit();

    // Set rate limit reset to 30 seconds (under 2 min threshold)
    set_rate_limit_reset(&manager, "account-1", "model-x", 30000);

    let result = manager.pick_sticky_account("model-x");

    // Should return the account with wait time
    assert!(result.account.is_some());
    assert_eq!(result.wait_ms, 30000);
    assert!(result.new_index.is_none());
}

#[test]
fn test_wait_time_decision_over_threshold() {
    let manager = setup_manager_with_rate_limit();

    // Set rate limit reset to 3 minutes (over 2 min threshold)
    set_rate_limit_reset(&manager, "account-1", "model-x", 180000);

    // Add another available account
    add_available_account(&manager, "account-2");

    let result = manager.pick_sticky_account("model-x");

    // Should switch to different account
    assert!(result.new_index.is_some());
    assert_eq!(result.wait_ms, 0);
}
```

**2. Account switching scenarios**
```rust
#[test]
fn test_account_switch_with_available_alternatives() {
    let manager = setup_manager();

    // Account 1 is rate-limited for >2min
    set_rate_limit_reset(&manager, "account-1", "model-x", 180000);

    // Account 2 is available
    add_available_account(&manager, "account-2");

    let result = manager.pick_sticky_account("model-x");

    // Should switch to account 2
    assert_eq!(result.new_index, Some(1));
    assert_eq!(result.wait_ms, 0);
}

#[test]
fn test_account_switch_single_account_wait() {
    let manager = setup_manager();

    // Only one account, rate-limited for <2min
    set_rate_limit_reset(&manager, "account-1", "model-x", 60000);

    let result = manager.pick_sticky_account("model-x");

    // Should wait (no alternative)
    assert_eq!(result.wait_ms, 60000);
    assert!(result.new_index.is_none());
}
```

**3. Baseline cache hit rate measurement**
```rust
#[test]
fn measure_baseline_cache_hit_rate() {
    // Run 100 requests and track cache_read_input_tokens
    let total_cached: u64 = responses
        .iter()
        .map(|r| r.usage.cache_read_input_tokens.unwrap_or(0))
        .sum();

    let total_input: u64 = responses
        .iter()
        .map(|r| r.usage.input_tokens)
        .sum();

    let baseline_rate = (total_cached as f64 / total_input as f64) * 100.0;

    assert!(baseline_rate > 0.0, "Should have some cache hits");
    println!("Baseline cache hit rate: {:.1}%", baseline_rate);
}
```

**4. Manual testing**
```bash
# Test: Rate-limited account with <2min reset
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: test" \
  -d '{
    "model": "claude-sonnet-4-5-thinking",
    "max_tokens": 100,
    "messages": [{"role": "user", "content": "Hello"}]
  }'

# Expected: 429 with Retry-After header
# Expected headers:
# Retry-After: 30 (if reset is 30 seconds away)
# X-Wait-For-Rate-Reset: true
```

**Deliverables**:
- Comprehensive test suite with actual code
- Manual testing report
- Performance benchmarks (cache hit rate)

---

### Phase 5: Documentation & Rollout (Day 5)

**Objective**: Document changes and prepare for rollout.

#### Tasks

1. **Update documentation**
   - Add enhanced sticky session section to CLAUDE.md
   - Document wait-time decision logic
   - Document operational metric (cache hit rate formula)
   - Add configuration options

2. **Configuration**
   ```json
   {
     "sticky_session": {
       "enabled": true,
       "duration_ms": 300000,
       "max_wait_ms": 120000
     }
   }
   ```

3. **Monitoring setup**
   - Track prompt cache hit rate using operational formula
   - Monitor account switch frequency
   - Measure wait times
   - Set up alerts on degradation

4. **Baseline Measurement**
   ```bash
   # Before implementation, run:
   curl http://localhost:8045/api/metrics/cache-hit-rate

   # This endpoint returns:
   # {
   #   "total_requests": 1000,
   #   "average_cache_hit_rate": 35.2,
   #   "p50_cache_hit_rate": 40.0,
   #   "p95_cache_hit_rate": 50.0
   # }
   ```

**Deliverables**:
- Updated documentation
- Configuration examples
- Monitoring setup
- Baseline measurement report

---

## Success Metrics

### Quantitative

| Metric | Before | Target | Measurement |
|--------|--------|--------|-------------|
| **Prompt Cache Hit Rate** | 30-40% | 50-60% | Formula: `cached_tokens / input_tokens × 100` |
| **Account Switch Frequency** | High | Optimized | Switch event tracking |
| **Average Wait Time** | N/A | <30s | Wait time tracking |
| **429 with Retry-After** | 0% | Track | Response header tracking |

### Baseline Measurement Plan

**Week 1: Pre-Implementation**
1. Enable metric tracking for `cache_read_input_tokens`
2. Collect data from 1000+ requests
3. Calculate baseline cache hit rate
4. Document current account switch frequency

**Week 2-4: Implementation**
- Implement enhanced sticky session
- Feature flag = false initially

**Week 5: Comparison**
1. Enable feature flag at 50%
2. Collect data from 1000+ requests
3. Compare against baseline
4. Calculate improvement

### Qualitative

- **Performance**: Reduced latency from cache hits
- **Cost**: Lower token usage from better caching
- **Transparency**: Clear retry signals to clients

---

## Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Incorrect wait-time calculation** | Low | Medium | Comprehensive testing |
| **Excessive waiting** | Low | Medium | Configurable max_wait_ms |
| **Race conditions in selection** | Low | Low | Thread-safe DashMap |
| **Metric measurement accuracy** | Medium | Low | Validate formula with real data |

### Operational Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Increased 429 responses** | Medium | Low | Client-side retry handling |
| **Cache hit rate lower than expected** | Low | Low | Monitoring and tuning |
| **Configuration migration issues** | Low | Low | Auto-migration with validation |

---

## Dependencies

### EPIC-028: Model-Specific Rate Limiting
- **Why**: Needs model-specific rate limit tracking
- **Dependency Level**: Required
- **Blocking**: Yes - cannot implement without EPIC-028

---

## Implementation Checklist

### Phase 1: Analysis & Design
- [ ] Read current implementation
- [ ] Analyze selection patterns
- [ ] Design enhanced logic with complete implementations
- [ ] Write baseline measurement plan
- [ ] Design configuration migration

### Phase 2: Core Implementation
- [ ] Implement `AccountSelectionResult` struct
- [ ] Implement `WaitInfo` struct
- [ ] Implement `EnhancedStickyConfig` struct
- [ ] Implement `get_sticky_key()` method
- [ ] Implement `pick_sticky_account()` method
- [ ] Implement `should_wait_for_current_account()` method
- [ ] Implement `force_account_switch()` method (COMPLETE)
- [ ] Implement `get_available_account_count_for_model()` method
- [ ] Add configuration migration logic
- [ ] Write unit tests

### Phase 3: Server Integration
- [ ] Update server to use enhanced selection
- [ ] Handle wait responses with 429 + Retry-After
- [ ] Add client retry documentation
- [ ] Write integration tests

### Phase 4: Testing & Validation
- [ ] Measure baseline cache hit rate (1000+ requests)
- [ ] Test wait-time decision logic (with actual code)
- [ ] Test account switching scenarios (with actual code)
- [ ] Measure post-implementation cache hit rate
- [ ] Compare against baseline
- [ ] Manual testing with real accounts

### Phase 5: Documentation & Rollout
- [ ] Update CLAUDE.md
- [ ] Add configuration options
- [ ] Set up monitoring with operational formula
- [ ] Execute gradual rollout (50% → 100%)
- [ ] Compare metrics against baseline

---

## Rollout Plan

### Day 1: Design & Baseline
- Analysis and design of enhanced logic
- Measure baseline cache hit rate

### Day 2-3: Implementation
- Core enhanced sticky session implementation
- Configuration migration

### Day 4: Integration & Testing
- Server integration and testing
- Complete test coverage

### Day 5: Rollout
- Morning: Documentation and monitoring
- Afternoon: Gradual rollout (50% → 100%)
- End of week: Compare metrics against baseline

### Rollback Criteria
- Cache hit rate decrease >10% from baseline
- Account switch frequency increase >50%
- User complaints about excessive waiting
- Average wait time >60s

---

## Questions for Team Review

1. **Max Wait Time**: Should 2 minutes be configurable or fixed?
2. **Retry Handling**: How should clients handle 429 with Retry-After?
3. **Baseline Duration**: Is 1000 requests sufficient for baseline measurement?
4. **Defer**: Should this be deferred to cost optimization sprint?

---

## Next Steps

1. **Review this epic** with tech lead
2. **Confirm baseline measurement approach**
3. **Begin implementation** after EPIC-028 completion
4. **Measure baseline** before implementing changes
5. **Execute rollout** with before/after comparison

---

**Author**: Amelia (Dev Agent)
**Status**: Updated for Re-Validation
**Version**: 2.0
**Date**: 2026-01-15
