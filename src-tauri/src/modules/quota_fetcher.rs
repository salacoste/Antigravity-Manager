// Story QUOTA-001-01: Quota API Integration
// Focused API client for Google Antigravity v1internal quota APIs

#![allow(dead_code)] // Module will be integrated in Phase 2

use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info};

// ================================
// API Constants
// ================================

const API_BASE: &str = "https://cloudcode-pa.googleapis.com/v1internal";
const USER_AGENT: &str = "Antigravity-Tools/3.3.20";
const REQUEST_TIMEOUT_SECS: u64 = 30;

// ================================
// Public Data Structures
// ================================

/// Quota information for a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaInfo {
    /// Remaining quota as fraction (0.0 - 1.0)
    /// - 1.0 = 100% quota available
    /// - 0.5 = 50% quota used
    /// - 0.0 = quota fully exhausted
    pub remaining_fraction: f64,

    /// UTC timestamp when quota resets (always UTC midnight)
    /// Format: ISO 8601 "2026-01-11T00:00:00Z"
    pub reset_time: DateTime<Utc>,
}

/// Model quota information including display metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelQuotaInfo {
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_info: Option<QuotaInfo>,
}

/// Subscription tier enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionTier {
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "PRO")]
    Pro,
    #[serde(rename = "ULTRA")]
    Ultra,
}

impl SubscriptionTier {
    pub fn from_string(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ULTRA" => SubscriptionTier::Ultra,
            "PRO" => SubscriptionTier::Pro,
            _ => SubscriptionTier::Free,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionTier::Free => "FREE",
            SubscriptionTier::Pro => "PRO",
            SubscriptionTier::Ultra => "ULTRA",
        }
    }
}

/// Subscription information from loadCodeAssist API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    pub cloudaicompanion_project: String,
    pub tier: SubscriptionTier,
}

// ================================
// Internal API Response Structures
// ================================

#[derive(Debug, Deserialize)]
struct FetchModelsResponse {
    models: HashMap<String, RawModelInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawModelInfo {
    display_name: String,
    #[serde(default)]
    quota_info: Option<RawQuotaInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawQuotaInfo {
    remaining_fraction: f64,
    reset_time: String, // ISO 8601 format
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoadCodeAssistResponse {
    cloudaicompanion_project: String,
    current_tier: Option<TierInfo>,
    paid_tier: Option<TierInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TierInfo {
    id: String,
    #[allow(dead_code)]
    quota_tier: Option<String>,
    #[allow(dead_code)]
    name: Option<String>,
    #[allow(dead_code)]
    slug: Option<String>,
}

// ================================
// QuotaFetcher Client
// ================================

/// HTTP client for quota API operations
pub struct QuotaFetcher {
    client: Client,
    api_base: String,
}

impl QuotaFetcher {
    /// Create new quota fetcher instance
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_base: API_BASE.to_string(),
        }
    }

    /// Fetch available models with quota information
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `project_id` - GCP project ID (e.g., "projects/123456789" or "bamboo-precept-lgxtn")
    ///
    /// # Returns
    /// HashMap of model_name -> ModelQuotaInfo
    ///
    /// # Errors
    /// - "Invalid access token" (401)
    /// - "Quota exhausted" (429)
    /// - "Access forbidden" (403)
    /// - Network errors
    /// - Invalid JSON response
    pub async fn fetch_available_models(
        &self,
        access_token: &str,
        project_id: &str,
    ) -> Result<HashMap<String, ModelQuotaInfo>, String> {
        let url = format!("{}:fetchAvailableModels", self.api_base);

        info!("Fetching quotas for project: {}", project_id);

        let response = self
            .client
            .post(&url)
            .bearer_auth(access_token)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "project": project_id
            }))
            .send()
            .await
            .map_err(|e| {
                error!("Network error fetching models: {}", e);
                format!("Network error: {}", e)
            })?;

        let status = response.status();

