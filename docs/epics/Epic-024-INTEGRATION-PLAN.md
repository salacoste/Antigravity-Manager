# Epic-024 Integration Plan: ModelSelector → TokenManager

**Story**: Story-024-02 (Adaptive Model Selection)
**Timeline**: Week 2-3 Implementation (Feb 8-21)
**Status**: Design Phase - Week 1 (Feb 1-7)

---

## Overview

This document details the integration points between the new `ModelSelector` module and the existing `TokenManager` in `token_manager.rs`.

## Integration Architecture

```text
┌─────────────────────────────────────────────────────────────────┐
│                        Request Handler                          │
│                    (openai.rs/claude.rs)                        │
└────────────────────────────────┬────────────────────────────────┘
                                 │
                                 ▼
                    ┌────────────────────────┐
                    │   ModelRecommender     │
                    │  (model_selector.rs)   │
                    └────────────┬───────────┘
                                 │
                                 ▼
                    ┌────────────────────────┐
                    │  ModelRecommendation   │
                    │  {model_id, reasoning} │
                    └────────────┬───────────┘
                                 │
                                 ▼
                    ┌────────────────────────┐
                    │    TokenManager        │
                    │  (token_manager.rs)    │
                    │  get_token_with_model()│
                    └────────────────────────┘
```

## Integration Point: token_manager.rs

### Location: Section 2 (Lines 301-400)

**Current State (Week 1)**:
- Lines 297-313: Mode B - 60s global lock logic
- Lines 314-355: Account selection loop with rate limit checking

**Proposed Integration (Week 2-3)**:

#### Step 1: Add ModelRecommender to TokenManager struct

**Location**: Lines 24-32 (struct definition)

```rust
pub struct TokenManager {
    tokens: Arc<DashMap<String, ProxyToken>>,
    current_index: Arc<AtomicUsize>,
    last_used_account: Arc<tokio::sync::Mutex<Option<(String, std::time::Instant)>>>,
    data_dir: PathBuf,
    rate_limit_tracker: Arc<RateLimitTracker>,
    sticky_config: Arc<tokio::sync::RwLock<StickySessionConfig>>,
    session_accounts: Arc<DashMap<String, String>>,
    // 🆕 INTEGRATION POINT 1: Add ModelRecommender
    model_recommender: Option<Arc<crate::modules::model_selector::ModelRecommender>>,
}
```

#### Step 2: Update TokenManager::new()

**Location**: Lines 36-46 (constructor)

```rust
pub fn new(data_dir: PathBuf, enable_adaptive_model: bool) -> Self {
    Self {
        tokens: Arc::new(DashMap::new()),
        current_index: Arc::new(AtomicUsize::new(0)),
        last_used_account: Arc::new(tokio::sync::Mutex::new(None)),
        data_dir,
        rate_limit_tracker: Arc::new(RateLimitTracker::new()),
        sticky_config: Arc::new(tokio::sync::RwLock::new(StickySessionConfig::default())),
        session_accounts: Arc::new(DashMap::new()),
        // 🆕 INTEGRATION POINT 2: Initialize ModelRecommender
        model_recommender: if enable_adaptive_model {
            Some(Arc::new(crate::modules::model_selector::ModelRecommender::new()))
        } else {
            None
        },
    }
}
```

#### Step 3: Create New Method - get_token_with_model()

**Location**: NEW - Insert after line 211 (after get_token method)

```rust
/// Get token with intelligent model selection
///
/// This method integrates with ModelRecommender to select optimal model
/// before account selection.
///
/// # Arguments
///
/// * `messages` - Request messages for complexity analysis
/// * `quota_group` - Quota group ("claude" or "gemini")
/// * `force_rotate` - Force account rotation
/// * `session_id` - Session ID for sticky session
///
/// # Returns
///
/// Tuple of (access_token, project_id, email, recommended_model)
pub async fn get_token_with_model(
    &self,
    messages: &[crate::modules::model_selector::RequestMessage],
    quota_group: &str,
    force_rotate: bool,
    session_id: Option<&str>,
) -> Result<(String, String, String, String), String> {
    // 🆕 INTEGRATION POINT 3: Model Selection Hook
    let recommended_model = if let Some(recommender) = &self.model_recommender {
        let recommendation = recommender.recommend_model(messages).await;
        tracing::info!(
            "[ModelSelector] Recommended model: {} (complexity: {:?}, confidence: {:.2})",
            recommendation.model_name,
            recommendation.complexity,
            recommendation.confidence
        );
        tracing::debug!(
            "[ModelSelector] Reasoning: {}",
            recommendation.reasoning
        );
        recommendation.model_id
    } else {
        // Fallback to default model if adaptive selection disabled
        "312".to_string()
    };

    // Call existing get_token with recommended model for rate limit checking
    let (access_token, project_id, email) = self
        .get_token(quota_group, force_rotate, session_id, Some(&recommended_model))
        .await?;

    Ok((access_token, project_id, email, recommended_model))
}
```

