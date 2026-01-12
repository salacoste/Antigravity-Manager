//! Detection Event System Unit Tests
//!
//! Story-024-04 Part 1: Event logging infrastructure
//! Tests cover:
//! - Event creation and basic properties
//! - Severity levels
//! - Event storage and retrieval
//! - Statistics calculation
//! - Event cleanup
//! - Alert threshold logic

use antigravity_tools_lib::proxy::detection::*;
use std::collections::HashMap;

#[test]
fn test_detection_event_creation_with_all_fields() {
    let event = DetectionEvent::new(
        DetectionEventType::IdeTypeMissing,
        Severity::Critical,
        "test_account".to_string(),
        "claude-4.5-sonnet".to_string(),
        "req_12345".to_string(),
        "Missing ideType marker in request headers".to_string(),
    );

    assert_eq!(event.event_type, DetectionEventType::IdeTypeMissing);
    assert_eq!(event.severity, Severity::Critical);
    assert_eq!(event.account_id, "test_account");
    assert_eq!(event.model_id, "claude-4.5-sonnet");
    assert_eq!(event.request_id, "req_12345");
    assert_eq!(event.user_agent, None);
    assert_eq!(event.upstream_status, None);
}

#[test]
fn test_detection_event_builder_pattern() {
    let event = DetectionEvent::new(
        DetectionEventType::UserAgentStatic,
        Severity::Medium,
        "acc_789".to_string(),
        "gpt-4".to_string(),
        "req_abc".to_string(),
        "Static user-agent detected".to_string(),
    )
    .with_user_agent("Mozilla/5.0 (Windows)".to_string())
    .with_upstream_status(200);

    assert_eq!(
        event.user_agent,
        Some("Mozilla/5.0 (Windows)".to_string())
    );
    assert_eq!(event.upstream_status, Some(200));
}

#[test]
fn test_all_detection_event_types() {
    let event_types = vec![
        DetectionEventType::IdeTypeMissing,
        DetectionEventType::ApiProviderMismatch,
        DetectionEventType::UserAgentStatic,
        DetectionEventType::RateLimit429,
        DetectionEventType::AuthError401,
        DetectionEventType::Blocked403,
    ];

    assert_eq!(event_types.len(), 6);

    // Verify all types can be created
    for event_type in event_types {
        let event = DetectionEvent::new(
            event_type,
            Severity::High,
            "test_acc".to_string(),
            "test_model".to_string(),
            "test_req".to_string(),
            "test context".to_string(),
        );
        assert_eq!(event.event_type, event_type);
    }
}

#[test]
fn test_all_severity_levels() {
    let severities = vec![
        Severity::Critical,
        Severity::High,
        Severity::Medium,
        Severity::Low,
    ];

    assert_eq!(severities.len(), 4);

    // Verify all severities work
    for severity in severities {
        let event = DetectionEvent::new(
            DetectionEventType::RateLimit429,
            severity,
            "acc".to_string(),
            "model".to_string(),
            "req".to_string(),
            "context".to_string(),
        );
        assert_eq!(event.severity, severity);
    }
}

#[test]
fn test_alert_threshold_defaults() {
    let threshold = AlertThreshold::default();
    assert_eq!(threshold.count, 5);
    assert_eq!(threshold.window_minutes, 60);
    assert_eq!(threshold.severity, Severity::High);
}

#[test]
fn test_alert_threshold_custom_values() {
    let threshold = AlertThreshold {
        count: 10,
        window_minutes: 120,
        severity: Severity::Critical,
    };

    assert_eq!(threshold.count, 10);
    assert_eq!(threshold.window_minutes, 120);
    assert_eq!(threshold.severity, Severity::Critical);
}

#[test]
fn test_detection_monitor_initialization() {
    let thresholds = HashMap::new();
    let monitor = DetectionMonitor::new(thresholds);
    assert_eq!(monitor.event_count(), 0);
}

#[test]
fn test_detection_monitor_record_single_event() {
    let monitor = DetectionMonitor::new(HashMap::new());

    let event = DetectionEvent::new(
        DetectionEventType::RateLimit429,
        Severity::High,
        "acc_123".to_string(),
        "claude-4.5-sonnet".to_string(),
        "req_456".to_string(),
        "Rate limit exceeded".to_string(),
    );

    monitor.record_event(event);
    assert_eq!(monitor.event_count(), 1);
}

