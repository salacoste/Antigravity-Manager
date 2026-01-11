# Compliance Testing & Validation Guide

**Document Version**: 1.0
**Last Updated**: 2026-01-11
**Target Audience**: QA Engineers, Developers
**Related Epic**: [Epic-003](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md)

---

## Table of Contents

1. [Overview](#overview)
2. [Unit Testing](#unit-testing)
3. [Integration Testing](#integration-testing)
4. [End-to-End Testing](#end-to-end-testing)
5. [Performance Testing](#performance-testing)
6. [Validation Checklist](#validation-checklist)
7. [Test Data](#test-data)
8. [Continuous Integration](#continuous-integration)

---

## Overview

### Testing Philosophy

The compliance system requires comprehensive testing across multiple layers:

1. **Unit Tests**: Validate core logic (budget/position detection, rate calculation)
2. **Integration Tests**: Verify component interaction (monitor, mapper, handler)
3. **E2E Tests**: Validate full request flow (client → proxy → Antigravity)
4. **Performance Tests**: Ensure <2ms validation overhead

### Testing Tools

**Rust Backend**:
- `cargo test` - Unit and integration tests
- `tokio::test` - Async test runtime
- `assert_eq!`, `assert!` - Assertions

**TypeScript Frontend**:
- Jest - Testing framework
- React Testing Library - Component testing
- `@testing-library/user-event` - User interactions

---

## Unit Testing

### Budget Validation Tests

**Test File**: `src-tauri/src/proxy/mappers/claude/thinking_utils_test.rs`

#### Test: Compliant Configuration

```rust
#[tokio::test]
async fn test_budget_validation_compliant() {
    let monitor = Arc::new(ProxyMonitor::new(100, None));

    // Compliant: maxOutputTokens > thinkingBudget + 100
    check_budget_compliance(
        4096,  // thinking_budget
        8192,  // max_output_tokens
        &monitor
    ).await;

    let stats = monitor.get_stats().await;
    assert_eq!(
        stats.thinking_budget_violations,
        0,
        "Should have zero budget violations"
    );
}
```

**Expected Result**: ✅ No violations recorded

#### Test: Violating Configuration

```rust
#[tokio::test]
async fn test_budget_validation_violation() {
    let monitor = Arc::new(ProxyMonitor::new(100, None));

    // Violation: 4000 < 4096 + 100
    check_budget_compliance(
        4096,  // thinking_budget
        4000,  // max_output_tokens (too low)
        &monitor
    ).await;

    let stats = monitor.get_stats().await;
    assert_eq!(
        stats.thinking_budget_violations,
        1,
        "Should have exactly one budget violation"
    );
}
```

**Expected Result**: ✅ One violation recorded

#### Test: Edge Cases

```rust
#[tokio::test]
async fn test_budget_validation_edge_cases() {
    let monitor = Arc::new(ProxyMonitor::new(100, None));

    // Edge case 1: Exactly at threshold (compliant)
    check_budget_compliance(4096, 4196, &monitor).await;
    let stats1 = monitor.get_stats().await;
    assert_eq!(stats1.thinking_budget_violations, 0);

    // Edge case 2: One below threshold (violation)
    monitor.reset_violation_metrics().await;
    check_budget_compliance(4096, 4195, &monitor).await;
    let stats2 = monitor.get_stats().await;
    assert_eq!(stats2.thinking_budget_violations, 1);

    // Edge case 3: Zero budget (always compliant)
    monitor.reset_violation_metrics().await;
    check_budget_compliance(0, 4096, &monitor).await;
    let stats3 = monitor.get_stats().await;
    assert_eq!(stats3.thinking_budget_violations, 0);
}
```

### Position Validation Tests

**Test File**: `src-tauri/src/proxy/mappers/claude/thinking_utils_test.rs`

#### Test: Correct Order

```rust
#[tokio::test]
async fn test_position_validation_correct_order() {
    let monitor = Arc::new(ProxyMonitor::new(100, None));

    let content = vec![
        ContentBlock {
            r#type: "thinking".to_string(),
            thinking: Some("test".to_string()),
            text: None,
        },
        ContentBlock {
            r#type: "text".to_string(),
            thinking: None,
            text: Some("user message".to_string()),
        },
    ];

    validate_content_ordering(&content, &monitor, MessageRole::User).await;

    let stats = monitor.get_stats().await;
    assert_eq!(
        stats.thinking_position_violations,
        0,
        "Correct order should have zero violations"
    );
}
```

**Expected Result**: ✅ No violations

#### Test: Wrong Order (User Message)

```rust
#[tokio::test]
async fn test_position_validation_wrong_order_user() {
    let monitor = Arc::new(ProxyMonitor::new(100, None));

    let content = vec![
        ContentBlock {
            r#type: "text".to_string(),
            thinking: None,
            text: Some("user message".to_string()),
        },
        ContentBlock {
            r#type: "thinking".to_string(),
            thinking: Some("test".to_string()),
            text: None,
        },
    ];

    validate_content_ordering(&content, &monitor, MessageRole::User).await;

    let stats = monitor.get_stats().await;
    assert_eq!(stats.thinking_position_violations, 1);
    assert_eq!(stats.thinking_position_violations_user, 1);
    assert_eq!(stats.thinking_position_violations_model, 0);
}
```

**Expected Result**: ✅ One user position violation

#### Test: Wrong Order (Model Message)

```rust
#[tokio::test]
async fn test_position_validation_wrong_order_model() {
    let monitor = Arc::new(ProxyMonitor::new(100, None));

    let content = vec![
        ContentBlock { r#type: "text".to_string(), /* ... */ },
        ContentBlock { r#type: "thinking".to_string(), /* ... */ },
    ];

    validate_content_ordering(&content, &monitor, MessageRole::Assistant).await;

    let stats = monitor.get_stats().await;
    assert_eq!(stats.thinking_position_violations, 1);
    assert_eq!(stats.thinking_position_violations_user, 0);
    assert_eq!(stats.thinking_position_violations_model, 1);
}
```

**Expected Result**: ✅ One model position violation

### Rate Calculation Tests

**Test File**: `src-tauri/src/proxy/monitor_test.rs`

#### Test: Rate Calculation Accuracy

```rust
#[tokio::test]
async fn test_violation_rate_calculation() {
    let monitor = Arc::new(ProxyMonitor::new(100, None));

    // Record 6 violations over 60 seconds
    for _ in 0..6 {
        monitor.record_budget_violation().await;
        tokio::time::sleep(Duration::from_secs(10)).await;
    }

    let metrics = monitor.get_violation_metrics().await;

    // Expected rate: 6 violations / 60 seconds = 0.1/sec
    let expected_rate = 0.1;
    let actual_rate = metrics.rates.budget_violations_per_second;

    assert!(
        (actual_rate - expected_rate).abs() < 0.01,
        "Rate should be approximately {} (actual: {})",
        expected_rate,
        actual_rate
    );
}
```

**Expected Result**: ✅ Rate ≈ 0.1/sec (±0.01 tolerance)

#### Test: Sliding Window Decay

```rust
#[tokio::test]
async fn test_violation_rate_sliding_window() {
    let monitor = Arc::new(ProxyMonitor::new(100, None));

    // Record 10 violations
    for _ in 0..10 {
        monitor.record_budget_violation().await;
    }

    // Immediate rate: 10/60 ≈ 0.167/sec
    let metrics1 = monitor.get_violation_metrics().await;
    assert!(metrics1.rates.budget_violations_per_second > 0.15);

    // Wait 61 seconds (violations age out)
    tokio::time::sleep(Duration::from_secs(61)).await;

    // Rate should drop to 0 (all violations outside window)
    let metrics2 = monitor.get_violation_metrics().await;
    assert_eq!(
        metrics2.rates.budget_violations_per_second,
        0.0,
        "Rate should decay to 0 after 60 seconds"
    );
}
```

**Expected Result**: ✅ Rate decays from >0.15 to 0.0

### Histogram Generation Tests

```rust
#[tokio::test]
async fn test_position_histogram_generation() {
    let monitor = Arc::new(ProxyMonitor::new(100, None));

    // Record violations at specific indices
    monitor.record_position_violation(1, MessageRole::User).await;
    monitor.record_position_violation(1, MessageRole::User).await;
    monitor.record_position_violation(2, MessageRole::User).await;
    monitor.record_position_violation(5, MessageRole::User).await;

    let metrics = monitor.get_violation_metrics().await;
    let histogram = metrics.position_histogram;

    // Verify buckets
    let bucket1 = histogram.iter().find(|e| e.bucket == 1).unwrap();
    let bucket2 = histogram.iter().find(|e| e.bucket == 2).unwrap();
    let bucket5 = histogram.iter().find(|e| e.bucket == 5).unwrap();

    assert_eq!(bucket1.count, 2, "Bucket 1 should have 2 violations");
    assert_eq!(bucket2.count, 1, "Bucket 2 should have 1 violation");
    assert_eq!(bucket5.count, 1, "Bucket 5 should have 1 violation");
}
```

**Expected Result**: ✅ Histogram matches recorded violations

### Run Unit Tests

```bash
cd src-tauri
cargo test --lib
```

**Expected Output**:
```
running 15 tests
test proxy::monitor::tests::test_violation_rate_calculation ... ok
test proxy::monitor::tests::test_violation_rate_sliding_window ... ok
test proxy::mappers::claude::tests::test_budget_validation_compliant ... ok
test proxy::mappers::claude::tests::test_budget_validation_violation ... ok
test proxy::mappers::claude::tests::test_position_validation_correct_order ... ok
test proxy::mappers::claude::tests::test_position_validation_wrong_order_user ... ok
test proxy::mappers::claude::tests::test_position_validation_wrong_order_model ... ok
test proxy::mappers::claude::tests::test_position_histogram_generation ... ok
...

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## Integration Testing

### Full Request Flow Test

**Test File**: `src-tauri/tests/integration_test.rs`

#### Test Scenario 1: Budget Violation Detection

```rust
#[tokio::test]
async fn test_budget_violation_detection_integration() {
    // Setup
    let monitor = Arc::new(ProxyMonitor::new(1000, None));
    let token_manager = setup_test_token_manager().await;

    // Create request with violating config
    let request = AnthropicRequest {
        model: "claude-sonnet-4.5".to_string(),
        max_tokens: Some(4000),  // Violates: 4000 < 4096 + 100
        messages: vec![
            Message {
                role: "user".to_string(),
                content: vec![ContentBlock::text("test")],
            }
        ],
        thinking: Some(ThinkingConfig {
            enabled: true,
            budget: Some(4096),
            type_: Some("enabled".to_string()),
        }),
        // ... other fields
    };

    // Process request through full pipeline
    let result = handle_claude_request(
        request,
        token_manager.clone(),
        monitor.clone(),
        // ... other params
    ).await;

    // Verify violation was recorded
    let stats = monitor.get_stats().await;
    assert_eq!(
        stats.thinking_budget_violations,
        1,
        "Budget violation should be detected during request processing"
    );
}
```

**Expected Result**: ✅ Budget violation recorded after full request processing

#### Test Scenario 2: Position Violation Detection

```rust
#[tokio::test]
async fn test_position_violation_detection_integration() {
    let monitor = Arc::new(ProxyMonitor::new(1000, None));
    let token_manager = setup_test_token_manager().await;

    let request = AnthropicRequest {
        model: "claude-sonnet-4.5".to_string(),
        max_tokens: Some(8192),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: vec![
                    ContentBlock::text("message"),     // index 0
                    ContentBlock::thinking("think"),   // index 1 - VIOLATION
                ],
            }
        ],
        thinking: Some(ThinkingConfig {
            enabled: true,
            budget: Some(4096),
            type_: Some("enabled".to_string()),
        }),
        // ... other fields
    };

    let result = handle_claude_request(
        request,
        token_manager,
        monitor.clone(),
        // ... other params
    ).await;

    let stats = monitor.get_stats().await;
    assert_eq!(stats.thinking_position_violations, 1);
    assert_eq!(stats.thinking_position_violations_user, 1);

    // Verify histogram
    let metrics = monitor.get_violation_metrics().await;
    let bucket1 = metrics.position_histogram
        .iter()
        .find(|e| e.bucket == 1)
        .unwrap();
    assert_eq!(bucket1.count, 1);
}
```

**Expected Result**: ✅ Position violation recorded with correct index

#### Test Scenario 3: Event Emission

```rust
#[tokio::test]
async fn test_violation_event_emission() {
    use tauri::test::{mock_builder, MockRuntime};

    let app = mock_builder().build(MockRuntime::default()).unwrap();
    let monitor = Arc::new(ProxyMonitor::new(100, Some(app.handle())));

    // Setup event listener
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.listen_global("proxy://violation", move |event| {
        tx.send(event.payload()).unwrap();
    });

    // Trigger violation
    monitor.record_budget_violation().await;

    // Verify event received
    let payload = rx.await.unwrap();
    assert_eq!(payload, "budget");
}
```

**Expected Result**: ✅ Event emitted with correct payload

### Run Integration Tests

```bash
cd src-tauri
cargo test --test integration_test
```

---

## End-to-End Testing

### Manual E2E Test Scenarios

#### Scenario 1: Budget Violation E2E

**Setup**:
1. Start proxy service
2. Open compliance dashboard
3. Reset metrics

**Steps**:
```bash
# Send request with violating budget
curl -X POST http://localhost:8045/v1/chat/completions \
  -H 'Content-Type: application/json' \
  -H 'x-api-key: your-key' \
  -d '{
    "model": "claude-sonnet-4.5",
    "max_tokens": 4000,
    "messages": [{"role": "user", "content": "test"}],
    "thinking": {
      "enabled": true,
      "budget": 4096
    }
  }'
```

**Validation**:
- [ ] Dashboard updates immediately
- [ ] Budget violations count = 1
- [ ] Compliance score < 100%
- [ ] Alert level = GREEN (first violation)
- [ ] Event logged in browser console

#### Scenario 2: Position Violation E2E

**Setup**: Same as Scenario 1

**Steps**:
```bash
curl -X POST http://localhost:8045/v1/messages \
  -H 'Content-Type: application/json' \
  -H 'x-api-key: your-key' \
  -H 'anthropic-version: 2023-06-01' \
  -d '{
    "model": "claude-sonnet-4.5",
    "max_tokens": 8192,
    "messages": [{
      "role": "user",
      "content": [
        {"type": "text", "text": "test"},
        {"type": "thinking", "thinking": "analysis"}
      ]
    }],
    "thinking": {
      "type": "enabled",
      "budget": 4096
    }
  }'
```

**Validation**:
- [ ] Position violations count = 1
- [ ] User violations = 1, Model violations = 0
- [ ] Histogram shows bucket 1 = 1
- [ ] Alert level appropriate
- [ ] Event emitted

#### Scenario 3: Rate Calculation E2E

**Setup**: Reset metrics

**Steps**:
```bash
# Send 10 violating requests in quick succession
for i in {1..10}; do
  curl -X POST http://localhost:8045/v1/chat/completions \
    -H 'Content-Type: application/json' \
    -H 'x-api-key: your-key' \
    -d '{
      "model": "claude-sonnet-4.5",
      "max_tokens": 4000,
      "messages": [{"role": "user", "content": "test"}],
      "thinking": {"enabled": true, "budget": 4096}
    }'
  sleep 1
done
```

**Validation**:
- [ ] Budget violations count = 10
- [ ] Rate ≈ 0.167/sec (10 violations / 60 seconds)
- [ ] Alert level = YELLOW (0.03-0.28 range)
- [ ] Wait 61 seconds → rate drops to 0

#### Scenario 4: Reset Functionality E2E

**Setup**: Create some violations (use Scenario 1 or 2)

**Steps**:
1. Note current violation counts
2. Click "Reset" button in dashboard
3. Confirm dialog

**Validation**:
- [ ] Confirmation dialog appears
- [ ] After confirm, all counts = 0
- [ ] Rates = 0.00/sec
- [ ] Histogram empty
- [ ] Compliance score = 100%
- [ ] `proxy://violation-reset` event emitted

#### Scenario 5: Export Report E2E

**Setup**: Create some violations

**Steps**:
1. Click "Export" button
2. Check downloads folder

**Validation**:
- [ ] File downloaded: `compliance-report-{timestamp}.md`
- [ ] Report contains correct violation counts
- [ ] Report contains correct rates
- [ ] Report contains histogram data
- [ ] Report contains recommendations
- [ ] Report is valid Markdown

### Automated E2E Tests (Playwright)

**Test File**: `e2e/compliance.spec.ts`

```typescript
import { test, expect } from '@playwright/test';

test.describe('Compliance Dashboard E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:5173/monitor');
    await page.waitForSelector('.compliance-dashboard');
  });

  test('should display compliance score', async ({ page }) => {
    const score = await page.textContent('.compliance-score');
    expect(score).toMatch(/\d+\.\d+%/);
  });

  test('should update on violation', async ({ page }) => {
    // Get initial count
    const initialCount = await page.textContent('.budget-violations-count');

    // Trigger violation via API
    await fetch('http://localhost:8045/v1/chat/completions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': 'test-key'
      },
      body: JSON.stringify({
        model: 'claude-sonnet-4.5',
        max_tokens: 4000,
        messages: [{ role: 'user', content: 'test' }],
        thinking: { enabled: true, budget: 4096 }
      })
    });

    // Wait for update
    await page.waitForTimeout(1000);

    // Verify count increased
    const newCount = await page.textContent('.budget-violations-count');
    expect(parseInt(newCount)).toBe(parseInt(initialCount) + 1);
  });

  test('should reset metrics', async ({ page }) => {
    // Click reset button
    await page.click('button:has-text("Reset")');

    // Confirm dialog
    page.on('dialog', dialog => dialog.accept());

    // Wait for reset
    await page.waitForTimeout(500);

    // Verify counts are zero
    const budgetCount = await page.textContent('.budget-violations-count');
    const positionCount = await page.textContent('.position-violations-count');

    expect(budgetCount).toBe('0');
    expect(positionCount).toBe('0');
  });
});
```

**Run E2E Tests**:
```bash
npx playwright test e2e/compliance.spec.ts
```

---

## Performance Testing

### Validation Overhead Benchmarks

**Test File**: `src-tauri/benches/validation_bench.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;

fn benchmark_budget_validation(c: &mut Criterion) {
    let monitor = Arc::new(ProxyMonitor::new(1000, None));

    c.bench_function("budget_validation", |b| {
        b.iter(|| {
            check_budget_compliance(
                black_box(4096),
                black_box(8192),
                &monitor
            )
        });
    });
}

fn benchmark_position_validation(c: &mut Criterion) {
    let monitor = Arc::new(ProxyMonitor::new(1000, None));

    let content = vec![
        ContentBlock::thinking("test"),
        ContentBlock::text("message"),
    ];

    c.bench_function("position_validation", |b| {
        b.iter(|| {
            validate_content_ordering(
                black_box(&content),
                &monitor,
                MessageRole::User
            )
        });
    });
}

criterion_group!(benches, benchmark_budget_validation, benchmark_position_validation);
criterion_main!(benches);
```

**Run Benchmarks**:
```bash
cd src-tauri
cargo bench
```

**Expected Results**:
```
budget_validation       time: [95 ns 98 ns 102 ns]
position_validation     time: [450 ns 475 ns 500 ns]
```

**Performance Requirements**:
- ✅ Budget validation: <1 ms
- ✅ Position validation: <1 ms
- ✅ Total overhead: <2 ms per request

### Load Testing

**Test Script**: `load-test.sh`

```bash
#!/bin/bash

# Send 1000 requests with violations
echo "Starting load test (1000 requests)..."

for i in {1..1000}; do
  curl -s -X POST http://localhost:8045/v1/chat/completions \
    -H 'Content-Type: application/json' \
    -H 'x-api-key: test-key' \
    -d '{
      "model": "claude-sonnet-4.5",
      "max_tokens": 4000,
      "messages": [{"role": "user", "content": "test"}],
      "thinking": {"enabled": true, "budget": 4096}
    }' > /dev/null &

  if [ $((i % 100)) -eq 0 ]; then
    echo "Sent $i requests"
    wait  # Wait for batch to complete
  fi
done

wait  # Wait for all requests

echo "Load test complete"
echo "Check dashboard for violation counts and rates"
```

**Validation**:
- [ ] All 1000 violations recorded
- [ ] No memory leaks (check `top` or Activity Monitor)
- [ ] Response times stable
- [ ] Dashboard remains responsive
- [ ] Rates calculated correctly

---

## Validation Checklist

### Pre-Release Validation

**Unit Tests**:
- [ ] All unit tests passing (`cargo test --lib`)
- [ ] Coverage ≥80% for validation logic
- [ ] Edge cases tested

**Integration Tests**:
- [ ] All integration tests passing (`cargo test --test integration_test`)
- [ ] Event emission verified
- [ ] Full request flow tested

**E2E Tests**:
- [ ] Manual E2E scenarios completed
- [ ] Playwright tests passing
- [ ] Real violations detected correctly

**Performance**:
- [ ] Validation overhead <2ms
- [ ] Load test (1000 requests) passed
- [ ] Memory usage stable

**Dashboard**:
- [ ] Real-time updates working
- [ ] Reset functionality working
- [ ] Export functionality working
- [ ] Alerts displaying correctly

**Documentation**:
- [ ] User guide complete
- [ ] API reference accurate
- [ ] Troubleshooting guide tested

### Regression Testing

**After Code Changes**:
1. Run full test suite: `cargo test && npm test`
2. Manual smoke test (send 10 requests, check dashboard)
3. Verify no new violations in clean state
4. Check performance benchmarks

**After Dependency Updates**:
1. Run integration tests
2. Verify Tauri event system still working
3. Check database persistence

---

## Test Data

### Sample Valid Requests

**Compliant Budget**:
```json
{
  "model": "claude-sonnet-4.5",
  "max_tokens": 8192,
  "messages": [{"role": "user", "content": "test"}],
  "thinking": {
    "enabled": true,
    "budget": 4096
  }
}
```

**Compliant Position**:
```json
{
  "model": "claude-sonnet-4.5",
  "max_tokens": 8192,
  "messages": [{
    "role": "user",
    "content": [
      {"type": "thinking", "thinking": "analysis"},
      {"type": "text", "text": "message"}
    ]
  }],
  "thinking": {
    "enabled": true,
    "budget": 4096
  }
}
```

### Sample Violating Requests

**Budget Violation**:
```json
{
  "model": "claude-sonnet-4.5",
  "max_tokens": 4000,  // VIOLATION: 4000 < 4096 + 100
  "messages": [{"role": "user", "content": "test"}],
  "thinking": {
    "enabled": true,
    "budget": 4096
  }
}
```

**Position Violation**:
```json
{
  "model": "claude-sonnet-4.5",
  "max_tokens": 8192,
  "messages": [{
    "role": "user",
    "content": [
      {"type": "text", "text": "message"},
      {"type": "thinking", "thinking": "analysis"}  // VIOLATION: index 1
    ]
  }],
  "thinking": {
    "enabled": true,
    "budget": 4096
  }
}
```

---

## Continuous Integration

### GitHub Actions Workflow

**File**: `.github/workflows/compliance-tests.yml`

```yaml
name: Compliance Tests

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  rust-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run unit tests
        run: cd src-tauri && cargo test --lib

      - name: Run integration tests
        run: cd src-tauri && cargo test --test integration_test

      - name: Run benchmarks
        run: cd src-tauri && cargo bench --no-run

  frontend-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Install dependencies
        run: npm install

      - name: Run frontend tests
        run: npm test

      - name: Type check
        run: npx tsc --noEmit

  e2e-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Install dependencies
        run: npm install

      - name: Install Playwright
        run: npx playwright install --with-deps

      - name: Run E2E tests
        run: npx playwright test
```

### Pre-Commit Hooks

**File**: `.git/hooks/pre-commit`

```bash
#!/bin/bash

echo "Running compliance tests..."

# Run Rust tests
cd src-tauri
cargo test --lib --quiet
if [ $? -ne 0 ]; then
  echo "❌ Rust unit tests failed"
  exit 1
fi

# Run TypeScript type check
cd ..
npx tsc --noEmit
if [ $? -ne 0 ]; then
  echo "❌ TypeScript type check failed"
  exit 1
fi

echo "✅ All pre-commit checks passed"
exit 0
```

---

## Related Documentation

- [Compliance Dashboard User Guide](./compliance-dashboard-guide.md)
- [Thinking Mode Validation Architecture](./thinking-mode-validation-architecture.md)
- [Compliance Troubleshooting Guide](./compliance-troubleshooting-guide.md)
- [API Reference: Compliance Commands](./compliance-api-reference.md)
- [Epic-003: Claude 4.5 Sonnet Thinking Compliance](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md)
- [Story-003-11: Integration Test Infrastructure](../stories/story-011-integration-tests.md)

---

## Revision History

| Date | Version | Changes | Author |
|------|---------|---------|--------|
| 2026-01-11 | 1.0 | Initial testing guide | Engineering Team |