#### Step 4: Modify get_token_internal for Model-Aware Selection

**Location**: Lines 314-355 (account selection loop in Mode B)

**Current Code (Lines 314-355)**:
```rust
// 若无锁定，则轮询选择新账号
if target_token.is_none() {
    let start_idx = self.current_index.fetch_add(1, Ordering::SeqCst) % total;
    for offset in 0..total {
        let idx = (start_idx + offset) % total;
        let candidate = &tokens_snapshot[idx];
        if attempted.contains(&candidate.account_id) {
            continue;
        }

        // 【新增】主动避开限流或 5xx 锁定的账号
        // 🆕 Model-aware rate limit checking
        let is_limited = if let Some(m) = model {
            self.rate_limit_tracker
                .is_rate_limited_for_model(&candidate.account_id, m)
        } else {
            self.is_rate_limited(&candidate.account_id)
        };
        if is_limited {
            continue;
        }

        target_token = Some(candidate.clone());
        // ...
    }
}
```

**No Changes Required**: This code already supports model-aware rate limiting via the `model` parameter. The integration will pass the recommended model from Step 3.

---

## Integration Sequence (Week 2-3)

### Week 2 (Feb 8-14): Core Implementation

1. **Day 1-2**: Implement `ComplexityAnalyzer`
   - Keyword detector
   - Code block detector
   - Prompt structure analyzer
   - Unit tests

2. **Day 3-4**: Implement `ModelRecommender`
   - Integration with `ComplexityAnalyzer`
   - Cost tracking
   - Integration tests

3. **Day 5**: Implement `CostTracker`
   - Statistics calculation
   - Cost savings algorithm
   - Unit tests

### Week 3 (Feb 15-21): Integration

1. **Day 1-2**: Modify `TokenManager`
   - Add `model_recommender` field
   - Update constructor
   - Create `get_token_with_model()` method

2. **Day 3-4**: Update Request Handlers
   - Modify `openai.rs` to use `get_token_with_model()`
   - Modify `claude.rs` to use `get_token_with_model()`
   - Pass model recommendation to upstream API

3. **Day 5**: Integration Testing
   - End-to-end tests
   - Performance benchmarks
   - Cost savings validation

---

## Configuration Integration

### Add to `ProxyConfig` (proxy/config.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    // ... existing fields ...

    /// Enable adaptive model selection (Story-024-02)
    #[serde(default = "default_enable_adaptive_model")]
    pub enable_adaptive_model: bool,
}

fn default_enable_adaptive_model() -> bool {
    true // Default ON for Epic-024
}
```

### Add to Frontend Settings (src/pages/ApiProxy.tsx)

```typescript
// Settings section for adaptive model selection
<div className="form-control">
  <label className="label cursor-pointer">
    <span className="label-text">
      {t('proxy.adaptiveModelSelection')}
      <span className="badge badge-sm badge-info ml-2">NEW</span>
    </span>
    <input
      type="checkbox"
      className="toggle toggle-primary"
      checked={config.enable_adaptive_model}
      onChange={(e) => handleConfigChange('enable_adaptive_model', e.target.checked)}
    />
  </label>
  <span className="label-text-alt text-gray-500">
    {t('proxy.adaptiveModelSelectionDesc')}
  </span>
</div>
```

---

## Request Handler Integration

### Before (Current)

```rust
// openai.rs - get_token call (approx line 150)
let (access_token, project_id, email) = token_manager
    .get_token(quota_group, force_rotate, session_id, model_override.as_deref())
    .await?;
```

### After (Week 3)

```rust
// openai.rs - get_token_with_model call
let (access_token, project_id, email, recommended_model) = token_manager
    .get_token_with_model(&request_messages, quota_group, force_rotate, session_id)
    .await?;

