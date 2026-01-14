# Story-003-12: Compliance Monitoring Dashboard

**Story ID**: Story-003-12
**Epic**: [Epic-003](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md) - Claude 4.5 Sonnet Thinking - 100% Compliance
**Priority**: P3 (Enhancement - Monitoring)
**Estimated Effort**: 1 hour
**Status**: ✅ IMPLEMENTED [THINKING-SPECIFIC]
**Implementation**:
- Frontend: compliance.ts (163 lines), ComplianceMetrics.tsx (400+ lines)
- Backend: proxy.rs:196-227 (Tauri commands)
**Created**: 2026-01-10
**Updated**: 2026-01-11 (Verified implementation)
**Owner**: Engineering Team

---

## User Story

**As a** system administrator monitoring Antigravity proxy compliance
**I want** real-time compliance dashboard with violation metrics and alerts
**So that** I can ensure 100% compliance, detect issues proactively, and validate anti-detection effectiveness

---

## Context

### Current Situation

**Existing Monitor Page** (`src/pages/Monitor.tsx`, `src/components/proxy/ProxyMonitor.tsx`):
```typescript
// Current dashboard shows:
interface ProxyStats {
    total_requests: number;
    success_count: number;
    error_count: number;
}

// ❌ Missing compliance-specific metrics:
// - No violation counters
// - No compliance score
// - No alert thresholds
// - No rate tracking
```

**Issues**:
- ❌ **No violation visibility**: Cannot see budget/position violations
- ❌ **No compliance score**: Cannot track progress to 100%
- ❌ **No alert system**: Cannot detect high violation rates
- ❌ **No metrics breakdown**: Cannot diagnose issues
- ❌ **Missing dashboard section**: Compliance metrics not displayed

### Expected Behavior

**Comprehensive Compliance Dashboard** (Phase 4 Success Criteria):
```
┌────────────────────────────────────────────────────────────┐
│ Compliance Monitoring Dashboard                            │
├────────────────────────────────────────────────────────────┤
│                                                            │
│ 📊 Compliance Score: 100% ✅                               │
│                                                            │
│ ┌──────────────────────┐ ┌──────────────────────┐        │
│ │ Budget Violations    │ │ Position Violations  │        │
│ │ Total: 12            │ │ Total: 5             │        │
│ │ Rate: 0.05/sec 🟢   │ │ Rate: 0.02/sec 🟢   │        │
│ └──────────────────────┘ └──────────────────────┘        │
│                                                            │
│ Position Violation Histogram:                              │
│ Index 1: ████████ 3                                       │
│ Index 2: ████ 1                                            │
│ Index 5: ████ 1                                            │
│                                                            │
│ 🔔 Alerts: None (all thresholds GREEN)                     │
│                                                            │
│ Last Updated: 2026-01-10 15:30:45                          │
│ [Refresh] [Reset Metrics] [Export Report]                 │
└────────────────────────────────────────────────────────────┘
```

### Gap Analysis

**Reference**: Epic-003 Phase 4 Success Criteria

| Feature | Expected | Current | Gap | Priority |
|---------|----------|---------|-----|----------|
| **Real-time metrics** | ✅ Required | ❌ Missing | Complete | HIGH |
| **Detection alerts** | ✅ Required | ❌ Missing | Complete | HIGH |
| **Compliance score** | ✅ Required | ❌ Missing | Complete | MEDIUM |
| **Violation breakdown** | ✅ Required | ❌ Missing | Complete | MEDIUM |
| **Dashboard UI** | ✅ Required | ❌ Missing | Complete | MEDIUM |

**Impact**: MEDIUM to HIGH
- **Observability Gap**: Cannot monitor compliance in real-time
- **Issue Detection**: Cannot detect high violation rates proactively
- **Validation Gap**: Cannot confirm 100% compliance
- **User Experience**: Operators have no visibility into compliance status

**Priority**: P3 - Enhancement, but HIGH value for operations

---

## Reference Documentation

### Primary Dependencies
- **Story-003-06**: `docs/stories/Story-003-06-budget-constraint-warnings.md`
  - Budget violation logging

- **Story-003-07**: `docs/stories/Story-003-07-position-enforcement-logging.md`
  - Position violation logging

- **Story-003-08**: `docs/stories/Story-003-08-enhanced-violation-metrics.md`
  - Lines 260-385: ViolationMetrics structure
  - Lines 570-630: Tauri command `get_violation_metrics`
  - Lines 755-830: Frontend integration types

### Current Implementation
- **Monitor Page**: `src/pages/Monitor.tsx` (38 lines)
- **ProxyMonitor Component**: `src/components/proxy/ProxyMonitor.tsx` (~400 lines)
- **Current Stats**: ProxyStats (total, success, error counts only)

### Epic Success Criteria
- **Epic-003**: Lines 153-157 (Phase 4)
  - Real-time compliance metrics
  - Detection failure alerts
  - 100% compliance validated

---

## Technical Details

### Architecture Overview

**Dashboard Component Structure**:
```
┌────────────────────────────────────────────────────────────┐
│ Monitor Page (pages/Monitor.tsx)                          │
│ ├─ ProxyMonitor Component (existing)                      │
│ │  ├─ Stats Cards (existing)                              │
│ │  ├─ Request Logs Table (existing)                       │
│ │  └─ Request Details Modal (existing)                    │
│ │                                                          │
│ └─ 🆕 ComplianceMetrics Component (new)                   │
│    ├─ Compliance Score Card                               │
│    ├─ Violation Counters                                  │
│    ├─ Violation Rate Indicators                           │
│    ├─ Position Histogram Chart                            │
│    ├─ Alert System                                        │
│    └─ Action Buttons (refresh, reset, export)             │
└────────────────────────────────────────────────────────────┘
```

