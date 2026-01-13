# Story-014-04: Audio Usage Analytics

**Epic**: Epic-014 - Gemini 2.0 Flash Experimental Audio Specialist (Team 1, Gemini Specialists)
**Priority**: P2 (MEDIUM - operational insight)
**Effort**: 3 days
**Assignee**: Dev 1C (Junior QA/Developer)
**Status**: ✅ READY FOR EXECUTION (Start: 2026-01-22)
**Created**: 2026-01-12

---

## Objective

Implement audio-specific metrics tracking (duration, format distribution, file size patterns, quality trends) to enable data-driven cache optimization, quota planning, and operational excellence for gemini-2.0-flash-exp audio transcription.

---

## Business Context

### Problem Statement

Current monitoring is generic (no audio-specific insights):

```yaml
current_metrics: "Request count, success rate, response times"
  - "No audio duration tracking"
  - "No format distribution analysis"
  - "No file size patterns"
  - "No quality trend monitoring"

missed_opportunities:
  cache_optimization: "Don't know which formats to cache aggressively"
  quota_planning: "Can't predict quota needs based on duration"
  quality_monitoring: "Can't detect transcription quality degradation"
  feature_prioritization: "Don't know which formats users prefer"
```

### Success Metrics

**Primary KPI**: 15-20% cost savings from data-driven optimization
**Operational Visibility**: 100% audio usage patterns tracked
**Cache Optimization**: 25% hit rate improvement (vs blind caching)
**Feature ROI**: Data-driven format support prioritization

### Business Value

- **Cost optimization**: $50-100/month savings from cache optimization
- **Operational excellence**: Data-driven decisions vs guesswork
- **Product intelligence**: Know which features users actually use
- **Quality assurance**: Detect issues before users report them

---

## Acceptance Criteria

### AC1: Audio Duration Tracking

**GIVEN** audio transcription requests
**WHEN** processing audio files
**THEN** duration metrics MUST be tracked (min, max, avg, P50, P95, P99)

**Duration Metrics**:
```yaml
metrics_tracked:
  min_duration_seconds: "Shortest audio file in period"
  max_duration_seconds: "Longest audio file in period"
  avg_duration_seconds: "Average audio duration"
  p50_duration_seconds: "Median audio duration"
  p95_duration_seconds: "95th percentile (outliers)"
  p99_duration_seconds: "99th percentile (extreme outliers)"
  total_audio_hours: "Sum of all audio processed"

aggregation_periods:
  - "Last hour"
  - "Last 24 hours"
  - "Last 7 days"
  - "Last 30 days"

use_cases:
  quota_planning: "Predict quota needs based on avg duration"
  cost_estimation: "Estimate costs for new users"
  outlier_detection: "Identify unusually long files (potential issues)"
```

**Implementation**:
```rust
// src-tauri/src/proxy/monitor.rs

#[derive(Debug, Clone, Serialize)]
pub struct AudioDurationMetrics {
    pub period: String,                    // "1h", "24h", "7d", "30d"
    pub min_duration_seconds: f64,
    pub max_duration_seconds: f64,
    pub avg_duration_seconds: f64,
    pub p50_duration_seconds: f64,
    pub p95_duration_seconds: f64,
    pub p99_duration_seconds: f64,
    pub total_audio_hours: f64,
    pub sample_count: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl RequestMonitor {
    pub fn record_audio_duration(&self, duration_seconds: f64) {
        let mut durations = self.audio_durations.lock().unwrap();
        durations.push(duration_seconds);

        // Calculate percentiles (if enough samples)
        if durations.len() >= 10 {
            let metrics = Self::calculate_duration_metrics(&durations);
            self.audio_duration_metrics.lock().unwrap().replace(metrics);
        }
    }

    fn calculate_duration_metrics(durations: &[f64]) -> AudioDurationMetrics {
        let mut sorted = durations.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        AudioDurationMetrics {
            period: "24h".to_string(),
            min_duration_seconds: *sorted.first().unwrap_or(&0.0),
            max_duration_seconds: *sorted.last().unwrap_or(&0.0),
            avg_duration_seconds: sorted.iter().sum::<f64>() / sorted.len() as f64,
            p50_duration_seconds: Self::percentile(&sorted, 0.50),
            p95_duration_seconds: Self::percentile(&sorted, 0.95),
            p99_duration_seconds: Self::percentile(&sorted, 0.99),
            total_audio_hours: sorted.iter().sum::<f64>() / 3600.0,
            sample_count: sorted.len() as u64,
            last_updated: chrono::Utc::now(),
        }
    }

    fn percentile(sorted: &[f64], p: f64) -> f64 {
        let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
        sorted[idx]
    }
}
```

