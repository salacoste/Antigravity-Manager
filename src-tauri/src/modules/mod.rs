pub mod account;
pub mod budget_detector; // Epic-025 Story-025-03
pub mod budget_optimizer; // Epic-025 Story-025-01
pub mod config;
pub mod db;
pub mod i18n;
pub mod logger;
pub mod migration;
pub mod model_selector; // Epic-024 Story-024-02 Adaptive Model Selection
pub mod oauth;
pub mod oauth_server;
pub mod process;
pub mod proxy_db;
pub mod quota;
pub mod quota_cache; // Epic-001 Phase 1: Quota caching (Dev 3)
pub mod quota_fetcher; // Epic-001 Phase 1: Quota API client (Dev 1)
pub mod quota_manager; // Epic-001 Phase 2: Background monitoring (Dev 2)
pub mod signature_cache; // Epic-025 Story-025-02
pub mod thinking_quality; // Epic-025 Story-025-04
pub mod tray;
pub mod weekly_tuner; // Epic-025 Week 7

use crate::models;

// 重新导出常用函数到 modules 命名空间顶级，方便外部调用
pub use account::*;
pub use config::*;
#[allow(unused_imports)]
pub use logger::*;
#[allow(unused_imports)]
pub use quota::*;

pub async fn fetch_quota(
    access_token: &str,
    email: &str,
) -> crate::error::AppResult<(models::QuotaData, Option<String>)> {
    quota::fetch_quota(access_token, email).await
}
