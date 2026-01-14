# Story-004-04: Flexible Tool Configuration Modes for Claude 4.5 Sonnet (Standard)

**Epic**: [Epic-004](../epics/Epic-004-Claude-4.5-Sonnet-Standard-Compliance.md) - Claude 4.5 Sonnet (Standard) 100% Compliance
**Phase**: Phase 2 - Feature Parity (P2)
**Story ID**: Story-004-04
**Status**: ✅ **IMPLEMENTED** (Shared with Epic-003)
**Priority**: P2 (Medium)
**Effort**: 2 hours (validation + 1 integration test)
**Tags**: `[SHARED]`, `compliance`, `tool-configuration`, `[ALREADY-DONE]`, `[TEST-GAP]`

---

## ⚠️ IMPORTANT: Implementation Status

**This story's CORE FUNCTIONALITY is ALREADY IMPLEMENTED** as part of Epic-003 (Story-003-09).

**Current Implementation**:
- ✅ ToolChoice enum: `models.rs:240-281`
- ✅ Tool configuration logic: `request.rs:444-498`
- ✅ AC-13 validation: `request.rs:446-462`
- ✅ 11 comprehensive tests: `request.rs:3086-3500+`

**❌ GAP IDENTIFIED**: All tests use only thinking model (334)
- **Missing**: Integration test for standard model (333) with tool configuration
- **Impact**: No validation that tool modes work correctly for model ID 333

**Story Purpose**: VALIDATION + Add 1 integration test for standard model

---

## Story Overview

### User Story
**As a** developer using the API proxy for tool-enabled workflows with standard Claude 4.5 Sonnet
**I want** flexible tool calling configuration modes (AUTO, ANY, NONE, VALIDATED) to work correctly
**So that** I can control when and how the model uses tools for both standard (333) and thinking (334) models

### Business Value
- **Feature Parity**: Ensures tool configuration works identically for both Claude 4.5 Sonnet variants
- **Quality Assurance**: Validates that standard model (333) routing works with all tool modes
- **Compliance**: Confirms 100% implementation compliance for tool configuration
- **Regression Protection**: Prevents breaking changes to standard model tool configuration

### Related Stories
- **Story-003-09**: Flexible Tool Configuration (Epic-003) ✅ IMPLEMENTED
- **Story-004-01/02/03**: Phase 1 validation stories (Epic-004)
- **Developer Review** (2026-01-11): Story-003-09 improved to 98% completeness (AC-13, AC-14 added)

---

## Problem Statement

### Current State

**Core functionality IMPLEMENTED** for both models:

**ToolChoice Enum** (`models.rs:240-281`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,    // Maps to AUTO mode
    Any,     // Maps to ANY mode
    #[serde(rename = "none")]
    None,    // Maps to NONE mode
    Tool { name: String },  // Maps to VALIDATED + allowedFunctionNames
}

impl ToolChoice {
    pub fn to_gemini_mode(&self) -> &'static str {
        match self {
            ToolChoice::Auto => "AUTO",
            ToolChoice::Any => "ANY",
            ToolChoice::None => "NONE",
            ToolChoice::Tool { .. } => "VALIDATED",
        }
    }
}
```

**Tool Configuration Logic** (`request.rs:467-498`):
```rust
// 🆕 Story #9: Flexible tool calling mode configuration
let tool_choice = claude_req.tool_choice.as_ref();

// Default to VALIDATED for backward compatibility
let mode = tool_choice
    .map(|tc| tc.to_gemini_mode())
    .unwrap_or("VALIDATED");

// Build function calling config
let mut function_calling_config = json!({
    "mode": mode
});

// For specific tool forcing, add allowedFunctionNames
if let Some(tool_name) = tool_choice.and_then(|tc| tc.get_tool_name()) {
    function_calling_config["allowedFunctionNames"] = json!([tool_name]);
}