---

### AC2: Format Distribution Tracking

**GIVEN** audio transcription requests with different formats
**WHEN** analyzing usage patterns
**THEN** format distribution MUST be tracked (count + percentage)

**Format Metrics**:
```yaml
metrics_tracked:
  mp3_count: "Number of MP3 files"
  mp3_percentage: "% of total requests"
  wav_count: "Number of WAV files"
  wav_percentage: "% of total requests"
  m4a_count: "Number of M4A files"
  m4a_percentage: "% of total requests"
  ogg_count: "Number of OGG files"
  ogg_percentage: "% of total requests"
  flac_count: "Number of FLAC files"
  flac_percentage: "% of total requests"
  aiff_count: "Number of AIFF files"
  aiff_percentage: "% of total requests"

insights:
  cache_strategy: "Cache popular formats aggressively (e.g., MP3 if 80%)"
  support_prioritization: "Focus optimization on top 2 formats"
  feature_decisions: "Deprioritize rare formats (<1% usage)"
```

**Implementation**:
```rust
// src-tauri/src/proxy/monitor.rs

#[derive(Debug, Clone, Serialize)]
pub struct AudioFormatMetrics {
    pub period: String,
    pub mp3_count: u64,
    pub mp3_percentage: f64,
    pub wav_count: u64,
    pub wav_percentage: f64,
    pub m4a_count: u64,
    pub m4a_percentage: f64,
    pub ogg_count: u64,
    pub ogg_percentage: f64,
    pub flac_count: u64,
    pub flac_percentage: f64,
    pub aiff_count: u64,
    pub aiff_percentage: f64,
    pub total_count: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl RequestMonitor {
    pub fn record_audio_format(&self, mime_type: &str) {
        let format = Self::normalize_format(mime_type);
        let mut formats = self.audio_formats.lock().unwrap();
        *formats.entry(format).or_insert(0) += 1;

        // Calculate percentages
        let total: u64 = formats.values().sum();
        let metrics = AudioFormatMetrics {
            period: "24h".to_string(),
            mp3_count: *formats.get("mp3").unwrap_or(&0),
            mp3_percentage: (*formats.get("mp3").unwrap_or(&0) as f64 / total as f64) * 100.0,
            wav_count: *formats.get("wav").unwrap_or(&0),
            wav_percentage: (*formats.get("wav").unwrap_or(&0) as f64 / total as f64) * 100.0,
            m4a_count: *formats.get("m4a").unwrap_or(&0),
            m4a_percentage: (*formats.get("m4a").unwrap_or(&0) as f64 / total as f64) * 100.0,
            ogg_count: *formats.get("ogg").unwrap_or(&0),
            ogg_percentage: (*formats.get("ogg").unwrap_or(&0) as f64 / total as f64) * 100.0,
            flac_count: *formats.get("flac").unwrap_or(&0),
            flac_percentage: (*formats.get("flac").unwrap_or(&0) as f64 / total as f64) * 100.0,
            aiff_count: *formats.get("aiff").unwrap_or(&0),
            aiff_percentage: (*formats.get("aiff").unwrap_or(&0) as f64 / total as f64) * 100.0,
            total_count: total,
            last_updated: chrono::Utc::now(),
        };
        self.audio_format_metrics.lock().unwrap().replace(metrics);
    }

    fn normalize_format(mime_type: &str) -> String {
        match mime_type {
            "audio/mpeg" | "audio/mp3" => "mp3",
            "audio/wav" | "audio/x-wav" => "wav",
            "audio/mp4" | "audio/m4a" => "m4a",
            "audio/ogg" | "audio/vorbis" => "ogg",
            "audio/flac" => "flac",
            "audio/aiff" | "audio/x-aiff" => "aiff",
            _ => "unknown",
        }.to_string()
    }
}
```

---

### AC3: File Size Distribution Tracking

**GIVEN** audio file uploads
**WHEN** analyzing file size patterns
**THEN** file size distribution MUST be tracked

**File Size Metrics**:
```yaml
metrics_tracked:
  size_under_1mb_count: "Files <1MB"
  size_1mb_to_5mb_count: "Files 1-5MB"
  size_5mb_to_10mb_count: "Files 5-10MB"
  size_10mb_to_15mb_count: "Files 10-15MB (near limit)"
  size_over_15mb_count: "Files >15MB (rejected)"
  avg_file_size_mb: "Average file size"
  p95_file_size_mb: "95th percentile"

insights:
  bandwidth_optimization: "Most files <5MB → optimize for small files"
  limit_awareness: "X% of files near 15MB limit → consider increase"
  compression_opportunity: "Large files → recommend compression"
```

