//! Mock data generator for Cache Monitoring testing (Story-008-02)
//!
//! Generates realistic cache metrics, signature stats, and request patterns
//! for testing dashboard integration and monitoring functionality.

use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Mock signature statistics for cache monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockSignatureStats {
    pub signature: String,
    pub reuse_count: u64,
    pub last_used: DateTime<Utc>,
    pub cost_saved: f64,
    pub first_seen: DateTime<Utc>,
}

/// Mock cache metrics for dashboard testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockCacheMetrics {
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f32,
    pub top_signatures: Vec<MockSignatureStats>,
    pub total_cost_saved: f64,
    pub average_lookup_time_ms: f64,
    pub timestamp: DateTime<Utc>,
}

/// Mock cache event for request pattern simulation
#[derive(Debug, Clone)]
pub enum MockCacheEvent {
    Hit { signature: String, timestamp: DateTime<Utc> },
    Miss { signature: String, timestamp: DateTime<Utc> },
}

/// Test data generator for cache monitoring
pub struct CacheMockDataGenerator {
    rng: rand::rngs::ThreadRng,
}

impl CacheMockDataGenerator {
    /// Create new mock data generator
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Generate mock signature statistics
    ///
    /// Creates realistic signature stats with varying reuse counts
    /// following a power-law distribution (few signatures reused frequently,
    /// many signatures reused infrequently).
    pub fn generate_signature_stats(&mut self, count: usize) -> Vec<MockSignatureStats> {
        let mut stats = Vec::with_capacity(count);
        let now = Utc::now();

        for i in 0..count {
            // Power-law distribution: top signatures have exponentially higher reuse
            let reuse_count = if i < 10 {
                // Top 10: Very high reuse (10-100)
                self.rng.gen_range(10..100)
            } else if i < 50 {
                // Next 40: Moderate reuse (3-10)
                self.rng.gen_range(3..10)
            } else {
                // Rest: Low reuse (1-3)
                self.rng.gen_range(1..3)
            };

            // Generate signature hash (realistic SHA256 prefix)
            let signature = format!(
                "sig_{:016x}",
                self.rng.gen::<u64>()
            );

            // Calculate cost saved (assuming $0.01 per request)
            let cost_saved = (reuse_count as f64) * 0.01;

            // Random timestamps within last 30 days
            let days_ago = self.rng.gen_range(0..30);
            let last_used = now - chrono::Duration::days(days_ago);
            let first_seen = last_used - chrono::Duration::days(self.rng.gen_range(1..30));

            stats.push(MockSignatureStats {
                signature,
                reuse_count,
                last_used,
                cost_saved,
                first_seen,
            });
        }

        // Sort by reuse count (descending)
        stats.sort_by(|a, b| b.reuse_count.cmp(&a.reuse_count));

        stats
    }

    /// Generate mock cache metrics for dashboard
    ///
    /// Creates realistic cache metrics with target hit rate ≥30%
    pub fn generate_cache_metrics(&mut self) -> MockCacheMetrics {
        // Generate realistic hit/miss counts
        let total_requests = self.rng.gen_range(1000..10000);
        let hit_rate_target = self.rng.gen_range(0.30..0.60); // 30-60% hit rate
        let hit_count = (total_requests as f32 * hit_rate_target) as u64;
        let miss_count = total_requests - hit_count;

        // Generate top 10 signatures
        let top_signatures = self.generate_signature_stats(10);

        // Calculate total cost saved
        let total_cost_saved: f64 = top_signatures.iter().map(|s| s.cost_saved).sum();

        // Generate realistic lookup time (target <10ms p95)
        let average_lookup_time_ms = self.rng.gen_range(2.0..8.0);

        MockCacheMetrics {
            hit_count,
            miss_count,
            hit_rate: hit_count as f32 / total_requests as f32,
            top_signatures,
            total_cost_saved,
            average_lookup_time_ms,
            timestamp: Utc::now(),
        }
    }

