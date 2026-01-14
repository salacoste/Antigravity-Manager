use crate::models::Account;
use crate::modules::{account, config, logger, quota};
use chrono::Utc;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::Manager;
use tokio::time::{self, Duration};

// 预热历史记录：key = "email:model_name:100", value = 预热时间戳
static WARMUP_HISTORY: Lazy<Mutex<HashMap<String, i64>>> = Lazy::new(|| Mutex::new(load_warmup_history()));

fn get_warmup_history_path() -> Result<PathBuf, String> {
    let data_dir = account::get_data_dir()?;
    Ok(data_dir.join("warmup_history.json"))
}

fn load_warmup_history() -> HashMap<String, i64> {
    match get_warmup_history_path() {
        Ok(path) if path.exists() => {
            match std::fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => HashMap::new(),
            }
        }
        _ => HashMap::new(),
    }
}

fn save_warmup_history(history: &HashMap<String, i64>) {
    if let Ok(path) = get_warmup_history_path() {
        if let Ok(content) = serde_json::to_string_pretty(history) {
            let _ = std::fs::write(&path, content);
        }
    }
}

pub fn record_warmup_history(key: &str, timestamp: i64) {
    let mut history = WARMUP_HISTORY.lock().unwrap();
    history.insert(key.to_string(), timestamp);
    save_warmup_history(&history);
}

pub fn check_cooldown(key: &str, cooldown_seconds: i64) -> bool {
    let history = WARMUP_HISTORY.lock().unwrap();
    if let Some(&last_ts) = history.get(key) {
        let now = chrono::Utc::now().timestamp();
        now - last_ts < cooldown_seconds
    } else {
        false
    }
}

pub fn start_scheduler(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        logger::log_info("Smart Warmup Scheduler started. Monitoring quota at 100%...");

        // 每 10 分钟扫描一次
        let mut interval = time::interval(Duration::from_secs(600));

        loop {
            interval.tick().await;

            // 加载配置
            let Ok(app_config) = config::load_app_config() else {
                continue;
            };

            if !app_config.scheduled_warmup.enabled {
                continue;
            }

            // 获取所有账号（不再过滤等级）
            let Ok(accounts) = account::list_accounts() else {
                continue;
            };

            if accounts.is_empty() {
                continue;
            }

            logger::log_info(&format!(
                "[Scheduler] Scanning {} accounts for 100% quota models...",
                accounts.len()
            ));

            let mut warmup_tasks = Vec::new();
            let mut skipped_cooldown = 0;

            // 扫描每个账号的每个模型
            for account in &accounts {
                // 获取有效 token
                let Ok((token, pid)) = quota::get_valid_token_for_warmup(account).await else {
                    continue;
                };

                // 获取实时配额
                let Ok((fresh_quota, _)) =
                    quota::fetch_quota_with_cache(&token, &account.email, Some(&pid)).await
                else {
                    continue;
                };

                let now_ts = Utc::now().timestamp();

                for model in fresh_quota.models {
                    let history_key = format!("{}:{}:100", account.email, model.name);

                    // 核心逻辑：检测 100% 额度
                    if model.percentage == 100 {
                        // 模型名称映射（先映射再检查）
                        let model_to_ping = if model.name == "gemini-2.5-flash" {
                            "gemini-3-flash".to_string()
                        } else {
                            model.name.clone()
                        };

                        // 仅对用户配置的模型进行预热
                        if app_config
                            .scheduled_warmup
                            .monitored_models
                            .contains(&model_to_ping)
                        {
                            warmup_tasks.push((
                                account.email.clone(),
                                model_to_ping.clone(),
                                token.clone(),
                                pid.clone(),
                                model.percentage,
                            ));

                            logger::log_info(&format!(
                                "[Scheduler] ✓ Scheduled warmup: {} @ {} (quota at 100%)",
                                model_to_ping, account.email
                            ));
                        }

                        // 使用映射后的名字作为 key
                        let history_key = format!("{}:{}:100", account.email, model_to_ping);
                        
                        // 检查冷却期：4小时内不重复预热
                        {
                            let history = WARMUP_HISTORY.lock().unwrap();
                            if let Some(&last_warmup_ts) = history.get(&history_key) {
                                let cooldown_seconds = 14400;
                                if now_ts - last_warmup_ts < cooldown_seconds {
                                    skipped_cooldown += 1;
                                    continue;
                                }
                            }
                        }

                        warmup_tasks.push((
                            account.email.clone(),
                            model_to_ping.clone(),
                            token.clone(),
                            pid.clone(),
                            model.percentage,
                            history_key.clone(),
                        ));

                        logger::log_info(&format!(
                            "[Scheduler] ✓ Scheduled warmup: {} @ {} (quota at 100%)",
                            model_to_ping, account.email
                        ));
                    } else if model.percentage < 100 {
                        // 额度未满，清除历史记录，需要先映射名字
                        let model_to_ping = if model.name == "gemini-2.5-flash" {
                            "gemini-3-flash".to_string()
                        } else {
                            model.name.clone()
                        };
                        let history_key = format!("{}:{}:100", account.email, model_to_ping);
                        
                        let mut history = WARMUP_HISTORY.lock().unwrap();
                        if history.remove(&history_key).is_some() {
                            save_warmup_history(&history);
                            logger::log_info(&format!(
                                "[Scheduler] Cleared history for {} @ {} (quota: {}%)",
                                model_to_ping, account.email, model.percentage
                            ));
                        }
                    }
                }
            }

            // 执行预热任务
            if !warmup_tasks.is_empty() {
                let total = warmup_tasks.len();
                if skipped_cooldown > 0 {
                    logger::log_info(&format!(
                        "[Scheduler] 已跳过 {} 个冷却期内的模型，将预热 {} 个",
                        skipped_cooldown, total
                    ));
                }
                logger::log_info(&format!(
                    "[Scheduler] 🔥 Triggering {} warmup tasks...",
                    total
                ));

                let handle_for_warmup = app_handle.clone();
                tokio::spawn(async move {
                    let mut success = 0;
                    for (idx, (email, model, token, pid, pct)) in
                        warmup_tasks.into_iter().enumerate()
                    {
                        logger::log_info(&format!(
                            "[Warmup {}/{}] {} @ {} ({}%)",
                            idx + 1,
                            total,
                            model,
                            email,
                            pct
                        ));

                        if quota::warmup_model_directly(&token, &model, &pid, &email, pct).await {
                            success += 1;
                        }
                        
                        for handle in handles {
                            match handle.await {
                                Ok((true, history_key)) => {
                                    success += 1;
                                    record_warmup_history(&history_key, now_ts);
                                }
                                _ => {}
                            }
                        }
                        
                        if batch_idx < (warmup_tasks.len() + batch_size - 1) / batch_size - 1 {
                            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        }
                    }

                    logger::log_info(&format!(
                        "[Scheduler] ✅ Warmup completed: {}/{} successful",
                        success, total
                    ));

                    // 刷新配额，同步到前端
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    let state =
                        handle_for_warmup.state::<crate::commands::proxy::ProxyServiceState>();
                    let _ = crate::commands::refresh_all_quotas(state).await;
                });
            } else if skipped_cooldown > 0 {
                logger::log_info(&format!(
                    "[Scheduler] 扫描完成，所有100%模型均在冷却期内，已跳过 {} 个",
                    skipped_cooldown
                ));
            } else {
                logger::log_info("[Scheduler] 扫描完成，无100%额度的模型需要预热");
            }

            // 扫描完成后刷新前端显示（确保调度器获取的最新数据同步到 UI）
            let handle_inner = app_handle.clone();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                let state = handle_inner.state::<crate::commands::proxy::ProxyServiceState>();
                let _ = crate::commands::refresh_all_quotas(state).await;
                logger::log_info("[Scheduler] Quota data synced to frontend");
            });

            // 定期清理历史记录（保留最近 24 小时）
            {
                let now_ts = Utc::now().timestamp();
                let mut history = WARMUP_HISTORY.lock().unwrap();
                let cutoff = now_ts - 86400; // 24 小时前
                history.retain(|_, &mut ts| ts > cutoff);
            }
        }
    });
}

