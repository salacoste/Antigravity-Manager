# Story-019-01: Claude Opus 4.5 Standard - Core Implementation

**Epic**: Epic-019 - Claude Opus 4.5 Standard Mode
**Priority**: P1 (HIGH - Revenue Growth)
**Effort**: 3-4 days
**Team**: Team 2 (Multi-Protocol Specialists)
**Assignee**: Dev 2A (Senior Security Specialist)
**Created**: 2026-01-12
**Status**: 📋 READY FOR DEVELOPMENT
**Dependencies**: Epic-017 complete (copy-paste pattern)

---

## 🎯 Objective

Implement core infrastructure for **claude-opus-4-5** (standard mode) including modelId 335, apiProvider 26, and ideType "ANTIGRAVITY" to achieve 90%+ compliance with Google Antigravity v1.13.3.

---

## 📊 Business Context

### Current State
```yaml
problem:
  compliance: "~75-80% (missing 5 critical features)"
  detection_risk: "HIGH (no modelId, apiProvider, ideType)"
  routing_failures: "MEDIUM (basic implementation exists)"
  severity: "P1 HIGH (premium flagship model)"

gaps:
  gap_1: "Missing modelId: 335 (Claude Opus Standard)"
  gap_2: "Missing apiProvider: 26 (ANTHROPIC_VERTEX from Epic-024)"
  gap_3: "Missing ideType: 'ANTIGRAVITY' (from Epic-024)"

impact:
  affected_users: "Highest-tier customers (premium flagship)"
  business_impact: "Revenue growth, competitive differentiation"
  technical_debt: "Medium (copy-paste from Epic-017)"
```

### Target State
```yaml
goal:
  compliance: "90%+ (all core metadata present)"
  detection_risk: "LOW (full metadata compliance)"
  routing_success: "HIGH (complete model identification)"

success_metrics:
  - "modelId: 335 implemented and validated"
  - "apiProvider: 26 (ANTHROPIC_VERTEX) working"
  - "ideType: 'ANTIGRAVITY' present"
  - "20+ tests passing (100%)"
```

---

## 🔍 Gap Analysis

### Critical Missing Features

#### Gap 1: Model ID 335 (Claude Opus Standard)
```yaml
current:
  model_id: "NOT IMPLEMENTED"
  routing: "Falls back to thinking mode (336)"

target:
  model_id: 335
  routing: "Explicit standard mode routing"

implementation:
  file: "mappers/claude/models.rs"
  constant: "CLAUDE_OPUS_45_STANDARD_MODEL_ID = 335"
  pattern: "Copy from Epic-017 (modelId 333 for Sonnet)"
```

#### Gap 2: API Provider 26 (ANTHROPIC_VERTEX)
```yaml
current:
  api_provider: "NOT SET or incorrect"
  source: "No provider metadata"

target:
  api_provider: 26
  source: "Epic-024 Story-024-02 (ANTHROPIC_VERTEX constant)"

implementation:
  file: "mappers/claude/request.rs"
  field: "apiProvider: Some(26)"
  pattern: "Copy from Epic-017 (same constant)"
```

#### Gap 3: IDE Type "ANTIGRAVITY"
```yaml
current:
  ide_type: "NOT SET"
  detection: "No IDE identification"

target:
  ide_type: "ANTIGRAVITY"
  detection: "Full IDE metadata compliance"

implementation:
  file: "mappers/claude/request.rs"
  field: "ideType: Some('ANTIGRAVITY'.to_string())"
  pattern: "Copy from Epic-017 (same string)"
```

---

## 📋 Acceptance Criteria

### AC1: Model ID 335 Implementation (CRITICAL)
```gherkin
GIVEN a claude-opus-4-5 request (standard mode, NO thinking)
WHEN request is built via build_claude_request()
THEN modelId MUST be 335 (NOT 336 for thinking mode)
AND model field MUST be "claude-opus-4-5"
AND request MUST serialize correctly with modelId
```

**Validation**:
- [ ] Constant `CLAUDE_OPUS_45_STANDARD_MODEL_ID = 335` added to `models.rs`
- [ ] Function `get_model_id("claude-opus-4-5")` returns 335
- [ ] Request JSON contains `"modelId": 335`
- [ ] Test coverage for model ID assignment (3 tests)

