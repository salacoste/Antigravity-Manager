// Gemini Handler
use axum::{
    extract::State,
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{json, Value};
use tracing::{debug, error, info};

use crate::proxy::mappers::gemini::{unwrap_response, wrap_request};
use crate::proxy::server::AppState;
use crate::proxy::session_manager::SessionManager;

const MAX_RETRY_ATTEMPTS: usize = 3;

/// 处理 generateContent 和 streamGenerateContent
/// 路径参数: model_name, method (e.g. "gemini-pro", "generateContent")
pub async fn handle_generate(
    State(state): State<AppState>,
    Path(model_action): Path<String>,
    Json(body): Json<Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 解析 model:method
    let (model_name, method) = if let Some((m, action)) = model_action.rsplit_once(':') {
        (m.to_string(), action.to_string())
    } else {
        (model_action, "generateContent".to_string())
    };

    crate::modules::logger::log_info(&format!(
        "Received Gemini request: {}/{}",
        model_name, method
    ));

    // 1. 验证方法
    if method != "generateContent" && method != "streamGenerateContent" {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Unsupported method: {}", method),
        ));
    }
    let is_stream = method == "streamGenerateContent";

    // 2. 获取 UpstreamClient 和 TokenManager
    let upstream = state.upstream.clone();
    let token_manager = state.token_manager;
    let pool_size = token_manager.len();
    let max_attempts = MAX_RETRY_ATTEMPTS.min(pool_size).max(1);

    let mut last_error = String::new();

    for attempt in 0..max_attempts {
        // 3. 模型路由解析
        let mapped_model = crate::proxy::common::model_mapping::resolve_model_route(
            &model_name,
            &*state.custom_mapping.read().await,
        );
        // 提取 tools 列表以进行联网探测 (Gemini 风格可能是嵌套的)
        let tools_val: Option<Vec<Value>> =
            body.get("tools").and_then(|t| t.as_array()).map(|arr| {
                let mut flattened = Vec::new();
                for tool_entry in arr {
                    if let Some(decls) = tool_entry
                        .get("functionDeclarations")
                        .and_then(|v| v.as_array())
                    {
                        flattened.extend(decls.iter().cloned());
                    } else {
                        flattened.push(tool_entry.clone());
                    }
                }
                flattened
            });

        let config = crate::proxy::mappers::common_utils::resolve_request_config(
            &model_name,
            &mapped_model,
            &tools_val,
        );

        // 4. 获取 Token (使用准确的 request_type)
        // 提取 SessionId (粘性指纹)
        let session_id = SessionManager::extract_gemini_session_id(&body, &model_name);

        // 关键：在重试尝试 (attempt > 0) 时强制轮换账号
        // 🆕 传递模型参数实现 model-aware rate limiting
        let (access_token, project_id, email) = match token_manager
            .get_token(
                &config.request_type,
                attempt > 0,
                Some(&session_id),
                Some(&mapped_model),
            )
            .await
        {
            Ok(t) => t,
            Err(e) => {
                return Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    format!("Token error: {}", e),
                ));
            }
        };

        info!("✓ Using account: {} (type: {})", email, config.request_type);

        // 5. 包装请求 (project injection)
        let wrapped_body = wrap_request(&body, &project_id, &mapped_model);

        // 5. 上游调用
        let query_string = if is_stream { Some("alt=sse") } else { None };
        let upstream_method = if is_stream {
            "streamGenerateContent"
        } else {
            "generateContent"
        };

        let response = match upstream
            .call_v1_internal(upstream_method, &access_token, wrapped_body, query_string)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                last_error = e.clone();
                debug!(
                    "Gemini Request failed on attempt {}/{}: {}",
                    attempt + 1,
                    max_attempts,
                    e
                );
                continue;
            }
        };

        let status = response.status();
        if status.is_success() {
            // 6. 响应处理
            if is_stream {
                use axum::body::Body;
                use axum::response::Response;
                use bytes::{Bytes, BytesMut};
                use futures::StreamExt;

                let mut response_stream = response.bytes_stream();
                let mut buffer = BytesMut::new();

                let stream = async_stream::stream! {
                    while let Some(item) = response_stream.next().await {
                        match item {
                            Ok(bytes) => {
                                debug!("[Gemini-SSE] Received chunk: {} bytes", bytes.len());
                                buffer.extend_from_slice(&bytes);
                                while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                                    let line_raw = buffer.split_to(pos + 1);
                                    if let Ok(line_str) = std::str::from_utf8(&line_raw) {
                                        let line = line_str.trim();
                                        if line.is_empty() { continue; }

                                        if line.starts_with("data: ") {
                                            let json_part = line.trim_start_matches("data: ").trim();
                                            if json_part == "[DONE]" {
                                                yield Ok::<Bytes, String>(Bytes::from("data: [DONE]\n\n"));
                                                continue;
                                            }

                                            match serde_json::from_str::<Value>(json_part) {
                                                Ok(mut json) => {
                                                    // Unwrap v1internal response wrapper
                                                    if let Some(inner) = json.get_mut("response").map(|v| v.take()) {
                                                        let new_line = format!("data: {}\n\n", serde_json::to_string(&inner).unwrap_or_default());
                                                        yield Ok::<Bytes, String>(Bytes::from(new_line));
                                                    } else {
                                                        yield Ok::<Bytes, String>(Bytes::from(format!("data: {}\n\n", serde_json::to_string(&json).unwrap_or_default())));
                                                    }
                                                }
                                                Err(e) => {
                                                    debug!("[Gemini-SSE] JSON parse error: {}, passing raw line", e);
                                                    yield Ok::<Bytes, String>(Bytes::from(format!("{}\n\n", line)));
                                                }
                                            }
                                        } else {
                                            // Non-data lines (comments, etc.)
                                            yield Ok::<Bytes, String>(Bytes::from(format!("{}\n\n", line)));
                                        }
                                    } else {
                                        // Non-UTF8 data? Just pass it through or skip
                                        debug!("[Gemini-SSE] Non-UTF8 line encountered");
                                        yield Ok::<Bytes, String>(line_raw.freeze());
                                    }
                                }
                            }
                            Err(e) => {
                                error!("[Gemini-SSE] Connection error: {}", e);
                                yield Err(format!("Stream error: {}", e));
                            }
                        }
                    }
                };

                let body = Body::from_stream(stream);
                return Ok(Response::builder()
                    .header("Content-Type", "text/event-stream")
                    .header("Cache-Control", "no-cache")
                    .header("Connection", "keep-alive")
                    .header("X-Account-Email", &email)
                    .header("X-Mapped-Model", &mapped_model)
                    .body(body)
                    .unwrap()
                    .into_response());
            }

            let gemini_resp: Value = response
                .json()
                .await
                .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Parse error: {}", e)))?;

            let unwrapped = unwrap_response(&gemini_resp);
            return Ok((
                StatusCode::OK,
                [
                    ("X-Account-Email", email.as_str()),
                    ("X-Mapped-Model", mapped_model.as_str()),
                ],
                Json(unwrapped),
            )
                .into_response());
        }

        // 处理错误并重试
        let status_code = status.as_u16();
        let retry_after = response
            .headers()
            .get("Retry-After")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| format!("HTTP {}", status_code));
        last_error = format!("HTTP {}: {}", status_code, error_text);

        // 只有 429 (限流), 529 (过载), 503, 403 (权限) 和 401 (认证失效) 触发账号轮换
        if status_code == 429
            || status_code == 529
            || status_code == 503
            || status_code == 500
            || status_code == 403
            || status_code == 401
        {
            // 记录限流信息 (全局同步) - 🆕 传递模型实现 model-level rate limiting
            token_manager.mark_rate_limited(
                &email,
                status_code,
                retry_after.as_deref(),
                &error_text,
                Some(&mapped_model),
            );

            // 只有明确包含 "QUOTA_EXHAUSTED" 才停止 -> 【Fix PR#493】改为继续尝试下一个账号
            if status_code == 429 && error_text.contains("QUOTA_EXHAUSTED") {
                error!("Gemini Quota exhausted (429) on account {} attempt {}/{}, will rotate to next account.", email, attempt + 1, max_attempts);
                // 标记该账号受限 (已经在上面的 mark_rate_limited 完成)
                // 继续循环以尝试下一个账号
                continue;
            }

            tracing::warn!(
                "Gemini Upstream {} on account {} attempt {}/{}, rotating account",
                status_code,
                email,
                attempt + 1,
                max_attempts
            );
            continue;
        }

        // 404 等由于模型配置或路径错误的 HTTP 异常，直接报错，不进行无效轮换
        error!(
            "Gemini Upstream non-retryable error {}: {}",
            status_code, error_text
        );
        return Err((status, error_text));
    }

    Ok((
        StatusCode::TOO_MANY_REQUESTS,
        format!("All accounts exhausted. Last error: {}", last_error),
    )
        .into_response())
}

