//! # Signature Cache Tauri Commands
//!
//! This module provides Tauri command interfaces for the SignatureCacheLRU.
//! It exposes cache metrics and management operations to the frontend UI.
//!
//! ## Commands
//!
//! - `get_signature_cache_metrics`: Get current cache performance metrics
//! - `clear_signature_cache`: Clear all cached signatures
//! - `get_signature_cache_stats`: Get detailed cache statistics

use crate::commands::proxy::ProxyServiceState;
use crate::modules::signature_cache::CacheMetrics;

/// Get current signature cache metrics
///
/// Returns performance metrics including:
/// - Cache hits and misses
/// - Hit rate percentage
/// - Eviction count
/// - Corruption count
/// - Current cache size and capacity
#[tauri::command]
pub async fn get_signature_cache_metrics(
    proxy_state: tauri::State<'_, ProxyServiceState>,
) -> Result<SignatureCacheMetrics, String> {
    let instance_lock = proxy_state.instance.read().await;

    if let Some(instance) = instance_lock.as_ref() {
        let cache = &instance.signature_cache;
        let metrics = cache.get_metrics();

        Ok(SignatureCacheMetrics {
            hits: metrics.hits,
            misses: metrics.misses,
            hit_rate: metrics.hit_rate(),
            evictions: metrics.evictions,
            corruptions: metrics.corruptions,
            corruption_rate: metrics.corruption_rate(),
            size: cache.len(),
            capacity: cache.capacity(),
            ttl_days: cache.ttl_days(),
        })
    } else {
        Err("Proxy service not running".to_string())
    }
}

/// Get signature cache hit rate
///
/// Returns just the hit rate percentage as a convenience method.
/// Returns 0.0 if proxy is not running.
#[tauri::command]
pub async fn get_signature_cache_hit_rate(
    proxy_state: tauri::State<'_, ProxyServiceState>,
) -> Result<f64, String> {
    let instance_lock = proxy_state.instance.read().await;

    if let Some(instance) = instance_lock.as_ref() {
        let metrics = instance.signature_cache.get_metrics();
        Ok(metrics.hit_rate())
    } else {
        Ok(0.0)
    }
}

/// Clear all cached signatures
///
/// This operation:
/// - Removes all cached signature entries
/// - Resets all metrics to zero
/// - Logs the operation
///
/// Only works when proxy service is running.
#[tauri::command]
pub async fn clear_signature_cache(
    proxy_state: tauri::State<'_, ProxyServiceState>,
) -> Result<(), String> {
    let instance_lock = proxy_state.instance.read().await;

    if let Some(instance) = instance_lock.as_ref() {
        instance.signature_cache.clear();
        crate::modules::logger::log_info("Signature cache cleared by user command");
        Ok(())
    } else {
        Err("Proxy service not running".to_string())
    }
}

/// Get detailed signature cache statistics
///
/// Returns comprehensive statistics including:
/// - Current cache state (size, capacity, hit rate)
/// - Performance metrics (hits, misses, evictions, corruptions)
/// - Configuration settings (TTL, capacity)
#[tauri::command]
pub async fn get_signature_cache_stats(
    proxy_state: tauri::State<'_, ProxyServiceState>,
) -> Result<SignatureCacheStats, String> {
    let instance_lock = proxy_state.instance.read().await;

    if let Some(instance) = instance_lock.as_ref() {
        let cache = &instance.signature_cache;
        let metrics = cache.get_metrics();

        Ok(SignatureCacheStats {
            enabled: true,
            metrics: SignatureCacheMetrics {
                hits: metrics.hits,
                misses: metrics.misses,
                hit_rate: metrics.hit_rate(),
                evictions: metrics.evictions,
                corruptions: metrics.corruptions,
                corruption_rate: metrics.corruption_rate(),
                size: cache.len(),
                capacity: cache.capacity(),
                ttl_days: cache.ttl_days(),
            },
            health_status: calculate_health_status(&metrics, cache.len(), cache.capacity()),
        })
    } else {
        Ok(SignatureCacheStats {
            enabled: false,
            metrics: SignatureCacheMetrics {
                hits: 0,
                misses: 0,
                hit_rate: 0.0,
                evictions: 0,
                corruptions: 0,
                corruption_rate: 0.0,
                size: 0,
                capacity: 0,
                ttl_days: 0,
            },
            health_status: "offline".to_string(),
        })
    }
}