**Implementation**:
```rust
// src-tauri/src/proxy/monitor.rs

#[derive(Debug, Clone, Serialize)]
pub struct AudioFileSizeMetrics {
    pub period: String,
    pub size_under_1mb_count: u64,
    pub size_1mb_to_5mb_count: u64,
    pub size_5mb_to_10mb_count: u64,
    pub size_10mb_to_15mb_count: u64,
    pub size_over_15mb_count: u64,
    pub avg_file_size_mb: f64,
    pub p95_file_size_mb: f64,
    pub total_count: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl RequestMonitor {
    pub fn record_audio_file_size(&self, size_bytes: usize) {
        let size_mb = size_bytes as f64 / 1_048_576.0;
        let mut sizes = self.audio_file_sizes.lock().unwrap();
        sizes.push(size_mb);

        // Calculate distribution
        let metrics = Self::calculate_file_size_metrics(&sizes);
        self.audio_file_size_metrics.lock().unwrap().replace(metrics);
    }

    fn calculate_file_size_metrics(sizes: &[f64]) -> AudioFileSizeMetrics {
        let under_1mb = sizes.iter().filter(|&&s| s < 1.0).count() as u64;
        let mb_1_to_5 = sizes.iter().filter(|&&s| s >= 1.0 && s < 5.0).count() as u64;
        let mb_5_to_10 = sizes.iter().filter(|&&s| s >= 5.0 && s < 10.0).count() as u64;
        let mb_10_to_15 = sizes.iter().filter(|&&s| s >= 10.0 && s < 15.0).count() as u64;
        let over_15mb = sizes.iter().filter(|&&s| s >= 15.0).count() as u64;

        let avg = sizes.iter().sum::<f64>() / sizes.len() as f64;
        let mut sorted = sizes.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p95 = Self::percentile(&sorted, 0.95);

        AudioFileSizeMetrics {
            period: "24h".to_string(),
            size_under_1mb_count: under_1mb,
            size_1mb_to_5mb_count: mb_1_to_5,
            size_5mb_to_10mb_count: mb_5_to_10,
            size_10mb_to_15mb_count: mb_10_to_15,
            size_over_15mb_count: over_15mb,
            avg_file_size_mb: avg,
            p95_file_size_mb: p95,
            total_count: sizes.len() as u64,
            last_updated: chrono::Utc::now(),
        }
    }
}
```

---

### AC4: Dashboard Visualization

**GIVEN** audio analytics data
**WHEN** viewing Monitor dashboard
**THEN** audio-specific charts MUST be displayed

