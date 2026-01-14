# Story-005-07: Document maxOutputTokens Auto-Correction

**Epic**: Epic-005-Gemini-3-Pro-High-Compliance
**Priority**: LOW
**Effort**: 1 hour
**Type**: DOCS (Documentation Only)
**Status**: PENDING
**Sequential Order**: 7/8 (Must complete Story-005-06 first)

---

## 📋 User Story

**As a** developer configuring thinking budgets
**I want** documentation of automatic maxOutputTokens correction
**So that** I understand why my max_tokens value changes and how to avoid constraint violations

---

## 🎯 Context and Background

### Current State

**maxOutputTokens Auto-Correction FULLY IMPLEMENTED**:
- ✅ Automatic adjustment when `maxOutputTokens <= thinkingBudget`
- ✅ Safety margin: `budget + 100` tokens
- ✅ Prevents upstream API rejection due to invalid configuration
- ✅ Warning logged: `"⚠️ Constraint violation: maxOutputTokens <= thinkingBudget"`

**Documentation Gap**:
- ❌ Feature not documented
- ❌ Users unaware of automatic correction
- ❌ No explanation why their max_tokens value changes
- ❌ No guidance on proper budget configuration

**Reference**: `gemini-3-pro-high-COMPARISON.md:754-770`

```yaml
discovery: "Automatic increase when maxOutputTokens ≤ thinkingBudget"

reference: "src-tauri/src/proxy/mappers/claude/request.rs:1523-1545"

impact: "POSITIVE - Prevents invalid configurations, user-friendly"

recommendation: "✅ ADD to documentation (Configuration Profiles section)"
```

### Why This Matters

**1. Constraint Compliance**:
- Upstream API requires: `maxOutputTokens > thinkingBudget`
- Invalid configurations cause 400 errors
- Auto-correction ensures request success

**2. User Experience**:
- Prevents confusing API errors
- Requests succeed even with misconfigured budgets
- Transparent logging shows what happened

**3. Profile Compatibility**:
- Profile presets (Story-005-02) must respect constraint
- Auto-correction acts as safety net
- Prevents user configuration errors

---

## 🔨 Documentation Specification

### Document to Update

**File**: `docs/features/thinking-activation.md` (UPDATE - created in Story-005-04)

**New Section to Add**: "maxOutputTokens Auto-Correction" (in "Budget Management" section)

---

### Section Content: maxOutputTokens Auto-Correction

```markdown
---

## maxOutputTokens Auto-Correction 🆕

### Overview

When `maxOutputTokens` is configured equal to or less than `thinkingBudget`, the system automatically adjusts `maxOutputTokens` to prevent upstream API rejection. This safety mechanism ensures requests succeed even with misconfigured budgets.

**Constraint Rule**: `maxOutputTokens MUST BE > thinkingBudget`

**Auto-Correction Formula**: `maxOutputTokens = thinkingBudget + 100`

### Why This Constraint Exists

**Upstream API Requirement**:
- `thinkingBudget`: Tokens allocated for extended reasoning (invisible to user)
- `maxOutputTokens`: Total tokens for thinking + final answer
- Constraint: Final answer needs space after thinking completes

**Example**:
```
thinkingBudget: 8000 tokens  →  Used for <thinking> blocks
maxOutputTokens: 16000 tokens  →  8000 (thinking) + 8000 (answer)
                                  ↑ MUST be > 8000