inner_request["toolConfig"] = json!({
    "functionCallingConfig": function_calling_config
});
```

**AC-13 Validation** (`request.rs:446-462`):
```rust
// AC-13: Validate tool_choice without tools edge case
if let Some(ref tc) = claude_req.tool_choice {
    if tools.is_none() {
        match tc {
            ToolChoice::Auto | ToolChoice::Any | ToolChoice::Tool { .. } => {
                tracing::warn!(
                    "[Tool-Config] ⚠️ tool_choice specified ({:?}) but no tools provided",
                    tc.to_gemini_mode()
                );
            }
            ToolChoice::None => {
                tracing::debug!("[Tool-Config] tool_choice: None (valid without tools)");
            }
        }
    }
}
```

### ❌ GAP IDENTIFIED: Test Coverage

**Existing Tests** (11 total) - All use thinking model:
```rust
// test_auto_mode_mapping (line 3185)
model: "claude-4.5-sonnet-thinking"  // ❌ Only 334

// test_any_mode_mapping (line 3222)
model: "claude-4.5-sonnet-thinking"  // ❌ Only 334

// test_none_mode_mapping (line 3256)
model: "claude-4.5-sonnet-thinking"  // ❌ Only 334

// test_tool_forcing_mode (line 3290)
model: "claude-4.5-sonnet-thinking"  // ❌ Only 334

