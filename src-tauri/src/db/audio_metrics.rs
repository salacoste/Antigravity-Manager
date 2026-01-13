// Audio Usage Analytics - SQLite Storage
// Epic-014 Story-014-04: Audio-specific metrics tracking for operational insights

use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};

/// Audio metrics record for analytics
/// Epic-014: This will be used by Story-014-04 audio analytics dashboard
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMetric {
    pub id: Option<i64>,
    pub timestamp: i64, // Unix timestamp
    pub model_id: String,
    pub duration_secs: Option<u64>,
    pub format: String, // mp3, wav, m4a, ogg, flac, aiff
    pub file_size_bytes: usize,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Audio analytics aggregation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAnalytics {
    pub duration_stats: DurationStats,
    pub format_distribution: Vec<FormatStats>,
    pub file_size_distribution: FileSizeDistribution,
    pub total_requests: u64,
    pub success_rate: f64,
    pub total_audio_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationStats {
    pub min_secs: u64,
    pub max_secs: u64,
    pub avg_secs: f64,
    pub p50_secs: u64, // Median
    pub p95_secs: u64,
    pub p99_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatStats {
    pub format: String,
    pub count: u64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSizeDistribution {
    pub under_1mb: u64,
    pub mb_1_to_5: u64,
    pub mb_5_to_10: u64,
    pub mb_10_to_15: u64,
    pub over_15mb: u64,
    pub avg_size_mb: f64,
    pub p95_size_mb: f64,
}

/// Initialize audio_metrics table
/// Epic-014: Database schema initialization
#[allow(dead_code)]
pub fn init_audio_metrics_table(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS audio_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,
            model_id TEXT NOT NULL,
            duration_secs INTEGER,
            format TEXT NOT NULL,
            file_size_bytes INTEGER NOT NULL,
            success INTEGER NOT NULL,
            error_message TEXT,
            UNIQUE(timestamp, model_id, file_size_bytes)
        )",
        [],
    )?;

    // Create index for faster queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_audio_metrics_timestamp
         ON audio_metrics(timestamp DESC)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_audio_metrics_model
         ON audio_metrics(model_id)",
        [],
    )?;

    Ok(())
}

/// Record audio transcription metric
/// Epic-014: Metrics collection (will be used by audio handler)
#[allow(dead_code)]
pub fn record_audio_metric(conn: &Connection, metric: &AudioMetric) -> SqliteResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO audio_metrics
         (timestamp, model_id, duration_secs, format, file_size_bytes, success, error_message)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            metric.timestamp,
            metric.model_id,
            metric.duration_secs,
            metric.format,
            metric.file_size_bytes as i64,
            metric.success as i32,
            metric.error_message,
        ],
    )?;

    Ok(())
}

