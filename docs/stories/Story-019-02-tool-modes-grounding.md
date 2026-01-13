# Story-019-02: Claude Opus 4.5 Standard - Tool Modes & Grounding

**Epic**: Epic-019 - Claude Opus 4.5 Standard Mode
**Priority**: P1 (HIGH - Feature Parity)
**Effort**: 2-3 days
**Team**: Team 2 (Multi-Protocol Specialists)
**Assignee**: Dev 2B (Mid-Level Protocol Specialist)
**Created**: 2026-01-12
**Status**: 📋 READY FOR DEVELOPMENT
**Dependencies**: Story 019-01 (core implementation complete)

---

## 🎯 Objective

Implement flexible tool configuration modes (AUTO/ANY/NONE) and grounding configuration for **claude-opus-4-5** (standard mode) to achieve 95%+ compliance with complete feature parity.

---

## 📊 Business Context

### Current State
```yaml
problem:
  compliance: "90% (after Story 019-01)"
  missing_features: "2 feature gaps"
  risk: "Limited tool flexibility, no grounding support"
  severity: "P1 HIGH"

gaps:
  gap_4: "Flexible tool modes (AUTO/ANY/NONE) missing"
  gap_5: "Grounding config missing"

impact:
  affected_users: "Users requiring advanced tool control"
  use_cases: "Search integration, function calling control"
  business_impact: "Feature parity gap with competitors"
```

### Target State
```yaml
goal:
  compliance: "95%+ (feature parity complete)"
  missing_features: "0 feature gaps"
  tool_flexibility: "COMPLETE (AUTO/ANY/NONE supported)"
  grounding_support: "OPERATIONAL"

success_metrics:
  - "Tool mode AUTO (default) working"
  - "Tool mode ANY (force enable) working"
  - "Tool mode NONE (disable) working"
  - "Grounding config operational"
  - "15+ tests passing (100%)"
```

---

## 🔍 Gap Analysis

### Critical Missing Features

#### Gap 4: Flexible Tool Modes
```yaml
current:
  tool_modes: "NOT SUPPORTED or basic only"
  flexibility: "Limited control over tool usage"

target:
  tool_modes: "AUTO, ANY, NONE"
  flexibility: "Full control over tool behavior"

modes:
  AUTO:
    description: "Model decides when to use tools"
    use_case: "Default behavior, intelligent tool selection"
    default: true

  ANY:
    description: "Force tool usage (model MUST use a tool)"
    use_case: "Enforce function calling"
    validation: "Error if no tools provided"

  NONE:
    description: "Disable tools completely"
    use_case: "Text-only responses"
    behavior: "Ignore tool definitions"

pattern: "Extend Epic-017 implementation for Opus (modelId 335)"
```

#### Gap 5: Grounding Configuration
```yaml
current:
  grounding: "NOT SUPPORTED"
  search: "No Google Search integration"

target:
  grounding: "Google Search integration"
  search_sources: "Web search, code search"

config:
  google_search_retrieval:
    enabled: true
    dynamic_retrieval_config:
      mode: "MODE_DYNAMIC"
      dynamic_threshold: 0.3

  recitation_policy:
    enabled: true
    policy: "CITED_DOCUMENTS_ONLY"

pattern: "Extend Epic-017 grounding.rs module for Opus"
```

---

## 📋 Acceptance Criteria

### AC1: Tool Mode AUTO (Default) (CRITICAL)
```gherkin
GIVEN a claude-opus-4-5 request with tools
WHEN tool_choice is not specified or set to "auto"
THEN model MUST decide when to use tools
AND tools MUST be available to model
AND model CAN choose to respond without tools
```

**Validation**:
- [ ] Default tool mode is AUTO
- [ ] Tools included in request when provided
- [ ] Model can choose to use or not use tools
- [ ] Test coverage for AUTO mode (3 tests)

---

