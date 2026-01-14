# Story-012-03: Epic-008 Cache Metrics Restoration

**Epic**: EPIC-012
**Priority**: P0 (Lower than 012-02)
**Effort**: 1 hour
**Owner**: Developer C
**Dependencies**: Story-012-01

---

## 📋 Description

Wire `load_cache_metrics()` to `CacheMonitor::new()` so metrics persist across restarts.

**Current**: Metrics reset to zero on every restart
**Target**: Metrics restored from database on startup

---

## 🎯 Acceptance Criteria

### AC1: Metrics Restored on Startup ✅
```rust
// In CacheMonitor::new()
if let Ok(saved) = proxy_db::load_cache_metrics() {
    *self.metrics.write().unwrap() = saved;
    tracing::info!("Restored cache metrics from database");
}
```

### AC2: Empty Database Handled ✅
First run (no saved metrics) → Use defaults gracefully

### AC3: Test Created ✅
Integration test validates restoration

### AC4: Clippy Error Eliminated ✅
`load_cache_metrics()` no longer unused (125 → 114 errors)

---

## 🛠️ Implementation

**File**: `src/proxy/cache_monitor.rs`

**Change**:
```rust
impl CacheMonitor {
    pub fn new() -> Self {
        let monitor = Self {
            metrics: Arc::new(RwLock::new(CacheMetrics::default())),
        };

        // Restore from database
        match crate::modules::proxy_db::load_cache_metrics() {
            Ok(saved) => {
                *monitor.metrics.write().unwrap() = saved;
                tracing::info!(
                    "Restored cache metrics: hits={}, misses={}",
                    saved.hit_count,
                    saved.miss_count
                );
            }
            Err(e) => {
                tracing::warn!("Failed to load cache metrics: {}. Using defaults.", e);
            }
        }

        monitor
    }
}
```

**Test** (`src/proxy/tests/cache_monitor_integration_tests.rs`):
```rust
#[test]
fn test_cache_metrics_restoration() {
    let monitor1 = CacheMonitor::new();
    monitor1.record_hit();
    monitor1.record_hit();
    monitor1.record_miss();

    // Save to database
    let metrics = monitor1.get_metrics();
    proxy_db::save_cache_metrics(&metrics).unwrap();

    // Create new monitor and verify restoration
    let monitor2 = CacheMonitor::new();
    let restored = monitor2.get_metrics();

    assert_eq!(restored.hit_count, 2);
    assert_eq!(restored.miss_count, 1);
}
```

---

## 📊 Impact

**Clippy**: 1 error eliminated (114 remaining)
**Feature**: Cache metrics now persistent across restarts
**Risk**: LOW (simple integration)

---

**Status**: READY