/// Get audio analytics for last N days
pub fn get_audio_analytics(conn: &Connection, days: u32) -> SqliteResult<AudioAnalytics> {
    let cutoff_timestamp = chrono::Utc::now().timestamp() - (days as i64 * 86400);

    // Total requests and success rate
    let (total_requests, successful_requests): (u64, u64) = conn.query_row(
        "SELECT COUNT(*), SUM(success) FROM audio_metrics WHERE timestamp >= ?1",
        params![cutoff_timestamp],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    let success_rate = if total_requests > 0 {
        (successful_requests as f64 / total_requests as f64) * 100.0
    } else {
        0.0
    };

    // Duration statistics (only successful requests with duration)
    let duration_stats = calculate_duration_stats(conn, cutoff_timestamp)?;

    // Format distribution
    let format_distribution =
        calculate_format_distribution(conn, cutoff_timestamp, total_requests)?;

    // File size distribution
    let file_size_distribution = calculate_file_size_distribution(conn, cutoff_timestamp)?;

    // Total audio hours
    let total_audio_secs: Option<i64> = conn.query_row(
        "SELECT SUM(duration_secs) FROM audio_metrics
         WHERE timestamp >= ?1 AND success = 1 AND duration_secs IS NOT NULL",
        params![cutoff_timestamp],
        |row| row.get(0),
    )?;

    let total_audio_hours = total_audio_secs.unwrap_or(0) as f64 / 3600.0;

    Ok(AudioAnalytics {
        duration_stats,
        format_distribution,
        file_size_distribution,
        total_requests,
        success_rate,
        total_audio_hours,
    })
}

/// Calculate duration statistics (percentiles)
fn calculate_duration_stats(
    conn: &Connection,
    cutoff_timestamp: i64,
) -> SqliteResult<DurationStats> {
    // Get all durations for percentile calculation
    let mut stmt = conn.prepare(
        "SELECT duration_secs FROM audio_metrics
         WHERE timestamp >= ?1 AND success = 1 AND duration_secs IS NOT NULL
         ORDER BY duration_secs ASC",
    )?;

    let durations: Vec<u64> = stmt
        .query_map(params![cutoff_timestamp], |row| {
            row.get::<_, i64>(0).map(|v| v as u64)
        })?
        .filter_map(Result::ok)
        .collect();

    if durations.is_empty() {
        return Ok(DurationStats {
            min_secs: 0,
            max_secs: 0,
            avg_secs: 0.0,
            p50_secs: 0,
            p95_secs: 0,
            p99_secs: 0,
        });
    }

    let min_secs = *durations.first().unwrap();
    let max_secs = *durations.last().unwrap();
    let avg_secs = durations.iter().sum::<u64>() as f64 / durations.len() as f64;

    // Calculate percentiles
    let p50_secs = percentile(&durations, 50);
    let p95_secs = percentile(&durations, 95);
    let p99_secs = percentile(&durations, 99);

    Ok(DurationStats {
        min_secs,
        max_secs,
        avg_secs,
        p50_secs,
        p95_secs,
        p99_secs,
    })
}

/// Calculate format distribution
fn calculate_format_distribution(
    conn: &Connection,
    cutoff_timestamp: i64,
    total_requests: u64,
) -> SqliteResult<Vec<FormatStats>> {
    let mut stmt = conn.prepare(
        "SELECT format, COUNT(*) as count FROM audio_metrics
         WHERE timestamp >= ?1
         GROUP BY format
         ORDER BY count DESC",
    )?;

    let formats = stmt
        .query_map(params![cutoff_timestamp], |row| {
            let format: String = row.get(0)?;
            let count: u64 = row.get(1)?;
            let percentage = if total_requests > 0 {
                (count as f64 / total_requests as f64) * 100.0
            } else {
                0.0
            };

            Ok(FormatStats {
                format,
                count,
                percentage,
            })
        })?
        .filter_map(Result::ok)
        .collect();

    Ok(formats)
}

/// Calculate file size distribution
fn calculate_file_size_distribution(
    conn: &Connection,
    cutoff_timestamp: i64,
) -> SqliteResult<FileSizeDistribution> {
    // Count by size buckets
    let (under_1mb, mb_1_to_5, mb_5_to_10, mb_10_to_15, over_15mb): (u64, u64, u64, u64, u64) = conn.query_row(
        "SELECT
            SUM(CASE WHEN file_size_bytes < 1048576 THEN 1 ELSE 0 END) as under_1mb,
            SUM(CASE WHEN file_size_bytes >= 1048576 AND file_size_bytes < 5242880 THEN 1 ELSE 0 END) as mb_1_to_5,
            SUM(CASE WHEN file_size_bytes >= 5242880 AND file_size_bytes < 10485760 THEN 1 ELSE 0 END) as mb_5_to_10,
            SUM(CASE WHEN file_size_bytes >= 10485760 AND file_size_bytes < 15728640 THEN 1 ELSE 0 END) as mb_10_to_15,
            SUM(CASE WHEN file_size_bytes >= 15728640 THEN 1 ELSE 0 END) as over_15mb
         FROM audio_metrics WHERE timestamp >= ?1",
        params![cutoff_timestamp],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
    )?;

    // Average file size
    let avg_size_bytes: Option<i64> = conn.query_row(
        "SELECT AVG(file_size_bytes) FROM audio_metrics WHERE timestamp >= ?1",
        params![cutoff_timestamp],
        |row| row.get(0),
    )?;

    let avg_size_mb = avg_size_bytes.unwrap_or(0) as f64 / 1_048_576.0;

    // P95 file size
    let mut stmt = conn.prepare(
        "SELECT file_size_bytes FROM audio_metrics
         WHERE timestamp >= ?1
         ORDER BY file_size_bytes ASC",
    )?;

    let file_sizes: Vec<u64> = stmt
        .query_map(params![cutoff_timestamp], |row| {
            row.get::<_, i64>(0).map(|v| v as u64)
        })?
        .filter_map(Result::ok)
        .collect();

    let p95_size_mb = if !file_sizes.is_empty() {
        percentile(&file_sizes, 95) as f64 / 1_048_576.0
    } else {
        0.0
    };

    Ok(FileSizeDistribution {
        under_1mb,
        mb_1_to_5,
        mb_5_to_10,
        mb_10_to_15,
        over_15mb,
        avg_size_mb,
        p95_size_mb,
    })
}

/// Calculate percentile from sorted array
fn percentile(sorted_data: &[u64], p: u8) -> u64 {
    if sorted_data.is_empty() {
        return 0;
    }

    let index = ((p as f64 / 100.0) * (sorted_data.len() - 1) as f64).round() as usize;
    sorted_data[index]
}

/// Clean up old metrics (older than N days)
/// Epic-014: Maintenance function for data retention
#[allow(dead_code)]
pub fn cleanup_old_metrics(conn: &Connection, days: u32) -> SqliteResult<usize> {
    let cutoff_timestamp = chrono::Utc::now().timestamp() - (days as i64 * 86400);

    conn.execute(
        "DELETE FROM audio_metrics WHERE timestamp < ?1",
        params![cutoff_timestamp],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_audio_metrics_table(&conn).unwrap();
        conn
    }

    #[test]
    fn test_init_audio_metrics_table() {
        let conn = create_test_db();

        // Verify table exists
        let table_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='audio_metrics'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(table_exists, 1);
    }

    #[test]
    fn test_record_audio_metric() {
        let conn = create_test_db();

        let metric = AudioMetric {
            id: None,
            timestamp: chrono::Utc::now().timestamp(),
            model_id: "gemini-2.0-flash-exp".to_string(),
            duration_secs: Some(120),
            format: "mp3".to_string(),
            file_size_bytes: 5_000_000,
            success: true,
            error_message: None,
        };

        record_audio_metric(&conn, &metric).unwrap();

        // Verify record exists
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM audio_metrics", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn test_percentile_calculation() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(percentile(&data, 50), 5); // Median
        assert_eq!(percentile(&data, 95), 10); // P95
        assert_eq!(percentile(&data, 99), 10); // P99
    }

    #[test]
    fn test_cleanup_old_metrics() {
        let conn = create_test_db();

        // Insert old metric
        let old_metric = AudioMetric {
            id: None,
            timestamp: chrono::Utc::now().timestamp() - (40 * 86400), // 40 days ago
            model_id: "gemini-2.0-flash-exp".to_string(),
            duration_secs: Some(60),
            format: "wav".to_string(),
            file_size_bytes: 2_000_000,
            success: true,
            error_message: None,
        };

        record_audio_metric(&conn, &old_metric).unwrap();

        // Insert recent metric
        let recent_metric = AudioMetric {
            id: None,
            timestamp: chrono::Utc::now().timestamp(),
            model_id: "gemini-2.0-flash-exp".to_string(),
            duration_secs: Some(90),
            format: "mp3".to_string(),
            file_size_bytes: 3_000_000,
            success: true,
            error_message: None,
        };

        record_audio_metric(&conn, &recent_metric).unwrap();

        // Cleanup metrics older than 30 days
        let deleted = cleanup_old_metrics(&conn, 30).unwrap();
        assert_eq!(deleted, 1);

        // Verify only recent metric remains
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM audio_metrics", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);
    }
}
