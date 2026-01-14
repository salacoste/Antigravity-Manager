# Story-005-04: Document Thinking Activation Architecture

**Epic**: Epic-005-Gemini-3-Pro-High-Compliance
**Priority**: MEDIUM
**Effort**: 1 hour
**Type**: DOCS (Documentation Only)
**Status**: PENDING
**Sequential Order**: 4/8 (Must complete Story-005-03 first)

---

## 📋 User Story

**As a** developer integrating extended thinking capabilities
**I want** comprehensive documentation of thinking activation architecture
**So that** I understand how to enable thinking for Gemini 3 Pro High and how it differs from Claude models

---

## 🎯 Context and Background

### Current State

**Thinking Activation FULLY IMPLEMENTED**:
- ✅ Parameter-based activation via `thinkingConfig` object
- ✅ 32000 token budget limit with automatic clamping
- ✅ Signature validation for function calls (auto-disable if invalid)
- ✅ Web search conflict detection (thinking disabled when conflicts)
- ✅ Graceful degradation (continues without thinking on errors)

**Documentation Gap**:
- ❌ No comprehensive documentation of thinking activation flow
- ❌ Parameter-based architecture not explained vs Claude's model suffix approach
- ❌ Budget clamping logic not documented
- ❌ Conflict detection rules not documented

**Reference**: `gemini-3-pro-high-COMPARISON.md:145-227`

```yaml
status: ✅ FULLY IMPLEMENTED

what_works:
  - ✅ Parameter-based activation (thinkingConfig)
  - ✅ 32000 token budget limit
  - ✅ Budget clamping validation
  - ✅ Signature validation for function calls
  - ✅ Graceful degradation (auto-disable)
  - ✅ Web search conflict detection

recommendation: |
  UPDATE documentation to include:
  1. Complete thinking activation flow
  2. Parameter-based architecture explanation
  3. Budget clamping rules
  4. Conflict detection logic
  5. Graceful degradation examples
```

### Why This Matters

**1. Architectural Understanding**:
- **Gemini 3**: Parameter-based thinking (`thinkingConfig` object)
- **Claude**: Model name suffix (`claude-4.5-sonnet-thinking`)
- Different activation patterns require different client code

**2. Integration Guidance**:
- Developers need to know how to enable thinking
- Budget limits vary by model (32000 vs 24576)
- Conflict rules determine when thinking is auto-disabled

**3. Troubleshooting Support**:
- Understanding why thinking was disabled in specific requests
- Budget clamping behavior explanation
- Signature validation failure scenarios

---

## 🔨 Documentation Specification

### Document to Create

**File**: `docs/features/thinking-activation.md` (NEW)

**Table of Contents**:
```markdown
# Extended Thinking Activation Architecture

## Overview
## Architectural Differences: Gemini vs Claude
## Activation Flow
## Budget Management
## Conflict Detection & Auto-Disable
## Configuration Examples
## Troubleshooting Guide
## References
```

---

### Section 1: Overview