// ... all 11 tests use thinking model
```

**Missing Coverage**:
- ❌ No integration test verifies standard model (333) with tool configuration
- ❌ No validation that tool modes work correctly for model ID 333
- ❌ Cannot prove tool configuration works identically for both models

### Risk Without Story
- **Regression risk**: Changes could break standard model (333) tool configuration
- **Compliance gap**: No validation that tool modes work for model ID 333
- **Confidence gap**: Cannot verify identical behavior between models
- **Quality gap**: Test coverage incomplete for standard model

---

## Acceptance Criteria

### AC-1: Standard Model Tool Configuration Test
**Verification Method**: Integration test
```rust
#[test]
fn test_standard_model_tool_configuration_modes() {
    // Test AUTO mode for standard model
    let req_auto = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),  // ← Standard model
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("What's the weather?".to_string()),
        }],
        system: None,
        tools: Some(vec![Tool {
            type_: None,
            name: Some("get_weather".to_string()),
            description: Some("Get weather data".to_string()),
            input_schema: Some(json!({"type": "object"})),
        }]),
        stream: false,
        max_tokens: Some(1024),
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: Some(ToolChoice::Auto),
    };

    let (body_auto, _) = transform_claude_request_in(&req_auto, "test-project").unwrap();
    let request_auto = &body_auto["request"];

    // ✅ Verify model ID 333
    assert_eq!(body_auto["modelId"], 333);

    // ✅ Verify AUTO mode for standard model
    assert_eq!(
        request_auto["toolConfig"]["functionCallingConfig"]["mode"],
        "AUTO"
    );
    assert!(request_auto["tools"].is_array());

    // Test ANY mode for standard model
    let mut req_any = req_auto.clone();
    req_any.tool_choice = Some(ToolChoice::Any);

    let (body_any, _) = transform_claude_request_in(&req_any, "test-project").unwrap();
    let request_any = &body_any["request"];

    // ✅ Verify ANY mode for standard model
    assert_eq!(body_any["modelId"], 333);
    assert_eq!(
        request_any["toolConfig"]["functionCallingConfig"]["mode"],
        "ANY"
    );

    // Test NONE mode for standard model
    let mut req_none = req_auto.clone();
    req_none.tool_choice = Some(ToolChoice::None);

    let (body_none, _) = transform_claude_request_in(&req_none, "test-project").unwrap();
    let request_none = &body_none["request"];

    // ✅ Verify NONE mode for standard model
    assert_eq!(body_none["modelId"], 333);
    assert_eq!(
        request_none["toolConfig"]["functionCallingConfig"]["mode"],
        "NONE"
    );

    // Test VALIDATED mode (backward compatibility) for standard model
    let mut req_validated = req_auto.clone();
    req_validated.tool_choice = None;  // No tool_choice → default VALIDATED

    let (body_validated, _) = transform_claude_request_in(&req_validated, "test-project").unwrap();
    let request_validated = &body_validated["request"];

    // ✅ Verify VALIDATED mode (default) for standard model
    assert_eq!(body_validated["modelId"], 333);
    assert_eq!(
        request_validated["toolConfig"]["functionCallingConfig"]["mode"],
        "VALIDATED"
    );
}
```

**Success Criteria**:
- Test passes with standard model name "claude-4.5-sonnet"
- All 4 tool modes (AUTO, ANY, NONE, VALIDATED) work correctly for model ID 333
- Tool configuration identical to thinking model (334) except for model ID
- Backward compatibility verified (None → VALIDATED)

---

### AC-2: Tool Forcing Test for Standard Model
**Verification Method**: Integration test
```rust
#[test]
fn test_standard_model_tool_forcing() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),  // ← Standard model
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Get weather".to_string()),
        }],
        system: None,
        tools: Some(vec![
            Tool {
                type_: None,
                name: Some("get_weather".to_string()),
                description: Some("Get weather".to_string()),
                input_schema: Some(json!({"type": "object"})),
            },
            Tool {
                type_: None,
                name: Some("get_news".to_string()),
                description: Some("Get news".to_string()),
                input_schema: Some(json!({"type": "object"})),
            },
        ]),
        stream: false,
        max_tokens: Some(1024),
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: Some(ToolChoice::Tool {
            name: "get_weather".to_string(),
        }),
    };

    let (body, _) = transform_claude_request_in(&req, "test-project").unwrap();
    let request = &body["request"];

    // ✅ Verify model ID 333
    assert_eq!(body["modelId"], 333);

    // ✅ Verify VALIDATED mode with specific tool for standard model
    assert_eq!(
        request["toolConfig"]["functionCallingConfig"]["mode"],
        "VALIDATED"
    );
    assert_eq!(
        request["toolConfig"]["functionCallingConfig"]["allowedFunctionNames"][0],
        "get_weather"
    );

    // Verify only 1 tool in allowedFunctionNames (per AC-3 from developer review)
    assert_eq!(
        request["toolConfig"]["functionCallingConfig"]["allowedFunctionNames"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}
```

**Success Criteria**:
- Tool forcing works correctly for standard model (333)
- allowedFunctionNames contains specified tool
- Only single tool forced (AC-3 limitation documented)
- VALIDATED mode used with specific tool

---

### AC-3: Standard Model Without Tools + tool_choice Validation
**Verification Method**: Integration test (AC-13 from developer review)
```rust
#[test]
fn test_standard_model_tool_choice_without_tools() {
    // Test Auto mode without tools (should warn)
    let req_auto = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),  // ← Standard model
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Hello".to_string()),
        }],
        system: None,
        tools: None,  // ❌ No tools
        stream: false,
        max_tokens: Some(1024),
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: Some(ToolChoice::Auto),  // ❌ But tool_choice specified
    };

    let (body_auto, _) = transform_claude_request_in(&req_auto, "test-project").unwrap();
    let request_auto = &body_auto["request"];

    // ✅ Should ignore tool_choice when no tools
    assert_eq!(body_auto["modelId"], 333);
    assert!(request_auto.get("toolConfig").is_none());
    assert!(request_auto.get("tools").is_none());

    // Test None mode without tools (valid case)
    let mut req_none = req_auto.clone();
    req_none.tool_choice = Some(ToolChoice::None);

    let (body_none, _) = transform_claude_request_in(&req_none, "test-project").unwrap();
    let request_none = &body_none["request"];

    // ✅ None mode without tools is valid (AC-13)
    assert_eq!(body_none["modelId"], 333);
    assert!(request_none.get("toolConfig").is_none());
}
```

**Success Criteria**:
- AC-13 validation works for standard model (333)
- tool_choice without tools → ignored (with warning)
- tool_choice: None without tools → valid (no warning)
- Standard model handles edge case identically to thinking model

---

## Technical Design

### Implementation Status

**100% Code Sharing with Epic-003-09**:
- ToolChoice enum: Shared (no model-specific logic)
- Tool configuration logic: Shared (no model-specific logic)
- AC-13 validation: Shared (no model-specific logic)
- Helper methods: Shared (to_gemini_mode(), get_tool_name())

**Model Differentiation**: ONLY in model ID (333 vs 334), NOT in tool configuration

### Test Organization

**Test Location**: `src-tauri/src/proxy/mappers/claude/request.rs`

**Existing Tests** (11 tests, all for thinking model):
- Lines 3086-3500+: Story #9 tool configuration tests
- All tests use "claude-4.5-sonnet-thinking"

**New Tests** (3 tests for standard model):
```rust
// Story #4-04: Standard Model (333) Tool Configuration Tests
// Reference: docs/stories/Story-004-04-flexible-tool-configuration-modes.md

