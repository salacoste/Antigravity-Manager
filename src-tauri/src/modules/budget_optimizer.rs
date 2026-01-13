// Budget Optimizer Module - Story-025-01
// Adaptive budget allocation for Flash Thinking API calls
// Implements proven Epic-015 pattern with 16.4% cost savings

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Complexity tiers for token budget allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComplexityTier {
    Simple,   // 4096 tokens - short queries, simple tasks
    Moderate, // 12288 tokens - multi-step reasoning, moderate analysis
    Complex,  // 24576 tokens - deep analysis, extensive code generation
}

impl ComplexityTier {
    /// Get token budget for this tier
    pub fn budget(&self) -> u32 {
        match self {
            ComplexityTier::Simple => 4096,
            ComplexityTier::Moderate => 12288,
            ComplexityTier::Complex => 24576,
        }
    }

    /// Get tier name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            ComplexityTier::Simple => "simple",
            ComplexityTier::Moderate => "moderate",
            ComplexityTier::Complex => "complex",
        }
    }
}

/// Budget allocation result with confidence score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetAllocation {
    pub budget: u32,
    pub tier: ComplexityTier,
    pub confidence: f32,
    pub factors: HashMap<String, i32>,
}

impl BudgetAllocation {
    pub fn new(tier: ComplexityTier, confidence: f32, factors: HashMap<String, i32>) -> Self {
        Self {
            budget: tier.budget(),
            tier,
            confidence,
            factors,
        }
    }
}

/// Optimization metrics for tracking performance
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptimizationMetrics {
    pub total_requests: u64,
    pub simple_count: u64,
    pub moderate_count: u64,
    pub complex_count: u64,
    pub total_tokens_saved: u64,
    pub avg_confidence: f32,
}

impl OptimizationMetrics {
    /// Update metrics with new allocation
    pub fn record_allocation(&mut self, allocation: &BudgetAllocation) {
        self.total_requests += 1;

        match allocation.tier {
            ComplexityTier::Simple => self.simple_count += 1,
            ComplexityTier::Moderate => self.moderate_count += 1,
            ComplexityTier::Complex => self.complex_count += 1,
        }

        // Calculate tokens saved vs. always using Complex tier
        let max_budget = ComplexityTier::Complex.budget();
        self.total_tokens_saved += (max_budget - allocation.budget) as u64;

        // Update running average of confidence
        let prev_total = (self.total_requests - 1) as f32 * self.avg_confidence;
        self.avg_confidence = (prev_total + allocation.confidence) / self.total_requests as f32;
    }

    /// Calculate cost savings percentage
    pub fn savings_percentage(&self) -> f32 {
        if self.total_requests == 0 {
            return 0.0;
        }
        let max_possible = self.total_requests as u64 * ComplexityTier::Complex.budget() as u64;
        (self.total_tokens_saved as f32 / max_possible as f32) * 100.0
    }
}

/// Keyword detector for multi-step reasoning
struct KeywordDetector {
    multi_step_keywords: Vec<&'static str>,
    technical_terms: Vec<&'static str>,
}

impl KeywordDetector {
    fn new() -> Self {
        Self {
            multi_step_keywords: vec![
                "first",
                "then",
                "next",
                "after",
                "finally",
                "lastly",
                "step",
                "stage",
                "phase",
                "initially",
                "subsequently",
            ],
            technical_terms: vec![
                "function",
                "class",
                "method",
                "variable",
                "parameter",
                "return",
                "import",
                "export",
                "async",
                "await",
                "interface",
                "type",
                "struct",
                "enum",
                "trait",
                "algorithm",
                "optimization",
                "performance",
                "refactor",
                "database",
                "query",
                "schema",
                "migration",
                "index",
            ],
        }
    }

    /// Count multi-step reasoning indicators
    fn count_multi_step_keywords(&self, text: &str) -> usize {
        let text_lower = text.to_lowercase();
        self.multi_step_keywords
            .iter()
            .filter(|&&kw| text_lower.contains(kw))
            .count()
    }

    /// Count unique technical terms
    fn count_technical_terms(&self, text: &str) -> usize {
        let text_lower = text.to_lowercase();
        let mut found_terms = std::collections::HashSet::new();

        for &term in &self.technical_terms {
            if text_lower.contains(term) {
                found_terms.insert(term);
            }
        }

        found_terms.len()
    }
}

/// Code block detector
struct CodeBlockDetector;

impl CodeBlockDetector {
    /// Count code blocks in text (```...```)
    fn count_code_blocks(text: &str) -> usize {
        text.matches("```").count() / 2
    }

    /// Check if text contains code-like patterns
    fn has_code_patterns(text: &str) -> bool {
        // Simple heuristics for code detection
        text.contains("function ")
            || text.contains("const ")
            || text.contains("let ")
            || text.contains("class ")
            || text.contains("import ")
            || text.contains("def ")
            || text.contains("pub fn")
            || text.contains("async fn")
    }
}