```markdown
# Extended Thinking Activation Architecture

## Overview

Extended thinking (also known as "chain of thought") is a feature that allows models to show their reasoning process before generating the final response. Antigravity Manager implements thinking activation differently for Gemini and Claude models due to architectural differences in their APIs.

**Supported Models**:
- **Gemini 3 Pro High** (primary production model)
- **Gemini 3 Pro Low** (cost-optimized variant)
- **Claude 4.5 Sonnet** (reference comparison)

**Key Capabilities**:
- Parameter-based thinking activation (Gemini)
- Model name suffix thinking activation (Claude)
- Automatic budget clamping to model limits
- Intelligent conflict detection (web search, function calls)
- Graceful degradation on errors

---

## Architectural Differences: Gemini vs Claude

### Gemini 3 Approach (Parameter-Based)

**Activation Method**: Via `thinkingConfig` parameter in request

```json
{
  "model": "gemini-3-pro-high",
  "max_tokens": 16000,
  "messages": [...],
  "thinking": {
    "type": "enabled",
    "budget_tokens": 8000
  }
}
```

**Upstream Transformation** (to Google v1internal format):

```json
{
  "request": {
    "generationConfig": {
      "maxOutputTokens": 16000,
      "thinkingConfig": {
        "includeThoughts": true,
        "thinkingBudget": 8000
      }
    }
  }
}
```

**Advantages**:
- ✅ Single model ID for both base and thinking modes
- ✅ Fine-grained control over thinking budget
- ✅ Can disable thinking dynamically without changing model
- ✅ Easier to adjust budget mid-conversation

### Claude Approach (Model Name Suffix)

**Activation Method**: Via model name suffix `-thinking`

```json
{
  "model": "claude-4.5-sonnet-thinking",
  "max_tokens": 16000,
  "messages": [...]
}
```

**Characteristics**:
- Model name determines thinking capability
- Budget hardcoded in backend (not user-configurable)
- Requires model switch to enable/disable thinking
- Simpler API surface (no extra parameters)

### Architectural Decision: Why Parameter-Based?

**Rationale** (Gemini 3 models):
1. **Flexibility**: Adjust budget per request without model changes
2. **Granularity**: Fine-tune thinking effort vs cost tradeoff
3. **Compatibility**: Works seamlessly with OpenAI SDK format
4. **Future-Proof**: Easier to add thinking configuration options

---

## Activation Flow

### Decision Tree

```
START: Incoming request with model="gemini-3-pro-high"
│
├─➤ Check explicit thinking parameter
│   │
│   ├─ thinking.type == "enabled"? ──➤ YES ──➤ is_thinking_enabled = true
│   │
│   └─ thinking.type == "disabled"? ──➤ YES ──➤ is_thinking_enabled = false
│
├─➤ Check default activation (if no explicit thinking param)
│   │
│   ├─ model.starts_with("gemini-3")? ──➤ YES ──➤ is_thinking_enabled = true
│   │
│   └─ model.starts_with("claude-")? ──➤ YES ──➤ is_thinking_enabled = true (if -thinking suffix)
│
├─➤ Validate model support
│   │
│   ├─ model supports thinking? ──➤ NO ──➤ AUTO-DISABLE ──➤ is_thinking_enabled = false
│   │
│   └─ model supports thinking? ──➤ YES ──➤ CONTINUE
│
├─➤ Check conflict conditions
│   │
│   ├─ has_web_search_tool? ──➤ YES ──➤ AUTO-DISABLE ──➤ is_thinking_enabled = false
│   │
│   ├─ has_function_calls WITHOUT valid signature? ──➤ YES ──➤ AUTO-DISABLE
│   │
│   └─ No conflicts ──➤ CONTINUE
│
├─➤ Clamp budget to model limits
│   │
│   ├─ model == "gemini-3-*"? ──➤ budget = min(budget, 32000)
│   │
│   ├─ model == "gemini-2.5-flash"? ──➤ budget = min(budget, 24576)
│   │
│   └─ model == "claude-*"? ──➤ budget = min(budget, 32000)
│
└─➤ Build thinkingConfig
    │
    ├─ is_thinking_enabled == true ──➤ Include thinkingConfig in request
    │
    └─ is_thinking_enabled == false ──➤ Omit thinkingConfig from request
```

### Implementation Reference

**File**: `src-tauri/src/proxy/mappers/claude/request.rs:277-362`

```rust
// Step 1: Check explicit thinking parameter
let mut is_thinking_enabled = claude_req
    .thinking
    .as_ref()
    .map(|t| t.type_ == "enabled")
    .unwrap_or_else(|| should_enable_thinking_by_default(&claude_req.model));

// Step 2: Validate model support
let target_model_supports_thinking = mapped_model.contains("-thinking")
    || mapped_model.starts_with("claude-")
    || mapped_model.starts_with("gemini-");

if !target_model_supports_thinking {
    is_thinking_enabled = false;
}

// Step 3: Check web search conflict
if has_web_search && is_thinking_enabled {
    tracing::info!("[Thinking-Mode] Disabling thinking due to web search conflict");
    is_thinking_enabled = false;
}

// Step 4: Clamp budget to model limits
let budget = if has_web_search || mapped_model.contains("gemini-2.5-flash") {
    budget.min(24576)  // Flash limit
} else {
    budget.min(32000)  // Gemini 3 Pro High limit
};

// Step 5: Check signature validation (for function calls)
if needs_signature_check && !has_valid_signature_for_function_calls(...) {
    tracing::info!("[Thinking-Mode] Auto-disabling due to missing/invalid signature");
    is_thinking_enabled = false;
}