```

**Invalid Configuration**:
```
thinkingBudget: 8000 tokens
maxOutputTokens: 8000 tokens  ←  INVALID (no space for answer!)
```

**Auto-Corrected Configuration**:
```
thinkingBudget: 8000 tokens
maxOutputTokens: 8100 tokens  ←  AUTO-CORRECTED (8000 + 100 safety margin)
```

### Implementation Details

**Detection and Correction**:

**Reference**: `src-tauri/src/proxy/mappers/claude/request.rs:1523-1545`

```rust
// [CRITICAL] Constraint violation per RE spec (Gap Analysis #4)
// Expected: Error if maxOutputTokens <= thinkingBudget
// Current: Auto-fix for backwards compatibility + enhanced warning
if max_tokens <= clamped_budget {
    let adjusted = clamped_budget + 100;  // Safety margin

    // Record violation for monitoring
    violations.record_budget_violation();

    // Log warning with client guidance
    tracing::warn!(
        "[Thinking-Budget] ⚠️ Constraint violation: maxOutputTokens ({}) <= thinkingBudget ({}). \
         Auto-fixing to {} to maintain compatibility. \
         Client should fix configuration to prevent this warning.",
        max_tokens,
        clamped_budget,
        adjusted
    );

    adjusted  // Return corrected value
} else {
    max_tokens  // Configuration valid, no correction needed
}
```

**Safety Margin**: +100 tokens

**Rationale**:
- Minimal overhead (100 tokens = ~75 words)
- Ensures constraint compliance
- Prevents upstream 400 errors
- Backwards compatible with existing configurations

### Request Examples

#### Example 1: Valid Configuration (No Correction)

**Request**:
```json
{
  "model": "gemini-3-pro-high",
  "max_tokens": 16000,
  "thinking": {
    "type": "enabled",
    "budget_tokens": 8000
  },
  "messages": [...]
}
```

**Processing**:
```
1. Check constraint: 16000 > 8000 ✅ VALID
2. No correction needed
3. Send to upstream as-is
```

**Upstream Request**:
```json
{
  "generationConfig": {
    "maxOutputTokens": 16000,  // ← Unchanged
    "thinkingConfig": {
      "thinkingBudget": 8000
    }
  }
}
```

**Log**: (No warning)

#### Example 2: Invalid Configuration (Auto-Corrected)

**Request**:
```json
{
  "model": "gemini-3-pro-high",
  "max_tokens": 8000,
  "thinking": {
    "type": "enabled",
    "budget_tokens": 8000
  },
  "messages": [...]
}
```

**Processing**:
```
1. Check constraint: 8000 <= 8000 ❌ INVALID
2. Auto-correct: 8000 + 100 = 8100
3. Record violation
4. Log warning
5. Send to upstream with corrected value
```

**Upstream Request**:
```json
{
  "generationConfig": {
    "maxOutputTokens": 8100,  // ← AUTO-CORRECTED
    "thinkingConfig": {
      "thinkingBudget": 8000
    }
  }
}
```

**Log**:
```
[Thinking-Budget] ⚠️ Constraint violation: maxOutputTokens (8000) <= thinkingBudget (8000).
Auto-fixing to 8100 to maintain compatibility.
Client should fix configuration to prevent this warning.
```

#### Example 3: Severely Invalid Configuration

**Request**:
```json
{
  "model": "gemini-3-pro-high",
  "max_tokens": 2000,
  "thinking": {
    "type": "enabled",
    "budget_tokens": 32000
  },
  "messages": [...]
}
```

**Processing**:
```
1. Check constraint: 2000 <= 32000 ❌ SEVERELY INVALID
2. Auto-correct: 32000 + 100 = 32100
3. Record violation
4. Log warning
5. Send to upstream with corrected value
```

**Upstream Request**:
```json
{
  "generationConfig": {
    "maxOutputTokens": 32100,  // ← AUTO-CORRECTED (30100 token increase!)
    "thinkingConfig": {
      "thinkingBudget": 32000
    }
  }
}
```

**Log**:
```
[Thinking-Budget] ⚠️ Constraint violation: maxOutputTokens (2000) <= thinkingBudget (32000).
Auto-fixing to 32100 to maintain compatibility.
Client should fix configuration to prevent this warning.
```

**Note**: Response may be truncated at 32100 tokens (model limit)

### Profile Preset Compliance

**From Story-005-02**: Configuration profile presets respect constraint

| Profile | Thinking Budget | maxOutputTokens | Valid? |
|---------|----------------|-----------------|---------|
| PRODUCTION | 8192 | 8192 | ❌ → 8292 (corrected) |
| LOW_LATENCY | 2048 | 2048 | ❌ → 2148 (corrected) |
| HIGH_QUALITY | 16384 | 16384 | ❌ → 16484 (corrected) |
| TOOL_OPTIMIZED | 4096 | 4096 | ❌ → 4196 (corrected) |
| BALANCED_THINKING | 8000 | 16000 | ✅ Valid |
| DEEP_THINKING | 32000 | 40000 | ✅ Valid |
| EFFICIENT_THINKING | 2000 | 6000 | ✅ Valid |
| COMPREHENSIVE_THINKING | 20000 | 30000 | ✅ Valid |

**Recommended Fix** (for Story-005-02):
```javascript
// Update base profiles to respect constraint
const BASE_PROFILES: ProfilePreset[] = [
  {
    id: 'production',
    max_tokens: 16384,  // ← Increase from 8192
    description: 'Balanced configuration for production workloads'
  },
  // ... similar fixes for other base profiles
];
```

### Best Practices

#### ✅ DO: Configure Proper Budgets

**Good Configuration** (thinking has room for answer):
```json
{
  "thinking": {"budget_tokens": 8000},
  "max_tokens": 16000  // ← 2x budget (8000 thinking + 8000 answer)
}
```

**Good Configuration** (higher ratio):
```json
{
  "thinking": {"budget_tokens": 8000},
  "max_tokens": 24000  // ← 3x budget (8000 thinking + 16000 answer)
}
```

#### ❌ DON'T: Equal Budgets

**Bad Configuration**:
```json
{
  "thinking": {"budget_tokens": 8000},
  "max_tokens": 8000  // ← INVALID (no room for answer)
}
```
**Result**: Auto-corrected to 8100, warning logged

#### ❌ DON'T: Insufficient Space

**Bad Configuration**:
```json
{
  "thinking": {"budget_tokens": 32000},
  "max_tokens": 10000  // ← INVALID (thinking alone uses 32000!)
}
```
**Result**: Auto-corrected to 32100, warning logged, likely truncated response

### Monitoring Auto-Corrections

**Log Filtering**:
```bash
# Find all auto-correction warnings
grep "Thinking-Budget.*Constraint violation" logs/antigravity.log

