/// Account Management Handlers
///
/// Provides endpoints for account information and limits
/// Compatible with alternative proxy's `/account-limits` API

use crate::proxy::server::AppState;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json, Response},
    http::StatusCode,
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
    include_history: Option<bool>,
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
    // Get all accounts from TokenManager
    let accounts_json = match state.token_manager.get_all_accounts_info() {
        Ok(accounts) => accounts,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to load accounts: {}", e)
                })),
            )
                .into_response()
        }
    };

    let accounts: Vec<AccountLimitInfo> = accounts_json
        .into_iter()
        .filter_map(|account| {
            // Support both Antigravity Manager and alternative proxy formats
            let email = account.get("email").and_then(|v| v.as_str())?.to_string();

            // Check if account is enabled (alternative proxy format)
            let enabled = account.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
            if !enabled {
                return None;
            }

            // Check if account is disabled (Antigravity Manager format)
            let disabled = account.get("disabled").and_then(|v| v.as_bool()).unwrap_or(false);
            if disabled {
                return None;
            }

            // Determine account status
            let is_invalid = account.get("isInvalid").and_then(|v| v.as_bool()).unwrap_or(false);
            let proxy_disabled = account.get("proxy_disabled").and_then(|v| v.as_bool()).unwrap_or(false);

            let status = if is_invalid {
                "invalid".to_string()
            } else if proxy_disabled {
                "proxy_disabled".to_string()
            } else {
                "ok".to_string()
            };

            // Extract subscription info (both formats)
            let subscription = if let Some(sub) = account.get("subscription") {
                // Alternative proxy format
                sub.get("tier").and_then(|tier| tier.as_str()).map(|tier| {
                    let project_id = sub.get("projectId").and_then(|p| p.as_str()).map(|s| s.to_string());
                    SubscriptionInfo {
                        tier: tier.to_string(),
                        project_id,
                    }
                })
            } else if let Some(quota) = account.get("quota") {
                // Antigravity Manager format
                quota.get("subscription_tier").and_then(|tier| tier.as_str()).map(|tier| {
                    let project_id = account.get("token").and_then(|t| t.get("project_id")).and_then(|p| p.as_str()).map(|s| s.to_string());
                    SubscriptionInfo {
                        tier: tier.to_string(),
                        project_id,
                    }
                })
            } else {
                None
            };

            // Extract model limits from quota (both formats)
            let limits = account.get("quota").and_then(|q| {
                // Try object format first (alternative proxy)
                if let Some(models_obj) = q.get("models").and_then(|m| m.as_object()) {
                    if !models_obj.is_empty() {
                        let mut limits_map = HashMap::new();
                        for (model_name, model_info) in models_obj {
                            let remaining = model_info.get("remainingFraction").and_then(|v| v.as_f64());
                            let reset_time = model_info.get("resetTime").and_then(|v| v.as_str()).map(|s| s.to_string());
                            limits_map.insert(
                                model_name.clone(),
                                ModelLimitInfo {
                                    remaining_fraction: remaining,
                                    reset_time,
                                },
                            );
                        }
                        return Some(limits_map);
                    }
                }

                // Try array format (Antigravity Manager)
                if let Some(models_arr) = q.get("models").and_then(|m| m.as_array()) {
                    if !models_arr.is_empty() {
                        let mut limits_map = HashMap::new();
                        for model_info in models_arr {
                            let model_name = model_info.get("name").and_then(|n| n.as_str());
                            let remaining = model_info.get("percentage").and_then(|p| p.as_f64()).map(|p| p / 100.0);
                            let reset_time = model_info.get("reset_time").and_then(|t| t.as_str()).map(|s| s.to_string());

                            if let (Some(name), Some(frac)) = (model_name, remaining) {
                                limits_map.insert(
                                    name.to_string(),
                                    ModelLimitInfo {
                                        remaining_fraction: Some(frac),
                                        reset_time,
                                    },
                                );
                            }
                        }
                        return Some(limits_map);
                    }
                }

                None
            });

            Some(AccountLimitInfo {
                email,
                status,
                subscription,
                limits,
                quota: account.get("quota").cloned(),
            })
        })
        .collect();

    // Get available models from custom mapping
    let models = if state.custom_mapping.read().await.is_empty() {
        None
    } else {
        Some(
            state
                .custom_mapping
                .read()
                .await
                .keys()
                .cloned()
                .collect(),
        )
    };

    let response = AccountLimitsResponse {
        accounts,
        models,
    };

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
    let mut lines = Vec::new();

    lines.push("╔══════════════════════════════════════════════════════════════╗".to_string());
    lines.push("║                    Account Limits                           ║".to_string());
    lines.push("╚══════════════════════════════════════════════════════════════╝".to_string());
    lines.push("".to_string());

    for account in &response.accounts {
        let status_icon = match account.status.as_str() {
            "ok" => "✓",
            "disabled" => "✗",
            "proxy_disabled" => "⊘",
            _ => "?",
        };

        lines.push(format!("{} {} [{}]", status_icon, account.email, account.status));

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