// Step 6: Build thinkingConfig if enabled
if is_thinking_enabled {
    gen_config["thinkingConfig"] = json!({
        "includeThoughts": true,
        "thinkingBudget": budget
    });
}
```

---

## Budget Management

### Model Budget Limits

| Model | Maximum Thinking Budget | Total Max Tokens |
|-------|------------------------|------------------|
| Gemini 3 Pro High | 32,000 | 40,000+ |
| Gemini 3 Pro Low | 32,000 | 40,000+ |
| Gemini 2.5 Flash | 24,576 | 32,000 |
| Claude 4.5 Sonnet | 32,000 | 40,000+ |

### Budget Clamping Rules

**Rule 1: Model-Based Clamping**

```
IF model.contains("gemini-3"):
    clamped_budget = user_budget.min(32000)

ELSE IF model.contains("gemini-2.5-flash"):
    clamped_budget = user_budget.min(24576)

ELSE IF model.contains("claude"):
    clamped_budget = user_budget.min(32000)
```

**Rule 2: Conflict-Based Clamping**

```
IF has_web_search_tool:
    # Web search forces Flash model (lower limit)
    clamped_budget = user_budget.min(24576)
```

**Rule 3: maxOutputTokens Auto-Correction**

```
# Ensure max_tokens accommodates thinking + response
IF max_tokens <= thinking_budget:
    max_tokens = thinking_budget + 4000  # Safety margin
```

**Example**:
```json
// User Request
{
  "model": "gemini-3-pro-high",
  "max_tokens": 8000,
  "thinking": { "budget_tokens": 10000 }
}

// After Clamping (10000 > 8000)
{
  "generationConfig": {
    "maxOutputTokens": 14000,  // 10000 + 4000 safety margin
    "thinkingConfig": {
      "thinkingBudget": 10000  // Within 32000 limit
    }
  }
}
```

---

## Conflict Detection & Auto-Disable

### Conflict Type 1: Web Search Tools

**Problem**: Web search requires Gemini 2.5 Flash, which has lower thinking budget (24576 vs 32000)

**Detection**:
```rust
let has_web_search = claude_req.tools.iter().any(|tool|
    tool.name.contains("web_search") || tool.name.contains("brave_search")
);
```

**Action**:
```
IF has_web_search:
    1. Auto-route request to gemini-2.5-flash
    2. Clamp thinking_budget to min(budget, 24576)
    3. Log: "[Thinking-Mode] Web search detected, clamping budget to 24576"
```

### Conflict Type 2: Function Calls Without Valid Signature

**Problem**: Function calls in thinking mode require valid JWT signature for integrity

**Detection**:
```rust
let needs_signature_check = has_function_calls && is_thinking_enabled;

if needs_signature_check && !has_valid_signature_for_function_calls(...) {
    // Signature missing or invalid
}
```

**Action**:
```
IF function_calls && thinking_enabled && !valid_signature:
    1. Set is_thinking_enabled = false
    2. Omit thinkingConfig from request
    3. Log: "[Thinking-Mode] Auto-disabling due to missing/invalid signature"
    4. Request proceeds without thinking
```

### Conflict Type 3: Model Doesn't Support Thinking

**Problem**: User requests thinking for non-thinking model (e.g., `gemini-2.0-flash-lite`)

**Detection**:
```rust
let target_model_supports_thinking =
    mapped_model.contains("-thinking") ||
    mapped_model.starts_with("claude-") ||
    mapped_model.starts_with("gemini-3");
```

**Action**:
```
IF !target_model_supports_thinking:
    1. Set is_thinking_enabled = false
    2. Omit thinkingConfig from request
    3. Log: "[Thinking-Mode] Target model does not support thinking: {model}"
    4. Request proceeds without thinking
```

### Graceful Degradation Philosophy

**Principle**: Never fail requests due to thinking conflicts—auto-disable and continue

**Benefits**:
- ✅ Higher request success rate
- ✅ Better user experience (no 400 errors)
- ✅ Transparent conflict resolution
- ✅ Logging provides visibility

---

## Configuration Examples

### Example 1: Basic Thinking Activation (Gemini 3)

```bash
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gemini-3-pro-high",
    "max_tokens": 16000,
    "messages": [
      {"role": "user", "content": "Explain quantum entanglement"}
    ],
    "thinking": {
      "type": "enabled",
      "budget_tokens": 8000
    }
  }'
