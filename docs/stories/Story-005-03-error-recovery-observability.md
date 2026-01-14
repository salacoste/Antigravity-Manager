# Story-005-03: Error Recovery Docs & Observability

**Epic**: Epic-005-Gemini-3-Pro-High-Compliance
**Priority**: MEDIUM
**Effort**: 2 hours
**Type**: CODE + DOCS (Hybrid)
**Status**: PENDING
**Sequential Order**: 3/8 (Must complete Story-005-02 first)

---

## 📋 User Story

**As a** developer troubleshooting API errors
**I want** comprehensive documentation of error recovery strategies and observable retry events
**So that** I can understand system behavior, diagnose issues, and monitor error patterns effectively

---

## 🎯 Context and Background

### Current State

**Error Recovery Mechanisms Implemented**:
1. **Multi-Endpoint Fallback** (`upstream/client.rs:65-277`)
   - Primary: `cloudcode-pa.googleapis.com/v1internal` (prod)
   - Fallback: `daily-cloudcode-pa.sandbox.googleapis.com/v1internal` (daily)
   - Auto-retry on: 429, 408, 404, 5xx errors

2. **Corrupted Signature Retry** (`handlers/claude.rs:23-119`)
   - JWT signature validation (header.payload.signature)
   - Filters invalid thinking blocks
   - Max retry attempts: 3
   - Min signature length: 100 characters

3. **Account Rotation** (`token_manager.rs`)
   - Automatic account switching on 429 (rate limit)
   - Subscription tier prioritization (Ultra > Pro > Free)

**Gap Analysis**:
- ✅ **Implementation**: All retry logic is implemented and working
- ❌ **Documentation**: Retry strategies not comprehensively documented
- ❌ **Observability**: Limited event logging for retry events
- ❌ **Transparency**: Users cannot see retry attempts in real-time

**Reference**: `gemini-3-pro-high-COMPARISON.md:689-703`

```yaml
gap: "Corrupted signature retry logic not fully documented"
impact: "LOW - Affects error recovery transparency"
effort: "LOW - Documentation update + minimal observability code"

action_required:
  step_1: "Investigate upstream client retry logic"  # ✅ COMPLETED
  step_2: "Document retry strategy in error handling section"  # ⬅️ THIS STORY
  step_3: "Add observability for retry events"  # ⬅️ THIS STORY

timeline: "2-3 days"
owner: "Documentation team + Backend team"
```

### Why This Matters

**1. Transparency Parity with Antigravity v1.13.3**:
- Professional tools document error recovery strategies
- Users expect clear error handling documentation
- Monitoring capabilities improve production reliability

**2. Operational Excellence**:
- Faster troubleshooting with observable retry events
- Reduced support burden through self-service documentation
- Better understanding of system resilience

**3. Production Confidence**:
- Clear understanding of when retries happen
- Monitoring capabilities for detecting systematic failures
- Evidence-based debugging

---

## 🔨 Implementation Specification

### Part 1: Documentation Updates (1.5 hours)

#### Document 1: Error Recovery Architecture

**File**: `docs/architecture/error-recovery.md` (NEW)

**Structure**:
```markdown
# Error Recovery Architecture

## Overview

Antigravity Manager implements a three-layer error recovery system:
1. **Multi-Endpoint Fallback** - Infrastructure resilience
2. **Corrupted Signature Retry** - Protocol-level error handling
3. **Account Rotation** - Rate limit mitigation

---

## Layer 1: Multi-Endpoint Fallback

### Mechanism

**Endpoint Hierarchy**:
1. Primary: `https://cloudcode-pa.googleapis.com/v1internal` (prod)
2. Fallback: `https://daily-cloudcode-pa.sandbox.googleapis.com/v1internal` (daily)

**Retry Conditions**:
- `429 Too Many Requests` - Rate limiting from upstream
- `408 Request Timeout` - Network or server timeout
- `404 Not Found` - Endpoint temporarily unavailable
- `5xx Server Error` - Upstream service degradation