**Data Flow**:
```
Backend (monitor.rs)
  → get_violation_metrics() Tauri command
  → DetailedViolationMetrics JSON
Frontend (ComplianceMetrics.tsx)
  → useState<DetailedViolationMetrics>
  → Real-time updates via Tauri events
  → Visual components render
```

### Implementation Steps

#### Step 1: Define Frontend Types

**Location**: `src/types/compliance.ts` (new file)

**Type Definitions**:
```typescript
/**
 * Detailed violation metrics from Story-003-08
 */
export interface DetailedViolationMetrics {
    // Cumulative counters
    total_budget_violations: number;
    total_position_violations: number;
    position_violations_user: number;
    position_violations_model: number;

    // Histogram data
    position_histogram: Array<{
        index: number;
        count: number;
    }>;

    // Rate metrics (violations per second)
    budget_violation_rate: number;
    position_violation_rate: number;

    // Alert levels
    budget_alert_level: 'GREEN' | 'YELLOW' | 'RED';
    position_alert_level: 'GREEN' | 'YELLOW' | 'RED';

    // Timestamps
    last_budget_violation: number | null;
    last_position_violation: number | null;
    metrics_start_time: number;
}

/**
 * Alert configuration thresholds
 */
export interface AlertThresholds {
    green_max: number;    // 0.03 violations/sec
    yellow_max: number;   // 0.28 violations/sec
    // red: > yellow_max
}

export const DEFAULT_ALERT_THRESHOLDS: AlertThresholds = {
    green_max: 0.03,
    yellow_max: 0.28,
};
```

#### Step 2: Create ComplianceMetrics Component

**Location**: `src/components/proxy/ComplianceMetrics.tsx` (new file)

**Component Structure**:
```typescript
import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { request as invoke } from '../../utils/request';
import { useTranslation } from 'react-i18next';
import { AlertCircle, CheckCircle, RefreshCw, Trash2, Download } from 'lucide-react';
import { DetailedViolationMetrics, DEFAULT_ALERT_THRESHOLDS } from '../../types/compliance';

interface ComplianceMetricsProps {
    className?: string;
}

export const ComplianceMetrics: React.FC<ComplianceMetricsProps> = ({ className }) => {
    const { t } = useTranslation();
    const [metrics, setMetrics] = useState<DetailedViolationMetrics | null>(null);
    const [loading, setLoading] = useState(true);

    // Load metrics on mount
    useEffect(() => {
        loadMetrics();

        // Listen for violation events
        const setupListener = async () => {
            await listen<string>('proxy://violation', async (_event) => {
                // Reload metrics when violation occurs
                await loadMetrics();
            });
        };
        setupListener();
    }, []);

    const loadMetrics = async () => {
        try {
            setLoading(true);
            const data = await invoke<DetailedViolationMetrics>('get_violation_metrics');
            setMetrics(data);
        } catch (e) {
            console.error('Failed to load violation metrics:', e);
        } finally {
            setLoading(false);
        }
    };

    const resetMetrics = async () => {
        if (!confirm(t('compliance.confirm_reset'))) return;

        try {
            await invoke('reset_violation_metrics');
            await loadMetrics();
        } catch (e) {
            console.error('Failed to reset metrics:', e);
        }
    };

    const exportReport = async () => {
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

    if (loading) {
        return (
            <div className={`flex items-center justify-center p-8 ${className}`}>
                <span className="loading loading-spinner loading-lg"></span>
            </div>
        );
    }

    if (!metrics) {
        return (
            <div className={`flex items-center justify-center p-8 ${className}`}>
                <p className="text-gray-500">{t('compliance.no_data')}</p>
            </div>
        );
    }

    const totalViolations = metrics.total_budget_violations + metrics.total_position_violations;
    const complianceScore = calculateComplianceScore(metrics);

    return (
        <div className={`bg-white dark:bg-base-100 rounded-xl shadow-sm border border-gray-100 dark:border-base-200 ${className}`}>
            {/* Header */}
            <div className="p-4 border-b border-gray-100 dark:border-base-200 flex items-center justify-between">
                <h2 className="text-lg font-bold text-gray-900 dark:text-base-content">
                    {t('compliance.title')}
                </h2>
                <div className="flex gap-2">
                    <button
                        onClick={loadMetrics}
                        className="btn btn-sm btn-ghost"
                        title={t('compliance.refresh')}
                    >
                        <RefreshCw size={16} />
                    </button>
                    <button
                        onClick={resetMetrics}
                        className="btn btn-sm btn-ghost"
                        title={t('compliance.reset')}
                    >
                        <Trash2 size={16} />
                    </button>
                    <button
                        onClick={exportReport}
                        className="btn btn-sm btn-ghost"
                        title={t('compliance.export')}
                    >
                        <Download size={16} />
                    </button>
                </div>
            </div>

            {/* Compliance Score */}
            <div className="p-4 border-b border-gray-100 dark:border-base-200">
                <div className="flex items-center justify-between">
                    <div>
                        <div className="text-sm text-gray-500 dark:text-gray-400">
                            {t('compliance.score')}
                        </div>
                        <div className="text-3xl font-bold text-green-600 dark:text-green-400">
                            {complianceScore.toFixed(2)}%
                        </div>
                    </div>
                    {complianceScore >= 99.5 ? (
                        <CheckCircle size={48} className="text-green-500" />
                    ) : (
                        <AlertCircle size={48} className="text-yellow-500" />
                    )}
                </div>
            </div>

            {/* Violation Cards */}
            <div className="p-4 grid grid-cols-2 gap-4">
                {/* Budget Violations */}
                <ViolationCard
                    title={t('compliance.budget_violations')}
                    count={metrics.total_budget_violations}
                    rate={metrics.budget_violation_rate}
                    alertLevel={metrics.budget_alert_level}
                    lastViolation={metrics.last_budget_violation}
                />

                {/* Position Violations */}
                <ViolationCard
                    title={t('compliance.position_violations')}
                    count={metrics.total_position_violations}
                    rate={metrics.position_violation_rate}
                    alertLevel={metrics.position_alert_level}
                    lastViolation={metrics.last_position_violation}
                    breakdown={{
                        user: metrics.position_violations_user,
                        model: metrics.position_violations_model,
                    }}
                />
            </div>

            {/* Position Histogram */}
            {metrics.position_histogram.length > 0 && (
                <div className="p-4 border-t border-gray-100 dark:border-base-200">
                    <h3 className="text-sm font-semibold mb-3 text-gray-700 dark:text-gray-300">
                        {t('compliance.position_histogram')}
                    </h3>
                    <PositionHistogram data={metrics.position_histogram} />
                </div>
            )}

            {/* Alerts */}
            {(metrics.budget_alert_level !== 'GREEN' || metrics.position_alert_level !== 'GREEN') && (
                <div className="p-4 border-t border-gray-100 dark:border-base-200">
                    <AlertPanel
                        budgetLevel={metrics.budget_alert_level}
                        positionLevel={metrics.position_alert_level}
                        budgetRate={metrics.budget_violation_rate}
                        positionRate={metrics.position_violation_rate}
                    />
                </div>
            )}
        </div>
    );
};

// Helper function: Calculate compliance score
function calculateComplianceScore(metrics: DetailedViolationMetrics): number {
    // Base score: 100%
    // Deduct based on violation rates
    let score = 100.0;

    // Budget violations: -0.1% per 0.01 violations/sec over GREEN
    const budgetPenalty = Math.max(0, (metrics.budget_violation_rate - 0.03) * 10);
    score -= budgetPenalty;

    // Position violations: -0.1% per 0.01 violations/sec over GREEN
    const positionPenalty = Math.max(0, (metrics.position_violation_rate - 0.03) * 10);
    score -= positionPenalty;

    // Cap at 0%
    return Math.max(0, score);
}

// Helper function: Generate compliance report
function generateComplianceReport(metrics: DetailedViolationMetrics): string {
    const score = calculateComplianceScore(metrics);
    const timestamp = new Date().toISOString();

    return `# Antigravity Compliance Report