```

**Expected Behavior**:
- ✅ Thinking enabled with 8000 token budget
- ✅ maxOutputTokens set to 16000
- ✅ Response includes `<thinking>` blocks with reasoning

### Example 2: Profile-Based Thinking (BALANCED_THINKING)

```bash
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gemini-3-pro-high",
    "max_tokens": 16000,
    "messages": [
      {"role": "user", "content": "Solve this complex math problem"}
    ],
    "thinking": {
      "type": "enabled",
      "budget_tokens": 8000
    }
  }'
```

**Profile**: BALANCED_THINKING (8000 budget, 16000 max)

### Example 3: Deep Reasoning (DEEP_THINKING)

```bash
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type": "application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gemini-3-pro-high",
    "max_tokens": 40000,
    "messages": [
      {"role": "user", "content": "Design a distributed system architecture"}
    ],
    "thinking": {
      "type": "enabled",
      "budget_tokens": 32000
    }
  }'
```

**Profile**: DEEP_THINKING (32000 budget, 40000 max)

**Note**: 40000 exceeds budget limit, will be clamped to 32000 + 4000 = 36000 max_tokens

### Example 4: Disabling Thinking Explicitly

```bash
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type": application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gemini-3-pro-high",
    "max_tokens": 8192,
    "messages": [
      {"role": "user", "content": "Simple factual question"}
    ],
    "thinking": {
      "type": "disabled"
    }
  }'
```

**Expected Behavior**:
- ✅ Thinking explicitly disabled
- ✅ No thinkingConfig in upstream request
- ✅ Response contains only final answer (no `<thinking>` blocks)

### Example 5: Auto-Disable Due to Web Search

```bash
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gemini-3-pro-high",
    "max_tokens": 16000,
    "messages": [
      {"role": "user", "content": "Search the web for latest AI news"}
    ],
    "thinking": {
      "type": "enabled",
      "budget_tokens": 8000
    },
    "tools": [
      {"type": "web_search_20241022"}
    ]
  }'
```

**Expected Behavior**:
- ⚠️ Thinking AUTO-DISABLED due to web search conflict
- ⚠️ Request routed to `gemini-2.5-flash` instead
- ⚠️ Budget clamped to 24576 (Flash limit)
- ✅ Web search proceeds successfully

---

## Troubleshooting Guide

### Issue 1: Thinking Not Appearing in Response

**Symptoms**:
- Request sent with `thinking: {type: "enabled"}`
- Response contains no `<thinking>` blocks

**Diagnosis**:
```
Check logs for auto-disable messages:
- "[Thinking-Mode] Target model does not support thinking"
- "[Thinking-Mode] Disabling thinking due to web search conflict"
- "[Thinking-Mode] Auto-disabling due to missing/invalid signature"
```

**Solutions**:
1. **Model doesn't support thinking**: Use `gemini-3-pro-high` or `claude-4.5-sonnet-thinking`
2. **Web search conflict**: Remove `thinking` parameter when using web search tools
3. **Signature validation failure**: Ensure function call history has valid signatures

### Issue 2: Budget Exceeded Error

**Symptoms**:
- Request fails with "Budget exceeds model limit"
- 400 error from upstream

**Diagnosis**:
```
Check requested budget:
- Gemini 3 Pro: max 32000
- Gemini 2.5 Flash: max 24576
```

**Solutions**:
1. Reduce `thinking.budget_tokens` to fit within model limit
2. Use profile presets (BALANCED: 8000, DEEP: 32000)
3. Enable auto-clamping (default behavior)

### Issue 3: maxOutputTokens Too Low

**Symptoms**:
- Thinking works but response truncated
- Response ends mid-sentence

**Diagnosis**:
```
Check: max_tokens <= thinking_budget?
If yes: Auto-correction should apply (+4000 margin)
```

**Solutions**:
1. Increase `max_tokens` to at least `budget_tokens + 4000`
2. Rely on auto-correction feature (Story-005-07)
3. Use profile presets with correct ratios

---

## References

### Implementation Files

- **Request Mapping**: `src-tauri/src/proxy/mappers/claude/request.rs:277-362`
- **Thinking Utilities**: `src-tauri/src/proxy/mappers/claude/thinking_utils.rs`
- **Budget Clamping**: `src-tauri/src/proxy/mappers/claude/request.rs:604-656`
- **Conflict Detection**: `src-tauri/src/proxy/mappers/claude/request.rs:148-175`

### Related Documentation

- **Epic-005**: `docs/epics/Epic-005-Gemini-3-Pro-High-Compliance.md`
- **COMPARISON**: `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-COMPARISON.md:145-227`
- **Story-005-02**: Configuration profile presets (thinking budgets)
- **Story-005-05**: OpenAI protocol auto-injection (next story)
- **Story-005-06**: First-time permissive mode (next story)
- **Story-005-07**: maxOutputTokens auto-correction (next story)

### External References

- **Google Gemini API**: https://ai.google.dev/gemini-api/docs/thinking
- **Extended Thinking Paper**: https://arxiv.org/abs/2312.14563 (Chain-of-Thought reasoning)
```