        // Handle error status codes
        if !status.is_success() {
            return match status.as_u16() {
                401 => {
                    error!("401 Unauthorized - token expired");
                    Err("Invalid access token".to_string())
                }
                429 => {
                    error!("429 Resource Exhausted");
                    Err("Quota exhausted".to_string())
                }
                403 => {
                    error!("403 Forbidden - access denied");
                    Err("Access forbidden".to_string())
                }
                _ => {
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    error!("API error {}: {}", status, error_text);
                    Err(format!("API error {}: {}", status, error_text))
                }
            };
        }

        // Parse successful response
        let api_response: FetchModelsResponse = response.json().await.map_err(|e| {
            error!("Failed to parse API response: {}", e);
            format!("Invalid API response: {}", e)
        })?;

        debug!("Successfully fetched {} models", api_response.models.len());

        // Convert to public data structures
        let mut result = HashMap::new();
        for (model_name, raw_info) in api_response.models {
            let quota_info = raw_info.quota_info.and_then(|raw_quota| {
                // Parse ISO 8601 reset time
                match DateTime::parse_from_rfc3339(&raw_quota.reset_time) {
                    Ok(reset_dt) => Some(QuotaInfo {
                        remaining_fraction: raw_quota.remaining_fraction,
                        reset_time: reset_dt.with_timezone(&Utc),
                    }),
                    Err(e) => {
                        error!("Failed to parse reset_time for {}: {}", model_name, e);
                        None
                    }
                }
            });

            result.insert(
                model_name.clone(),
                ModelQuotaInfo {
                    display_name: raw_info.display_name,
                    quota_info,
                },
            );
        }

        Ok(result)
    }

    /// Load code assist to determine subscription tier
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    ///
    /// # Returns
    /// SubscriptionInfo with project ID and tier
    ///
    /// # Errors
    /// - "Invalid access token" (401)
    /// - "Access forbidden" (403)
    /// - Network errors
    /// - Invalid JSON response
    pub async fn load_code_assist(&self, access_token: &str) -> Result<SubscriptionInfo, String> {
        let url = format!("{}:loadCodeAssist", self.api_base);

        info!("Loading code assist to detect subscription tier");

        let response = self
            .client
            .post(&url)
            .bearer_auth(access_token)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "metadata": {
                    "ideType": "ANTIGRAVITY"
                }
            }))
            .send()
            .await
            .map_err(|e| {
                error!("Network error loading code assist: {}", e);
                format!("Network error: {}", e)
            })?;

        let status = response.status();

        if !status.is_success() {
            return match status.as_u16() {
                401 => {
                    error!("401 Unauthorized - token expired");
                    Err("Invalid access token".to_string())
                }
                403 => {
                    error!("403 Forbidden - access denied");
                    Err("Access forbidden".to_string())
                }
                _ => {
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    error!("API error {}: {}", status, error_text);
                    Err(format!("API error {}: {}", status, error_text))
                }
            };
        }

        let api_response: LoadCodeAssistResponse = response.json().await.map_err(|e| {
            error!("Failed to parse loadCodeAssist response: {}", e);
            format!("Invalid API response: {}", e)
        })?;

        // Priority: paid_tier > current_tier (as per reverse engineering documentation)
        let tier_id = api_response
            .paid_tier
            .map(|t| t.id)
            .or_else(|| api_response.current_tier.map(|t| t.id))
            .unwrap_or_else(|| "FREE".to_string());

        let tier = SubscriptionTier::from_string(&tier_id);

        info!(
            "Subscription tier detected: {} for project: {}",
            tier.as_str(),
            api_response.cloudaicompanion_project
        );

        Ok(SubscriptionInfo {
            cloudaicompanion_project: api_response.cloudaicompanion_project,
            tier,
        })
    }
}

impl Default for QuotaFetcher {
    fn default() -> Self {
        Self::new()
    }
}