**Non-Retry Conditions**:
- `400 Bad Request` - Client error (invalid request format)
- `401 Unauthorized` - Authentication failure (invalid token)
- `403 Forbidden` - Account blocked or quota exceeded

### Algorithm

```
FOR each endpoint IN [prod, daily]:
    TRY:
        response = POST request with OAuth token

        IF response.status IN [429, 408, 404, 5xx]:
            IF has_next_endpoint:
                LOG warning "Endpoint {} returned {}, trying next endpoint"
                CONTINUE to next endpoint
            ELSE:
                RETURN response (last attempt, no more fallbacks)

        IF response.status == 200:
            LOG success "Endpoint {} succeeded"
            RETURN response

        ELSE:
            RETURN response (non-retriable error)

    CATCH network_error:
        IF has_next_endpoint:
            LOG debug "HTTP request failed at {}: {}"
            CONTINUE to next endpoint
        ELSE:
            RETURN error (all endpoints exhausted)
```

### Configuration

**Timeouts**:
- Connect timeout: 20 seconds
- Request timeout: 600 seconds (10 minutes)
- Pool idle timeout: 90 seconds
- TCP keepalive: 60 seconds

**Connection Pooling**:
- Max idle per host: 16 connections
- Connection reuse enabled for performance

---

## Layer 2: Corrupted Signature Retry

### Problem Statement

Extended thinking responses include JWT signatures for integrity verification:
```json
{
  "type": "thinking",
  "thinking": "<extended reasoning content>",
  "signature": "eyJhbGc...header.eyJzdWI...payload.SflKxwR...signature"
}
```

**Issue**: Occasionally signatures become corrupted (incomplete, malformed, or truncated).

### Detection Criteria

**Valid JWT Signature**:
1. Format: `header.payload.signature` (exactly 3 parts)
2. Each part: Base64url-encoded (alphanumeric + `-` + `_`)
3. Minimum length: 100 characters (typical JWT ~200+ chars)
4. No empty parts

**Invalid Signature Examples**:
- `"eyJhbGc."` (missing parts)
- `"invalid+format/here"` (not base64url)
- `"abc"` (too short, <100 chars)

### Retry Strategy

**Max Retry Attempts**: 3 per request

**Filtering Process**:
```
1. Scan last 30 messages (performance optimization)
2. FOR each message.content[]:
     IF block.type == "thinking":
       IF !has_valid_signature(block):
         REMOVE block from message
         INCREMENT filtered_count
3. IF filtered_count > 0:
     LOG warning "{} invalid thinking blocks filtered"
     RETRY request with cleaned messages
```

**Signature Validation**:
```rust
fn is_valid_jwt_format(signature: &str) -> bool {
    let parts: Vec<&str> = signature.split('.').collect();

    // Must have exactly 3 parts
    if parts.len() != 3 { return false; }

    // Each part must be non-empty and base64url
    for part in parts {
        if part.is_empty() { return false; }
        if !part.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return false;
        }
    }

    true
}

fn has_valid_signature(block: &ContentBlock) -> bool {
    match block {
        ContentBlock::Thinking { signature, thinking, .. } => {
            // Trailing signature case (empty thinking)
            if thinking.is_empty() && signature.is_some() {
                return signature.as_ref().map_or(false, |s| is_valid_jwt_format(s));
            }

            // Normal case (content + signature)
            signature.as_ref().map_or(false, |s| {
                s.len() >= MIN_SIGNATURE_LENGTH && is_valid_jwt_format(s)
            })
        }
        _ => true, // Non-thinking blocks always valid
    }
}
```

### Reference Implementation

**File**: `src-tauri/src/proxy/handlers/claude.rs:41-119`

**Constants**:
- `MAX_RETRY_ATTEMPTS = 3`
- `MIN_SIGNATURE_LENGTH = 100`
- `JWT_PARTS = 3`

---

## Layer 3: Account Rotation

### Mechanism

**Trigger**: 429 (Too Many Requests) from upstream API

