// Epic-025 Story-025-04: Thinking Quality Monitoring Commands

use crate::modules::thinking_quality::{QualityAnalysis, QualityMetrics, WeeklyFeedback};
use crate::modules::{proxy_db, thinking_quality};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Global quality monitor instance
pub struct QualityMonitorState {
    pub monitor: Arc<RwLock<Option<thinking_quality::ThinkingQualityMonitor>>>,
}

impl QualityMonitorState {
    pub fn new() -> Self {
        Self {
            monitor: Arc::new(RwLock::new(None)),
        }
    }

    /// Initialize monitor if not already initialized
    pub async fn ensure_initialized(&self) {
        let mut monitor = self.monitor.write().await;
        if monitor.is_none() {
            *monitor = Some(thinking_quality::ThinkingQualityMonitor::new());
            tracing::info!("Quality monitor initialized");
        }
    }
}

/// Get current quality metrics
#[tauri::command]
pub async fn get_quality_metrics(
    state: tauri::State<'_, QualityMonitorState>,
) -> Result<QualityMetrics, String> {
    state.ensure_initialized().await;

    let monitor_lock = state.monitor.read().await;
    let monitor = monitor_lock
        .as_ref()
        .ok_or("Quality monitor not initialized")?;

    Ok(monitor.get_metrics().await)
}

/// Get weekly feedback for classifier tuning
#[tauri::command]
pub async fn get_weekly_feedback(
    state: tauri::State<'_, QualityMonitorState>,
    days: Option<u32>,
) -> Result<WeeklyFeedback, String> {
    state.ensure_initialized().await;

    let monitor_lock = state.monitor.read().await;
    let monitor = monitor_lock
        .as_ref()
        .ok_or("Quality monitor not initialized")?;

    let days = days.unwrap_or(7);
    monitor.get_weekly_feedback(days).await
}

/// Get quality history (recent analyses)
#[tauri::command]
pub async fn get_quality_history(limit: Option<usize>) -> Result<Vec<QualityAnalysis>, String> {
    let limit = limit.unwrap_or(100);
    proxy_db::load_quality_analyses(limit)
}

/// Submit user rating for a request
#[tauri::command]
pub async fn submit_user_rating(request_id: String, rating: f64) -> Result<(), String> {
    // Validate rating (0.0-5.0)
    if !(0.0..=5.0).contains(&rating) {
        return Err("Rating must be between 0.0 and 5.0".to_string());
    }

    proxy_db::update_quality_user_rating(&request_id, rating)
}

/// Reset quality metrics (for testing/debugging)
#[tauri::command]
pub async fn reset_quality_metrics(
    state: tauri::State<'_, QualityMonitorState>,
) -> Result<(), String> {
    state.ensure_initialized().await;

    let monitor_lock = state.monitor.read().await;
    let monitor = monitor_lock
        .as_ref()
        .ok_or("Quality monitor not initialized")?;

    monitor.reset_metrics().await;
    Ok(())
}
