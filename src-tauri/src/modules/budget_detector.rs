// Epic-025 Story-025-03: Budget Sufficiency Detection System
//
// Purpose: Detect insufficient thinking budgets (finishReason: MAX_TOKENS) and
//          automatically escalate to higher budgets: 4K→12K→24K→Pro 32K
//
// Success Metrics:
// - Completeness rate: 100%
// - Escalation accuracy: >95%
// - Unnecessary escalations: <5%
// - Performance overhead: <50ms

#![allow(dead_code)] // WIP: Epic-025 Story-025-03 implementation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn};

// ========== Core Types ==========

/// Finish reason from Gemini API response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    /// Response completed naturally (SUCCESS)
    Stop,
    /// Budget exhausted - thinking truncated (NEEDS ESCALATION)
    MaxTokens,
    /// Safety filter triggered
    Safety,
    /// Recitation filter triggered
    Recitation,
    /// Other reasons
    Other,
    /// Unknown/unspecified
    #[serde(other)]
    Unspecified,
}

impl FinishReason {
    pub fn from_string(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "STOP" => Self::Stop,
            "MAX_TOKENS" => Self::MaxTokens,
            "SAFETY" => Self::Safety,
            "RECITATION" => Self::Recitation,
            "OTHER" => Self::Other,
            _ => Self::Unspecified,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stop => "STOP",
            Self::MaxTokens => "MAX_TOKENS",
            Self::Safety => "SAFETY",
            Self::Recitation => "RECITATION",
            Self::Other => "OTHER",
            Self::Unspecified => "UNSPECIFIED",
        }
    }
}

/// Response metadata for budget detection
#[derive(Debug, Clone)]
pub struct ResponseMetadata {
    pub request_id: String,
    pub finish_reason: FinishReason,
    pub thinking_budget: u32,
    pub thinking_tokens: u32,
    pub model_id: String,
}

/// Detection result
#[derive(Debug, Clone, PartialEq)]
pub enum DetectionResult {
    /// Response completed successfully
    Sufficient,
    /// Budget insufficient - escalation recommended
    Insufficient {
        current_budget: u32,
        recommended_budget: u32,
        reason: String,
    },
}

/// Escalation event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationEvent {
    pub request_id: String,
    pub original_budget: u32,
    pub escalated_budget: u32,
    pub model_switch: Option<String>, // Some("313" → "246") for Pro escalation
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub finish_reason: String,
}

/// Escalation metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationMetrics {
    pub total_escalations: u64,
    pub successful_escalations: u64,
    pub failed_escalations: u64,
    pub escalation_success_rate: f64,
    pub unnecessary_escalations: u64,
    pub unnecessary_rate: f64,
    pub budget_4k_to_12k: u64,
    pub budget_12k_to_24k: u64,
    pub budget_24k_to_pro: u64,
    pub avg_escalation_latency_ms: f64,
}

impl Default for EscalationMetrics {
    fn default() -> Self {
        Self {
            total_escalations: 0,
            successful_escalations: 0,
            failed_escalations: 0,
            escalation_success_rate: 0.0,
            unnecessary_escalations: 0,
            unnecessary_rate: 0.0,
            budget_4k_to_12k: 0,
            budget_12k_to_24k: 0,
            budget_24k_to_pro: 0,
            avg_escalation_latency_ms: 0.0,
        }
    }
}

// ========== Detector Metrics ==========

#[derive(Debug, Clone)]
pub(crate) struct DetectorMetrics {
    pub(crate) detections: u64,
    pub(crate) insufficient_detected: u64,
    pub(crate) false_positives: u64,
    pub(crate) avg_detection_time_ms: f64,
}

impl Default for DetectorMetrics {
    fn default() -> Self {
        Self {
            detections: 0,
            insufficient_detected: 0,
            false_positives: 0,
            avg_detection_time_ms: 0.0,
        }
    }
}

// ========== Escalation Manager ==========

pub struct EscalationManager {
    max_retries: usize,
    escalation_history: Arc<RwLock<HashMap<String, Vec<EscalationEvent>>>>,
}