**Account Selection Strategy**:
1. **Subscription Tier Prioritization**:
   - Ultra subscribers: Unlimited quota, highest priority
   - Pro subscribers: Higher quota than Free, medium priority
   - Free tier: Limited quota, lowest priority

2. **Quota-Based Weighting**:
   - Prefer accounts with more remaining quota
   - Avoid recently rate-limited accounts

3. **Sticky Session Support** (optional):
   - Bind session ID to specific account
   - Maintain account consistency across requests

### Rotation Flow

```
ON 429 Error:
    1. Mark current account as rate-limited (cooldown: 60 seconds)
    2. Remove from available pool temporarily
    3. Select next account from available pool using strategy
    4. Retry request with new account token
    5. IF all accounts rate-limited:
         RETURN 429 to client with Retry-After header
```

---

## Error Code Reference

### Retriable Errors

| Code | Meaning | Retry Strategy | Expected Duration |
|------|---------|----------------|-------------------|
| 429 | Rate Limit | Account rotation + endpoint fallback | Immediate |
| 408 | Request Timeout | Endpoint fallback | <1 second |
| 404 | Not Found | Endpoint fallback | <1 second |
| 500 | Internal Server Error | Endpoint fallback | <1 second |
| 502 | Bad Gateway | Endpoint fallback | <1 second |
| 503 | Service Unavailable | Endpoint fallback | <1 second |
| 504 | Gateway Timeout | Endpoint fallback | <1 second |

### Non-Retriable Errors

| Code | Meaning | Recommended Action |
|------|---------|-------------------|
| 400 | Bad Request | Fix request format, check API documentation |
| 401 | Unauthorized | Re-authenticate account, verify OAuth token |
| 403 | Forbidden | Check account status, verify quota not exceeded |

---

## Monitoring Best Practices

### Key Metrics to Track

1. **Retry Success Rate**: % of retries that eventually succeed
2. **Endpoint Fallback Frequency**: How often daily endpoint is used
3. **Account Rotation Rate**: % of requests requiring account switch
4. **Signature Corruption Rate**: % of responses with invalid signatures

### Alert Thresholds

- **High Retry Rate** (>20%): Indicates upstream instability
- **Frequent Fallbacks** (>10%): Primary endpoint may be degraded
- **Account Rotation Spike** (>30%): Quota exhaustion across accounts
- **Signature Corruption** (>5%): Potential protocol issue

---

## Troubleshooting Guide

### Scenario 1: All Requests Failing with 429

**Diagnosis**:
- All accounts exhausted quota simultaneously
- Possible cause: Rate limit changes, excessive usage spike

**Resolution**:
1. Check account quotas in Dashboard
2. Wait for quota reset (varies by account type)
3. Consider adding more accounts or upgrading to Pro/Ultra

### Scenario 2: Intermittent 5xx Errors

**Diagnosis**:
- Upstream service degradation
- Multi-endpoint fallback activating

**Resolution**:
- Errors should auto-resolve via fallback to daily endpoint
- If persists >5 minutes, check Google Cloud Status

### Scenario 3: Repeated Signature Corruption

**Diagnosis**:
- Upstream protocol issue or data truncation

**Resolution**:
- Automatic retry should handle most cases (max 3 attempts)
- If >5% of requests affected, report to support with logs

---

## References