### AC2: Tool Mode ANY (Force Enable) (CRITICAL)
```gherkin
GIVEN a claude-opus-4-5 request with tools
WHEN tool_choice is set to "any" or {type: "any"}
THEN model MUST use a tool in response
AND request MUST fail if no tools provided
AND model CANNOT respond without tool usage
```

**Validation**:
- [ ] ANY mode forces tool usage
- [ ] Error if no tools provided with ANY mode
- [ ] Model MUST use a tool (validated)
- [ ] Test coverage for ANY mode (3 tests)

---

### AC3: Tool Mode NONE (Disable) (CRITICAL)
```gherkin
GIVEN a claude-opus-4-5 request
WHEN tool_choice is set to "none"
THEN tools MUST be completely disabled
AND model MUST respond with text only
AND tool definitions MUST be ignored
```

**Validation**:
- [ ] NONE mode disables all tools
- [ ] Model responds with text only
- [ ] Tool definitions ignored
- [ ] Test coverage for NONE mode (3 tests)

---

### AC4: Grounding Configuration (HIGH)
```gherkin
GIVEN a claude-opus-4-5 request
WHEN grounding is enabled
THEN Google Search integration MUST be active
AND recitation policy MUST be enforced
AND search results CAN be incorporated into response
```

**Validation**:
- [ ] Google Search retrieval config working
- [ ] Dynamic retrieval mode operational
- [ ] Recitation policy enforced
- [ ] Test coverage for grounding (3 tests)

---

### AC5: Test Coverage (HIGH)
```gherkin
GIVEN tool modes and grounding implementation
WHEN tests are executed
THEN all tool mode variations MUST be tested
AND grounding config MUST be validated
AND integration tests MUST cover all scenarios
```

**Test Requirements**:
- [ ] Unit tests: tool mode detection (5 tests)
- [ ] Unit tests: grounding config (3 tests)
- [ ] Integration tests: tool mode behavior (5 tests)
- [ ] Integration tests: grounding integration (2 tests)
- [ ] All tests passing (100%)

---

## 🛠️ Implementation Details

### File Changes

#### 1. Tool Mode Configuration Extension
**File**: `src-tauri/src/proxy/mappers/claude/request.rs`

**Tool Mode Logic Already Exists from Epic-017** - Extend for Opus:
```rust
// Tool mode enum and parsing already exist from Epic-017
// No changes needed to ToolMode enum or ToolConfiguration struct

// Update build_claude_request() to handle Opus models
pub fn build_claude_request(
    openai_request: &OpenAIRequest,
    model_name: &str,
) -> Result<ClaudeRequest, String> {
    // ... existing code ...

    // Parse tool mode (ALREADY EXISTS from Epic-017)
    let tool_mode = ToolMode::from_openai_tool_choice(&openai_request.tool_choice);

    // Validate tool mode (ALREADY EXISTS from Epic-017)
    if tool_mode == ToolMode::Any && openai_request.tools.is_none() {
        return Err("Tool mode 'any' requires tools to be provided".to_string());
    }

    // Build tool configuration (ALREADY EXISTS from Epic-017)
    let tool_config = if openai_request.tools.is_some() {
        Some(ToolConfiguration::new(tool_mode))
    } else {
        None
    };

    // Build request
    let request = ClaudeRequest {
        model: model_name.to_string(),
        messages: transform_messages(&openai_request.messages)?,

        // Tool configuration (ALREADY EXISTS from Epic-017)
        tool_config,
        tools: if tool_mode == ToolMode::None {
            None  // Ignore tools if mode is NONE
        } else {
            transform_tools(&openai_request.tools)?
        },

        // Metadata (from Story 019-01)
        metadata: Some(Metadata {
            model_id: Some(335),  // Opus standard
            api_provider: Some(26),
            model_provider: Some(3),
            ide_type: Some("ANTIGRAVITY".to_string()),
        }),

        // ... other fields ...
    };

    Ok(request)
}
```

**NOTE**: Tool mode implementation already exists from Epic-017. No code changes needed - just validation that Opus models work with existing implementation.

---

