use crate::proxy::monitor::ProxyRequestLog;
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub fn get_proxy_db_path() -> Result<PathBuf, String> {
    let data_dir = crate::modules::account::get_data_dir()?;
    Ok(data_dir.join("proxy_logs.db"))
}

pub fn init_db() -> Result<(), String> {
    let db_path = get_proxy_db_path()?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS request_logs (
            id TEXT PRIMARY KEY,
            timestamp INTEGER,
            method TEXT,
            url TEXT,
            status INTEGER,
            duration INTEGER,
            model TEXT,
            error TEXT
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    // Try to add new columns (ignore errors if they exist)
    let _ = conn.execute("ALTER TABLE request_logs ADD COLUMN request_body TEXT", []);
    let _ = conn.execute("ALTER TABLE request_logs ADD COLUMN response_body TEXT", []);
    let _ = conn.execute(
        "ALTER TABLE request_logs ADD COLUMN input_tokens INTEGER",
        [],
    );
    let _ = conn.execute(
        "ALTER TABLE request_logs ADD COLUMN output_tokens INTEGER",
        [],
    );
    let _ = conn.execute("ALTER TABLE request_logs ADD COLUMN account_email TEXT", []);
    let _ = conn.execute("ALTER TABLE request_logs ADD COLUMN mapped_model TEXT", []);

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_timestamp ON request_logs (timestamp DESC)",
        [],
    )
    .map_err(|e| e.to_string())?;

    // 🆕 Story #8: Run migration for proxy_stats table
    migrate_stats_table()?;

    Ok(())
}

pub fn save_log(log: &ProxyRequestLog) -> Result<(), String> {
    let db_path = get_proxy_db_path()?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO request_logs (id, timestamp, method, url, status, duration, model, error, request_body, response_body, input_tokens, output_tokens, account_email, mapped_model)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            log.id,
            log.timestamp,
            log.method,
            log.url,
            log.status,
            log.duration,
            log.model,
            log.error,
            log.request_body,
            log.response_body,
            log.input_tokens,
            log.output_tokens,
            log.account_email,
            log.mapped_model,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn get_logs(limit: usize) -> Result<Vec<ProxyRequestLog>, String> {
    let db_path = get_proxy_db_path()?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, timestamp, method, url, status, duration, model, error, request_body, response_body, input_tokens, output_tokens, account_email, mapped_model
         FROM request_logs 
         ORDER BY timestamp DESC 
         LIMIT ?1"
    ).map_err(|e| e.to_string())?;

    let logs_iter = stmt
        .query_map([limit], |row| {
            Ok(ProxyRequestLog {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                method: row.get(2)?,
                url: row.get(3)?,
                status: row.get(4)?,
                duration: row.get(5)?,
                model: row.get(6)?,
                mapped_model: row.get(13).unwrap_or(None),
                account_email: row.get(12).unwrap_or(None),
                error: row.get(7)?,
                request_body: row.get(8).unwrap_or(None),
                response_body: row.get(9).unwrap_or(None),
                input_tokens: row.get(10).unwrap_or(None),
                output_tokens: row.get(11).unwrap_or(None),
            })
        })
        .map_err(|e| e.to_string())?;

    let mut logs = Vec::new();
    for log in logs_iter {
        logs.push(log.map_err(|e| e.to_string())?);
    }
    Ok(logs)
}