# Count violations per day
grep "Constraint violation" logs/antigravity.log | \
  grep "$(date +%Y-%m-%d)" | wc -l
```

**Violation Metrics** (tracked internally):
```rust
// Recorded for monitoring dashboard (future enhancement)
violations.record_budget_violation();
```

**High Violation Rate** (>10% of requests):
- Indicates profile presets need fixing
- Or client-side configuration errors
- Review logs for patterns

### Troubleshooting

#### Issue 1: Seeing "Constraint violation" Warnings

**Symptoms**:
- Logs show: `"⚠️ Constraint violation: maxOutputTokens <= thinkingBudget"`
- Requests succeed but warnings appear frequently

**Diagnosis**:
```
Auto-correction working as designed
Configuration invalid, needs fixing
```

**Solutions**:
1. **Fix configuration**:
   ```json
   // Before (invalid)
   {"thinking": {"budget_tokens": 8000}, "max_tokens": 8000}

   // After (valid)
   {"thinking": {"budget_tokens": 8000}, "max_tokens": 16000}
   ```

2. **Use profile presets** (Story-005-02):
   - BALANCED_THINKING: 8000 budget, 16000 max ✅
   - DEEP_THINKING: 32000 budget, 40000 max ✅

3. **Formula**: `max_tokens >= thinking_budget + 1000` (1000 token minimum for answer)

#### Issue 2: Response Truncated

**Symptoms**:
- Response ends mid-sentence
- Total tokens near maxOutputTokens limit
- Used all thinking budget + answer space

**Diagnosis**:
```
maxOutputTokens too low for thinking + answer
Auto-correction provided minimal margin (+ 100)
```

**Solutions**:
1. **Increase maxOutputTokens**:
   ```json
   // Before (minimal margin)
   {"thinking": {"budget_tokens": 8000}, "max_tokens": 8100}

   // After (generous margin)
   {"thinking": {"budget_tokens": 8000}, "max_tokens": 20000}
   ```

2. **Reduce thinking budget**:
   ```json
   // If model limit constrains maxOutputTokens
   {"thinking": {"budget_tokens": 4000}, "max_tokens": 10000}
   ```

3. **Check model limits**:
   - Gemini 3 Pro: 32000 token budget limit
   - Total response: ~40000 tokens max

#### Issue 3: Want to Disable Auto-Correction

**Symptoms**:
- User prefers strict validation (fail on invalid config)
- Auto-correction masks configuration errors

**Diagnosis**:
```
Auto-correction is always enabled
Cannot be disabled via configuration
```

**Solutions**:
1. **Accept design**: Auto-correction improves UX and prevents API failures
2. **Monitor warnings**: Review logs for configuration issues
3. **Fix configurations**: Eliminate violations at source
4. **Future enhancement**: Add `STRICT_BUDGET_VALIDATION` env var (not currently implemented)

---

## Configuration Validation Checklist

Before sending thinking requests, validate:

- [ ] **Constraint**: `max_tokens > thinking.budget_tokens`
- [ ] **Margin**: `max_tokens >= thinking.budget_tokens + 1000` (recommended)
- [ ] **Budget Limit**: `thinking.budget_tokens <= 32000` (Gemini 3 Pro High)
- [ ] **Total Limit**: `max_tokens <= 40000` (approximate model maximum)
- [ ] **Profile Presets**: Use validated presets (Story-005-02)

**Example Valid Configurations**:
```javascript
// Minimal margin (gets auto-corrected if max_tokens <= budget)
{"thinking": {"budget_tokens": 8000}, "max_tokens": 8001}  // ⚠️ Marginal

