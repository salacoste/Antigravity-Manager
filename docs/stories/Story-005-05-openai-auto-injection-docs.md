# Story-005-05: Document OpenAI Auto-Injection Feature

**Epic**: Epic-005-Gemini-3-Pro-High-Compliance
**Priority**: LOW
**Effort**: 1 hour
**Type**: DOCS (Documentation Only)
**Status**: PENDING
**Sequential Order**: 5/8 (Must complete Story-005-04 first)

---

## 📋 User Story

**As a** developer using OpenAI SDK to interact with Gemini 3 models
**I want** documentation of automatic thinking injection for Gemini 3 Pro
**So that** I understand why thinking appears in responses without explicit configuration

---

## 🎯 Context and Background

### Current State

**OpenAI Auto-Injection FULLY IMPLEMENTED**:
- ✅ Automatic thinking enablement for Gemini 3 Pro models via OpenAI protocol
- ✅ Default thinking budget: 16000 tokens
- ✅ Works transparently without user configuration
- ✅ Logging shows injection: `"[OpenAI-Request] Injected thinkingConfig for Gemini 3 Pro: thinkingBudget=16000"`

**Documentation Gap**:
- ❌ Feature not documented anywhere
- ❌ Users unaware of automatic thinking injection
- ❌ No explanation why OpenAI requests get thinking by default
- ❌ No guidance on how to disable if needed

**Reference**: `gemini-3-pro-high-COMPARISON.md:709-730`

```yaml
discovery: "Gemini 3 Pro models automatically get thinking enabled via OpenAI protocol"

reference: "src-tauri/src/proxy/mappers/openai/request.rs:247-272"

impact: "POSITIVE - Seamless thinking enablement for OpenAI API users"

recommendation: "✅ ADD to documentation (Example 3 in thinking workflow)"
```

### Why This Matters

**1. User Experience**:
- OpenAI SDK users get extended thinking without extra configuration
- Reduces friction for users migrating from OpenAI to Gemini
- Better out-of-box experience for complex reasoning tasks

**2. Transparency**:
- Users should know about automatic feature injection
- Understanding default behavior prevents confusion
- Clear documentation builds trust

**3. Configuration Control**:
- Users need to know how to customize budget (if needed)
- Users need to know how to disable thinking (if needed)
- Power users want full control over behavior

---

## 🔨 Documentation Specification

### Document to Update

**File**: `docs/features/thinking-activation.md` (UPDATE - created in Story-005-04)

**New Section to Add**: "OpenAI Protocol Auto-Injection" (after "Activation Flow" section)

---

### Section Content: OpenAI Protocol Auto-Injection

```markdown
---

## OpenAI Protocol Auto-Injection 🆕

### Overview

When using the OpenAI-compatible API (`/v1/chat/completions` endpoint), Gemini 3 Pro models automatically receive thinking configuration without explicit user request. This seamless injection provides better reasoning capabilities by default.

**Affected Models**:
- `gemini-3-pro-high` ✅
- `gemini-3-pro-low` ✅
- `gemini-3-flash` ❌ (no auto-injection)
- `gemini-2.5-flash` ❌ (no auto-injection)

**Auto-Injection Behavior**:
```
IF request.protocol == "OpenAI"
   AND model.contains("gemini-3")
   AND (model.ends_with("-high") OR model.ends_with("-low") OR model.contains("-pro")):

    THEN inject:
        thinkingConfig: {
            includeThoughts: true,
            thinkingBudget: 16000
        }
```

### Implementation Details

**Detection Logic**:

**Reference**: `src-tauri/src/proxy/mappers/openai/request.rs:247-250`

```rust
let is_gemini_3_thinking = mapped_model.contains("gemini-3")
    && (mapped_model.ends_with("-high")
        || mapped_model.ends_with("-low")
        || mapped_model.contains("-pro"));
