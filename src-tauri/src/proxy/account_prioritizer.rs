// ============================================================================
// Epic-001: Account Prioritization Logic (QUOTA-001-04)
// ============================================================================
//
// Multi-factor account prioritization for intelligent account selection.
//
// Priority Order:
// 1. Subscription Tier (ULTRA > PRO > FREE)
// 2. Quota Remaining (higher is better)
// 3. Last 429 Time (older 429 is better, no 429 is best)
//
// Usage:
// ```rust
// let mut accounts = vec![...];
// let quotas = HashMap::from([("acc1".to_string(), 0.8), ...]);
// AccountPrioritizer::prioritize_accounts(&mut accounts, &quotas);
// // accounts now sorted by priority (highest first)
// ```

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::proxy::rate_limit::RateLimitTracker;
use crate::proxy::token_manager::ProxyToken;

// ============================================================================
// Subscription Tier Definition
// ============================================================================

/// Subscription tier for Google Antigravity accounts
///
/// Each tier has different quota limits and reset frequencies:
/// - FREE: Basic quota, daily reset
/// - PRO: ~3x quota vs FREE, daily reset
/// - ULTRA: ~10x quota vs FREE, daily reset + advanced features
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum SubscriptionTier {
    FREE = 0,
    PRO = 1,
    ULTRA = 2,
}

impl SubscriptionTier {
    /// Get priority score (higher is better)
    pub fn priority_score(&self) -> u8 {
        match self {
            SubscriptionTier::FREE => 0,
            SubscriptionTier::PRO => 1,
            SubscriptionTier::ULTRA => 2,
        }
    }

    /// Get reset frequency description
    pub fn reset_frequency(&self) -> &'static str {
        match self {
            SubscriptionTier::FREE => "daily",
            SubscriptionTier::PRO => "daily",
            SubscriptionTier::ULTRA => "daily",
        }
    }

    /// Get quota multiplier relative to FREE tier
    pub fn quota_multiplier(&self) -> f64 {
        match self {
            SubscriptionTier::FREE => 1.0,
            SubscriptionTier::PRO => 3.0,    // ~3x quota vs FREE
            SubscriptionTier::ULTRA => 10.0, // ~10x quota vs FREE
        }
    }

    /// Parse from string (case-insensitive)
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ULTRA" => SubscriptionTier::ULTRA,
            "PRO" => SubscriptionTier::PRO,
            _ => SubscriptionTier::FREE,
        }
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            SubscriptionTier::FREE => "FREE",
            SubscriptionTier::PRO => "PRO",
            SubscriptionTier::ULTRA => "ULTRA",
        }
    }
}

impl From<&str> for SubscriptionTier {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<Option<&String>> for SubscriptionTier {
    fn from(opt: Option<&String>) -> Self {
        opt.map(|s| Self::from_str(s))
            .unwrap_or(SubscriptionTier::FREE)
    }
}

// ============================================================================
// Account Prioritizer
// ============================================================================

/// Account prioritization logic for intelligent account selection
pub struct AccountPrioritizer;

impl AccountPrioritizer {
    /// Sort accounts by priority (highest priority first)
    ///
    /// Priority factors (in order):
    /// 1. Subscription tier (ULTRA > PRO > FREE)
    /// 2. Rate limit status (not limited > limited with shorter wait)
    /// 3. Quota remaining (higher is better, if tier and rate limit are same)
    ///
    /// Rationale:
    /// - Tier is most important: ULTRA accounts have 10x quota vs FREE
    /// - Rate limit status is critical: a limited account is temporarily unusable
    /// - Quota is the tiebreaker: prefer accounts with more remaining quota
    ///
    /// # Arguments
    /// * `accounts` - Mutable slice of accounts to sort in-place
    /// * `quotas` - Map of account_id -> remaining quota fraction (0.0-1.0)
    /// * `rate_limit_tracker` - Rate limit tracker for checking 429 status
    ///
    /// # Example
    /// ```
    /// let mut accounts = vec![
    ///     ProxyToken { account_id: "free1".to_string(), subscription_tier: Some("FREE".to_string()), ... },
    ///     ProxyToken { account_id: "ultra1".to_string(), subscription_tier: Some("ULTRA".to_string()), ... },
    ///     ProxyToken { account_id: "pro1".to_string(), subscription_tier: Some("PRO".to_string()), ... },
    /// ];
    /// let quotas = HashMap::from([
    ///     ("free1".to_string(), 0.9),
    ///     ("ultra1".to_string(), 0.5),
    ///     ("pro1".to_string(), 0.8),
    /// ]);
    /// let rate_tracker = RateLimitTracker::new();
    ///
    /// AccountPrioritizer::prioritize_accounts(&mut accounts, &quotas, &rate_tracker);
    /// // Result: [ultra1 (tier 2, not limited), pro1 (tier 1, not limited), free1 (tier 0, not limited)]
    /// ```
    pub fn prioritize_accounts(
        accounts: &mut [ProxyToken],
        quotas: &HashMap<String, f64>,
        rate_limit_tracker: &RateLimitTracker,
    ) {
        accounts.sort_by(|a, b| Self::compare_priority(a, b, quotas, rate_limit_tracker));
    }

