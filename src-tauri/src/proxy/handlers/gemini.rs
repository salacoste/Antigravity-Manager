// Gemini Handler
use axum::{
    extract::State,
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{json, Value};
use tracing::{debug, error, info};

use crate::modules::budget_optimizer::BudgetOptimizer;
use crate::proxy::mappers::gemini::{unwrap_response, wrap_request};
use crate::proxy::server::AppState;
use crate::proxy::session_manager::SessionManager;
use once_cell::sync::Lazy;

const MAX_RETRY_ATTEMPTS: usize = 3;

// Epic-025 Story-025-01: Global budget optimizer instance
static BUDGET_OPTIMIZER: Lazy<BudgetOptimizer> = Lazy::new(BudgetOptimizer::new);

/// Epic-025: Check if model is Gemini 2.5 Flash Thinking (Model ID 313)
fn is_flash_thinking_model(model_id: &str) -> bool {
    model_id == "313"
        || model_id == "gemini-2.5-flash-thinking"
        || model_id.contains("flash-thinking")
}

/// Epic-025 Story-025-04: Extract response metadata for quality monitoring
fn extract_response_metadata(
    response_json: &Value,
    request_id: &str,
    model_id: &str,
    thinking_budget: u32,
) -> Option<(u32, u32, crate::modules::budget_detector::FinishReason)> {
    use crate::modules::budget_detector::FinishReason;

    // Extract from usageMetadata
    let usage = response_json.get("usageMetadata")?;

    let thinking_tokens = usage
        .get("cachedContentTokenCount")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    let output_tokens = usage
        .get("candidatesTokenCount")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    // Extract finish_reason from first candidate
    let finish_reason_str = response_json
        .get("candidates")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.first())
        .and_then(|c| c.get("finishReason"))
        .and_then(|fr| fr.as_str())
        .unwrap_or("UNSPECIFIED");

    let finish_reason = FinishReason::from_string(finish_reason_str);

    debug!(
        "[Epic-025] Response metadata extracted: request_id={}, model={}, thinking_tokens={}, output_tokens={}, finish_reason={:?}, thinking_budget={}",
        request_id, model_id, thinking_tokens, output_tokens, finish_reason, thinking_budget
    );

    Some((thinking_tokens, output_tokens, finish_reason))
}