#[test]
fn test_standard_model_tool_configuration_modes() { /* AC-1 */ }

#[test]
fn test_standard_model_tool_forcing() { /* AC-2 */ }

#[test]
fn test_standard_model_tool_choice_without_tools() { /* AC-3 */ }
```

### Code Locations
- ToolChoice enum: `models.rs:240-281` (shared)
- Tool configuration: `request.rs:444-498` (shared)
- AC-13 validation: `request.rs:446-462` (shared)
- Existing tests: `request.rs:3086-3500+` (thinking model)
- New tests: `request.rs:3500+` (standard model)

---

## Implementation Plan

### Step 1: Review Existing Implementation (15 min)
- Read Story-003-09 (Epic-003 reference)
- Verify ToolChoice enum implementation
- Confirm tool configuration logic works for both models
- Review AC-13 validation

### Step 2: Add AC-1 Test (30 min)
- Create `test_standard_model_tool_configuration_modes()`
- Test all 4 modes (AUTO, ANY, NONE, VALIDATED) for model 333
- Verify toolConfig structure for each mode
- Most comprehensive test

### Step 3: Add AC-2 Test (25 min)
- Create `test_standard_model_tool_forcing()`
- Test Tool{name} variant for model 333
- Verify allowedFunctionNames array
- Confirm single tool limitation

### Step 4: Add AC-3 Test (20 min)
- Create `test_standard_model_tool_choice_without_tools()`
- Test AC-13 validation for model 333
- Verify edge case handling
- Confirm warning/debug logging

### Step 5: Documentation (10 min)
- Update test count in story
- Mark story as IMPLEMENTED + TEST-GAP
- Update Epic-004 progress

**Total Effort**: ~2 hours (validation + 3 new tests)

---

## Testing Strategy

### Existing Tests (11 tests)
All tests in `request.rs:3086-3500+` use thinking model (334):
- AC-1: ToolChoice enum variants
- AC-2: ClaudeRequest deserialization
- AC-3-7: Mode mapping (AUTO/ANY/NONE/Tool/backward compat)
- AC-8: JSON deserialization
- AC-9: Complex scenarios
- AC-13: tool_choice without tools validation

### New Tests (3 tests for standard model)
- **AC-1**: Comprehensive tool modes test (AUTO, ANY, NONE, VALIDATED)
- **AC-2**: Tool forcing with allowedFunctionNames
- **AC-3**: AC-13 validation for standard model

### Test Execution
```bash
# Run all tool configuration tests
cargo test --package antigravity_tools_lib test_tool

# Run only standard model tests
cargo test --package antigravity_tools_lib test_standard_model_tool

