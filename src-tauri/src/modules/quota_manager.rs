//! Quota Manager Module
//!
//! Orchestrates quota monitoring with background sync task.
//!
//! # Architecture
//!
//! ```
//! ┌──────────────────────────────────────────────┐
//! │        QuotaManager (Orchestrator)           │
//! │  • Coordinates QuotaFetcher + QuotaCache     │
//! │  • Pre-request quota validation              │
//! │  • Background monitoring task                │
//! └──────────────────────────────────────────────┘
//!          │                    │
//!          ▼                    ▼
//!   ┌──────────────┐    ┌──────────────┐
//!   │ QuotaFetcher │    │  QuotaCache  │
//!   │  (API calls) │    │ (DashMap/TTL)│
//!   └──────────────┘    └──────────────┘
//! ```
//!
//! # Background Monitor
//!
//! - Runs every 5 minutes (configurable)
//! - Parallel quota sync for all accounts
//! - Graceful error handling (no panics)
//! - Automatic cache cleanup
//! - Low quota warnings (<10%)
//!
//! # Development Note
//!
//! This module will be integrated in Phase 2 by Dev 1 (TokenManager integration).
//!
//! # Example
//!
//! ```rust
//! use quota_manager::QuotaManager;
//! use std::sync::Arc;
//!
//! let manager = Arc::new(QuotaManager::new(300)); // 5-min TTL
//!
//! // Start background monitoring
//! let handle = manager.clone().start_background_monitor(
//!     tokens.clone(),
//!     300, // 5 minutes
//! );
//!
//! // Pre-request quota check
//! let decision = manager.check_quota(
//!     "account@gmail.com",
//!     "gemini-2.5-flash",
//!     &access_token,
//!     &project_id,
//! ).await?;
//! ```

#![allow(dead_code)] // Module will be integrated in Phase 2 (Dev 1 + Dev 3)

use crate::modules::quota_cache::{QuotaCache, QuotaInfo};
use crate::modules::quota_fetcher::{QuotaFetcher, SubscriptionTier};
use crate::proxy::token_manager::ProxyToken;

use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::interval;
use tracing::{debug, error, info, warn};

// ============================================================================
// Quota Decision Types
// ============================================================================

/// Decision result from quota validation
#[derive(Debug, Clone)]
pub enum QuotaDecision {
    /// Quota is healthy (>10% remaining)
    Proceed,

    /// Quota is low (0-10% remaining)
    LowQuota { remaining: f64, reset_time: String },

    /// Quota is exhausted (0% remaining)
    Exhausted { reset_time: String },
}

/// Monitor statistics for observability
#[derive(Debug, Clone, serde::Serialize)]
pub struct MonitorStats {
    pub total_accounts: usize,
    pub cached_quotas: usize,
    pub expired_entries: usize,
    pub exhausted_quotas: usize,
    pub last_sync_duration_ms: u64,
    pub sync_success_count: usize,
    pub sync_error_count: usize,
}

// ============================================================================
// Quota Manager
// ============================================================================

/// Quota management orchestrator
///
/// Coordinates quota fetching, caching, and background monitoring.
pub struct QuotaManager {
    /// Quota cache (DashMap-based)
    cache: Arc<QuotaCache>,

    /// API client for quota fetching
    fetcher: Arc<QuotaFetcher>,

    /// Subscription tier cache (account_id -> tier)
    /// Public for testing purposes (QUOTA-001-04)
    pub tier_cache: Arc<DashMap<String, SubscriptionTier>>,

    /// Last sync statistics
    last_sync_stats: Arc<tokio::sync::RwLock<SyncStats>>,
}

#[derive(Debug, Clone)]
struct SyncStats {
    duration_ms: u64,
    success_count: usize,
    error_count: usize,
}

impl Default for SyncStats {
    fn default() -> Self {
        Self {
            duration_ms: 0,
            success_count: 0,
            error_count: 0,
        }
    }
}