```

**Injection Code**:

**Reference**: `src-tauri/src/proxy/mappers/openai/request.rs:264-272`

```rust
if is_gemini_3_thinking {
    gen_config["thinkingConfig"] = json!({
        "includeThoughts": true,
        "thinkingBudget": 16000  // Default budget for OpenAI protocol
    });
    tracing::debug!(
        "[OpenAI-Request] Injected thinkingConfig for Gemini 3 Pro: thinkingBudget=16000"
    );
}
```

### Why 16000 Token Budget?

**Rationale**: Balanced default for OpenAI SDK users

**Comparison with Other Protocols**:
- **OpenAI Protocol**: 16000 tokens (this feature)
- **Claude Protocol**: 8000 tokens (user-configurable via `thinking.budget_tokens`)
- **Maximum Limit**: 32000 tokens (Gemini 3 Pro High hardware limit)

**Budget Selection Logic**:
1. **OpenAI users** expect GPT-4 level reasoning (typically ~10k-16k tokens)
2. **16000 tokens** provides comprehensive thinking without excessive costs
3. **Higher than Claude default** (8000) because OpenAI protocol lacks explicit thinking parameter
4. **Lower than maximum** (32000) to avoid unexpected cost spikes

### Request Flow Example

**User Request** (OpenAI SDK):
```python
from openai import OpenAI

client = OpenAI(
    base_url="http://localhost:8045/v1",
    api_key="YOUR_API_KEY"
)

response = client.chat.completions.create(
    model="gemini-3-pro-high",
    messages=[
        {"role": "user", "content": "Explain quantum computing"}
    ],
    max_tokens=20000
    # Note: No thinking parameter specified
)
```

**Transformed Request** (to Google v1internal):
```json
{
  "model": "gemini-3-pro-high",
  "request": {
    "contents": [...],
    "generationConfig": {
      "maxOutputTokens": 20000,
      "thinkingConfig": {
        "includeThoughts": true,
        "thinkingBudget": 16000
      }
    }
  }
}
```

**Response** (contains thinking blocks):
```json
{
  "id": "chatcmpl-...",
  "object": "chat.completion",
  "model": "gemini-3-pro-high",
  "choices": [{
    "message": {
      "role": "assistant",
      "content": [
        {
          "type": "thinking",
          "thinking": "<extended reasoning about quantum computing...>",
          "signature": "eyJhbGc..."
        },
        {
          "type": "text",
          "text": "Quantum computing is a revolutionary..."
        }
      ]
    }
  }]
}
```

### Configuration Examples

#### Example 1: Default Auto-Injection (No Configuration)

```bash
curl -X POST http://localhost:8045/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gemini-3-pro-high",
    "messages": [
      {"role": "user", "content": "Design a distributed caching system"}
    ],
    "max_tokens": 20000
  }'
```

**Expected Behavior**:
- ✅ Thinking **automatically injected** with 16000 budget
- ✅ Response includes `<thinking>` blocks with reasoning
- ✅ No extra configuration required
- ✅ Log: `[OpenAI-Request] Injected thinkingConfig for Gemini 3 Pro: thinkingBudget=16000`

#### Example 2: Using with OpenAI Python SDK

```python
from openai import OpenAI

client = OpenAI(
    base_url="http://localhost:8045/v1",
    api_key="YOUR_API_KEY"
)

# Simple request - thinking auto-injected
response = client.chat.completions.create(
    model="gemini-3-pro-high",
    messages=[
        {"role": "user", "content": "Solve this algorithmic problem: ..."}
    ],
    max_tokens=25000
)

# Response automatically includes thinking
for choice in response.choices:
    print(choice.message.content)  # Includes <thinking> blocks
```

**Expected Behavior**:
- ✅ Thinking budget: 16000 tokens (auto-injected)
- ✅ maxOutputTokens: 25000 (user-specified)
- ✅ Seamless experience, no thinking parameter needed

#### Example 3: Streaming with Auto-Injection

```python
from openai import OpenAI

