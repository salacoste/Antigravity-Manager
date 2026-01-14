# Story-005-06: Document First-Time Permissive Mode

**Epic**: Epic-005-Gemini-3-Pro-High-Compliance
**Priority**: LOW
**Effort**: 1 hour
**Type**: DOCS (Documentation Only)
**Status**: PENDING
**Sequential Order**: 6/8 (Must complete Story-005-05 first)

---

## 📋 User Story

**As a** developer enabling thinking for the first time
**I want** documentation of lenient signature validation on first requests
**So that** I understand why thinking succeeds initially but may require signatures later

---

## 🎯 Context and Background

### Current State

**First-Time Permissive Mode FULLY IMPLEMENTED**:
- ✅ First thinking requests bypass strict signature validation
- ✅ Upstream API handles signature validation instead of proxy
- ✅ Only enforces strict checks when function calls are involved
- ✅ Logging shows permissive mode: `"[Thinking-Mode] First thinking request detected. Using permissive mode"`

**Documentation Gap**:
- ❌ Feature not documented
- ❌ Users unaware of different validation modes
- ❌ No explanation why first request succeeds without signature
- ❌ No guidance on signature requirements for subsequent requests

**Reference**: `gemini-3-pro-high-COMPARISON.md:733-751`

```yaml
discovery: "First thinking requests use lenient signature validation"

reference: "src-tauri/src/proxy/mappers/claude/request.rs:357-400"

impact: "POSITIVE - Better thinking enablement rate, fewer rejections"

recommendation: "✅ ADD to documentation (Error Handling section)"
```

### Why This Matters

**1. User Experience**:
- First thinking request "just works" without setup
- No signature errors on initial use
- Smoother onboarding experience

**2. Transparency**:
- Users should understand validation behavior changes
- Clear explanation prevents confusion when signatures become required
- Understanding modes helps debug signature errors

**3. Function Call Behavior**:
- Function calls ALWAYS require valid signatures (no permissive mode)
- Understanding this prevents unexpected auto-disable scenarios

---

## 🔨 Documentation Specification

### Document to Update

**File**: `docs/features/thinking-activation.md` (UPDATE - created in Story-005-04)

**New Section to Add**: "First-Time Permissive Mode" (in "Conflict Detection & Auto-Disable" section)

---

### Section Content: First-Time Permissive Mode

```markdown
---

## First-Time Permissive Mode 🆕

### Overview

The first thinking request in a conversation uses **permissive signature validation mode**, allowing thinking to succeed even without pre-existing JWT signatures. Subsequent requests require valid signatures to maintain integrity.

**Mode Logic**:
```
IF first_thinking_request (no thinking blocks in history):
    THEN permissive_mode = true
         → Skip signature validation
         → Let upstream API handle validation

ELSE subsequent_thinking_request (has thinking history):
    THEN strict_mode = true
         → Require valid JWT signatures
         → Auto-disable if signatures missing/invalid
```

**Exception**: Function calls ALWAYS require strict signature validation, even on first request.

### Why Permissive Mode?

**Problem**: Chicken-and-egg signature dilemma

```
User wants thinking → Send request with thinkingConfig
                   → Upstream generates response with signature
                   → Future requests use that signature
```

**Issue**: First request has NO signature yet (nothing to validate against)

**Solution**: Permissive mode allows first request to proceed, upstream provides signature in response

### Implementation Details

**First Request Detection**:

**Reference**: `src-tauri/src/proxy/mappers/claude/request.rs:357-366`

```rust
// Check if there are any thinking blocks in message history
let has_thinking_history = claude_req.messages.iter().any(|m| {
    if m.role == "assistant" {
        if let MessageContent::Array(blocks) = &m.content {
            return blocks
                .iter()
                .any(|b| matches!(b, ContentBlock::Thinking { .. }));
        }
    }
    false
});
```

**Permissive Mode Activation**:

**Reference**: `src-tauri/src/proxy/mappers/claude/request.rs:384-389`

```rust
if !has_thinking_history && is_thinking_enabled {
    tracing::info!(
        "[Thinking-Mode] First thinking request detected. Using permissive mode - \
         signature validation will be handled by upstream API."
    );
}
```

**Strict Mode (Function Calls)**:

**Reference**: `src-tauri/src/proxy/mappers/claude/request.rs:368-400`

```rust
// Check if there are function calls in the request
let has_function_calls = claude_req.messages.iter().any(|m| {
    if let MessageContent::Array(blocks) = &m.content {
        blocks.iter().any(|b| matches!(b, ContentBlock::ToolUse { .. }))
    } else {
        false
    }
});