**Generated**: ${timestamp}
**Compliance Score**: ${score.toFixed(2)}%

## Violation Summary

### Budget Violations
- **Total**: ${metrics.total_budget_violations}
- **Rate**: ${metrics.budget_violation_rate.toFixed(4)} violations/sec
- **Alert Level**: ${metrics.budget_alert_level}
- **Last Violation**: ${metrics.last_budget_violation ? new Date(metrics.last_budget_violation).toISOString() : 'Never'}

### Position Violations
- **Total**: ${metrics.total_position_violations}
  - User messages: ${metrics.position_violations_user}
  - Model messages: ${metrics.position_violations_model}
- **Rate**: ${metrics.position_violation_rate.toFixed(4)} violations/sec
- **Alert Level**: ${metrics.position_alert_level}
- **Last Violation**: ${metrics.last_position_violation ? new Date(metrics.last_position_violation).toISOString() : 'Never'}

### Position Histogram

${metrics.position_histogram.map(entry =>
    `- Index ${entry.index}: ${entry.count} violations`
).join('\n')}

## Recommendations

${score >= 99.5 ? '✅ Compliance is excellent. Continue monitoring.' : '⚠️ Review client configurations to reduce violations.'}
`;
}
```

#### Step 3: Create ViolationCard Sub-Component

**Location**: Same file (`ComplianceMetrics.tsx`)

```typescript
interface ViolationCardProps {
    title: string;
    count: number;
    rate: number;
    alertLevel: 'GREEN' | 'YELLOW' | 'RED';
    lastViolation: number | null;
    breakdown?: {
        user: number;
        model: number;
    };
}

const ViolationCard: React.FC<ViolationCardProps> = ({
    title,
    count,
    rate,
    alertLevel,
    lastViolation,
    breakdown,
}) => {
    const { t } = useTranslation();

    const alertColors = {
        GREEN: 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400',
        YELLOW: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400',
        RED: 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400',
    };

    return (
        <div className="border border-gray-200 dark:border-base-300 rounded-lg p-3">
            <div className="text-xs text-gray-500 dark:text-gray-400 mb-1">
                {title}
            </div>
            <div className="text-2xl font-bold text-gray-900 dark:text-base-content mb-2">
                {count.toLocaleString()}
            </div>

            {/* Rate with alert indicator */}
            <div className="flex items-center gap-2 mb-2">
                <span className="text-xs text-gray-600 dark:text-gray-400">
                    {rate.toFixed(4)} /sec
                </span>
                <span className={`text-xs px-2 py-0.5 rounded-full ${alertColors[alertLevel]}`}>
                    {alertLevel}
                </span>
            </div>

            {/* Breakdown (if provided) */}
            {breakdown && (
                <div className="text-xs text-gray-500 dark:text-gray-400 space-y-1">
                    <div className="flex justify-between">
                        <span>{t('compliance.user_messages')}:</span>
                        <span className="font-medium">{breakdown.user}</span>
                    </div>
                    <div className="flex justify-between">
                        <span>{t('compliance.model_messages')}:</span>
                        <span className="font-medium">{breakdown.model}</span>
                    </div>
                </div>
            )}

            {/* Last violation */}
            {lastViolation && (
                <div className="text-xs text-gray-400 dark:text-gray-500 mt-2">
                    {t('compliance.last')}: {new Date(lastViolation).toLocaleTimeString()}
                </div>
            )}
        </div>
    );
};
```