// Helper Structures

/// Signature cache metrics for frontend consumption
#[derive(Debug, serde::Serialize)]
pub struct SignatureCacheMetrics {
    /// Number of successful cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Cache hit rate percentage (0-100)
    pub hit_rate: f64,
    /// Number of entries evicted (LRU or TTL)
    pub evictions: u64,
    /// Number of corrupted signatures detected
    pub corruptions: u64,
    /// Corruption rate percentage (0-100)
    pub corruption_rate: f64,
    /// Current number of cached entries
    pub size: usize,
    /// Maximum cache capacity
    pub capacity: usize,
    /// Time-to-live in days
    pub ttl_days: i64,
}

/// Comprehensive cache statistics
#[derive(Debug, serde::Serialize)]
pub struct SignatureCacheStats {
    /// Whether cache is currently enabled/running
    pub enabled: bool,
    /// Current metrics
    pub metrics: SignatureCacheMetrics,
    /// Health status: "healthy" | "degraded" | "critical" | "offline"
    pub health_status: String,
}

/// Calculate cache health status based on metrics
///
/// Health indicators:
/// - "healthy": hit_rate > 80%, corruption_rate < 1%
/// - "degraded": hit_rate 60-80% OR corruption_rate 1-5%
/// - "critical": hit_rate < 60% OR corruption_rate > 5%
/// - "offline": proxy not running
fn calculate_health_status(metrics: &CacheMetrics, size: usize, capacity: usize) -> String {
    let hit_rate = metrics.hit_rate();
    let corruption_rate = metrics.corruption_rate();
    let usage_ratio = if capacity > 0 {
        (size as f64 / capacity as f64) * 100.0
    } else {
        0.0
    };

    // Critical conditions
    if hit_rate < 60.0 {
        return "critical".to_string();
    }

    if corruption_rate > 5.0 {
        return "critical".to_string();
    }

    // Degraded conditions
    if hit_rate < 80.0 {
        return "degraded".to_string();
    }

    if corruption_rate > 1.0 {
        return "degraded".to_string();
    }

    // Warning if cache is nearly full (>90%)
    if usage_ratio > 90.0 {
        return "degraded".to_string();
    }

    "healthy".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::signature_cache::CacheMetrics;

    #[test]
    fn test_health_status_healthy() {
        let mut metrics = CacheMetrics::new();
        metrics.hits = 90;
        metrics.misses = 10;
        metrics.corruptions = 0;

        let status = calculate_health_status(&metrics, 500, 1000);
        assert_eq!(status, "healthy");
    }

    #[test]
    fn test_health_status_degraded_low_hit_rate() {
        let mut metrics = CacheMetrics::new();
        metrics.hits = 70;
        metrics.misses = 30;
        metrics.corruptions = 0;

        let status = calculate_health_status(&metrics, 500, 1000);
        assert_eq!(status, "degraded");
    }

    #[test]
    fn test_health_status_critical_low_hit_rate() {
        let mut metrics = CacheMetrics::new();
        metrics.hits = 50;
        metrics.misses = 50;
        metrics.corruptions = 0;

        let status = calculate_health_status(&metrics, 500, 1000);
        assert_eq!(status, "critical");
    }

    #[test]
    fn test_health_status_critical_high_corruption() {
        let mut metrics = CacheMetrics::new();
        metrics.hits = 90;
        metrics.misses = 4;
        metrics.corruptions = 6; // 6/94 = 6.38%

        let status = calculate_health_status(&metrics, 500, 1000);
        assert_eq!(status, "critical");
    }

    #[test]
    fn test_health_status_degraded_nearly_full() {
        let mut metrics = CacheMetrics::new();
        metrics.hits = 90;
        metrics.misses = 10;
        metrics.corruptions = 0;

        let status = calculate_health_status(&metrics, 950, 1000); // 95% full
        assert_eq!(status, "degraded");
    }
}
