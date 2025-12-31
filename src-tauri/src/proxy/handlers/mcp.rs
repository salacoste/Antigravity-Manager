use axum::{
    body::{to_bytes, Body},
    extract::State,
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use futures::StreamExt;
use serde_json::{json, Value};
use tokio::time::Duration;
use tokio_stream::wrappers::IntervalStream;

use crate::proxy::server::AppState;

fn should_strip_tracking_param(key: &str) -> bool {
    let k = key.to_ascii_lowercase();
    k.starts_with("utm_")
        || k.starts_with("hsa_")
        || matches!(
            k.as_str(),
            "gclid" | "fbclid" | "gbraid" | "wbraid" | "msclkid"
        )
}

fn normalize_web_reader_url(
    url_str: &str,
    mode: crate::proxy::config::ZaiWebReaderUrlNormalizationMode,
) -> Option<String> {
    use crate::proxy::config::ZaiWebReaderUrlNormalizationMode as Mode;

    if matches!(mode, Mode::Off) {
        return None;
    }

    let mut url = url::Url::parse(url_str).ok()?;
    if url.scheme() != "http" && url.scheme() != "https" {
        return None;
    }

    match mode {
        Mode::Off => None,
        Mode::StripQuery => {
            if url.query().is_none() {
                return None;
            }
            url.set_query(None);
            Some(url.to_string())
        }
        Mode::StripTrackingQuery => {
            let Some(q) = url.query() else {
                return None;
            };
            let pairs: Vec<(String, String)> = url::form_urlencoded::parse(q.as_bytes())
                .into_owned()
                .filter(|(k, _)| !should_strip_tracking_param(k))
                .collect();

            // If nothing changes, keep as-is.
            let original_len = url::form_urlencoded::parse(q.as_bytes()).count();
            if pairs.len() == original_len {
                return None;
            }

            if pairs.is_empty() {
                url.set_query(None);
                return Some(url.to_string());
            }

            let mut ser = url::form_urlencoded::Serializer::new(String::new());
            for (k, v) in pairs {
                ser.append_pair(&k, &v);
            }
            let new_q = ser.finish();
            url.set_query(Some(&new_q));
            Some(url.to_string())
        }
    }
}

fn maybe_normalize_web_reader_body(
    collected: bytes::Bytes,
    mode: crate::proxy::config::ZaiWebReaderUrlNormalizationMode,
) -> bytes::Bytes {
    use serde_json::Value;

    if matches!(mode, crate::proxy::config::ZaiWebReaderUrlNormalizationMode::Off) {
        return collected;
    }

    let Ok(mut v) = serde_json::from_slice::<Value>(&collected) else {
        return collected;
    };

    // Expect JSON-RPC: { method: "tools/call", params: { name, arguments: { url } } }
    let is_tools_call = v
        .get("method")
        .and_then(|m| m.as_str())
        .map(|m| m == "tools/call")
        .unwrap_or(false);
    if !is_tools_call {
        return collected;
    }

    let tool_name = v
        .get("params")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("");
    if tool_name != "webReader" {
        return collected;
    }

    let Some(url_value) = v
        .get_mut("params")
        .and_then(|p| p.get_mut("arguments"))
        .and_then(|a| a.get_mut("url"))
    else {
        return collected;
    };

    let Some(url_str) = url_value.as_str() else {
        return collected;
    };

    let Some(normalized) = normalize_web_reader_url(url_str, mode) else {
        return collected;
    };

    *url_value = Value::String(normalized);
    match serde_json::to_vec(&v) {
        Ok(bytes) => bytes::Bytes::from(bytes),
        Err(_) => collected,
    }
}

fn build_client(
    upstream_proxy: crate::proxy::config::UpstreamProxyConfig,
    timeout_secs: u64,
) -> Result<reqwest::Client, String> {
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs.max(5)));

    if upstream_proxy.enabled && !upstream_proxy.url.is_empty() {
        let proxy = reqwest::Proxy::all(&upstream_proxy.url)
            .map_err(|e| format!("Invalid upstream proxy url: {}", e))?;
        builder = builder.proxy(proxy);
    }

    builder.build().map_err(|e| format!("Failed to build HTTP client: {}", e))
}

fn copy_passthrough_headers(incoming: &HeaderMap) -> HeaderMap {
    let mut out = HeaderMap::new();
    for (k, v) in incoming.iter() {
        let key = k.as_str().to_ascii_lowercase();
        match key.as_str() {
            // Forward minimal safe set + MCP session header for streamableHttp.
            "content-type" | "accept" | "user-agent" | "mcp-session-id" => {
                out.insert(k.clone(), v.clone());
            }
            _ => {}
        }
    }
    out
}

