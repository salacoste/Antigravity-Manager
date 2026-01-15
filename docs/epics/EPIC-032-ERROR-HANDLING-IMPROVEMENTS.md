# EPIC-032: Error Handling Improvements

**Status**: 📋 Planning
**Priority**: 🟡 P3 LOW
**Estimate**: 2-3 days
**Dependency**: None
**Author**: Amelia (Dev Agent)
**Date**: 2026-01-15

---

## Executive Summary

**Problem**: Two reliability issues impact user experience:
1. Empty SSE responses from Antigravity API (2-5% failure rate)
2. 5xx server errors treated as fatal, causing premature account switching

**Solution**: Implement empty response retry with exponential backoff and treat 5xx errors as soft failures (try next account without marking rate-limited).

**Impact**: Reduces transient error rate from 2-5% to <0.5%, improving reliability without complex changes.

---

## Business Case

### Current State
```
Empty Response Failures: 2-5%
5xx Error Handling: Fatal (marks account rate-limited)
User Pain: LOW - Infrequent but annoying errors
Revenue Impact: LOW - Marginal reliability improvement
Complexity: LOW - Isolated error handling changes
```

### Target State
```
Empty Response Failures: <0.5%
5xx Error Handling: Soft failure (try next account)
User Experience: Graceful handling of transient errors
```

### Quantitative Metrics
```
Before: Empty response failures 2-5%
After:  Empty response failures <0.5%
Improvement: -90%

Visibility: Low (rare issue)
Complexity: Low (isolated changes)
Risk: Low (easy rollback)
Effort: 2-3 days
```

---

## Technical Analysis

### Problem 1: Empty Response Retry

**Current Implementation**:
```rust
// src-tauri/src/proxy/handlers/claude.rs (streaming)
// No retry for empty responses
// Returns "No response after retries" error immediately
```

**Problem**: Antigravity API sometimes returns empty SSE streams due to transient issues.

### Alternative Proxy Solution

**File**: `alternative_proxy_app/src/cloudcode/sse-streamer.js`

```javascript
/**
 * Handle empty response with retry
 * @param {Response} response - Fetch response
 * @param {number} retryCount - Current retry count
 * @param {number} maxRetries - Maximum retry attempts (default: 2)
 */
async function* streamSSEWithRetry(response, retryCount = 0, maxRetries = 2) {
  let isEmpty = true;

  try {
    // Check if response is empty
    const reader = response.body.getReader();
    const { done, value } = await reader.read();

    if (done || !value || value.length === 0) {
      isEmpty = true;
    } else {
      isEmpty = false;
      // Yield the first chunk
      yield value;
    }

    // Continue reading rest of stream
    while (!done) {
      const { done, value } = await reader.read();
      if (!done && value && value.length > 0) {
        yield value;
      }
    }

  } catch (error) {
    if (isEmpty && retryCount < maxRetries) {
      // Retry with exponential backoff
      const backoffMs = Math.pow(2, retryCount) * 500; // 500ms, 1000ms, 2000ms
      console.warn(`Empty response detected, retrying in ${backoffMs}ms (attempt ${retryCount + 1}/${maxRetries})`);

      await sleep(backoffMs);

      // Retry the request
      const retryResponse = await fetch(request.url, {
        ...request.options,
        signal: AbortSignal.timeout(30000) // 30s timeout
      });

      yield* streamSSEWithRetry(retryResponse, retryCount + 1, maxRetries);
    } else {
      throw new Error(`No response after ${maxRetries} retries`);
    }
  }
}

// Configuration
const MAX_EMPTY_RESPONSE_RETRIES = 2;
const INITIAL_BACKOFF_MS = 500;
```

### Problem 2: 5xx Error Handling as Soft Failure

**Current Implementation**:
```rust
// src-tauri/src/proxy/handlers/claude.rs
Err(e) => return Err(e.to_string()),  // ❌ Fatal error - marks account rate-limited
```

**Problem**: 5xx errors (server-side issues) are treated as fatal, causing unnecessary account rotation when a retry would succeed.

### Alternative Proxy Solution

**File**: `alternative_proxy_app/src/cloudcode/request-builder.js`