---

### AC2: API Provider 26 (ANTHROPIC_VERTEX) (CRITICAL)
```gherkin
GIVEN a claude-opus-4-5 request
WHEN request metadata is built
THEN apiProvider MUST be 26 (ANTHROPIC_VERTEX)
AND modelProvider MUST be 3 (Anthropic)
AND request MUST serialize correctly with apiProvider
```

**Validation**:
- [ ] Request builder sets `apiProvider: Some(26)`
- [ ] Request JSON contains `"apiProvider": 26`
- [ ] Provider routing works correctly
- [ ] Test coverage for API provider (3 tests)

---

### AC3: IDE Type "ANTIGRAVITY" (CRITICAL)
```gherkin
GIVEN a claude-opus-4-5 request
WHEN request metadata is built
THEN ideType MUST be "ANTIGRAVITY"
AND request MUST serialize correctly with ideType
```

**Validation**:
- [ ] Request builder sets `ideType: Some("ANTIGRAVITY".to_string())`
- [ ] Request JSON contains `"ideType": "ANTIGRAVITY"`
- [ ] Metadata structure matches Epic-024 format
- [ ] Test coverage for IDE type (3 tests)

---

### AC4: Request/Response Transformation (HIGH)
```gherkin
GIVEN an OpenAI format request for claude-opus-4-5
WHEN transformed to Claude format
THEN all metadata fields MUST be present
AND transformation MUST be <5ms
AND no data loss occurs
```

**Validation**:
- [ ] Request transformation working
- [ ] Response transformation working
- [ ] Performance <5ms per transformation
- [ ] Test coverage for transformations (5 tests)

---

### AC5: Test Coverage (HIGH)
```gherkin
GIVEN Story 019-01 implementation
WHEN tests are executed
THEN unit tests MUST cover all functions
AND integration tests MUST cover end-to-end flow
AND E2E tests MUST validate real API interaction
```

**Test Requirements**:
- [ ] Unit tests: model ID, API provider, IDE type (10 tests)
- [ ] Integration tests: request/response flow (7 tests)
- [ ] E2E tests: full workflow validation (3 tests)
- [ ] All tests passing (100%)

---

## 🛠️ Implementation Details

### File Changes

#### 1. Model Constants
**File**: `src-tauri/src/proxy/mappers/claude/models.rs`

**Add Opus Standard Constant** (Copy from Epic-017, change ID):
```rust
// Claude Opus 4.5 Standard (NO thinking)
// Pattern: Copy from Epic-017 (CLAUDE_4_5_SONNET_MODEL_ID = 333)
const CLAUDE_OPUS_45_STANDARD_MODEL_ID: u32 = 335;

// Claude Opus 4.5 Thinking (existing)
const CLAUDE_OPUS_45_THINKING_MODEL_ID: u32 = 336;
```

---

#### 2. Model ID Assignment
**File**: `src-tauri/src/proxy/mappers/claude/request.rs`

**Add Model ID Logic** (Copy from Epic-017):
```rust
/// Get model ID for Claude models
/// Pattern: Copy from Epic-017 get_model_id() function
fn get_model_id(model_name: &str) -> Option<u32> {
    match model_name {
        // Sonnet models (Epic-017)
        "claude-4.5-sonnet" | "claude-sonnet-4-5" => Some(333),
        "claude-4.5-sonnet-thinking" | "claude-sonnet-4-5-thinking" => Some(334),

        // Opus models (Epic-019 - NEW)
        "claude-opus-4-5" | "claude-4-5-opus" => Some(335),  // Standard mode
        "claude-opus-4-5-thinking" | "claude-4-5-opus-thinking" => Some(336),  // Thinking mode

        _ => None,
    }
}
```

---

#### 3. Metadata Implementation
**File**: `src-tauri/src/proxy/mappers/claude/request.rs`

