# Compliance API Reference

**Document Version**: 1.0
**Last Updated**: 2026-01-11
**Target Audience**: Developers
**Related Epic**: [Epic-003](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md)

---

## Table of Contents

1. [Overview](#overview)
2. [Tauri Commands](#tauri-commands)
3. [TypeScript Types](#typescript-types)
4. [Events](#events)
5. [Utility Functions](#utility-functions)
6. [Examples](#examples)

---

## Overview

The Compliance API provides programmatic access to thinking mode validation metrics, violation tracking, and real-time monitoring. All commands are exposed via Tauri's IPC layer and can be invoked from the frontend using `@tauri-apps/api/core`.

### Import Pattern

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
  DetailedViolationMetrics,
  AlertLevel,
  // ... other types
} from '../types/compliance';
```

---

## Tauri Commands

### get_violation_metrics

Retrieves detailed violation metrics including counts, rates, and histogram data.

**Signature**:
```rust
#[tauri::command]
pub async fn get_violation_metrics(
    state: State<'_, ProxyServiceState>,
) -> Result<DetailedViolationMetrics, String>
```

**Frontend Usage**:
```typescript
const metrics = await invoke<DetailedViolationMetrics>(
  'get_violation_metrics'
);
```

**Returns**: `DetailedViolationMetrics`

**Errors**:
- `"Proxy monitor not initialized"` - Monitor not available

**Performance**: O(n) where n = number of violations (typically <1ms)

**Example**:
```typescript
try {
  const metrics = await invoke<DetailedViolationMetrics>(
    'get_violation_metrics'
  );

  console.log('Compliance Score:', calculateComplianceScore(metrics));
  console.log('Budget Violations:', metrics.stats.thinking_budget_violations);
  console.log('Position Violations:', metrics.stats.thinking_position_violations);
} catch (error) {
  console.error('Failed to get metrics:', error);
}
```

---

### reset_violation_metrics

Resets all violation counters and clears historical data. This action is irreversible.

**Signature**:
```rust
#[tauri::command]
pub async fn reset_violation_metrics(
    state: State<'_, ProxyServiceState>,
) -> Result<(), String>
```

**Frontend Usage**:
```typescript
await invoke('reset_violation_metrics');
```

**Returns**: `void`

**Errors**:
- `"Proxy monitor not initialized"` - Monitor not available

**Side Effects**:
- Clears all violation counters (budget + position)
- Clears detailed metrics (timestamps, indices)
- Persists reset state to database
- Emits `proxy://violation-reset` event

**Performance**: O(1) - constant time operation

**Example**:
```typescript
const confirmed = window.confirm(
  'Are you sure you want to reset all violation metrics? This cannot be undone.'
);

if (confirmed) {
  try {
    await invoke('reset_violation_metrics');
    console.log('Metrics reset successfully');

    // Reload metrics to show zeroed state
    const metrics = await invoke('get_violation_metrics');
    updateDashboard(metrics);
  } catch (error) {
    console.error('Failed to reset metrics:', error);
  }
}
```

---

### get_proxy_stats

Retrieves basic proxy statistics including request counts and violation counts (simplified version).

**Signature**:
```rust
#[tauri::command]
pub async fn get_proxy_stats(
    state: State<'_, ProxyServiceState>
) -> Result<ProxyStats, String>
```

**Frontend Usage**:
```typescript
const stats = await invoke<ProxyStats>('get_proxy_stats');
```

**Returns**: `ProxyStats` (subset of DetailedViolationMetrics)

**Note**: For compliance monitoring, prefer `get_violation_metrics` which includes rates and histogram.

**Example**:
```typescript
const stats = await invoke<ProxyStats>('get_proxy_stats');

console.log('Total Requests:', stats.total_requests);
console.log('Success Rate:',
  (stats.success_count / stats.total_requests * 100).toFixed(2) + '%'
);
```

---

### get_proxy_status

Checks if proxy service is running (prerequisite for compliance monitoring).

**Signature**:
```rust
#[tauri::command]
pub async fn get_proxy_status(
    state: State<'_, ProxyServiceState>
) -> Result<ProxyStatus, String>
```

**Frontend Usage**:
```typescript
const status = await invoke<ProxyStatus>('get_proxy_status');
```

**Returns**: `ProxyStatus`

**Example**:
```typescript
const status = await invoke<ProxyStatus>('get_proxy_status');

if (!status.running) {
  alert('Proxy service must be running to view compliance metrics');
  return;
}

// Proceed to load metrics
const metrics = await invoke('get_violation_metrics');
```

---

## TypeScript Types

### DetailedViolationMetrics

Complete violation metrics with rates and histogram.

**Definition**:
```typescript
export interface DetailedViolationMetrics {
  // Basic statistics
  stats: {
    total_requests: number;
    success_count: number;
    error_count: number;
    thinking_budget_violations: number;
    thinking_position_violations: number;
    thinking_position_violations_user: number;
    thinking_position_violations_model: number;
  };

  // Position histogram data
  position_histogram: Array<{
    bucket: number;   // Index bucket (1, 2, 3, 5, 10, 20, 50)
    count: number;    // Number of violations in this bucket
  }>;

  // Rate metrics (violations per second)
  rates: {
    budget_violations_per_second: number;
    position_violations_per_second: number;
  };
}
```

**Usage**:
```typescript
const metrics: DetailedViolationMetrics =
  await invoke('get_violation_metrics');

// Access fields
const budgetCount = metrics.stats.thinking_budget_violations;
const budgetRate = metrics.rates.budget_violations_per_second;
const histogram = metrics.position_histogram;
```

---

### ProxyStats

Basic proxy statistics (subset of DetailedViolationMetrics).

**Definition**:
```typescript
export interface ProxyStats {
  total_requests: number;
  success_count: number;
  error_count: number;
  thinking_budget_violations: number;
  thinking_position_violations: number;
  thinking_position_violations_user: number;
  thinking_position_violations_model: number;
}
```

**Usage**:
```typescript
const stats: ProxyStats = await invoke('get_proxy_stats');

const successRate = (stats.success_count / stats.total_requests) * 100;
console.log(`Success Rate: ${successRate.toFixed(2)}%`);
```

---

### ViolationRates

Rate calculations for violations.

**Definition**:
```typescript
export interface ViolationRates {
  budget_violations_per_second: number;
  position_violations_per_second: number;
}
```

**Usage**:
```typescript
const metrics = await invoke<DetailedViolationMetrics>('get_violation_metrics');
const rates: ViolationRates = metrics.rates;

if (rates.budget_violations_per_second > 0.28) {
  alert('RED: Budget violation rate critical!');
}
```

---

### AlertLevel

Alert severity levels based on violation rates.

**Definition**:
```typescript
export type AlertLevel = 'GREEN' | 'YELLOW' | 'RED';
```

**Usage**:
```typescript
import { getAlertLevel } from '../types/compliance';

const budgetRate = 0.15;
const level: AlertLevel = getAlertLevel(budgetRate);

console.log(`Alert Level: ${level}`);  // Output: "YELLOW"
```

---

### AlertThresholds

Configurable thresholds for alert levels.

**Definition**:
```typescript
export interface AlertThresholds {
  green_max: number;    // ≤0.03 violations/sec
  yellow_max: number;   // ≤0.28 violations/sec
  // RED: >yellow_max (implicit)
}

export const DEFAULT_ALERT_THRESHOLDS: AlertThresholds = {
  green_max: 0.03,
  yellow_max: 0.28,
};
```

**Usage**:
```typescript
import { DEFAULT_ALERT_THRESHOLDS, getAlertLevel } from '../types/compliance';

const customThresholds = {
  green_max: 0.01,   // Stricter
  yellow_max: 0.10,  // Stricter
};

const level = getAlertLevel(0.05, customThresholds);
console.log(level);  // "YELLOW" (custom) vs "GREEN" (default)
```

---

### ProxyStatus

Proxy service status information.

**Definition**:
```typescript
export interface ProxyStatus {
  running: boolean;
  port: number;
  base_url: string;
  active_accounts: number;
}
```

**Usage**:
```typescript
const status = await invoke<ProxyStatus>('get_proxy_status');

if (status.running) {
  console.log(`Proxy running on ${status.base_url}`);
  console.log(`Active accounts: ${status.active_accounts}`);
}
```

---

## Events

### proxy://violation

Emitted when a violation is detected (budget or position).

**Event Payload**:
```typescript
type ViolationEventPayload = 'budget' | 'position';
```

**Listener Setup**:
```typescript
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen<string>(
  'proxy://violation',
  async (event) => {
    console.log('Violation detected:', event.payload);

    // Reload metrics
    const metrics = await invoke<DetailedViolationMetrics>(
      'get_violation_metrics'
    );

    // Update UI
    updateDashboard(metrics);
  }
);

// Cleanup
unlisten();
```

**Use Cases**:
- Real-time dashboard updates
- Violation logging
- Alert triggers
- Metric recalculation

---

### proxy://violation-reset

Emitted when violation metrics are reset.

**Event Payload**: `void` (no payload)

**Listener Setup**:
```typescript
const unlisten = await listen(
  'proxy://violation-reset',
  async () => {
    console.log('Metrics reset');

    // Reload to show zeroed state
    const metrics = await invoke<DetailedViolationMetrics>(
      'get_violation_metrics'
    );

    updateDashboard(metrics);
  }
);
```

**Use Cases**:
- Dashboard synchronization after reset
- Notification to other components
- Logging reset events

---

## Utility Functions

### getAlertLevel

Determines alert level based on violation rate.

**Signature**:
```typescript
export function getAlertLevel(
  rate: number,
  thresholds: AlertThresholds = DEFAULT_ALERT_THRESHOLDS
): AlertLevel
```

**Parameters**:
- `rate`: Violations per second (number)
- `thresholds`: Optional custom thresholds

**Returns**: `AlertLevel` ('GREEN' | 'YELLOW' | 'RED')

**Algorithm**:
```typescript
if (rate <= thresholds.green_max) {
  return 'GREEN';   // ≤0.03/sec
} else if (rate <= thresholds.yellow_max) {
  return 'YELLOW';  // 0.03-0.28/sec
} else {
  return 'RED';     // >0.28/sec
}
```

**Examples**:
```typescript
getAlertLevel(0.00);  // 'GREEN'
getAlertLevel(0.03);  // 'GREEN'
getAlertLevel(0.05);  // 'YELLOW'
getAlertLevel(0.28);  // 'YELLOW'
getAlertLevel(0.30);  // 'RED'
getAlertLevel(1.00);  // 'RED'
```

---

### calculateComplianceScore

Calculates overall compliance score (0-100%).

**Signature**:
```typescript
export function calculateComplianceScore(
  metrics: DetailedViolationMetrics
): number
```

**Parameters**:
- `metrics`: Complete violation metrics

**Returns**: `number` (0-100, representing percentage)

**Algorithm**:
```typescript
let score = 100.0;

// Budget violations penalty
const budgetOverage = Math.max(
  0,
  metrics.rates.budget_violations_per_second - 0.03
);
score -= budgetOverage * 10;  // -0.1% per 0.01/sec

// Position violations penalty
const positionOverage = Math.max(
  0,
  metrics.rates.position_violations_per_second - 0.03
);
score -= positionOverage * 10;  // -0.1% per 0.01/sec

// Cap at 0%
return Math.max(0, score);
```

**Examples**:
```typescript
// Perfect compliance
const metrics1 = {
  rates: {
    budget_violations_per_second: 0.00,
    position_violations_per_second: 0.00
  },
  // ... other fields
};
calculateComplianceScore(metrics1);  // 100.00%

// Minor violations
const metrics2 = {
  rates: {
    budget_violations_per_second: 0.05,  // +0.02 overage
    position_violations_per_second: 0.04  // +0.01 overage
  },
  // ... other fields
};
calculateComplianceScore(metrics2);  // 99.70%

// Critical violations
const metrics3 = {
  rates: {
    budget_violations_per_second: 0.50,
    position_violations_per_second: 0.50
  },
  // ... other fields
};
calculateComplianceScore(metrics3);  // 90.60%
```

---

### generateComplianceReport

Generates Markdown compliance report.

**Signature**:
```typescript
export function generateComplianceReport(
  metrics: DetailedViolationMetrics
): string
```

**Parameters**:
- `metrics`: Complete violation metrics

**Returns**: `string` (Markdown formatted report)

**Report Structure**:
```markdown
# Antigravity Compliance Report

**Generated**: 2026-01-11T10:30:00Z
**Compliance Score**: 99.70%

## Violation Summary
### Budget Violations
- Total: 50
- Rate: 0.05 violations/sec
- Alert Level: YELLOW

### Position Violations
- Total: 30
  - User messages: 20
  - Model messages: 10
- Rate: 0.04 violations/sec
- Alert Level: YELLOW

### Position Histogram
- Bucket 1: 25 violations
- Bucket 2: 5 violations

## Overall Assessment
Request Statistics:
- Total requests: 1000
- Successful: 995
- Errors: 5

## Recommendations
⚠️ Minor compliance issues detected...
```

**Example**:
```typescript
const metrics = await invoke<DetailedViolationMetrics>(
  'get_violation_metrics'
);

const report = generateComplianceReport(metrics);

// Download as file
const blob = new Blob([report], { type: 'text/markdown' });
const url = URL.createObjectURL(blob);
const a = document.createElement('a');
a.href = url;
a.download = `compliance-report-${Date.now()}.md`;
a.click();
URL.revokeObjectURL(url);
```

---

## Examples

### Complete Dashboard Implementation

```typescript
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { DetailedViolationMetrics } from '../types/compliance';
import {
  calculateComplianceScore,
  getAlertLevel,
  generateComplianceReport
} from '../types/compliance';

export const ComplianceDashboard: React.FC = () => {
  const [metrics, setMetrics] = useState<DetailedViolationMetrics | null>(null);
  const [loading, setLoading] = useState(true);

  // Load metrics on mount
  useEffect(() => {
    loadMetrics();

    // Listen for violations
    const setupListener = async () => {
      await listen<string>('proxy://violation', async () => {
        await loadMetrics();
      });
    };
    setupListener();
  }, []);

  const loadMetrics = async () => {
    try {
      setLoading(true);
      const data = await invoke<DetailedViolationMetrics>(
        'get_violation_metrics'
      );
      setMetrics(data);
    } catch (error) {
      console.error('Failed to load metrics:', error);
    } finally {
      setLoading(false);
    }
  };

  const resetMetrics = async () => {
    if (!confirm('Reset all violation metrics?')) return;

    try {
      await invoke('reset_violation_metrics');
      await loadMetrics();
    } catch (error) {
      console.error('Failed to reset metrics:', error);
    }
  };

  const exportReport = () => {
    if (!metrics) return;

    const report = generateComplianceReport(metrics);
    const blob = new Blob([report], { type: 'text/markdown' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `compliance-report-${Date.now()}.md`;
    a.click();
    URL.revokeObjectURL(url);
  };

  if (loading) return <div>Loading...</div>;
  if (!metrics) return <div>No data</div>;

  const score = calculateComplianceScore(metrics);
  const budgetAlert = getAlertLevel(metrics.rates.budget_violations_per_second);
  const positionAlert = getAlertLevel(metrics.rates.position_violations_per_second);

  return (
    <div className="compliance-dashboard">
      <h2>Compliance Score: {score.toFixed(2)}%</h2>

      <div className="violations">
        <div className="budget">
          <h3>Budget Violations</h3>
          <p>Count: {metrics.stats.thinking_budget_violations}</p>
          <p>Rate: {metrics.rates.budget_violations_per_second.toFixed(4)}/sec</p>
          <span className={`alert-${budgetAlert.toLowerCase()}`}>
            {budgetAlert}
          </span>
        </div>

        <div className="position">
          <h3>Position Violations</h3>
          <p>Count: {metrics.stats.thinking_position_violations}</p>
          <p>Rate: {metrics.rates.position_violations_per_second.toFixed(4)}/sec</p>
          <span className={`alert-${positionAlert.toLowerCase()}`}>
            {positionAlert}
          </span>
        </div>
      </div>

      <div className="actions">
        <button onClick={loadMetrics}>Refresh</button>
        <button onClick={resetMetrics}>Reset</button>
        <button onClick={exportReport}>Export</button>
      </div>
    </div>
  );
};
```

### Custom Alert Handler

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { DetailedViolationMetrics, AlertLevel } from '../types/compliance';
import { getAlertLevel } from '../types/compliance';

class ComplianceAlertManager {
  private lastAlertLevel: AlertLevel = 'GREEN';

  async init() {
    // Check initial state
    await this.checkAlerts();

    // Listen for violations
    await listen('proxy://violation', async () => {
      await this.checkAlerts();
    });
  }

  private async checkAlerts() {
    try {
      const metrics = await invoke<DetailedViolationMetrics>(
        'get_violation_metrics'
      );

      const budgetLevel = getAlertLevel(
        metrics.rates.budget_violations_per_second
      );
      const positionLevel = getAlertLevel(
        metrics.rates.position_violations_per_second
      );

      // Determine overall level (worst of two)
      const overallLevel = this.getWorstLevel(budgetLevel, positionLevel);

      // Alert on escalation
      if (this.shouldAlert(overallLevel)) {
        this.sendAlert(overallLevel, metrics);
      }

      this.lastAlertLevel = overallLevel;
    } catch (error) {
      console.error('Alert check failed:', error);
    }
  }

  private getWorstLevel(a: AlertLevel, b: AlertLevel): AlertLevel {
    if (a === 'RED' || b === 'RED') return 'RED';
    if (a === 'YELLOW' || b === 'YELLOW') return 'YELLOW';
    return 'GREEN';
  }

  private shouldAlert(level: AlertLevel): boolean {
    // Alert on escalation only
    const levels: AlertLevel[] = ['GREEN', 'YELLOW', 'RED'];
    const currentIndex = levels.indexOf(level);
    const lastIndex = levels.indexOf(this.lastAlertLevel);
    return currentIndex > lastIndex;
  }

  private sendAlert(level: AlertLevel, metrics: DetailedViolationMetrics) {
    const messages = {
      YELLOW: `⚠️ Compliance Warning: Violation rates in YELLOW range`,
      RED: `🚨 Compliance Alert: Violation rates CRITICAL (RED)`
    };

    if (level !== 'GREEN') {
      // Send notification
      new Notification('Compliance Alert', {
        body: messages[level],
        icon: '/icons/alert.png'
      });

      // Log to console
      console.warn(messages[level], metrics);
    }
  }
}

// Usage
const alertManager = new ComplianceAlertManager();
alertManager.init();
```

### Rate Monitoring Service

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { DetailedViolationMetrics } from '../types/compliance';

class RateMonitoringService {
  private checkInterval: number = 60000;  // 1 minute
  private intervalId?: number;

  start() {
    this.intervalId = window.setInterval(() => {
      this.checkRates();
    }, this.checkInterval);

    // Immediate check
    this.checkRates();
  }

  stop() {
    if (this.intervalId) {
      window.clearInterval(this.intervalId);
    }
  }

  private async checkRates() {
    try {
      const metrics = await invoke<DetailedViolationMetrics>(
        'get_violation_metrics'
      );

      const budgetRate = metrics.rates.budget_violations_per_second;
      const positionRate = metrics.rates.position_violations_per_second;

      // Log rates
      console.log('Rate Check:', {
        timestamp: new Date().toISOString(),
        budgetRate: budgetRate.toFixed(4),
        positionRate: positionRate.toFixed(4),
        totalViolations:
          metrics.stats.thinking_budget_violations +
          metrics.stats.thinking_position_violations
      });

      // Alert if rates exceed thresholds
      if (budgetRate > 0.28 || positionRate > 0.28) {
        console.error('CRITICAL: Violation rates exceed RED threshold');
        this.handleCritical(metrics);
      }
    } catch (error) {
      console.error('Rate check failed:', error);
    }
  }

  private handleCritical(metrics: DetailedViolationMetrics) {
    // Send alert, log to external service, etc.
    const report = {
      timestamp: Date.now(),
      budgetRate: metrics.rates.budget_violations_per_second,
      positionRate: metrics.rates.position_violations_per_second,
      budgetCount: metrics.stats.thinking_budget_violations,
      positionCount: metrics.stats.thinking_position_violations
    };

    // Example: Send to logging service
    fetch('/api/compliance-alerts', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(report)
    });
  }
}

// Usage
const monitor = new RateMonitoringService();
monitor.start();

// Cleanup on unmount
window.addEventListener('beforeunload', () => {
  monitor.stop();
});
```

---

## Related Documentation

- [Compliance Dashboard User Guide](./compliance-dashboard-guide.md)
- [Thinking Mode Validation Architecture](./thinking-mode-validation-architecture.md)
- [Compliance Troubleshooting Guide](./compliance-troubleshooting-guide.md)
- [Epic-003: Claude 4.5 Sonnet Thinking Compliance](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md)

---

## Revision History

| Date | Version | Changes | Author |
|------|---------|---------|--------|
| 2026-01-11 | 1.0 | Initial API reference documentation | Engineering Team |
