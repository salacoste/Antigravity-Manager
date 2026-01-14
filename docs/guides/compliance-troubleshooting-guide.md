# Compliance Troubleshooting Guide

**Document Version**: 1.0
**Last Updated**: 2026-01-11
**Target Audience**: Users, Developers, Support Engineers
**Related Epic**: [Epic-003](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md)

---

## Table of Contents

1. [Quick Diagnostic Checklist](#quick-diagnostic-checklist)
2. [Budget Violations](#budget-violations)
3. [Position Violations](#position-violations)
4. [Dashboard Issues](#dashboard-issues)
5. [Performance Problems](#performance-problems)
6. [Integration Issues](#integration-issues)
7. [Common Error Messages](#common-error-messages)
8. [Advanced Debugging](#advanced-debugging)

---

## Quick Diagnostic Checklist

### Step 1: Check Dashboard Status

```
✅ Dashboard visible on Monitor page
✅ Proxy service running
✅ Compliance score displaying
✅ No "No data available" message
```

**If failing**: See [Dashboard Issues](#dashboard-issues)

### Step 2: Check Violation Metrics

```
✅ Budget violations: Count and rate visible
✅ Position violations: Count and rate visible
✅ Alert level badges showing (GREEN/YELLOW/RED)
```

**If RED alerts**: Immediate action required
**If YELLOW alerts**: Review within 24 hours
**If GREEN**: Continue monitoring

### Step 3: Check Recent Activity

```
✅ Request logs showing in Monitor table
✅ Violation events emitting (check browser console)
✅ Metrics updating on new violations
```

**If stale data**: Try manual refresh

### Step 4: Validate Configuration

```
✅ Proxy config: enable_logging = true
✅ Client config: maxOutputTokens > thinkingBudget + 100
✅ Client config: Thinking blocks sent FIRST
```

**If misconfigured**: See specific violation sections below

---

## Budget Violations

### Symptom: High Budget Violation Count

**What You See**:
- Budget violations card shows high count
- Rate >0.03/sec (YELLOW or RED alert)
- Compliance score <99.5%

### Root Cause Analysis

#### Check 1: Client Configuration

**Verify**:
```typescript
// Client code
const request = {
  maxTokens: ???,  // Check this value
  thinking: {
    budget: ???,   // Check this value
    enabled: true
  }
};
```

**Diagnosis**:
```
IF maxTokens < thinking.budget + 100
THEN Budget violation will occur
```

**Example Issues**:
```typescript
// ❌ WRONG - Violates budget rule
maxTokens: 4000
thinking: { budget: 4096, enabled: true }
// 4000 < 4096 + 100 = VIOLATION

// ❌ WRONG - Hardcoded low value
maxTokens: 4096  // Default, not adjusted for thinking
thinking: { budget: 4096, enabled: true }
// 4096 < 4096 + 100 = VIOLATION

// ✅ CORRECT
maxTokens: 8192
thinking: { budget: 4096, enabled: true }
// 8192 > 4096 + 100 ✓
```

#### Check 2: Default Values

**Common Mistake**: Using framework defaults without thinking mode awareness

**Claude SDK (JavaScript)**:
```typescript
// ❌ WRONG - Uses default maxTokens
const response = await anthropic.messages.create({
  model: "claude-sonnet-4.5",
  messages: [...],
  thinking: {
    type: "enabled",
    budget: 4096
  }
  // Missing: max_tokens adjustment
});

// ✅ CORRECT
const response = await anthropic.messages.create({
  model: "claude-sonnet-4.5",
  messages: [...],
  max_tokens: 8192,  // Explicit adjustment
  thinking: {
    type: "enabled",
    budget: 4096
  }
});
```

**OpenAI Format (via proxy)**:
```typescript
// ❌ WRONG
fetch('http://localhost:8045/v1/chat/completions', {
  method: 'POST',
  body: JSON.stringify({
    model: 'claude-sonnet-4.5',
    messages: [...],
    max_tokens: 4096,  // Default, too low
    thinking: {
      enabled: true,
      budget: 4096
    }
  })
});

// ✅ CORRECT
fetch('http://localhost:8045/v1/chat/completions', {
  method: 'POST',
  body: JSON.stringify({
    model: 'claude-sonnet-4.5',
    messages: [...],
    max_tokens: 8192,  // Adjusted
    thinking: {
      enabled: true,
      budget: 4096
    }
  })
});
```

### Resolution Steps

#### Step 1: Identify Violating Clients

**Method 1: Check Request Logs**
1. Open Monitor page
2. Filter for recent requests
3. Check `max_tokens` in request details
4. Compare with `thinkingBudget`

**Method 2: Review Client Code**
1. Search codebase for "max_tokens" or "maxTokens"
2. Search for "thinking" configuration
3. Verify formula: `maxTokens > thinkingBudget + 100`

#### Step 2: Update Client Configuration

**Recommended Values**:
```typescript
const THINKING_BUDGET = 4096;
const MAX_TOKENS = THINKING_BUDGET * 2;  // 8192 (2x buffer)

// Or conservative approach
const MAX_TOKENS = THINKING_BUDGET + 1000;  // 5096 (minimum + margin)
```

**Configuration Templates**:

**Anthropic SDK**:
```typescript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic({ apiKey: '...' });

const THINKING_CONFIG = {
  budget: 4096,
  max_tokens: 8192  // 2x budget
};

const response = await anthropic.messages.create({
  model: 'claude-sonnet-4.5',
  max_tokens: THINKING_CONFIG.max_tokens,
  messages: [...],
  thinking: {
    type: 'enabled',
    budget: THINKING_CONFIG.budget
  }
});
```

**OpenAI Format (Proxy)**:
```typescript
const THINKING_CONFIG = {
  budget: 4096,
  max_tokens: 8192
};

const response = await fetch('http://localhost:8045/v1/chat/completions', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'x-api-key': 'your-api-key'
  },
  body: JSON.stringify({
    model: 'claude-sonnet-4.5',
    max_tokens: THINKING_CONFIG.max_tokens,
    messages: [...],
    thinking: {
      enabled: true,
      budget: THINKING_CONFIG.budget
    }
  })
});
```

#### Step 3: Validate Fix

1. Reset metrics in dashboard (optional - for clean validation)
2. Run test request with updated configuration
3. Check dashboard - budget violations should stop
4. Monitor for 5-10 minutes
5. Verify rate drops to 0.00/sec (GREEN)

### Prevention

**Code Review Checklist**:
- [ ] All thinking mode requests have explicit `max_tokens`
- [ ] Formula verified: `max_tokens > thinking.budget + 100`
- [ ] Configuration centralized (not hardcoded per-request)
- [ ] Tests validate token budget compliance

**Automated Testing**:
```typescript
describe('Thinking Mode Configuration', () => {
  it('should have sufficient max_tokens', () => {
    const config = getThinkingConfig();
    expect(config.max_tokens).toBeGreaterThan(
      config.thinking.budget + 100
    );
  });
});
```

---

## Position Violations

### Symptom: High Position Violation Count

**What You See**:
- Position violations card shows high count
- Rate >0.03/sec (YELLOW or RED alert)
- Histogram shows violations at index 1+
- Compliance score <99.5%

### Root Cause Analysis

#### Check 1: Message Construction Order

**Verify**:
```typescript
// Client code
const message = {
  role: 'user',
  content: [
    { type: '???', ... },  // What's at index 0?
    { type: '???', ... },  // What's at index 1?
  ]
};
```

**Diagnosis**:
```
IF content[0].type != 'thinking' AND thinking_enabled
THEN Position violation will occur
```

**Example Issues**:
```typescript
// ❌ WRONG - Text before thinking
content: [
  { type: 'text', text: 'Hello' },        // index 0
  { type: 'thinking', thinking: '...' }   // index 1 - VIOLATION
]

// ❌ WRONG - Only text (thinking missing)
content: [
  { type: 'text', text: 'Hello' }  // index 0, no thinking
]

// ✅ CORRECT - Thinking first
content: [
  { type: 'thinking', thinking: '...' },  // index 0
  { type: 'text', text: 'Hello' }        // index 1
]

// ✅ ALSO CORRECT - Thinking only
content: [
  { type: 'thinking', thinking: '...' }  // index 0
]
```

#### Check 2: Framework Integration

**Common Mistake**: Framework middleware reorders content

**React Example**:
```typescript
// ❌ WRONG - Array.push adds to end
const buildMessage = (text: string, thinking: string) => {
  const content = [];
  content.push({ type: 'text', text });           // index 0
  content.push({ type: 'thinking', thinking });   // index 1 - VIOLATION
  return { role: 'user', content };
};

// ✅ CORRECT - Array.unshift adds to beginning
const buildMessage = (text: string, thinking: string) => {
  const content = [];
  content.push({ type: 'thinking', thinking });   // index 0
  content.push({ type: 'text', text });          // index 1
  return { role: 'user', content };
};

// ✅ BETTER - Explicit ordering
const buildMessage = (text: string, thinking: string) => {
  return {
    role: 'user',
    content: [
      { type: 'thinking', thinking },  // Explicit index 0
      { type: 'text', text }           // Explicit index 1
    ]
  };
};
```

#### Check 3: Histogram Analysis

**Dashboard Histogram**:
```
Index 1: ████████████ 50   ← Most violations
Index 2: ██           8
Index 5: █            3
```

**Interpretation**:
- **Index 1 dominant**: Systematic ordering issue (thinking at index 1 instead of 0)
- **Multiple indices**: Multiple different ordering patterns
- **High indices (10+)**: Severe issue, thinking very late in message

### Resolution Steps

#### Step 1: Identify Violating Code Paths

**Method 1: Code Search**
```bash
# Search for message construction
grep -r "content:" src/
grep -r "type.*thinking" src/
grep -r "\.push.*thinking" src/
```

**Method 2: Request Inspection**
1. Open Monitor page
2. View request details for recent violations
3. Check `contents` array structure
4. Note position of thinking blocks

#### Step 2: Update Message Construction

**Pattern 1: Array Literal (Recommended)**
```typescript
// ✅ BEST - Explicit, clear, immutable
const createMessage = (text: string, thinking?: string) => {
  const content = thinking
    ? [
        { type: 'thinking', thinking },
        { type: 'text', text }
      ]
    : [{ type: 'text', text }];

  return { role: 'user', content };
};
```

**Pattern 2: Conditional Construction**
```typescript
// ✅ GOOD - Clear control flow
const createMessage = (text: string, thinking?: string) => {
  const content = [];

  if (thinking) {
    content.push({ type: 'thinking', thinking });  // Add first
  }

  content.push({ type: 'text', text });  // Add after

  return { role: 'user', content };
};
```

**Pattern 3: Builder Pattern**
```typescript
// ✅ ADVANCED - Encapsulated logic
class MessageBuilder {
  private content: ContentBlock[] = [];

  withThinking(thinking: string): this {
    this.content.unshift({ type: 'thinking', thinking });
    return this;
  }

  withText(text: string): this {
    this.content.push({ type: 'text', text });
    return this;
  }

  build(): Message {
    return { role: 'user', content: this.content };
  }
}

// Usage
const message = new MessageBuilder()
  .withThinking('...')  // Automatically index 0
  .withText('...')      // Automatically index 1
  .build();
```

#### Step 3: Handle Assistant Messages

**Issue**: Model responses may also have thinking blocks

**Solution**: Ensure response parser preserves order
```typescript
// ❌ WRONG - Parser reorders
const parseResponse = (response: any) => {
  const text = response.content.find(b => b.type === 'text');
  const thinking = response.content.find(b => b.type === 'thinking');

  return {
    content: [
      { type: 'text', text: text.text },        // Reordered
      { type: 'thinking', thinking: thinking.thinking }  // Wrong order
    ]
  };
};

// ✅ CORRECT - Preserve original order
const parseResponse = (response: any) => {
  return {
    content: response.content  // Keep as-is
  };
};
```

#### Step 4: Validate Fix

1. Reset metrics in dashboard
2. Send test request with fixed code
3. Check histogram - should be empty
4. Monitor for 5-10 minutes
5. Verify position violations = 0

### Prevention

**Code Review Checklist**:
- [ ] All thinking blocks added at index 0
- [ ] No middleware reordering message content
- [ ] Response parsing preserves original order
- [ ] Tests validate content order

**Automated Testing**:
```typescript
describe('Message Content Order', () => {
  it('should place thinking block first', () => {
    const message = createMessage('text', 'thinking');

    expect(message.content[0].type).toBe('thinking');
    expect(message.content[1].type).toBe('text');
  });

  it('should handle text-only messages', () => {
    const message = createMessage('text');

    expect(message.content[0].type).toBe('text');
    expect(message.content).toHaveLength(1);
  });
});
```

---

## Dashboard Issues

### Dashboard Not Visible

**Symptom**: Compliance section missing on Monitor page

**Possible Causes**:
1. Toggle button not clicked (hidden by default)
2. Component import error
3. Build error (TypeScript compilation)

**Resolution**:
1. Check toggle button state (should be blue when visible)
2. Open browser console for errors
3. Rebuild frontend: `npm run build`

### "No Data Available" Message

**Symptom**: Dashboard shows "No compliance data available"

**Possible Causes**:
1. Proxy service not running
2. Monitor not initialized
3. No requests processed yet

**Resolution**:
1. Check proxy status: Settings > API Proxy > Status = "Running"
2. Send test request to generate data
3. Manually refresh metrics

### Real-time Updates Not Working

**Symptom**: Dashboard not updating on new violations

**Possible Causes**:
1. Event listeners not active
2. Page not re-rendered
3. Proxy service restarted (events lost)

**Resolution**:
1. Check browser console for event listener errors
2. Reload page to reinitialize listeners
3. Manually refresh metrics

**Debugging**:
```typescript
// Add to browser console
window.addEventListener('__TAURI__::proxy://violation', (e) => {
  console.log('Violation event:', e);
});
```

### Metrics Seem Incorrect

**Symptom**: Numbers don't match expected values

**Possible Causes**:
1. Recent reset not accounted for
2. Proxy service restarted (in-memory data lost)
3. Rate calculation window active (60s delay)

**Resolution**:
1. Export report to verify calculations
2. Check for service restarts in logs
3. Wait 60 seconds for rate stabilization

---

## Performance Problems

### Slow Dashboard Loading

**Symptom**: Dashboard takes >5 seconds to load

**Possible Causes**:
1. Large violation history (>10,000 entries)
2. Database query slow
3. Network latency

**Resolution**:
1. Reset metrics to clear history
2. Check database file size: `du -h antigravity.db`
3. Optimize queries (see Advanced Debugging)

### High Memory Usage

**Symptom**: Application memory >500MB

**Possible Causes**:
1. Violation timestamps not cleaned up
2. Memory leak in event listeners
3. Large request log cache

**Resolution**:
1. Reset metrics periodically
2. Clear request logs: Monitor > Clear Logs
3. Restart proxy service

### Request Processing Slow

**Symptom**: API requests taking >5 seconds

**Possible Causes**:
1. Validation overhead (rare, <2ms expected)
2. Database writes blocking
3. Network issues to upstream

**Resolution**:
1. Check validation overhead in logs
2. Disable logging temporarily: `enable_logging: false`
3. Test upstream latency separately

---

## Integration Issues

### Client Can't Connect to Proxy

**Symptom**: Connection refused or timeout

**Possible Causes**:
1. Proxy service not running
2. Port blocked by firewall
3. Incorrect host/port in client

**Resolution**:
1. Verify proxy status: Settings > API Proxy > Status
2. Check port: Default 8045, configurable
3. Test connection: `curl http://localhost:8045/v1/models`

### Authentication Errors

**Symptom**: 401 Unauthorized or 403 Forbidden

**Possible Causes**:
1. Missing API key header
2. Invalid API key
3. Proxy authentication disabled

**Resolution**:
1. Check proxy config: `require_api_key: true`
2. Verify header: `x-api-key: your-key-here`
3. Generate new key: Settings > Generate API Key

### Model Not Found Errors

**Symptom**: 404 Model not found

**Possible Causes**:
1. Incorrect model name
2. Model mapping not configured
3. Proxy not forwarding correctly

**Resolution**:
1. Check supported models: Settings > Model Mapping
2. Verify model name: `claude-sonnet-4.5` (correct)
3. Check request logs for transformation errors

---

## Common Error Messages

### "maxOutputTokens must be greater than thinkingBudget + 100"

**Meaning**: Budget violation detected

**Fix**: Update client configuration
```typescript
// Before
max_tokens: 4096

// After
max_tokens: 8192  // > thinkingBudget (4096) + 100
```

### "Thinking block must be first content block"

**Meaning**: Position violation detected

**Fix**: Reorder content array
```typescript
// Before
content: [
  { type: 'text', text: '...' },
  { type: 'thinking', thinking: '...' }
]

// After
content: [
  { type: 'thinking', thinking: '...' },
  { type: 'text', text: '...' }
]
```

### "Proxy monitor not initialized"

**Meaning**: Dashboard queried before monitor ready

**Fix**: Wait for proxy service to fully start
1. Check proxy status
2. Restart proxy service if needed
3. Refresh dashboard

### "Failed to save stats to DB"

**Meaning**: Database write error

**Fix**: Check database permissions
```bash
# Check file permissions
ls -l ~/Library/Application\ Support/com.lbjlaq.antigravity-tools/antigravity.db

# Fix permissions (macOS)
chmod 644 ~/Library/Application\ Support/com.lbjlaq.antigravity-tools/antigravity.db
```

---

## Advanced Debugging

### Enable Debug Logging

**Rust Backend**:
```toml
# src-tauri/Cargo.toml
[dependencies]
tracing = { version = "0.1", features = ["log"] }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

```bash
# Set log level
export RUST_LOG=antigravity=debug
npm run tauri dev
```

**Frontend**:
```typescript
// Enable verbose logging
localStorage.setItem('debug', 'compliance:*');
```

### Inspect Database

```bash
# Open database
sqlite3 ~/Library/Application\ Support/com.lbjlaq.antigravity-tools/antigravity.db

# Query stats
SELECT * FROM proxy_stats ORDER BY timestamp DESC LIMIT 10;

# Check violations over time
SELECT
  datetime(timestamp, 'unixepoch') as time,
  thinking_budget_violations,
  thinking_position_violations
FROM proxy_stats
ORDER BY timestamp DESC
LIMIT 20;
```

### Capture Network Traffic

**Browser DevTools**:
1. Open DevTools (F12)
2. Network tab
3. Filter: XHR/Fetch
4. Check request/response payloads

**Proxy Logs**:
```bash
# Watch proxy logs in real-time
tail -f ~/Library/Application\ Support/com.lbjlaq.antigravity-tools/logs/proxy.log

# Filter for violations
grep "violation" ~/Library/Application\ Support/com.lbjlaq.antigravity-tools/logs/proxy.log
```

### Test Violation Detection

**Manual Test Script**:
```typescript
// test-budget-violation.ts
import fetch from 'node-fetch';

const testBudgetViolation = async () => {
  const response = await fetch('http://localhost:8045/v1/chat/completions', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'x-api-key': 'your-key'
    },
    body: JSON.stringify({
      model: 'claude-sonnet-4.5',
      max_tokens: 4000,  // INTENTIONALLY LOW - triggers violation
      messages: [{ role: 'user', content: 'test' }],
      thinking: {
        enabled: true,
        budget: 4096
      }
    })
  });

  console.log('Response status:', response.status);
  console.log('Check dashboard for budget violation');
};

testBudgetViolation();
```

```typescript
// test-position-violation.ts
const testPositionViolation = async () => {
  const response = await fetch('http://localhost:8045/v1/chat/completions', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'x-api-key': 'your-key'
    },
    body: JSON.stringify({
      model: 'claude-sonnet-4.5',
      max_tokens: 8192,
      messages: [{
        role: 'user',
        content: [
          { type: 'text', text: 'test' },              // index 0
          { type: 'thinking', thinking: 'test' }       // index 1 - VIOLATION
        ]
      }],
      thinking: {
        enabled: true,
        budget: 4096
      }
    })
  });

  console.log('Response status:', response.status);
  console.log('Check dashboard for position violation');
};

testPositionViolation();
```

### Performance Profiling

**Backend**:
```rust
// Add timing instrumentation
use std::time::Instant;

let start = Instant::now();
check_budget_compliance(...).await;
let duration = start.elapsed();
tracing::debug!("Budget validation took: {:?}", duration);
```

**Frontend**:
```typescript
// Measure dashboard render time
console.time('dashboard-render');
const metrics = await invoke('get_violation_metrics');
console.timeEnd('dashboard-render');
```

---

## Getting Help

### Support Resources

1. **Documentation**:
   - [Compliance Dashboard User Guide](./compliance-dashboard-guide.md)
   - [Thinking Mode Validation Architecture](./thinking-mode-validation-architecture.md)
   - [API Reference](./compliance-api-reference.md)

2. **Issue Tracking**:
   - Open GitHub issue with:
     - Compliance score and violation counts
     - Request logs (sanitized)
     - Client configuration
     - Error messages

3. **Community**:
   - Stack Overflow: Tag `antigravity-tools`
   - Discord/Slack: Share screenshots of dashboard

### Information to Provide

When reporting issues, include:

```markdown
## Environment
- Antigravity Tools version: v3.4.0
- OS: macOS 14.0
- Node.js: v18.0.0
- Client: Claude Code / Custom

## Symptoms
- Compliance score: 95.2%
- Budget violations: 50 (0.05/sec)
- Position violations: 30 (0.03/sec)

## Configuration
```typescript
// Client config (sanitized)
{
  max_tokens: 4096,
  thinking: {
    enabled: true,
    budget: 4096
  }
}
```

## Steps to Reproduce
1. Start proxy service
2. Send request with above config
3. Check dashboard

## Expected vs Actual
- Expected: No violations
- Actual: Budget violations occurring

## Logs/Screenshots
[Attach compliance dashboard screenshot]
[Attach request log excerpt]
```

---

## Related Documentation

- [Compliance Dashboard User Guide](./compliance-dashboard-guide.md)
- [Thinking Mode Validation Architecture](./thinking-mode-validation-architecture.md)
- [API Reference: Compliance Commands](./compliance-api-reference.md)
- [Epic-003: Claude 4.5 Sonnet Thinking Compliance](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md)

---

## Revision History

| Date | Version | Changes | Author |
|------|---------|---------|--------|
| 2026-01-11 | 1.0 | Initial troubleshooting guide | Engineering Team |
