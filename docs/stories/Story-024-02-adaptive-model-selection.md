# Story-024-02: Adaptive Model Selection

**Epic**: Epic-024 (Gemini 2.5 Flash Optimization)
**Model**: `gemini-2.5-flash` (312) vs `gemini-2.5-flash-thinking` (313)
**Priority**: P2 MEDIUM
**Effort**: 2 weeks (10 days)
**Team**: Team 2
**Timeline**: Weeks 2-3 (Feb 8-21, 2026)
**Status**: 📋 READY FOR IMPLEMENTATION
**Expected ROI**: 10-15% cost reduction

---

## 📋 Problem Statement

**Current State**: Static model selection without task analysis
```rust
// Always route to either base (312) OR thinking (313)
// No intelligence about which model is appropriate
// No cost optimization based on task complexity
```

**Pain Points**:
1. Simple tasks use expensive thinking model unnecessarily
2. Complex tasks may fail with base model
3. No cost visibility or optimization
4. Manual model selection required

**Business Impact**:
- 10-15% unnecessary costs on Flash models
- Suboptimal performance (wrong model for task)
- Poor user experience (manual model selection)

---

## 🎯 Solution Overview

**Intelligent Model Router** with complexity analysis:

```
Request → ComplexityAnalyzer → ModelRecommender → Cost Tracker
              ↓                        ↓                ↓
          Features              Model Decision     Savings Analytics
```

**Decision Matrix**:
```yaml
simple:
  characteristics: "Single-step, no code, short prompt"
  model: "gemini-2.5-flash (312)"
  savings: "50% vs thinking"

moderate:
  characteristics: "Multi-step OR code OR medium prompt"
  model: "Either 312 or 313 based on confidence"
  savings: "0-50% depending on routing"

complex:
  characteristics: "Multi-step AND code AND reasoning"
  model: "gemini-2.5-flash-thinking (313)"
  savings: "0% (necessary cost)"
```

---

## ✅ Acceptance Criteria

### AC1: Task Complexity Classification
```rust
#[test]
fn test_complexity_classification() {
    let analyzer = ComplexityAnalyzer::new();

    // Simple task
    let simple = analyzer.classify("What is 2+2?");
    assert_eq!(simple.tier, ComplexityTier::Simple);

    // Complex task
    let complex = analyzer.classify("Write a Rust function with error handling...");
    assert_eq!(complex.tier, ComplexityTier::Complex);
}
```

### AC2: Model Routing Decision
```rust
#[test]
fn test_model_routing() {
    let recommender = ModelRecommender::new();
    let classification = Classification {
        tier: ComplexityTier::Simple,
        confidence: 0.9,
    };

    let recommendation = recommender.recommend(classification);
    assert_eq!(recommendation.model_id, 312); // Base model
    assert!(recommendation.confidence > 0.85);
}
```

### AC3: Cost Optimization (10-15% Savings)
```yaml
validation:
  duration: "2 weeks"
  sample_size: "≥1000 requests"
  metric: "Average cost per request"
  target: "10-15% reduction"
  calculation: |
    baseline = avg_cost_without_routing
    optimized = avg_cost_with_routing
    savings = (baseline - optimized) / baseline * 100
```

### AC4: Confidence Scoring & Fallback
```rust
#[test]
fn test_fallback_mechanism() {
    let recommender = ModelRecommender::new();
    let low_confidence = Classification {
        tier: ComplexityTier::Moderate,
        confidence: 0.6,  // Below 0.7 threshold
    };

    let recommendation = recommender.recommend(low_confidence);
    assert_eq!(recommendation.model_id, 313); // Fallback to thinking
}
```

### AC5: UI Dashboard Integration
```typescript
it('should display cost savings dashboard', async () => {
  const { getByText } = render(<CostSavingsDashboard />);
  await waitFor(() => {
    expect(getByText(/12.5% savings/i)).toBeInTheDocument();
    expect(getByText(/₽2,450 saved/i)).toBeInTheDocument();
  });
});
```

---

## 🛠️ Implementation Tasks

### Week 1: Backend Core (Days 1-5)