/// Epic-025: Extract request text and messages from Gemini request body
fn extract_request_context(body: &Value) -> (String, Vec<Value>) {
    let mut request_text = String::new();
    let mut messages = Vec::new();

    // Extract from systemInstruction
    if let Some(sys_inst) = body.get("systemInstruction") {
        if let Some(parts) = sys_inst.get("parts").and_then(|p| p.as_array()) {
            for part in parts {
                if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                    request_text.push_str(text);
                    request_text.push(' ');
                }
            }
        }
    }

    // Extract from contents (conversation history)
    if let Some(contents) = body.get("contents").and_then(|c| c.as_array()) {
        for content in contents {
            messages.push(content.clone());

            // Extract text from latest user message for complexity analysis
            if let Some(parts) = content.get("parts").and_then(|p| p.as_array()) {
                for part in parts {
                    if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                        request_text.push_str(text);
                        request_text.push(' ');
                    }
                }
            }
        }
    }

    (request_text.trim().to_string(), messages)
}

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

    // Epic-025 Story-025-04: Generate request ID for quality tracking
    let request_id = uuid::Uuid::new_v4().to_string();

    crate::modules::logger::log_info(&format!(
        "Received Gemini request: {}/{} (request_id={})",
        model_name, method, request_id
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
    let mut last_email: Option<String> = None;

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

        last_email = Some(email.clone());
        info!("✓ Using account: {} (type: {})", email, config.request_type);

        // Epic-025 Story-025-01: Apply budget optimization for Flash Thinking (Model ID 313)
        // Epic-025 Story-025-04: Track applied budget for quality monitoring
        let mut optimized_body = body.clone();
        let mut applied_thinking_budget: u32 = 0;

        if is_flash_thinking_model(&mapped_model) {
            let (request_text, messages) = extract_request_context(&body);

            // Only optimize if we have meaningful content
            if !request_text.is_empty() {
                let allocation = BUDGET_OPTIMIZER
                    .allocate_budget(&request_text, &messages)
                    .await;

                applied_thinking_budget = allocation.budget;

                // Apply optimized budget to thinkingConfig
                if let Some(gen_config) = optimized_body.get_mut("generationConfig") {
                    if let Some(gen_obj) = gen_config.as_object_mut() {
                        let thinking_config =
                            gen_obj.entry("thinkingConfig").or_insert_with(|| json!({}));
                        if let Some(think_obj) = thinking_config.as_object_mut() {
                            think_obj
                                .insert("thinkingBudget".to_string(), json!(allocation.budget));
                        }
                    }
                } else {
                    // Create generationConfig if it doesn't exist
                    optimized_body["generationConfig"] = json!({
                        "thinkingConfig": {
                            "thinkingBudget": allocation.budget
                        }
                    });
                }

                info!(
                    "[Epic-025] Budget optimization applied: model={}, tier={}, budget={}, confidence={:.2}, factors={:?}",
                    mapped_model,
                    allocation.tier.as_str(),
                    allocation.budget,
                    allocation.confidence,
                    allocation.factors
                );
            } else {
                debug!("[Epic-025] Skipping budget optimization (no content extracted)");
            }
        }

        // 5. 包装请求 (project injection)
        let wrapped_body = wrap_request(&optimized_body, &project_id, &mapped_model);

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

            // Epic-025 Story-025-04: Quality monitoring for Model ID 313
            let mut quality_score_header = None;
            let mut ftr_header = None;

            if is_flash_thinking_model(&mapped_model) && applied_thinking_budget > 0 {
                // Extract response metadata
                if let Some((thinking_tokens, output_tokens, finish_reason)) =
                    extract_response_metadata(
                        &unwrapped,
                        &request_id,
                        &mapped_model,
                        applied_thinking_budget,
                    )
                {
                    // Clone for use in both async task and header
                    let finish_reason_clone = finish_reason.clone();

                    // Analyze quality (async non-blocking)
                    let quality_monitor = state.quality_monitor.clone();
                    let req_id = request_id.clone();

                    tokio::spawn(async move {
                        use std::time::Instant;
                        let start = Instant::now();

                        let analysis = quality_monitor
                            .analyze_quality(
                                req_id.clone(),
                                thinking_tokens,
                                output_tokens,
                                applied_thinking_budget,
                                finish_reason_clone,
                                0, // escalation_count = 0 for initial request
                            )
                            .await;

                        let elapsed = start.elapsed();

                        // Log if overhead exceeds target
                        if elapsed.as_millis() > 50 {
                            tracing::warn!(
                                "[Epic-025] Quality analysis overhead: {}ms (target: <50ms)",
                                elapsed.as_millis()
                            );
                        }

                        // Save to database (async, non-blocking)
                        if let Err(e) = crate::modules::proxy_db::save_quality_analysis(&analysis) {
                            tracing::error!("[Epic-025] Failed to save quality analysis: {}", e);
                        } else {
                            tracing::debug!(
                                "[Epic-025] Quality analysis saved: request_id={}, overall_score={:.2}, FTR={}",
                                req_id, analysis.overall_score, analysis.first_time_right
                            );
                        }
                    });

                    // Set response headers for immediate monitoring (synchronous - minimal overhead)
                    quality_score_header = Some(format!(
                        "{:.2}",
                        (thinking_tokens as f64 / applied_thinking_budget as f64) * 100.0
                    ));
                    ftr_header = Some(
                        (finish_reason == crate::modules::budget_detector::FinishReason::Stop)
                            .to_string(),
                    );
                }
            }

            // Build response with headers
            let mut response = (StatusCode::OK, Json(unwrapped)).into_response();

            // Add standard headers
            response
                .headers_mut()
                .insert("X-Account-Email", email.parse().unwrap());
            response
                .headers_mut()
                .insert("X-Mapped-Model", mapped_model.parse().unwrap());

            // Add quality headers if available
            if let Some(score) = quality_score_header {
                response
                    .headers_mut()
                    .insert("X-Quality-Budget-Utilization", score.parse().unwrap());
            }
            if let Some(ftr) = ftr_header {
                response
                    .headers_mut()
                    .insert("X-First-Time-Right", ftr.parse().unwrap());
            }

            return Ok(response);
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
        error!("Gemini Upstream non-retryable error {}: {}", status_code, error_text);
        return Ok((status, [("X-Account-Email", email.as_str())], error_text).into_response());
    }

    // Return final error with last account email if available
    if let Some(email) = last_email {
        Ok((StatusCode::TOO_MANY_REQUESTS, [("X-Account-Email", email)], format!("All accounts exhausted. Last error: {}", last_error)).into_response())
    } else {
        Ok((StatusCode::TOO_MANY_REQUESTS, format!("All accounts exhausted. Last error: {}", last_error)).into_response())
    }
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