client = OpenAI(
    base_url="http://localhost:8045/v1",
    api_key="YOUR_API_KEY"
)

# Streaming request
stream = client.chat.completions.create(
    model="gemini-3-pro-high",
    messages=[
        {"role": "user", "content": "Explain machine learning algorithms"}
    ],
    max_tokens=30000,
    stream=True
)

# Thinking blocks stream first, then final answer
for chunk in stream:
    if chunk.choices[0].delta.content:
        print(chunk.choices[0].delta.content, end="")
```

**Expected Behavior**:
- ✅ Thinking blocks stream incrementally
- ✅ 16000 token thinking budget applied
- ✅ Total response up to 30000 tokens

#### Example 4: Cost-Optimized Usage (Disable Thinking)

**Problem**: Auto-injection may increase costs for simple queries

**Solution**: Use non-Gemini-3 models to avoid auto-injection

```bash
curl -X POST http://localhost:8045/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gemini-2.5-flash",
    "messages": [
      {"role": "user", "content": "What is 2+2?"}
    ],
    "max_tokens": 100
  }'
```

**Expected Behavior**:
- ✅ No thinking injection (gemini-2.5-flash not in Gemini 3 family)
- ✅ Fast response without extended reasoning
- ✅ Lower token consumption

**Alternative**: Use Claude protocol (`/v1/messages` endpoint) with `thinking.type: "disabled"`

### Comparison: OpenAI vs Claude Protocol

| Feature | OpenAI Protocol | Claude Protocol |
|---------|----------------|-----------------|
| **Endpoint** | `/v1/chat/completions` | `/v1/messages` |
| **Thinking Parameter** | None (auto-injected) | Explicit `thinking` object |
| **Default Budget** | 16000 (auto) | 8000 (user-specified) |
| **Disable Thinking** | Use non-Gemini-3 model | `thinking: {type: "disabled"}` |
| **Budget Customization** | Not supported (fixed 16000) | Fully customizable |
| **SDK Compatibility** | OpenAI SDK | Anthropic SDK |

**When to Use Each**:

**OpenAI Protocol** (`/v1/chat/completions`):
- ✅ Migrating from OpenAI to Gemini
- ✅ Using OpenAI Python/Node SDK
- ✅ Want thinking without extra configuration
- ✅ 16000 token budget is sufficient

**Claude Protocol** (`/v1/messages`):
- ✅ Need fine-grained thinking control
- ✅ Want to customize thinking budget (e.g., 32000 for deep reasoning)
- ✅ Need to disable thinking dynamically
- ✅ Using Anthropic SDK

### Troubleshooting

#### Issue 1: Unexpected Thinking in Responses

**Symptoms**:
- Responses contain `<thinking>` blocks
- Token usage higher than expected
- User didn't request thinking

**Diagnosis**:
```
Check:
1. Model name: gemini-3-pro-high or gemini-3-pro-low?
2. Endpoint: /v1/chat/completions (OpenAI protocol)?
3. Logs: "[OpenAI-Request] Injected thinkingConfig"?