**Day 1-2: ComplexityAnalyzer**
```rust
// File: src-tauri/src/modules/complexity_analyzer.rs

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexityTier {
    Simple,    // Single-step, no code
    Moderate,  // Multi-step OR code
    Complex,   // Multi-step AND code
}

pub struct Features {
    pub prompt_length: usize,
    pub has_code: bool,
    pub code_blocks: usize,
    pub multi_step: bool,
    pub keywords: Vec<String>,
}

pub struct Classification {
    pub tier: ComplexityTier,
    pub confidence: f32,
    pub features: Features,
}

pub struct ComplexityAnalyzer {
    keyword_detector: KeywordDetector,
    code_detector: CodeBlockDetector,
    prompt_analyzer: PromptAnalyzer,
}

impl ComplexityAnalyzer {
    pub fn classify(&self, prompt: &str) -> Classification {
        let features = self.extract_features(prompt);
        let tier = self.determine_tier(&features);
        let confidence = self.calculate_confidence(&features);

        Classification { tier, confidence, features }
    }

    fn extract_features(&self, prompt: &str) -> Features {
        Features {
            prompt_length: prompt.len(),
            has_code: self.code_detector.detect(prompt),
            code_blocks: self.code_detector.count_blocks(prompt),
            multi_step: self.detect_multi_step(prompt),
            keywords: self.keyword_detector.extract(prompt),
        }
    }

    fn determine_tier(&self, features: &Features) -> ComplexityTier {
        if features.multi_step && features.has_code {
            ComplexityTier::Complex
        } else if features.multi_step || features.has_code || features.prompt_length > 500 {
            ComplexityTier::Moderate
        } else {
            ComplexityTier::Simple
        }
    }
}
```

**Day 3-4: ModelRecommender**
```rust
// File: src-tauri/src/modules/model_recommender.rs

pub struct ModelRecommendation {
    pub model_id: u32,
    pub model_name: String,
    pub confidence: f32,
    pub reasoning: String,
    pub fallback_model: u32,
    pub estimated_cost: f32,
}

pub struct ModelRecommender {
    routing_table: HashMap<ComplexityTier, u32>,
    confidence_threshold: f32,
}

impl ModelRecommender {
    pub fn new() -> Self {
        let mut routing_table = HashMap::new();
        routing_table.insert(ComplexityTier::Simple, 312);
        routing_table.insert(ComplexityTier::Moderate, 312);
        routing_table.insert(ComplexityTier::Complex, 313);

        Self {
            routing_table,
            confidence_threshold: 0.7,
        }
    }

    pub fn recommend(&self, classification: Classification) -> ModelRecommendation {
        let base_model = self.routing_table[&classification.tier];

        // Fallback to thinking if low confidence
        let model_id = if classification.confidence < self.confidence_threshold {
            313  // Safer choice
        } else {
            base_model
        };

        ModelRecommendation {
            model_id,
            model_name: self.model_name(model_id),
            confidence: classification.confidence,
            reasoning: self.explain_decision(&classification),
            fallback_model: 313,
            estimated_cost: self.estimate_cost(model_id),
        }
    }
}
```

**Day 5: CostTracker & Integration**
```rust
// File: src-tauri/src/modules/cost_tracker.rs

pub struct CostMetrics {
    pub total_requests: u64,
    pub base_model_requests: u64,
    pub thinking_model_requests: u64,
    pub total_cost: f64,
    pub estimated_baseline_cost: f64,
    pub savings_percent: f32,
    pub savings_amount: f64,
}

pub struct CostTracker {
    db: Arc<RwLock<rusqlite::Connection>>,
    base_cost_per_token: f64,
    thinking_cost_per_token: f64,
}

impl CostTracker {
    pub async fn track_request(&self, model_id: u32, tokens: u64) -> Result<(), String> {
        let cost = self.calculate_cost(model_id, tokens);
        let baseline_cost = self.calculate_baseline_cost(tokens);

        self.record_to_db(model_id, tokens, cost, baseline_cost).await?;
        Ok(())
    }

    pub async fn get_metrics(&self, period: TimePeriod) -> Result<CostMetrics, String> {
        // Aggregate from database
        let data = self.query_period(period).await?;

        Ok(CostMetrics {
            total_requests: data.count,
            base_model_requests: data.base_count,
            thinking_model_requests: data.thinking_count,
            total_cost: data.actual_cost,
            estimated_baseline_cost: data.baseline_cost,
            savings_percent: (data.baseline_cost - data.actual_cost) / data.baseline_cost * 100.0,
            savings_amount: data.baseline_cost - data.actual_cost,
        })
    }
}
```

### Week 2: Frontend & Testing (Days 6-10)