#### Step 4: Create PositionHistogram Chart

**Location**: Same file (`ComplianceMetrics.tsx`)

```typescript
interface PositionHistogramProps {
    data: Array<{ index: number; count: number }>;
}

const PositionHistogram: React.FC<PositionHistogramProps> = ({ data }) => {
    const maxCount = Math.max(...data.map(d => d.count), 1);

    return (
        <div className="space-y-2">
            {data.map(entry => {
                const percentage = (entry.count / maxCount) * 100;

                return (
                    <div key={entry.index} className="flex items-center gap-3">
                        <span className="text-xs text-gray-500 dark:text-gray-400 w-16">
                            Index {entry.index}:
                        </span>
                        <div className="flex-1 bg-gray-100 dark:bg-base-300 rounded-full h-4 overflow-hidden">
                            <div
                                className="h-full bg-blue-500 dark:bg-blue-400 transition-all"
                                style={{ width: `${percentage}%` }}
                            />
                        </div>
                        <span className="text-xs font-medium text-gray-700 dark:text-gray-300 w-8 text-right">
                            {entry.count}
                        </span>
                    </div>
                );
            })}
        </div>
    );
};
```

#### Step 5: Create AlertPanel Component

**Location**: Same file (`ComplianceMetrics.tsx`)

```typescript
interface AlertPanelProps {
    budgetLevel: 'GREEN' | 'YELLOW' | 'RED';
    positionLevel: 'GREEN' | 'YELLOW' | 'RED';
    budgetRate: number;
    positionRate: number;
}

const AlertPanel: React.FC<AlertPanelProps> = ({
    budgetLevel,
    positionLevel,
    budgetRate,
    positionRate,
}) => {
    const { t } = useTranslation();

    const alerts = [];

    if (budgetLevel === 'RED') {
        alerts.push({
            type: 'error',
            message: t('compliance.alerts.budget_red', { rate: budgetRate.toFixed(4) }),
            recommendation: t('compliance.alerts.budget_red_fix'),
        });
    } else if (budgetLevel === 'YELLOW') {
        alerts.push({
            type: 'warning',
            message: t('compliance.alerts.budget_yellow', { rate: budgetRate.toFixed(4) }),
            recommendation: t('compliance.alerts.budget_yellow_fix'),
        });
    }

    if (positionLevel === 'RED') {
        alerts.push({
            type: 'error',
            message: t('compliance.alerts.position_red', { rate: positionRate.toFixed(4) }),
            recommendation: t('compliance.alerts.position_red_fix'),
        });
    } else if (positionLevel === 'YELLOW') {
        alerts.push({
            type: 'warning',
            message: t('compliance.alerts.position_yellow', { rate: positionRate.toFixed(4) }),
            recommendation: t('compliance.alerts.position_yellow_fix'),
        });
    }

    if (alerts.length === 0) {
        return (
            <div className="flex items-center gap-2 p-3 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg">
                <CheckCircle size={20} className="text-green-600 dark:text-green-400" />
                <span className="text-sm text-green-800 dark:text-green-300">
                    {t('compliance.alerts.all_green')}
                </span>
            </div>
        );
    }

    return (
        <div className="space-y-2">
            {alerts.map((alert, index) => (
                <div
                    key={index}
                    className={`p-3 rounded-lg border ${
                        alert.type === 'error'
                            ? 'bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800'
                            : 'bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-800'
                    }`}
                >
                    <div className="flex items-start gap-2">
                        <AlertCircle
                            size={20}
                            className={
                                alert.type === 'error'
                                    ? 'text-red-600 dark:text-red-400'
                                    : 'text-yellow-600 dark:text-yellow-400'
                            }
                        />
                        <div className="flex-1">
                            <p className={`text-sm font-medium ${
                                alert.type === 'error'
                                    ? 'text-red-800 dark:text-red-300'
                                    : 'text-yellow-800 dark:text-yellow-300'
                            }`}>
                                {alert.message}
                            </p>
                            <p className={`text-xs mt-1 ${
                                alert.type === 'error'
                                    ? 'text-red-700 dark:text-red-400'
                                    : 'text-yellow-700 dark:text-yellow-400'
                            }`}>
                                {alert.recommendation}
                            </p>
                        </div>
                    </div>
                </div>
            ))}
        </div>
    );
};
```

#### Step 6: Add Tauri Command for Reset

**Location**: `src-tauri/src/commands/proxy.rs` (add to existing file)

**Add Reset Command**:
```rust
/// Reset violation metrics (for testing and manual reset)
#[tauri::command]
pub async fn reset_violation_metrics() -> Result<(), String> {
    if let Some(monitor) = crate::proxy::PROXY_MONITOR.get() {
        monitor.reset_violation_metrics().await;
        Ok(())
    } else {
        Err("Proxy monitor not initialized".to_string())
    }
}
```

**Add to Handler Registration** (`lib.rs`):
```rust
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    get_violation_metrics,
    reset_violation_metrics,  // 🆕
])
```

#### Step 7: Add Violation Event Emission

**Location**: `src-tauri/src/proxy/monitor.rs` (in ViolationMetrics methods)