// For first-time thinking requests, we use permissive mode UNLESS
// function calls are involved (which always need signatures)
let needs_signature_check = has_function_calls;

if needs_signature_check
    && !has_valid_signature_for_function_calls(&claude_req.messages, &global_sig)
{
    tracing::warn!(
        "[Thinking-Mode] No valid signature found for function calls. \
         Disabling thinking to prevent rejection."
    );
    is_thinking_enabled = false;
}
```

### Request Flow Examples

#### Example 1: First Thinking Request (Permissive Mode)

**Request** (first time, no history):
```json
{
  "model": "gemini-3-pro-high",
  "messages": [
    {"role": "user", "content": "Explain quantum physics"}
  ],
  "thinking": {
    "type": "enabled",
    "budget_tokens": 8000
  }
}
```

**Processing**:
```
1. Check history: No thinking blocks found
2. Mode: PERMISSIVE (first request)
3. Signature check: SKIPPED
4. Send to upstream with thinkingConfig
```

**Response** (includes signature):
```json
{
  "role": "assistant",
  "content": [
    {
      "type": "thinking",
      "thinking": "<quantum physics reasoning...>",
      "signature": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
    },
    {
      "type": "text",
      "text": "Quantum physics is the study of..."
    }
  ]
}
```

**Log**:
```
[Thinking-Mode] First thinking request detected. Using permissive mode -
signature validation will be handled by upstream API.
```

#### Example 2: Subsequent Request (Strict Mode)

**Request** (second message, has thinking history):
```json
{
  "model": "gemini-3-pro-high",
  "messages": [
    {
      "role": "user",
      "content": "Explain quantum physics"
    },
    {
      "role": "assistant",
      "content": [
        {
          "type": "thinking",
          "thinking": "<previous reasoning...>",
          "signature": "eyJhbGc..."  // ← Valid signature from first response
        },
        {
          "type": "text",
          "text": "Quantum physics is..."
        }
      ]
    },
    {
      "role": "user",
      "content": "Now explain entanglement"
    }
  ],
  "thinking": {
    "type": "enabled",
    "budget_tokens": 8000
  }
}
```

**Processing**:
```
1. Check history: Thinking blocks FOUND (has signature)
2. Mode: STRICT (subsequent request)
3. Signature check: PASSED (signature valid)
4. Send to upstream with thinkingConfig
```

**Log**:
```
(No permissive mode log - using strict validation)
```

#### Example 3: First Request with Function Calls (Strict Mode)

**Request** (first time, but has function calls):
```json
{
  "model": "gemini-3-pro-high",
  "messages": [
    {"role": "user", "content": "Calculate Pi to 100 digits"},
    {
      "role": "assistant",
      "content": [
        {
          "type": "tool_use",
          "id": "call_123",
          "name": "calculate",
          "input": {"expression": "pi"}
        }
      ]
    }
  ],
  "thinking": {
    "type": "enabled",
    "budget_tokens": 8000
  },
  "tools": [...]
}
```

**Processing**:
```
1. Check history: No thinking blocks (first request)
2. Check function calls: FOUND (tool_use block)
3. Mode: STRICT (function calls override permissive mode)
4. Signature check: FAILED (no signature in history)
5. Action: AUTO-DISABLE thinking
```

**Log**:
```
[Thinking-Mode] No valid signature found for function calls.
Disabling thinking to prevent rejection.
```

**Result**: Request proceeds WITHOUT thinking (thinking auto-disabled)

### Mode Comparison Table

| Scenario | Thinking History? | Function Calls? | Validation Mode | Signature Required? |
|----------|------------------|-----------------|-----------------|---------------------|
| First request, no tools | ❌ No | ❌ No | Permissive | ❌ No |
| First request, with tools | ❌ No | ✅ Yes | Strict | ✅ Yes |
| Subsequent request, no tools | ✅ Yes | ❌ No | Strict | ✅ Yes |
| Subsequent request, with tools | ✅ Yes | ✅ Yes | Strict | ✅ Yes |

**Key Insight**: Only **first request WITHOUT function calls** uses permissive mode

### Benefits of Permissive Mode

**1. Better Enablement Rate**:
- First-time users don't get signature errors
- Thinking "just works" on initial use
- Reduces friction for new users

**2. Natural Signature Bootstrap**:
- First response provides signature
- Future requests use that signature
- Seamless signature lifecycle

**3. Graceful Degradation**:
- Function calls enforce strict validation (safety)
- Simple queries use permissive mode (convenience)
- Balanced approach for different use cases

### Troubleshooting

#### Issue 1: First Request Succeeds, Second Fails

**Symptoms**:
- First thinking request works perfectly
- Second thinking request auto-disables or returns error
- Log shows: "No valid signature found"

**Diagnosis**:
```
First request: PERMISSIVE mode (no signature needed)
Second request: STRICT mode (signature REQUIRED)