/// 为单个账号触发即时智能预热检查
#[allow(dead_code)]
pub async fn trigger_warmup_for_account(account: &Account) {
    // 获取有效 token
    let Ok((token, pid)) = quota::get_valid_token_for_warmup(account).await else {
        return;
    };

    // 获取配额信息 (优先从缓存读取，因为刷新命令通常刚更新完磁盘/缓存)
    let Ok((fresh_quota, _)) =
        quota::fetch_quota_with_cache(&token, &account.email, Some(&pid)).await
    else {
        return;
    };

    let now_ts = Utc::now().timestamp();
    let mut tasks_to_run = Vec::new();

    for model in fresh_quota.models {
        let history_key = format!("{}:{}:100", account.email, model.name);

        if model.percentage == 100 {
            // 检查历史，避免重复预热（带冷却期）
            {
                let mut history = WARMUP_HISTORY.lock().unwrap();
                
                // 4小时冷却期
                if let Some(&last_warmup_ts) = history.get(&history_key) {
                    let cooldown_seconds = 14400; // 4 小时（pro账号5h重置，留1h余量）
                    if now_ts - last_warmup_ts < cooldown_seconds {
                        // 仍在冷却期，跳过
                        continue;
                    }
                }
                
                history.insert(history_key, now_ts);
                save_warmup_history(&history);
            }

            let model_to_ping = if model.name == "gemini-2.5-flash" {
                "gemini-3-flash".to_string()
            } else {
                model.name.clone()
            };

            // 仅对用户勾选的模型进行预热
            let Ok(app_config) = config::load_app_config() else {
                continue;
            };

            if app_config
                .scheduled_warmup
                .monitored_models
                .contains(&model_to_ping)
            {
                tasks_to_run.push((model_to_ping, model.percentage));
            }
        } else if model.percentage < 100 {
            // 额度未满，清除历史，记录允许下次 100% 时再预热
            let mut history = WARMUP_HISTORY.lock().unwrap();
            history.remove(&history_key);
        }
    }

    // 执行预热
    if !tasks_to_run.is_empty() {
        for (model, pct) in tasks_to_run {
            logger::log_info(&format!(
                "[Scheduler] 🔥 Triggering individual warmup: {} @ {} (Sync)",
                model, account.email
            ));
            quota::warmup_model_directly(&token, &model, &pid, &account.email, pct).await;
        }
    }
}
