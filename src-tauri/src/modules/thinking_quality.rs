// Epic-025 Story-025-04: Thinking Quality Monitoring System
//
// Purpose: Track thinking quality metrics to enable budget classifier optimization
//          through comprehensive efficiency, completeness, and coherence scoring
//
// Success Metrics:
// - Quality score accuracy: >85%
// - First-time-right rate: >90%
// - Budget utilization: 75-95% (optimal range)
// - Weekly tuning: Automated recommendations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::modules::budget_detector::FinishReason;

// ========== Core Types ==========

/// Quality analysis result for a single request/response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAnalysis {
    pub request_id: String,
    pub timestamp: DateTime<Utc>,

    // Individual scores (0.0-1.0)
    pub efficiency_score: f64,
    pub completeness_score: f64,
    pub coherence_score: f64,
    pub overall_score: f64,

    // Metrics
    pub thinking_tokens: u32,
    pub output_tokens: u32,
    pub thinking_budget: u32,
    pub budget_utilization: f64, // 0.0-1.0

    // Quality indicators
    pub first_time_right: bool,
    pub escalation_count: u32,
    pub finish_reason: String,

    // Optional user feedback
    pub user_rating: Option<f64>,
}

/// Aggregated weekly feedback for classifier tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyFeedback {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    pub total_requests: usize,
    pub avg_quality_score: f64,
    pub first_time_right_rate: f64,
    pub avg_budget_utilization: f64,

    // Score breakdowns
    pub avg_efficiency: f64,
    pub avg_completeness: f64,
    pub avg_coherence: f64,

    // Recommendations for classifier tuning
    pub recommendations: Vec<String>,
    pub tuning_priority: TuningPriority,
}

/// Priority level for classifier tuning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TuningPriority {
    Critical, // FTR <80% or utilization >98%
    High,     // FTR <90% or utilization <70% or >95%
    Medium,   // FTR <95% or utilization outside 75-95%
    Low,      // All metrics within optimal range
}

/// Overall quality metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QualityMetrics {
    pub total_requests: u64,
    pub first_time_right: u64,
    pub escalations_needed: u64,

    pub average_quality_score: f64,
    pub average_efficiency: f64,
    pub average_completeness: f64,
    pub average_coherence: f64,
    pub average_budget_utilization: f64,

    // Last updated timestamp
    pub last_updated: Option<i64>,
}

// ========== Quality Scorer ==========

/// Scores response quality on multiple dimensions
pub struct QualityScorer;

impl QualityScorer {
    pub fn new() -> Self {
        Self
    }

    /// Score response completeness (0.0-1.0)
    /// Based on finish_reason and whether response appears truncated
    pub fn score_completeness(
        &self,
        finish_reason: &FinishReason,
        thinking_tokens: u32,
        thinking_budget: u32,
    ) -> f64 {
        match finish_reason {
            FinishReason::Stop => {
                // Complete response - check if budget was exhausted
                let utilization = thinking_tokens as f64 / thinking_budget as f64;
                if utilization > 0.98 {
                    // Nearly exhausted budget but finished - slightly penalize
                    0.95
                } else {
                    1.0
                }
            }
            FinishReason::MaxTokens => {
                // Truncated response - major completeness issue
                0.3
            }
            FinishReason::Safety | FinishReason::Recitation => {
                // Safety/recitation block - not a completeness issue
                0.5
            }
            FinishReason::Other | FinishReason::Unspecified => {
                // Unknown reason - moderate penalty
                0.6
            }
        }
    }

    /// Score response coherence (0.0-1.0)
    /// Based on thinking/output token balance
    pub fn score_coherence(&self, thinking_tokens: u32, output_tokens: u32) -> f64 {
        if thinking_tokens == 0 && output_tokens == 0 {
            return 0.0;
        }

        if thinking_tokens == 0 {
            // No thinking tokens - might be appropriate for simple queries
            return 0.5;
        }

        let total_tokens = thinking_tokens + output_tokens;
        let thinking_ratio = thinking_tokens as f64 / total_tokens as f64;

        // Optimal thinking ratio: 10-90%
        // Too little thinking (<10%): likely insufficient reasoning
        // Too much thinking (>90%): likely inefficient or truncated output
        if (0.1..=0.9).contains(&thinking_ratio) {
            // Within optimal range
            if (0.2..=0.7).contains(&thinking_ratio) {
                // Sweet spot (20-70%)
                1.0
            } else {
                // Acceptable but not ideal
                0.9
            }
        } else if thinking_ratio < 0.1 {
            // Too little thinking
            0.6
        } else {
            // Too much thinking relative to output
            0.7
        }
    }
}

