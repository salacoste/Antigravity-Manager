use crate::modules::{quota, quota_monitor};
use crate::models::Account;
use serde::{Deserialize, Serialize};

/// Story-024-01: Quota Monitoring & Alerts Commands

/// Get quota status for an account
#[tauri::command]
pub async fn get_quota_status(
    account_id: String,
) -> Result<quota_monitor::QuotaStatus, String> {
    let monitor_lock = quota::get_quota_monitor().await;
    let monitor_opt = monitor_lock.read().await;

    if let Some(ref monitor) = *monitor_opt {
        monitor.check_quota(&account_id).await
    } else {
        Err("Quota monitor not initialized".to_string())
    }
}

/// Get quota history for an account
#[tauri::command]
pub async fn get_quota_history(
    account_id: String,
    limit: Option<usize>,
) -> Result<Vec<quota_monitor::QuotaHistoryRecord>, String> {
    let monitor_lock = quota::get_quota_monitor().await;
    let monitor_opt = monitor_lock.read().await;

    if let Some(ref monitor) = *monitor_opt {
        monitor.get_history(&account_id, limit.unwrap_or(100)).await
    } else {
        Err("Quota monitor not initialized".to_string())
    }
}

/// Get active alerts for an account
#[tauri::command]
pub async fn get_quota_alerts(
    account_id: String,
) -> Result<Vec<quota_monitor::QuotaAlert>, String> {
    let monitor_lock = quota::get_quota_monitor().await;
    let monitor_opt = monitor_lock.read().await;

    if let Some(ref monitor) = *monitor_opt {
        monitor.get_active_alerts(&account_id).await
    } else {
        Err("Quota monitor not initialized".to_string())
    }
}

/// Trigger auto-mitigation for an account
#[tauri::command]
pub async fn trigger_quota_mitigation(
    account_id: String,
) -> Result<quota_monitor::MitigationAction, String> {
    let monitor_lock = quota::get_quota_monitor().await;
    let monitor_opt = monitor_lock.read().await;

    if let Some(ref monitor) = *monitor_opt {
        monitor.auto_mitigate(&account_id).await
    } else {
        Err("Quota monitor not initialized".to_string())
    }
}

/// Get all monitored accounts
#[tauri::command]
pub async fn get_monitored_accounts() -> Result<Vec<String>, String> {
    let monitor_lock = quota::get_quota_monitor().await;
    let monitor_opt = monitor_lock.read().await;

    if let Some(ref monitor) = *monitor_opt {
        Ok(monitor.get_monitored_accounts().await)
    } else {
        Err("Quota monitor not initialized".to_string())
    }
}

/// Quota monitoring metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaMonitoringMetrics {
    pub total_accounts: usize,
    pub healthy_accounts: usize,
    pub warning_accounts: usize,
    pub critical_accounts: usize,
    pub exhausted_accounts: usize,
    pub forbidden_accounts: usize,
    pub active_alerts: usize,
    pub alert_accuracy: f32,
    pub mitigation_success_rate: f32,
}

/// Get quota monitoring metrics
#[tauri::command]
pub async fn get_quota_metrics() -> Result<QuotaMonitoringMetrics, String> {
    let monitor_lock = quota::get_quota_monitor().await;
    let monitor_opt = monitor_lock.read().await;

    if let Some(ref monitor) = *monitor_opt {
        let accounts = monitor.get_monitored_accounts().await;
        let total_accounts = accounts.len();

        let mut healthy = 0;
        let mut warning = 0;
        let mut critical = 0;
        let mut exhausted = 0;
        let mut forbidden = 0;
        let mut active_alerts = 0;

        for account_id in accounts {
            match monitor.check_quota(&account_id).await {
                Ok(status) => match status {
                    quota_monitor::QuotaStatus::Healthy => healthy += 1,
                    quota_monitor::QuotaStatus::Warning(_) => warning += 1,
                    quota_monitor::QuotaStatus::Critical(_) => critical += 1,
                    quota_monitor::QuotaStatus::Exhausted => exhausted += 1,
                    quota_monitor::QuotaStatus::Forbidden => forbidden += 1,
                },
                Err(_) => continue,
            }

            if let Ok(alerts) = monitor.get_active_alerts(&account_id).await {
                active_alerts += alerts.len();
            }
        }

        let alert_accuracy = monitor.get_alert_accuracy().await.unwrap_or(0.0);
        let mitigation_success_rate = monitor.get_mitigation_success_rate().await.unwrap_or(0.0);

        Ok(QuotaMonitoringMetrics {
            total_accounts,
            healthy_accounts: healthy,
            warning_accounts: warning,
            critical_accounts: critical,
            exhausted_accounts: exhausted,
            forbidden_accounts: forbidden,
            active_alerts,
            alert_accuracy,
            mitigation_success_rate,
        })
    } else {
        Err("Quota monitor not initialized".to_string())
    }
}

/// Update quota in monitor (called after quota refresh)
#[tauri::command]
pub async fn update_quota_monitor(account: Account) -> Result<(), String> {
    let monitor_lock = quota::get_quota_monitor().await;
    let monitor_opt = monitor_lock.read().await;

    if let Some(ref monitor) = *monitor_opt {
        monitor.update_quota(&account).await
    } else {
        Err("Quota monitor not initialized".to_string())
    }
}

/// Initialize quota monitoring system
#[tauri::command]
pub async fn init_quota_monitoring() -> Result<(), String> {
    quota::init_quota_monitor().await
}
