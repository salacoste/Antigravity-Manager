# Thinking Quality Monitoring System

**Epic-025 Story-025-04**: Comprehensive quality scoring and continuous improvement feedback loop

## Quick Start

```rust
use crate::modules::thinking_quality::ThinkingQualityMonitor;
use crate::modules::budget_detector::FinishReason;

// Initialize monitor
let monitor = ThinkingQualityMonitor::new();

// Analyze request/response quality
let analysis = monitor.analyze_quality(
    "req-123".to_string(),
    4500,  // thinking_tokens
    1200,  // output_tokens
    6000,  // thinking_budget
    FinishReason::Stop,
    0,     // escalation_count
).await;

// Get current metrics
let metrics = monitor.get_metrics().await;
println!("Overall quality: {:.1}%", metrics.average_quality_score * 100.0);
println!("FTR rate: {:.1}%",
    (metrics.first_time_right as f64 / metrics.total_requests as f64) * 100.0);

// Get weekly tuning recommendations
let feedback = monitor.get_weekly_feedback(7).await?;
println!("Priority: {:?}", feedback.tuning_priority);
for rec in feedback.recommendations {
    println!("- {}", rec);
}
```

## Architecture

### Three-Layer Quality Scoring

#### 1. Efficiency Score (0.0-1.0)
**Weight**: 30%

Measures budget utilization efficiency:
- **Optimal**: 75-95% utilization → 1.0
- **Under-utilized**: <75% → Linear penalty
- **Over-utilized**: >95% → Penalty for hitting limit

```rust
if utilization >= 0.75 && utilization <= 0.95 {
    1.0  // Perfect efficiency
} else if utilization < 0.75 {
    utilization / 0.75  // Wasting budget
} else {
    1.0 - ((utilization - 0.95) / 0.05) * 0.5  // Maxing out
}
```

#### 2. Completeness Score (0.0-1.0)
**Weight**: 40% (highest - most critical for user experience)

Based on finish reason:
- **Stop (<98% util)**: 1.0 - Perfect completion
- **Stop (>98% util)**: 0.95 - Nearly exhausted but finished
- **MaxTokens**: 0.3 - Truncated response (major issue)
- **Safety/Recitation**: 0.5 - Blocked (not completeness issue)

#### 3. Coherence Score (0.0-1.0)
**Weight**: 30%

Thinking/output token balance:
- **Optimal (20-70%)**: 1.0 - Good reasoning balance
- **Acceptable (10-90%)**: 0.9 - Slightly imbalanced
- **Too little (<10%)**: 0.6 - Insufficient reasoning
- **Too much (>90%)**: 0.7 - Over-thinking or truncated output
- **No thinking**: 0.5 - Might be appropriate for simple queries

### Overall Score Calculation
```rust
overall = efficiency * 0.3 + completeness * 0.4 + coherence * 0.3
```

## Key Metrics

### First-Time-Right (FTR) Rate
**Target**: >90%

Percentage of requests that complete successfully without needing budget escalation.

```rust
FTR = (escalation_count == 0 && finish_reason == Stop)
FTR_rate = first_time_right / total_requests
```

**Thresholds**:
- <80%: Critical - Severe under-allocation
- <90%: High - Below target
- <95%: Medium - Acceptable but improvable
- ≥95%: Low - Optimal

### Budget Utilization
**Target**: 75-95%

Average thinking token usage as percentage of allocated budget.

**Why this range?**
- <75%: Over-allocating, wasting budget → Higher costs
- 75-95%: Optimal - Good utilization without hitting limits
- >95%: Frequently maxing out → Responses likely truncated

## Tuning Recommendations

### Priority Levels

#### Critical (Red Alert)
**Triggers**:
- FTR rate <80%
- Budget utilization >98%

**Actions**:
- Increase default budgets by 25-50%
- Immediate investigation required

#### High (Warning)
**Triggers**:
- FTR rate <90%
- Budget utilization <70% or >95%

**Actions**:
- Increase budgets by 15-25% (if high utilization)
- Decrease budgets by 10-20% (if low utilization)

#### Medium (Advisory)
**Triggers**:
- FTR rate <95%
- Budget utilization outside 75-95%

**Actions**:
- Fine-tune budgets by 5-10%

#### Low (Optimal)
**Conditions**:
- FTR rate ≥95%
- Budget utilization 75-95%
- All quality scores >0.8

**Actions**: No tuning needed

### Example Recommendations

```
⚠️ CRITICAL: First-time-right rate very low (75.0%). Budget classifier
is under-allocating. Increase default budgets by 25-50%.

⚠️ Budget utilization high (96.0% > 95%). Budgets are frequently at
limit. Increase by 15-25%.

Budget under-utilized (68.0% < 75%). Consider decreasing budgets by
10-20% to save costs.

✅ Budget utilization optimal (85.0%). Classifier is well-tuned.
```

## Database Schema

```sql
CREATE TABLE thinking_quality_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    request_id TEXT UNIQUE NOT NULL,
    timestamp INTEGER NOT NULL,
    thinking_tokens INTEGER NOT NULL,
    output_tokens INTEGER NOT NULL,
    thinking_budget INTEGER NOT NULL,
    budget_utilization REAL NOT NULL,
    efficiency_score REAL NOT NULL,
    completeness_score REAL NOT NULL,
    coherence_score REAL NOT NULL,
    overall_score REAL NOT NULL,
    first_time_right INTEGER NOT NULL,
    escalation_count INTEGER NOT NULL,
    finish_reason TEXT NOT NULL,
    user_rating REAL
);
```

### Indexes
- `idx_quality_metrics_timestamp`: Time-range queries
- `idx_quality_metrics_first_time_right`: FTR rate calculations
- `idx_quality_metrics_overall_score`: Quality analysis

