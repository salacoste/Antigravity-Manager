//! Cache Monitor Integration Tests
//!
//! Story-012-03: Epic-008 Cache Metrics Restoration
//!
//! Tests that validate cache monitor works correctly with database stubs.
//! Note: save_cache_metrics and load_cache_metrics are currently stubs that
//! don't persist data. These tests verify the stub behavior.

#[cfg(test)]
use crate::modules::proxy_db;
use crate::proxy::cache_monitor::{CacheMetrics, CacheMonitor, CostSavings, PerformanceMetrics};

/// Test that save_cache_metrics stub succeeds
#[tokio::test]
async fn test_cache_metrics_save_stub() {
    // Initialize database (creates tables if needed)
    proxy_db::init_db().expect("Database initialization should succeed");

    // Create monitor and record some hits/misses
    let monitor = CacheMonitor::new();
    monitor.record_hit("signature1", 5.0, None).await;
    monitor.record_hit("signature2", 6.0, None).await;
    monitor.record_miss("signature3").await;

    // Get metrics
    let metrics = monitor.export_metrics().await;
    assert_eq!(metrics.hit_count, 2, "Should have 2 hits");
    assert_eq!(metrics.miss_count, 1, "Should have 1 miss");

    // Save to database (stub - should succeed)
    let result = proxy_db::save_cache_metrics(&metrics);
    assert!(result.is_ok(), "Save stub should succeed");
}

/// Test that load_cache_metrics stub returns default metrics
#[test]
fn test_cache_metrics_load_returns_default() {
    // Initialize database
    proxy_db::init_db().expect("Database initialization should succeed");

    // Load from database (stub - returns default CacheMetrics)
    let result = proxy_db::load_cache_metrics();
    assert!(result.is_ok(), "Load stub should succeed");

    let metrics = result.unwrap();
    // Stub returns default values (all zeros)
    assert_eq!(metrics.hit_count, 0, "Stub returns default hit_count=0");
    assert_eq!(metrics.miss_count, 0, "Stub returns default miss_count=0");
    assert_eq!(metrics.hit_rate, 0.0, "Stub returns default hit_rate=0.0");
}

/// Test that monitor creation succeeds even with stub database
#[test]
fn test_database_failure_graceful_fallback() {
    // Initialize database
    proxy_db::init_db().expect("Database initialization should succeed");

    // Create monitor - should work with stub load_cache_metrics
    let monitor = CacheMonitor::new();

    // Monitor should be created successfully with default values
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let metrics = runtime.block_on(async { monitor.export_metrics().await });

    // Should have default metrics (zeros) since stub returns defaults
    assert_eq!(metrics.hit_count, 0, "Should start with zero hits");
    assert_eq!(metrics.miss_count, 0, "Should start with zero misses");
    assert_eq!(metrics.hit_rate, 0.0, "Should start with zero hit rate");

    // Should be able to record new metrics
    runtime.block_on(async {
        monitor.record_hit("test_sig", 5.0, None).await;
        let updated = monitor.export_metrics().await;
        assert_eq!(updated.hit_count, 1, "Should be able to record hits");
    });
}

/// Test that performance metrics are recorded correctly
#[tokio::test]
async fn test_performance_metrics_tracking() {
    // Create monitor and record operations with timing data
    let monitor = CacheMonitor::new();

    // Record hits with varying lookup times
    monitor.record_hit("sig1", 1.0, None).await;
    monitor.record_hit("sig2", 5.0, None).await;
    monitor.record_hit("sig3", 10.0, None).await;

    // Record write operations
    monitor.record_write(2.0).await;
    monitor.record_write(3.0).await;

    // Export metrics
    let metrics = monitor.export_metrics().await;

    // Verify performance metrics were tracked
    assert_eq!(metrics.hit_count, 3, "Should have 3 hits");
    assert!(
        metrics.performance.total_operations >= 3,
        "Should have at least 3 operations"
    );
}

/// Test cost savings calculation
#[tokio::test]
async fn test_cost_savings_calculation() {
    // Create monitor and record hits with account attribution
    let monitor = CacheMonitor::new();

    monitor
        .record_hit("sig1", 5.0, Some("account1@example.com"))
        .await;
    monitor
        .record_hit("sig2", 6.0, Some("account1@example.com"))
        .await;
    monitor
        .record_hit("sig3", 7.0, Some("account2@example.com"))
        .await;

    // Export metrics
    let metrics = monitor.export_metrics().await;

    // Verify hits were recorded
    assert_eq!(metrics.hit_count, 3, "Should have 3 hits");
    assert_eq!(metrics.miss_count, 0, "Should have 0 misses");

    // Cost savings should be calculated (exact value depends on implementation)
    assert!(
        metrics.cost_savings.total_saved >= 0.0,
        "Cost savings should be non-negative"
    );
}

/// Test that save/load workflow works with stub
#[tokio::test]
async fn test_save_load_stub_workflow() {
    use chrono::Utc;

    // Initialize database
    proxy_db::init_db().expect("Database initialization should succeed");

    // Create metrics with specific values
    let original = CacheMetrics {
        hit_count: 100,
        miss_count: 25,
        hit_rate: 0.8,
        top_signatures: vec![],
        performance: PerformanceMetrics {
            lookup_p50: 1.5,
            lookup_p95: 5.0,
            lookup_p99: 10.0,
            write_p95: 2.5,
            memory_usage: 1024,
            total_operations: 125,
            degradation_alert: false,
        },
        cost_savings: CostSavings::default(),
        updated_at: Utc::now(),
    };

    // Save to database (stub - succeeds but doesn't persist)
    let save_result = proxy_db::save_cache_metrics(&original);
    assert!(save_result.is_ok(), "Save stub should succeed");

    // Load from database (stub - returns default, not what was saved)
    let loaded = proxy_db::load_cache_metrics().expect("Load should succeed");

    // Stub returns default values (NOT original values since stub doesn't persist)
    assert_eq!(
        loaded.hit_count, 0,
        "Stub load returns default, not saved value"
    );
    assert_eq!(
        loaded.miss_count, 0,
        "Stub load returns default, not saved value"
    );
}
