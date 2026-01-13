#[cfg(test)]
mod tests {
    use super::super::quota_monitor::*;
    use crate::models::{Account, QuotaData, TokenData};
    use chrono::Utc;
    use std::fs;
    use tempfile::TempDir;

    // Test helper: Create a test account with quota
    fn create_test_account(id: &str, email: &str, quota_percentage: i32) -> Account {
        let mut quota = QuotaData::new();
        quota.add_model(
            "gemini-2.0-flash-exp".to_string(),
            quota_percentage,
            Utc::now().to_rfc3339(),
        );

        Account {
            id: id.to_string(),
            email: email.to_string(),
            name: Some(format!("Test User {}", id)),
            quota: Some(quota),
            token: TokenData::new(
                "test_access_token".to_string(),
                "test_refresh_token".to_string(),
                3600,
                Some(email.to_string()),
                None,
                None,
            ),
            disabled: false,
            is_current: false,
            created_at: 0,
            updated_at: 0,
            avatar_url: None,
            subscription_tier: None,
            proxy_disabled: None,
            proxy_disabled_at: None,
            proxy_disabled_reason: None,
            order: 0,
        }
    }

    // Test helper: Setup temp database for testing
    fn setup_test_db() -> (TempDir, QuotaHistoryStore) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_quota.db");

        // Mock the proxy_db path
        std::env::set_var("TEST_QUOTA_DB_PATH", db_path.to_str().unwrap());

        let store = QuotaHistoryStore {
            db_path: db_path.clone(),
        };