---

## ✅ Acceptance Criteria

### AC-1: Documentation File Created

**Given** I navigate to `docs/features/thinking-activation.md`
**When** I review the documentation
**Then** I see:
- Complete architecture comparison (Gemini parameter-based vs Claude model name suffix)
- Decision tree diagram with 6 major decision points
- Budget management section with clamping rules
- Conflict detection section with 3 conflict types
- 5 configuration examples (basic, profiles, deep, disable, web search)
- Troubleshooting guide with 3+ scenarios

**Validation**: Documentation peer review + completeness check

### AC-2: Architectural Differences Explained

**Given** I'm a developer choosing between Gemini and Claude
**When** I read the "Architectural Differences" section
**Then** I understand:
- Why Gemini uses parameter-based thinking (`thinkingConfig`)
- Why Claude uses model name suffix (`-thinking`)
- Advantages of parameter-based approach (4 listed)
- Which approach to use for different use cases

**Validation**: Survey 3 developers for comprehension

### AC-3: Activation Flow Documented

**Given** I want to understand how thinking activation works
**When** I review the "Activation Flow" section
**Then** I see:
- ASCII decision tree with all branches
- Step-by-step process (explicit check → default → model support → conflicts → budget → thinkingConfig)
- Code references to actual implementation
- Clear explanation of each decision point

**Validation**: Follow decision tree with 5 example requests, verify outcomes

### AC-4: Budget Clamping Rules Documented

**Given** I want to set thinking budget correctly
**When** I review the "Budget Management" section
**Then** I see:
- Table of model budget limits (4 models)
- 3 clamping rules (model-based, conflict-based, auto-correction)
- Example showing before/after clamping
- maxOutputTokens auto-correction formula

**Validation**: Apply clamping rules to 5 test budgets, verify calculations

### AC-5: Conflict Detection Rules Documented

**Given** I want to know when thinking is auto-disabled
**When** I review the "Conflict Detection & Auto-Disable" section
**Then** I see:
- 3 conflict types (web search, function calls, unsupported models)
- Detection code for each conflict
- Action taken for each conflict
- Graceful degradation philosophy explanation

**Validation**: Trigger each conflict type, verify auto-disable behavior

### AC-6: Configuration Examples Work

**Given** I want to enable thinking for Gemini 3 Pro High
**When** I follow Example 1 (Basic Thinking Activation)
**Then**:
- curl command executes successfully
- Response includes `<thinking>` blocks
- Budget respected (8000 tokens)
- maxOutputTokens set to 16000

**Validation**: Execute all 5 examples, verify expected behavior

---

## 🧪 Testing Strategy

### Documentation Testing

**Validation Method**: Technical review + hands-on verification

**Checklist**:
- [ ] All code references accurate (file paths, line numbers)
- [ ] All examples executable (curl commands work)
- [ ] Decision tree logic matches implementation
- [ ] Budget calculations correct
- [ ] Conflict scenarios reproducible

### Example Execution Tests

**Test Case 1**: Basic Thinking (Example 1)
```bash
# Execute Example 1 curl command
# Verify: Response contains <thinking> blocks
# Verify: thinking_budget = 8000 in logs
```

**Test Case 2**: Web Search Conflict (Example 5)
```bash
# Execute Example 5 curl command
# Verify: Log shows "[Thinking-Mode] Disabling thinking due to web search conflict"
# Verify: Request routed to gemini-2.5-flash
# Verify: Budget clamped to 24576
```