// ================================
// Unit Tests
// ================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_tier_from_string() {
        assert_eq!(
            SubscriptionTier::from_string("ULTRA"),
            SubscriptionTier::Ultra
        );
        assert_eq!(SubscriptionTier::from_string("PRO"), SubscriptionTier::Pro);
        assert_eq!(
            SubscriptionTier::from_string("FREE"),
            SubscriptionTier::Free
        );
        assert_eq!(
            SubscriptionTier::from_string("unknown"),
            SubscriptionTier::Free
        );
        assert_eq!(
            SubscriptionTier::from_string("ultra"),
            SubscriptionTier::Ultra
        ); // Case insensitive
    }

    #[test]
    fn test_subscription_tier_as_str() {
        assert_eq!(SubscriptionTier::Ultra.as_str(), "ULTRA");
        assert_eq!(SubscriptionTier::Pro.as_str(), "PRO");
        assert_eq!(SubscriptionTier::Free.as_str(), "FREE");
    }

    #[tokio::test]
    async fn test_quota_fetcher_creation() {
        let fetcher = QuotaFetcher::new();
        assert_eq!(fetcher.api_base, API_BASE);
    }

    #[tokio::test]
    async fn test_fetch_available_models_invalid_token() {
        let fetcher = QuotaFetcher::new();
        let result = fetcher
            .fetch_available_models("invalid_token_12345", "test-project")
            .await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        // Should be either "Invalid access token" (401) or network error
        assert!(
            error.contains("Invalid access token")
                || error.contains("Network error")
                || error.contains("API error")
        );
    }

    #[tokio::test]
    async fn test_load_code_assist_invalid_token() {
        let fetcher = QuotaFetcher::new();
        let result = fetcher.load_code_assist("invalid_token_12345").await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        // Should be either "Invalid access token" (401) or network error
        assert!(
            error.contains("Invalid access token")
                || error.contains("Network error")
                || error.contains("API error")
        );
    }

    #[test]
    fn test_quota_info_structure() {
        let now = Utc::now();
        let quota = QuotaInfo {
            remaining_fraction: 0.87,
            reset_time: now,
        };

        assert_eq!(quota.remaining_fraction, 0.87);
        assert_eq!(quota.reset_time, now);
    }

    #[test]
    fn test_model_quota_info_serialization() {
        let quota = QuotaInfo {
            remaining_fraction: 0.95,
            reset_time: DateTime::parse_from_rfc3339("2026-01-11T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        };

        let model_info = ModelQuotaInfo {
            display_name: "Gemini 2.5 Flash".to_string(),
            quota_info: Some(quota),
        };

        // Test serialization
        let json = serde_json::to_string(&model_info).unwrap();
        assert!(json.contains("Gemini 2.5 Flash"));
        assert!(json.contains("remaining_fraction"));

        // Test deserialization roundtrip
        let deserialized: ModelQuotaInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.display_name, "Gemini 2.5 Flash");
        assert!(deserialized.quota_info.is_some());
    }

    // Integration test markers (uncomment when running with real API)
    // These tests require valid credentials and should be run manually

    /*
    #[tokio::test]
    #[ignore] // Run with: cargo test test_fetch_models_real_api -- --ignored
    async fn test_fetch_models_real_api() {
        // Set valid credentials in environment variables
        let access_token = std::env::var("TEST_ACCESS_TOKEN").expect("TEST_ACCESS_TOKEN not set");
        let project_id = std::env::var("TEST_PROJECT_ID").unwrap_or_else(|_| "bamboo-precept-lgxtn".to_string());

        let fetcher = QuotaFetcher::new();
        let result = fetcher.fetch_available_models(&access_token, &project_id).await;

        assert!(result.is_ok());
        let models = result.unwrap();
        assert!(!models.is_empty());

        // Check that we got quota info for at least one model
        let has_quota_info = models.values().any(|m| m.quota_info.is_some());
        assert!(has_quota_info, "At least one model should have quota info");
    }

    #[tokio::test]
    #[ignore]
    async fn test_load_code_assist_real_api() {
        let access_token = std::env::var("TEST_ACCESS_TOKEN").expect("TEST_ACCESS_TOKEN not set");

        let fetcher = QuotaFetcher::new();
        let result = fetcher.load_code_assist(&access_token).await;

        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(!info.cloudaicompanion_project.is_empty());
        // Tier should be one of the valid tiers
        assert!(matches!(info.tier, SubscriptionTier::Free | SubscriptionTier::Pro | SubscriptionTier::Ultra));
    }
    */
}
