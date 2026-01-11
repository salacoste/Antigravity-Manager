# Compliance Dashboard User Guide

**Document Version**: 1.0
**Last Updated**: 2026-01-11
**Target Audience**: Users, Administrators, Developers
**Related Epic**: [Epic-003](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md)

---

## Table of Contents

1. [Overview](#overview)
2. [Getting Started](#getting-started)
3. [Dashboard Components](#dashboard-components)
4. [Understanding Metrics](#understanding-metrics)
5. [Alert System](#alert-system)
6. [Actions & Controls](#actions--controls)
7. [Best Practices](#best-practices)
8. [FAQ](#faq)

---

## Overview

### What is the Compliance Dashboard?

The Compliance Dashboard is a **real-time monitoring tool** that validates 100% compliance with Google Antigravity v1.13.3 specifications for Claude 4.5 Sonnet Thinking mode. It tracks violation metrics, analyzes patterns, and provides actionable alerts to ensure optimal API proxy performance.

### Key Features

- ✅ **Real-time Compliance Score** (0-100% scale)
- ✅ **Violation Tracking** (Budget + Position violations)
- ✅ **Visual Analytics** (Histogram, cards, alerts)
- ✅ **Event-driven Updates** (Automatic refresh on violations)
- ✅ **Export Functionality** (Markdown compliance reports)
- ✅ **Reset Controls** (Clear historical data)

### When to Use

- **Proactive Monitoring**: Regular health checks during normal operation
- **Troubleshooting**: Investigating API rejection or blocking issues
- **Integration Testing**: Validating new client implementations
- **Compliance Audits**: Generating compliance reports for stakeholders

---

## Getting Started

### Accessing the Dashboard

1. **Navigate to Monitor Page**
   - Click **"API Proxy"** in sidebar navigation
   - Click **"Open Monitor"** button
   - OR directly navigate to `/monitor` route

2. **Toggle Compliance View**
   - Look for **"Compliance"** button in top-right header
   - Button states:
     - **Blue (Primary)**: Dashboard visible
     - **Gray (Ghost)**: Dashboard hidden
   - Click to show/hide compliance dashboard

### Dashboard Layout

```
┌─────────────────────────────────────────────────┐
│ Header: "API Monitor Dashboard"                 │
│ [← Back] [Compliance ✓]                        │
├─────────────────────────────────────────────────┤
│                                                  │
│  Compliance Monitoring Dashboard                │
│  ┌──────────────────────────────────────────┐  │
│  │ Compliance Score: 99.87%         [✓]    │  │
│  ├──────────────────────────────────────────┤  │
│  │ Budget Violations  │ Position Violations │  │
│  │ Count: 5           │ Count: 3           │  │
│  │ Rate: 0.02/sec     │ Rate: 0.01/sec     │  │
│  │ Status: GREEN      │ Status: GREEN      │  │
│  ├──────────────────────────────────────────┤  │
│  │ Position Histogram                       │  │
│  │ [Bar chart showing violation distribution]│  │
│  └──────────────────────────────────────────┘  │
│                                                  │
│  Request Monitor (Existing)                     │
│  [Request logs table...]                        │
│                                                  │
└─────────────────────────────────────────────────┘
```

### Initial Setup

**No configuration required** - the dashboard works out-of-the-box:

1. Start proxy service (if not already running)
2. Navigate to Monitor page
3. Ensure "Compliance" toggle is enabled
4. Dashboard loads automatically with current metrics

---

## Dashboard Components

### 1. Compliance Score Card

**Location**: Top section of dashboard

**What It Shows**:
- **Compliance Score**: 0-100% scale (larger number, green color)
- **Status Icon**:
  - ✅ Green CheckCircle: Score ≥99.5% (Excellent)
  - ⚠️ Yellow AlertCircle: Score <99.5% (Needs attention)

**Score Calculation**:
```typescript
Starting Score: 100%

Budget Violations Penalty:
- For each 0.01 violations/sec over 0.03 threshold → -0.1%

Position Violations Penalty:
- For each 0.01 violations/sec over 0.03 threshold → -0.1%

Final Score = max(0%, 100% - penalties)
```

**Example**:
- Budget rate: 0.05/sec → Overage: 0.02 → Penalty: 0.2%
- Position rate: 0.04/sec → Overage: 0.01 → Penalty: 0.1%
- **Final Score**: 100% - 0.2% - 0.1% = 99.7% ✅

### 2. Violation Cards

**Location**: Middle section, two cards side-by-side

#### Budget Violations Card

**Displays**:
- **Total Count**: Lifetime budget violation count
- **Rate**: Violations per second (4 decimal places)
- **Alert Level**: GREEN/YELLOW/RED badge
- **No breakdown** (budget violations don't have user/model split)

**What Causes Budget Violations**:
- Client sets `maxOutputTokens` too low
- `maxOutputTokens < thinkingBudget + 100`
- Example: thinkingBudget=4096, maxOutputTokens=4000 ❌

**Resolution**:
- Review client configurations
- Ensure `maxOutputTokens > thinkingBudget + 100`
- See [Troubleshooting Guide](./compliance-troubleshooting-guide.md)

#### Position Violations Card

**Displays**:
- **Total Count**: Lifetime position violation count
- **Rate**: Violations per second (4 decimal places)
- **Alert Level**: GREEN/YELLOW/RED badge
- **Breakdown**:
  - User messages: Violations in user content
  - Model messages: Violations in assistant content

**What Causes Position Violations**:
- Thinking blocks sent AFTER text content
- Incorrect message ordering in client implementation
- Example: `[{text: "..."}, {thinking: "..."}]` ❌

**Resolution**:
- Review client message construction
- Ensure thinking blocks come FIRST
- Correct order: `[{thinking: "..."}, {text: "..."}]` ✅

### 3. Position Histogram

**Location**: Below violation cards (only shown if violations exist)

**What It Shows**:
- **X-axis**: Index buckets (1, 2, 3, 5, 10, 20, 50)
- **Y-axis**: Violation count per bucket
- **Visualization**: Horizontal bar chart (blue bars)

**How to Read**:
- **Index 1**: First content block (should be thinking)
- **Index 2**: Second content block
- **Higher indices**: Later in message (rare for thinking blocks)

**Interpretation**:
```
Index 1: ████████████ 50   ← Most violations (thinking at index 1 = wrong)
Index 2: ██           8    ← Some violations (thinking at index 2)
Index 3: █            3    ← Few violations
...
```

**Good Pattern** (No violations):
- All bars empty or minimal
- Compliance score ≥99.5%

**Bad Pattern** (Many violations):
- Large bar at Index 1 or 2
- Indicates systemic client implementation issue

### 4. Alert Panel

**Location**: Bottom section (only shown if alerts exist)

**Alert Levels**:

#### 🟢 GREEN - All Clear
- Budget rate: ≤0.03/sec
- Position rate: ≤0.03/sec
- **Message**: "All thresholds GREEN - Compliance excellent"
- **Action**: None required, continue monitoring

#### 🟡 YELLOW - Warning
- Budget rate: 0.03-0.28/sec
- Position rate: 0.03-0.28/sec
- **Message**: Rate in YELLOW range with specific value
- **Recommendation**: Monitor and consider adjustments
- **Action**: Review configurations, not urgent

#### 🔴 RED - Critical
- Budget rate: >0.28/sec
- Position rate: >0.28/sec
- **Message**: Rate exceeds RED threshold with specific value
- **Recommendation**: Immediate review required
- **Action**: Investigate and fix immediately

**Alert Display**:
```
┌──────────────────────────────────────────────┐
│ ⚠️ Budget violations rate (0.35/sec) exceeds │
│    RED threshold (>0.28/sec)                 │
│                                              │
│ 💡 Review client configurations to ensure    │
│    maxOutputTokens > thinkingBudget + 100   │
└──────────────────────────────────────────────┘
```

---

## Understanding Metrics

### Compliance Score Deep Dive

**Formula**:
```
score = 100.0

// Budget penalty
budget_overage = max(0, budget_rate - 0.03)
budget_penalty = budget_overage * 10  // -0.1% per 0.01/sec
score -= budget_penalty

// Position penalty
position_overage = max(0, position_rate - 0.03)
position_penalty = position_overage * 10  // -0.1% per 0.01/sec
score -= position_penalty

// Cap at 0%
final_score = max(0, score)
```

**Examples**:

| Budget Rate | Position Rate | Score | Status |
|-------------|---------------|-------|--------|
| 0.00/sec | 0.00/sec | 100.00% | Perfect ✅ |
| 0.03/sec | 0.03/sec | 100.00% | Excellent ✅ |
| 0.05/sec | 0.04/sec | 99.70% | Good ✅ |
| 0.10/sec | 0.10/sec | 98.60% | Acceptable ⚠️ |
| 0.30/sec | 0.30/sec | 94.60% | Poor ❌ |
| 1.00/sec | 1.00/sec | 80.60% | Critical ❌ |

### Violation Rates

**How Rates are Calculated**:

The system tracks timestamps of all violations and calculates rate using a sliding window:

```rust
// Pseudo-code
recent_violations = violations_in_last_60_seconds
rate = recent_violations / 60.0  // violations per second
```

**Rate Interpretation**:

- **0.00-0.03/sec**: GREEN zone, excellent compliance
- **0.03-0.28/sec**: YELLOW zone, minor issues
- **>0.28/sec**: RED zone, immediate action required

**Important Notes**:
- Rates update in real-time (every violation triggers recalculation)
- Old violations (>60s) automatically decay from rate calculation
- Rate stabilizes after 60 seconds of operation

### Histogram Buckets

**Bucket Definitions**:
```
Bucket 1: index = 1 (first content block)
Bucket 2: index = 2 (second content block)
Bucket 3: index = 3 (third content block)
Bucket 5: index = 5 (fifth content block)
Bucket 10: index = 10 (tenth content block)
Bucket 20: index = 20 (twentieth content block)
Bucket 50: index ≥ 50 (very late content blocks)
```

**Why These Buckets?**:
- Thinking blocks should be at index 0 (before all content)
- Index 1+ indicates thinking block was placed AFTER content
- Higher indices are exponentially worse (indicates severe ordering issues)

---

## Alert System

### Alert Thresholds

**Configured Thresholds**:
```typescript
const THRESHOLDS = {
  green_max: 0.03,    // ≤0.03/sec = GREEN
  yellow_max: 0.28,   // 0.03-0.28/sec = YELLOW
  // >0.28/sec = RED (implicit)
};
```

**Threshold Rationale**:
- **0.03/sec**: Allows ~2 violations/minute (normal client testing)
- **0.28/sec**: Allows ~17 violations/minute (development debugging)
- **>0.28/sec**: Production issue threshold (immediate attention)

### Alert Actions

#### Budget RED Alert
**Message**: "Budget violations rate (X/sec) exceeds RED threshold (>0.28/sec)"

**Recommended Actions**:
1. Check client configurations immediately
2. Verify `maxOutputTokens` settings
3. Ensure formula: `maxOutputTokens > thinkingBudget + 100`
4. Review recent code changes to client
5. Check if multiple clients are misconfigured

**Common Causes**:
- Hardcoded low `maxOutputTokens` value
- Client using default values without thinking mode awareness
- Configuration not updated after thinking mode enabled

#### Position RED Alert
**Message**: "Position violations rate (X/sec) exceeds RED threshold (>0.28/sec)"

**Recommended Actions**:
1. Review client message construction logic
2. Verify thinking blocks are sent FIRST
3. Check message array ordering
4. Review Claude API integration code
5. Test with single request to isolate issue

**Common Causes**:
- Incorrect message array construction
- Client appending thinking blocks instead of prepending
- Middleware reordering message content
- Framework-specific message serialization issues

#### YELLOW Alerts
**Action**: Monitor, not urgent
- Review configurations when convenient
- Check if trend is improving or worsening
- Document any recent changes that might correlate

---

## Actions & Controls

### Refresh Metrics

**Location**: Top-right header (🔄 icon)

**What It Does**:
- Manually reloads violation metrics from backend
- Updates all dashboard components
- Does NOT clear historical data

**When to Use**:
- After making client configuration changes
- To verify current state
- If automatic updates seem delayed

**How to Use**:
1. Click refresh icon (🔄)
2. Wait 1-2 seconds for data load
3. Dashboard updates with latest metrics

### Reset Metrics

**Location**: Top-right header (🗑️ icon)

**What It Does**:
- Clears ALL violation counters (budget + position)
- Resets detailed metrics (indices, timestamps)
- Persists reset to database
- Emits `proxy://violation-reset` event

**⚠️ WARNING**: This action is IRREVERSIBLE

**When to Use**:
- After fixing client issues (fresh start for validation)
- Starting new testing phase
- Clearing test data before production deployment
- Resetting after troubleshooting session

**How to Use**:
1. Click reset icon (🗑️)
2. Confirm in dialog: "Are you sure you want to reset all violation metrics?"
3. Click "OK" to proceed
4. Dashboard reloads with zeroed metrics

**Best Practice**:
- Export report BEFORE resetting (to preserve historical data)
- Document reason for reset in change log
- Notify team if metrics are being reset in shared environment

### Export Report

**Location**: Top-right header (⬇️ icon)

**What It Does**:
- Generates Markdown compliance report
- Downloads as `.md` file to browser downloads folder
- Filename format: `compliance-report-{timestamp}.md`

**Report Contents**:
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
⚠️ Minor compliance issues detected. Review client configurations.
**Budget Violations**: Review client configurations to ensure...
**Position Violations**: Review client implementations to ensure...
```

**When to Use**:
- Before resetting metrics (preserve historical data)
- Compliance audits or stakeholder reporting
- Troubleshooting documentation
- Before/after comparison for validation

**How to Use**:
1. Click export icon (⬇️)
2. Browser downloads file automatically
3. Open in Markdown viewer or text editor

---

## Best Practices

### Monitoring Cadence

**Daily Monitoring** (Production):
- Check compliance score: Should be ≥99.5%
- Review violation rates: Should be GREEN
- Export weekly reports for auditing

**Active Development**:
- Monitor dashboard during testing
- Reset metrics after each testing cycle
- Export reports before major changes

**Post-Deployment**:
- Monitor for 24 hours after deployment
- Check for rate spikes or new violation patterns
- Verify compliance score remains stable

### Alert Response

**GREEN Status**:
- No action required
- Continue regular monitoring
- Maintain current configurations

**YELLOW Status**:
- Review configurations within 24 hours
- Monitor for trend (improving or worsening)
- Document any recent changes
- Schedule fix if rate increases

**RED Status**:
- Immediate investigation required
- Stop new deployments until resolved
- Export report for documentation
- Fix root cause before proceeding

### Testing Workflows

#### New Client Integration
1. Reset metrics (fresh baseline)
2. Run client test suite
3. Monitor dashboard in real-time
4. Export report after testing
5. Verify compliance score ≥99.5%

#### Configuration Changes
1. Export report (before state)
2. Apply configuration changes
3. Reset metrics
4. Run validation tests
5. Export report (after state)
6. Compare before/after

#### Troubleshooting
1. Export current report (preserve state)
2. Review histogram for patterns
3. Check alert recommendations
4. Apply fixes
5. Reset metrics
6. Validate improvements

---

## FAQ

### General Questions

**Q: Why is my compliance score not 100%?**

A: Even a few violations will reduce the score. The formula deducts 0.1% per 0.01 violations/sec over the GREEN threshold (0.03/sec). Example: 0.05/sec rate = 99.8% score.

**Q: What's a "good" compliance score?**

A:
- **≥99.5%**: Excellent (production-ready)
- **95-99.5%**: Good (acceptable with minor issues)
- **90-95%**: Fair (needs attention)
- **<90%**: Poor (immediate action required)

**Q: How often does the dashboard update?**

A: Automatically on every violation via `proxy://violation` event. You can also manually refresh.

### Budget Violations

**Q: What causes budget violations?**

A: When `maxOutputTokens < thinkingBudget + 100`. The proxy detects this configuration error and logs a violation.

**Q: How do I fix budget violations?**

A: Update client configuration:
```typescript
// Before (causes violations)
maxOutputTokens: 4000,
thinkingBudget: 4096

// After (correct)
maxOutputTokens: 8192,  // > 4096 + 100
thinkingBudget: 4096
```

**Q: Can I ignore budget violations?**

A: No. They indicate client misconfiguration that will cause API request failures.

### Position Violations

**Q: What causes position violations?**

A: Thinking blocks sent AFTER text content. Correct order: `[{thinking}, {text}]`. Wrong order: `[{text}, {thinking}]`.

**Q: Why is position important?**

A: Google Antigravity v1.13.3 expects thinking BEFORE content. Wrong order may trigger detection or API rejection.

**Q: How do I fix position violations?**

A: Update client message construction:
```typescript
// Before (causes violations)
const content = [
  { type: "text", text: "user message" },
  { type: "thinking", thinking: "..." }  // ❌ thinking AFTER text
];

// After (correct)
const content = [
  { type: "thinking", thinking: "..." },  // ✅ thinking FIRST
  { type: "text", text: "user message" }
];
```

### Histogram

**Q: What does "Bucket 1: 50" mean?**

A: 50 violations where thinking block was at index 1 (second position). It should be at index 0 (first position).

**Q: Why are there different buckets?**

A: To identify patterns. Consistent violations at same index suggest systematic client issue.

**Q: What if histogram is empty?**

A: Either no position violations occurred, or all violations are at index 0 (which is correct - no violation logged).

### Actions

**Q: Will resetting metrics affect my proxy service?**

A: No. Reset only clears violation counters, does not affect proxy operation or request handling.

**Q: Can I recover data after reset?**

A: No. Reset is permanent. Always export report before resetting if you need historical data.

**Q: Why is my export file empty?**

A: Ensure you have violation data before exporting. If all metrics are zero, report will be minimal.

### Troubleshooting

**Q: Dashboard shows "No compliance data available"**

A:
1. Verify proxy service is running
2. Check that monitor is initialized
3. Try refreshing metrics manually
4. Check browser console for errors

**Q: Real-time updates not working**

A:
1. Verify event listeners are active (check browser console)
2. Reload the page
3. Check that proxy service hasn't been restarted
4. Verify `enable_logging` is enabled in proxy config

**Q: Compliance score seems wrong**

A:
1. Verify violation counts are accurate
2. Recalculate manually using formula
3. Check for recent resets or service restarts
4. Export report to see detailed calculations

---

## Related Documentation

- [Compliance Troubleshooting Guide](./compliance-troubleshooting-guide.md)
- [Thinking Mode Validation Architecture](./thinking-mode-validation-architecture.md)
- [API Reference: Compliance Commands](./compliance-api-reference.md)
- [Epic-003: Claude 4.5 Sonnet Thinking Compliance](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md)
- [Story-003-12: Compliance Monitoring Dashboard](../stories/story-012-compliance-monitoring-dashboard.md)

---

## Support

For issues, questions, or feedback:
- Check [Troubleshooting Guide](./compliance-troubleshooting-guide.md)
- Review [Story-003-12 QA Report](../stories/story-012-qa-report.md)
- Open issue in project repository