impl QuotaManager {
    /// Create new QuotaManager
    ///
    /// # Arguments
    ///
    /// * `cache_ttl_seconds` - TTL for quota cache entries (recommended: 300 = 5 minutes)
    pub fn new(cache_ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(QuotaCache::new(cache_ttl_seconds)),
            fetcher: Arc::new(QuotaFetcher::new()),
            tier_cache: Arc::new(DashMap::new()),
            last_sync_stats: Arc::new(tokio::sync::RwLock::new(SyncStats::default())),
        }
    }

    /// Start background quota monitoring task
    ///
    /// Returns JoinHandle for graceful shutdown control.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Shared reference to ProxyToken map
    /// * `interval_seconds` - Sync interval in seconds (recommended: 300 = 5 minutes)
    ///
    /// # Example
    ///
    /// ```rust
    /// let manager = Arc::new(QuotaManager::new(300));
    /// let handle = manager.clone().start_background_monitor(tokens, 300);
    ///
    /// // Later: graceful shutdown
    /// handle.abort();
    /// ```
    pub fn start_background_monitor(
        self: Arc<Self>,
        tokens: Arc<DashMap<String, ProxyToken>>,
        interval_seconds: u64,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(interval_seconds));

            info!(
                "Background quota monitor started (interval: {}s)",
                interval_seconds
            );

            loop {
                ticker.tick().await;

                if let Err(e) = self.sync_all_quotas(&tokens).await {
                    error!("Background quota sync failed: {}", e);
                    // Continue loop - do not panic
                }
            }
        })
    }

    /// Sync quotas for all accounts
    ///
    /// Executes parallel sync with timeout protection.
    /// Records sync statistics for monitoring.
    async fn sync_all_quotas(&self, tokens: &DashMap<String, ProxyToken>) -> Result<(), String> {
        let start = Instant::now();
        let mut success_count = 0;
        let mut error_count = 0;

        let account_count = tokens.len();
        debug!("Starting quota sync for {} accounts", account_count);

        // Collect sync tasks
        let mut tasks = Vec::new();

        for entry in tokens.iter() {
            let token = entry.value().clone();
            let manager = self.clone_arc();

            let task = tokio::spawn(async move { manager.sync_account_quota(&token).await });

            tasks.push(task);
        }

        // Wait for all tasks with individual timeouts
        for task in tasks {
            match tokio::time::timeout(Duration::from_secs(30), task).await {
                Ok(Ok(Ok(_))) => success_count += 1,
                Ok(Ok(Err(e))) => {
                    error_count += 1;
                    warn!("Account sync failed: {}", e);
                }
                Ok(Err(e)) => {
                    error_count += 1;
                    error!("Task panicked: {}", e);
                }
                Err(_) => {
                    error_count += 1;
                    error!("Account sync timeout (30s)");
                }
            }
        }

        let duration = start.elapsed();

        info!(
            "Quota sync complete: {} success, {} errors, took {:?}",
            success_count, error_count, duration
        );

        // Cleanup expired cache entries
        let removed = self.cache.cleanup_expired();
        if removed > 0 {
            debug!("Cleaned up {} expired cache entries", removed);
        }

        // Update sync statistics
        {
            let mut stats = self.last_sync_stats.write().await;
            stats.duration_ms = duration.as_millis() as u64;
            stats.success_count = success_count;
            stats.error_count = error_count;
        }

        Ok(())
    }

    /// Sync quota for single account
    ///
    /// Fetches quotas via API and updates cache.
    /// Logs low quota warnings (<10%).
    async fn sync_account_quota(&self, token: &ProxyToken) -> Result<(), String> {
        debug!("Syncing quota for account: {}", token.account_id);

        // Require project_id
        let project_id = token
            .project_id
            .as_ref()
            .ok_or_else(|| format!("Account {} missing project_id", token.account_id))?;

        // Fetch fresh quotas from API
        match self
            .fetcher
            .fetch_available_models(&token.access_token, project_id)
            .await
        {
            Ok(models) => {
                let mut updated = 0;

                for (model_id, info) in models {
                    if let Some(quota_info_raw) = info.quota_info {
                        let quota_info = QuotaInfo::new(
                            quota_info_raw.remaining_fraction,
                            quota_info_raw.reset_time,
                            info.display_name.clone(),
                        );

                        self.cache
                            .set(&token.account_id, &model_id, quota_info.clone());
                        updated += 1;

                        // Log low quota warnings
                        if quota_info_raw.remaining_fraction < 0.1 {
                            warn!(
                                "Low quota for {}/{}: {:.1}%",
                                token.account_id,
                                model_id,
                                quota_info_raw.remaining_fraction * 100.0
                            );
                        }
                    }
                }

                debug!(
                    "Updated {} quotas for account {}",
                    updated, token.account_id
                );

                Ok(())
            }
            Err(e) => {
                error!("Failed to fetch quotas for {}: {}", token.account_id, e);
                Err(e)
            }
        }
    }

    /// Clone Arc reference for use in spawned tasks
    fn clone_arc(&self) -> Arc<Self> {
        Arc::new(Self {
            cache: Arc::clone(&self.cache),
            fetcher: Arc::clone(&self.fetcher),
            tier_cache: Arc::clone(&self.tier_cache),
            last_sync_stats: Arc::clone(&self.last_sync_stats),
        })
    }

    /// Get monitor statistics
    ///
    /// Returns current cache state and sync performance metrics.
    pub async fn get_monitor_stats(&self) -> MonitorStats {
        let cache_stats = self.cache.stats();
        let sync_stats = self.last_sync_stats.read().await;

        MonitorStats {
            total_accounts: cache_stats.total_entries,
            cached_quotas: cache_stats.active_entries,
            expired_entries: cache_stats.expired_entries,
            exhausted_quotas: cache_stats.exhausted_quotas,
            last_sync_duration_ms: sync_stats.duration_ms,
            sync_success_count: sync_stats.success_count,
            sync_error_count: sync_stats.error_count,
        }
    }

    // ========================================================================
    // Pre-Request Quota Validation (Phase 2 - Dev 1 + Dev 3)
    // ========================================================================

    /// Check quota before making request
    ///
    /// Primary validation method for TokenManager integration.
    /// Implements cache-first strategy with automatic API fallback.
    ///
    /// # Flow
    /// 1. Check cache first (<1ms if hit)
    /// 2. If miss, fetch from API (<250ms)
    /// 3. Update cache for all models
    /// 4. Make decision based on remaining fraction
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account ID (email)
    /// * `model` - Model ID (e.g., "gemini-2.5-flash")
    /// * `access_token` - Valid OAuth2 access token
    /// * `project_id` - GCP project ID
    ///
    /// # Returns
    ///
    /// * `QuotaDecision::Proceed` - Quota healthy (>10%)
    /// * `QuotaDecision::LowQuota` - Quota low (0-10%)
    /// * `QuotaDecision::Exhausted` - Quota exhausted (0%)
    ///
    /// # Example
    ///
    /// ```rust
    /// match manager.check_quota(account_id, model, token, project).await? {
    ///     QuotaDecision::Proceed => { /* use account */ }
    ///     QuotaDecision::LowQuota { remaining, .. } => {
    ///         warn!("Low quota: {:.1}%", remaining * 100.0);
    ///         /* proceed with caution */
    ///     }
    ///     QuotaDecision::Exhausted { reset_time } => {
    ///         error!("Quota exhausted until {}", reset_time);
    ///         /* switch account */
    ///     }
    /// }
    /// ```
    pub async fn check_quota(
        &self,
        account_id: &str,
        model: &str,
        access_token: &str,
        project_id: &str,
    ) -> Result<QuotaDecision, String> {
        // 1. Try cache first (fast path)
        if let Some(quota) = self.cache.get(account_id, model) {
            debug!(
                "Quota cache hit for {}/{}: {:.1}%",
                account_id,
                model,
                quota.remaining_fraction * 100.0
            );
            return Ok(self.make_decision(&quota));
        }

        // 2. Cache miss - fetch from API
        debug!(
            "Quota cache miss for {}/{}, fetching from API",
            account_id, model
        );

        let models = self
            .fetcher
            .fetch_available_models(access_token, project_id)
            .await?;

        // 3. Update cache for all models
        let quota_map: HashMap<String, QuotaInfo> = models
            .into_iter()
            .filter_map(|(name, info)| {
                info.quota_info.map(|qi| {
                    (
                        name,
                        QuotaInfo::new(qi.remaining_fraction, qi.reset_time, info.display_name),
                    )
                })
            })
            .collect();

        self.cache.set_all(account_id, quota_map.clone());

        // 4. Get specific model quota
        let quota = quota_map
            .get(model)
            .ok_or_else(|| format!("Model {} not found in quota data", model))?;

        Ok(self.make_decision(quota))
    }

    /// Make decision based on remaining fraction
    ///
    /// # Thresholds
    /// - ≥10%: Proceed (healthy quota)
    /// - 0-10%: LowQuota (warning, but proceed)
    /// - 0%: Exhausted (switch account)
    ///
    /// # Arguments
    ///
    /// * `quota` - QuotaInfo with remaining_fraction and reset_time
    fn make_decision(&self, quota: &QuotaInfo) -> QuotaDecision {
        if quota.remaining_fraction >= 0.10 {
            QuotaDecision::Proceed
        } else if quota.remaining_fraction > 0.0 {
            QuotaDecision::LowQuota {
                remaining: quota.remaining_fraction,
                reset_time: quota.reset_time.to_rfc3339(),
            }
        } else {
            QuotaDecision::Exhausted {
                reset_time: quota.reset_time.to_rfc3339(),
            }
        }
    }

    /// Get all quotas for account (for dashboard/UI)
    ///
    /// Returns cached quotas if available, otherwise fetches from API.
    pub async fn get_all_quotas(
        &self,
        account_id: &str,
        access_token: &str,
        project_id: &str,
    ) -> Result<HashMap<String, QuotaInfo>, String> {
        // Try cache first
        let cached = self.cache.get_all(account_id);
        if !cached.is_empty() {
            debug!(
                "Cache hit for all quotas: {} (count: {})",
                account_id,
                cached.len()
            );
            return Ok(cached);
        }

        // Cache miss - fetch from API
        debug!(
            "Cache miss for all quotas: {}, fetching from API",
            account_id
        );

        let models = self
            .fetcher
            .fetch_available_models(access_token, project_id)
            .await?;

        let mut result = HashMap::new();
        for (model_id, info) in models {
            if let Some(quota_info) = info.quota_info {
                let quota = QuotaInfo::new(
                    quota_info.remaining_fraction,
                    quota_info.reset_time,
                    info.display_name,
                );
                self.cache.set(account_id, &model_id, quota.clone());
                result.insert(model_id, quota);
            }
        }

        info!("Fetched {} quotas for account {}", result.len(), account_id);

        Ok(result)
    }

    /// Get subscription tier for account
    pub async fn get_subscription_tier(
        &self,
        account_id: &str,
        access_token: &str,
    ) -> Result<SubscriptionTier, String> {
        // Check tier cache
        if let Some(tier) = self.tier_cache.get(account_id) {
            debug!("Tier cache hit for {}: {:?}", account_id, tier.value());
            return Ok(tier.value().clone());
        }

        // Fetch from API
        debug!("Tier cache miss for {}, fetching from API", account_id);

        let sub_info = self.fetcher.load_code_assist(access_token).await?;
        self.tier_cache
            .insert(account_id.to_string(), sub_info.tier.clone());

        info!("Fetched tier for {}: {:?}", account_id, sub_info.tier);

        Ok(sub_info.tier)
    }

    /// Clear tier cache for specific account
    pub fn clear_tier_cache(&self, account_id: &str) -> bool {
        self.tier_cache.remove(account_id).is_some()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::path::PathBuf;

    fn create_test_token(account_id: &str, project_id: Option<&str>) -> ProxyToken {
        ProxyToken {
            account_id: account_id.to_string(),
            access_token: "test_token".to_string(),
            refresh_token: "test_refresh".to_string(),
            expires_in: 3600,
            timestamp: 9999999999,
            email: format!("{}@test.com", account_id),
            account_path: PathBuf::from("/tmp/test.json"),
            project_id: project_id.map(|s| s.to_string()),
            subscription_tier: Some("PRO".to_string()),
        }
    }

    #[tokio::test]
    async fn test_quota_manager_creation() {
        let manager = QuotaManager::new(300);
        assert_eq!(manager.tier_cache.len(), 0);
    }

    #[tokio::test]
    async fn test_sync_account_quota_missing_project_id() {
        let manager = QuotaManager::new(300);
        let token = create_test_token("acc1", None);

        let result = manager.sync_account_quota(&token).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing project_id"));
    }

    #[tokio::test]
    async fn test_sync_account_quota_invalid_token() {
        let manager = QuotaManager::new(300);
        let token = create_test_token("acc1", Some("test-project"));

        // Invalid token should return error (401)
        let result = manager.sync_account_quota(&token).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sync_all_quotas_empty_accounts() {
        let manager = QuotaManager::new(300);
        let tokens = Arc::new(DashMap::new());

        let result = manager.sync_all_quotas(&tokens).await;
        assert!(result.is_ok());

        let stats = manager.get_monitor_stats().await;
        assert_eq!(stats.sync_success_count, 0);
        assert_eq!(stats.sync_error_count, 0);
    }

    #[tokio::test]
    async fn test_sync_all_quotas_with_invalid_accounts() {
        let manager = QuotaManager::new(300);
        let tokens = Arc::new(DashMap::new());

        tokens.insert(
            "acc1".to_string(),
            create_test_token("acc1", Some("project1")),
        );
        tokens.insert(
            "acc2".to_string(),
            create_test_token("acc2", Some("project2")),
        );

        let result = manager.sync_all_quotas(&tokens).await;
        assert!(result.is_ok());

        // All should fail with invalid tokens
        let stats = manager.get_monitor_stats().await;
        assert_eq!(stats.sync_success_count, 0);
        assert_eq!(stats.sync_error_count, 2);
    }

    #[tokio::test]
    async fn test_background_monitor_starts() {
        let manager = Arc::new(QuotaManager::new(300));
        let tokens = Arc::new(DashMap::new());

        // Start monitor (will run indefinitely)
        let handle = manager.clone().start_background_monitor(tokens.clone(), 1);

        // Wait a bit to ensure task started
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Abort the task
        handle.abort();

        // Wait for task to actually finish after abort
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Task should be aborted
        assert!(handle.is_finished());
    }

    #[tokio::test]
    async fn test_get_monitor_stats() {
        let manager = QuotaManager::new(300);

        let stats = manager.get_monitor_stats().await;
        assert_eq!(stats.total_accounts, 0);
        assert_eq!(stats.cached_quotas, 0);
        assert_eq!(stats.last_sync_duration_ms, 0);
        assert_eq!(stats.sync_success_count, 0);
        assert_eq!(stats.sync_error_count, 0);
    }

    #[tokio::test]
    async fn test_cache_cleanup_during_sync() {
        let manager = QuotaManager::new(1); // 1 second TTL
        let tokens = Arc::new(DashMap::new());

        // Add expired cache entry
        manager.cache.set(
            "acc1",
            "model1",
            QuotaInfo::new(0.8, Utc::now(), "Test".to_string()),
        );

        // Wait for expiration
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Sync should cleanup expired entries
        let result = manager.sync_all_quotas(&tokens).await;
        assert!(result.is_ok());

        // Cache should be empty after cleanup
        assert_eq!(manager.cache.stats().active_entries, 0);
    }

    #[tokio::test]
    async fn test_sync_with_missing_project_id() {
        let manager = QuotaManager::new(300);
        let tokens = Arc::new(DashMap::new());

        // Account without project_id
        tokens.insert("acc1".to_string(), create_test_token("acc1", None));

        let result = manager.sync_all_quotas(&tokens).await;
        assert!(result.is_ok());

        // Should have 1 error (missing project_id)
        let stats = manager.get_monitor_stats().await;
        assert_eq!(stats.sync_error_count, 1);
        assert_eq!(stats.sync_success_count, 0);
    }

    #[tokio::test]
    async fn test_parallel_sync_performance() {
        let manager = QuotaManager::new(300);
        let tokens = Arc::new(DashMap::new());

        // Add 10 accounts
        for i in 0..10 {
            tokens.insert(
                format!("acc{}", i),
                create_test_token(&format!("acc{}", i), Some("project")),
            );
        }

        let start = Instant::now();
        let result = manager.sync_all_quotas(&tokens).await;
        let duration = start.elapsed();

        assert!(result.is_ok());

        // Should complete within reasonable time (parallel execution)
        // 10 accounts * 30s timeout would be 300s sequential
        // Parallel should be ~30s (limited by slowest account)
        assert!(duration.as_secs() < 60);

        let stats = manager.get_monitor_stats().await;
        assert_eq!(stats.sync_error_count, 10); // All fail with invalid tokens
    }
}