// Use recommended_model for upstream API call
let upstream_model = model_override.unwrap_or(recommended_model);
```

---

## Testing Strategy

### Unit Tests (Week 2)

**File**: `src-tauri/src/modules/model_selector.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_simple_request() {
        let analyzer = ComplexityAnalyzer::new();
        let messages = vec![RequestMessage {
            role: "user".to_string(),
            content: "What is the capital of France?".to_string(),
        }];

        let (complexity, confidence, _) = analyzer.classify(&messages);
        assert_eq!(complexity, RequestComplexity::Simple);
        assert!(confidence > 0.8);
    }

    #[test]
    fn test_classify_complex_request() {
        let analyzer = ComplexityAnalyzer::new();
        let messages = vec![RequestMessage {
            role: "user".to_string(),
            content: "Debug this Python code:\n```python\ndef fibonacci(n):\n    return n\n```".to_string(),
        }];

        let (complexity, confidence, _) = analyzer.classify(&messages);
        assert_eq!(complexity, RequestComplexity::Complex);
        assert!(confidence > 0.8);
    }

    #[tokio::test]
    async fn test_model_recommender() {
        let recommender = ModelRecommender::new();
        let messages = vec![RequestMessage {
            role: "user".to_string(),
            content: "Hello world".to_string(),
        }];

        let recommendation = recommender.recommend_model(&messages).await;
        assert_eq!(recommendation.model_id, "312");
        assert_eq!(recommendation.model_name, "gemini-2.5-flash");
    }

    #[test]
    fn test_cost_tracker() {
        let mut tracker = CostTracker::new();

        // Record 8 simple, 2 complex
        for _ in 0..8 {
            tracker.record_request(RequestComplexity::Simple);
        }
        for _ in 0..2 {
            tracker.record_request(RequestComplexity::Complex);
        }

        let stats = tracker.get_stats();
        assert_eq!(stats.total_requests, 10);
        assert_eq!(stats.base_model_count, 8);
        assert_eq!(stats.thinking_model_count, 2);
        assert!(stats.cost_savings_percent > 30.0); // Expect ~40% savings
    }
}
```

### Integration Tests (Week 3)

**File**: `src-tauri/tests/integration_model_selection.rs`

```rust
#[tokio::test]
async fn test_token_manager_with_model_selection() {
    // TODO: Week 3
    // 1. Create TokenManager with adaptive model enabled
    // 2. Call get_token_with_model() with simple request
    // 3. Verify recommended_model = "312"
    // 4. Call get_token_with_model() with complex request
    // 5. Verify recommended_model = "313"
}

#[tokio::test]
async fn test_cost_savings_tracking() {
    // TODO: Week 3
    // 1. Create TokenManager with adaptive model enabled
    // 2. Process 100 mixed requests
    // 3. Verify cost_savings_percent > 10%
}
```

---

## Performance Considerations

### Overhead Analysis

**Target**: <10ms per request

**Breakdown**:
- Complexity analysis: ~2-5ms
  - Keyword detection: ~0.5ms
  - Code block detection: ~1-2ms
  - Prompt structure analysis: ~0.5-2ms
- Model recommendation: ~1-2ms
- Cost tracking update: ~0.5ms
- Total: ~4-8ms ✅

### Optimization Strategies

1. **Lazy Evaluation**: Skip complex analysis if request is obviously simple
2. **Caching**: Cache keyword/pattern matches for repeated prompts
3. **Parallel Processing**: Run detectors concurrently
4. **Early Exit**: Stop analysis once confidence threshold reached

---

## Rollback Plan

If integration issues arise:

1. **Disable via Config**: Set `enable_adaptive_model: false`
2. **Remove from TokenManager**: Use `Option<ModelRecommender>` allows easy disabling
3. **Fallback to Default**: Handlers fall back to model override or default "312"
4. **No Breaking Changes**: Existing `get_token()` method remains unchanged

---

## Success Metrics (Story-024-02)

**Target Validation** (End of Week 3):
- ✅ Classification accuracy >80% (test suite validation)
- ✅ Cost savings 10-15% (production monitoring)
- ✅ Response time overhead <10ms (performance benchmarks)
- ✅ No regression in token acquisition success rate

---

## Next Steps

**Week 1 Deliverable** (Current): ✅ Interface design complete

**Week 2 Tasks**:
1. Implement `ComplexityAnalyzer` with all detectors
2. Implement `ModelRecommender` with cost tracking
3. Write comprehensive unit tests
4. Validate classification accuracy >80%

**Week 3 Tasks**:
1. Integrate with `TokenManager`
2. Update request handlers (openai.rs, claude.rs)
3. Add configuration options (backend + frontend)
4. Run integration tests and validate cost savings

---

## Related Documents

- **Story Definition**: `docs/epics/Epic-024-Flash-Base-Optimization.md`
- **Implementation Guide**: `docs/stories/Story-024-02-Implementation.md` (to be created Week 2)
- **Test Plan**: `docs/stories/Story-024-02-Testing.md` (to be created Week 2)