**Update record_budget_violation**:
```rust
impl ProxyMonitor {
    pub async fn record_budget_violation(&self) {
        // ... existing code ...

        // 🆕 Emit event for real-time dashboard updates
        if let Some(app) = &self.app_handle {
            let _ = app.emit("proxy://violation", "budget");
        }
    }

    pub async fn record_position_violation(&self, index: usize, role: &str) {
        // ... existing code ...

        // 🆕 Emit event for real-time dashboard updates
        if let Some(app) = &self.app_handle {
            let _ = app.emit("proxy://violation", "position");
        }
    }

    pub async fn reset_violation_metrics(&self) {
        // Reset all violation counters
        {
            let mut stats = self.stats.write().await;
            stats.thinking_budget_violations = 0;
            stats.thinking_position_violations = 0;
            stats.thinking_position_violations_user = 0;
            stats.thinking_position_violations_model = 0;
        }

        // Reset detailed metrics
        {
            self.violation_metrics.position_violation_indices.write().await.clear();
            self.violation_metrics.budget_violation_timestamps.write().await.clear();
            self.violation_metrics.position_violation_timestamps.write().await.clear();
        }

        // Persist to database
        if let Err(e) = crate::modules::proxy_db::reset_violation_stats() {
            tracing::error!("Failed to reset violation stats in DB: {}", e);
        }

        // 🆕 Emit reset event
        if let Some(app) = &self.app_handle {
            let _ = app.emit("proxy://violation-reset", ());
        }
    }
}
```

#### Step 8: Integrate into Monitor Page

**Location**: `src/pages/Monitor.tsx`

**Add ComplianceMetrics Section**:
```typescript
import { ComplianceMetrics } from '../components/proxy/ComplianceMetrics';

const Monitor: React.FC = () => {
    const navigate = useNavigate();
    const { t } = useTranslation();
    const [showCompliance, setShowCompliance] = useState(true);

    return (
        <div className="h-full w-full flex flex-col bg-gray-50 dark:bg-base-200">
            {/* Header */}
            <div className="bg-white dark:bg-base-100 border-b border-gray-200 dark:border-base-300 px-4 py-3 flex items-center gap-4 shadow-sm z-10">
                {/* ... existing header ... */}

                {/* 🆕 Toggle compliance view */}
                <div className="ml-auto">
                    <button
                        onClick={() => setShowCompliance(!showCompliance)}
                        className="btn btn-sm btn-ghost"
                    >
                        {showCompliance ? t('monitor.hide_compliance') : t('monitor.show_compliance')}
                    </button>
                </div>
            </div>

            {/* Main Content */}
            <div className="flex-1 p-4 overflow-auto space-y-4">
                {/* 🆕 Compliance Metrics Section */}
                {showCompliance && (
                    <ComplianceMetrics className="mb-4" />
                )}

                {/* Existing ProxyMonitor */}
                <ProxyMonitor className="h-[600px] border border-gray-200 dark:border-base-300 shadow-md" />
            </div>
        </div>
    );
};
```

#### Step 9: Add Translations

**Location**: `src/locales/en/translation.json`, `src/locales/zh/translation.json`

**Add Compliance Keys**:
```json
{
  "compliance": {
    "title": "Compliance Monitoring",
    "score": "Compliance Score",
    "budget_violations": "Budget Violations",
    "position_violations": "Position Violations",
    "position_histogram": "Position Violation Distribution",
    "user_messages": "User messages",
    "model_messages": "Model messages",
    "last": "Last violation",
    "refresh": "Refresh metrics",
    "reset": "Reset metrics",
    "export": "Export report",
    "confirm_reset": "Are you sure you want to reset all violation metrics? This cannot be undone.",
    "no_data": "No compliance data available",
    "alerts": {
      "all_green": "All thresholds GREEN - Compliance excellent",
      "budget_red": "Budget violations rate ({rate}/sec) exceeds RED threshold (0.28/sec)",
      "budget_red_fix": "Recommendation: Review client configurations to ensure maxOutputTokens > thinkingBudget + 100",
      "budget_yellow": "Budget violations rate ({rate}/sec) exceeds YELLOW threshold (0.03/sec)",
      "budget_yellow_fix": "Recommendation: Monitor client configurations and consider warning users",
      "position_red": "Position violations rate ({rate}/sec) exceeds RED threshold (0.28/sec)",
      "position_red_fix": "Recommendation: Review client implementations to ensure thinking blocks are sent first",
      "position_yellow": "Position violations rate ({rate}/sec) exceeds YELLOW threshold (0.03/sec)",
      "position_yellow_fix": "Recommendation: Monitor client behavior and investigate message construction"
    }
  },
  "monitor": {
    "show_compliance": "Show Compliance",
    "hide_compliance": "Hide Compliance"
  }
}
```

---

## Acceptance Criteria

### AC-1: ComplianceMetrics Component Created ✅

**Given** the frontend codebase
**When** I create ComplianceMetrics component
**Then** it should:
- ✅ Display compliance score (calculated)
- ✅ Show budget violation card
- ✅ Show position violation card
- ✅ Display position histogram
- ✅ Show alert panel
- ✅ Provide refresh/reset/export actions

### AC-2: Real-Time Updates Working ✅

**Given** the compliance dashboard
**When** a violation occurs
**Then** it should:
- ✅ Listen to `proxy://violation` events
- ✅ Automatically reload metrics
- ✅ Update UI without manual refresh
- ✅ Show latest violation timestamp

**Validation**:
```typescript
// Test: Trigger violation, verify dashboard updates
await triggerBudgetViolation();
await waitFor(1000);
const metrics = await invoke('get_violation_metrics');
expect(metrics.total_budget_violations).toBe(1);
```