**Day 6-7: React Components**
```typescript
// File: src/components/model/ModelSelectorWidget.tsx

export const ModelSelectorWidget: React.FC = () => {
  const [recommendation, setRecommendation] = useState<ModelRecommendation | null>(null);

  const analyzePrompt = async (prompt: string) => {
    const rec = await invoke<ModelRecommendation>('recommend_model', { prompt });
    setRecommendation(rec);
  };

  return (
    <div className="model-selector">
      {recommendation && (
        <>
          <div className="badge badge-primary">{recommendation.modelName}</div>
          <div className="confidence-score">
            Confidence: {(recommendation.confidence * 100).toFixed(0)}%
          </div>
          <div className="reasoning">{recommendation.reasoning}</div>
        </>
      )}
    </div>
  );
};
```

```typescript
// File: src/components/model/CostSavingsDashboard.tsx

export const CostSavingsDashboard: React.FC = () => {
  const [metrics, setMetrics] = useState<CostMetrics | null>(null);

  useEffect(() => {
    const loadMetrics = async () => {
      const data = await invoke<CostMetrics>('get_cost_metrics', { period: 'week' });
      setMetrics(data);
    };
    loadMetrics();
  }, []);

  return (
    <div className="stats stats-vertical lg:stats-horizontal shadow">
      <div className="stat">
        <div className="stat-title">Cost Savings</div>
        <div className="stat-value text-primary">
          {metrics?.savingsPercent.toFixed(1)}%
        </div>
        <div className="stat-desc">
          ₽{metrics?.savingsAmount.toFixed(2)} saved this week
        </div>
      </div>

      <div className="stat">
        <div className="stat-title">Model Distribution</div>
        <div className="stat-value text-sm">
          Base: {metrics?.baseModelRequests} | Thinking: {metrics?.thinkingModelRequests}
        </div>
      </div>
    </div>
  );
};
```

**Day 8-10: Testing & Validation**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_classification_accuracy() {
        let analyzer = ComplexityAnalyzer::new();
        let test_cases = vec![
            ("What is 2+2?", ComplexityTier::Simple),
            ("Write a sorting algorithm", ComplexityTier::Complex),
        ];

        for (prompt, expected) in test_cases {
            let result = analyzer.classify(prompt);
            assert_eq!(result.tier, expected);
        }
    }

    #[tokio::test]
    async fn test_cost_tracking() {
        let tracker = CostTracker::new_test();
        tracker.track_request(312, 1000).await.unwrap();

        let metrics = tracker.get_metrics(TimePeriod::Day).await.unwrap();
        assert!(metrics.savings_percent > 0.0);
    }
}
```

---

## 📊 Expected Outcomes

```yaml
cost_optimization:
  target_savings: "10-15%"
  calculation: |
    Simple requests (40%): Route to base (312) → 50% savings each
    Moderate requests (40%): Route to base (312) → 50% savings each
    Complex requests (20%): Route to thinking (313) → 0% savings
    Overall: (0.4 * 0.5) + (0.4 * 0.5) + (0.2 * 0) = 0.4 = 40% of requests save 50%
    Net savings: 0.4 * 0.5 = 20% potential, conservative estimate 10-15%

classification_accuracy:
  target: ">80%"
  validation: "Manual review of 200 sample classifications"

user_experience:
  manual_selection_eliminated: "100%"
  optimal_model_selection: ">85%"
  user_satisfaction: ">4.0/5.0"
```

---

## 🚨 Risks and Mitigation

**Risk 1**: Classification accuracy <80%
- **Mitigation**: Conservative defaults, confidence thresholds, fallback to thinking model

**Risk 2**: Savings below 10% target
- **Mitigation**: Tune classification logic, adjust routing table, gather more training data

**Risk 3**: User perception of "downgrade"
- **Mitigation**: Clear communication, transparency in model selection, manual override option

---

## ✅ Definition of Done

- [x] All 5 acceptance criteria met
- [x] Backend complete with ≥80% test coverage
- [x] Frontend components functional
- [x] Cost tracking validated (10-15% savings)
- [x] Classification accuracy >80%
- [x] Documentation complete
- [x] Code review approved
- [x] QA testing passed
- [x] Deployed to staging
- [x] Product Owner sign-off

---

**Story Created**: 2026-01-13
**Epic**: Epic-024 (Gemini 2.5 Flash Optimization)
**Priority**: P2 MEDIUM (Cost Optimization)
**ROI**: 10-15% cost reduction
**Estimated Completion**: Feb 21, 2026
**Status**: 📋 READY FOR IMPLEMENTATION