#### 2. Grounding Configuration Extension
**File**: `src-tauri/src/proxy/mappers/claude/grounding.rs` (ALREADY EXISTS from Epic-017)

**Grounding Module Already Complete from Epic-017** - Extend for Opus:
```rust
// Grounding structs and implementation already exist from Epic-017
// No changes needed to GroundingConfig, GoogleSearchRetrieval, etc.

// Update request builder to enable grounding for Opus (same as Sonnet)
// In request.rs:

pub fn build_claude_request(
    openai_request: &OpenAIRequest,
    model_name: &str,
) -> Result<ClaudeRequest, String> {
    // ... existing code ...

    // Add grounding config (ALREADY EXISTS from Epic-017, works for Opus too)
    let grounding_enabled = std::env::var("CLAUDE_GROUNDING_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false);

    let grounding = if grounding_enabled {
        Some(GroundingConfig::new())  // Uses default from Epic-017
    } else {
        None
    };

    let request = ClaudeRequest {
        // ... existing fields ...

        // Grounding configuration (ALREADY EXISTS from Epic-017)
        grounding,

        // ... other fields ...
    };

    Ok(request)
}
```

**NOTE**: Grounding implementation already exists from Epic-017. No code changes needed - just validation that Opus models work with existing implementation.

---

### Test Strategy

#### Unit Tests (8 tests)
**File**: `tests/claude/opus_tool_modes_tests.rs` (NEW)

```rust
#[cfg(test)]
mod opus_tool_mode_tests {
    use super::*;
    use crate::proxy::mappers::claude::request::ToolMode;
    use serde_json::json;

    #[test]
    fn test_opus_tool_mode_default_is_auto() {
        let openai_req = create_test_openai_request("claude-opus-4-5");
        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        // Default should be AUTO
        assert!(claude_req.tool_config.is_none() ||
                claude_req.tool_config.unwrap().function_calling_config.mode == ToolMode::Auto);
    }

    #[test]
    fn test_opus_tool_mode_auto_includes_tools() {
        let mut openai_req = create_test_openai_request("claude-opus-4-5");
        openai_req.tools = Some(vec![create_test_tool()]);
        openai_req.tool_choice = Some(json!("auto"));

        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        assert!(claude_req.tool_config.is_some());
        assert_eq!(claude_req.tool_config.unwrap().function_calling_config.mode, ToolMode::Auto);
        assert!(claude_req.tools.is_some());
    }

    #[test]
    fn test_opus_tool_mode_any_requires_tools() {
        let mut openai_req = create_test_openai_request("claude-opus-4-5");
        openai_req.tool_choice = Some(json!("any"));
        // No tools provided

        let result = build_claude_request(&openai_req, "claude-opus-4-5");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Tool mode 'any' requires tools"));
    }

    #[test]
    fn test_opus_tool_mode_any_with_tools() {
        let mut openai_req = create_test_openai_request("claude-opus-4-5");
        openai_req.tools = Some(vec![create_test_tool()]);
        openai_req.tool_choice = Some(json!("any"));

        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        assert!(claude_req.tool_config.is_some());
        assert_eq!(claude_req.tool_config.unwrap().function_calling_config.mode, ToolMode::Any);
    }

    #[test]
    fn test_opus_tool_mode_none_disables_tools() {
        let mut openai_req = create_test_openai_request("claude-opus-4-5");
        openai_req.tools = Some(vec![create_test_tool()]);
        openai_req.tool_choice = Some(json!("none"));

        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        assert!(claude_req.tools.is_none());  // Tools ignored
    }

    #[test]
    fn test_opus_tool_configuration_serialization() {
        let mut openai_req = create_test_openai_request("claude-opus-4-5");
        openai_req.tools = Some(vec![create_test_tool()]);
        openai_req.tool_choice = Some(json!("any"));

        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();
        let json = serde_json::to_value(&claude_req).unwrap();

        assert!(json.get("toolConfig").is_some());
        assert_eq!(json["toolConfig"]["functionCallingConfig"]["mode"], "ANY");
    }

    #[test]
    fn test_opus_grounding_config_default() {
        use crate::proxy::mappers::claude::grounding::GroundingConfig;

        let config = GroundingConfig::default();
        assert!(config.google_search_retrieval.enabled);
        assert_eq!(config.google_search_retrieval.dynamic_retrieval_config.dynamic_threshold, 0.3);
    }

    #[test]
    fn test_opus_grounding_in_request() {
        std::env::set_var("CLAUDE_GROUNDING_ENABLED", "true");

        let openai_req = create_test_openai_request("claude-opus-4-5");
        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        assert!(claude_req.grounding.is_some());
        let grounding = claude_req.grounding.unwrap();
        assert!(grounding.google_search_retrieval.enabled);

        std::env::remove_var("CLAUDE_GROUNDING_ENABLED");
    }
}
```