// Recommended margin (1000+ tokens for answer)
{"thinking": {"budget_tokens": 8000}, "max_tokens": 9000}  // ✅ Good

// Generous margin (2x budget)
{"thinking": {"budget_tokens": 8000}, "max_tokens": 16000}  // ✅ Excellent

// Maximum thinking (32000 budget)
{"thinking": {"budget_tokens": 32000}, "max_tokens": 40000}  // ✅ Max capacity
```

---

## References

### Implementation Files

- **Auto-Correction Logic**: `src-tauri/src/proxy/mappers/claude/request.rs:1523-1545`
- **Test Case 1**: `request.rs:2440-2468` (Gemini 3 Pro auto-fix)
- **Test Case 2**: `request.rs:2470-2504` (Claude auto-fix)
- **Violation Tracking**: `violations.record_budget_violation()`

### Related Documentation

- **Epic-005**: `docs/epics/Epic-005-Gemini-3-Pro-High-Compliance.md`
- **COMPARISON**: `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-COMPARISON.md:754-770`
- **Story-005-02**: Profile Presets UI (needs update for constraint compliance)
- **Story-005-04**: Thinking Activation Architecture (budget management section)

### External References

- **Gemini API Constraints**: https://ai.google.dev/gemini-api/docs/thinking#constraints
```

---

## ✅ Acceptance Criteria

### AC-1: Auto-Correction Section Added