```javascript
/**
 * Determine if error should be treated as soft failure
 * @param {Response} response - Fetch response
 * @param {Error} error - Network error
 * @returns {boolean}
 */
function isSoftFailure(response, error) {
  // 5xx errors = soft failure - try next account
  if (response && response.status >= 500 && response.status < 600) {
    return true;
  }

  // Network errors = soft failure - try next account
  if (error && isNetworkError(error)) {
    return true;
  }

  return false;
}

/**
 * Handle request with soft failure detection
 * @param {string} url - Request URL
 * @param {object} options - Fetch options
 * @returns {Promise<Response>}
 */
async function fetchWithSoftFailure(url, options) {
  try {
    const response = await fetch(url, options);

    if (isSoftFailure(response, null)) {
      // Don't mark account as rate-limited
      // Just return error to trigger account switch
      throw new SoftFailureError(`Server error: ${response.status}`);
    }

    return response;

  } catch (error) {
    if (isNetworkError(error)) {
      // Network error = soft failure
      throw new SoftFailureError(`Network error: ${error.message}`);
    }
    throw error;
  }
}
```

---

## Implementation Plan

### Phase 1: Empty Response Retry (Day 1)

**Objective**: Implement exponential backoff retry for empty responses.

#### Update `src-tauri/src/proxy/handlers/claude.rs`

```rust
use tokio::time::{sleep, Duration};
use std::time::Duration;

const MAX_EMPTY_RESPONSE_RETRIES: u32 = 2;

/// Handle streaming with empty response retry
async fn stream_with_retry(
    account: &ProxyToken,
    request: &ClaudeRequest,
    model: &str,
) -> Result<impl Stream<Item = Result<Bytes, String>>, String> {
    let mut retry_count = 0;

    loop {
        match attempt_stream(account, request, model).await {
            Ok(mut stream) => {
                // Check if stream is empty
                let first_chunk = stream.next().await;

                match first_chunk {
                    Some(Ok(bytes)) if !bytes.is_empty() => {
                        // Stream has data - return it
                        return Ok(stream);
                    }
                    Some(Ok(_)) | None => {
                        // Empty stream - check retry
                        if retry_count < MAX_EMPTY_RESPONSE_RETRIES {
                            let backoff_ms = 500 * 2_u64.pow(retry_count);
                            tracing::warn!(
                                "Empty response detected, retrying in {}ms (attempt {}/{})",
                                backoff_ms,
                                retry_count + 1,
                                MAX_EMPTY_RESPONSE_RETRIES
                            );

                            sleep(Duration::from_millis(backoff_ms)).await;
                            retry_count += 1;
                            continue;
                        } else {
                            return Err("No response after retries".to_string());
                        }
                    }
                    Some(Err(e)) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
    }
}

/// Attempt single streaming request
async fn attempt_stream(
    account: &ProxyToken,
    request: &ClaudeRequest,
    model: &str,
) -> Result<impl Stream<Item = Result<Bytes, String>>, String> {
    // ... existing streaming logic
}
```

**Deliverables**:
- Empty response retry implementation
- Exponential backoff logic
- Unit tests

---

### Phase 2: 5xx Soft Failure (Day 2)

**Objective**: Treat 5xx errors as soft failures.

#### Update Error Handling

```rust
/// Custom error types
#[derive(Debug)]
pub enum ProxyError {
    RateLimitError(String),
    AuthError(String),
    SoftFailureError(String), // NEW: Soft failure - try next account
    FatalError(String),
}

impl ProxyError {
    /// Check if error is soft failure
    pub fn is_soft_failure(&self) -> bool {
        matches!(self, ProxyError::SoftFailureError(_))
    }

    /// Convert HTTP status to error
    pub fn from_status(status: u16, message: String) -> Self {
        match status {
            429 => ProxyError::RateLimitError(message),
            401 | 403 => ProxyError::AuthError(message),
            500..=599 => ProxyError::SoftFailureError(message), // NEW: 5xx = soft failure
            _ => ProxyError::FatalError(message),
        }
    }
}

// Update request handler
async fn handle_request(
    State(manager): State<Arc<TokenManager>>,
    req: Request,
) -> Result<Response, StatusCode> {
    let model_id = extract_model_id(&req);

    loop {
        let account = manager.select_account(&model_id)?;

        match make_request(&account, &req).await {
            Ok(response) => {
                return Ok(response);
            }
            Err(ProxyError::SoftFailureError(e)) => {
                // Soft failure - try next account without marking rate-limited
                tracing::warn!("Soft failure for account {}: {}", account.account_id, e);
                continue; // Loop to next account
            }
            Err(ProxyError::RateLimitError(e)) => {
                // Mark account as rate-limited
                manager.mark_rate_limited(&account.account_id, &model_id, 60000);
                continue; // Loop to next account
            }
            Err(e) => {
                // Fatal error - return to client
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
}
```

**Deliverables**:
- Soft failure error type
- Updated error handling logic
- Unit tests

---

### Phase 3: Testing & Validation (Day 2, Afternoon - Day 3)

**Objective**: Comprehensive testing of error handling improvements.