    /// Compare two accounts for priority ordering
    ///
    /// Returns Ordering::Less if `a` should come before `b` (higher priority)
    fn compare_priority(
        a: &ProxyToken,
        b: &ProxyToken,
        quotas: &HashMap<String, f64>,
        rate_limit_tracker: &RateLimitTracker,
    ) -> Ordering {
        // 1. Compare subscription tier (higher tier = higher priority)
        let tier_cmp = Self::compare_tiers(&a.subscription_tier, &b.subscription_tier);
        if tier_cmp != Ordering::Equal {
            return tier_cmp;
        }

        // 2. Compare rate limit status (not limited > limited with shorter wait)
        //    This takes precedence over quota because a limited account is unusable
        let rate_cmp =
            Self::compare_rate_limit_status(&a.account_id, &b.account_id, rate_limit_tracker);
        if rate_cmp != Ordering::Equal {
            return rate_cmp;
        }

        // 3. Compare quota remaining (higher quota = higher priority, if tier and rate limit status are same)
        let quota_a = quotas.get(&a.account_id).copied().unwrap_or(0.0);
        let quota_b = quotas.get(&b.account_id).copied().unwrap_or(0.0);
        quota_b.partial_cmp(&quota_a).unwrap_or(Ordering::Equal)
    }

    /// Compare subscription tiers (higher tier = higher priority)
    ///
    /// Returns Ordering::Less if `a` has higher priority than `b`
    fn compare_tiers(a: &Option<String>, b: &Option<String>) -> Ordering {
        let tier_a = SubscriptionTier::from(a.as_ref());
        let tier_b = SubscriptionTier::from(b.as_ref());

        // Reverse comparison: higher tier value = higher priority
        tier_b.cmp(&tier_a)
    }