async fn forward_mcp(
    state: &AppState,
    incoming_headers: HeaderMap,
    method: Method,
    upstream_url: &str,
    body: Body,
) -> Response {
    let zai = state.zai.read().await.clone();
    if !zai.enabled || zai.api_key.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "z.ai is not configured").into_response();
    }

    if !zai.mcp.enabled {
        return StatusCode::NOT_FOUND.into_response();
    }

    let upstream_proxy = state.upstream_proxy.read().await.clone();
    let client = match build_client(upstream_proxy, state.request_timeout) {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };

    let mut collected = match to_bytes(body, 100 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Failed to read request body: {}", e),
            )
                .into_response();
        }
    };

    // Optional normalization for Web Reader tool calls to improve upstream compatibility.
    if method == Method::POST && upstream_url.contains("/mcp/web_reader/") {
        collected = maybe_normalize_web_reader_body(collected, zai.mcp.web_reader_url_normalization);
    }

    let mut headers = copy_passthrough_headers(&incoming_headers);
    // z.ai MCP server requires Accept to include both application/json and text/event-stream.
    // We set it here to be resilient to clients that send only application/json or */*.
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_static("application/json, text/event-stream"),
    );
    let mcp_api_key = if !zai.mcp.api_key_override.trim().is_empty() {
        zai.mcp.api_key_override.trim()
    } else {
        zai.api_key.trim()
    };
    if let Ok(v) = HeaderValue::from_str(&format!("Bearer {}", mcp_api_key)) {
        headers.insert(header::AUTHORIZATION, v);
    }

    let req = client
        .request(method, upstream_url)
        .headers(headers)
        .body(collected);

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

    let stream = resp.bytes_stream().map(|chunk| match chunk {
        Ok(b) => Ok::<Bytes, std::io::Error>(b),
        Err(e) => Ok(Bytes::from(format!("Upstream stream error: {}", e))),
    });

    out.body(Body::from_stream(stream)).unwrap_or_else(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build response").into_response()
    })
}

pub async fn handle_web_search_prime(
    State(state): State<AppState>,
    headers: HeaderMap,
    method: Method,
    body: Body,
) -> Response {
    let zai = state.zai.read().await.clone();
    if !zai.mcp.web_search_enabled {
        return StatusCode::NOT_FOUND.into_response();
    }
    drop(zai);

    let mut resp = forward_mcp(
        &state,
        headers,
        method,
        "https://api.z.ai/api/mcp/web_search_prime/mcp",
        body,
    )
    .await;
    resp.extensions_mut()
        .insert(crate::proxy::observability::UpstreamRoute("zai_mcp"));
    resp
}

pub async fn handle_web_reader(
    State(state): State<AppState>,
    headers: HeaderMap,
    method: Method,
    body: Body,
) -> Response {
    let zai = state.zai.read().await.clone();
    if !zai.mcp.web_reader_enabled {
        return StatusCode::NOT_FOUND.into_response();
    }
    drop(zai);

    let mut resp = forward_mcp(
        &state,
        headers,
        method,
        "https://api.z.ai/api/mcp/web_reader/mcp",
        body,
    )
    .await;
    resp.extensions_mut()
        .insert(crate::proxy::observability::UpstreamRoute("zai_mcp"));
    resp
}

pub async fn handle_zread(
    State(state): State<AppState>,
    headers: HeaderMap,
    method: Method,
    body: Body,
) -> Response {
    let zai = state.zai.read().await.clone();
    if !zai.mcp.zread_enabled {
        return StatusCode::NOT_FOUND.into_response();
    }
    drop(zai);

    let mut resp = forward_mcp(&state, headers, method, "https://api.z.ai/api/mcp/zread/mcp", body).await;
    resp.extensions_mut()
        .insert(crate::proxy::observability::UpstreamRoute("zai_mcp"));
    resp
}

fn mcp_session_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get("mcp-session-id")
        .or_else(|| headers.get("Mcp-Session-Id"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

fn jsonrpc_error(id: Value, code: i64, message: impl Into<String>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "error": {
            "code": code,
            "message": message.into(),
        },
        "id": id,
    })
}

fn jsonrpc_result(id: Value, result: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "result": result,
        "id": id,
    })
}

fn is_initialize_request(body: &Value) -> bool {
    body.get("method").and_then(|m| m.as_str()) == Some("initialize")
}

async fn handle_vision_get(state: AppState, headers: HeaderMap) -> Response {
    let Some(session_id) = mcp_session_id(&headers) else {
        return (StatusCode::BAD_REQUEST, "Missing Mcp-Session-Id").into_response();
    };
    if !state.zai_vision_mcp.has_session(&session_id).await {
        return (StatusCode::BAD_REQUEST, "Invalid Mcp-Session-Id").into_response();
    }

    let ping_stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(15))).map(|_| {
        Ok::<axum::response::sse::Event, std::convert::Infallible>(
            axum::response::sse::Event::default()
                .event("ping")
                .data("keepalive"),
        )
    });

    let mut resp = axum::response::sse::Sse::new(ping_stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(15))
                .text("keepalive"),
        )
        .into_response();

    if let Ok(v) = HeaderValue::from_str(&session_id) {
        resp.headers_mut().insert("mcp-session-id", v);
    }
    resp
}

