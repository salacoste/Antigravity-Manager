mod commands;
mod db; // Epic-014 Story-014-04: Audio analytics database
pub mod error;
pub mod models; // Public for integration testing (Story-024-02)
mod modules;
pub mod proxy; // 反代服务模块 (public for testing)
mod utils;

#[cfg(test)]
mod tests;

use modules::logger;
use tauri::Manager;
use tracing::{error, info};

// 测试命令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    logger::init_logger();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main").map(|window| {
                let _ = window.show();
                let _ = window.set_focus();
                #[cfg(target_os = "macos")]
                app.set_activation_policy(tauri::ActivationPolicy::Regular)
                    .unwrap_or(());
            });
        }))
        .manage(commands::proxy::ProxyServiceState::new())
        .manage(commands::budget::BudgetOptimizerState::new())
        .manage(commands::quality::QualityMonitorState::new())
        .manage({
            use crate::modules::quota_manager::QuotaManager;
            use std::sync::Arc;
            commands::quota_manager_commands::QuotaManagerState::new(Arc::new(QuotaManager::new(
                300,
            )))
        })
        .setup(|app| {
            info!("Setup starting...");
            modules::tray::create_tray(app.handle())?;
            info!("Tray created");

            // 自动启动反代服务
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // 加载配置
                if let Ok(config) = modules::config::load_app_config() {
                    if config.proxy.auto_start {
                        let state = handle.state::<commands::proxy::ProxyServiceState>();
                        // 尝试启动服务
                        if let Err(e) = commands::proxy::start_proxy_service(
                            config.proxy,
                            state,
                            handle.clone(),
                        )
                        .await
                        {
                            error!("自动启动反代服务失败: {}", e);
                        } else {
                            info!("反代服务自动启动成功");
                        }
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                #[cfg(target_os = "macos")]
                {
                    use tauri::Manager;
                    window
                        .app_handle()
                        .set_activation_policy(tauri::ActivationPolicy::Accessory)
                        .unwrap_or(());
                }
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // 账号管理命令
            commands::list_accounts,
            commands::add_account,
            commands::delete_account,
            commands::delete_accounts,
            commands::reorder_accounts,
            commands::switch_account,
            commands::get_current_account,
            // 配额命令
            commands::fetch_account_quota,
            commands::refresh_all_quotas,
            // Epic-001 Phase 3: QuotaManager Dashboard Commands (QUOTA-001-06)
            commands::quota_manager_commands::get_account_quotas,
            commands::quota_manager_commands::get_account_tier,
            commands::quota_manager_commands::get_quota_manager_stats,
            commands::quota_manager_commands::clear_tier_cache,
            // 配置命令
            commands::load_config,
            commands::save_config,
            // 新增命令
            commands::prepare_oauth_url,
            commands::start_oauth_login,
            commands::complete_oauth_login,
            commands::cancel_oauth_login,
            commands::import_v1_accounts,
            commands::import_from_db,
            commands::import_custom_db,
            commands::sync_account_from_db,
            commands::save_text_file,
            commands::clear_log_cache,
            commands::open_data_folder,
            commands::get_data_dir_path,
            commands::show_main_window,
            commands::get_antigravity_path,
            commands::get_antigravity_args,
            commands::check_for_updates,
            commands::toggle_proxy_status,
            // 反代服务命令
            commands::proxy::start_proxy_service,
            commands::proxy::stop_proxy_service,
            commands::proxy::get_proxy_status,
            commands::proxy::get_proxy_stats,
            commands::proxy::get_violation_metrics, // 🆕 Story #8
            commands::proxy::reset_violation_metrics, // 🆕 Story #12
            commands::proxy::get_cache_metrics,     // 🆕 Story-008-02
            commands::proxy::get_cache_hit_rate,    // 🆕 Story-008-02
            commands::proxy::get_top_cache_signatures, // 🆕 Story-008-02
            commands::proxy::get_cache_cost_savings, // 🆕 Story-008-02
            commands::proxy::clear_cache_metrics,   // 🆕 Story-008-02
            commands::proxy::get_analytics_report,  // 🆕 Story-013-06
            commands::proxy::get_cost_breakdown,    // 🆕 Story-013-06
            commands::proxy::reset_analytics,       // 🆕 Story-013-06
            commands::proxy::get_audio_analytics,   // 🆕 Epic-014 Story-014-04
            commands::proxy::get_proxy_logs,
            commands::proxy::set_proxy_monitor_enabled,
            commands::proxy::clear_proxy_logs,
            commands::proxy::generate_api_key,
            commands::proxy::reload_proxy_accounts,
            commands::proxy::update_model_mapping,
            commands::proxy::fetch_zai_models,
            commands::proxy::get_proxy_scheduling_config,
            commands::proxy::update_proxy_scheduling_config,
            commands::proxy::clear_proxy_session_bindings,
            // Autostart 命令
            commands::autostart::toggle_auto_launch,
            commands::autostart::is_auto_launch_enabled,
            // Detection monitoring 命令 (Story-024-04 Part 2)
            commands::detection::get_detection_statistics,
            commands::detection::get_recent_detection_events,
            commands::detection::clear_detection_events,
            // Budget optimization 命令 (Epic-025 Story-025-01)
            commands::budget::allocate_budget,
            commands::budget::get_budget_metrics,
            commands::budget::reset_budget_metrics,
            commands::budget::test_budget_allocation,
            // Epic-025 Story-025-04: Quality monitoring commands
            commands::quality::get_quality_metrics,
            commands::quality::get_weekly_feedback,
            commands::quality::get_quality_history,
            commands::quality::submit_user_rating,
            commands::quality::reset_quality_metrics,
            commands::quality::get_quality_history_with_trends,
            commands::quality::get_budget_distribution,
            // 测试命令
            commands::test_model_fallback_notification,
            commands::open_devtools,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            // Variables used on macOS only
            #[cfg(not(target_os = "macos"))]
            let _ = (&app_handle, &event);

            // Handle macOS dock icon click to reopen window
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen { .. } = event {
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                    app_handle
                        .set_activation_policy(tauri::ActivationPolicy::Regular)
                        .unwrap_or(());
                }
            }
        });
}
