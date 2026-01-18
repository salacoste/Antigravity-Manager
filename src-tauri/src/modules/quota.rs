use crate::models::QuotaData;
use crate::modules::config;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;

const QUOTA_API_URL: &str = "https://cloudcode-pa.googleapis.com/v1internal:fetchAvailableModels";
const USER_AGENT: &str = "antigravity/1.11.3 Darwin/arm64";

/// 临界值重试阈值：当配额达到 95% 时认为接近恢复
const NEAR_READY_THRESHOLD: i32 = 95;
const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_SECS: u64 = 30;

#[derive(Debug, Serialize, Deserialize)]
struct QuotaResponse {
    models: std::collections::HashMap<String, ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelInfo {
    #[serde(rename = "quotaInfo")]
    quota_info: Option<QuotaInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QuotaInfo {
    #[serde(rename = "remainingFraction")]
    remaining_fraction: Option<f64>,
    #[serde(rename = "resetTime")]
    reset_time: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LoadProjectResponse {
    #[serde(rename = "cloudaicompanionProject")]
    project_id: Option<String>,
    #[serde(rename = "currentTier")]
    current_tier: Option<Tier>,
    #[serde(rename = "paidTier")]
    paid_tier: Option<Tier>,
}

#[derive(Debug, Deserialize)]
struct Tier {
    id: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "quotaTier")]
    quota_tier: Option<String>,
    #[allow(dead_code)]
    name: Option<String>,
    #[allow(dead_code)]
    slug: Option<String>,
}

/// 创建配置好的 HTTP Client
fn create_client() -> reqwest::Client {
    crate::utils::http::create_client(15)
}

fn create_warmup_client() -> reqwest::Client {
    crate::utils::http::create_client(60) // 60 秒超时
}

const CLOUD_CODE_BASE_URL: &str = "https://cloudcode-pa.googleapis.com";

/// 获取项目 ID 和订阅类型
async fn fetch_project_id(access_token: &str, email: &str) -> (Option<String>, Option<String>) {
    let client = create_client();
    let meta = json!({"metadata": {"ideType": "ANTIGRAVITY"}});

    let res = client
        .post(format!("{}/v1internal:loadCodeAssist", CLOUD_CODE_BASE_URL))
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", access_token),
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::USER_AGENT, "antigravity/windows/amd64")
        .json(&meta)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                if let Ok(data) = res.json::<LoadProjectResponse>().await {
                    let project_id = data.project_id.clone();

                    // 核心逻辑：优先从 paid_tier 获取订阅 ID，这比 current_tier 更能反映真实账户权益
                    let subscription_tier = data
                        .paid_tier
                        .and_then(|t| t.id)
                        .or_else(|| data.current_tier.and_then(|t| t.id));

                    if let Some(ref tier) = subscription_tier {
                        crate::modules::logger::log_info(&format!(
                            "📊 [{}] 订阅识别成功: {}",
                            email, tier
                        ));
                    }

                    return (project_id, subscription_tier);
                }
            } else {
                crate::modules::logger::log_warn(&format!(
                    "⚠️  [{}] loadCodeAssist 失败: Status: {}",
                    email,
                    res.status()
                ));
            }
        }
        Err(e) => {
            crate::modules::logger::log_error(&format!(
                "❌ [{}] loadCodeAssist 网络错误: {}",
                email, e
            ));
        }
    }

    (None, None)
}

/// 查询账号配额的统一入口
pub async fn fetch_quota(
    access_token: &str,
    email: &str,
) -> crate::error::AppResult<(QuotaData, Option<String>)> {
    fetch_quota_with_cache(access_token, email, None).await
}