---

#### Integration Tests (7 tests)
**File**: `tests/claude/opus_tool_integration_tests.rs` (NEW)

```rust
#[tokio::test]
async fn test_opus_tool_mode_auto_behavior() {
    let mut openai_req = create_test_openai_request("claude-opus-4-5");
    openai_req.tools = Some(vec![create_test_tool()]);
    openai_req.tool_choice = Some(json!("auto"));

    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    assert!(claude_req.tool_config.is_some());
    assert!(claude_req.tools.is_some());
}

#[tokio::test]
async fn test_opus_tool_mode_any_enforcement() {
    let mut openai_req = create_test_openai_request("claude-opus-4-5");
    openai_req.tools = Some(vec![create_test_tool()]);
    openai_req.tool_choice = Some(json!("any"));

    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    assert!(claude_req.tool_config.is_some());
    assert_eq!(claude_req.tool_config.unwrap().function_calling_config.mode, ToolMode::Any);
}

#[tokio::test]
async fn test_opus_tool_mode_none_ignores_tools() {
    let mut openai_req = create_test_openai_request("claude-opus-4-5");
    openai_req.tools = Some(vec![create_test_tool()]);
    openai_req.tool_choice = Some(json!("none"));

    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    assert!(claude_req.tools.is_none());  // Tools completely ignored
}

#[tokio::test]
async fn test_opus_grounding_serialization() {
    std::env::set_var("CLAUDE_GROUNDING_ENABLED", "true");

    let openai_req = create_test_openai_request("claude-opus-4-5");
    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();
    let json = serde_json::to_value(&claude_req).unwrap();

    assert!(json.get("grounding").is_some());
    assert_eq!(json["grounding"]["googleSearchRetrieval"]["enabled"], true);

    std::env::remove_var("CLAUDE_GROUNDING_ENABLED");
}

#[tokio::test]
async fn test_opus_tool_and_grounding_together() {
    std::env::set_var("CLAUDE_GROUNDING_ENABLED", "true");

    let mut openai_req = create_test_openai_request("claude-opus-4-5");
    openai_req.tools = Some(vec![create_test_tool()]);
    openai_req.tool_choice = Some(json!("auto"));

    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    // Both tool config and grounding should be present
    assert!(claude_req.tool_config.is_some());
    assert!(claude_req.grounding.is_some());

    std::env::remove_var("CLAUDE_GROUNDING_ENABLED");
}

#[tokio::test]
async fn test_opus_tool_mode_with_metadata() {
    let mut openai_req = create_test_openai_request("claude-opus-4-5");
    openai_req.tools = Some(vec![create_test_tool()]);
    openai_req.tool_choice = Some(json!("any"));

    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    // Metadata from Story 019-01 should be present
    assert_eq!(claude_req.metadata.as_ref().unwrap().model_id, Some(335));
    // Tool config from Story 019-02 should be present
    assert!(claude_req.tool_config.is_some());
}

#[tokio::test]
async fn test_opus_complex_tool_scenario() {
    let mut openai_req = create_test_openai_request("claude-opus-4-5");
    openai_req.tools = Some(vec![
        create_test_tool_named("get_weather"),
        create_test_tool_named("search_web"),
        create_test_tool_named("calculate"),
    ]);
    openai_req.tool_choice = Some(json!("auto"));

    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    assert!(claude_req.tools.is_some());
    assert_eq!(claude_req.tools.as_ref().unwrap().len(), 3);
}
```

