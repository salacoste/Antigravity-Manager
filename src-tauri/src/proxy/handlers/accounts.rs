/// Account Management Handlers
///
/// Provides endpoints for account information and limits
/// Compatible with alternative proxy's `/account-limits` API
use crate::proxy::server::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Query parameters for /account-limits
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AccountLimitsQuery {
    /// Return format: 'json' or 'table' (ASCII table)
    format: Option<String>,
    /// Include usage history if available
    _include_history: Option<bool>,
}

/// Account limit response for single account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLimitInfo {
    pub email: String,
    pub status: String,
    pub subscription: Option<SubscriptionInfo>,
    pub limits: Option<HashMap<String, ModelLimitInfo>>,
    /// DEBUG: Raw quota data
    pub quota: Option<serde_json::Value>,
}

/// Subscription tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    pub tier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

/// Model-specific limit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLimitInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_time: Option<String>,
}

/// Full response for /account-limits endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLimitsResponse {
    pub accounts: Vec<AccountLimitInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
}

/// GET /account-limits
///
/// Returns account information, subscription tiers, and model quotas.
/// Compatible with alternative proxy's API format.
///
/// Query parameters:
/// - format: 'json' (default) or 'table' (ASCII table)
/// - include_history: Include usage history if available
pub async fn handle_account_limits(
    State(state): State<AppState>,
    Query(params): Query<AccountLimitsQuery>,
) -> Response {
    // TODO: Implement get_all_accounts_info on TokenManager
    // For now, return empty account list as a stub
    // The actual account information would need to be exposed from TokenManager

    let accounts: Vec<AccountLimitInfo> = Vec::new();

    // Get available models from custom mapping
    let models = if state.custom_mapping.read().await.is_empty() {
        None
    } else {
        Some(state.custom_mapping.read().await.keys().cloned().collect())
    };

    let response = AccountLimitsResponse { accounts, models };

    // Check format parameter
    match params.format.as_deref() {
        Some("table") => {
            // Return ASCII table format (for CLI compatibility)
            let table_text = format_account_limits_table(&response);
            (
                StatusCode::OK,
                [("content-type", "text/plain; charset=utf-8")],
                table_text,
            )
                .into_response()
        }
        _ => {
            // Default: JSON format
            Json(response).into_response()
        }
    }
}

/// Format account limits as ASCII table (for CLI compatibility)
fn format_account_limits_table(response: &AccountLimitsResponse) -> String {
    let mut lines = vec![
        "╔══════════════════════════════════════════════════════════════╗".to_string(),
        "║                    Account Limits                           ║".to_string(),
        "╚══════════════════════════════════════════════════════════════╝".to_string(),
        "".to_string(),
    ];

    for account in &response.accounts {
        let status_icon = match account.status.as_str() {
            "ok" => "✓",
            "disabled" => "✗",
            "proxy_disabled" => "⊘",
            _ => "?",
        };

        lines.push(format!(
            "{} {} [{}]",
            status_icon, account.email, account.status
        ));

        if let Some(sub) = &account.subscription {
            lines.push(format!("  Tier: {}", sub.tier));
        }

        if let Some(limits) = &account.limits {
            if !limits.is_empty() {
                lines.push("  Limits:".to_string());
                for (model, limit) in limits {
                    if let Some(fraction) = limit.remaining_fraction {
                        let percentage = (fraction * 100.0) as i32;
                        lines.push(format!("    {}: {}%", model, percentage));
                    }
                }
            }
        }

        lines.push("".to_string());
    }

    if let Some(models) = &response.models {
        if !models.is_empty() {
            lines.push("Available models:".to_string());
            for model in models {
                lines.push(format!("  {}", model));
            }
        }
    }

    lines.join("\n")
}