        store.init_tables().unwrap();
        (temp_dir, store)
    }

    #[test]
    fn test_quota_alert_creation() {
        let alert = QuotaAlert::new(
            "acc123".to_string(),
            "test@example.com".to_string(),
            0.8,
            75,
            Some("gemini-2.0-flash-exp".to_string()),
        );

        assert_eq!(alert.account_id, "acc123");
        assert_eq!(alert.account_email, "test@example.com");
        assert_eq!(alert.threshold, 0.8);
        assert_eq!(alert.quota_percentage, 75);
        assert!(alert.model_name.is_some());
        assert!(alert.resolved_at.is_none());
        assert!(alert.mitigation_action.is_none());
    }

    #[test]
    fn test_quota_status_enum() {
        // Test enum variants
        let healthy = QuotaStatus::Healthy;
        let warning = QuotaStatus::Warning(0.8);
        let critical = QuotaStatus::Critical(0.95);
        let exhausted = QuotaStatus::Exhausted;
        let forbidden = QuotaStatus::Forbidden;

        assert_eq!(healthy, QuotaStatus::Healthy);
        assert_eq!(warning, QuotaStatus::Warning(0.8));
        assert_eq!(critical, QuotaStatus::Critical(0.95));
        assert_eq!(exhausted, QuotaStatus::Exhausted);
        assert_eq!(forbidden, QuotaStatus::Forbidden);
    }

    #[test]
    fn test_mitigation_action_types() {
        let switch = MitigationAction::AccountSwitch {
            from: "acc1".to_string(),
            to: "acc2".to_string(),
        };

        let rate_limit = MitigationAction::RateLimitApplied {
            account_id: "acc1".to_string(),
            limit_ms: 5000,
        };

        let alert_only = MitigationAction::AlertOnly;
        let no_action = MitigationAction::NoActionNeeded;

        // Verify serialization/deserialization
        let json = serde_json::to_string(&switch).unwrap();
        assert!(json.contains("AccountSwitch"));

        let json = serde_json::to_string(&rate_limit).unwrap();
        assert!(json.contains("RateLimitApplied"));

        let json = serde_json::to_string(&alert_only).unwrap();
        assert!(json.contains("AlertOnly"));

        let json = serde_json::to_string(&no_action).unwrap();
        assert!(json.contains("NoActionNeeded"));
    }

    #[test]
    fn test_quota_history_store_init() {
        let (_temp_dir, store) = setup_test_db();

        // Verify tables were created
        let conn = rusqlite::Connection::open(&store.db_path).unwrap();

        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='quota_history'")
            .unwrap();
        let table_exists: bool = stmt.exists([]).unwrap();
        assert!(table_exists);

        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='quota_alerts'")
            .unwrap();
        let table_exists: bool = stmt.exists([]).unwrap();
        assert!(table_exists);
    }

    #[test]
    fn test_quota_history_record() {
        let (_temp_dir, store) = setup_test_db();

        let account = create_test_account("acc1", "test@example.com", 85);
        let quota = account.quota.unwrap();

        // Record quota
        let result = store.record_quota(&account.id, &quota);
        assert!(result.is_ok());

        // Retrieve history
        let history = store.get_history(&account.id, 10).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].account_id, "acc1");
        assert_eq!(history[0].percentage, 85);
    }

    #[test]
    fn test_alert_save_and_retrieve() {
        let (_temp_dir, store) = setup_test_db();

        let alert = QuotaAlert::new(
            "acc1".to_string(),
            "test@example.com".to_string(),
            0.9,
            88,
            Some("gemini-2.0-flash-exp".to_string()),
        );

        // Save alert
        let alert_id = store.save_alert(&alert).unwrap();
        assert!(alert_id > 0);

        // Retrieve active alerts
        let alerts = store.get_active_alerts("acc1").unwrap();
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].account_id, "acc1");
        assert_eq!(alerts[0].threshold, 0.9);
        assert!(alerts[0].resolved_at.is_none());
    }

    #[test]
    fn test_alert_resolution() {
        let (_temp_dir, store) = setup_test_db();

        let alert = QuotaAlert::new(
            "acc1".to_string(),
            "test@example.com".to_string(),
            0.9,
            88,
            None,
        );

        // Save alert
        let alert_id = store.save_alert(&alert).unwrap();

        // Resolve alert
        let result = store.resolve_alert(alert_id);
        assert!(result.is_ok());

        // Verify alert is no longer active
        let active_alerts = store.get_active_alerts("acc1").unwrap();
        assert_eq!(active_alerts.len(), 0);
    }

    #[tokio::test]
    async fn test_alert_manager_trigger() {
        let (_temp_dir, _store) = setup_test_db();

        let alert_manager = AlertManager::new(true).unwrap();

        let alert = QuotaAlert::new(
            "acc1".to_string(),
            "test@example.com".to_string(),
            0.8,
            75,
            Some("gemini-2.0-flash-exp".to_string()),
        );

        let result = alert_manager.trigger_alert(alert).await;
        assert!(result.is_ok());

        let active_alerts = alert_manager.get_active_alerts("acc1").await.unwrap();
        assert_eq!(active_alerts.len(), 1);
    }

    #[tokio::test]
    async fn test_alert_manager_should_mitigate() {
        let (_temp_dir, _store) = setup_test_db();

        let alert_manager = AlertManager::new(true).unwrap();

        // Should mitigate when enabled and below threshold
        assert!(alert_manager.should_mitigate(75, 0.8));
        assert!(alert_manager.should_mitigate(88, 0.9));
        assert!(!alert_manager.should_mitigate(85, 0.8));

        // Should not mitigate when disabled
        let alert_manager_disabled = AlertManager::new(false).unwrap();
        assert!(!alert_manager_disabled.should_mitigate(75, 0.8));
    }

    #[tokio::test]
    async fn test_quota_monitor_initialization() {
        let monitor = QuotaMonitor::new();
        assert!(monitor.is_ok());

        let monitor = monitor.unwrap();
        assert_eq!(monitor.alert_thresholds, vec![0.8, 0.9, 0.95]);
    }

    #[tokio::test]
    async fn test_quota_monitor_custom_thresholds() {
        let thresholds = vec![0.5, 0.75, 0.9];
        let monitor = QuotaMonitor::with_thresholds(thresholds.clone(), true);
        assert!(monitor.is_ok());

        let monitor = monitor.unwrap();
        assert_eq!(monitor.alert_thresholds, vec![0.5, 0.75, 0.9]);
    }

    #[tokio::test]
    async fn test_quota_monitor_update_quota() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();
        let account = create_test_account("acc1", "test@example.com", 85);

        let result = monitor.update_quota(&account).await;
        assert!(result.is_ok());

        let monitored = monitor.get_monitored_accounts().await;
        assert_eq!(monitored.len(), 1);
        assert_eq!(monitored[0], "acc1");
    }

    #[tokio::test]
    async fn test_quota_monitor_check_quota_healthy() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();
        let account = create_test_account("acc1", "test@example.com", 85);

        monitor.update_quota(&account).await.unwrap();

        let status = monitor.check_quota("acc1").await.unwrap();
        assert_eq!(status, QuotaStatus::Healthy);
    }

    #[tokio::test]
    async fn test_quota_monitor_check_quota_warning() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();
        let account = create_test_account("acc1", "test@example.com", 75); // Below 80% threshold

        monitor.update_quota(&account).await.unwrap();

        let status = monitor.check_quota("acc1").await.unwrap();
        match status {
            QuotaStatus::Warning(threshold) => assert_eq!(threshold, 0.8),
            _ => panic!("Expected Warning status"),
        }
    }

    #[tokio::test]
    async fn test_quota_monitor_check_quota_critical() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();
        let account = create_test_account("acc1", "test@example.com", 5); // Below 95% threshold

        monitor.update_quota(&account).await.unwrap();

        let status = monitor.check_quota("acc1").await.unwrap();
        match status {
            QuotaStatus::Critical(threshold) => assert_eq!(threshold, 0.95),
            _ => panic!("Expected Critical status"),
        }
    }

    #[tokio::test]
    async fn test_quota_monitor_check_quota_exhausted() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();
        let account = create_test_account("acc1", "test@example.com", 0); // 0% remaining

        monitor.update_quota(&account).await.unwrap();

        let status = monitor.check_quota("acc1").await.unwrap();
        assert_eq!(status, QuotaStatus::Exhausted);
    }

    #[tokio::test]
    async fn test_quota_monitor_check_quota_forbidden() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();
        let mut account = create_test_account("acc1", "test@example.com", 50);
        account.quota.as_mut().unwrap().is_forbidden = true;

        monitor.update_quota(&account).await.unwrap();

        let status = monitor.check_quota("acc1").await.unwrap();
        assert_eq!(status, QuotaStatus::Forbidden);
    }

    #[tokio::test]
    async fn test_quota_monitor_auto_mitigate() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();

        // Add low quota account
        let low_quota_account = create_test_account("acc1", "test1@example.com", 10);
        monitor.update_quota(&low_quota_account).await.unwrap();

        // Add healthy account
        let healthy_account = create_test_account("acc2", "test2@example.com", 90);
        monitor.update_quota(&healthy_account).await.unwrap();

        // Request mitigation for low quota account
        let action = monitor.auto_mitigate("acc1").await.unwrap();

        match action {
            MitigationAction::AccountSwitch { from, to } => {
                assert_eq!(from, "acc1");
                assert_eq!(to, "acc2");
            }
            _ => panic!("Expected AccountSwitch action"),
        }
    }

    #[tokio::test]
    async fn test_quota_monitor_auto_mitigate_no_alternative() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();

        // Add only low quota account
        let low_quota_account = create_test_account("acc1", "test1@example.com", 10);
        monitor.update_quota(&low_quota_account).await.unwrap();

        // Request mitigation with no healthy alternatives
        let action = monitor.auto_mitigate("acc1").await.unwrap();

        match action {
            MitigationAction::AlertOnly => {}
            _ => panic!("Expected AlertOnly action"),
        }
    }

    #[tokio::test]
    async fn test_quota_monitor_alert_triggering() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();

        // Add account with quota below threshold
        let account = create_test_account("acc1", "test@example.com", 75);
        monitor.update_quota(&account).await.unwrap();

        // Check alerts were triggered
        let alerts = monitor.get_active_alerts("acc1").await.unwrap();
        assert!(alerts.len() > 0);
        assert_eq!(alerts[0].account_id, "acc1");
    }

    #[tokio::test]
    async fn test_quota_monitor_metrics() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();

        // Test alert accuracy metric
        let accuracy = monitor.get_alert_accuracy().await.unwrap();
        assert!(accuracy > 0.95); // >95% target

        // Test mitigation success rate metric
        let success_rate = monitor.get_mitigation_success_rate().await.unwrap();
        assert!(success_rate > 0.90); // >90% target
    }

    #[tokio::test]
    async fn test_quota_monitor_history_retrieval() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();

        // Add account and record multiple quota snapshots
        let account = create_test_account("acc1", "test@example.com", 90);
        monitor.update_quota(&account).await.unwrap();

        let account2 = create_test_account("acc1", "test@example.com", 85);
        monitor.update_quota(&account2).await.unwrap();

        // Retrieve history
        let history = monitor.get_history("acc1", 10).await.unwrap();
        assert!(history.len() >= 2);
    }

    #[test]
    fn test_quota_history_record_serialization() {
        let record = QuotaHistoryRecord {
            id: Some(1),
            account_id: "acc1".to_string(),
            timestamp: Utc::now(),
            tokens_used: 150,
            tokens_remaining: 850,
            percentage: 85,
            reset_time: Some(Utc::now()),
            model_name: Some("gemini-2.0-flash-exp".to_string()),
        };

        // Test serialization
        let json = serde_json::to_string(&record).unwrap();
        assert!(json.contains("acc1"));
        assert!(json.contains("\"percentage\":85"));

        // Test deserialization
        let deserialized: QuotaHistoryRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.account_id, "acc1");
        assert_eq!(deserialized.percentage, 85);
    }

    #[test]
    fn test_threshold_sorting() {
        // Test that custom thresholds are sorted
        let unsorted_thresholds = vec![0.95, 0.75, 0.5, 0.9];
        let monitor = QuotaMonitor::with_thresholds(unsorted_thresholds, true).unwrap();

        assert_eq!(monitor.alert_thresholds, vec![0.5, 0.75, 0.9, 0.95]);
    }

    #[tokio::test]
    async fn test_quota_monitor_multiple_accounts() {
        let (_temp_dir, _store) = setup_test_db();

        let monitor = QuotaMonitor::new().unwrap();

        // Add multiple accounts with different quota levels
        let accounts = vec![
            create_test_account("acc1", "test1@example.com", 95), // Healthy
            create_test_account("acc2", "test2@example.com", 75), // Warning
            create_test_account("acc3", "test3@example.com", 5),  // Critical
            create_test_account("acc4", "test4@example.com", 0),  // Exhausted
        ];

        for account in accounts {
            monitor.update_quota(&account).await.unwrap();
        }

        let monitored = monitor.get_monitored_accounts().await;
        assert_eq!(monitored.len(), 4);

        // Verify different statuses
        assert_eq!(
            monitor.check_quota("acc1").await.unwrap(),
            QuotaStatus::Healthy
        );
        assert!(matches!(
            monitor.check_quota("acc2").await.unwrap(),
            QuotaStatus::Warning(_)
        ));
        assert!(matches!(
            monitor.check_quota("acc3").await.unwrap(),
            QuotaStatus::Critical(_)
        ));
        assert_eq!(
            monitor.check_quota("acc4").await.unwrap(),
            QuotaStatus::Exhausted
        );
    }
}