- **Implementation**: `src-tauri/src/proxy/upstream/client.rs:65-277`
- **Signature Validation**: `src-tauri/src/proxy/handlers/claude.rs:41-119`
- **Token Manager**: `src-tauri/src/proxy/token_manager.rs`
- **Story**: `docs/stories/Story-005-03-error-recovery-observability.md`
```

---

### Part 2: Observability Enhancements (0.5 hours)

#### Code Changes: Retry Event Logging

**File**: `src-tauri/src/proxy/upstream/client.rs`

**Enhancement 1: Structured Retry Event Logging**

**Location**: After line 258 (endpoint fallback decision)

```rust
// Enhanced observability for retry events
if has_next && Self::should_try_next_endpoint(status) {
    // 🆕 Story-005-03: Emit structured retry event
    tracing::warn!(
        target: "retry_event",
        event_type = "endpoint_fallback",
        current_endpoint = %base_url,
        next_endpoint = %V1_INTERNAL_BASE_URL_FALLBACKS[idx + 1],
        status_code = %status,
        method = %method,
        attempt = idx + 1,
        max_attempts = V1_INTERNAL_BASE_URL_FALLBACKS.len(),
        "Endpoint fallback triggered: {} → {}",
        base_url,
        V1_INTERNAL_BASE_URL_FALLBACKS[idx + 1]
    );

    last_err = Some(format!("Upstream {} returned {}", base_url, status));
    continue;
}
```

**Enhancement 2: Retry Success Logging**

**Location**: After line 254 (successful response)

```rust
if status.is_success() {
    if idx > 0 {
        // 🆕 Story-005-03: Log successful retry with metrics
        tracing::info!(
            target: "retry_event",
            event_type = "retry_success",
            endpoint = %base_url,
            status_code = %status,
            attempt = idx + 1,
            max_attempts = V1_INTERNAL_BASE_URL_FALLBACKS.len(),
            "✓ Upstream fallback succeeded | Endpoint: {} | Status: {} | Attempt: {}/{}",
            base_url,
            status,
            idx + 1,
            V1_INTERNAL_BASE_URL_FALLBACKS.len()
        );
    }
    return Ok(resp);
}
```

---

**File**: `src-tauri/src/proxy/handlers/claude.rs`

**Enhancement 3: Signature Corruption Event Logging**

**Location**: In `filter_invalid_thinking_blocks()` function after line 138

```rust
fn filter_invalid_thinking_blocks(messages: &mut Vec<Message>) {
    let mut total_filtered = 0;
    let start_idx = messages.len().saturating_sub(30);
    let messages_to_filter = &mut messages[start_idx..];

    for message in messages_to_filter.iter_mut() {
        // ... existing filtering logic ...

        // 🆕 Story-005-03: Log signature corruption details
        if filtered > 0 {
            tracing::warn!(
                target: "retry_event",
                event_type = "signature_corruption",
                message_role = ?message.role,
                filtered_blocks = filtered,
                total_blocks = original_count,
                corruption_rate = %format!("{:.1}%", (filtered as f64 / original_count as f64) * 100.0),
                "Invalid thinking blocks filtered: {}/{} (corruption rate: {:.1}%)",
                filtered,
                original_count,
                (filtered as f64 / original_count as f64) * 100.0
            );
        }
    }

    // 🆕 Story-005-03: Summary event if any blocks filtered
    if total_filtered > 0 {
        tracing::warn!(
            target: "retry_event",
            event_type = "signature_corruption_summary",
            total_filtered = total_filtered,
            messages_affected = messages_to_filter.len(),
            "Signature corruption summary: {} blocks filtered across {} messages",
            total_filtered,
            messages_to_filter.len()
        );
    }
}
```

**Enhancement 4: Account Rotation Event Logging**

**File**: `src-tauri/src/proxy/token_manager.rs`

**Location**: In account rotation logic (around `select_account()` function)

```rust
// 🆕 Story-005-03: Log account rotation event
tracing::info!(
    target: "retry_event",
    event_type = "account_rotation",
    reason = "rate_limit_429",
    previous_account = %previous_account_id,
    new_account = %selected_account.account_id,
    subscription_tier = %selected_account.subscription_tier,
    remaining_quota = selected_account.quota.remaining,
    "Account rotation: {} → {} (tier: {}, quota: {})",
    previous_account_id,
    selected_account.account_id,
    selected_account.subscription_tier,
    selected_account.quota.remaining
);
```

---

#### Structured Logging Format

**Log Target**: `retry_event` (filterable via `RUST_LOG=retry_event=info`)

**Event Types**:
1. `endpoint_fallback` - Multi-endpoint retry triggered
2. `retry_success` - Retry succeeded after fallback
3. `signature_corruption` - Invalid JWT signature detected
4. `signature_corruption_summary` - Aggregate corruption stats
5. `account_rotation` - Account switched due to rate limit

**Fields**:
- `event_type`: Event classification
- `attempt`: Current retry attempt number
- `max_attempts`: Maximum retry limit
- `status_code`: HTTP status code
- `filtered_blocks`: Number of corrupted blocks removed
- `corruption_rate`: Percentage of corrupted blocks

---

### Part 3: Monitoring Dashboard Integration (Optional - Future Story)

**Note**: This is NOT in scope for Story-005-03, but documented for future reference.

**Potential Enhancement**: Add Retry Metrics panel to Monitor page

**Metrics to Display**:
- Retry success rate (last 1h, 24h, 7d)
- Endpoint fallback frequency
- Account rotation rate
- Signature corruption rate

**UI Mockup**:
```
┌─────────────────────────────────────────────┐
│ Error Recovery Metrics (Last 24h)          │
├─────────────────────────────────────────────┤
│ Retry Success Rate:        96.2%  ✅        │
│ Endpoint Fallbacks:        12     (3.4%)    │
│ Account Rotations:         48     (13.7%)   │
│ Signature Corruptions:     3      (0.9%)    │
└─────────────────────────────────────────────┘
```

---

## ✅ Acceptance Criteria

### AC-1: Error Recovery Documentation Created

**Given** I navigate to `docs/architecture/error-recovery.md`
**When** I review the documentation
**Then** I see:
- Complete description of all 3 error recovery layers
- Algorithm pseudocode for each mechanism
- Retry condition tables (retriable vs non-retriable)
- Troubleshooting guide with 3+ scenarios
- Code references to actual implementation

**Validation**: Documentation peer review + technical accuracy verification

### AC-2: Structured Retry Events Logged

**Given** a request fails with 429 error at primary endpoint
**When** the system performs endpoint fallback
**Then**:
- Log entry created with target `retry_event`
- Event type: `endpoint_fallback`
- Fields include: current_endpoint, next_endpoint, status_code, attempt
- Log level: WARN

**Validation**:
1. Trigger 429 error (block account temporarily)
2. Check logs: `grep "retry_event" logs/antigravity.log`
3. Verify structured fields present

### AC-3: Signature Corruption Events Logged

**Given** a response contains 2 thinking blocks with invalid JWT signatures
**When** the signature validation filter runs
**Then**:
- Log entry created with target `retry_event`
- Event type: `signature_corruption`
- Fields include: filtered_blocks, total_blocks, corruption_rate
- Summary event shows total filtered across all messages

**Validation**:
1. Create test with mock corrupted signatures
2. Verify filtering logic detects corruption
3. Check logs for `signature_corruption` events

### AC-4: Account Rotation Events Logged

**Given** current account hits rate limit (429)
**When** TokenManager selects next available account
**Then**:
- Log entry created with target `retry_event`
- Event type: `account_rotation`
- Fields include: previous_account, new_account, subscription_tier, remaining_quota
- Log level: INFO

**Validation**:
1. Exhaust account quota
2. Trigger 429 error
3. Verify account rotation event logged

### AC-5: Filterable Retry Logs

**Given** I want to analyze retry behavior
**When** I run with `RUST_LOG=retry_event=info`
**Then**:
- Only retry-related events appear
- Other debug logs suppressed
- Structured JSON logs parseable by log aggregators

**Validation**:
```bash
RUST_LOG=retry_event=info cargo run
# Should see only retry_event logs
```

### AC-6: Documentation Cross-References

**Given** I'm reading error recovery documentation
**When** I follow code references
**Then**:
- All file paths and line numbers are accurate
- Referenced code sections exist and match descriptions
- No broken links or outdated references

**Validation**: Manual verification of all code references in documentation

---

## 🧪 Testing Strategy

### Unit Tests

**File**: `src-tauri/src/proxy/upstream/client_test.rs`

```rust
#[tokio::test]
async fn test_retry_event_logging() {
    // Setup mock server returning 429
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(429))
        .mount(&mock_server)
        .await;

    // Capture logs
    let subscriber = tracing_subscriber::fmt()
        .with_test_writer()
        .finish();

    // Execute request
    let client = UpstreamClient::new(None);
    let result = client.call_v1_internal("test", "token", json!({}), None).await;

    // Verify retry event logged
    assert!(logs_contain("retry_event"));
    assert!(logs_contain("endpoint_fallback"));
}
```

### Integration Tests

**Scenario**: End-to-end retry with fallback

```rust
#[tokio::test]
async fn test_endpoint_fallback_observability() {
    // 1. Mock primary endpoint returning 503
    // 2. Mock fallback endpoint returning 200
    // 3. Execute request
    // 4. Verify logs show:
    //    - endpoint_fallback event (primary → fallback)
    //    - retry_success event (fallback succeeded)
}
```

### Manual Testing

**Test Case 1: Trigger Endpoint Fallback**
1. Temporarily block primary endpoint in /etc/hosts
2. Execute API request
3. Verify fallback to daily endpoint
4. Check logs for structured retry events

**Test Case 2: Trigger Account Rotation**
1. Exhaust quota on current account
2. Execute request causing 429
3. Verify automatic account switch
4. Check logs for account_rotation event

---

## 🔗 Dependencies and Relationships

### Sequential Dependencies

**Blocks**:
- **Story-005-06** (Document First-Time Permissive Mode)
  - Permissive mode references error recovery mechanisms
  - Need error recovery docs complete first

**Blocked By**:
- **Story-005-02** (Profile Presets UI Implementation)
  - MUST complete first per sequential execution order
  - No technical dependency, organizational only

### Technical Dependencies

**Backend Dependencies**:
- `tracing` crate (already in use)
- `tracing-subscriber` (log configuration)
- No new dependencies required

**Documentation Dependencies**:
- Understanding of existing retry implementations
- Access to code references for validation

---

## 📊 Success Metrics

### Documentation Metrics

- **Completeness**: 100% of retry mechanisms documented
- **Accuracy**: 0 technical errors in code references
- **Readability**: <10 minutes to understand core concepts
- **Actionability**: 3+ troubleshooting scenarios with solutions

### Observability Metrics

- **Log Coverage**: 100% of retry paths emit structured events
- **Log Noise**: <5% increase in total log volume
- **Performance Impact**: <1ms overhead per retry event
- **Filterability**: 100% of retry events filterable via `RUST_LOG`

### User Impact Metrics

- **Troubleshooting Time**: -30% (estimated via better observability)
- **Support Tickets**: -20% (via self-service documentation)
- **Production Confidence**: +25% (via transparent error handling)

---

## ⚠️ Risks and Mitigation

### Risk 1: Log Volume Explosion

**Description**: Structured retry events may increase log volume significantly

**Probability**: MEDIUM
**Impact**: LOW (disk space, log aggregation costs)

**Mitigation**:
1. Use WARN level for retry events (not DEBUG)
2. Implement log sampling for high-frequency events (>100/min)
3. Make retry events filterable via `RUST_LOG=retry_event=info`
4. Document log rotation configuration

### Risk 2: Performance Overhead

**Description**: Structured logging may add latency to retry paths

**Probability**: LOW
**Impact**: LOW (<1ms per event)

**Mitigation**:
1. Use async logging (non-blocking)
2. Benchmark retry path latency before/after
3. Keep structured fields minimal (5-8 fields max)
4. Use static strings where possible (avoid allocations)

### Risk 3: Documentation Becomes Outdated

**Description**: Code changes may invalidate documentation references

**Probability**: MEDIUM (over time)
**Impact**: MEDIUM (confusion, outdated info)

**Mitigation**:
1. Add comment in code: `// 📖 Documented in: docs/architecture/error-recovery.md:LINE`
2. Include doc validation in PR review checklist
3. Automated link checking in CI (future enhancement)
4. Version documentation with implementation (Epic-005 context)