**Dashboard Components**:
```tsx
// src/pages/Monitor.tsx

export function AudioAnalyticsDashboard() {
  const [durationMetrics, setDurationMetrics] = useState<AudioDurationMetrics | null>(null);
  const [formatMetrics, setFormatMetrics] = useState<AudioFormatMetrics | null>(null);
  const [fileSizeMetrics, setFileSizeMetrics] = useState<AudioFileSizeMetrics | null>(null);

  useEffect(() => {
    // Fetch metrics from Tauri backend
    invoke<AudioDurationMetrics>('get_audio_duration_metrics').then(setDurationMetrics);
    invoke<AudioFormatMetrics>('get_audio_format_metrics').then(setFormatMetrics);
    invoke<AudioFileSizeMetrics>('get_audio_file_size_metrics').then(setFileSizeMetrics);
  }, []);

  return (
    <div className="space-y-6">
      {/* Duration Metrics Card */}
      <div className="card bg-base-100 shadow-xl">
        <div className="card-body">
          <h2 className="card-title">Audio Duration Distribution</h2>
          <div className="stats stats-vertical lg:stats-horizontal shadow">
            <div className="stat">
              <div className="stat-title">Avg Duration</div>
              <div className="stat-value text-primary">
                {durationMetrics?.avg_duration_seconds.toFixed(0)}s
              </div>
            </div>
            <div className="stat">
              <div className="stat-title">P95 Duration</div>
              <div className="stat-value text-secondary">
                {durationMetrics?.p95_duration_seconds.toFixed(0)}s
              </div>
            </div>
            <div className="stat">
              <div className="stat-title">Total Audio Hours</div>
              <div className="stat-value text-accent">
                {durationMetrics?.total_audio_hours.toFixed(1)}h
              </div>
            </div>
          </div>
          {/* Duration histogram chart */}
          <AudioDurationChart data={durationMetrics} />
        </div>
      </div>

      {/* Format Distribution Card */}
      <div className="card bg-base-100 shadow-xl">
        <div className="card-body">
          <h2 className="card-title">Format Distribution</h2>
          {/* Pie chart showing format percentages */}
          <AudioFormatPieChart data={formatMetrics} />
          <div className="grid grid-cols-2 md:grid-cols-3 gap-4 mt-4">
            <FormatBadge format="MP3" count={formatMetrics?.mp3_count} percentage={formatMetrics?.mp3_percentage} />
            <FormatBadge format="WAV" count={formatMetrics?.wav_count} percentage={formatMetrics?.wav_percentage} />
            <FormatBadge format="M4A" count={formatMetrics?.m4a_count} percentage={formatMetrics?.m4a_percentage} />
            <FormatBadge format="OGG" count={formatMetrics?.ogg_count} percentage={formatMetrics?.ogg_percentage} />
            <FormatBadge format="FLAC" count={formatMetrics?.flac_count} percentage={formatMetrics?.flac_percentage} />
            <FormatBadge format="AIFF" count={formatMetrics?.aiff_count} percentage={formatMetrics?.aiff_percentage} />
          </div>
        </div>
      </div>

      {/* File Size Distribution Card */}
      <div className="card bg-base-100 shadow-xl">
        <div className="card-body">
          <h2 className="card-title">File Size Distribution</h2>
          {/* Bar chart showing size buckets */}
          <AudioFileSizeChart data={fileSizeMetrics} />
          <div className="stats shadow mt-4">
            <div className="stat">
              <div className="stat-title">Avg File Size</div>
              <div className="stat-value">{fileSizeMetrics?.avg_file_size_mb.toFixed(1)}MB</div>
            </div>
            <div className="stat">
              <div className="stat-title">P95 File Size</div>
              <div className="stat-value">{fileSizeMetrics?.p95_file_size_mb.toFixed(1)}MB</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
```

---

### AC5: Historical Data Storage (30-Day Retention)

**GIVEN** audio analytics data
**WHEN** storing metrics over time
**THEN** 30-day historical data MUST be retained in SQLite

**Database Schema**:
```sql
-- src-tauri/src/db/audio_metrics.rs

CREATE TABLE audio_duration_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,          -- Unix timestamp
    period TEXT NOT NULL,                -- "1h", "24h", "7d", "30d"
    min_duration_seconds REAL NOT NULL,
    max_duration_seconds REAL NOT NULL,
    avg_duration_seconds REAL NOT NULL,
    p50_duration_seconds REAL NOT NULL,
    p95_duration_seconds REAL NOT NULL,
    p99_duration_seconds REAL NOT NULL,
    total_audio_hours REAL NOT NULL,
    sample_count INTEGER NOT NULL,
    UNIQUE(timestamp, period)
);

CREATE TABLE audio_format_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    period TEXT NOT NULL,
    mp3_count INTEGER NOT NULL,
    mp3_percentage REAL NOT NULL,
    wav_count INTEGER NOT NULL,
    wav_percentage REAL NOT NULL,
    m4a_count INTEGER NOT NULL,
    m4a_percentage REAL NOT NULL,
    ogg_count INTEGER NOT NULL,
    ogg_percentage REAL NOT NULL,
    flac_count INTEGER NOT NULL,
    flac_percentage REAL NOT NULL,
    aiff_count INTEGER NOT NULL,
    aiff_percentage REAL NOT NULL,
    total_count INTEGER NOT NULL,
    UNIQUE(timestamp, period)
);

CREATE TABLE audio_filesize_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    period TEXT NOT NULL,
    size_under_1mb_count INTEGER NOT NULL,
    size_1mb_to_5mb_count INTEGER NOT NULL,
    size_5mb_to_10mb_count INTEGER NOT NULL,
    size_10mb_to_15mb_count INTEGER NOT NULL,
    size_over_15mb_count INTEGER NOT NULL,
    avg_file_size_mb REAL NOT NULL,
    p95_file_size_mb REAL NOT NULL,
    total_count INTEGER NOT NULL,
    UNIQUE(timestamp, period)
);

-- Automatic cleanup: Delete records older than 30 days
CREATE TRIGGER cleanup_audio_duration_history
AFTER INSERT ON audio_duration_history
BEGIN
    DELETE FROM audio_duration_history
    WHERE timestamp < (strftime('%s', 'now') - 30 * 24 * 3600);
END;

CREATE TRIGGER cleanup_audio_format_history
AFTER INSERT ON audio_format_history
BEGIN
    DELETE FROM audio_format_history
    WHERE timestamp < (strftime('%s', 'now') - 30 * 24 * 3600);
END;

CREATE TRIGGER cleanup_audio_filesize_history
AFTER INSERT ON audio_filesize_history
BEGIN
    DELETE FROM audio_filesize_history
    WHERE timestamp < (strftime('%s', 'now') - 30 * 24 * 3600);
END;
```