// ========== Efficiency Tracker ==========

/// Tracks budget efficiency and first-time-right rate
pub struct EfficiencyTracker {
    optimal_range: (f64, f64), // (0.75, 0.95)
}

impl EfficiencyTracker {
    pub fn new() -> Self {
        Self {
            optimal_range: (0.75, 0.95),
        }
    }

    /// Calculate budget efficiency score (0.0-1.0)
    pub fn calculate_efficiency(&self, thinking_tokens: u32, thinking_budget: u32) -> f64 {
        if thinking_budget == 0 {
            return 0.0;
        }

        let utilization = thinking_tokens as f64 / thinking_budget as f64;

        if utilization >= self.optimal_range.0 && utilization <= self.optimal_range.1 {
            // Optimal range (75-95%) - perfect score
            1.0
        } else if utilization < self.optimal_range.0 {
            // Under-utilized - linear penalty below 75%
            // 0% utilization = 0.0, 75% = 1.0
            utilization / self.optimal_range.0
        } else {
            // Over-utilized (>95%) - hitting limit is suboptimal
            // 95% = 1.0, 100% = 0.5
            let overage = utilization - self.optimal_range.1;
            let penalty = overage / (1.0 - self.optimal_range.1);
            1.0 - (penalty * 0.5)
        }
    }

    /// Check if response was first-time-right (no escalation needed)
    pub fn is_first_time_right(&self, finish_reason: &FinishReason, escalation_count: u32) -> bool {
        escalation_count == 0 && *finish_reason == FinishReason::Stop
    }
}

// ========== Feedback Aggregator ==========

/// Aggregates quality data for weekly tuning recommendations
pub struct FeedbackAggregator;

impl FeedbackAggregator {
    pub fn new() -> Self {
        Self
    }

    /// Aggregate weekly feedback from recent quality data
    pub async fn aggregate_weekly_feedback(&self, days: u32) -> Result<WeeklyFeedback, String> {
        let now = Utc::now();
        let period_start = now - chrono::Duration::days(days as i64);

        // Load quality analyses from database
        let analyses = self
            .load_quality_analyses_since(period_start.timestamp())
            .await?;

        if analyses.is_empty() {
            return Ok(WeeklyFeedback {
                period_start,
                period_end: now,
                total_requests: 0,
                avg_quality_score: 0.0,
                first_time_right_rate: 0.0,
                avg_budget_utilization: 0.0,
                avg_efficiency: 0.0,
                avg_completeness: 0.0,
                avg_coherence: 0.0,
                recommendations: vec!["Insufficient data for recommendations".to_string()],
                tuning_priority: TuningPriority::Low,
            });
        }

        let total = analyses.len() as f64;

        // Calculate averages
        let avg_quality_score = analyses.iter().map(|a| a.overall_score).sum::<f64>() / total;
        let avg_efficiency = analyses.iter().map(|a| a.efficiency_score).sum::<f64>() / total;
        let avg_completeness = analyses.iter().map(|a| a.completeness_score).sum::<f64>() / total;
        let avg_coherence = analyses.iter().map(|a| a.coherence_score).sum::<f64>() / total;
        let avg_budget_utilization =
            analyses.iter().map(|a| a.budget_utilization).sum::<f64>() / total;

        let ftr_count = analyses.iter().filter(|a| a.first_time_right).count();
        let first_time_right_rate = ftr_count as f64 / total;

        // Generate recommendations
        let (recommendations, priority) = self.generate_tuning_recommendations(
            first_time_right_rate,
            avg_budget_utilization,
            avg_efficiency,
            avg_completeness,
            avg_coherence,
        );

        Ok(WeeklyFeedback {
            period_start,
            period_end: now,
            total_requests: analyses.len(),
            avg_quality_score,
            first_time_right_rate,
            avg_budget_utilization,
            avg_efficiency,
            avg_completeness,
            avg_coherence,
            recommendations,
            tuning_priority: priority,
        })
    }