    /// Compare rate limit status (not limited > limited with shorter wait)
    ///
    /// Returns Ordering::Less if `a` has higher priority than `b`
    fn compare_rate_limit_status(
        a_id: &str,
        b_id: &str,
        rate_limit_tracker: &RateLimitTracker,
    ) -> Ordering {
        let wait_a = rate_limit_tracker.get_remaining_wait(a_id);
        let wait_b = rate_limit_tracker.get_remaining_wait(b_id);

        match (wait_a, wait_b) {
            // Both not limited: equal priority
            (0, 0) => Ordering::Equal,
            // a limited, b not: b has higher priority
            (_, 0) if wait_a > 0 => Ordering::Greater,
            // a not limited, b is: a has higher priority
            (0, _) if wait_b > 0 => Ordering::Less,
            // Both limited: shorter wait = higher priority
            _ => wait_a.cmp(&wait_b),
        }
    }
}

// ============================================================================
// Tier Detection Helpers
// ============================================================================

/// Extract subscription tier from ProxyToken
pub fn extract_tier(token: &ProxyToken) -> SubscriptionTier {
    SubscriptionTier::from(token.subscription_tier.as_ref())
}

/// Get tier priority score from ProxyToken
pub fn get_tier_priority(token: &ProxyToken) -> u8 {
    extract_tier(token).priority_score()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_token(account_id: &str, tier: Option<&str>) -> ProxyToken {
        ProxyToken {
            account_id: account_id.to_string(),
            access_token: "test_token".to_string(),
            refresh_token: "test_refresh".to_string(),
            expires_in: 3600,
            timestamp: 9999999999,
            email: format!("{}@test.com", account_id),
            account_path: PathBuf::from("/tmp/test.json"),
            project_id: Some("test-project".to_string()),
            subscription_tier: tier.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_subscription_tier_priority() {
        assert_eq!(SubscriptionTier::FREE.priority_score(), 0);
        assert_eq!(SubscriptionTier::PRO.priority_score(), 1);
        assert_eq!(SubscriptionTier::ULTRA.priority_score(), 2);
    }

    #[test]
    fn test_subscription_tier_multiplier() {
        assert_eq!(SubscriptionTier::FREE.quota_multiplier(), 1.0);
        assert_eq!(SubscriptionTier::PRO.quota_multiplier(), 3.0);
        assert_eq!(SubscriptionTier::ULTRA.quota_multiplier(), 10.0);
    }

    #[test]
    fn test_subscription_tier_from_str() {
        assert_eq!(SubscriptionTier::from_str("ULTRA"), SubscriptionTier::ULTRA);
        assert_eq!(SubscriptionTier::from_str("PRO"), SubscriptionTier::PRO);
        assert_eq!(SubscriptionTier::from_str("FREE"), SubscriptionTier::FREE);
        assert_eq!(SubscriptionTier::from_str("ultra"), SubscriptionTier::ULTRA);
        assert_eq!(SubscriptionTier::from_str("pro"), SubscriptionTier::PRO);
        assert_eq!(
            SubscriptionTier::from_str("unknown"),
            SubscriptionTier::FREE
        );
    }

    #[test]
    fn test_prioritize_ultra_over_free() {
        let mut accounts = vec![
            create_test_token("free1", Some("FREE")),
            create_test_token("ultra1", Some("ULTRA")),
        ];
        let quotas = HashMap::from([("free1".to_string(), 0.9), ("ultra1".to_string(), 0.5)]);
        let rate_tracker = RateLimitTracker::new();

        AccountPrioritizer::prioritize_accounts(&mut accounts, &quotas, &rate_tracker);

        // ULTRA should come first despite lower quota
        assert_eq!(accounts[0].account_id, "ultra1");
        assert_eq!(accounts[1].account_id, "free1");
    }

    #[test]
    fn test_prioritize_higher_quota_same_tier() {
        let mut accounts = vec![
            create_test_token("pro1", Some("PRO")),
            create_test_token("pro2", Some("PRO")),
        ];
        let quotas = HashMap::from([
            ("pro1".to_string(), 0.2), // Low quota
            ("pro2".to_string(), 0.8), // High quota
        ]);
        let rate_tracker = RateLimitTracker::new();

        AccountPrioritizer::prioritize_accounts(&mut accounts, &quotas, &rate_tracker);

        // Higher quota should come first when tier is same
        assert_eq!(accounts[0].account_id, "pro2");
        assert_eq!(accounts[1].account_id, "pro1");
    }

    #[test]
    fn test_prioritize_no_rate_limit_over_limited() {
        use crate::proxy::rate_limit::RateLimitReason;
        use std::time::SystemTime;

        let mut accounts = vec![
            create_test_token("acc1", Some("PRO")), // Will be rate-limited
            create_test_token("acc2", Some("PRO")), // Not limited
        ];
        let quotas = HashMap::from([("acc1".to_string(), 0.8), ("acc2".to_string(), 0.8)]);
        let rate_tracker = RateLimitTracker::new();

        // Simulate acc1 being rate-limited
        let reset_time = SystemTime::now() + std::time::Duration::from_secs(300);
        rate_tracker.set_lockout_until("acc1", reset_time, RateLimitReason::QuotaExhausted, None);

        AccountPrioritizer::prioritize_accounts(&mut accounts, &quotas, &rate_tracker);

        // Not limited should come first
        assert_eq!(accounts[0].account_id, "acc2");
        assert_eq!(accounts[1].account_id, "acc1");
    }

    #[test]
    fn test_prioritize_shorter_wait_over_longer_wait() {
        use crate::proxy::rate_limit::RateLimitReason;
        use std::time::SystemTime;

        let mut accounts = vec![
            create_test_token("acc1", Some("PRO")), // Long wait
            create_test_token("acc2", Some("PRO")), // Short wait
        ];
        let quotas = HashMap::from([("acc1".to_string(), 0.8), ("acc2".to_string(), 0.8)]);
        let rate_tracker = RateLimitTracker::new();

        // acc1: 600s wait
        let reset_time_long = SystemTime::now() + std::time::Duration::from_secs(600);
        rate_tracker.set_lockout_until(
            "acc1",
            reset_time_long,
            RateLimitReason::QuotaExhausted,
            None,
        );

        // acc2: 60s wait
        let reset_time_short = SystemTime::now() + std::time::Duration::from_secs(60);
        rate_tracker.set_lockout_until(
            "acc2",
            reset_time_short,
            RateLimitReason::QuotaExhausted,
            None,
        );

        AccountPrioritizer::prioritize_accounts(&mut accounts, &quotas, &rate_tracker);

        // Shorter wait should come first
        assert_eq!(accounts[0].account_id, "acc2");
        assert_eq!(accounts[1].account_id, "acc1");
    }

    #[test]
    fn test_prioritize_complex_scenario() {
        use crate::proxy::rate_limit::RateLimitReason;
        use std::time::SystemTime;

        // Complex scenario: multiple tiers, quotas, and rate limits
        let mut accounts = vec![
            create_test_token("free_high_quota", Some("FREE")),
            create_test_token("pro_limited", Some("PRO")),
            create_test_token("ultra_low_quota", Some("ULTRA")),
            create_test_token("pro_not_limited", Some("PRO")),
        ];

        let quotas = HashMap::from([
            ("free_high_quota".to_string(), 0.9),
            ("pro_limited".to_string(), 0.8),
            ("ultra_low_quota".to_string(), 0.1),
            ("pro_not_limited".to_string(), 0.7),
        ]);

        let rate_tracker = RateLimitTracker::new();

        // pro_limited has 300s wait
        let reset_time = SystemTime::now() + std::time::Duration::from_secs(300);
        rate_tracker.set_lockout_until(
            "pro_limited",
            reset_time,
            RateLimitReason::QuotaExhausted,
            None,
        );

        AccountPrioritizer::prioritize_accounts(&mut accounts, &quotas, &rate_tracker);

        // Expected order:
        // 1. ultra_low_quota (tier 2, no rate limit, despite low quota)
        // 2. pro_limited (tier 1, quota 0.8, not limited - wait is 0 after being set)
        //    Note: pro_limited has higher quota (0.8) than pro_not_limited (0.7)
        // 3. pro_not_limited (tier 1, quota 0.7, not limited)
        // 4. free_high_quota (tier 0, despite high quota)
        //
        // Wait, let me reconsider: when both are not limited (wait = 0),
        // higher quota should come first. So pro_limited (0.8) > pro_not_limited (0.7).
        // But pro_limited has a rate limit set (300s wait), so it should sort differently.
        assert_eq!(accounts[0].account_id, "ultra_low_quota");
        // The next two depend on rate limit status:
        // - pro_not_limited: tier 1, quota 0.7, wait = 0
        // - pro_limited: tier 1, quota 0.8, wait = 300s
        // Not limited (wait = 0) should come before limited (wait > 0)
        assert_eq!(accounts[1].account_id, "pro_not_limited");
        assert_eq!(accounts[2].account_id, "pro_limited");
        assert_eq!(accounts[3].account_id, "free_high_quota");
    }

    #[test]
    fn test_extract_tier() {
        let ultra_token = create_test_token("ultra1", Some("ULTRA"));
        let pro_token = create_test_token("pro1", Some("PRO"));
        let free_token = create_test_token("free1", Some("FREE"));
        let none_token = create_test_token("none1", None);

        assert_eq!(extract_tier(&ultra_token), SubscriptionTier::ULTRA);
        assert_eq!(extract_tier(&pro_token), SubscriptionTier::PRO);
        assert_eq!(extract_tier(&free_token), SubscriptionTier::FREE);
        assert_eq!(extract_tier(&none_token), SubscriptionTier::FREE);
    }

    #[test]
    fn test_get_tier_priority() {
        let ultra_token = create_test_token("ultra1", Some("ULTRA"));
        let pro_token = create_test_token("pro1", Some("PRO"));
        let free_token = create_test_token("free1", Some("FREE"));

        assert_eq!(get_tier_priority(&ultra_token), 2);
        assert_eq!(get_tier_priority(&pro_token), 1);
        assert_eq!(get_tier_priority(&free_token), 0);
    }
}
