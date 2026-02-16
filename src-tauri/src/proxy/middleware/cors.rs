// CORS 中间件
use axum::http::HeaderValue;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

fn configured_cors_origins() -> Option<Vec<HeaderValue>> {
    let raw = std::env::var("ABV_CORS_ALLOW_ORIGINS")
        .or_else(|_| std::env::var("CORS_ALLOW_ORIGINS"))
        .unwrap_or_default();

    let origins: Vec<HeaderValue> = raw
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect();

    if origins.is_empty() {
        None
    } else {
        Some(origins)
    }
}

/// 创建 CORS layer
pub fn cors_layer() -> CorsLayer {
    let base = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
            Method::PATCH,
        ])
        .allow_headers(Any)
        .allow_credentials(false)
        .max_age(std::time::Duration::from_secs(3600));

    if let Some(origins) = configured_cors_origins() {
        tracing::info!("[CORS] Restricting allowed origins from env");
        base.allow_origin(origins)
    } else {
        tracing::warn!("[CORS] ABV_CORS_ALLOW_ORIGINS is empty, allowing any origin");
        base.allow_origin(Any)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_layer_creation() {
        let _layer = cors_layer();
        // Layer 创建成功
        assert!(true);
    }
}
