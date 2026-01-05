use axum::{
    body::Body,
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use futures::StreamExt;
use serde_json::Value;
use tokio::time::Duration;

use crate::proxy::server::AppState;

fn sanitize_body_for_zai(mut body: Value) -> Value {
    // z.ai's Anthropic-compatible endpoint is stricter than Anthropic itself:
    // it rejects some optional sampling parameters (observed: `temperature`, `top_p`).
    // To keep client compatibility (e.g. Vercel AI SDK / OpenCode), drop them when present.
    if let Some(obj) = body.as_object_mut() {
        let max_tokens = obj.get("max_tokens").and_then(|v| v.as_u64());
        obj.remove("temperature");
        obj.remove("top_p");
        // `effort` is a client hint not supported by all upstreams.
        obj.remove("effort");
        if let Some(thinking) = obj.get_mut("thinking").and_then(|v| v.as_object_mut()) {
            if let Some(budget) = thinking.remove("budgetTokens") {
                thinking.insert("budget_tokens".to_string(), budget);
            }
            let budget_tokens = thinking
                .get("budget_tokens")
                .and_then(|v| v.as_u64());
            if let (Some(max_tokens), Some(budget_tokens)) = (max_tokens, budget_tokens) {
                if max_tokens <= budget_tokens {
                    let adjusted = max_tokens.saturating_sub(1);
                    tracing::warn!(
                        "[z.ai] Adjusting thinking.budget_tokens {} -> {} to satisfy max_tokens ({}) constraint",
                        budget_tokens,
                        adjusted,
                        max_tokens
                    );
                    thinking.insert("budget_tokens".to_string(), Value::from(adjusted));
                }
            }
        }
    }
    body
}

fn map_model_for_zai(original: &str, state: &crate::proxy::ZaiConfig) -> String {
    let m = original.to_lowercase();
    if let Some(mapped) = state.model_mapping.get(original) {
        return mapped.clone();
    }
    if let Some(mapped) = state.model_mapping.get(&m) {
        return mapped.clone();
    }
    if m.starts_with("zai:") {
        return original[4..].to_string();
    }
    if m.starts_with("glm-") {
        return original.to_string();
    }
    if !m.starts_with("claude-") {
        return original.to_string();
    }
    if m.contains("opus") {
        return state.models.opus.clone();
    }
    if m.contains("haiku") {
        return state.models.haiku.clone();
    }
    state.models.sonnet.clone()
}

fn join_base_url(base: &str, path: &str) -> Result<String, String> {
    let base = base.trim_end_matches('/');
    let path = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    };
    Ok(format!("{}{}", base, path))
}

fn build_client(
    upstream_proxy: Option<crate::proxy::config::UpstreamProxyConfig>,
    timeout_secs: u64,
) -> Result<reqwest::Client, String> {
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs.max(5)));

    if let Some(config) = upstream_proxy {
        if config.enabled && !config.url.is_empty() {
            let proxy = reqwest::Proxy::all(&config.url)
                .map_err(|e| format!("Invalid upstream proxy url: {}", e))?;
            builder = builder.proxy(proxy);
        }
    }

    builder.build().map_err(|e| format!("Failed to build HTTP client: {}", e))
}

fn copy_passthrough_headers(incoming: &HeaderMap) -> HeaderMap {
    // Only forward a conservative set of headers to avoid leaking the local proxy key or cookies.
    let mut out = HeaderMap::new();

    for (k, v) in incoming.iter() {
        let key = k.as_str().to_ascii_lowercase();
        match key.as_str() {
            "content-type" | "accept" | "anthropic-version" | "anthropic-beta" | "user-agent" => {
                out.insert(k.clone(), v.clone());
            }
            // Some clients use these for streaming; safe to pass through.
            "accept-encoding" | "cache-control" => {
                out.insert(k.clone(), v.clone());
            }
            _ => {}
        }
    }

    out
}

fn set_zai_auth(headers: &mut HeaderMap, incoming: &HeaderMap, api_key: &str) {
    // Prefer to keep the same auth scheme as the incoming request:
    // - If the client used x-api-key (Anthropic style), replace it.
    // - Else if it used Authorization, replace it with Bearer.
    // - Else default to x-api-key.
    let has_x_api_key = incoming.contains_key("x-api-key");
    let has_auth = incoming.contains_key(header::AUTHORIZATION);

    if has_x_api_key || !has_auth {
        if let Ok(v) = HeaderValue::from_str(api_key) {
            headers.insert("x-api-key", v);
        }
    }

    if has_auth {
        if let Ok(v) = HeaderValue::from_str(&format!("Bearer {}", api_key)) {
            headers.insert(header::AUTHORIZATION, v);
        }
    }
}