impl EscalationManager {
    pub fn new(max_retries: usize) -> Self {
        Self {
            max_retries,
            escalation_history: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Determines if escalation should be attempted
    pub fn should_escalate(&self, request_id: &str) -> bool {
        let history = self.escalation_history.read().unwrap();
        let attempts = history.get(request_id).map(|v| v.len()).unwrap_or(0);
        let should = attempts < self.max_retries;

        debug!(
            "[EscalationManager] Request {}: attempts={}/{}, should_escalate={}",
            request_id, attempts, self.max_retries, should
        );

        should
    }

    /// Records escalation attempt
    pub fn record_escalation(&self, event: EscalationEvent) {
        let request_id = event.request_id.clone();
        let mut history = self.escalation_history.write().unwrap();

        history
            .entry(request_id.clone())
            .or_default()
            .push(event.clone());

        if event.success {
            info!(
                "[EscalationManager] Escalation SUCCESS for {}: {}→{} {}",
                request_id,
                event.original_budget,
                event.escalated_budget,
                event
                    .model_switch
                    .as_ref()
                    .map(|s| format!("({})", s))
                    .unwrap_or_default()
            );
        } else {
            warn!(
                "[EscalationManager] Escalation FAILED for {}: {}→{} ({})",
                request_id, event.original_budget, event.escalated_budget, event.finish_reason
            );
        }

        // Persist to database
        if let Err(e) = self.persist_event(&event) {
            error!("[EscalationManager] Failed to persist event: {}", e);
        }
    }

    /// Get escalation history for a request
    pub fn get_history(&self, request_id: &str) -> Vec<EscalationEvent> {
        let history = self.escalation_history.read().unwrap();
        history.get(request_id).cloned().unwrap_or_default()
    }

    /// Calculate escalation metrics
    pub fn calculate_metrics(&self) -> EscalationMetrics {
        let history = self.escalation_history.read().unwrap();

        let mut metrics = EscalationMetrics::default();

        for events in history.values() {
            for event in events {
                metrics.total_escalations += 1;

                if event.success {
                    metrics.successful_escalations += 1;
                } else {
                    metrics.failed_escalations += 1;
                }

                // Track budget transitions
                match (event.original_budget, event.escalated_budget) {
                    (0..=4096, 12288) => metrics.budget_4k_to_12k += 1,
                    (4097..=12288, 24576) => metrics.budget_12k_to_24k += 1,
                    (12289..=24576, 32000) => metrics.budget_24k_to_pro += 1,
                    _ => {}
                }
            }
        }

        if metrics.total_escalations > 0 {
            metrics.escalation_success_rate =
                (metrics.successful_escalations as f64 / metrics.total_escalations as f64) * 100.0;

            metrics.unnecessary_rate =
                (metrics.unnecessary_escalations as f64 / metrics.total_escalations as f64) * 100.0;
        }

        metrics
    }

    /// Clear escalation history (for testing/reset)
    pub fn clear_history(&self) {
        let mut history = self.escalation_history.write().unwrap();
        history.clear();
        info!("[EscalationManager] Escalation history cleared");
    }

    /// Persist escalation event to database
    fn persist_event(&self, event: &EscalationEvent) -> Result<(), String> {
        use crate::modules::proxy_db::get_proxy_db_path;
        use rusqlite::{params, Connection};

        let db_path = get_proxy_db_path()?;
        let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO budget_escalations
             (request_id, original_budget, escalated_budget, model_switch, timestamp, success, finish_reason)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                event.request_id,
                event.original_budget as i64,
                event.escalated_budget as i64,
                event.model_switch,
                event.timestamp.timestamp(),
                if event.success { 1 } else { 0 },
                event.finish_reason,
            ],
        )
        .map_err(|e| format!("Failed to persist escalation event: {}", e))?;

        Ok(())
    }