Cause: First response signature missing or corrupted
```

**Solutions**:
1. **Check first response**: Verify thinking blocks have valid signatures
   ```json
   {
     "type": "thinking",
     "signature": "eyJhbGc..."  // ← Must be present
   }
   ```

2. **Re-send full history**: Include signature from first response
   ```json
   {
     "messages": [
       {"role": "user", "content": "First question"},
       {
         "role": "assistant",
         "content": [
           {
             "type": "thinking",
             "signature": "eyJhbGc..."  // ← CRITICAL
           },
           {"type": "text", "text": "Answer"}
         ]
       },
       {"role": "user", "content": "Second question"}
     ]
   }
   ```

3. **Start new conversation**: First request will use permissive mode again

#### Issue 2: Function Calls Fail with Thinking

**Symptoms**:
- First request with function calls + thinking fails
- Log shows: "Disabling thinking to prevent rejection"
- No permissive mode message

**Diagnosis**:
```
Function calls ALWAYS require signatures (even first request)
Permissive mode does NOT apply to function calls
```

**Solutions**:
1. **Disable thinking for function calls**:
   ```json
   {
     "thinking": {"type": "disabled"},
     "tools": [...]
   }
   ```

2. **Use thinking after function calls complete**:
   ```
   Step 1: User asks question requiring function call
   Step 2: Execute function call WITHOUT thinking
   Step 3: Use function result with thinking enabled
   ```

3. **Separate thinking and tool use**:
   - Use thinking for reasoning steps
   - Use function calls for data retrieval
   - Don't mix in same request

#### Issue 3: Want to Skip Permissive Mode

**Symptoms**:
- User prefers consistent validation behavior
- Wants strict mode even on first request

**Diagnosis**:
```
Permissive mode is automatic for first requests
Cannot be disabled via configuration
```

**Solutions**:
1. **Accept design**: Permissive mode improves UX
2. **Test with dummy request**: Send throwaway first request to establish signature
   ```python
   # Dummy first request to get signature
   client.messages.create(
       model="gemini-3-pro-high",
       messages=[{"role": "user", "content": "Hi"}],
       thinking={"type": "enabled"}
   )

   # Real requests use strict mode (have signature now)
   real_response = client.messages.create(...)
   ```

3. **Monitor logs**: `[Thinking-Mode] First thinking request detected` indicates permissive mode

---

## Signature Lifecycle

### Complete Flow

```
┌─────────────────────────────────────────────────────────────┐
│ First Request (Permissive Mode)                            │
├─────────────────────────────────────────────────────────────┤
│ User: "Explain quantum physics"                             │
│ Thinking: enabled (8000 budget)                            │
│ History: EMPTY (no thinking blocks)                         │
│ Mode: PERMISSIVE                                            │
│                                                              │
│ → Send to upstream with thinkingConfig                      │
│ → Upstream generates thinking + signature                   │
│ → Response includes JWT signature                           │
└─────────────────────────────────────────────────────────────┘
                         ↓
                   [Signature: eyJhbGc...]
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ Second Request (Strict Mode)                                │
├─────────────────────────────────────────────────────────────┤
│ User: "Now explain entanglement"                            │
│ Thinking: enabled (8000 budget)                            │
│ History: HAS thinking blocks with signature                 │
│ Mode: STRICT                                                │
│                                                              │
│ → Validate signature from first response                    │
│ → Signature VALID → Proceed                                 │
│ → Send to upstream with thinkingConfig                      │
│ → Response includes NEW signature                           │
└─────────────────────────────────────────────────────────────┘
                         ↓
                   [Signature: eyJzdWI...]
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ Third Request (Strict Mode)                                 │
├─────────────────────────────────────────────────────────────┤
│ User: "Compare with classical physics"                      │
│ Thinking: enabled (8000 budget)                            │
│ History: HAS thinking blocks with signatures                │
│ Mode: STRICT                                                │
│                                                              │
│ → Validate signatures from previous responses               │
│ → Signatures VALID → Proceed                                │
│ → Send to upstream with thinkingConfig                      │
└─────────────────────────────────────────────────────────────┘
```

### Signature Propagation

**Key Rule**: Always include assistant thinking blocks (with signatures) in subsequent requests

**Example Multi-Turn Conversation**:

```python
from anthropic import Anthropic

client = Anthropic(
    base_url="http://localhost:8045",
    api_key="YOUR_API_KEY"
)