---

## 🔗 Dependencies

### Prerequisite
- ✅ Story 019-01 COMPLETE (core implementation with metadata)
- ✅ Epic-017 COMPLETE (tool modes + grounding implementation)

### Blocks
- 🔴 Story 019-03 (Testing & Documentation) - needs complete feature set

### Enables
- ✅ 95%+ compliance (all features complete)
- ✅ Feature parity with thinking mode
- ✅ Advanced tool control for users

---

## 📊 Success Metrics

### Code Metrics
```yaml
files_created: "2 (opus_tool_modes_tests.rs, opus_tool_integration_tests.rs)"
files_modified: "1 (request.rs - minimal extension)"
code_reuse: "100% (Epic-017 implementation works for Opus)"
test_coverage:
  unit_tests: "8 tests"
  integration_tests: "7 tests"
  total: "15 tests"
```

### Quality Metrics
```yaml
regression_tests: "67/67 passing (Epic-017 baseline)"
story_019_01_tests: "20 tests passing (from Story 019-01)"
new_tests: "15 tests passing (100%)"
code_review: "Mid-Level Protocol (Dev 2B) + Team Lead"
linting: "cargo clippy clean"
```

### Business Metrics
```yaml
compliance_before: "90% (after Story 019-01)"
compliance_after: "95%+ (feature parity complete)"
gaps_resolved: "5/5 (all gaps closed)"
tool_flexibility: "COMPLETE (AUTO/ANY/NONE)"
grounding_support: "OPERATIONAL"
```

---

## 🎯 Definition of Done

- [ ] **Code Complete**:
  - [ ] Opus models work with Epic-017 tool mode implementation
  - [ ] Opus models work with Epic-017 grounding implementation
  - [ ] Request builder integrates tool modes for Opus
  - [ ] Request builder integrates grounding for Opus
  - [ ] Validation for tool mode ANY (requires tools)

- [ ] **Tests Passing**:
  - [ ] 67/67 regression tests passing
  - [ ] 20 tests from Story 019-01 passing
  - [ ] 8 new unit tests passing (100%)
  - [ ] 7 new integration tests passing (100%)

- [ ] **Quality Gates**:
  - [ ] Code review approved (Dev 2B + Team Lead)
  - [ ] Linting clean (cargo clippy)
  - [ ] Formatted correctly (cargo fmt)
  - [ ] No regressions detected

- [ ] **Documentation**:
  - [ ] Test documentation complete
  - [ ] Implementation notes recorded

- [ ] **Deployment Ready**:
  - [ ] Builds successfully
  - [ ] Integration tested with Story 019-01
  - [ ] Ready for Story 019-03 (testing)

---

## 📝 Implementation Notes

### Code Reuse from Epic-017
```yaml
reusable_patterns: "100%"
  - "Tool mode structure" ✅ (no changes needed)
  - "Grounding config structure" ✅ (no changes needed)
  - "Serialization patterns" ✅ (no changes needed)

differences: "None (Opus uses same implementation as Sonnet)"
implementation: "Validation only - no code changes needed"
```

### Performance Considerations
```yaml
overhead: "Minimal (~0.5ms for tool mode parsing)"
grounding_overhead: "Variable (depends on search results)"
serialization: "Standard serde (no custom logic)"
```

### Security Considerations
```yaml
validation: "Tool mode ANY requires tools (error if missing)"
grounding: "Recitation policy enforced"
search_safety: "Google Search integration (safe)"
```

---

**Story Status**: 📋 READY FOR DEVELOPMENT
**Assigned To**: Dev 2B (Mid-Level Protocol Specialist)
**Start Date**: Week 7 Day 3 (after Story 019-01 complete)
**Estimated Completion**: Week 8 Day 2 (2-3 days)