pub async fn handle_list_models(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    use crate::proxy::common::model_mapping::get_all_dynamic_models;

    // 获取所有动态模型列表（与 /v1/models 一致）
    let model_ids = get_all_dynamic_models(&state.custom_mapping).await;

    // 转换为 Gemini API 格式
    let models: Vec<_> = model_ids
        .into_iter()
        .map(|id| {
            json!({
                "name": format!("models/{}", id),
                "version": "001",
                "displayName": id.clone(),
                "description": "",
                "inputTokenLimit": 128000,
                "outputTokenLimit": 8192,
                "supportedGenerationMethods": ["generateContent", "countTokens"],
                "temperature": 1.0,
                "topP": 0.95,
                "topK": 64
            })
        })
        .collect();

    Ok(Json(json!({ "models": models })))
}

pub async fn handle_get_model(Path(model_name): Path<String>) -> impl IntoResponse {
    Json(json!({
        "name": format!("models/{}", model_name),
        "displayName": model_name
    }))
}

pub async fn handle_count_tokens(
    State(state): State<AppState>,
    Path(_model_name): Path<String>,
    Json(_body): Json<Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let model_group = "gemini";
    // 🆕 count_tokens 工具不需要 model-specific rate limiting, 传递 None
    let (_access_token, _project_id, _) = state
        .token_manager
        .get_token(model_group, false, None, None)
        .await
        .map_err(|e| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Token error: {}", e),
            )
        })?;

    Ok(Json(json!({"totalTokens": 0})))
}
