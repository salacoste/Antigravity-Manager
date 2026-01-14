# Story-025-02: Budget Optimizer Integration Guide

**For**: Developer 2 (Frontend Lead, Team 1)
**Week**: 2 (Feb 8-14, 2026)
**Prerequisites**: Story-025-01 complete ✅

## Quick Start

### 1. Add BudgetOptimizer to AppState

**File**: `src-tauri/src/proxy/server.rs`

```rust
use crate::modules::budget_optimizer::BudgetOptimizer;

pub struct AppState {
    pub upstream: Arc<UpstreamClient>,
    pub token_manager: Arc<TokenManager>,
    pub budget_optimizer: Arc<BudgetOptimizer>, // ADD THIS
    // ... other fields
}

// In server initialization
let budget_optimizer = Arc::new(BudgetOptimizer::new());

let state = AppState {
    upstream,
    token_manager,
    budget_optimizer, // ADD THIS
    // ... other fields
};
```

### 2. Integrate in gemini.rs Handler

**File**: `src-tauri/src/proxy/handlers/gemini.rs`

**Location**: Inside `handle_generate()` function, before API call

```rust
pub async fn handle_generate(
    State(state): State<AppState>,
    Path(model_action): Path<String>,
    Json(body): Json<Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // ... existing code ...

    // STEP 1: Extract request text for classification
    let request_text = extract_request_text(&body);

    // STEP 2: Extract conversation history
    let messages = body
        .get("contents")
        .and_then(|c| c.as_array())
        .cloned()
        .unwrap_or_default();

    // STEP 3: Allocate budget
    let allocation = state.budget_optimizer
        .allocate_budget(&request_text, &messages)
        .await;

    // STEP 4: Log allocation details
    info!(
        "Budget allocation: tier={}, budget={}, confidence={:.2}, factors={:?}",
        allocation.tier.as_str(),
        allocation.budget,
        allocation.confidence,
        allocation.factors
    );

    // STEP 5: Apply budget to request
    let thinking_config = if mapped_model.contains("flash-thinking") {
        Some(json!({
            "thinkingTokenBudget": allocation.budget, // USE HERE
            "thinkingStyle": "gemini_flash_thinking_adaptive"
        }))
    } else {
        None
    };

    // ... rest of handler code ...
}

// HELPER FUNCTION: Extract request text
fn extract_request_text(body: &Value) -> String {
    body.get("contents")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.last())
        .and_then(|last| last.get("parts"))
        .and_then(|parts| parts.as_array())
        .and_then(|parts_arr| parts_arr.first())
        .and_then(|part| part.get("text"))
        .and_then(|text| text.as_str())
        .unwrap_or("")
        .to_string()
}
```

### 3. Add Metrics Collection

**Add to your metrics tracking**:

```rust
// After receiving API response
let actual_tokens_used = response.usage_metadata.thinking_tokens_used;
let tokens_allocated = allocation.budget;

// Log efficiency
info!(
    "Token efficiency: used={}, allocated={}, efficiency={:.2}%",
    actual_tokens_used,
    tokens_allocated,
    (actual_tokens_used as f32 / tokens_allocated as f32) * 100.0
);

// Store for Week 3 A/B testing
if let Some(monitor) = &state.performance_monitor {
    monitor.record_budget_efficiency(
        allocation.tier,
        tokens_allocated,
        actual_tokens_used,
    ).await;
}
```

## Testing Checklist

### Unit Tests
```bash
cd src-tauri
cargo test --lib budget_optimizer
cargo test --lib gemini::tests
```

### Integration Tests
```bash
# Test simple query
curl -X POST http://localhost:8045/v1/models/gemini-2.0-flash-thinking-exp-01-21:generateContent \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [{
      "parts": [{"text": "What is 2+2?"}]
    }]
  }'

# Expected: Budget allocation with Simple tier (4096 tokens)

# Test code review
curl -X POST http://localhost:8045/v1/models/gemini-2.0-flash-thinking-exp-01-21:generateContent \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [{
      "parts": [{"text": "Review this code:\n```rust\nfn main() {}\n```"}]
    }]
  }'

# Expected: Budget allocation with Moderate tier (12288 tokens)
```

### Validation Metrics
Monitor for Week 2:
1. **Classification accuracy**: Compare allocated budget vs actual usage
2. **Cost savings**: Track total tokens saved vs. baseline (24K always)
3. **Response times**: Verify no performance degradation
4. **Error rates**: Ensure budget limits don't cause failures

## Common Integration Issues

### Issue 1: Budget optimizer not in state
**Error**: `state.budget_optimizer` not found
**Fix**: Add `budget_optimizer: Arc<BudgetOptimizer>` to AppState

### Issue 2: Request text extraction fails
**Error**: Empty request text, always Simple tier
**Fix**: Verify JSON structure matches Gemini format

### Issue 3: Metrics not tracking
**Error**: Metrics show 0 requests
**Fix**: Ensure `allocate_budget()` is called before API request

## Performance Expectations

| Metric | Expected Value |
|--------|----------------|
| Classification Time | <1ms |
| Memory Overhead | <1MB |
| Accuracy | >80% |
| Cost Savings | 20-30% overall |

## Week 2 Success Criteria

✅ Budget optimizer integrated in gemini.rs handler
✅ Budget allocated for every Flash Thinking request
✅ Metrics collected (allocated vs actual tokens)
✅ Logs show tier, confidence, factors for each request
✅ No performance degradation (<5% latency increase)
✅ Zero budget-related API errors

## Next Steps (Week 3)

Week 3 will focus on:
1. A/B testing validation
2. Fine-tuning thresholds based on real usage
3. Adding historical learning
4. Performance optimization

## Support

If you encounter issues:
1. Check `docs/epics/Story-025-01-Implementation.md` for detailed docs
2. Review unit tests in `budget_optimizer.rs` for usage examples
3. Test standalone with `commands/budget.rs` Tauri commands

## Quick Reference

**Module**: `crate::modules::budget_optimizer`
**Commands**: `crate::commands::budget`
**State**: `Arc<BudgetOptimizer>` (thread-safe)

**Key Functions**:
- `allocate_budget(text, messages) -> BudgetAllocation`
- `get_metrics() -> OptimizationMetrics`

**Data Structures**:
- `ComplexityTier`: Simple, Moderate, Complex
- `BudgetAllocation`: { budget, tier, confidence, factors }
- `OptimizationMetrics`: { total_requests, savings, etc. }

Good luck with Week 2 integration! 🚀
