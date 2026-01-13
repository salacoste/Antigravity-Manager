// Epic-025 Story-025-03: Budget Detector Tauri Commands
//
// Provides frontend access to budget escalation metrics and history

use crate::modules::budget_detector::{get_detector, EscalationEvent, EscalationMetrics};

/// Get escalation metrics
/// Returns aggregated metrics about budget escalations
#[tauri::command]
pub async fn get_escalation_metrics() -> Result<EscalationMetrics, String> {
    let detector = get_detector();
    let manager = detector.escalation_manager();
    let metrics = manager.calculate_metrics();

    Ok(metrics)
}

/// Get escalation history
/// Returns recent escalation events
/// - limit: Maximum number of events to return (default: 100)
#[tauri::command]
pub async fn get_escalation_history(limit: Option<usize>) -> Result<Vec<EscalationEvent>, String> {
    let detector = get_detector();
    let manager = detector.escalation_manager();

    let limit = limit.unwrap_or(100);
    manager.load_history(limit)
}

/// Reset escalation statistics
/// Clears in-memory escalation history (database records are preserved)
#[tauri::command]
pub async fn reset_escalation_stats() -> Result<(), String> {
    let detector = get_detector();

    // Clear in-memory history
    let manager = detector.escalation_manager();
    manager.clear_history();

    // Reset detector metrics
    detector.reset_metrics();

    tracing::info!("[BudgetDetector] Statistics reset");
    Ok(())
}