    /// Generate tuning recommendations based on metrics
    fn generate_tuning_recommendations(
        &self,
        ftr_rate: f64,
        avg_util: f64,
        avg_eff: f64,
        avg_comp: f64,
        avg_coh: f64,
    ) -> (Vec<String>, TuningPriority) {
        let mut recommendations = Vec::new();
        let mut priority = TuningPriority::Low;

        // Check first-time-right rate
        if ftr_rate < 0.80 {
            recommendations.push(format!(
                "⚠️ CRITICAL: First-time-right rate very low ({:.1}%). Budget classifier is under-allocating. Increase default budgets by 25-50%.",
                ftr_rate * 100.0
            ));
            priority = TuningPriority::Critical;
        } else if ftr_rate < 0.90 {
            recommendations.push(format!(
                "⚠️ First-time-right rate below target ({:.1}% < 90%). Consider increasing budgets by 15-25%.",
                ftr_rate * 100.0
            ));
            if priority == TuningPriority::Low {
                priority = TuningPriority::High;
            }
        } else if ftr_rate < 0.95 {
            recommendations.push(format!(
                "First-time-right rate acceptable ({:.1}%) but can improve. Consider increasing budgets by 5-10%.",
                ftr_rate * 100.0
            ));
            if priority == TuningPriority::Low {
                priority = TuningPriority::Medium;
            }
        }

        // Check budget utilization
        if avg_util > 0.98 {
            recommendations.push(format!(
                "⚠️ CRITICAL: Budget utilization very high ({:.1}%). Budgets are consistently maxed out. Increase by 25-50%.",
                avg_util * 100.0
            ));
            priority = TuningPriority::Critical;
        } else if avg_util > 0.95 {
            recommendations.push(format!(
                "⚠️ Budget utilization high ({:.1}% > 95%). Budgets are frequently at limit. Increase by 15-25%.",
                avg_util * 100.0
            ));
            if priority != TuningPriority::Critical {
                priority = TuningPriority::High;
            }
        } else if avg_util < 0.70 {
            recommendations.push(format!(
                "Budget under-utilized ({:.1}% < 75%). Consider decreasing budgets by 10-20% to save costs.",
                avg_util * 100.0
            ));
            if priority == TuningPriority::Low {
                priority = TuningPriority::High;
            }
        } else if avg_util < 0.75 {
            recommendations.push(format!(
                "Budget slightly under-utilized ({:.1}%). Consider decreasing budgets by 5-10%.",
                avg_util * 100.0
            ));
            if priority == TuningPriority::Low {
                priority = TuningPriority::Medium;
            }
        } else if (0.75..=0.95).contains(&avg_util) {
            recommendations.push(format!(
                "✅ Budget utilization optimal ({:.1}%). Classifier is well-tuned.",
                avg_util * 100.0
            ));
        }

        // Check completeness
        if avg_comp < 0.7 {
            recommendations.push(format!(
                "⚠️ Low completeness score ({:.1}%). Many responses are truncated. Increase budgets.",
                avg_comp
            ));
            if priority == TuningPriority::Low {
                priority = TuningPriority::High;
            }
        }

        // Check coherence
        if avg_coh < 0.7 {
            recommendations.push(format!(
                "Low coherence score ({:.1}%). Thinking/output balance is suboptimal. Review classifier logic.",
                avg_coh
            ));
        }

        // Check efficiency
        if avg_eff < 0.7 {
            recommendations.push(format!(
                "Low efficiency score ({:.1}%). Budget allocation strategy needs tuning.",
                avg_eff
            ));
        }

        if recommendations.is_empty() {
            recommendations
                .push("✅ All metrics within optimal range. No tuning needed.".to_string());
        }

        (recommendations, priority)
    }