pub async fn forward_anthropic_json(
    state: &AppState,
    method: Method,
    path: &str,
    incoming_headers: &HeaderMap,
    mut body: Value,
) -> Response {
    let zai = state.zai.read().await.clone();
    if !zai.enabled || zai.dispatch_mode == crate::proxy::ZaiDispatchMode::Off {
        return (StatusCode::BAD_REQUEST, "z.ai is disabled").into_response();
    }

    if zai.api_key.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "z.ai api_key is not set").into_response();
    }

    body = sanitize_body_for_zai(body);

    if let Some(model) = body.get("model").and_then(|v| v.as_str()) {
        let mapped = map_model_for_zai(model, &zai);
        body["model"] = Value::String(mapped);
    }

    let url = match join_base_url(&zai.base_url, path) {
        Ok(u) => u,
        Err(e) => return (StatusCode::BAD_REQUEST, e).into_response(),
    };

    let timeout_secs = state.request_timeout.max(5);
    let upstream_proxy = state.upstream_proxy.read().await.clone();
    let client = match build_client(Some(upstream_proxy), timeout_secs) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };

    let mut headers = copy_passthrough_headers(incoming_headers);
    set_zai_auth(&mut headers, incoming_headers, &zai.api_key);

    // Ensure JSON content type.
    headers
        .entry(header::CONTENT_TYPE)
        .or_insert(HeaderValue::from_static("application/json"));

    let req = client.request(method, &url).headers(headers).json(&body);

    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                format!("Upstream request failed: {}", e),
            )
                .into_response();
        }
    };

    let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);

    let mut out = Response::builder().status(status);
    if let Some(ct) = resp.headers().get(header::CONTENT_TYPE) {
        out = out.header(header::CONTENT_TYPE, ct.clone());
    }

    let is_sse = resp
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("text/event-stream"))
        .unwrap_or(false);

    // Stream response body to the client (covers SSE and non-SSE).
    // For SSE, normalize z.ai `event: error` payloads to Anthropic-compatible shape so clients
    // that validate the `type` discriminator don't fail.
    let stream = if is_sse {
        use async_stream::stream;
        use bytes::BytesMut;

        let mut upstream = resp.bytes_stream();

        Body::from_stream(stream! {
            let mut buffer = BytesMut::new();
            let mut current_event: Option<String> = None;

            while let Some(chunk) = upstream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.extend_from_slice(&bytes);

                        while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                            let line = buffer.split_to(pos + 1);
                            let line_bytes = line.freeze();

                            let Ok(line_str) = std::str::from_utf8(&line_bytes) else {
                                yield Ok::<Bytes, std::io::Error>(line_bytes);
                                continue;
                            };

                            let trimmed = line_str.trim_end_matches('\n');
                            if trimmed.trim().is_empty() {
                                current_event = None;
                                yield Ok::<Bytes, std::io::Error>(line_bytes);
                                continue;
                            }

                            if let Some(rest) = trimmed.strip_prefix("event:") {
                                current_event = Some(rest.trim().to_string());
                                yield Ok::<Bytes, std::io::Error>(line_bytes);
                                continue;
                            }

                            if let Some(rest) = trimmed.strip_prefix("data:") {
                                let data = rest.trim();

                                // z.ai sometimes ends error streams with OpenAI-style [DONE].
                                // Convert it to Anthropic-style termination.
                                if data == "[DONE]" {
                                    yield Ok::<Bytes, std::io::Error>(Bytes::from_static(b"event: message_stop\n"));
                                    yield Ok::<Bytes, std::io::Error>(Bytes::from_static(b"data: {\"type\":\"message_stop\"}\n\n"));
                                    current_event = None;
                                    continue;
                                }

                                if current_event.as_deref() == Some("error") {
                                    if let Ok(json) = serde_json::from_str::<Value>(data) {
                                        // z.ai error payload is usually `{ error: {code, message}, request_id }`
                                        // which is missing the Anthropic `type` discriminator.
                                        if json.get("type").is_none() && json.get("error").is_some() {
                                            let code = json
                                                .get("error")
                                                .and_then(|e| e.get("code"))
                                                .and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| v.as_u64().map(|n| n.to_string())))
                                                .unwrap_or_else(|| "unknown".to_string());
                                            let message = json
                                                .get("error")
                                                .and_then(|e| e.get("message"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("Upstream error")
                                                .to_string();
                                            let request_id = json.get("request_id").cloned();

                                            let mut out_json = serde_json::json!({
                                                "type": "error",
                                                "error": {
                                                    "type": "invalid_request_error",
                                                    "message": message,
                                                    "code": code
                                                }
                                            });
                                            if let Some(request_id) = request_id {
                                                out_json["request_id"] = request_id;
                                            }

                                            let encoded = match serde_json::to_string(&out_json) {
                                                Ok(s) => s,
                                                Err(_) => {
                                                    yield Ok::<Bytes, std::io::Error>(line_bytes);
                                                    continue;
                                                }
                                            };

                                            let rewritten = Bytes::from(format!("data: {}\n", encoded));
                                            yield Ok::<Bytes, std::io::Error>(rewritten);
                                            continue;
                                        }
                                    }
                                }

                                yield Ok::<Bytes, std::io::Error>(line_bytes);
                                continue;
                            }

                            yield Ok::<Bytes, std::io::Error>(line_bytes);
                        }
                    }
                    Err(e) => {
                        yield Ok::<Bytes, std::io::Error>(Bytes::from(format!("Upstream stream error: {}", e)));
                        break;
                    }
                }
            }

            if !buffer.is_empty() {
                yield Ok::<Bytes, std::io::Error>(buffer.freeze());
            }
        })
    } else {
        Body::from_stream(resp.bytes_stream().map(|chunk| match chunk {
            Ok(b) => Ok::<Bytes, std::io::Error>(b),
            Err(e) => Ok(Bytes::from(format!("Upstream stream error: {}", e))),
        }))
    };

    out.body(stream).unwrap_or_else(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build response").into_response()
    })
}