### AC-3: Alert System Functional ✅

**Given** violation rates exceeding thresholds
**When** dashboard displays alerts
**Then** it should:
- ✅ Show GREEN for rate <0.03/sec
- ✅ Show YELLOW for rate 0.03-0.28/sec
- ✅ Show RED for rate >0.28/sec
- ✅ Display actionable recommendations

**Validation**: Manual testing with different violation rates

### AC-4: Compliance Score Calculation ✅

**Given** violation metrics
**When** I calculate compliance score
**Then** it should:
- ✅ Start at 100%
- ✅ Deduct based on violation rates
- ✅ Budget penalty: -0.1% per 0.01/sec over GREEN
- ✅ Position penalty: -0.1% per 0.01/sec over GREEN
- ✅ Never go below 0%

**Validation**:
```typescript
#[test]
fn test_compliance_score_calculation() {
    // Perfect: No violations
    let metrics1 = createMetrics({ budget_rate: 0.0, position_rate: 0.0 });
    assert_eq!(calculateComplianceScore(metrics1), 100.0);

    // Green: Below threshold
    let metrics2 = createMetrics({ budget_rate: 0.02, position_rate: 0.01 });
    assert_eq!(calculateComplianceScore(metrics2), 100.0);

    // Yellow: At threshold
    let metrics3 = createMetrics({ budget_rate: 0.05, position_rate: 0.10 });
    // Penalty: (0.05-0.03)*10 + (0.10-0.03)*10 = 0.2 + 0.7 = 0.9%
    assert_eq!(calculateComplianceScore(metrics3), 99.1);

    // Red: Exceeded
    let metrics4 = createMetrics({ budget_rate: 0.50, position_rate: 0.50 });
    // Penalty: (0.50-0.03)*10 + (0.50-0.03)*10 = 4.7 + 4.7 = 9.4%
    assert_eq!(calculateComplianceScore(metrics4), 90.6);
}
```

### AC-5: Position Histogram Display ✅

**Given** position violation indices
**When** dashboard displays histogram
**Then** it should:
- ✅ Show bar chart with violation count per index
- ✅ Normalize bars to max count
- ✅ Display index labels clearly
- ✅ Show counts on right side

### AC-6: Reset Functionality ✅

**Given** the reset button
**When** I click and confirm
**Then** it should:
- ✅ Call `reset_violation_metrics` command
- ✅ Clear all violation counters
- ✅ Clear histogram data
- ✅ Update dashboard to show zeros
- ✅ Require confirmation dialog

### AC-7: Export Report Functionality ✅

**Given** the export button
**When** I click it
**Then** it should:
- ✅ Generate markdown report
- ✅ Include compliance score
- ✅ Include all violation metrics
- ✅ Include histogram data
- ✅ Include recommendations
- ✅ Download as .md file

### AC-8: Integration with Monitor Page ✅

**Given** the Monitor page
**When** I open it
**Then** it should:
- ✅ Show ComplianceMetrics above ProxyMonitor
- ✅ Allow toggling compliance view
- ✅ Maintain existing ProxyMonitor functionality
- ✅ Responsive layout

### AC-9: Translations Complete ✅

**Given** the dashboard
**When** I switch language
**Then** it should:
- ✅ Support English translations
- ✅ Support Chinese translations
- ✅ All labels translated
- ✅ All alerts translated
- ✅ All recommendations translated

### AC-10: 100% Compliance Validated ✅

**Given** all Epic-003 stories implemented
**When** I check compliance dashboard
**Then** it should:
- ✅ Show compliance score ≥99.5%
- ✅ Validate all 8 gaps closed
- ✅ Confirm anti-detection effective
- ✅ Display GREEN alerts

---

## Testing Strategy

### Component Tests (4 tests)

**File**: `src/components/proxy/__tests__/ComplianceMetrics.test.tsx`

```typescript
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { ComplianceMetrics } from '../ComplianceMetrics';
import { vi } from 'vitest';

vi.mock('../../utils/request', () => ({
    request: vi.fn(),
}));

describe('ComplianceMetrics', () => {
    test('displays compliance score correctly', async () => {
        const mockMetrics = {
            total_budget_violations: 10,
            total_position_violations: 5,
            budget_violation_rate: 0.02,
            position_violation_rate: 0.01,
            budget_alert_level: 'GREEN',
            position_alert_level: 'GREEN',
            // ...
        };

        vi.mocked(request).mockResolvedValue(mockMetrics);

        render(<ComplianceMetrics />);

        await waitFor(() => {
            expect(screen.getByText('100.00%')).toBeInTheDocument();
        });
    });

    test('shows alerts for YELLOW thresholds', async () => {
        const mockMetrics = {
            // ... metrics with YELLOW alert levels
            budget_alert_level: 'YELLOW',
            budget_violation_rate: 0.15,
        };

        vi.mocked(request).mockResolvedValue(mockMetrics);

        render(<ComplianceMetrics />);

        await waitFor(() => {
            expect(screen.getByText(/YELLOW/)).toBeInTheDocument();
            expect(screen.getByText(/0.15\/sec/)).toBeInTheDocument();
        });
    });

    test('reset button clears metrics', async () => {
        const mockMetrics = { /* ... */ };
        vi.mocked(request).mockResolvedValue(mockMetrics);

        render(<ComplianceMetrics />);

        const resetButton = screen.getByTitle(/reset/i);
        fireEvent.click(resetButton);

        // Confirm dialog
        window.confirm = vi.fn(() => true);

        await waitFor(() => {
            expect(request).toHaveBeenCalledWith('reset_violation_metrics');
        });
    });

    test('export generates markdown report', async () => {
        const mockMetrics = { /* ... */ };
        vi.mocked(request).mockResolvedValue(mockMetrics);

        render(<ComplianceMetrics />);

        const exportButton = screen.getByTitle(/export/i);
        fireEvent.click(exportButton);

        // Verify download triggered
        // (implementation depends on test environment)
    });
});
```