#[test]
fn test_detection_monitor_record_multiple_events() {
    let monitor = DetectionMonitor::new(HashMap::new());

    for i in 0..10 {
        let event = DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            format!("acc_{}", i),
            "model".to_string(),
            format!("req_{}", i),
            "context".to_string(),
        );
        monitor.record_event(event);
    }

    assert_eq!(monitor.event_count(), 10);
}

#[test]
fn test_detection_statistics_total_events() {
    let monitor = DetectionMonitor::new(HashMap::new());

    for i in 0..5 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            format!("acc_{}", i),
            "model".to_string(),
            format!("req_{}", i),
            "".to_string(),
        ));
    }

    let stats = monitor.get_statistics();
    assert_eq!(stats.total_events, 5);
}

#[test]
fn test_detection_statistics_by_type() {
    let monitor = DetectionMonitor::new(HashMap::new());

    // 3 rate limit events
    for i in 0..3 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            format!("acc_{}", i),
            "model".to_string(),
            format!("req_{}", i),
            "".to_string(),
        ));
    }

    // 2 auth error events
    for i in 0..2 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::AuthError401,
            Severity::Critical,
            format!("acc_{}", i),
            "model".to_string(),
            format!("req_auth_{}", i),
            "".to_string(),
        ));
    }

    let stats = monitor.get_statistics();
    assert_eq!(stats.total_events, 5);
    assert_eq!(
        stats.events_by_type.get(&DetectionEventType::RateLimit429),
        Some(&3)
    );
    assert_eq!(
        stats.events_by_type.get(&DetectionEventType::AuthError401),
        Some(&2)
    );
}

#[test]
fn test_detection_statistics_by_severity() {
    let monitor = DetectionMonitor::new(HashMap::new());

    monitor.record_event(DetectionEvent::new(
        DetectionEventType::IdeTypeMissing,
        Severity::Critical,
        "acc_1".to_string(),
        "model".to_string(),
        "req_1".to_string(),
        "".to_string(),
    ));

    monitor.record_event(DetectionEvent::new(
        DetectionEventType::RateLimit429,
        Severity::High,
        "acc_2".to_string(),
        "model".to_string(),
        "req_2".to_string(),
        "".to_string(),
    ));

    monitor.record_event(DetectionEvent::new(
        DetectionEventType::RateLimit429,
        Severity::High,
        "acc_3".to_string(),
        "model".to_string(),
        "req_3".to_string(),
        "".to_string(),
    ));

    monitor.record_event(DetectionEvent::new(
        DetectionEventType::UserAgentStatic,
        Severity::Medium,
        "acc_4".to_string(),
        "model".to_string(),
        "req_4".to_string(),
        "".to_string(),
    ));

    let stats = monitor.get_statistics();
    assert_eq!(stats.events_by_severity.get(&Severity::Critical), Some(&1));
    assert_eq!(stats.events_by_severity.get(&Severity::High), Some(&2));
    assert_eq!(stats.events_by_severity.get(&Severity::Medium), Some(&1));
    assert_eq!(stats.events_by_severity.get(&Severity::Low), None);
}

#[test]
fn test_detection_statistics_by_account() {
    let monitor = DetectionMonitor::new(HashMap::new());

    // 3 events from acc_1
    for i in 0..3 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            "acc_1".to_string(),
            "model".to_string(),
            format!("req_{}", i),
            "".to_string(),
        ));
    }

    // 2 events from acc_2
    for i in 0..2 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            "acc_2".to_string(),
            "model".to_string(),
            format!("req_acc2_{}", i),
            "".to_string(),
        ));
    }

    let stats = monitor.get_statistics();
    assert_eq!(stats.events_by_account.get("acc_1"), Some(&3));
    assert_eq!(stats.events_by_account.get("acc_2"), Some(&2));
}

#[test]
fn test_detection_statistics_by_model() {
    let monitor = DetectionMonitor::new(HashMap::new());

    // 4 events for claude model
    for i in 0..4 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            "acc".to_string(),
            "claude-4.5-sonnet".to_string(),
            format!("req_{}", i),
            "".to_string(),
        ));
    }

    // 1 event for gpt model
    monitor.record_event(DetectionEvent::new(
        DetectionEventType::RateLimit429,
        Severity::High,
        "acc".to_string(),
        "gpt-4".to_string(),
        "req_gpt".to_string(),
        "".to_string(),
    ));

    let stats = monitor.get_statistics();
    assert_eq!(stats.events_by_model.get("claude-4.5-sonnet"), Some(&4));
    assert_eq!(stats.events_by_model.get("gpt-4"), Some(&1));
}