#### Test Scenarios

1. **Empty response retry**
   ```rust
   #[tokio::test]
   async fn test_empty_response_retry() {
       // Test: Empty response triggers retry
       // Test: Retry with exponential backoff
       // Test: Max retries respected
   }
   ```

2. **5xx soft failure**
   ```rust
   #[tokio::test]
   async fn test_5xx_soft_failure() {
       // Test: 500 error = soft failure
       // Test: Account not marked rate-limited
       // Test: Next account tried automatically
   }
   ```

3. **Manual testing**
   ```bash
   # Test: Trigger empty response (simulate transient error)
   # Test: Trigger 5xx error (simulate server issue)
   # Verify: Retry behavior and account switching
   ```

**Deliverables**:
- Comprehensive test suite
- Manual testing report
- Bug fixes

---

### Phase 4: Documentation & Rollout (Day 3, Afternoon)

**Objective**: Document changes and prepare for rollout.

#### Tasks

1. **Update documentation**
   - Add error handling section to CLAUDE.md
   - Document retry behavior
   - Add troubleshooting guide

2. **Configuration** (optional)
   ```json
   {
     "error_handling": {
       "max_empty_retries": 2,
       "initial_backoff_ms": 500,
       "treat_5xx_as_soft_failure": true
     }
   }
   ```

3. **Monitoring setup**
   - Track empty response retry rate
   - Monitor 5xx error rate
   - Measure success rate after improvements

**Deliverables**:
- Updated documentation
- Configuration examples
- Monitoring setup

---

## Success Metrics

### Quantitative

| Metric | Before | Target | Measurement |
|--------|--------|--------|-------------|
| **Empty Response Failure Rate** | 2-5% | <0.5% | Error monitoring |
| **5xx Recovery Success Rate** | 0% | Track | Retry success rate |
| **Overall Success Rate** | 95-97% | >99% | Request completion |

### Qualitative

- **Reliability**: Graceful handling of transient errors
- **User Experience**: Fewer "No response" errors
- **Maintainability**: Clear error type hierarchy

---

## Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Infinite retry loop** | Low | High | Max retry limit |
| **Excessive retry delay** | Low | Medium | Configurable backoff |
| **Misclassified errors** | Low | Low | Testing and monitoring |

### Operational Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Increased latency** | Low | Low | Monitoring and tuning |
| **Account pool exhaustion** | Low | Medium | Soft failure logic |

---

## Dependencies

### None - Can be implemented independently

---

## Implementation Checklist

### Phase 1: Empty Response Retry
- [ ] Implement empty response detection
- [ ] Implement exponential backoff retry
- [ ] Add retry count tracking
- [ ] Write unit tests

### Phase 2: 5xx Soft Failure
- [ ] Define `ProxyError` enum with soft failure variant
- [ ] Update error classification logic
- [ ] Update account switching logic
- [ ] Write unit tests

### Phase 3: Testing & Validation
- [ ] Test empty response retry scenarios
- [ ] Test 5xx soft failure scenarios
- [ ] Manual testing with simulated errors
- [ ] Performance benchmarks

### Phase 4: Documentation & Rollout
- [ ] Update CLAUDE.md
- [ ] Add configuration options
- [ ] Set up monitoring
- [ ] Execute rollout

---

## Rollout Plan

### Day 1: Empty Response Retry
- Implement retry logic
- Unit tests

### Day 2: 5xx Soft Failure
- Implement soft failure handling
- Integration tests

### Day 3: Rollout
- Morning: Documentation and monitoring
- Afternoon: Full rollout (low risk)

### Rollback Criteria
- Empty response failure rate increase > 1%
- Infinite retry loops detected
- User complaints about increased latency

---

## Questions for Team Review

1. **Max Retries**: Should 2 retries be configurable?
2. **Backoff Strategy**: Is exponential backoff (500ms, 1000ms, 2000ms) appropriate?
3. **Bundling**: Should this be bundled with other P3 items?
4. **Defer**: Should this be fix-on-demand only?

---

## Alternative: Fix-On-Demand Approach

Given the LOW priority and minimal user impact, consider:

**Option A**: Implement now (2-3 days)
**Option B**: Defer until users report issues
**Option C**: Implement as "error handling polish" with other P3 items

**Recommendation**: ⚠️ **DEFER to P3** - Fix on-demand only, nice-to-have polish

---

## Next Steps

1. **Review this epic** with tech lead
2. **Decide on approach** (implement now vs. defer)
3. **Begin implementation** if approved

---

**Author**: Amelia (Dev Agent)
**Status**: Ready for Review
**Version**: 1.0