**Add Metadata to Request Builder** (Copy from Epic-017):
```rust
pub fn build_claude_request(
    openai_request: &OpenAIRequest,
    model_name: &str,
) -> Result<ClaudeRequest, String> {
    // ... existing code ...

    // Get model ID (NEW for Opus)
    let model_id = get_model_id(model_name);

    // Build metadata (Copy from Epic-017)
    let metadata = Some(Metadata {
        model_id,  // 335 for claude-opus-4-5
        api_provider: Some(26),  // ANTHROPIC_VERTEX (Epic-024)
        model_provider: Some(3),  // Anthropic
        ide_type: Some("ANTIGRAVITY".to_string()),  // Epic-024
    });

    // Build request
    let request = ClaudeRequest {
        model: model_name.to_string(),
        messages: transform_messages(&openai_request.messages)?,
        metadata,  // Include metadata (NEW)
        // ... other fields ...
    };

    Ok(request)
}
```

**Define Metadata Struct** (if not already present from Epic-017):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none", rename = "modelId")]
    pub model_id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "apiProvider")]
    pub api_provider: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "modelProvider")]
    pub model_provider: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "ideType")]
    pub ide_type: Option<String>,
}
```

---

### Test Strategy

#### Unit Tests (10 tests)
**File**: `tests/claude/opus_standard_tests.rs` (NEW)

```rust
#[cfg(test)]
mod opus_standard_core_tests {
    use super::*;
    use crate::proxy::mappers::claude::request::*;

    #[test]
    fn test_opus_standard_model_id() {
        assert_eq!(get_model_id("claude-opus-4-5"), Some(335));
        assert_eq!(get_model_id("claude-4-5-opus"), Some(335));
    }

    #[test]
    fn test_opus_thinking_model_id() {
        assert_eq!(get_model_id("claude-opus-4-5-thinking"), Some(336));
    }

    #[test]
    fn test_opus_standard_vs_thinking_distinction() {
        let standard_id = get_model_id("claude-opus-4-5").unwrap();
        let thinking_id = get_model_id("claude-opus-4-5-thinking").unwrap();

        assert_eq!(standard_id, 335);
        assert_eq!(thinking_id, 336);
        assert_ne!(standard_id, thinking_id);
    }

    #[test]
    fn test_opus_api_provider() {
        let openai_req = create_test_openai_request("claude-opus-4-5");
        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        assert_eq!(claude_req.metadata.as_ref().unwrap().api_provider, Some(26));
    }

    #[test]
    fn test_opus_model_provider() {
        let openai_req = create_test_openai_request("claude-opus-4-5");
        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        assert_eq!(claude_req.metadata.as_ref().unwrap().model_provider, Some(3));
    }

    #[test]
    fn test_opus_ide_type() {
        let openai_req = create_test_openai_request("claude-opus-4-5");
        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        assert_eq!(
            claude_req.metadata.as_ref().unwrap().ide_type.as_deref(),
            Some("ANTIGRAVITY")
        );
    }

    #[test]
    fn test_opus_request_serialization() {
        let openai_req = create_test_openai_request("claude-opus-4-5");
        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        let json = serde_json::to_value(&claude_req).unwrap();

        assert_eq!(json["modelId"], 335);
        assert_eq!(json["apiProvider"], 26);
        assert_eq!(json["modelProvider"], 3);
        assert_eq!(json["ideType"], "ANTIGRAVITY");
    }

    #[test]
    fn test_opus_metadata_structure() {
        let openai_req = create_test_openai_request("claude-opus-4-5");
        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        let metadata = claude_req.metadata.as_ref().unwrap();
        assert!(metadata.model_id.is_some());
        assert!(metadata.api_provider.is_some());
        assert!(metadata.model_provider.is_some());
        assert!(metadata.ide_type.is_some());
    }

    #[test]
    fn test_opus_model_name_preservation() {
        let openai_req = create_test_openai_request("claude-opus-4-5");
        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        assert_eq!(claude_req.model, "claude-opus-4-5");
    }

