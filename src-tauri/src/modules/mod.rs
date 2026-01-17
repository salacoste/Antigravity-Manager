pub mod account;
pub mod budget_detector;
pub mod budget_optimizer;
pub mod config;
pub mod db;
pub mod device;
pub mod http_api;
pub mod i18n;
pub mod logger;
pub mod migration;
pub mod model_selector;
pub mod oauth;
pub mod oauth_server;
pub mod process;
pub mod proxy_db;
pub mod quota;
pub mod quota_cache;
pub mod quota_fetcher;
pub mod quota_manager;
pub mod quota_monitor;
pub mod scheduler;
pub mod signature_cache;
pub mod thinking_quality;
pub mod tray;
pub mod update_checker;
pub mod weekly_tuner;

use crate::models;

// 重新导出常用函数到 modules 命名空间顶级，方便外部调用
pub use account::*;
pub use config::*;
#[allow(unused_imports)]
pub use logger::*;
#[allow(unused_imports)]
pub use quota::*;
// pub use device::*;

pub async fn fetch_quota(
    access_token: &str,
    email: &str,
) -> crate::error::AppResult<(models::QuotaData, Option<String>)> {
    quota::fetch_quota(access_token, email).await
}