**Test Case 3**: Budget Clamping
```bash
# Request: thinking_budget = 50000 (exceeds limit)
# Verify: Clamped to 32000
# Verify: Log shows clamping message
```

---

## 🔗 Dependencies and Relationships

### Sequential Dependencies

**Blocks**:
- **Story-005-05** (Document OpenAI Auto-Injection)
  - References thinking activation architecture
  - Extends with OpenAI-specific behavior

- **Story-005-06** (Document First-Time Permissive Mode)
  - References activation flow
  - Adds first-request special handling

- **Story-005-07** (Document maxOutputTokens Auto-Correction)
  - References budget management section
  - Adds safety margin logic

**Blocked By**:
- **Story-005-03** (Error Recovery Docs & Observability)
  - MUST complete first per sequential execution order
  - No technical dependency, organizational only

### Cross-Story References

**Depends On**:
- **Story-005-02**: Profile Presets UI (provides example budgets: 8000, 32000, etc.)

**Referenced By**:
- **Story-005-08**: Configuration Profiles Documentation (will link to thinking activation)

---

## 📊 Success Metrics

### Documentation Quality Metrics

- **Completeness**: 100% of thinking activation features documented
- **Accuracy**: 0 technical errors in code references
- **Examples**: 5 executable configuration examples
- **Troubleshooting**: 3+ common issues with solutions

### User Impact Metrics

- **Integration Time**: -40% (via clear architecture explanation)
- **Support Tickets**: -25% (via troubleshooting guide)
- **Thinking Adoption**: +30% (via profile examples)

---

## ⚠️ Risks and Mitigation

### Risk 1: Documentation Becomes Outdated

**Description**: Thinking activation logic may change, documentation becomes stale

**Probability**: MEDIUM (over time)
**Impact**: MEDIUM (confusion, incorrect implementations)

**Mitigation**:
1. Add code comment: `// 📖 Documented in: docs/features/thinking-activation.md:LINE`
2. Include in PR review checklist: "Update thinking-activation.md if logic changed"
3. Automated doc validation in CI (future enhancement)

### Risk 2: Examples Don't Execute

**Description**: API changes may break curl examples

**Probability**: LOW (API stable)
**Impact**: LOW (examples fail, users frustrated)

**Mitigation**:
1. Include note: "Examples based on API v1 as of 2026-01-11"
2. Provide Postman collection as alternative
3. Automated example testing in CI (future enhancement)

---

## 📝 Notes for Developers

### Documentation Writing Guidelines

1. **Code References**: Include file path + line numbers for all implementation references
2. **Examples**: Provide complete, executable curl commands (not fragments)
3. **Diagrams**: Use ASCII art for decision trees (easy to maintain in markdown)
4. **Versioning**: Note implementation version and date

### Cross-Reference Pattern

**In Code**:
```rust
// 📖 Thinking Activation Documented: docs/features/thinking-activation.md:77-120
let mut is_thinking_enabled = claude_req
    .thinking
    .as_ref()
    .map(|t| t.type_ == "enabled")
    .unwrap_or_else(|| should_enable_thinking_by_default(&claude_req.model));
```

**In Docs**:
```markdown
**Implementation Reference**: `src-tauri/src/proxy/mappers/claude/request.rs:277-362`
```

---

## ✏️ Story Status Tracker

- [ ] **Documentation Creation** (1 hour)
  - [ ] Create `docs/features/thinking-activation.md`
  - [ ] Write Overview section
  - [ ] Write Architectural Differences section (Gemini vs Claude)
  - [ ] Write Activation Flow section (decision tree)
  - [ ] Write Budget Management section (clamping rules)
  - [ ] Write Conflict Detection section (3 conflict types)
  - [ ] Write Configuration Examples (5 examples)
  - [ ] Write Troubleshooting Guide (3+ scenarios)
  - [ ] Add References section (code files, related docs)
  - [ ] Validate all code references
  - [ ] Test all curl examples

**Total Effort**: 1 hour

---

**Story Created**: 2026-01-11
**Last Updated**: 2026-01-11
**Created By**: Documentation Team (Epic-005 Sequential Story Creation)
**Next Story**: Story-005-05 (Document OpenAI Auto-Injection Feature)