# Run with output
cargo test --package antigravity_tools_lib test_standard_model_tool -- --nocapture
```

### Success Criteria
- All 3 new tests pass ✅
- No regressions in existing 11 tests ✅
- Tool configuration works identically for both models (333 and 334) ✅

---

## Developer Review Integration

**Story-003-09 Review** (2026-01-11):
- ⭐⭐⭐⭐⭐ (5/5) - Excellent quality
- 95% → 98% completeness
- AC-13 added (tool_choice without tools)
- AC-14 added (optional metrics - P3)

**All improvements apply to Story-004-04**:
- ✅ AC-13 validation logic (lines 446-462)
- ✅ 2 AC-13 tests (tool_choice without tools)
- ✅ Implementation Notes clarifications
- ✅ Single tool limitation documented (AC-3)

**Story-004-04 inherits all improvements** due to 100% code sharing.

---

## Risk Assessment

### Technical Risks
- **Risk**: Tests might reveal bugs in standard model routing
- **Mitigation**: Core logic is shared and already tested for thinking model
- **Probability**: Very Low

- **Risk**: Tool configuration might behave differently for model 333
- **Mitigation**: No model-specific logic in tool configuration code
- **Probability**: Very Low

### Implementation Risks
- **Risk**: Tests might duplicate thinking model tests
- **Mitigation**: Tests focus on model ID 333 differentiation, not mode logic
- **Probability**: Low, acceptable for validation

---

## Compliance Impact

### Before Story-004-04
- **Tool Configuration**: ✅ Implemented for both models
- **Tests**: ✅ 11 tests for thinking model (334)
- **Tests**: ❌ 0 tests for standard model (333)
- **Validation**: ❌ No proof tool modes work for model 333
- **Compliance Score**: 90% (tool configuration gap)

### After Story-004-04
- **Tool Configuration**: ✅ Implemented for both models
- **Tests**: ✅ 11 tests for thinking model (334)
- **Tests**: ✅ 3 tests for standard model (333)
- **Validation**: ✅ Proven tool modes work for model 333
- **Compliance Score**: 95% (tool configuration validated)

**Compliance Progression**: 90% → 95%

---

## Dependencies

### Prerequisites
- ✅ Story-003-09: Flexible Tool Configuration (Epic-003) IMPLEMENTED
- ✅ Story-004-01: Model Provider Constants (helper functions)
- ✅ Story-004-02: ideType ANTIGRAVITY Metadata
- ✅ Story-004-03: Model-Specific Routing (model ID 333 tests)

### Blocking Issues
None - all required infrastructure already implemented

### Parallel Work
Can be developed in parallel with Story-004-05 (Grounding Configuration)

---

## Definition of Done

### Code
- [x] Core functionality already implemented (Story-003-09)
- [x] ToolChoice enum supports all modes
- [x] Tool configuration logic shared between models
- [x] AC-13 validation implemented
- [ ] 3 integration tests added for standard model (333)
- [ ] All tests pass without code changes
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings

### Testing
- [ ] All 3 new AC tests pass
- [ ] No regressions in existing 11 tests
- [ ] Tool configuration verified for model 333
- [ ] Test coverage complete for both models

### Documentation
- [x] Story-004-04 documented as IMPLEMENTED + TEST-GAP
- [x] 100% code sharing with Story-003-09 confirmed
- [ ] Epic-004 progress updated
- [ ] Test documentation updated

### Validation
- [x] Tool modes (AUTO/ANY/NONE/Tool) work for both models
- [x] AC-13 validation applies to both models
- [ ] Standard model (333) tool configuration validated
- [ ] Backward compatibility confirmed

---

## Success Metrics

| Metric | Target | Validation |
|--------|--------|------------|
| Code sharing | 100% | No model-specific logic |
| New tests added | 3 | Count in test file |
| Test pass rate | 100% | `cargo test` output |
| Standard model coverage | Complete | All tool modes tested |
| Compliance score | 95% | Gap analysis update |
| No regressions | 0 failures | Existing 11 tests pass |

---

## File Impact Analysis

### Files Modified
**`src-tauri/src/proxy/mappers/claude/request.rs`** (+80 lines)
- **Section**: Test module (after line 3500)
- **Changes**: Add 3 new integration tests for standard model
- **Impact**: Low - test-only changes

**Current File Stats**:
- Total lines: ~3500 lines (after Story-003-09)
- Test lines: ~988 lines (28%)
- After changes: +80 test lines → ~1068 lines (31% tests)

### Files Already Implemented
**`src-tauri/src/proxy/mappers/claude/models.rs`** (no changes needed)
- Lines 240-281: ToolChoice enum (shared)

**`src-tauri/src/proxy/mappers/claude/request.rs`** (no code changes)
- Lines 444-498: Tool configuration logic (shared)
- Lines 3086-3500: Existing tests (thinking model)

---

## Related Documentation

### Epic-004 Documents
- [Epic-004](../epics/Epic-004-Claude-4.5-Sonnet-Standard-Compliance.md) - Master epic
- [Story-004-01](Story-004-01-model-provider-constants.md) - Constants (IMPLEMENTED)
- [Story-004-02](Story-004-02-antigravity-metadata.md) - Metadata (IMPLEMENTED)
- [Story-004-03](Story-004-03-model-specific-routing.md) - Routing (IN PROGRESS)

### Epic-003 Reference (Shared Code)
- [Story-003-09](Story-003-09-flexible-tool-configuration-modes.md) - Tool Configuration (IMPLEMENTED)
  - Developer review (2026-01-11): 5/5 stars, 98% complete
  - AC-13 added, AC-14 added (optional P3)
  - 11 comprehensive tests

### Gap Analysis
- `docs/comparison/claude/claude-4-5-sonnet/current-implementation.md` - Standard model gaps (lines 1112-1166)
- `docs/comparison/claude/claude-4-5-sonnet/EXECUTIVE-SUMMARY.md` - Quick reference

### Reverse Engineering Spec
- `docs/antigravity/workflows/models/claude/claude-4-5-sonnet-workflow.md` - Model ID 333 specification

---

## Implementation Notes

### Why [SHARED] Tag?
- ToolChoice enum: 100% shared, no model-specific logic
- Tool configuration logic: 100% shared, no model-specific logic
- AC-13 validation: 100% shared, no model-specific logic
- Only difference: model ID (333 vs 334) in routing

### Why [TEST-GAP] Tag?
- Core functionality works for both models
- Existing 11 tests only validate thinking model (334)
- Missing validation for standard model (333)
- 3 new tests close the gap

### Relationship to Story-003-09
- **Code**: 100% shared implementation
- **Tests**: Similar structure, different model ID
- **Developer Review**: All improvements inherited
- **AC-13**: Applies to both models identically

### Key Insights
1. **No code changes needed**: Implementation already supports both models
2. **Test gap critical**: Without tests, no proof standard model works
3. **Identical behavior expected**: Tool configuration should work same for 333 and 334
4. **Validation story**: Focus is proving existing implementation works, not building new features

---

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2026-01-11 | Story-004-04 created | BMad Master |
| 2026-01-11 | Verified implementation ALREADY DONE (Story-003-09) | BMad Master |
| 2026-01-11 | Identified test gap: all 11 tests use thinking model only | BMad Master |
| 2026-01-11 | Defined 3 AC for standard model validation | BMad Master |
| 2026-01-11 | Inherited Story-003-09 developer review improvements | BMad Master |

---

## Notes

### Critical Discovery
During deep analysis, discovered that:
1. ✅ Tool configuration fully implemented (Story-003-09)
2. ✅ 11 comprehensive tests exist
3. ❌ **All tests use "claude-4.5-sonnet-thinking"** (model 334)
4. ❌ **ZERO tests for "claude-4.5-sonnet"** (model 333)

This is identical gap pattern as Story-004-03 (model routing tests).

### Why This Matters
- Without tests, cannot prove tool modes work for standard model (333)
- Regression risk: changes could break 333 without detection
- Compliance gap: no validation that 333 routing works with tool configuration
- Quality standard: both models should have equivalent test coverage

### Next Steps After Completion
1. Update Epic-004 progress (Story 4/6 complete)
2. Proceed to Story-004-05 (Grounding Configuration)
3. Validate compliance score increased to 95%
4. Document tool configuration as fully validated for both models