If all YES → Auto-injection is working as designed
```

**Solutions**:
1. **Keep thinking**: Accept the extended reasoning (improves quality)
2. **Disable thinking**: Switch to `gemini-2.5-flash` or `gemini-3-flash`
3. **Use Claude protocol**: `/v1/messages` endpoint with `thinking.type: "disabled"`

#### Issue 2: Want Custom Thinking Budget (Not 16000)

**Symptoms**:
- 16000 token budget insufficient for deep reasoning
- Want 32000 budget for complex problems

**Diagnosis**:
```
OpenAI protocol does NOT support custom thinking budget.
Auto-injection always uses 16000 tokens.
```

**Solutions**:
1. **Switch to Claude protocol**: Use `/v1/messages` endpoint
   ```json
   {
     "model": "gemini-3-pro-high",
     "thinking": {"type": "enabled", "budget_tokens": 32000},
     "messages": [...]
   }
   ```

2. **Use profile presets**: DEEP_THINKING profile (32000 budget) via Claude protocol

3. **Accept limitation**: 16000 tokens sufficient for 90% of use cases

#### Issue 3: Token Usage Tracking

**Symptoms**:
- Hard to track thinking token consumption
- Want to separate thinking cost from response cost

**Diagnosis**:
```
OpenAI protocol response includes usage breakdown:
{
  "usage": {
    "prompt_tokens": 45,
    "completion_tokens": 18234,  // Includes thinking + response
    "total_tokens": 18279
  }
}
```

**Solutions**:
1. **Monitor total_tokens**: Tracks both thinking and response
2. **Use Claude protocol**: Provides detailed `usage.thinking_tokens` breakdown
3. **Review logs**: Debug logs show thinking budget: 16000

---

## Protocol Selection Guide

### Decision Matrix

**Choose OpenAI Protocol** (`/v1/chat/completions`) if:
- ✅ Using OpenAI SDK (Python, Node.js, etc.)
- ✅ Migrating from OpenAI API to Gemini
- ✅ Want automatic thinking without configuration
- ✅ 16000 token budget is sufficient
- ✅ Prefer simpler API surface (no thinking parameter)

**Choose Claude Protocol** (`/v1/messages`) if:
- ✅ Need custom thinking budget (>16000 or <16000)
- ✅ Want to disable thinking dynamically
- ✅ Need detailed token usage breakdown
- ✅ Using Anthropic SDK
- ✅ Require fine-grained control

### Migration Example: OpenAI → Gemini with Thinking

**Before** (OpenAI API):
```python
from openai import OpenAI

client = OpenAI(api_key="sk-...")
response = client.chat.completions.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Complex question"}]
)
```

**After** (Antigravity Manager with auto-injection):
```python
from openai import OpenAI

client = OpenAI(
    base_url="http://localhost:8045/v1",  # ← Point to Antigravity
    api_key="YOUR_API_KEY"
)
response = client.chat.completions.create(
    model="gemini-3-pro-high",  # ← Change model
    messages=[{"role": "user", "content": "Complex question"}]
)
# Thinking automatically enabled with 16000 budget
```

**Migration Benefits**:
- ✅ Minimal code changes (2 lines)
- ✅ Extended thinking enabled by default
- ✅ Better reasoning quality than GPT-4
- ✅ Seamless SDK compatibility

---

## References

### Implementation Files

- **Auto-Injection Logic**: `src-tauri/src/proxy/mappers/openai/request.rs:247-272`
- **Detection Function**: `is_gemini_3_thinking` (lines 247-250)
- **Injection Code**: `thinkingConfig` creation (lines 264-268)
- **Logging**: Debug log (lines 269-271)

### Related Documentation

- **Epic-005**: `docs/epics/Epic-005-Gemini-3-Pro-High-Compliance.md`
- **COMPARISON**: `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-COMPARISON.md:709-730`
- **Story-005-04**: Thinking Activation Architecture (base documentation)
- **Story-005-06**: First-Time Permissive Mode (related thinking feature)
- **Story-005-07**: maxOutputTokens Auto-Correction (related thinking feature)

### External References

- **OpenAI API Documentation**: https://platform.openai.com/docs/api-reference/chat
- **Gemini API Documentation**: https://ai.google.dev/gemini-api/docs/thinking
- **OpenAI Python SDK**: https://github.com/openai/openai-python
```

---

## ✅ Acceptance Criteria

### AC-1: OpenAI Auto-Injection Section Added

**Given** I navigate to `docs/features/thinking-activation.md`
**When** I review the "OpenAI Protocol Auto-Injection" section
**Then** I see:
- Overview explaining automatic thinking injection
- Detection logic with code references
- Injection code with line numbers
- Why 16000 token budget explanation
- Request flow example showing transformation
- 4 configuration examples (default, SDK, streaming, disable)

