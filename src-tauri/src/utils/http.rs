use crate::modules::config::load_app_config;
use once_cell::sync::Lazy;
use reqwest::{Client, Proxy};

/// 全局共享的 HTTP 客户端 (15秒超时)
/// Client 内置了连接池，克隆它非常轻量且共用底层连接池
pub static SHARED_CLIENT: Lazy<Client> = Lazy::new(|| create_base_client(15));

/// 全局共享的 HTTP 客户端 (长超时: 60秒，用于预热等)
pub static SHARED_CLIENT_LONG: Lazy<Client> = Lazy::new(|| create_base_client(60));

/// 基础客户端创建逻辑
fn create_base_client(timeout_secs: u64) -> Client {
    let mut builder = Client::builder().timeout(std::time::Duration::from_secs(timeout_secs));

    if let Ok(config) = load_app_config() {
        let proxy_config = config.proxy.upstream_proxy;
        if proxy_config.enabled && !proxy_config.url.is_empty() {
            match Proxy::all(&proxy_config.url) {
                Ok(proxy) => {
                    builder = builder.proxy(proxy);
                    tracing::info!("HTTP 共享客户端已启用上游代理: {}", proxy_config.url);
                }
                Err(e) => {
                    tracing::error!("无效的代理地址: {}, 错误: {}", proxy_config.url, e);
                }
            }
        }
    }

    builder.build().unwrap_or_else(|_| Client::new())
}

/// 获取统一配置的 HTTP 客户端 (15秒超时)
pub fn get_client() -> Client {
    SHARED_CLIENT.clone()
}

/// 获取长超时的 HTTP 客户端 (60秒超时)
pub fn get_long_client() -> Client {
    SHARED_CLIENT_LONG.clone()
}

/// 向后兼容接口：创建统一配置的 HTTP 客户端
pub fn create_client(timeout_secs: u64) -> Client {
    if timeout_secs == 15 {
        get_client()
    } else if timeout_secs == 60 {
        get_long_client()
    } else {
        create_base_client(timeout_secs)
    }
}

/// 创建带指定代理配置的 HTTP 客户端 (特殊用途)
pub fn create_client_with_proxy(
    timeout_secs: u64,
    proxy_config: Option<crate::proxy::config::UpstreamProxyConfig>,
) -> Client {
    let mut builder = Client::builder().timeout(std::time::Duration::from_secs(timeout_secs));

    if let Some(config) = proxy_config {
        if config.enabled && !config.url.is_empty() {
            match Proxy::all(&config.url) {
                Ok(proxy) => {
                    builder = builder.proxy(proxy);
                }
                Err(e) => {
                    tracing::error!("无效的代理地址: {}, 错误: {}", config.url, e);
                }
            }
        }
    }

    builder.build().unwrap_or_else(|_| Client::new())
}