### Risk 4: Excessive Event Noise for Developers

**Description**: Too many retry events may clutter development logs

**Probability**: MEDIUM
**Impact**: LOW (developer experience)

**Mitigation**:
1. Use dedicated log target `retry_event` (filterable)
2. Default log config excludes retry events in dev mode
3. Provide CLI flag: `--show-retry-events` for debugging
4. Document how to enable/disable retry logging

---

## 📝 Notes for Developers

### Implementation Order

1. **Phase 1: Documentation** (1.5 hours)
   - Create `docs/architecture/error-recovery.md`
   - Write all sections (overview, layers, troubleshooting)
   - Validate code references

2. **Phase 2: Observability Code** (0.5 hours)
   - Add structured logging to `upstream/client.rs` (3 locations)
   - Add structured logging to `handlers/claude.rs` (2 locations)
   - Add structured logging to `token_manager.rs` (1 location)
   - Test with `RUST_LOG=retry_event=info`

### Log Target Convention

**Target**: `retry_event` (use consistently across all retry logs)

**Example**:
```rust
tracing::warn!(
    target: "retry_event",  // ← Consistent target
    event_type = "endpoint_fallback",  // ← Event classification
    current_endpoint = %base_url,  // ← Structured fields
    "Human-readable message"  // ← Message for humans
);
```

