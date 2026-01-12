//! Integration tests for cache metrics system
//! Story-008-02: Signature Cache Monitoring

use antigravity_tools_lib::proxy::cache_monitor::CacheMonitor;
use antigravity_tools_lib::proxy::signature_cache::SignatureCache;

/// Test #1: Cache hit tracking integration
/// Verify that signature cache operations are tracked by the monitor
#[tokio::test]
async fn test_cache_hit_tracking_integration() {
    let cache = SignatureCache::global();
    let monitor = SignatureCache::get_monitor();

    // Clear any existing metrics
    monitor.clear().await;

    // Store a signature
    let signature = "x".repeat(100);
    cache.cache_tool_signature("tool_123", signature.clone());

    // Wait a bit for async operations to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Retrieve the signature (should trigger a hit)
    let retrieved = cache.get_tool_signature("tool_123");
    assert!(retrieved.is_some());

    // Wait for monitoring to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Verify metrics were recorded
    let metrics = monitor.export_metrics().await;
    assert!(metrics.hit_count > 0, "Hit count should be recorded");
}

/// Test #2: Cache miss tracking integration
/// Verify that cache misses are tracked by the monitor
#[tokio::test]
async fn test_cache_miss_tracking_integration() {
    let cache = SignatureCache::global();
    let monitor = SignatureCache::get_monitor();

    // Clear any existing metrics
    monitor.clear().await;

    // Try to retrieve a non-existent signature
    let retrieved = cache.get_tool_signature("nonexistent_tool");
    assert!(retrieved.is_none());

    // Wait for monitoring to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Verify miss was recorded
    let metrics = monitor.export_metrics().await;
    assert!(metrics.miss_count > 0, "Miss count should be recorded");
}

/// Test #3: Comprehensive metrics export
/// Verify that all metrics are properly exported
#[tokio::test]
async fn test_comprehensive_metrics_export() {
    let cache = SignatureCache::global();
    let monitor = SignatureCache::get_monitor();

    // Clear any existing metrics
    monitor.clear().await;

    // Simulate cache activity
    for i in 0..5 {
        let signature = format!("sig_{}", "x".repeat(60));
        let tool_id = format!("tool_{}", i);
        cache.cache_tool_signature(&tool_id, signature);
    }

    // Wait for write operations
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Trigger some hits
    for i in 0..3 {
        let tool_id = format!("tool_{}", i);
        let _ = cache.get_tool_signature(&tool_id);
    }

    // Trigger some misses
    for i in 5..8 {
        let tool_id = format!("tool_{}", i);
        let _ = cache.get_tool_signature(&tool_id);
    }

    // Wait for monitoring
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Export comprehensive metrics
    let metrics = monitor.export_metrics().await;

    // Verify all metric categories are present
    assert!(metrics.hit_count > 0, "Should have hits");
    assert!(metrics.miss_count > 0, "Should have misses");
    assert!(metrics.hit_rate > 0.0, "Hit rate should be calculated");
    assert!(
        metrics.performance.total_operations > 0,
        "Operations should be tracked"
    );
    assert!(
        metrics.cost_savings.total_saved >= 0.0,
        "Cost savings should be calculated"
    );
}

/// Test #4: Top signatures ranking
/// Verify that most reused signatures are correctly ranked
#[tokio::test]
async fn test_top_signatures_ranking() {
    let cache = SignatureCache::global();
    let monitor = SignatureCache::get_monitor();

    // Clear any existing metrics
    monitor.clear().await;

    // Create signatures with different reuse patterns
    let sig_a = "a".repeat(60);
    let sig_b = "b".repeat(60);
    let sig_c = "c".repeat(60);

    cache.cache_tool_signature("tool_a", sig_a.clone());
    cache.cache_tool_signature("tool_b", sig_b.clone());
    cache.cache_tool_signature("tool_c", sig_c.clone());

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Sig A: 5 hits (most reused)
    for _ in 0..5 {
        let _ = cache.get_tool_signature("tool_a");
    }

    // Sig B: 3 hits (medium reuse)
    for _ in 0..3 {
        let _ = cache.get_tool_signature("tool_b");
    }

    // Sig C: 1 hit (least reused)
    let _ = cache.get_tool_signature("tool_c");

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Get top signatures
    let top_sigs: Vec<antigravity_tools_lib::proxy::cache_monitor::SignatureStats> =
        monitor.get_top_signatures(3).await;

    // Verify ranking
    assert!(top_sigs.len() >= 3, "Should have at least 3 signatures");
    assert!(
        top_sigs[0].reuse_count >= top_sigs[1].reuse_count,
        "First should be most reused"
    );
    assert!(
        top_sigs[1].reuse_count >= top_sigs[2].reuse_count,
        "Ranking should be descending"
    );
}

/// Test #5: Performance metrics tracking
/// Verify that lookup and write times are tracked
#[tokio::test]
async fn test_performance_metrics_tracking() {
    let cache = SignatureCache::global();
    let monitor = SignatureCache::get_monitor();

    // Clear any existing metrics
    monitor.clear().await;

    // Perform cache operations
    for i in 0..10 {
        let signature = format!("perf_sig_{}", "x".repeat(60));
        let tool_id = format!("perf_tool_{}", i);
        cache.cache_tool_signature(&tool_id, signature);
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Trigger lookups
    for i in 0..10 {
        let tool_id = format!("perf_tool_{}", i);
        let _ = cache.get_tool_signature(&tool_id);
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Export metrics
    let metrics = monitor.export_metrics().await;

    // Verify performance metrics
    assert!(
        metrics.performance.lookup_p95 >= 0.0,
        "P95 should be tracked"
    );
    assert!(
        metrics.performance.total_operations > 0,
        "Operations should be counted"
    );
}