/// 带缓存的配额查询 (Story-027-01: Aggressive Context Scaling)
pub async fn fetch_quota_with_cache(
    access_token: &str,
    email: &str,
    cached_project_id: Option<&str>,
) -> crate::error::AppResult<(QuotaData, Option<String>)> {
    use crate::error::AppError;

    // 优化：如果有缓存的 project_id，跳过 loadCodeAssist 调用以节省 API 配额
    let (project_id, subscription_tier) = if let Some(pid) = cached_project_id {
        (Some(pid.to_string()), None)
    } else {
        fetch_project_id(access_token, email).await
    };

    let final_project_id = project_id.as_deref().unwrap_or("bamboo-precept-lgxtn");

    let client = create_client();
    let payload = json!({
        "project": final_project_id
    });

    let url = QUOTA_API_URL;
    let max_retries = 3;
    let mut last_error: Option<AppError> = None;

    for attempt in 1..=max_retries {
        match client
            .post(url)
            .bearer_auth(access_token)
            .header("User-Agent", USER_AGENT)
            .json(&json!(payload))
            .send()
            .await
        {
            Ok(response) => {
                // 将 HTTP 错误状态转换为 AppError
                if response.error_for_status_ref().is_err() {
                    let status = response.status();

                    // ✅ 特殊处理 403 Forbidden - 直接返回,不重试
                    if status == reqwest::StatusCode::FORBIDDEN {
                        crate::modules::logger::log_warn(
                            "账号无权限 (403 Forbidden),标记为 forbidden 状态",
                        );
                        let mut q = QuotaData::new();
                        q.is_forbidden = true;
                        q.subscription_tier = subscription_tier.clone();
                        return Ok((q, project_id.clone()));
                    }

                    // 其他错误继续重试逻辑
                    if attempt < max_retries {
                        let text = response.text().await.unwrap_or_default();
                        crate::modules::logger::log_warn(&format!(
                            "API 错误: {} - {} (尝试 {}/{})",
                            status, text, attempt, max_retries
                        ));
                        last_error = Some(AppError::Unknown(format!("HTTP {} - {}", status, text)));
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        continue;
                    } else {
                        let text = response.text().await.unwrap_or_default();
                        return Err(AppError::Unknown(format!(
                            "API 错误: {} - {}",
                            status, text
                        )));
                    }
                }

                let quota_response: QuotaResponse =
                    response.json().await.map_err(AppError::Network)?;

                let mut quota_data = QuotaData::new();

                // 使用 debug 级别记录详细信息，避免控制台噪音
                tracing::debug!("Quota API 返回了 {} 个模型", quota_response.models.len());

                for (name, info) in quota_response.models {
                    if let Some(quota_info) = info.quota_info {
                        let percentage = quota_info
                            .remaining_fraction
                            .map_or(0, |f| (f * 100.0) as i32);

                        let reset_time = quota_info.reset_time.unwrap_or_default();

                        // 只保存我们关心的模型
                        if name.contains("gemini") || name.contains("claude") {
                            quota_data.add_model(name, percentage, reset_time);
                        }
                    }
                }

                // 设置订阅类型
                quota_data.subscription_tier = subscription_tier.clone();

                return Ok((quota_data, project_id.clone()));
            }
            Err(e) => {
                crate::modules::logger::log_warn(&format!(
                    "请求失败: {} (尝试 {}/{})",
                    e, attempt, max_retries
                ));
                last_error = Some(AppError::Network(e));
                if attempt < max_retries {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| AppError::Unknown("配额查询失败".to_string())))
}

/// 查询账号配额逻辑
#[allow(dead_code)] // Internal function for quota fetching
pub async fn fetch_quota_inner(
    access_token: &str,
    email: &str,
) -> crate::error::AppResult<(QuotaData, Option<String>)> {
    fetch_quota_with_cache(access_token, email, None).await
}

/// 批量查询所有账号配额 (备用功能)
#[allow(dead_code)]
pub async fn fetch_all_quotas(
    accounts: Vec<(String, String)>,
) -> Vec<(String, crate::error::AppResult<QuotaData>)> {
    let mut results = Vec::new();

    for (account_id, access_token) in accounts {
        // 在批量查询中，我们将 account_id 传入以供日志标识
        let result = fetch_quota(&access_token, &account_id)
            .await
            .map(|(q, _)| q);
        results.push((account_id, result));
    }

    results
}

/// 获取有效 token（自动刷新过期的）
pub async fn get_valid_token_for_warmup(
    account: &crate::models::account::Account,
) -> Result<(String, String), String> {
    let mut account = account.clone();

    // 检查并自动刷新 token
    let new_token = crate::modules::oauth::ensure_fresh_token(&account.token).await?;

    // 如果 token 改变了（意味着刷新了），保存它
    if new_token.access_token != account.token.access_token {
        account.token = new_token;
        if let Err(e) = crate::modules::account::save_account(&account) {
            crate::modules::logger::log_warn(&format!("[Warmup] 保存刷新后的 Token 失败: {}", e));
        } else {
            crate::modules::logger::log_info(&format!(
                "[Warmup] 成功为 {} 刷新并保存了新 Token",
                account.email
            ));
        }
    }

    // 获取 project_id
    let (project_id, _) = fetch_project_id(&account.token.access_token, &account.email).await;
    let final_pid = project_id.unwrap_or_else(|| "bamboo-precept-lgxtn".to_string());

    Ok((account.token.access_token, final_pid))
}

/// 通过代理内部 API 发送预热请求
pub async fn warmup_model_directly(
    access_token: &str,
    model_name: &str,
    project_id: &str,
    email: &str,
    percentage: i32,
) -> bool {
    // 获取当前配置的代理端口
    let port = config::load_app_config()
        .map(|c| c.proxy.port)
        .unwrap_or(8045);

    let warmup_url = format!("http://127.0.0.1:{}/internal/warmup", port);
    let body = json!({
        "email": email,
        "model": model_name,
        "access_token": access_token,
        "project_id": project_id
    });

    let client = create_warmup_client();
    let resp = client
        .post(&warmup_url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match resp {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                crate::modules::logger::log_info(&format!(
                    "[Warmup] ✓ Triggered {} for {} (was {}%)",
                    model_name, email, percentage
                ));
                true
            } else {
                let text = response.text().await.unwrap_or_default();
                crate::modules::logger::log_warn(&format!(
                    "[Warmup] ✗ {} for {} (was {}%): HTTP {} - {}",
                    model_name, email, percentage, status, text
                ));
                false
            }
        }
        Err(e) => {
            crate::modules::logger::log_warn(&format!(
                "[Warmup] ✗ {} for {} (was {}%): {}",
                model_name, email, percentage, e
            ));
            false
        }
    }
}

/// 智能预热所有账号
pub async fn warm_up_all_accounts() -> Result<String, String> {
    let mut retry_count = 0;

    loop {
        let target_accounts = crate::modules::account::list_accounts().unwrap_or_default();

        if target_accounts.is_empty() {
            return Ok("没有可用账号".to_string());
        }

        crate::modules::logger::log_info(&format!(
            "[Warmup] 开始筛选 {} 个账号的模型...",
            target_accounts.len()
        ));

        let mut warmup_items = Vec::new();
        let mut has_near_ready_models = false;

        for account in &target_accounts {
            let (token, pid) = match get_valid_token_for_warmup(account).await {
                Ok(t) => t,
                Err(e) => {
                    crate::modules::logger::log_warn(&format!(
                        "[Warmup] 账号 {} 准备失败: {}",
                        account.email, e
                    ));
                    continue;
                }
            };

            // 获取最新实时配额
            if let Ok((fresh_quota, _)) =
                fetch_quota_with_cache(&token, &account.email, Some(&pid)).await
            {
                let mut account_warmed_series = std::collections::HashSet::new();
                for m in fresh_quota.models {
                    if m.percentage >= 100 {
                        // 1. 映射逻辑
                        let model_to_ping = if m.name == "gemini-2.5-flash" {
                            "gemini-3-flash".to_string()
                        } else {
                            m.name.clone()
                        };

                        // 2. 严格白名单过滤
                        match model_to_ping.as_str() {
                            "gemini-3-flash" | "claude-sonnet-4-5" | "gemini-3-pro-high"
                            | "gemini-3-pro-image" => {
                                if !account_warmed_series.contains(&model_to_ping) {
                                    warmup_items.push((
                                        account.email.clone(),
                                        model_to_ping.clone(),
                                        token.clone(),
                                        pid.clone(),
                                        m.percentage,
                                    ));
                                    account_warmed_series.insert(model_to_ping);
                                }
                            }
                            _ => {}
                        }
                    } else if m.percentage >= NEAR_READY_THRESHOLD {
                        has_near_ready_models = true;
                    }
                }
            }
        }

        if !warmup_items.is_empty() {
            let total_before = warmup_items.len();

            // 过滤掉4小时内已预热的模型
            warmup_items.retain(|(email, model, _, _, _)| {
                let history_key = format!("{}:{}:100", email, model);
                !crate::modules::scheduler::check_cooldown(&history_key, 14400)
            });

            if warmup_items.is_empty() {
                let skipped = total_before;
                crate::modules::logger::log_info(&format!(
                    "[Warmup] 返回前端: 所有模型均在冷却期内，跳过 {} 个",
                    skipped
                ));
                return Ok(format!("所有模型均在4小时冷却期内，已跳过 {} 个", skipped));
            }

            let total = warmup_items.len();
            let skipped = total_before - total;

            if skipped > 0 {
                crate::modules::logger::log_info(&format!(
                    "[Warmup] 已跳过 {} 个冷却期内的模型，将预热 {} 个",
                    skipped, total
                ));
            }

            crate::modules::logger::log_info(&format!(
                "[Warmup] 🔥 启动手动预热: {} 个模型",
                total
            ));

            tokio::spawn(async move {
                let mut success = 0;
                let batch_size = 3;
                let now_ts = chrono::Utc::now().timestamp();

                for (batch_idx, batch) in warmup_items.chunks(batch_size).enumerate() {
                    let mut handles = Vec::new();

                    for (email, model, token, pid, pct) in batch.iter() {
                        let email = email.clone();
                        let model = model.clone();
                        let token = token.clone();
                        let pid = pid.clone();
                        let pct = *pct;

                        let handle = tokio::spawn(async move {
                            let result =
                                warmup_model_directly(&token, &model, &pid, &email, pct).await;
                            (result, email, model)
                        });
                        handles.push(handle);
                    }

                    for handle in handles {
                        #[allow(clippy::single_match)]
                        match handle.await {
                            Ok((true, email, model)) => {
                                success += 1;
                                let history_key = format!("{}:{}:100", email, model);
                                crate::modules::scheduler::record_warmup_history(
                                    &history_key,
                                    now_ts,
                                );
                            }
                            _ => {}
                        }
                    }

                    if batch_idx < warmup_items.len().div_ceil(batch_size) - 1 {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    }
                }
                crate::modules::logger::log_info(&format!(
                    "[Warmup] 预热任务完成: 成功 {}/{}",
                    success, total
                ));
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                let _ = crate::modules::account::refresh_all_quotas_logic().await;
            });
            crate::modules::logger::log_info(&format!(
                "[Warmup] 返回前端: 已启动 {} 个模型的预热任务",
                total
            ));
            return Ok(format!("已启动 {} 个模型的预热任务", total));
        }

        if has_near_ready_models && retry_count < MAX_RETRIES {
            retry_count += 1;
            crate::modules::logger::log_info(&format!(
                "[Warmup] 检测到临界恢复模型，等待 {}s 后重试 ({}/{})",
                RETRY_DELAY_SECS, retry_count, MAX_RETRIES
            ));
            tokio::time::sleep(tokio::time::Duration::from_secs(RETRY_DELAY_SECS)).await;
            continue;
        }

        return Ok("没有模型需要预热".to_string());
    }
}

/// 单账号预热
pub async fn warm_up_account(account_id: &str) -> Result<String, String> {
    let accounts = crate::modules::account::list_accounts().unwrap_or_default();
    let account_owned = accounts
        .iter()
        .find(|a| a.id == account_id)
        .cloned()
        .ok_or_else(|| "账号未找到".to_string())?;

    let email = account_owned.email.clone();
    let (token, pid) = get_valid_token_for_warmup(&account_owned).await?;
    let (fresh_quota, _) = fetch_quota_with_cache(&token, &email, Some(&pid))
        .await
        .map_err(|e| format!("查询配额失败: {}", e))?;

    let mut models_to_warm = Vec::new();
    let mut warmed_series = std::collections::HashSet::new();

    for m in fresh_quota.models {
        if m.percentage >= 100 {
            // 1. 映射逻辑
            let model_name = if m.name == "gemini-2.5-flash" {
                "gemini-3-flash".to_string()
            } else {
                m.name.clone()
            };

            // 2. 严格白名单过滤
            match model_name.as_str() {
                "gemini-3-flash" | "claude-sonnet-4-5" | "gemini-3-pro-high"
                | "gemini-3-pro-image" => {
                    if !warmed_series.contains(&model_name) {
                        models_to_warm.push((model_name.clone(), m.percentage));
                        warmed_series.insert(model_name);
                    }
                }
                _ => continue,
            }
        }
    }

    if models_to_warm.is_empty() {
        return Ok("无需预热".to_string());
    }

    let warmed_count = models_to_warm.len();

    tokio::spawn(async move {
        for (name, pct) in models_to_warm {
            if warmup_model_directly(&token, &name, &pid, &email, pct).await {
                let history_key = format!("{}:{}:100", email, name);
                let now_ts = chrono::Utc::now().timestamp();
                crate::modules::scheduler::record_warmup_history(&history_key, now_ts);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        let _ = crate::modules::account::refresh_all_quotas_logic().await;
    });

    Ok(format!("成功触发 {} 个系列的模型预热", warmed_count))
}