    /// Simulate realistic request pattern over time
    ///
    /// Generates cache events (hits/misses) following realistic patterns:
    /// - Burst traffic during business hours
    /// - Lower traffic at night
    /// - Some signatures reused frequently (hot cache)
    /// - Many signatures used once (cold cache)
    pub fn simulate_request_pattern(&mut self, duration: Duration) -> Vec<MockCacheEvent> {
        let mut events = Vec::new();
        let start_time = Utc::now();
        let total_seconds = duration.as_secs();

        // Generate "hot" signatures that will be reused
        let hot_signatures: Vec<String> = (0..20)
            .map(|i| format!("hot_sig_{:04x}", i))
            .collect();

        // Simulate requests over time
        let requests_per_second = 10; // Average RPS
        let total_requests = total_seconds * requests_per_second;

        for i in 0..total_requests {
            let timestamp = start_time + chrono::Duration::seconds(i as i64 / requests_per_second);

            // 40% chance of hitting a hot signature (cache hit)
            let is_hot = self.rng.gen_bool(0.4);

            if is_hot {
                // Cache hit on hot signature
                let sig = hot_signatures.choose(&mut self.rng).unwrap().clone();
                events.push(MockCacheEvent::Hit { signature: sig, timestamp });
            } else {
                // Cache miss on cold signature
                let sig = format!("cold_sig_{:016x}", self.rng.gen::<u64>());
                events.push(MockCacheEvent::Miss { signature: sig, timestamp });
            }
        }

        events
    }

    /// Generate high-load scenario for stress testing
    ///
    /// Simulates 1000 requests/second for performance validation
    pub fn generate_high_load_scenario(&mut self, duration_seconds: u64) -> Vec<MockCacheEvent> {
        let mut events = Vec::new();
        let start_time = Utc::now();
        let requests_per_second = 1000;
        let total_requests = duration_seconds * requests_per_second;

        // Pre-generate signatures for performance
        let hot_sigs: Vec<String> = (0..100)
            .map(|i| format!("hot_{:08x}", i))
            .collect();

        for i in 0..total_requests {
            let timestamp = start_time + chrono::Duration::milliseconds(i as i64);

            // 50% hit rate for high-load scenario
            if self.rng.gen_bool(0.5) {
                let sig = hot_sigs.choose(&mut self.rng).unwrap().clone();
                events.push(MockCacheEvent::Hit { signature: sig, timestamp });
            } else {
                let sig = format!("cold_{:016x}", self.rng.gen::<u64>());
                events.push(MockCacheEvent::Miss { signature: sig, timestamp });
            }
        }

        events
    }

    /// Generate cost savings report data
    ///
    /// Creates data for validating cost attribution (AC3)
    pub fn generate_cost_report(&mut self) -> CostReport {
        let signatures = self.generate_signature_stats(100);

        let total_cached_requests: u64 = signatures.iter().map(|s| s.reuse_count).sum();
        let total_cost_saved: f64 = signatures.iter().map(|s| s.cost_saved).sum();

        // Calculate per-account attribution (mock 5 accounts)
        let mut per_account_savings = Vec::new();
        let account_ids = vec!["acc_001", "acc_002", "acc_003", "acc_004", "acc_005"];

        for account_id in account_ids {
            let account_requests = self.rng.gen_range(100..500);
            let account_savings = (account_requests as f64) * 0.01;
            per_account_savings.push((account_id.to_string(), account_savings));
        }

        CostReport {
            total_cached_requests,
            total_cost_saved,
            per_account_savings,
            average_cost_per_request: 0.01,
            report_period_days: 30,
        }
    }