### Tracing Best Practices

1. **Use appropriate log levels**:
   - `tracing::error!` - Unrecoverable errors
   - `tracing::warn!` - Retry events, degraded state
   - `tracing::info!` - Successful recovery
   - `tracing::debug!` - Detailed debugging

2. **Structured fields over string formatting**:
   ```rust
   // ✅ Good
   tracing::warn!(account_id = %id, status = %code, "Rotation triggered");

   // ❌ Bad
   tracing::warn!("Rotation triggered for account {} with status {}", id, code);
   ```

3. **Consistent event_type values**:
   - Use lowercase with underscores
   - Namespace by category: `retry_`, `error_`, `perf_`

---

## 🔍 Related Documentation

### Primary References

- **Epic-005**: `docs/epics/Epic-005-Gemini-3-Pro-High-Compliance.md`
- **COMPARISON**: `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-COMPARISON.md:689-703`
- **Story-003-05**: `docs/stories/Story-003-05-jwt-signature-validation.md` (related signature validation)

### Code References

- **Upstream Client**: `src-tauri/src/proxy/upstream/client.rs:65-277`
- **Signature Validation**: `src-tauri/src/proxy/handlers/claude.rs:23-119`
- **Token Manager**: `src-tauri/src/proxy/token_manager.rs`
- **Logger Setup**: `src-tauri/src/modules/logger.rs`