**Given** I navigate to `docs/features/thinking-activation.md`
**When** I review the "maxOutputTokens Auto-Correction" section
**Then** I see:
- Overview explaining constraint and correction
- Implementation details with code references
- Auto-correction formula (`budget + 100`)
- 3 request examples (valid, invalid, severely invalid)
- Profile preset compliance table
- Best practices (DO/DON'T)

**Validation**: Documentation peer review + completeness check

### AC-2: Constraint Rule Documented

**Given** I want to understand the constraint
**When** I review "Why This Constraint Exists" subsection
**Then** I see:
- Clear constraint rule: `maxOutputTokens > thinkingBudget`
- Example showing valid vs invalid configuration
- Explanation why constraint exists (space for answer)

**Validation**: Apply constraint rule to 5 configurations, verify correctness

### AC-3: Request Examples Work

**Given** I want to test auto-correction
**When** I execute Example 2 (Invalid Configuration)
**Then**:
- Request sent with max_tokens=8000, budget=8000
- Auto-corrected to max_tokens=8100
- Warning logged: "Constraint violation"
- Request succeeds

**Validation**: Execute all 3 examples, verify auto-correction behavior

### AC-4: Profile Preset Compliance Table

**Given** I'm reviewing profile presets from Story-005-02
**When** I check the Profile Preset Compliance table
**Then** I see:
- 8 profiles listed (4 base + 4 thinking)
- Validity status for each profile
- Auto-corrected values for invalid profiles
- Recommended fixes

**Validation**: Cross-reference with Story-005-02 profiles, verify table accuracy

### AC-5: Troubleshooting Scenarios

**Given** I have auto-correction warnings
**When** I review "Troubleshooting" section
**Then** I see:
- 3 common issues (warnings, truncation, disable)
- Diagnosis steps for each issue
- Multiple solution options with code examples

**Validation**: Trigger each issue, follow troubleshooting steps

### AC-6: Configuration Validation Checklist

**Given** I'm configuring thinking budgets
**When** I review "Configuration Validation Checklist"
**Then** I see:
- 5-item checklist with checkboxes
- 4 example valid configurations
- Clear margin recommendations (1000+ tokens)

**Validation**: Apply checklist to 10 configurations, verify validity

---

## 🧪 Testing Strategy

### Documentation Testing

**Validation Method**: Technical review + hands-on testing

**Checklist**:
- [ ] All code references accurate (line numbers match)
- [ ] All examples executable (curl, JSON requests)
- [ ] Auto-correction logic matches implementation
- [ ] Profile preset table accurate
- [ ] Troubleshooting scenarios reproducible

### Auto-Correction Tests

**Test Case 1**: Valid Configuration (No Correction)
```bash
# max_tokens=16000, budget=8000
# Verify: No correction, no warning
# Verify: Upstream receives 16000
```

**Test Case 2**: Invalid Configuration (Auto-Corrected)
```bash
# max_tokens=8000, budget=8000
# Verify: Corrected to 8100
# Verify: Warning logged
# Verify: Upstream receives 8100
```

**Test Case 3**: Severely Invalid Configuration
```bash
# max_tokens=2000, budget=32000
# Verify: Corrected to 32100
# Verify: Warning logged with large delta
# Verify: Response may be truncated
```

---

## 🔗 Dependencies and Relationships

### Sequential Dependencies

**Blocks**:
- **Story-005-08** (Update Configuration Profiles Documentation)
  - Profile presets need constraint compliance fixes
  - References auto-correction behavior

**Blocked By**:
- **Story-005-06** (Document First-Time Permissive Mode)
  - MUST complete first per sequential execution order
  - No technical dependency, organizational only

### Cross-Story Impact

**Impacts**:
- **Story-005-02** (Profile Presets UI):
  - Base profiles need updating (max_tokens too low)
  - Recommended fixes documented in this story

---

## 📊 Success Metrics

### Documentation Quality Metrics

- **Completeness**: 100% of auto-correction behavior documented
- **Accuracy**: 0 technical errors in constraint logic
- **Examples**: 3 executable request examples
- **Troubleshooting**: 3 common issues with solutions

### User Impact Metrics

- **Configuration Errors**: -40% (via clear constraint documentation)
- **API Rejections**: -95% (via auto-correction)
- **Support Tickets**: -30% (via troubleshooting guide)

---

## ⚠️ Risks and Mitigation

### Risk 1: Profile Presets Need Fixing

**Description**: Base profiles in Story-005-02 violate constraint

**Probability**: HIGH (confirmed in compliance table)
**Impact**: MEDIUM (frequent auto-correction warnings)

**Mitigation**:
1. ✅ Document issue in compliance table
2. ✅ Provide recommended fixes
3. Create follow-up task: Update Story-005-02 profiles
4. Note in Story-005-08 documentation

### Risk 2: Users Confused by Auto-Correction

**Description**: Users may not understand why max_tokens changes

**Probability**: MEDIUM
**Impact**: LOW (confusion, support questions)

**Mitigation**:
1. ✅ Clear Overview section explaining constraint
2. ✅ "Why This Constraint Exists" explanation
3. ✅ Example 2 shows before/after values
4. ✅ Warning message logged with clear explanation

### Risk 3: +100 Margin Too Small

**Description**: 100 token margin may be insufficient for some answers

**Probability**: LOW
**Impact**: LOW (response truncation)

**Mitigation**:
1. ✅ Best practices recommend +1000 margin minimum
2. ✅ Document response truncation in troubleshooting
3. ✅ Users can configure larger max_tokens
4. Auto-correction is safety net, not recommended configuration

---

## 📝 Notes for Developers

### Documentation Location

**File**: `docs/features/thinking-activation.md`
**Section**: "maxOutputTokens Auto-Correction" (new section)
**Position**: In "Budget Management" section, after budget clamping rules

### Cross-Reference Updates

**In Code** (`request.rs:1523-1545`):
```rust
// 📖 maxOutputTokens Auto-Correction Documented: docs/features/thinking-activation.md:maxOutputTokens-Auto-Correction
if max_tokens <= clamped_budget {
    let adjusted = clamped_budget + 100;
    // ... auto-correction logic
}
```

### Story-005-02 Follow-Up

**Action Required**: Update profile presets to respect constraint

**Recommended Changes**:
```javascript
// Current (invalid)
{id: 'production', max_tokens: 8192}

// Fixed (valid)
{id: 'production', max_tokens: 16384}  // 2x budget
```

---

## ✏️ Story Status Tracker

- [ ] **Documentation Update** (1 hour)
  - [ ] Add "maxOutputTokens Auto-Correction" section to `thinking-activation.md`
  - [ ] Write Overview (constraint rule + formula)
  - [ ] Write "Why This Constraint Exists" explanation
  - [ ] Write Implementation Details (code references)
  - [ ] Write 3 Request Examples (valid, invalid, severe)
  - [ ] Write Profile Preset Compliance table
  - [ ] Write Best Practices (DO/DON'T)
  - [ ] Write Monitoring subsection (log filtering)
  - [ ] Write Troubleshooting section (3 issues)
  - [ ] Write Configuration Validation Checklist
  - [ ] Add cross-references to implementation
  - [ ] Validate all code references
  - [ ] Test auto-correction examples

**Total Effort**: 1 hour

---

**Story Created**: 2026-01-11
**Last Updated**: 2026-01-11
**Created By**: Documentation Team (Epic-005 Sequential Story Creation)
**Next Story**: Story-005-08 (Update Configuration Profiles Documentation) - FINAL STORY