async fn handle_vision_delete(state: AppState, headers: HeaderMap) -> Response {
    let Some(session_id) = mcp_session_id(&headers) else {
        return (StatusCode::BAD_REQUEST, "Missing Mcp-Session-Id").into_response();
    };

    state.zai_vision_mcp.remove_session(&session_id).await;
    StatusCode::OK.into_response()
}

async fn handle_vision_post(state: AppState, headers: HeaderMap, body: Body) -> Response {
    let collected = match to_bytes(body, 100 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Failed to read request body: {}", e),
            )
                .into_response();
        }
    };

    let request_json: Value = match serde_json::from_slice(&collected) {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                axum::Json(jsonrpc_error(Value::Null, -32700, format!("Parse error: {}", e))),
            )
                .into_response();
        }
    };

    let id = request_json.get("id").cloned().unwrap_or(Value::Null);
    let method = request_json
        .get("method")
        .and_then(|m| m.as_str())
        .unwrap_or_default();

    if method.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            axum::Json(jsonrpc_error(id, -32600, "Invalid Request: missing method")),
        )
            .into_response();
    }

    // Notifications (no id) should not produce a response.
    if request_json.get("id").is_none() || request_json.get("id") == Some(&Value::Null) {
        return StatusCode::NO_CONTENT.into_response();
    }

    if is_initialize_request(&request_json) {
        let session_id = state.zai_vision_mcp.create_session().await;
        let requested_protocol = request_json
            .get("params")
            .and_then(|p| p.get("protocolVersion"))
            .and_then(|v| v.as_str())
            .unwrap_or("2024-11-05");

        let result = json!({
            "protocolVersion": requested_protocol,
            "capabilities": { "tools": {} },
            "serverInfo": {
                "name": "zai-mcp-server",
                "version": env!("CARGO_PKG_VERSION"),
            }
        });

        let mut resp = (StatusCode::OK, axum::Json(jsonrpc_result(id, result))).into_response();
        if let Ok(v) = HeaderValue::from_str(&session_id) {
            resp.headers_mut().insert("mcp-session-id", v);
        }
        return resp;
    }

    let Some(session_id) = mcp_session_id(&headers) else {
        return (
            StatusCode::BAD_REQUEST,
            axum::Json(jsonrpc_error(id, -32000, "Bad Request: missing Mcp-Session-Id")),
        )
            .into_response();
    };
    if !state.zai_vision_mcp.has_session(&session_id).await {
        return (
            StatusCode::BAD_REQUEST,
            axum::Json(jsonrpc_error(id, -32000, "Bad Request: invalid Mcp-Session-Id")),
        )
            .into_response();
    }

    match method {
        "tools/list" => {
            let result = json!({ "tools": crate::proxy::zai_vision_tools::tool_specs() });
            (StatusCode::OK, axum::Json(jsonrpc_result(id, result))).into_response()
        }
        "tools/call" => {
            let params = request_json.get("params").cloned().unwrap_or(Value::Null);
            let tool_name = params
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing params.name".to_string());

            let tool_name = match tool_name {
                Ok(v) => v,
                Err(e) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        axum::Json(jsonrpc_error(id, -32602, e)),
                    )
                        .into_response();
                }
            };

            let arguments = params.get("arguments").cloned().unwrap_or(Value::Object(Default::default()));

            let zai = state.zai.read().await.clone();
            let upstream_proxy = state.upstream_proxy.read().await.clone();
            let timeout = state.request_timeout;

            match crate::proxy::zai_vision_tools::call_tool(
                &zai,
                upstream_proxy,
                timeout,
                tool_name,
                &arguments,
            )
            .await
            {
                Ok(tool_result) => {
                    (StatusCode::OK, axum::Json(jsonrpc_result(id, tool_result))).into_response()
                }
                Err(e) => (
                    StatusCode::OK,
                    axum::Json(jsonrpc_result(
                        id,
                        json!({
                            "content": [ { "type": "text", "text": format!("Error: {}", e) } ],
                            "isError": true
                        }),
                    )),
                )
                    .into_response(),
            }
        }
        _ => (
            StatusCode::BAD_REQUEST,
            axum::Json(jsonrpc_error(
                id,
                -32601,
                format!("Method not found: {}", method),
            )),
        )
            .into_response(),
    }
}

pub async fn handle_zai_mcp_server(
    State(state): State<AppState>,
    headers: HeaderMap,
    method: Method,
    body: Body,
) -> Response {
    let zai = state.zai.read().await.clone();
    if !zai.enabled || zai.api_key.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "z.ai is not configured").into_response();
    }
    if !zai.mcp.enabled || !zai.mcp.vision_enabled {
        return StatusCode::NOT_FOUND.into_response();
    }
    drop(zai);

    let mut resp = match method {
        Method::GET => handle_vision_get(state, headers).await,
        Method::DELETE => handle_vision_delete(state, headers).await,
        Method::POST => handle_vision_post(state, headers, body).await,
        _ => StatusCode::METHOD_NOT_ALLOWED.into_response(),
    };
    resp.extensions_mut()
        .insert(crate::proxy::observability::UpstreamRoute("zai_vision_mcp"));
    resp
}