**Validation**: Documentation peer review + completeness check

### AC-2: Protocol Comparison Table

**Given** I'm deciding between OpenAI and Claude protocols
**When** I review the "Comparison: OpenAI vs Claude Protocol" table
**Then** I see:
- 6 comparison dimensions (endpoint, thinking parameter, budget, etc.)
- Clear "When to Use Each" guidance
- Decision matrix with checkboxes
- Migration example

**Validation**: Survey 3 developers for decision clarity

### AC-3: Configuration Examples Work

**Given** I want to use auto-injection with OpenAI SDK
**When** I follow Example 2 (Python SDK)
**Then**:
- Code executes successfully
- Response includes thinking blocks
- 16000 token budget applied
- No explicit thinking configuration needed

**Validation**: Execute Python example, verify thinking in response

### AC-4: Troubleshooting Scenarios

**Given** I have unexpected thinking in responses
**When** I review "Troubleshooting" section
**Then** I see:
- 3 common issues (unexpected thinking, custom budget, token tracking)
- Diagnosis steps for each issue
- Multiple solution options
- Clear explanations

**Validation**: Follow troubleshooting steps for each scenario

### AC-5: Protocol Selection Guidance

**Given** I'm choosing API protocol
**When** I review "Protocol Selection Guide"
**Then** I see:
- Decision matrix with clear criteria
- 5 reasons to choose OpenAI protocol
- 5 reasons to choose Claude protocol
- Migration example with before/after code

**Validation**: Apply decision matrix to 5 use cases, verify recommendations

### AC-6: Cross-References Accurate

**Given** I'm reading OpenAI auto-injection docs
**When** I follow code references and related docs
**Then**:
- All file paths and line numbers accurate
- Links to related stories work
- Implementation matches documentation
- No broken references

**Validation**: Manually verify all 8 code/doc references

---

## 🧪 Testing Strategy

### Documentation Testing

**Validation Method**: Technical review + hands-on SDK testing

**Checklist**:
- [ ] All code references accurate (line numbers match)
- [ ] All examples executable (Python SDK, curl)
- [ ] Protocol comparison table complete
- [ ] Troubleshooting scenarios reproducible
- [ ] Migration example works

### Example Execution Tests

**Test Case 1**: OpenAI SDK with Auto-Injection
```python
from openai import OpenAI

client = OpenAI(
    base_url="http://localhost:8045/v1",
    api_key="test-key"
)
response = client.chat.completions.create(
    model="gemini-3-pro-high",
    messages=[{"role": "user", "content": "Test"}]
)

# Verify: response contains thinking blocks
# Verify: logs show "[OpenAI-Request] Injected thinkingConfig"
```

**Test Case 2**: Streaming with Auto-Injection
```python
# Execute Example 3 (streaming)
# Verify: thinking blocks stream first
# Verify: final answer streams after thinking
# Verify: total tokens includes thinking budget
```

**Test Case 3**: No Auto-Injection for Gemini 2.5 Flash
```bash
# Request with gemini-2.5-flash model
# Verify: NO thinking injection
# Verify: logs do NOT show thinkingConfig injection
# Verify: response has no thinking blocks
```

---

## 🔗 Dependencies and Relationships

### Sequential Dependencies

**Blocks**:
- **Story-005-08** (Update Configuration Profiles Documentation)
  - May reference OpenAI auto-injection behavior
  - Links to OpenAI protocol guidance

**Blocked By**:
- **Story-005-04** (Document Thinking Activation Architecture)
  - MUST complete first (creates base `thinking-activation.md` file)
  - This story adds new section to that file

### Cross-Story References

**Extends**:
- **Story-005-04**: Adds OpenAI protocol specifics to thinking activation docs