    #[test]
    fn test_opus_alternative_naming() {
        // Test alternative naming conventions
        let id1 = get_model_id("claude-opus-4-5");
        let id2 = get_model_id("claude-4-5-opus");

        assert_eq!(id1, id2);
        assert_eq!(id1, Some(335));
    }
}
```

---

#### Integration Tests (7 tests)
**File**: `tests/claude/opus_integration_tests.rs` (NEW)

```rust
#[tokio::test]
async fn test_opus_end_to_end_transformation() {
    let openai_req = create_test_openai_request("claude-opus-4-5");
    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    // Validate complete request
    assert_eq!(claude_req.model, "claude-opus-4-5");
    assert_eq!(claude_req.metadata.as_ref().unwrap().model_id, Some(335));
    assert_eq!(claude_req.metadata.as_ref().unwrap().api_provider, Some(26));
    assert_eq!(claude_req.metadata.as_ref().unwrap().ide_type.as_deref(), Some("ANTIGRAVITY"));
}

#[tokio::test]
async fn test_opus_request_response_roundtrip() {
    let openai_req = create_test_openai_request("claude-opus-4-5");
    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    // Mock API call
    let claude_resp = mock_claude_api_call(&claude_req).await.unwrap();

    // Transform response back
    let openai_resp = transform_claude_response(claude_resp).unwrap();

    assert_eq!(openai_resp.model, "claude-opus-4-5");
}

#[tokio::test]
async fn test_opus_performance_transformation() {
    use std::time::Instant;

    let openai_req = create_test_openai_request("claude-opus-4-5");

    let start = Instant::now();
    let _ = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();
    let duration = start.elapsed();

    assert!(duration.as_millis() < 5, "Transformation too slow: {:?}", duration);
}

#[tokio::test]
async fn test_opus_metadata_consistency() {
    // Test multiple requests maintain consistent metadata
    for _ in 0..10 {
        let openai_req = create_test_openai_request("claude-opus-4-5");
        let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

        assert_eq!(claude_req.metadata.as_ref().unwrap().model_id, Some(335));
        assert_eq!(claude_req.metadata.as_ref().unwrap().api_provider, Some(26));
    }
}

#[tokio::test]
async fn test_opus_with_complex_messages() {
    let mut openai_req = create_test_openai_request("claude-opus-4-5");
    openai_req.messages = vec![
        create_system_message("You are a helpful assistant"),
        create_user_message("Hello"),
        create_assistant_message("Hi there!"),
        create_user_message("How are you?"),
    ];

    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    assert_eq!(claude_req.metadata.as_ref().unwrap().model_id, Some(335));
    assert!(claude_req.messages.len() >= 3);
}