# Turn 1: First request (permissive mode)
response1 = client.messages.create(
    model="gemini-3-pro-high",
    messages=[
        {"role": "user", "content": "Explain photosynthesis"}
    ],
    thinking={"type": "enabled", "budget_tokens": 8000}
)

# Turn 2: Include response1 thinking blocks (with signatures)
response2 = client.messages.create(
    model="gemini-3-pro-high",
    messages=[
        {"role": "user", "content": "Explain photosynthesis"},
        {
            "role": "assistant",
            "content": response1.content  # ← Includes thinking with signature
        },
        {"role": "user", "content": "How does it differ in C4 plants?"}
    ],
    thinking={"type": "enabled", "budget_tokens": 8000}
)

# Turn 3: Include ALL previous thinking blocks
response3 = client.messages.create(
    model="gemini-3-pro-high",
    messages=[
        {"role": "user", "content": "Explain photosynthesis"},
        {"role": "assistant", "content": response1.content},
        {"role": "user", "content": "How does it differ in C4 plants?"},
        {"role": "assistant", "content": response2.content},
        {"role": "user", "content": "What about CAM plants?"}
    ],
    thinking={"type": "enabled", "budget_tokens": 8000}
)
```

---

## References

### Implementation Files

- **First Request Detection**: `src-tauri/src/proxy/mappers/claude/request.rs:357-366`
- **Permissive Mode Logging**: `src-tauri/src/proxy/mappers/claude/request.rs:384-389`
- **Function Call Detection**: `src-tauri/src/proxy/mappers/claude/request.rs:368-377`
- **Strict Validation**: `src-tauri/src/proxy/mappers/claude/request.rs:391-400`

### Related Documentation

- **Epic-005**: `docs/epics/Epic-005-Gemini-3-Pro-High-Compliance.md`
- **COMPARISON**: `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-COMPARISON.md:733-751`
- **Story-005-04**: Thinking Activation Architecture (base documentation)
- **Story-005-03**: Error Recovery Docs (signature validation errors)

### External References

- **JWT Format**: https://datatracker.ietf.org/doc/html/rfc7519
- **Signature Validation**: `docs/stories/Story-003-05-jwt-signature-validation.md`
```

---

## ✅ Acceptance Criteria

### AC-1: First-Time Permissive Mode Section Added

**Given** I navigate to `docs/features/thinking-activation.md`
**When** I review the "First-Time Permissive Mode" section
**Then** I see:
- Overview explaining permissive vs strict modes
- Implementation details with code references
- Mode comparison table (4 scenarios)
- Signature lifecycle diagram
- 3 request flow examples (permissive, strict, function calls)

**Validation**: Documentation peer review + completeness check

### AC-2: Mode Logic Documented

**Given** I want to understand validation modes
**When** I review the "Mode Logic" pseudocode
**Then** I see:
- Clear IF/THEN logic for mode selection
- Exception for function calls
- Reference to implementation code

**Validation**: Apply logic to 5 test scenarios, verify mode selection

### AC-3: Request Flow Examples Work

**Given** I want to test permissive mode
**When** I execute Example 1 (First Request)
**Then**:
- Request succeeds without signature
- Response includes thinking blocks with signatures
- Log shows "[Thinking-Mode] First thinking request detected. Using permissive mode"

**Validation**: Execute all 3 examples, verify expected behavior

### AC-4: Troubleshooting Scenarios

**Given** I have signature validation errors
**When** I review "Troubleshooting" section
**Then** I see:
- 3 common issues (first succeeds/second fails, function calls, skip permissive)
- Diagnosis steps for each issue
- Multiple solution options with code examples

**Validation**: Follow troubleshooting for each scenario

### AC-5: Signature Lifecycle Documented

**Given** I want to understand signature propagation
**When** I review "Signature Lifecycle" section
**Then** I see:
- Complete multi-turn conversation flow diagram
- Signature propagation rules
- Python code example with 3 turns
- Clear visualization of mode transitions

**Validation**: Execute Python example, verify signatures propagate correctly

### AC-6: Mode Comparison Table Accurate

**Given** I want to know which mode applies
**When** I review the Mode Comparison Table
**Then** I see:
- 4 scenarios (all combinations of history/function calls)
- Validation mode for each scenario
- Signature requirement for each scenario
- Key insight highlighted

**Validation**: Test all 4 scenarios, verify mode matches documentation

---

## 🧪 Testing Strategy

### Documentation Testing

**Validation Method**: Technical review + hands-on testing

**Checklist**:
- [ ] All code references accurate (line numbers match)
- [ ] All examples executable (curl, Python)
- [ ] Mode logic matches implementation
- [ ] Troubleshooting scenarios reproducible
- [ ] Signature lifecycle correct