    /// Load quality analyses from database since timestamp
    async fn load_quality_analyses_since(
        &self,
        since_timestamp: i64,
    ) -> Result<Vec<QualityAnalysis>, String> {
        crate::modules::proxy_db::load_quality_analyses_since(since_timestamp)
    }
}

// ========== Main Quality Monitor ==========

/// Main thinking quality monitoring coordinator
pub struct ThinkingQualityMonitor {
    quality_scorer: Arc<QualityScorer>,
    efficiency_tracker: Arc<EfficiencyTracker>,
    feedback_aggregator: Arc<FeedbackAggregator>,
    metrics: Arc<RwLock<QualityMetrics>>,
}

impl ThinkingQualityMonitor {
    pub fn new() -> Self {
        Self {
            quality_scorer: Arc::new(QualityScorer::new()),
            efficiency_tracker: Arc::new(EfficiencyTracker::new()),
            feedback_aggregator: Arc::new(FeedbackAggregator::new()),
            metrics: Arc::new(RwLock::new(QualityMetrics::default())),
        }
    }

    /// Analyze request/response quality
    pub async fn analyze_quality(
        &self,
        request_id: String,
        thinking_tokens: u32,
        output_tokens: u32,
        thinking_budget: u32,
        finish_reason: FinishReason,
        escalation_count: u32,
    ) -> QualityAnalysis {
        let budget_utilization = if thinking_budget > 0 {
            thinking_tokens as f64 / thinking_budget as f64
        } else {
            0.0
        };

        // Calculate individual scores
        let efficiency_score = self
            .efficiency_tracker
            .calculate_efficiency(thinking_tokens, thinking_budget);
        let completeness_score = self.quality_scorer.score_completeness(
            &finish_reason,
            thinking_tokens,
            thinking_budget,
        );
        let coherence_score = self
            .quality_scorer
            .score_coherence(thinking_tokens, output_tokens);

        // Calculate overall score (weighted average)
        let overall_score =
            efficiency_score * 0.3 + completeness_score * 0.4 + coherence_score * 0.3;

        let first_time_right = self
            .efficiency_tracker
            .is_first_time_right(&finish_reason, escalation_count);

        let analysis = QualityAnalysis {
            request_id,
            timestamp: Utc::now(),
            efficiency_score,
            completeness_score,
            coherence_score,
            overall_score,
            thinking_tokens,
            output_tokens,
            thinking_budget,
            budget_utilization,
            first_time_right,
            escalation_count,
            finish_reason: finish_reason.as_str().to_string(),
            user_rating: None,
        };

        // Update running metrics
        self.update_metrics(&analysis).await;

        debug!(
            "Quality analysis: request_id={}, overall={:.2}, efficiency={:.2}, completeness={:.2}, coherence={:.2}, FTR={}",
            analysis.request_id,
            analysis.overall_score,
            analysis.efficiency_score,
            analysis.completeness_score,
            analysis.coherence_score,
            analysis.first_time_right
        );

        analysis
    }

    /// Update running quality metrics
    async fn update_metrics(&self, analysis: &QualityAnalysis) {
        let mut metrics = self.metrics.write().await;

        let total = metrics.total_requests as f64;
        let new_total = total + 1.0;

        // Update counters
        metrics.total_requests += 1;
        if analysis.first_time_right {
            metrics.first_time_right += 1;
        }
        if analysis.escalation_count > 0 {
            metrics.escalations_needed += 1;
        }

        // Update running averages
        metrics.average_quality_score =
            (metrics.average_quality_score * total + analysis.overall_score) / new_total;
        metrics.average_efficiency =
            (metrics.average_efficiency * total + analysis.efficiency_score) / new_total;
        metrics.average_completeness =
            (metrics.average_completeness * total + analysis.completeness_score) / new_total;
        metrics.average_coherence =
            (metrics.average_coherence * total + analysis.coherence_score) / new_total;
        metrics.average_budget_utilization =
            (metrics.average_budget_utilization * total + analysis.budget_utilization) / new_total;

        metrics.last_updated = Some(Utc::now().timestamp());
    }

    /// Get current quality metrics
    pub async fn get_metrics(&self) -> QualityMetrics {
        self.metrics.read().await.clone()
    }