// 🆕 Story #8: Updated to load from proxy_stats table
pub fn get_stats() -> Result<crate::proxy::monitor::ProxyStats, String> {
    let db_path = get_proxy_db_path()?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    // Try to load from proxy_stats table first
    let stats_result = conn.query_row(
        "SELECT total_requests, success_count, error_count,
                thinking_budget_violations, thinking_position_violations,
                thinking_position_violations_user, thinking_position_violations_model
         FROM proxy_stats WHERE id = 1",
        [],
        |row| {
            Ok(crate::proxy::monitor::ProxyStats {
                total_requests: row.get(0)?,
                success_count: row.get(1)?,
                error_count: row.get(2)?,
                thinking_budget_violations: row.get(3)?,
                thinking_position_violations: row.get(4)?,
                thinking_position_violations_user: row.get(5)?,
                thinking_position_violations_model: row.get(6)?,
            })
        },
    );

    match stats_result {
        Ok(stats) => Ok(stats),
        Err(_) => {
            // Fallback to calculating from request_logs if proxy_stats doesn't exist yet
            tracing::warn!("[ProxyDB] proxy_stats table not found, calculating from logs");

            let total_requests: u64 = conn
                .query_row("SELECT COUNT(*) FROM request_logs", [], |row| row.get(0))
                .map_err(|e| e.to_string())?;

            let success_count: u64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM request_logs WHERE status >= 200 AND status < 400",
                    [],
                    |row| row.get(0),
                )
                .map_err(|e| e.to_string())?;

            let error_count: u64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM request_logs WHERE status < 200 OR status >= 400",
                    [],
                    |row| row.get(0),
                )
                .map_err(|e| e.to_string())?;

            Ok(crate::proxy::monitor::ProxyStats {
                total_requests,
                success_count,
                error_count,
                thinking_budget_violations: 0,
                thinking_position_violations: 0,
                thinking_position_violations_user: 0,
                thinking_position_violations_model: 0,
            })
        }
    }
}

pub fn clear_logs() -> Result<(), String> {
    let db_path = get_proxy_db_path()?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM request_logs", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ========== Story #8: Violation Metrics Database Integration ==========

/// 🆕 Story #8 Step 10: Migrate proxy_stats table
/// Creates proxy_stats table with violation metrics columns
/// Idempotent - safe to call multiple times
pub fn migrate_stats_table() -> Result<(), String> {
    let db_path = get_proxy_db_path()?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    // Check if table already exists and has violation columns
    let has_violation_columns: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('proxy_stats')
             WHERE name IN ('thinking_budget_violations', 'thinking_position_violations',
                           'thinking_position_violations_user', 'thinking_position_violations_model')",
            [],
            |row| {
                let count: i32 = row.get(0)?;
                Ok(count == 4)
            },
        )
        .unwrap_or(false);

    if has_violation_columns {
        tracing::info!("[ProxyDB] proxy_stats table already migrated with violation columns");
        return Ok(());
    }

    // Create proxy_stats table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS proxy_stats (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            total_requests INTEGER DEFAULT 0,
            success_count INTEGER DEFAULT 0,
            error_count INTEGER DEFAULT 0,
            thinking_budget_violations INTEGER DEFAULT 0,
            thinking_position_violations INTEGER DEFAULT 0,
            thinking_position_violations_user INTEGER DEFAULT 0,
            thinking_position_violations_model INTEGER DEFAULT 0,
            updated_at INTEGER
        )",
        [],
    )
    .map_err(|e| format!("Failed to create proxy_stats table: {}", e))?;

    // Insert initial row (id=1, single-row table pattern)
    conn.execute("INSERT OR IGNORE INTO proxy_stats (id) VALUES (1)", [])
        .map_err(|e| format!("Failed to insert initial proxy_stats row: {}", e))?;

    tracing::info!("[ProxyDB] Successfully migrated proxy_stats table with violation metrics");
    Ok(())
}

/// 🆕 Story #8 Step 11: Save stats to database
/// Persists ProxyStats including violation counters
pub fn save_stats(stats: &crate::proxy::monitor::ProxyStats) -> Result<(), String> {
    let db_path = get_proxy_db_path()?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT OR REPLACE INTO proxy_stats (
            id,
            total_requests,
            success_count,
            error_count,
            thinking_budget_violations,
            thinking_position_violations,
            thinking_position_violations_user,
            thinking_position_violations_model,
            updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            1, // Single-row table, always id=1
            stats.total_requests,
            stats.success_count,
            stats.error_count,
            stats.thinking_budget_violations,
            stats.thinking_position_violations,
            stats.thinking_position_violations_user,
            stats.thinking_position_violations_model,
            now,
        ],
    )
    .map_err(|e| format!("Failed to save proxy stats: {}", e))?;

    Ok(())
}
