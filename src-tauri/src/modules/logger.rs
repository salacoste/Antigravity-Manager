use crate::modules::account::get_data_dir;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// 自定义本地时区时间格式化器
struct LocalTimer;

impl tracing_subscriber::fmt::time::FormatTime for LocalTimer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Local::now();
        write!(w, "{}", now.to_rfc3339())
    }
}

pub fn get_log_dir() -> Result<PathBuf, String> {
    let data_dir = get_data_dir()?;
    let log_dir = data_dir.join("logs");

    if !log_dir.exists() {
        fs::create_dir_all(&log_dir).map_err(|e| format!("创建日志目录失败: {}", e))?;
    }

    Ok(log_dir)
}

/// 初始化日志系统
pub fn init_logger() {
    // 捕获 log 宏日志
    let _ = tracing_log::LogTracer::init();

    let log_dir = match get_log_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("无法初始化日志目录: {}", e);
            return;
        }
    };

    // 1. 设置文件 Appender (使用 tracing-appender 实现滚动记录)
    // 这里使用每天滚动的策略
    let file_appender = tracing_appender::rolling::daily(log_dir, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 2. 终端输出层（使用本地时区）
    let console_layer = fmt::Layer::new()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .with_timer(LocalTimer);

    // 3. 文件输出层 (关闭 ANSI 格式化，使用本地时区)
    let file_layer = fmt::Layer::new()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_level(true)
        .with_timer(LocalTimer);

    // 4. 设置过滤层 (默认使用 INFO 级别以减少日志体积)
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 5. 初始化全局订阅器 (使用 try_init 避免重复初始化崩溃)
    let _ = tracing_subscriber::registry()
        .with(filter_layer)
        .with(console_layer)
        .with(file_layer)
        .try_init();

    // 泄漏 _guard 以确保其生命周期持续到程序退出
    // 这是使用 tracing_appender::non_blocking 时的推荐做法（如果不需要手动刷盘）
    std::mem::forget(_guard);

    info!("日志系统已完成初始化 (终端控制台 + 文件持久化)");

    // 自动清理 7 天前的旧日志
    if let Err(e) = cleanup_old_logs(7) {
        warn!("清理旧日志失败: {}", e);
    }
}

/// 清理指定天数之前的旧日志文件
pub fn cleanup_old_logs(days_to_keep: u64) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let log_dir = get_log_dir()?;
    if !log_dir.exists() {
        return Ok(());
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("获取系统时间失败: {}", e))?
        .as_secs();

    let cutoff_time = now.saturating_sub(days_to_keep * 24 * 60 * 60);
    let mut deleted_count = 0;
    let mut total_size_freed = 0u64;

    let entries = fs::read_dir(&log_dir).map_err(|e| format!("读取日志目录失败: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();

        // 只处理文件，跳过目录
        if !path.is_file() {
            continue;
        }

        // 获取文件修改时间
        if let Ok(metadata) = fs::metadata(&path) {
            if let Ok(modified) = metadata.modified() {
                let modified_secs = modified
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                // 如果文件早于截止时间，删除它
                if modified_secs < cutoff_time {
                    let file_size = metadata.len();
                    if let Err(e) = fs::remove_file(&path) {
                        warn!("删除旧日志文件失败 {:?}: {}", path, e);
                    } else {
                        deleted_count += 1;
                        total_size_freed += file_size;
                        info!("已删除旧日志文件: {:?}", path.file_name());
                    }
                }
            }
        }
    }

    if deleted_count > 0 {
        let size_mb = total_size_freed as f64 / 1024.0 / 1024.0;
        info!(
            "日志清理完成: 删除了 {} 个文件，释放 {:.2} MB 空间",
            deleted_count, size_mb
        );
    }

    Ok(())
}

/// 清理日志缓存 (采用截断模式以保持文件句柄有效)
pub fn clear_logs() -> Result<(), String> {
    let log_dir = get_log_dir()?;
    if log_dir.exists() {
        // 遍历目录下的所有文件并截断，而不是删除目录
        let entries = fs::read_dir(&log_dir).map_err(|e| format!("读取日志目录失败: {}", e))?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                // 使用截断模式打开文件，将大小设为 0
                let _ = fs::OpenOptions::new().write(true).truncate(true).open(path);
            }
        }
    }
    Ok(())
}

/// 记录信息日志 (向后兼容接口)
pub fn log_info(message: &str) {
    info!("{}", message);
}

/// 记录警告日志 (向后兼容接口)
pub fn log_warn(message: &str) {
    warn!("{}", message);
}

/// 记录错误日志 (向后兼容接口)
pub fn log_error(message: &str) {
    error!("{}", message);
}

/// 🆕 Story #024-04 Part 1: Log detection event with structured format
/// Detection events are logged with category="detection" for easy filtering
pub fn log_detection_event(event: &crate::proxy::detection::DetectionEvent) {
    warn!(
        category = "detection",
        event_type = ?event.event_type,
        severity = ?event.severity,
        account = %event.account_id,
        model = %event.model_id,
        request_id = %event.request_id,
        upstream_status = ?event.upstream_status,
        context = %event.context,
        "Detection event logged"
    );
}