### Integration Tests (2 tests)

**File**: `src-tauri/tests/compliance_dashboard_integration_test.rs`

```rust
#[tokio::test]
async fn test_compliance_dashboard_end_to_end() {
    // 1. Trigger violations
    let _response = trigger_budget_violation().await;
    let _response = trigger_position_violation().await;

    // 2. Get metrics via Tauri command
    let metrics: DetailedViolationMetrics =
        invoke_command("get_violation_metrics").await.unwrap();

    // 3. Verify metrics
    assert_eq!(metrics.total_budget_violations, 1);
    assert_eq!(metrics.total_position_violations, 1);

    // 4. Reset metrics
    invoke_command("reset_violation_metrics").await.unwrap();

    // 5. Verify reset
    let metrics_after: DetailedViolationMetrics =
        invoke_command("get_violation_metrics").await.unwrap();
    assert_eq!(metrics_after.total_budget_violations, 0);
    assert_eq!(metrics_after.total_position_violations, 0);
}

#[tokio::test]
async fn test_real_time_violation_events() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    // Listen for violation events
    let _listener = listen_to_event("proxy://violation", move |event| {
        tx.try_send(event).ok();
    });

    // Trigger violation
    trigger_budget_violation().await;

    // Wait for event
    let event = tokio::time::timeout(
        tokio::time::Duration::from_secs(2),
        rx.recv()
    ).await.unwrap();

    assert!(event.is_some());
}
```

### Manual Testing Checklist

- [ ] **Dashboard Display**:
  - [ ] Open Monitor page
  - [ ] Verify ComplianceMetrics section visible
  - [ ] Verify compliance score displayed
  - [ ] Verify violation cards show zeros initially

- [ ] **Real-Time Updates**:
  - [ ] Trigger budget violation (send request with maxTokens <= budget)
  - [ ] Verify dashboard updates without refresh
  - [ ] Verify counter increments
  - [ ] Verify rate updates

- [ ] **Alert System**:
  - [ ] Trigger multiple violations rapidly
  - [ ] Verify YELLOW alert appears when rate >0.03/sec
  - [ ] Verify RED alert appears when rate >0.28/sec
  - [ ] Verify recommendations displayed

- [ ] **Histogram**:
  - [ ] Trigger position violations at different indices
  - [ ] Verify histogram displays correctly
  - [ ] Verify bar widths proportional
  - [ ] Verify counts accurate

- [ ] **Reset Functionality**:
  - [ ] Click reset button
  - [ ] Confirm dialog
  - [ ] Verify all counters reset to zero
  - [ ] Verify histogram clears

- [ ] **Export Report**:
  - [ ] Click export button
  - [ ] Verify markdown file downloads
  - [ ] Open file and verify content
  - [ ] Verify all metrics included

- [ ] **Translations**:
  - [ ] Switch to English
  - [ ] Verify all labels translated
  - [ ] Switch to Chinese
  - [ ] Verify all labels translated

- [ ] **100% Compliance Validation**:
  - [ ] With zero violations, verify score = 100%
  - [ ] With low violations, verify score ≥99.5%
  - [ ] Verify GREEN alerts

---

## Definition of Done

### Code Quality
- [ ] ✅ ComplianceMetrics component implemented
- [ ] ✅ ViolationCard sub-component implemented
- [ ] ✅ PositionHistogram sub-component implemented
- [ ] ✅ AlertPanel sub-component implemented
- [ ] ✅ Compliance score calculation function
- [ ] ✅ Report generation function
- [ ] ✅ Reset Tauri command added
- [ ] ✅ Event emission added
- [ ] ✅ Monitor page integrated
- [ ] ✅ All TypeScript types defined
- [ ] ✅ Code follows React best practices
- [ ] ✅ No ESLint warnings

### Testing
- [ ] ✅ 4 component tests passing
- [ ] ✅ 2 integration tests passing
- [ ] ✅ Manual testing checklist completed
- [ ] ✅ Real-time updates validated
- [ ] ✅ Alert system validated

### Translations
- [ ] ✅ English translations added
- [ ] ✅ Chinese translations added
- [ ] ✅ All keys translated
- [ ] ✅ Both languages tested

### Validation
- [ ] ✅ All 10 AC validated
- [ ] ✅ Phase 4 success criteria met
- [ ] ✅ 100% compliance validated via dashboard
- [ ] ✅ Real-time metrics working
- [ ] ✅ Detection alerts working

### Epic Completion
- [ ] ✅ Story marked as completed
- [ ] ✅ Epic-003 marked as complete
- [ ] ✅ All 12 stories done
- [ ] ✅ 100% compliance achieved and validated
- [ ] ✅ Epic-level Definition of Done complete

---

## Dependencies

### Upstream Dependencies
- ✅ **Story-003-08**: Enhanced Violation Metrics (provides Tauri command)
- ✅ **Story-003-08-ADDENDUM**: Mapper → Monitor access (enables metrics collection)
- ✅ **All Previous Stories**: Foundation for 100% compliance

**Why**: This story consumes metrics from Story-003-08.

### Infrastructure Dependencies
- **Existing**: ProxyMonitor component and Monitor page
- **Existing**: Tauri event system (`listen`, `emit`)
- **Existing**: Translation system (i18next)
- **New**: `get_violation_metrics` command (from Story-003-08)
- **New**: `reset_violation_metrics` command (this story)