### External References

- **Tracing Docs**: https://docs.rs/tracing/latest/tracing/
- **Structured Logging**: https://www.brandur.org/logfmt
- **JWT Format**: https://datatracker.ietf.org/doc/html/rfc7519

---

## ✏️ Story Status Tracker

- [ ] **Phase 1: Documentation** (1.5 hours)
  - [ ] Create `docs/architecture/error-recovery.md`
  - [ ] Write Layer 1: Multi-Endpoint Fallback section
  - [ ] Write Layer 2: Corrupted Signature Retry section
  - [ ] Write Layer 3: Account Rotation section
  - [ ] Write Error Code Reference tables
  - [ ] Write Troubleshooting Guide (3+ scenarios)
  - [ ] Validate all code references

- [ ] **Phase 2: Observability Code** (0.5 hours)
  - [ ] Add endpoint fallback event logging (client.rs:258)
  - [ ] Add retry success event logging (client.rs:254)
  - [ ] Add signature corruption event logging (claude.rs:138)
  - [ ] Add account rotation event logging (token_manager.rs)
  - [ ] Test with `RUST_LOG=retry_event=info`
  - [ ] Verify log filtering works

**Total Effort**: 2 hours

---

**Story Created**: 2026-01-11
**Last Updated**: 2026-01-11
**Created By**: Documentation Team (Epic-005 Sequential Story Creation)
**Next Story**: Story-005-04 (Document Thinking Activation Architecture)
