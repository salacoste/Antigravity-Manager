//! Quota Manager Dashboard Commands Module
//!
//! Tauri commands for QuotaManager integration with frontend Dashboard.
//!
//! Part of Epic-001 Phase 3 Story QUOTA-001-06: Dashboard Integration

use crate::modules::quota_cache::QuotaInfo;
use crate::modules::quota_fetcher::SubscriptionTier;
use crate::modules::quota_manager::{QuotaManager, MonitorStats};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::State;

/// State for QuotaManager shared across commands
pub struct QuotaManagerState {
    pub quota_manager: Arc<QuotaManager>,
}

impl QuotaManagerState {
    pub fn new(quota_manager: Arc<QuotaManager>) -> Self {
        Self { quota_manager }
    }
}

/// Response structure for quota status
#[derive(Debug, Serialize)]
pub struct QuotaStatus {
    pub account_id: String,
    pub model_quotas: HashMap<String, QuotaInfoResponse>,
}

/// Response structure for individual quota info
#[derive(Debug, Serialize)]
pub struct QuotaInfoResponse {
    pub remaining_fraction: f64,
    pub reset_time: String, // ISO 8601
    pub display_name: String,
    pub status: String, // "healthy", "low", "exhausted"
}

impl From<QuotaInfo> for QuotaInfoResponse {
    fn from(info: QuotaInfo) -> Self {
        let status = if info.remaining_fraction >= 0.1 {
            "healthy"
        } else if info.remaining_fraction > 0.0 {
            "low"
        } else {
            "exhausted"
        };

        Self {
            remaining_fraction: info.remaining_fraction,
            reset_time: info.reset_time.to_rfc3339(),
            display_name: info.display_name,
            status: status.to_string(),
        }
    }
}

/// Response structure for subscription tier
#[derive(Debug, Serialize)]
pub struct TierInfo {
    pub account_id: String,
    pub tier: String, // "FREE", "PRO", "ULTRA"
}

/// Get quotas for all models for a specific account
#[tauri::command]
pub async fn get_account_quotas(
    account_id: String,
    access_token: String,
    project_id: String,
    state: State<'_, QuotaManagerState>,
) -> Result<QuotaStatus, String> {
    tracing::debug!(
        "get_account_quotas called for account: {}",
        account_id
    );

    let quotas = state
        .quota_manager
        .get_all_quotas(&account_id, &access_token, &project_id)
        .await?;

    let model_quotas: HashMap<String, QuotaInfoResponse> = quotas
        .into_iter()
        .map(|(model, info)| (model, info.into()))
        .collect();

    Ok(QuotaStatus {
        account_id,
        model_quotas,
    })
}

/// Get subscription tier for an account
#[tauri::command]
pub async fn get_account_tier(
    account_id: String,
    access_token: String,
    state: State<'_, QuotaManagerState>,
) -> Result<TierInfo, String> {
    tracing::debug!("get_account_tier called for account: {}", account_id);

    let tier = state
        .quota_manager
        .get_subscription_tier(&account_id, &access_token)
        .await?;

    let tier_str = match tier {
        SubscriptionTier::Free => "FREE",
        SubscriptionTier::Pro => "PRO",
        SubscriptionTier::Ultra => "ULTRA",
    };

    Ok(TierInfo {
        account_id,
        tier: tier_str.to_string(),
    })
}

/// Get quota monitor statistics
#[tauri::command]
pub async fn get_quota_manager_stats(
    state: State<'_, QuotaManagerState>,
) -> Result<MonitorStats, String> {
    tracing::debug!("get_quota_manager_stats called");

    Ok(state.quota_manager.get_monitor_stats().await)
}

/// Clear tier cache for an account (force refresh)
#[tauri::command]
pub async fn clear_tier_cache(
    account_id: String,
    state: State<'_, QuotaManagerState>,
) -> Result<(), String> {
    tracing::debug!("clear_tier_cache called for account: {}", account_id);

    let cleared = state.quota_manager.clear_tier_cache(&account_id);

    if cleared {
        tracing::info!("Tier cache cleared for account: {}", account_id);
        Ok(())
    } else {
        Err(format!("No cached tier found for account: {}", account_id))
    }
}