---

## Risk Analysis

### Risk #1: Complex UI Implementation
**Severity**: LOW
**Probability**: MEDIUM
**Impact**: Dashboard doesn't render correctly

**Mitigation**:
- ✅ Reuse existing ProxyMonitor patterns
- ✅ Simple component structure (cards + charts)
- ✅ Component testing
- ✅ Use DaisyUI for styling

**Contingency**: Simplify UI to basic metrics display if complex charts problematic.

### Risk #2: Real-Time Updates Performance
**Severity**: LOW
**Probability**: LOW
**Impact**: Dashboard sluggish with high violation rates

**Mitigation**:
- ✅ Debounce event listeners (max 1 update/sec)
- ✅ Batch updates if multiple violations
- ✅ Limit histogram to 7 buckets
- ✅ Optimize React re-renders

**Measurement**: Test with 10+ violations/sec.

### Risk #3: Translation Coverage
**Severity**: LOW
**Probability**: LOW
**Impact**: Missing translations in UI

**Mitigation**:
- ✅ Add all keys to both en and zh files
- ✅ Use i18next fallback
- ✅ Test both languages manually
- ✅ Review translation completeness

**Contingency**: English-only for v1, add Chinese later if needed.

---

## Implementation Notes

### Design Decisions

**1. Why Separate ComplianceMetrics Component?**
- ✅ **Separation of Concerns**: Compliance separate from request logs
- ✅ **Reusability**: Can add to Dashboard page later
- ✅ **Toggle-able**: User can hide/show compliance view
- ✅ **Maintainability**: Easier to update compliance UI independently

**2. Why Client-Side Score Calculation?**
- ✅ **Performance**: No backend computation needed
- ✅ **Flexibility**: Easy to adjust scoring formula
- ✅ **Real-time**: Instant updates on UI
- ✅ **Simple**: Straightforward TypeScript function

**3. Why Markdown Export Format?**
- ✅ **Human-Readable**: Easy to read and share
- ✅ **Version Control**: Can track reports in git
- ✅ **Simple**: No complex rendering needed
- ✅ **Standard**: Markdown widely supported

**4. Why Alert Thresholds (0.03, 0.28)?**
- ✅ **Based on Story-003-08**: Defined thresholds
- ✅ **GREEN (<0.03)**: ~1 violation per 30 seconds (acceptable)
- ✅ **YELLOW (0.03-0.28)**: ~1 violation per 3-30 seconds (warning)
- ✅ **RED (>0.28)**: >1 violation per 3 seconds (critical)

### Performance Considerations

**Impact**: NEGLIGIBLE
- ✅ Metrics fetch: <10ms (in-memory data)
- ✅ Event listeners: Minimal overhead
- ✅ UI rendering: Standard React performance
- ✅ Score calculation: O(1) simple arithmetic

**Optimization**:
- Debounce event handlers if needed
- React.memo for sub-components
- Lazy load histogram chart if large

### Future Enhancements

**Post-Epic Improvements**:
- Add trend charts (violations over time)
- Add compliance history tracking
- Add export to JSON/CSV formats
- Add dashboard to Dashboard page (not just Monitor)
- Add notification system (desktop notifications)
- Add compliance threshold configuration

---

## File Impact Analysis

### New Files

| File | Lines | Description |
|------|-------|-------------|
| `src/types/compliance.ts` | ~50 | TypeScript types for metrics |
| `src/components/proxy/ComplianceMetrics.tsx` | ~350 | Main dashboard component |
| `src/components/proxy/__tests__/ComplianceMetrics.test.tsx` | ~150 | Component tests |
| `tests/compliance_dashboard_integration_test.rs` | ~80 | Integration tests |

### Modified Files

| File | Lines Changed | Change Type | Description |
|------|---------------|-------------|-------------|
| `src/pages/Monitor.tsx` | +15 | Addition | Integrate ComplianceMetrics |
| `src/locales/en/translation.json` | +25 | Addition | English translations |
| `src/locales/zh/translation.json` | +25 | Addition | Chinese translations |
| `src-tauri/src/commands/proxy.rs` | +15 | Addition | Reset command |
| `src-tauri/src/proxy/monitor.rs` | +30 | Addition | Reset method, event emission |
| `src-tauri/src/lib.rs` | +1 | Modification | Register reset command |

**Total Changes**:
- **Frontend Code**: ~400 lines
- **Backend Code**: ~45 lines
- **Test Code**: ~230 lines
- **Translations**: ~50 lines
- **Total**: ~725 lines

---

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2026-01-10 | Story created with comprehensive dashboard design | BMad Master |

---

## References

### Dependency Stories
- **Story-003-08**: `docs/stories/Story-003-08-enhanced-violation-metrics.md`
  - ViolationMetrics structure
  - Tauri command: `get_violation_metrics`
  - Frontend integration types

- **Story-003-08-ADDENDUM**: `docs/stories/Story-003-08-ADDENDUM-mapper-monitor-access.md`
  - Mapper → Monitor access pattern
  - Enables metrics collection

### Current Implementation
- **Monitor Page**: `src/pages/Monitor.tsx`
- **ProxyMonitor Component**: `src/components/proxy/ProxyMonitor.tsx`
- **Existing Commands**: `get_proxy_logs`, `get_proxy_stats`, `clear_proxy_logs`

### Epic Success Criteria
- **Epic-003**: Lines 153-157 (Phase 4)
  - Real-time compliance metrics ← **This Story**
  - Detection failure alerts ← **This Story**
  - 100% compliance validated ← **This Story**