/// Conversation history analyzer
struct HistoryAnalyzer;

impl HistoryAnalyzer {
    /// Analyze conversation history for complexity indicators
    fn analyze_history_length(messages: &[serde_json::Value]) -> i32 {
        // +1 complexity per 5 messages
        (messages.len() / 5) as i32
    }

    /// Check for cumulative context complexity
    fn has_high_context_complexity(messages: &[serde_json::Value]) -> bool {
        messages.len() > 10
    }
}

/// Main complexity classifier
pub struct ComplexityClassifier {
    keyword_detector: KeywordDetector,
}

impl ComplexityClassifier {
    pub fn new() -> Self {
        Self {
            keyword_detector: KeywordDetector::new(),
        }
    }

    /// Classify request complexity and return score breakdown
    pub fn classify(&self, request_text: &str, messages: &[serde_json::Value]) -> BudgetAllocation {
        let mut factors = HashMap::new();
        let mut complexity_score = 0;

        // Factor 1: Code blocks (+2 per block)
        let code_blocks = CodeBlockDetector::count_code_blocks(request_text);
        if code_blocks > 0 {
            let score = (code_blocks * 2) as i32;
            factors.insert("code_blocks".to_string(), score);
            complexity_score += score;
        }

        // Factor 2: Multi-step reasoning keywords (+1 per keyword)
        let multi_step_count = self
            .keyword_detector
            .count_multi_step_keywords(request_text);
        if multi_step_count > 0 {
            let score = multi_step_count as i32;
            factors.insert("multi_step_keywords".to_string(), score);
            complexity_score += score;
        }

        // Factor 3: Technical terms (>5 unique = +1)
        let tech_terms = self.keyword_detector.count_technical_terms(request_text);
        if tech_terms > 5 {
            factors.insert("technical_terms".to_string(), 1);
            complexity_score += 1;
        }

        // Factor 4: Conversation history length (+1 per 5 messages)
        let history_score = HistoryAnalyzer::analyze_history_length(messages);
        if history_score > 0 {
            factors.insert("conversation_history".to_string(), history_score);
            complexity_score += history_score;
        }

        // Factor 5: Code patterns bonus
        if CodeBlockDetector::has_code_patterns(request_text) && code_blocks == 0 {
            factors.insert("code_patterns".to_string(), 1);
            complexity_score += 1;
        }

        // Factor 6: High context complexity
        if HistoryAnalyzer::has_high_context_complexity(messages) {
            factors.insert("high_context".to_string(), 1);
            complexity_score += 1;
        }

        // Determine tier based on score
        let (tier, confidence) = self.score_to_tier(complexity_score, &factors);

        debug!(
            "Complexity classification: score={}, tier={:?}, confidence={:.2}, factors={:?}",
            complexity_score, tier, confidence, factors
        );

        BudgetAllocation::new(tier, confidence, factors)
    }

    /// Convert complexity score to tier with confidence
    fn score_to_tier(&self, score: i32, factors: &HashMap<String, i32>) -> (ComplexityTier, f32) {
        let factor_count = factors.len();

        // Scoring thresholds (adjusted for better classification)
        let (tier, base_confidence) = match score {
            0..=1 => (ComplexityTier::Simple, 0.90),
            2..=5 => (ComplexityTier::Moderate, 0.85),
            _ => (ComplexityTier::Complex, 0.88),
        };

        // Adjust confidence based on factor diversity
        let confidence_boost = (factor_count as f32 * 0.02).min(0.1);
        let final_confidence = (base_confidence + confidence_boost).min(0.99);

        (tier, final_confidence)
    }
}

impl Default for ComplexityClassifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Budget optimizer with metrics tracking
pub struct BudgetOptimizer {
    classifier: ComplexityClassifier,
    metrics: Arc<RwLock<OptimizationMetrics>>,
}

impl BudgetOptimizer {
    pub fn new() -> Self {
        Self {
            classifier: ComplexityClassifier::new(),
            metrics: Arc::new(RwLock::new(OptimizationMetrics::default())),
        }
    }

    /// Allocate budget for a request
    pub async fn allocate_budget(
        &self,
        request_text: &str,
        messages: &[serde_json::Value],
    ) -> BudgetAllocation {
        let allocation = self.classifier.classify(request_text, messages);

        // Record metrics
        let mut metrics = self.metrics.write().await;
        metrics.record_allocation(&allocation);

        info!(
            "Budget allocation: tier={}, budget={}, confidence={:.2}",
            allocation.tier.as_str(),
            allocation.budget,
            allocation.confidence
        );

        allocation
    }

    /// Get current optimization metrics
    pub async fn get_metrics(&self) -> OptimizationMetrics {
        self.metrics.read().await.clone()
    }

    /// Reset metrics (useful for testing)
    pub async fn reset_metrics(&self) {
        let mut metrics = self.metrics.write().await;
        *metrics = OptimizationMetrics::default();
    }
}