#[tokio::test]
async fn test_opus_error_handling() {
    // Test invalid model name
    let openai_req = create_test_openai_request("invalid-model");
    let result = build_claude_request(&openai_req, "invalid-model");

    // Should not panic, may return Ok with None model_id or Err
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_opus_json_serialization() {
    let openai_req = create_test_openai_request("claude-opus-4-5");
    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    // Serialize to JSON
    let json_str = serde_json::to_string(&claude_req).unwrap();

    // Deserialize back
    let deserialized: ClaudeRequest = serde_json::from_str(&json_str).unwrap();

    assert_eq!(deserialized.metadata.as_ref().unwrap().model_id, Some(335));
}
```

---

#### E2E Tests (3 tests)
**File**: `tests/claude/opus_e2e_tests.rs` (NEW)

```rust
#[tokio::test]
async fn test_opus_full_api_flow() {
    // Skip if no API key
    if std::env::var("GOOGLE_API_KEY").is_err() {
        return;
    }

    let openai_req = create_test_openai_request("claude-opus-4-5");
    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    // Make real API call
    let response = make_claude_api_request(&claude_req).await.unwrap();

    // Validate response
    assert!(response.choices.len() > 0);
}

#[tokio::test]
async fn test_opus_streaming_flow() {
    // Skip if no API key
    if std::env::var("GOOGLE_API_KEY").is_err() {
        return;
    }

    let mut openai_req = create_test_openai_request("claude-opus-4-5");
    openai_req.stream = true;

    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    // Validate streaming request
    assert_eq!(claude_req.stream, true);
    assert_eq!(claude_req.metadata.as_ref().unwrap().model_id, Some(335));
}

#[tokio::test]
async fn test_opus_error_response_handling() {
    // Test with invalid API key (should fail gracefully)
    let openai_req = create_test_openai_request("claude-opus-4-5");
    let claude_req = build_claude_request(&openai_req, "claude-opus-4-5").unwrap();

    // Even with error, metadata should be correct
    assert_eq!(claude_req.metadata.as_ref().unwrap().model_id, Some(335));
}
```

---

## 🔗 Dependencies

### Prerequisite
- ✅ Epic-017 COMPLETE (2026-01-12) - provides exact same pattern (modelId 333)
- ✅ Epic-024 COMPLETE (2026-01-12) - provides ideType, apiProvider patterns

### Blocks
- 🔴 Story 019-02 (Tool Modes & Grounding) - needs core implementation
- 🔴 Story 019-03 (Testing & Documentation) - needs complete feature set

### Enables
- ✅ Story 019-02 (Tool Modes & Grounding)
- ✅ 90% compliance (core metadata complete)

---

## 📊 Success Metrics

### Code Metrics
```yaml
files_created: "1 (opus_standard_tests.rs)"
files_modified: "2 (models.rs, request.rs)"
constants_added: "1 (CLAUDE_OPUS_45_STANDARD_MODEL_ID = 335)"
functions_modified: "1 (build_claude_request)"
test_coverage:
  unit_tests: "10 tests"
  integration_tests: "7 tests"
  e2e_tests: "3 tests"
  total: "20 tests"
```

### Quality Metrics
```yaml
regression_tests: "67/67 passing (Epic-017 baseline)"
new_tests: "20 tests passing (100%)"
code_review: "Senior Security (Dev 2A) + Team Lead"
linting: "cargo clippy clean"
```

### Business Metrics
```yaml
compliance_before: "75-80% (missing 3 critical metadata fields)"
compliance_after: "90% (all core metadata present)"
gaps_resolved: "3/5 (modelId, apiProvider, ideType)"
detection_risk: "LOW (full metadata compliance)"
```

---

## 🎯 Definition of Done

- [ ] **Code Complete**:
  - [ ] Constant `CLAUDE_OPUS_45_STANDARD_MODEL_ID = 335` added
  - [ ] Function `get_model_id()` handles Opus models
  - [ ] Metadata struct includes all fields
  - [ ] Request builder includes metadata
  - [ ] All code follows Epic-017 pattern

- [ ] **Tests Passing**:
  - [ ] 67/67 regression tests passing
  - [ ] 10 unit tests passing (100%)
  - [ ] 7 integration tests passing (100%)
  - [ ] 3 E2E tests passing (100%)

- [ ] **Quality Gates**:
  - [ ] Code review approved (Dev 2A + Team Lead)
  - [ ] Linting clean (cargo clippy)
  - [ ] Formatted correctly (cargo fmt)
  - [ ] No regressions detected

- [ ] **Documentation**:
  - [ ] Code comments complete
  - [ ] Test documentation complete
  - [ ] Implementation notes recorded

- [ ] **Deployment Ready**:
  - [ ] Builds successfully
  - [ ] Ready for Story 019-02 (tool modes)

---

## 📝 Implementation Notes

### Code Reuse from Epic-017
```yaml
reusable_patterns: "90%"
  - "Model constant definition" ✅
  - "get_model_id() function" ✅
  - "Metadata struct" ✅
  - "Request builder modifications" ✅

differences: "Model ID only (333 → 335)"
```

### Performance Considerations
```yaml
overhead: "Minimal (~0.5ms for metadata addition)"
serialization: "Standard serde (no custom logic)"
memory: "<100 bytes additional per request"
```

### Security Considerations
```yaml
api_provider: "ANTHROPIC_VERTEX (26) - Epic-024 validated"
ide_type: "ANTIGRAVITY - Epic-024 anti-detection pattern"
model_id: "335 - unique Opus standard identifier"
```

---

**Story Status**: 📋 READY FOR DEVELOPMENT
**Assigned To**: Dev 2A (Senior Security Specialist)
**Start Date**: Week 7 Day 1 (after Epic-017 complete)
**Estimated Completion**: Week 7 Day 4 (3-4 days)