    /// Load escalation history from database
    pub fn load_history(&self, limit: usize) -> Result<Vec<EscalationEvent>, String> {
        use crate::modules::proxy_db::get_proxy_db_path;
        use rusqlite::Connection;

        let db_path = get_proxy_db_path()?;
        let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(
                "SELECT request_id, original_budget, escalated_budget, model_switch, timestamp, success, finish_reason
                 FROM budget_escalations
                 ORDER BY timestamp DESC
                 LIMIT ?1",
            )
            .map_err(|e| e.to_string())?;

        let events_iter = stmt
            .query_map([limit], |row| {
                let timestamp: i64 = row.get(4)?;
                let success_flag: i64 = row.get(5)?;

                Ok(EscalationEvent {
                    request_id: row.get(0)?,
                    original_budget: row.get::<_, i64>(1)? as u32,
                    escalated_budget: row.get::<_, i64>(2)? as u32,
                    model_switch: row.get(3)?,
                    timestamp: DateTime::from_timestamp(timestamp, 0).unwrap_or_else(Utc::now),
                    success: success_flag != 0,
                    finish_reason: row.get(6)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut events = Vec::new();
        for event in events_iter {
            events.push(event.map_err(|e| e.to_string())?);
        }

        Ok(events)
    }
}

// ========== Budget Sufficiency Detector ==========

pub struct BudgetSufficiencyDetector {
    escalation_manager: Arc<EscalationManager>,
    metrics: Arc<RwLock<DetectorMetrics>>,
    /// Threshold for detecting truncation (95% = likely truncated)
    truncation_threshold: f64,
}

impl BudgetSufficiencyDetector {
    pub fn new(max_retries: usize) -> Self {
        Self {
            escalation_manager: Arc::new(EscalationManager::new(max_retries)),
            metrics: Arc::new(RwLock::new(DetectorMetrics::default())),
            truncation_threshold: 0.95,
        }
    }

    /// Analyzes response to detect insufficient budget
    /// Returns DetectionResult indicating if escalation is needed
    pub fn detect_insufficiency(&self, metadata: &ResponseMetadata) -> DetectionResult {
        let start = std::time::Instant::now();

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            metrics.detections += 1;
        }

        let result = self.analyze_response(metadata);

        // Track detection time
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        {
            let mut metrics = self.metrics.write().unwrap();
            let total_time = metrics.avg_detection_time_ms * (metrics.detections - 1) as f64;
            metrics.avg_detection_time_ms = (total_time + elapsed_ms) / metrics.detections as f64;

            if matches!(result, DetectionResult::Insufficient { .. }) {
                metrics.insufficient_detected += 1;
            }
        }

        debug!(
            "[BudgetDetector] Detection completed in {:.2}ms: {:?}",
            elapsed_ms, result
        );

        result
    }

    /// Core analysis logic
    fn analyze_response(&self, metadata: &ResponseMetadata) -> DetectionResult {
        // Only detect insufficiency for MAX_TOKENS finish reason
        if metadata.finish_reason != FinishReason::MaxTokens {
            debug!(
                "[BudgetDetector] FinishReason is {:?}, no escalation needed",
                metadata.finish_reason
            );
            return DetectionResult::Sufficient;
        }

        // Check if thinking was truncated (used ≥95% of budget)
        let usage_ratio = metadata.thinking_tokens as f64 / metadata.thinking_budget as f64;

        if usage_ratio >= self.truncation_threshold {
            let recommended = self.calculate_next_budget(metadata.thinking_budget);

            info!(
                "[BudgetDetector] INSUFFICIENT budget detected: request={}, budget={}, used={} ({:.1}%), recommend={}",
                metadata.request_id,
                metadata.thinking_budget,
                metadata.thinking_tokens,
                usage_ratio * 100.0,
                recommended
            );

            DetectionResult::Insufficient {
                current_budget: metadata.thinking_budget,
                recommended_budget: recommended,
                reason: format!(
                    "Thinking truncated at {}% usage ({}/{} tokens)",
                    (usage_ratio * 100.0) as u32,
                    metadata.thinking_tokens,
                    metadata.thinking_budget
                ),
            }
        } else {
            debug!(
                "[BudgetDetector] MAX_TOKENS but usage {:.1}% < {:.1}%, natural completion",
                usage_ratio * 100.0,
                self.truncation_threshold * 100.0
            );
            DetectionResult::Sufficient
        }
    }

    /// Calculate next budget in escalation ladder
    /// 4K → 12K → 24K → 32K (Pro)
    fn calculate_next_budget(&self, current: u32) -> u32 {
        match current {
            0..=4096 => 12288,      // 4K → 12K
            4097..=12288 => 24576,  // 12K → 24K
            12289..=24576 => 32000, // 24K → 32K (Pro)
            _ => current,           // Already at max
        }
    }

    /// Determines if model should be switched to Pro
    pub fn should_switch_to_pro(&self, recommended_budget: u32) -> bool {
        recommended_budget > 24576
    }

    /// Access escalation manager
    pub fn escalation_manager(&self) -> &Arc<EscalationManager> {
        &self.escalation_manager
    }

    /// Get detector metrics
    pub fn get_metrics(&self) -> DetectorMetrics {
        self.metrics.read().unwrap().clone()
    }

    /// Reset detector metrics
    pub fn reset_metrics(&self) {
        let mut metrics = self.metrics.write().unwrap();
        *metrics = DetectorMetrics::default();
        info!("[BudgetDetector] Metrics reset");
    }
}

// ========== Global Detector Instance ==========

use once_cell::sync::Lazy;

static BUDGET_DETECTOR: Lazy<BudgetSufficiencyDetector> =
    Lazy::new(|| BudgetSufficiencyDetector::new(3));

/// Get global budget detector instance
pub fn get_detector() -> &'static BudgetSufficiencyDetector {
    &BUDGET_DETECTOR
}

// ========== Tests ==========

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finish_reason_parsing() {
        assert_eq!(FinishReason::from_string("STOP"), FinishReason::Stop);
        assert_eq!(
            FinishReason::from_string("MAX_TOKENS"),
            FinishReason::MaxTokens
        );
        assert_eq!(FinishReason::from_string("SAFETY"), FinishReason::Safety);
        assert_eq!(
            FinishReason::from_string("unknown"),
            FinishReason::Unspecified
        );
    }

    #[test]
    fn test_budget_ladder() {
        let detector = BudgetSufficiencyDetector::new(3);

        assert_eq!(detector.calculate_next_budget(4096), 12288);
        assert_eq!(detector.calculate_next_budget(12288), 24576);
        assert_eq!(detector.calculate_next_budget(24576), 32000);
        assert_eq!(detector.calculate_next_budget(32000), 32000); // Max
    }

    #[test]
    fn test_sufficient_budget() {
        let detector = BudgetSufficiencyDetector::new(3);

        let metadata = ResponseMetadata {
            request_id: "test-1".to_string(),
            finish_reason: FinishReason::Stop,
            thinking_budget: 4096,
            thinking_tokens: 2000,
            model_id: "313".to_string(),
        };

        let result = detector.detect_insufficiency(&metadata);
        assert_eq!(result, DetectionResult::Sufficient);
    }

    #[test]
    fn test_insufficient_budget() {
        let detector = BudgetSufficiencyDetector::new(3);

        let metadata = ResponseMetadata {
            request_id: "test-2".to_string(),
            finish_reason: FinishReason::MaxTokens,
            thinking_budget: 4096,
            thinking_tokens: 3900, // 95%+ usage
            model_id: "313".to_string(),
        };

        let result = detector.detect_insufficiency(&metadata);
        match result {
            DetectionResult::Insufficient {
                current_budget,
                recommended_budget,
                ..
            } => {
                assert_eq!(current_budget, 4096);
                assert_eq!(recommended_budget, 12288);
            }
            _ => panic!("Expected Insufficient result"),
        }
    }

    #[test]
    fn test_escalation_manager() {
        let manager = EscalationManager::new(3);

        let request_id = "test-request";

        // Should escalate initially
        assert!(manager.should_escalate(request_id));

        // Record 3 escalations
        for i in 0..3 {
            let event = EscalationEvent {
                request_id: request_id.to_string(),
                original_budget: 4096,
                escalated_budget: 12288,
                model_switch: None,
                timestamp: Utc::now(),
                success: i < 2, // First 2 succeed
                finish_reason: "MAX_TOKENS".to_string(),
            };
            manager.record_escalation(event);
        }

        // Should not escalate after max retries
        assert!(!manager.should_escalate(request_id));

        // Check metrics
        let metrics = manager.calculate_metrics();
        assert_eq!(metrics.total_escalations, 3);
        assert_eq!(metrics.successful_escalations, 2);
        assert_eq!(metrics.failed_escalations, 1);
    }

    #[test]
    fn test_pro_model_switch() {
        let detector = BudgetSufficiencyDetector::new(3);

        assert!(!detector.should_switch_to_pro(12288)); // 12K - stay Flash
        assert!(!detector.should_switch_to_pro(24576)); // 24K - stay Flash
        assert!(detector.should_switch_to_pro(32000)); // 32K - switch to Pro
    }
}