---

### AC6: Comprehensive Test Coverage

**Unit Tests** (8+ tests minimum):
```yaml
Duration Tracking Tests:
  - test_record_audio_duration_calculates_avg
  - test_percentile_calculation_p50_p95_p99
  - test_total_audio_hours_calculation

Format Tracking Tests:
  - test_record_audio_format_mp3_count_increment
  - test_format_percentage_calculation
  - test_format_normalization (mime_type → format)

File Size Tracking Tests:
  - test_record_file_size_bucket_distribution
  - test_avg_file_size_calculation
```

**Integration Tests** (4+ tests minimum):
```yaml
End-to-End Analytics:
  - test_e2e_audio_request_records_all_metrics
  - test_e2e_dashboard_displays_metrics
  - test_e2e_historical_data_stored_sqlite
  - test_e2e_30day_cleanup_trigger
```

---

## Implementation Details

### Module Structure

```
src-tauri/src/
├── proxy/
│   ├── monitor.rs                   (EXTEND - add audio analytics)
│   └── handlers/
│       └── audio.rs                 (MODIFY - collect audio metadata)
└── db/
    └── audio_metrics.rs             (NEW - SQLite schema + queries)

src/
└── pages/
    └── Monitor.tsx                  (MODIFY - add audio analytics dashboard)

tests/
└── audio/
    └── analytics_tests.rs           (NEW - 12 tests)
```

---

## Test Strategy

### Phase 1: Unit Testing (Day 1)
**Focus**: Metrics calculation logic

```bash
cargo test --package antigravity_tools_lib audio_analytics
```

---

### Phase 2: Integration Testing (Day 2)
**Focus**: End-to-end analytics flow

```bash
cargo test --package antigravity_tools_lib handlers::audio::analytics
```

---

### Phase 3: UI Testing (Day 3)
**Focus**: Dashboard visualization

**Manual Testing**:
1. Upload 20 audio files (various formats, sizes, durations)
2. Verify metrics update in dashboard
3. Check historical data persistence
4. Verify 30-day cleanup works

---

## Dependencies

### Internal Dependencies
- `src-tauri/src/proxy/monitor.rs` - SHARED with Team 2 (Epic-024)
- `src-tauri/src/proxy/handlers/audio.rs` - STABLE (collect metadata)
- `src-tauri/src/db/mod.rs` - STABLE (SQLite database)

---

## Success Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Cost savings (cache optimization) | 15-20% | Compare cache hit rate before/after analytics |
| Operational visibility | 100% | All audio patterns tracked |
| Dashboard usability | 90%+ helpful rating | User survey |
| Test coverage | 100% | `cargo tarpaulin` |

---

## Definition of Done

### Code Complete
- ✅ Audio duration, format, file size tracking implemented
- ✅ Dashboard visualization added
- ✅ SQLite schema created with 30-day retention
- ✅ monitor.rs extended with audio analytics

### Testing Complete
- ✅ 8+ unit tests passing
- ✅ 4+ integration tests passing
- ✅ Dashboard UI tested (manual verification)

### Quality Gates Passed
- ✅ 100% audio usage patterns tracked
- ✅ 30-day historical data persistence
- ✅ Dashboard shows actionable insights

---

## Risk Assessment

**Risk 1**: Performance overhead (metrics collection slows requests)
- **Impact**: MEDIUM (request latency)
- **Probability**: LOW
- **Mitigation**: Async metrics collection, no blocking operations

**Risk 2**: SQLite storage growth (30-day data too large)
- **Impact**: LOW (disk space)
- **Probability**: LOW
- **Mitigation**: Automatic cleanup trigger, estimate <10MB for 30 days

---

## Future Enhancements

- Machine learning: Predict optimal cache strategy from patterns
- Anomaly detection: Alert on unusual audio usage patterns
- Quality scoring: Track transcription accuracy trends
- Cost attribution: Per-user audio analytics

---

**Story Status**: ✅ READY FOR EXECUTION
**Assignee**: Dev 1C (Junior QA/Developer)
**Start Date**: 2026-01-22 (Days 1-3)
**Expected Completion**: 2026-01-24 (Day 3)