### Mode Testing

**Test Case 1**: First Request (Permissive)
```python
# Execute Example 1
# Verify: No signature required
# Verify: Log shows permissive mode
# Verify: Response has signature
```

**Test Case 2**: Subsequent Request (Strict)
```python
# Execute Example 2
# Verify: Signature validation happens
# Verify: No permissive mode log
# Verify: Request succeeds with valid signature
```

**Test Case 3**: Function Calls (Strict on First)
```python
# Execute Example 3
# Verify: Strict mode even on first request
# Verify: Thinking auto-disabled if no signature
# Verify: Log shows signature required for function calls
```

---

## 🔗 Dependencies and Relationships

### Sequential Dependencies

**Blocks**:
- **Story-005-08** (Update Configuration Profiles Documentation)
  - May reference permissive mode behavior

**Blocked By**:
- **Story-005-03** (Error Recovery Docs & Observability)
  - References error recovery mechanisms
  - This story extends with signature validation specifics

### Cross-Story References

**Extends**:
- **Story-005-04**: Adds permissive mode details to thinking activation docs

**Referenced By**:
- **Story-003-05**: JWT Signature Validation (original signature validation story)

---

## 📊 Success Metrics

### Documentation Quality Metrics

- **Completeness**: 100% of permissive mode behavior documented
- **Accuracy**: 0 technical errors in mode logic
- **Examples**: 3 executable request flow examples
- **Troubleshooting**: 3 common issues with solutions

### User Impact Metrics

- **First-Time Success Rate**: +40% (via permissive mode)
- **Signature Errors**: -30% (via clear propagation guidance)
- **Support Tickets**: -25% (via troubleshooting guide)

---

## ⚠️ Risks and Mitigation

### Risk 1: Users Confused by Mode Changes

**Description**: Users may not understand why first request succeeds but second fails

**Probability**: MEDIUM
**Impact**: MEDIUM (confusion, support tickets)

**Mitigation**:
1. ✅ Clear "Issue 1" in troubleshooting section
2. ✅ Signature lifecycle diagram
3. ✅ Python example showing multi-turn conversation
4. Add FAQ: "Why does thinking work first time but not second?"

### Risk 2: Function Call Behavior Unclear

**Description**: Users may expect permissive mode to apply to function calls

**Probability**: LOW
**Impact**: MEDIUM (unexpected auto-disable)

**Mitigation**:
1. ✅ Prominent exception note in Overview
2. ✅ Example 3 shows function call strict mode
3. ✅ Mode comparison table highlights function call behavior
4. ✅ "Issue 2" troubleshooting specifically addresses this

### Risk 3: Signature Propagation Not Clear

**Description**: Users may not include thinking blocks in subsequent requests

**Probability**: MEDIUM
**Impact**: HIGH (all subsequent requests fail)

**Mitigation**:
1. ✅ "Signature Propagation" section with clear rules
2. ✅ Python code example showing correct propagation
3. ✅ "Key Rule" highlighted in bold
4. ✅ Multi-turn conversation flow diagram

---

## 📝 Notes for Developers

### Documentation Location

**File**: `docs/features/thinking-activation.md`
**Section**: "First-Time Permissive Mode" (new section)
**Position**: Within "Conflict Detection & Auto-Disable" section

### Cross-Reference Updates

**In Code** (`request.rs:384-389`):
```rust
// 📖 First-Time Permissive Mode Documented: docs/features/thinking-activation.md:First-Time-Permissive-Mode
if !has_thinking_history && is_thinking_enabled {
    tracing::info!(
        "[Thinking-Mode] First thinking request detected. Using permissive mode"
    );
}
```

---

## ✏️ Story Status Tracker

- [ ] **Documentation Update** (1 hour)
  - [ ] Add "First-Time Permissive Mode" section to `thinking-activation.md`
  - [ ] Write Overview (mode logic)
  - [ ] Write Implementation Details (code references)
  - [ ] Write Request Flow Examples (3 examples)
  - [ ] Write Mode Comparison Table
  - [ ] Write Troubleshooting section (3 issues)
  - [ ] Write Signature Lifecycle section (diagram + Python example)
  - [ ] Add cross-references to implementation
  - [ ] Validate all code references
  - [ ] Test Python multi-turn example

**Total Effort**: 1 hour

---

**Story Created**: 2026-01-11
**Last Updated**: 2026-01-11
**Created By**: Documentation Team (Epic-005 Sequential Story Creation)
**Next Story**: Story-005-07 (Document maxOutputTokens Auto-Correction)