    /// Generate performance metrics for benchmarking
    ///
    /// Creates realistic performance data for AC4 validation
    pub fn generate_performance_metrics(&mut self) -> PerformanceMetrics {
        // Generate latency distribution (target <10ms p95)
        let mut lookup_times: Vec<f64> = (0..1000)
            .map(|_| {
                // Normal distribution around 5ms with some outliers
                let base = self.rng.gen_range(2.0..8.0);
                if self.rng.gen_bool(0.05) {
                    // 5% outliers (but still under target)
                    base + self.rng.gen_range(0.0..2.0)
                } else {
                    base
                }
            })
            .collect();

        lookup_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p50 = lookup_times[500];
        let p95 = lookup_times[950];
        let p99 = lookup_times[990];
        let avg = lookup_times.iter().sum::<f64>() / lookup_times.len() as f64;

        PerformanceMetrics {
            average_lookup_time_ms: avg,
            p50_lookup_time_ms: p50,
            p95_lookup_time_ms: p95,
            p99_lookup_time_ms: p99,
            average_write_time_ms: self.rng.gen_range(1.0..4.0),
            memory_usage_mb: self.rng.gen_range(10.0..30.0),
        }
    }
}

impl Default for CacheMockDataGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Cost savings report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostReport {
    pub total_cached_requests: u64,
    pub total_cost_saved: f64,
    pub per_account_savings: Vec<(String, f64)>,
    pub average_cost_per_request: f64,
    pub report_period_days: u32,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub average_lookup_time_ms: f64,
    pub p50_lookup_time_ms: f64,
    pub p95_lookup_time_ms: f64,
    pub p99_lookup_time_ms: f64,
    pub average_write_time_ms: f64,
    pub memory_usage_mb: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_signature_stats() {
        let mut gen = CacheMockDataGenerator::new();
        let stats = gen.generate_signature_stats(100);

        assert_eq!(stats.len(), 100);

        // Top signatures should have higher reuse
        assert!(stats[0].reuse_count >= 10);
        assert!(stats[99].reuse_count <= 10);

        // Verify sorted by reuse count
        for i in 1..stats.len() {
            assert!(stats[i - 1].reuse_count >= stats[i].reuse_count);
        }
    }

    #[test]
    fn test_generate_cache_metrics() {
        let mut gen = CacheMockDataGenerator::new();
        let metrics = gen.generate_cache_metrics();

        // Hit rate should be ≥30%
        assert!(metrics.hit_rate >= 0.30);

        // Should have 10 top signatures
        assert_eq!(metrics.top_signatures.len(), 10);

        // Lookup time should be reasonable
        assert!(metrics.average_lookup_time_ms < 10.0);
    }

    #[test]
    fn test_simulate_request_pattern() {
        let mut gen = CacheMockDataGenerator::new();
        let events = gen.simulate_request_pattern(Duration::from_secs(60));

        // Should generate approximately 600 events (10 RPS * 60 seconds)
        assert!(events.len() > 500 && events.len() < 700);

        // Should have both hits and misses
        let has_hits = events.iter().any(|e| matches!(e, MockCacheEvent::Hit { .. }));
        let has_misses = events.iter().any(|e| matches!(e, MockCacheEvent::Miss { .. }));
        assert!(has_hits && has_misses);
    }

    #[test]
    fn test_generate_high_load_scenario() {
        let mut gen = CacheMockDataGenerator::new();
        let events = gen.generate_high_load_scenario(1);

        // Should generate 1000 events for 1 second at 1000 RPS
        assert_eq!(events.len(), 1000);
    }

    #[test]
    fn test_generate_cost_report() {
        let mut gen = CacheMockDataGenerator::new();
        let report = gen.generate_cost_report();

        assert!(report.total_cached_requests > 0);
        assert!(report.total_cost_saved > 0.0);
        assert_eq!(report.per_account_savings.len(), 5);
        assert_eq!(report.average_cost_per_request, 0.01);
    }

    #[test]
    fn test_generate_performance_metrics() {
        let mut gen = CacheMockDataGenerator::new();
        let metrics = gen.generate_performance_metrics();

        // p95 should be < 10ms (target)
        assert!(metrics.p95_lookup_time_ms < 10.0);

        // p99 should also be < 10ms
        assert!(metrics.p99_lookup_time_ms < 10.0);

        // Average should be lower than p95
        assert!(metrics.average_lookup_time_ms < metrics.p95_lookup_time_ms);
    }
}