impl Default for BudgetOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_complexity_tiers() {
        assert_eq!(ComplexityTier::Simple.budget(), 4096);
        assert_eq!(ComplexityTier::Moderate.budget(), 12288);
        assert_eq!(ComplexityTier::Complex.budget(), 24576);
    }

    #[test]
    fn test_simple_query() {
        let classifier = ComplexityClassifier::new();
        let request = "What is the weather today?";
        let messages = vec![];

        let allocation = classifier.classify(request, &messages);

        assert_eq!(allocation.tier, ComplexityTier::Simple);
        assert!(allocation.confidence >= 0.85);
        assert_eq!(allocation.budget, 4096);
    }

    #[test]
    fn test_code_block_detection() {
        let classifier = ComplexityClassifier::new();
        let request = "Here is some code:\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let messages = vec![];

        let allocation = classifier.classify(request, &messages);

        assert!(allocation.factors.contains_key("code_blocks"));
        assert_eq!(allocation.factors["code_blocks"], 2); // 1 block * 2 points
        assert!(allocation.tier != ComplexityTier::Simple);
    }

    #[test]
    fn test_multi_step_reasoning() {
        let classifier = ComplexityClassifier::new();
        let request =
            "First, analyze the data. Then, process the results. Finally, generate a report.";
        let messages = vec![];

        let allocation = classifier.classify(request, &messages);

        assert!(allocation.factors.contains_key("multi_step_keywords"));
        assert!(allocation.factors["multi_step_keywords"] >= 3);
    }

    #[test]
    fn test_technical_terms() {
        let classifier = ComplexityClassifier::new();
        let request = "Optimize the algorithm using async functions, refactor the database query, and implement proper error handling with interfaces.";
        let messages = vec![];

        let allocation = classifier.classify(request, &messages);

        assert!(allocation.factors.contains_key("technical_terms"));
    }

    #[test]
    fn test_conversation_history() {
        let classifier = ComplexityClassifier::new();
        let request = "Continue with the analysis.";
        let messages: Vec<serde_json::Value> = (0..12)
            .map(|i| json!({ "role": "user", "content": format!("Message {}", i) }))
            .collect();

        let allocation = classifier.classify(request, &messages);

        assert!(allocation.factors.contains_key("conversation_history"));
        assert!(allocation.factors.contains_key("high_context"));
    }

    #[test]
    fn test_complex_request() {
        let classifier = ComplexityClassifier::new();
        let request = r#"
First, analyze this code:
```rust
async fn process_data(input: &str) -> Result<String, Error> {
    // Complex logic here
}
```
Then, refactor it to improve performance using async optimization.
Finally, add proper error handling and documentation.
"#;
        let messages: Vec<serde_json::Value> = (0..15)
            .map(|i| json!({ "role": "user", "content": format!("Message {}", i) }))
            .collect();

        let allocation = classifier.classify(request, &messages);

        assert_eq!(allocation.tier, ComplexityTier::Complex);
        assert!(allocation.factors.len() >= 3);
    }

    #[tokio::test]
    async fn test_metrics_tracking() {
        let optimizer = BudgetOptimizer::new();

        // Simulate different request types
        let simple_request = "Hello";
        let moderate_request = "First, do this. Then, do that.";
        let complex_request = "```rust\nfn test() {}\n```";

        let messages = vec![];

        optimizer.allocate_budget(simple_request, &messages).await;
        optimizer.allocate_budget(moderate_request, &messages).await;
        optimizer.allocate_budget(complex_request, &messages).await;

        let metrics = optimizer.get_metrics().await;

        assert_eq!(metrics.total_requests, 3);
        assert!(metrics.simple_count >= 1);
        assert!(metrics.total_tokens_saved > 0);
        assert!(metrics.avg_confidence > 0.0);
        assert!(metrics.savings_percentage() > 0.0);
    }

    #[tokio::test]
    async fn test_metrics_reset() {
        let optimizer = BudgetOptimizer::new();

        optimizer.allocate_budget("test", &vec![]).await;
        assert_eq!(optimizer.get_metrics().await.total_requests, 1);

        optimizer.reset_metrics().await;
        assert_eq!(optimizer.get_metrics().await.total_requests, 0);
    }

    #[test]
    fn test_savings_percentage() {
        let mut metrics = OptimizationMetrics::default();

        // Simulate 10 simple requests (4096 tokens each)
        for _ in 0..10 {
            let allocation = BudgetAllocation::new(ComplexityTier::Simple, 0.9, HashMap::new());
            metrics.record_allocation(&allocation);
        }

        // Expected savings: (24576 - 4096) * 10 / (24576 * 10) * 100
        // = 204800 / 245760 * 100 = 83.33%
        let savings = metrics.savings_percentage();
        assert!(savings > 80.0 && savings < 85.0);
    }
}