    /// Get weekly feedback for classifier tuning
    pub async fn get_weekly_feedback(&self, days: u32) -> Result<WeeklyFeedback, String> {
        self.feedback_aggregator
            .aggregate_weekly_feedback(days)
            .await
    }

    /// Reset metrics
    pub async fn reset_metrics(&self) {
        let mut metrics = self.metrics.write().await;
        *metrics = QualityMetrics::default();
        info!("Quality metrics reset");
    }
}

// ========== Tests ==========

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completeness_scoring() {
        let scorer = QualityScorer::new();

        // Perfect completion (not near limit)
        assert_eq!(
            scorer.score_completeness(&FinishReason::Stop, 1000, 2000),
            1.0
        );

        // Nearly exhausted but completed (>98% utilization = 0.981)
        let score = scorer.score_completeness(&FinishReason::Stop, 1963, 2000);
        assert!(
            (score - 0.95).abs() < 0.01,
            "Nearly exhausted score should be ~0.95, got {}",
            score
        );

        // Exactly at 98% boundary
        assert_eq!(
            scorer.score_completeness(&FinishReason::Stop, 1960, 2000),
            1.0
        );

        // Truncated
        assert_eq!(
            scorer.score_completeness(&FinishReason::MaxTokens, 2000, 2000),
            0.3
        );

        // Safety block
        assert_eq!(
            scorer.score_completeness(&FinishReason::Safety, 500, 2000),
            0.5
        );
    }

    #[test]
    fn test_coherence_scoring() {
        let scorer = QualityScorer::new();

        // Optimal ratio (50% thinking)
        assert_eq!(scorer.score_coherence(1000, 1000), 1.0);

        // Good ratio (30% thinking)
        assert_eq!(scorer.score_coherence(300, 700), 1.0);

        // Too little thinking (5%)
        assert_eq!(scorer.score_coherence(50, 950), 0.6);

        // Too much thinking (95%)
        assert_eq!(scorer.score_coherence(950, 50), 0.7);

        // No thinking
        assert_eq!(scorer.score_coherence(0, 1000), 0.5);
    }

    #[test]
    fn test_efficiency_scoring() {
        let tracker = EfficiencyTracker::new();

        // Optimal utilization (85%)
        assert_eq!(tracker.calculate_efficiency(1700, 2000), 1.0);

        // Optimal lower bound (75%)
        assert_eq!(tracker.calculate_efficiency(1500, 2000), 1.0);

        // Optimal upper bound (95%)
        assert_eq!(tracker.calculate_efficiency(1900, 2000), 1.0);

        // Under-utilized (50%)
        assert!((tracker.calculate_efficiency(1000, 2000) - 0.667).abs() < 0.01);

        // Maxed out (100%)
        assert_eq!(tracker.calculate_efficiency(2000, 2000), 0.5);
    }

    #[test]
    fn test_first_time_right() {
        let tracker = EfficiencyTracker::new();

        // Perfect FTR
        assert!(tracker.is_first_time_right(&FinishReason::Stop, 0));

        // Had escalation
        assert!(!tracker.is_first_time_right(&FinishReason::Stop, 1));

        // Truncated
        assert!(!tracker.is_first_time_right(&FinishReason::MaxTokens, 0));
    }

    #[test]
    fn test_tuning_priority() {
        let aggregator = FeedbackAggregator::new();

        // Critical: low FTR
        let (recs, priority) =
            aggregator.generate_tuning_recommendations(0.75, 0.85, 0.8, 0.8, 0.8);
        assert_eq!(priority, TuningPriority::Critical);
        assert!(recs[0].contains("CRITICAL"));

        // High: FTR below 90%
        let (_, priority) = aggregator.generate_tuning_recommendations(0.85, 0.85, 0.8, 0.8, 0.8);
        assert_eq!(priority, TuningPriority::High);

        // Low: all optimal
        let (recs, priority) =
            aggregator.generate_tuning_recommendations(0.95, 0.85, 0.9, 0.9, 0.9);
        assert_eq!(priority, TuningPriority::Low);
        assert!(recs.last().unwrap().contains("optimal"));
    }
}