## API (Tauri Commands)

```typescript
// Get current metrics
const metrics = await invoke<QualityMetrics>('get_quality_metrics');

// Get weekly feedback (default 7 days)
const feedback = await invoke<WeeklyFeedback>('get_weekly_feedback', {
    days: 7
});

// Get quality history
const history = await invoke<QualityAnalysis[]>('get_quality_history', {
    limit: 100
});

// Submit user rating (0.0-5.0)
await invoke('submit_user_rating', {
    request_id: 'req-123',
    rating: 4.5
});

// Reset metrics (for testing)
await invoke('reset_quality_metrics');
```

## Integration Guide

### Step 1: Analyze Quality After Response

```rust
// In gemini.rs handler after receiving response
use crate::commands::quality::QualityMonitorState;

let quality_state = app.state::<QualityMonitorState>();
quality_state.ensure_initialized().await;

let monitor_lock = quality_state.monitor.read().await;
if let Some(monitor) = monitor_lock.as_ref() {
    let analysis = monitor.analyze_quality(
        request_id,
        response_metadata.thinking_tokens,
        response_metadata.output_tokens,
        request.thinking_budget,
        response_metadata.finish_reason,
        escalation_count,
    ).await;

    // Persist to database
    crate::modules::proxy_db::save_quality_analysis(&analysis)?;
}
```

### Step 2: Weekly Tuning Automation

```rust
// Scheduled job (e.g., every Sunday at midnight)
async fn weekly_tuning_job(app: &AppHandle) -> Result<(), String> {
    let quality_state = app.state::<QualityMonitorState>();

    let monitor_lock = quality_state.monitor.read().await;
    let monitor = monitor_lock.as_ref().ok_or("Monitor not initialized")?;

    let feedback = monitor.get_weekly_feedback(7).await?;

    match feedback.tuning_priority {
        TuningPriority::Critical => {
            // Increase budgets by 25-50%
            adjust_budget_classifier(0.30)?;
            alert_admin("Critical quality issue - budgets increased 30%")?;
        }
        TuningPriority::High => {
            // Adjust by 15-25%
            let adjustment = if feedback.avg_budget_utilization > 0.95 {
                0.20  // Increase
            } else {
                -0.15  // Decrease
            };
            adjust_budget_classifier(adjustment)?;
        }
        TuningPriority::Medium => {
            // Fine-tune by 5-10%
            let adjustment = calculate_fine_tune_adjustment(&feedback);
            adjust_budget_classifier(adjustment)?;
        }
        TuningPriority::Low => {
            // No action needed
            log::info!("Quality metrics optimal - no tuning needed");
        }
    }

    Ok(())
}
```

## Performance Characteristics

### Memory
- Monitor state: ~8KB
- Running metrics: 144 bytes
- Per analysis: 240 bytes

### Computational Cost
- Quality analysis: <1ms (pure computation)
- Weekly aggregation: <100ms for 10K records
- Recommendation generation: <1ms

### Database
- Write: <10ms per analysis
- Weekly read: <50ms with timestamp index
- Storage: ~500 bytes per analysis

## Testing

```bash
# Run all quality tests
cargo test --lib thinking_quality

# Test coverage
# - test_completeness_scoring: 6 scenarios
# - test_coherence_scoring: 5 scenarios
# - test_efficiency_scoring: 4 scenarios
# - test_first_time_right: 3 scenarios
# - test_tuning_priority: 3 priority levels
```

## Configuration

Currently hardcoded optimal thresholds based on Epic-025 research:
- Efficiency optimal range: 75-95%
- FTR target: >90%
- Completeness threshold: >98% = nearly exhausted
- Coherence optimal ratio: 20-70%

Future enhancement: Make thresholds configurable per deployment.

## Roadmap

### Week 6 (Integration)
- [ ] Integrate with Gemini handler
- [ ] Real-time quality tracking
- [ ] Quality monitoring toggle in proxy config

### Week 7 (Automation & Polish)
- [ ] Automated weekly tuning
- [ ] Quality history charts
- [ ] User rating UI
- [ ] Export quality reports

### Post-MVP
- [ ] Per-model quality analytics
- [ ] A/B testing support
- [ ] ML-based anomaly detection
- [ ] Cost-quality tradeoff visualization

## Troubleshooting

### Low Quality Scores

**Symptom**: Overall quality <0.7

**Causes**:
1. Budget classifier under-allocating → FTR rate low
2. Frequent truncation → Completeness score low
3. Imbalanced thinking/output → Coherence score low

**Solutions**:
1. Increase default budgets
2. Review escalation patterns
3. Check model-specific issues

### High Escalation Rate

**Symptom**: FTR rate <80%

**Causes**:
1. Initial budgets too conservative
2. Model complexity underestimated
3. User query patterns changed

**Solutions**:
1. Follow Critical priority recommendations
2. Increase budgets by 25-50%
3. Re-train budget classifier

### Budget Waste

**Symptom**: Utilization consistently <70%

**Causes**:
1. Over-allocation from previous tuning
2. Query complexity decreased
3. Model efficiency improved

**Solutions**:
1. Follow High priority recommendations
2. Decrease budgets by 10-20%
3. Monitor cost savings

## References

- Epic-025 Story-025-04: Thinking Quality Monitoring
- `src-tauri/src/modules/thinking_quality.rs`: Implementation
- `src-tauri/src/commands/quality.rs`: Tauri commands
- `src/pages/QualityDashboardPage.tsx`: UI dashboard
- `STORY-025-04-WEEK5-SUMMARY.md`: Week 5 implementation summary