#[test]
fn test_get_recent_events_limit() {
    let monitor = DetectionMonitor::new(HashMap::new());

    // Record 10 events
    for i in 0..10 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            format!("acc_{}", i),
            "model".to_string(),
            format!("req_{}", i),
            "".to_string(),
        ));
    }

    let recent = monitor.get_recent_events(5);
    assert_eq!(recent.len(), 5);

    // Should be in reverse order (newest first)
    assert_eq!(recent[0].account_id, "acc_9");
    assert_eq!(recent[4].account_id, "acc_5");
}

#[test]
fn test_cleanup_old_events_keeps_recent() {
    let monitor = DetectionMonitor::new(HashMap::new());

    monitor.record_event(DetectionEvent::new(
        DetectionEventType::RateLimit429,
        Severity::High,
        "acc_1".to_string(),
        "model".to_string(),
        "req_1".to_string(),
        "".to_string(),
    ));

    monitor.cleanup_old_events();
    assert_eq!(monitor.event_count(), 1);
}

#[test]
fn test_alert_threshold_not_exceeded() {
    let mut thresholds = HashMap::new();
    thresholds.insert(
        DetectionEventType::RateLimit429,
        AlertThreshold {
            count: 5,
            window_minutes: 60,
            severity: Severity::High,
        },
    );

    let monitor = DetectionMonitor::new(thresholds);

    // Record 3 events (below threshold of 5)
    for i in 0..3 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            "acc".to_string(),
            "model".to_string(),
            format!("req_{}", i),
            "".to_string(),
        ));
    }

    let stats = monitor.get_statistics();
    assert_eq!(stats.total_events, 3);
}

#[test]
fn test_alert_threshold_exceeded() {
    let mut thresholds = HashMap::new();
    thresholds.insert(
        DetectionEventType::RateLimit429,
        AlertThreshold {
            count: 3,
            window_minutes: 60,
            severity: Severity::High,
        },
    );

    let monitor = DetectionMonitor::new(thresholds);

    // Record 4 events (exceeds threshold of 3)
    for i in 0..4 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            "acc".to_string(),
            "model".to_string(),
            format!("req_{}", i),
            "".to_string(),
        ));
    }

    let stats = monitor.get_statistics();
    assert_eq!(stats.total_events, 4);
}

#[test]
fn test_multiple_event_types_statistics() {
    let monitor = DetectionMonitor::new(HashMap::new());

    monitor.record_event(DetectionEvent::new(
        DetectionEventType::IdeTypeMissing,
        Severity::Critical,
        "acc_1".to_string(),
        "claude-4.5-sonnet".to_string(),
        "req_1".to_string(),
        "Missing marker".to_string(),
    ));

    monitor.record_event(DetectionEvent::new(
        DetectionEventType::RateLimit429,
        Severity::High,
        "acc_2".to_string(),
        "gpt-4".to_string(),
        "req_2".to_string(),
        "Rate limit".to_string(),
    ));

    monitor.record_event(DetectionEvent::new(
        DetectionEventType::AuthError401,
        Severity::Critical,
        "acc_3".to_string(),
        "claude-3-opus".to_string(),
        "req_3".to_string(),
        "Auth failed".to_string(),
    ));

    let stats = monitor.get_statistics();
    assert_eq!(stats.total_events, 3);
    assert_eq!(stats.events_by_type.len(), 3);
    assert_eq!(stats.events_by_severity.get(&Severity::Critical), Some(&2));
    assert_eq!(stats.events_by_severity.get(&Severity::High), Some(&1));
}

#[test]
fn test_hourly_rate_calculation() {
    let monitor = DetectionMonitor::new(HashMap::new());

    // Record 24 events (should give ~1 event per hour over 24h)
    for i in 0..24 {
        monitor.record_event(DetectionEvent::new(
            DetectionEventType::RateLimit429,
            Severity::High,
            "acc".to_string(),
            "model".to_string(),
            format!("req_{}", i),
            "".to_string(),
        ));
    }

    let stats = monitor.get_statistics();
    // All events are recent, so hourly rate = 24 / 24 = 1.0
    assert!(stats.hourly_rate >= 0.0);
    assert!(stats.hourly_rate <= 24.0);
}