**Referenced By**:
- **Story-005-06**: First-time permissive mode (mentions OpenAI protocol)
- **Story-005-07**: maxOutputTokens auto-correction (applies to OpenAI protocol)

---

## 📊 Success Metrics

### Documentation Quality Metrics

- **Completeness**: 100% of OpenAI auto-injection behavior documented
- **Accuracy**: 0 technical errors in code references
- **Examples**: 4 executable configuration examples
- **Troubleshooting**: 3 common issues with solutions

### User Impact Metrics

- **OpenAI SDK Adoption**: +40% (via migration guidance)
- **Thinking Usage**: +25% (via automatic enablement)
- **Support Tickets**: -30% (via troubleshooting guide)
- **Migration Time**: -50% (via clear examples)

---

## ⚠️ Risks and Mitigation

### Risk 1: Users Confused by Automatic Injection

**Description**: Users may not understand why thinking appears without configuration

**Probability**: MEDIUM
**Impact**: LOW (confusion, support questions)

**Mitigation**:
1. ✅ Clear "Overview" section explaining auto-injection
2. ✅ Prominent note in configuration examples
3. ✅ Troubleshooting section for "unexpected thinking"
4. Add FAQ: "Why do I see thinking blocks without asking for them?"

### Risk 2: Users Want to Disable Auto-Injection

**Description**: Some users prefer no thinking for cost optimization

**Probability**: LOW
**Impact**: LOW (users switch models)

**Mitigation**:
1. ✅ Document workaround: use `gemini-2.5-flash` or Claude protocol
2. ✅ Example 4 shows cost-optimized usage
3. Future enhancement: Add `DISABLE_AUTO_THINKING` env var

### Risk 3: 16000 Budget Insufficient for Some Users

**Description**: Power users may need 32000 token budget

**Probability**: LOW
**Impact**: LOW (users switch to Claude protocol)

**Mitigation**:
1. ✅ Document limitation clearly in "Issue 2" troubleshooting
2. ✅ Provide migration guide to Claude protocol
3. ✅ Explain why 16000 is reasonable default

---

## 📝 Notes for Developers

### Documentation Location

**File**: `docs/features/thinking-activation.md`
**Section**: "OpenAI Protocol Auto-Injection" (new section)
**Position**: After "Activation Flow" section, before "Budget Management"

### Cross-Reference Updates

**In Code** (`openai/request.rs:264-272`):
```rust
// 📖 OpenAI Auto-Injection Documented: docs/features/thinking-activation.md:OpenAI-Protocol-Auto-Injection
if is_gemini_3_thinking {
    gen_config["thinkingConfig"] = json!({
        "includeThoughts": true,
        "thinkingBudget": 16000
    });
}
```

### Testing Recommendations

1. **Test with OpenAI Python SDK**: Verify examples work
2. **Test with OpenAI Node SDK**: Ensure cross-language compatibility
3. **Test streaming**: Verify thinking blocks stream correctly
4. **Test non-Gemini-3**: Ensure no injection for other models

---

## ✏️ Story Status Tracker

- [ ] **Documentation Update** (1 hour)
  - [ ] Add "OpenAI Protocol Auto-Injection" section to `thinking-activation.md`
  - [ ] Write Overview subsection
  - [ ] Write Implementation Details subsection
  - [ ] Write "Why 16000 Token Budget?" explanation
  - [ ] Write Request Flow Example
  - [ ] Write 4 Configuration Examples
  - [ ] Write Protocol Comparison table
  - [ ] Write Troubleshooting section (3 issues)
  - [ ] Write Protocol Selection Guide
  - [ ] Add cross-references to implementation files
  - [ ] Validate all code references
  - [ ] Test Python SDK example

**Total Effort**: 1 hour

---

**Story Created**: 2026-01-11
**Last Updated**: 2026-01-11
**Created By**: Documentation Team (Epic-005 Sequential Story Creation)
**Next Story**: Story-005-06 (Document First-Time Permissive Mode)
