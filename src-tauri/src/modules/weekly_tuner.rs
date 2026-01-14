// Epic-025 Week 7: Automated Weekly Tuning System
// Automatically analyzes quality metrics and adjusts budget optimizer parameters

#![allow(dead_code)] // WIP: Epic-025 Week 7 implementation
#![allow(unused_imports)] // TuningPriority used only in tests

use crate::modules::thinking_quality::{ThinkingQualityMonitor, TuningPriority, WeeklyFeedback};
use chrono::{Datelike, Timelike};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};

/// Tuning recommendation with confidence score
#[derive(Debug, Clone, serde::Serialize)]
pub struct TuningRecommendation {
    pub category: String,
    pub action: String,
    pub reason: String,
    pub auto_apply: bool,
    pub confidence: f64,
    pub impact_estimate: String,
}

/// Weekly tuner for automated quality-based optimization
pub struct WeeklyTuner {
    quality_monitor: Arc<ThinkingQualityMonitor>,
    enabled: Arc<AtomicBool>,
    last_tuning: Arc<tokio::sync::RwLock<Option<i64>>>,
}

impl WeeklyTuner {
    /// Create new weekly tuner instance
    pub fn new(quality_monitor: Arc<ThinkingQualityMonitor>) -> Self {
        Self {
            quality_monitor,
            enabled: Arc::new(AtomicBool::new(true)),
            last_tuning: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    /// Enable or disable automated tuning
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
        if enabled {
            info!("[Epic-025] Weekly tuning enabled");
        } else {
            warn!("[Epic-025] Weekly tuning disabled");
        }
    }

    /// Check if tuning is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Start weekly tuning scheduler (runs every Sunday at midnight UTC)
    pub async fn start(self: Arc<Self>) {
        info!("[Epic-025] Starting weekly tuning scheduler");

        let mut check_interval = interval(Duration::from_secs(3600)); // Check every hour

        loop {
            check_interval.tick().await;

            if !self.is_enabled() {
                continue;
            }

            let now = chrono::Utc::now();

            // Check if it's Sunday at midnight UTC (within 1 hour window)
            if now.weekday() == chrono::Weekday::Sun && now.hour() == 0 {
                // Check if we already tuned this week
                let last_tuning = *self.last_tuning.read().await;
                let week_ago = now.timestamp() - 7 * 86400;

                if let Some(last) = last_tuning {
                    if last > week_ago {
                        // Already tuned this week
                        continue;
                    }
                }

                // Run weekly tuning
                match self.run_weekly_tuning().await {
                    Ok(_) => {
                        let mut last = self.last_tuning.write().await;
                        *last = Some(now.timestamp());
                    }
                    Err(e) => {
                        error!("[Epic-025] Weekly tuning failed: {}", e);
                    }
                }
            }
        }
    }

    /// Run weekly tuning analysis and apply recommendations
    async fn run_weekly_tuning(&self) -> Result<(), String> {
        info!("[Epic-025] Running weekly quality tuning...");

        // Get weekly feedback
        let feedback = self
            .quality_monitor
            .get_weekly_feedback(7)
            .await
            .map_err(|e| format!("Failed to get weekly feedback: {}", e))?;

        // Generate tuning recommendations
        let recommendations = self.generate_tuning_recommendations(&feedback);

        info!(
            "[Epic-025] Generated {} tuning recommendations",
            recommendations.len()
        );

        // Apply automatic adjustments (if safe)
        let mut applied_count = 0;
        for rec in &recommendations {
            if rec.auto_apply && rec.confidence > 0.8 {
                match self.apply_tuning(rec).await {
                    Ok(_) => {
                        info!("[Epic-025] Applied tuning: {}", rec.action);
                        applied_count += 1;
                    }
                    Err(e) => {
                        warn!("[Epic-025] Failed to apply tuning {}: {}", rec.action, e);
                    }
                }
            }
        }

        // Save tuning report
        self.save_tuning_report(&feedback, &recommendations).await?;

        info!(
            "[Epic-025] Weekly tuning complete: {}/{} recommendations applied",
            applied_count,
            recommendations.len()
        );

        Ok(())
    }

    /// Generate tuning recommendations based on weekly feedback
    fn generate_tuning_recommendations(
        &self,
        feedback: &WeeklyFeedback,
    ) -> Vec<TuningRecommendation> {
        let mut recommendations = Vec::new();

        // Check first-time-right rate
        if feedback.first_time_right_rate < 0.9 {
            let increase_pct = ((0.9 / feedback.first_time_right_rate - 1.0) * 100.0).ceil();
            recommendations.push(TuningRecommendation {
                category: "Budget Allocation".to_string(),
                action: format!("Increase default budgets by {}%", increase_pct),
                reason: format!(
                    "FTR rate {:.1}% below 90% target",
                    feedback.first_time_right_rate * 100.0
                ),
                auto_apply: false, // Manual approval required for budget increases
                confidence: 0.85,
                impact_estimate: format!(
                    "+{:.1}% cost, +{:.1}% quality",
                    increase_pct,
                    increase_pct * 0.8
                ),
            });
        } else if feedback.first_time_right_rate > 0.95 {
            recommendations.push(TuningRecommendation {
                category: "Budget Allocation".to_string(),
                action: "Consider gradual budget optimization (5%)".to_string(),
                reason: format!(
                    "FTR rate {:.1}% exceeds 95% - potential for cost savings",
                    feedback.first_time_right_rate * 100.0
                ),
                auto_apply: false,
                confidence: 0.75,
                impact_estimate: "-5% cost, maintain >90% FTR".to_string(),
            });
        }

        // Check budget utilization
        if feedback.avg_budget_utilization < 0.75 {
            let decrease_pct = ((0.75 - feedback.avg_budget_utilization) * 100.0).ceil();
            recommendations.push(TuningRecommendation {
                category: "Cost Optimization".to_string(),
                action: format!("Decrease default budgets by {:.0}%", decrease_pct),
                reason: format!(
                    "Budget under-utilized at {:.1}%",
                    feedback.avg_budget_utilization * 100.0
                ),
                auto_apply: true, // Safe to auto-apply decreases
                confidence: 0.9,
                impact_estimate: format!("-{:.0}% cost, maintain quality", decrease_pct),
            });
        } else if feedback.avg_budget_utilization > 0.95 {
            recommendations.push(TuningRecommendation {
                category: "Budget Allocation".to_string(),
                action: "Increase budgets by 10% to prevent truncation".to_string(),
                reason: format!(
                    "Budget over-utilized at {:.1}%",
                    feedback.avg_budget_utilization * 100.0
                ),
                auto_apply: false, // Manual approval for increases
                confidence: 0.88,
                impact_estimate: "+10% cost, prevent quality degradation".to_string(),
            });
        }

        // Check efficiency score
        if feedback.avg_efficiency < 0.8 {
            recommendations.push(TuningRecommendation {
                category: "Efficiency".to_string(),
                action: "Review budget allocation tiers".to_string(),
                reason: format!(
                    "Efficiency score {:.1}% indicates suboptimal budget utilization",
                    feedback.avg_efficiency * 100.0
                ),
                auto_apply: false,
                confidence: 0.75,
                impact_estimate: "Potential 10-15% cost reduction".to_string(),
            });
        }

        // Check completeness score
        if feedback.avg_completeness < 0.95 {
            recommendations.push(TuningRecommendation {
                category: "Quality".to_string(),
                action: "Investigate response truncation patterns".to_string(),
                reason: format!(
                    "Completeness score {:.1}% indicates responses being cut off",
                    feedback.avg_completeness * 100.0
                ),
                auto_apply: false,
                confidence: 0.82,
                impact_estimate: "Quality improvement".to_string(),
            });
        }

        // Check coherence score
        if feedback.avg_coherence < 0.85 {
            recommendations.push(TuningRecommendation {
                category: "Quality".to_string(),
                action: "Review thinking/output balance".to_string(),
                reason: format!(
                    "Coherence score {:.1}% indicates imbalanced responses",
                    feedback.avg_coherence * 100.0
                ),
                auto_apply: false,
                confidence: 0.78,
                impact_estimate: "Quality improvement".to_string(),
            });
        }

        recommendations
    }

    /// Apply a tuning recommendation
    async fn apply_tuning(&self, rec: &TuningRecommendation) -> Result<(), String> {
        // For now, log the tuning action
        // In production, this would integrate with BudgetOptimizer to adjust parameters
        info!(
            "[Epic-025] AUTO-TUNE: {} - {} (confidence: {:.2})",
            rec.category, rec.action, rec.confidence
        );

        // Placeholder for actual tuning logic
        // This would call budget_optimizer.adjust_default_budgets() or similar
        match rec.category.as_str() {
            "Cost Optimization" if rec.action.contains("Decrease") => {
                // Extract percentage from action string
                if let Some(pct_str) = rec.action.split("by ").nth(1) {
                    if let Some(pct) = pct_str.split('%').next() {
                        if let Ok(pct_val) = pct.parse::<f64>() {
                            let adjustment = -pct_val / 100.0;
                            info!(
                                "[Epic-025] Would adjust budgets by {:.2}%",
                                adjustment * 100.0
                            );
                            // TODO: budget_optimizer.adjust_default_budgets(adjustment).await?;
                        }
                    }
                }
            }
            _ => {
                // Other categories require manual intervention
            }
        }

        Ok(())
    }

    /// Save tuning report to database
    async fn save_tuning_report(
        &self,
        feedback: &WeeklyFeedback,
        recommendations: &[TuningRecommendation],
    ) -> Result<(), String> {
        let report = serde_json::json!({
            "timestamp": chrono::Utc::now().timestamp(),
            "period": {
                "start": feedback.period_start,
                "end": feedback.period_end,
            },
            "metrics": {
                "total_requests": feedback.total_requests,
                "avg_quality_score": feedback.avg_quality_score,
                "first_time_right_rate": feedback.first_time_right_rate,
                "avg_budget_utilization": feedback.avg_budget_utilization,
            },
            "recommendations": recommendations,
        });

        info!("[Epic-025] Tuning report: {}", report);

        // Save to database (would need a new table for tuning reports)
        // For now, just log it
        // TODO: proxy_db::save_tuning_report(&report)?;

        Ok(())
    }

    /// Manual trigger for weekly tuning (for testing)
    pub async fn trigger_manual_tuning(&self) -> Result<Vec<TuningRecommendation>, String> {
        info!("[Epic-025] Manual weekly tuning triggered");

        let feedback = self.quality_monitor.get_weekly_feedback(7).await?;
        let recommendations = self.generate_tuning_recommendations(&feedback);

        self.save_tuning_report(&feedback, &recommendations).await?;

        Ok(recommendations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tuning_recommendations_low_ftr() {
        use chrono::{TimeZone, Utc};
        let monitor = Arc::new(ThinkingQualityMonitor::new());
        let tuner = WeeklyTuner::new(monitor);

        let feedback = WeeklyFeedback {
            period_start: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            period_end: Utc.with_ymd_and_hms(2026, 3, 21, 0, 0, 0).unwrap(),
            total_requests: 100,
            avg_quality_score: 0.85,
            first_time_right_rate: 0.82, // Below 90%
            avg_budget_utilization: 0.85,
            avg_efficiency: 0.87,
            avg_completeness: 0.95,
            avg_coherence: 0.90,
            recommendations: vec![],
            tuning_priority: TuningPriority::High,
        };

        let recs = tuner.generate_tuning_recommendations(&feedback);

        assert!(!recs.is_empty());
        assert!(recs
            .iter()
            .any(|r| r.category == "Budget Allocation" && r.action.contains("Increase")));
    }

    #[tokio::test]
    async fn test_tuning_recommendations_low_utilization() {
        use chrono::{TimeZone, Utc};
        let monitor = Arc::new(ThinkingQualityMonitor::new());
        let tuner = WeeklyTuner::new(monitor);

        let feedback = WeeklyFeedback {
            period_start: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            period_end: Utc.with_ymd_and_hms(2026, 3, 21, 0, 0, 0).unwrap(),
            total_requests: 100,
            avg_quality_score: 0.90,
            first_time_right_rate: 0.92,
            avg_budget_utilization: 0.65, // Under-utilized
            avg_efficiency: 0.75,
            avg_completeness: 0.98,
            avg_coherence: 0.92,
            recommendations: vec![],
            tuning_priority: TuningPriority::Medium,
        };

        let recs = tuner.generate_tuning_recommendations(&feedback);

        assert!(!recs.is_empty());
        assert!(recs.iter().any(|r| r.category == "Cost Optimization"
            && r.action.contains("Decrease")
            && r.auto_apply));
    }
}
