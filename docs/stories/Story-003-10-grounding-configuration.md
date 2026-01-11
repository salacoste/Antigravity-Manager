# Story-003-10: Grounding Configuration (geminiSettings)

**Story ID**: Story-003-10
**Epic**: [Epic-003](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md) - Claude 4.5 Sonnet Thinking - 100% Compliance
**Priority**: P0 (CRITICAL - Compliance Blocker)
**Estimated Effort**: 1 hour (~15 lines of code + 3 tests)
**Status**: ❌ NOT IMPLEMENTED (CRITICAL GAP) [SHARED]
**Cross-Epic**: Also blocks Epic-004 (Story-004-05)
**Impact**: BLOCKS 100% compliance (currently 95%)
**Blocks**: Story-003-11 (Tool Mode Testing)
**Created**: 2026-01-10
**Updated**: 2026-01-11 (Added [SHARED] tag, NOT IMPLEMENTED)
**Owner**: Engineering Team

---

## 🚨 CRITICAL WARNING

**This feature is NOT IMPLEMENTED and is CRITICAL for anti-detection compliance.**

**Evidence**: `grep -r "geminiSettings\|recitationPolicy" src-tauri/` returns **NO MATCHES**

**Impact**:
- ❌ All requests missing geminiSettings field
- ❌ Clear fingerprint that proxy is not genuine Antigravity
- ❌ Compliance stuck at 95% (cannot reach 100%)
- ❌ Story-003-11 testing BLOCKED until this is implemented

---

## User Story

**As a** developer building the API proxy for 100% Antigravity compliance
**I want** geminiSettings configuration with recitationPolicy in all requests
**So that** our requests are indistinguishable from genuine Antigravity client requests and include anti-plagiarism protection

---

## Context

### Current Situation

**Missing Configuration** (`request.rs:434-446`):
```rust
if !generation_config.is_null() {
    inner_request["generationConfig"] = generation_config;
}

if let Some(tools_val) = tools {
    inner_request["tools"] = tools_val;
    // 显式设置工具配置模式为 VALIDATED
    inner_request["toolConfig"] = json!({
        "functionCallingConfig": {
            "mode": "VALIDATED"
        }
    });
}

// ❌ No geminiSettings field - missing entirely
```

**Issues**:
- ❌ **Missing geminiSettings entirely** - breaks protocol compliance
- ❌ No recitationPolicy configuration - missing anti-plagiarism protection
- ❌ Request structure differs from Antigravity baseline
- ❌ **CRITICAL for anti-detection**: Genuine Antigravity always includes geminiSettings
- ❌ Missing content attribution policy

### Expected Behavior

**Gemini Protocol Specification** (from Gap Analysis lines 3477-3488):
```json
{
  "request": {
    "geminiSettings": {
      "recitationPolicy": {
        "action": "BLOCK",
        "threshold": "LOW"
      }
    }
  }
}
```

**Field Semantics**:
- **geminiSettings**: Gemini-specific configuration container
  - **recitationPolicy**: Controls content recitation behavior
    - **action**: "BLOCK" - Block recited content (anti-plagiarism)
    - **threshold**: "LOW" - Block even low-confidence matches (strict policy)

**Purpose**:
- ✅ **Anti-Plagiarism Protection**: Prevents model from reciting copyrighted content
- ✅ **Content Attribution Policy**: Enforces proper attribution for referenced content
- ✅ **Protocol Compliance**: Required field for Gemini Protocol requests
- ✅ **Anti-Detection**: Matching Antigravity's configuration prevents fingerprinting

### Gap Analysis

**Reference**: Gap Analysis (`current-implementation-thinking.md:3472-3539`) - Gap #7: Grounding Configuration

| Feature | Expected | Current | Gap | Impact |
|---------|----------|---------|-----|--------|
| **geminiSettings** | ✅ Present | ❌ Missing | **Complete** | **HIGH for anti-detection** |
| **recitationPolicy** | ✅ Configured | ❌ Missing | **Complete** | **MEDIUM for content quality** |
| **action: BLOCK** | ✅ Set | ❌ Missing | **Complete** | Missing anti-plagiarism |
| **threshold: LOW** | ✅ Set | ❌ Missing | **Complete** | Missing strict policy |

**Impact**: MEDIUM to HIGH
- **Content Quality**: May affect output quality without recitation policy
- **Anti-Plagiarism**: Missing protection against copyrighted content recitation
- **Protocol Compliance**: Request structure differs from Antigravity baseline
- **Anti-Detection Risk**: **HIGH** - Missing field is a clear fingerprint that our proxy is not genuine Antigravity

**Priority**: P0 - **CRITICAL** - Blocks compliance for both Epic-003 AND Epic-004

**Cross-Epic Impact**:
- ❌ **Story-003-10** (This Story): NOT IMPLEMENTED for model 334
- ❌ **Story-004-05**: NOT IMPLEMENTED for model 333
- 🔗 Same code applies to both models (100% shared implementation)
- 🚨 Affects ALL Claude 4.5 Sonnet requests (thinking and standard)

---

## Reference Documentation

### Primary Analysis
- **Gap Analysis**: `docs/comparison/claude/claude-4-5-sonnet/current-implementation-thinking.md`
  - Lines 3472-3539: Complete Gap #7 analysis
  - Lines 3508-3517: Gap table and impact assessment
  - Lines 3519-3538: Required changes with code example

### Current Implementation
- **Request Mapper**: `src-tauri/src/proxy/mappers/claude/request.rs`
  - Lines 434-446: Current structure (generationConfig → toolConfig)
  - Missing: geminiSettings field between or after toolConfig

### Upstream Specification
- **Gemini Protocol**: Vertex AI v1internal API
  - Field: `geminiSettings.recitationPolicy`
  - Values: action ("BLOCK"), threshold ("LOW")

---

## Technical Details

### Architecture Overview

**Request Structure Enhancement**:
```
┌─────────────────────────────────────────────────────────────┐
│ Current Request Structure                                    │
│                                                              │
│ {                                                            │
│   "contents": [...],                                         │
│   "safetySettings": [...],                                   │
│   "systemInstruction": {...},                                │
│   "generationConfig": {...},                                 │
│   "toolConfig": {...}                                        │
│ }                                                            │
│                                                              │
│ ❌ Missing: geminiSettings                                   │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼ Add geminiSettings
┌─────────────────────────────────────────────────────────────┐
│ Enhanced Request Structure (100% Compliance)                 │
│                                                              │
│ {                                                            │
│   "contents": [...],                                         │
│   "safetySettings": [...],                                   │
│   "systemInstruction": {...},                                │
│   "generationConfig": {...},                                 │
│   "toolConfig": {...},                                       │
│   "geminiSettings": {           // 🆕 New field              │
│     "recitationPolicy": {                                    │
│       "action": "BLOCK",        // Anti-plagiarism           │
│       "threshold": "LOW"        // Strict policy             │
│     }                                                        │
│   }                                                          │
│ }                                                            │
│                                                              │
│ ✅ Indistinguishable from Antigravity                        │
└─────────────────────────────────────────────────────────────┘
```

### Implementation Steps

#### Step 1: Add geminiSettings After toolConfig (`request.rs`)

**Location**: `src-tauri/src/proxy/mappers/claude/request.rs:446` (after toolConfig)

**Add Configuration**:
```rust
if let Some(tools_val) = tools {
    inner_request["tools"] = tools_val;
    // 显式设置工具配置模式为 VALIDATED
    inner_request["toolConfig"] = json!({
        "functionCallingConfig": {
            "mode": "VALIDATED"
        }
    });
}

// 🆕 Add Gemini Settings (Anti-Plagiarism Protection)
// Required for 100% Antigravity compliance and anti-detection
inner_request["geminiSettings"] = json!({
    "recitationPolicy": {
        "action": "BLOCK",      // Block recited content
        "threshold": "LOW"      // Strict matching threshold
    }
});

tracing::debug!(
    "[Gemini-Settings] Added recitationPolicy: action=BLOCK, threshold=LOW for anti-plagiarism protection"
);
```

**Why This Location**:
- ✅ **After toolConfig**: Logical grouping of Gemini-specific settings
- ✅ **Before inject_google_search**: Standard fields before dynamic injection
- ✅ **Consistent order**: Matches Antigravity request structure
- ✅ **Always present**: Not conditional, always added for compliance

**Why These Values**:
- ✅ **action: "BLOCK"**: Strictest anti-plagiarism policy
- ✅ **threshold: "LOW"**: Block even low-confidence recitation matches
- ✅ **Matches Antigravity**: Exact values from reverse engineering
- ✅ **Anti-Detection**: Prevents fingerprinting through missing or different values

#### Step 2: Add Debug Logging (Included Above)

**Logging Strategy**:
```rust
tracing::debug!(
    "[Gemini-Settings] Added recitationPolicy: action=BLOCK, threshold=LOW for anti-plagiarism protection"
);
```

**Why Debug Level**:
- ✅ **Not user-facing**: Implementation detail, not error or warning
- ✅ **Troubleshooting**: Helps verify field is added correctly
- ✅ **Performance**: Debug logs can be disabled in production
- ✅ **Consistency**: Matches other configuration logging

### Recitation Policy Semantics

**What is Recitation?**
- **Definition**: Model reproducing large portions of content from training data
- **Risk**: Copyright infringement, plagiarism, content attribution issues
- **Detection**: Gemini backend detects high similarity to known content

**Policy Values**:

| Action | Threshold | Behavior | Use Case |
|--------|-----------|----------|----------|
| BLOCK | LOW | Block even low-confidence matches | **Strictest** (Antigravity default) |
| BLOCK | MEDIUM | Block medium-confidence matches | Balanced approach |
| BLOCK | HIGH | Block only high-confidence matches | Permissive |
| ALLOW | N/A | Allow all recitations (no blocking) | Research/testing only |

**Why BLOCK + LOW**:
- ✅ **Maximum Protection**: Strictest anti-plagiarism policy
- ✅ **Antigravity Standard**: Matches reverse-engineered configuration
- ✅ **Content Safety**: Reduces copyright infringement risk
- ✅ **Quality Signal**: Indicates production-grade client

### Anti-Detection Rationale

**Why This Field is Critical for Anti-Detection**:

1. **Always Present in Antigravity**: RE analysis confirms geminiSettings in 100% of requests
2. **Fingerprinting Vector**: Missing field is detectable by backend analytics
3. **Configuration Signature**: Specific values (BLOCK+LOW) match Antigravity profile
4. **Protocol Compliance**: Required for Gemini Protocol v1internal

**Detection Risk Without geminiSettings**:
```
Backend Analytics:
  IF geminiSettings is missing:
    → Flag as non-Antigravity client
    → Potential for rate limiting or blocking
    → Analytics show "custom client" pattern
```

**With geminiSettings**:
```
Backend Analytics:
  IF geminiSettings.recitationPolicy.action == "BLOCK"
    AND geminiSettings.recitationPolicy.threshold == "LOW":
    → Matches Antigravity baseline
    → No fingerprinting signal
    → Analytics show "standard client" pattern
```

---

## Acceptance Criteria

### AC-1: geminiSettings Field Added ✅

**Given** the request mapping function
**When** I build the inner_request
**Then** it should:
- ✅ Include `geminiSettings` field
- ✅ Include `recitationPolicy` nested object
- ✅ Set `action` to "BLOCK"
- ✅ Set `threshold` to "LOW"
- ✅ Place field after `toolConfig` (or appropriate location)

**Validation**:
```rust
#[test]
fn test_gemini_settings_present() {
    let claude_req = create_test_claude_request();
    let config = create_test_config();
    let inner_request = map_claude_to_vertex_ai(&claude_req, &config, "test-trace");

    // Verify geminiSettings exists
    assert!(inner_request.get("geminiSettings").is_some());

    // Verify recitationPolicy structure
    let gemini_settings = &inner_request["geminiSettings"];
    assert_eq!(gemini_settings["recitationPolicy"]["action"], "BLOCK");
    assert_eq!(gemini_settings["recitationPolicy"]["threshold"], "LOW");
}
```

### AC-2: Always Present (Not Conditional) ✅

**Given** any request configuration
**When** I map Claude request to Gemini
**Then** geminiSettings should:
- ✅ Always be present (not conditional on tools, thinking, etc.)
- ✅ Use same values for all requests (action=BLOCK, threshold=LOW)
- ✅ Not depend on client configuration
- ✅ Match Antigravity baseline consistently

**Validation**:
```rust
#[test]
fn test_gemini_settings_always_present() {
    // Test 1: Request without tools
    let mut req1 = create_test_claude_request();
    req1.tools = None;
    let inner1 = map_claude_to_vertex_ai(&req1, &create_test_config(), "test-1");
    assert!(inner1.get("geminiSettings").is_some());

    // Test 2: Request without thinking
    let mut req2 = create_test_claude_request();
    req2.thinking = None;
    let inner2 = map_claude_to_vertex_ai(&req2, &create_test_config(), "test-2");
    assert!(inner2.get("geminiSettings").is_some());

    // Test 3: Minimal request
    let req3 = ClaudeRequest {
        model: "claude-4.5-sonnet-thinking".to_string(),
        messages: vec![],
        stream: false,
        system: None,
        tools: None,
        max_tokens: Some(1024),
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };
    let inner3 = map_claude_to_vertex_ai(&req3, &create_test_config(), "test-3");
    assert!(inner3.get("geminiSettings").is_some());
}
```

### AC-3: Correct JSON Structure ✅

**Given** the geminiSettings field
**When** I serialize to JSON
**Then** it should match:
```json
{
  "geminiSettings": {
    "recitationPolicy": {
      "action": "BLOCK",
      "threshold": "LOW"
    }
  }
}
```

**Validation**:
```rust
#[test]
fn test_gemini_settings_json_structure() {
    let claude_req = create_test_claude_request();
    let config = create_test_config();
    let inner_request = map_claude_to_vertex_ai(&claude_req, &config, "test-trace");

    // Serialize and verify structure
    let json_str = serde_json::to_string_pretty(&inner_request).unwrap();
    assert!(json_str.contains("\"geminiSettings\""));
    assert!(json_str.contains("\"recitationPolicy\""));
    assert!(json_str.contains("\"action\": \"BLOCK\""));
    assert!(json_str.contains("\"threshold\": \"LOW\""));

    // Verify nested structure
    let gemini_settings = &inner_request["geminiSettings"];
    assert!(gemini_settings.is_object());
    assert!(gemini_settings["recitationPolicy"].is_object());
}
```

### AC-4: Debug Logging Present ✅

**Given** the geminiSettings addition
**When** I build the request
**Then** it should:
- ✅ Log debug message with `tracing::debug!()`
- ✅ Include prefix `[Gemini-Settings]` for filtering
- ✅ Mention action=BLOCK and threshold=LOW
- ✅ Reference anti-plagiarism purpose

**Validation**:
```rust
#[test]
fn test_gemini_settings_logging() {
    // Initialize test logger
    let _guard = init_test_logging();

    let claude_req = create_test_claude_request();
    let config = create_test_config();
    map_claude_to_vertex_ai(&claude_req, &config, "test-trace");

    // Verify log output (implementation-specific)
    // Should contain: "[Gemini-Settings]", "BLOCK", "LOW", "anti-plagiarism"
}
```

### AC-5: No Breaking Changes ✅

**Given** existing request mapping functionality
**When** I add geminiSettings
**Then** it should:
- ✅ Not affect existing field generation
- ✅ Not change contents, safetySettings, generationConfig
- ✅ Not break tool configuration
- ✅ Not affect thinking mode configuration
- ✅ Maintain backward compatibility

**Validation**:
```rust
#[test]
fn test_no_breaking_changes() {
    let claude_req = create_test_claude_request_with_all_fields();
    let config = create_test_config();
    let inner_request = map_claude_to_vertex_ai(&claude_req, &config, "test-trace");

    // Verify all existing fields still present
    assert!(inner_request.get("contents").is_some());
    assert!(inner_request.get("safetySettings").is_some());
    assert!(inner_request.get("generationConfig").is_some());
    assert!(inner_request.get("toolConfig").is_some());

    // Verify new field added
    assert!(inner_request.get("geminiSettings").is_some());
}
```

### AC-6: Field Ordering Compliance ✅

**Given** the complete request structure
**When** I compare field order with Antigravity
**Then** geminiSettings should:
- ✅ Be placed logically in request structure
- ✅ Match or approximate Antigravity field order
- ✅ Not disrupt existing field ordering

**Validation**: Manual inspection of serialized JSON output

### AC-7: Anti-Detection Validation ✅

**Given** a complete request with geminiSettings
**When** I compare with Antigravity baseline
**Then** the request should:
- ✅ Include identical geminiSettings structure
- ✅ Use identical values (action=BLOCK, threshold=LOW)
- ✅ Be indistinguishable in backend analytics
- ✅ Pass fingerprinting detection tests

**Validation**:
- Manual comparison with RE documentation
- Integration testing with live Vertex AI endpoint
- Backend response inspection for anomalies

### AC-8: Compliance Score Improvement ✅

**Given** this story's implementation
**When** I complete geminiSettings addition
**Then** it should:
- ✅ Close Gap #7: Grounding Configuration
- ✅ Increase compliance score from 96.67% → 98.33%
- ✅ Pass manual validation against RE spec
- ✅ Reduce anti-detection risk

**Validation**: Gap Analysis checklist completion

---

## Testing Strategy

### Unit Tests (5 tests)

**File**: `src-tauri/src/proxy/mappers/claude/request_tests.rs`

```rust
#[cfg(test)]
mod gemini_settings_tests {
    use super::*;

    #[test]
    fn test_gemini_settings_present() {
        // AC-1: Verify geminiSettings field exists with correct structure
        let claude_req = create_test_claude_request();
        let config = create_test_config();
        let inner_request = map_claude_to_vertex_ai(&claude_req, &config, "test-trace");

        assert!(inner_request.get("geminiSettings").is_some());

        let gemini_settings = &inner_request["geminiSettings"];
        assert_eq!(gemini_settings["recitationPolicy"]["action"], "BLOCK");
        assert_eq!(gemini_settings["recitationPolicy"]["threshold"], "LOW");
    }

    #[test]
    fn test_gemini_settings_always_present() {
        // AC-2: Verify geminiSettings present in all request types

        // Test without tools
        let mut req1 = create_test_claude_request();
        req1.tools = None;
        let inner1 = map_claude_to_vertex_ai(&req1, &create_test_config(), "test-1");
        assert!(inner1.get("geminiSettings").is_some());

        // Test without thinking
        let mut req2 = create_test_claude_request();
        req2.thinking = None;
        let inner2 = map_claude_to_vertex_ai(&req2, &create_test_config(), "test-2");
        assert!(inner2.get("geminiSettings").is_some());

        // Test minimal request
        let req3 = ClaudeRequest {
            model: "claude-4.5-sonnet-thinking".to_string(),
            messages: vec![],
            stream: false,
            system: None,
            tools: None,
            max_tokens: Some(1024),
            temperature: None,
            top_p: None,
            top_k: None,
            thinking: None,
            metadata: None,
            output_config: None,
            tool_choice: None,
        };
        let inner3 = map_claude_to_vertex_ai(&req3, &create_test_config(), "test-3");
        assert!(inner3.get("geminiSettings").is_some());
    }

    #[test]
    fn test_gemini_settings_json_structure() {
        // AC-3: Verify correct JSON serialization
        let claude_req = create_test_claude_request();
        let config = create_test_config();
        let inner_request = map_claude_to_vertex_ai(&claude_req, &config, "test-trace");

        // Serialize to JSON
        let json_str = serde_json::to_string_pretty(&inner_request).unwrap();

        // Verify structure in JSON output
        assert!(json_str.contains("\"geminiSettings\""));
        assert!(json_str.contains("\"recitationPolicy\""));
        assert!(json_str.contains("\"action\": \"BLOCK\""));
        assert!(json_str.contains("\"threshold\": \"LOW\""));

        // Verify nested object structure
        let gemini_settings = &inner_request["geminiSettings"];
        assert!(gemini_settings.is_object());
        assert!(gemini_settings["recitationPolicy"].is_object());
        assert!(gemini_settings["recitationPolicy"]["action"].is_string());
        assert!(gemini_settings["recitationPolicy"]["threshold"].is_string());
    }

    #[test]
    fn test_no_breaking_changes() {
        // AC-5: Verify existing fields not affected
        let claude_req = create_test_claude_request_with_all_fields();
        let config = create_test_config();
        let inner_request = map_claude_to_vertex_ai(&claude_req, &config, "test-trace");

        // Verify all existing fields still present
        assert!(inner_request.get("contents").is_some());
        assert!(inner_request.get("safetySettings").is_some());
        assert!(inner_request.get("generationConfig").is_some());
        assert!(inner_request.get("toolConfig").is_some());
        assert!(inner_request.get("systemInstruction").is_some());

        // Verify new field added
        assert!(inner_request.get("geminiSettings").is_some());

        // Verify field values unchanged
        assert!(inner_request["contents"].is_array());
        assert!(inner_request["safetySettings"].is_array());
        assert!(inner_request["generationConfig"].is_object());
    }

    #[test]
    fn test_recitation_policy_values() {
        // Verify exact values match Antigravity spec
        let claude_req = create_test_claude_request();
        let config = create_test_config();
        let inner_request = map_claude_to_vertex_ai(&claude_req, &config, "test-trace");

        let policy = &inner_request["geminiSettings"]["recitationPolicy"];

        // Exact string matching (case-sensitive)
        assert_eq!(policy["action"].as_str().unwrap(), "BLOCK");
        assert_eq!(policy["threshold"].as_str().unwrap(), "LOW");

        // Verify no extra fields
        assert_eq!(policy.as_object().unwrap().len(), 2);
    }

    // Test helper functions
    fn create_test_claude_request() -> ClaudeRequest {
        ClaudeRequest {
            model: "claude-4.5-sonnet-thinking".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: MessageContent::String("Test message".to_string()),
            }],
            stream: false,
            system: None,
            tools: Some(vec![create_test_tool("test_tool")]),
            max_tokens: Some(1024),
            temperature: None,
            top_p: None,
            top_k: None,
            thinking: Some(ThinkingConfig {
                type_: "enabled".to_string(),
                budget_tokens: Some(32000),
            }),
            metadata: None,
            output_config: None,
            tool_choice: None,
        }
    }

    fn create_test_claude_request_with_all_fields() -> ClaudeRequest {
        // Create request with all possible fields populated
        // ...
    }

    fn create_test_tool(name: &str) -> serde_json::Value {
        json!({
            "type": "function",
            "function": {
                "name": name,
                "description": "Test tool",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            }
        })
    }

    fn create_test_config() -> ProxyConfig {
        // Create minimal config for testing
        // ...
    }
}
```

### Integration Tests (1 test)

**File**: `src-tauri/src/proxy/handlers/claude_tests.rs`

```rust
#[cfg(test)]
mod gemini_settings_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_end_to_end_gemini_settings() {
        // AC-7: Test full request/response flow with geminiSettings
        let request_body = json!({
            "model": "claude-4.5-sonnet-thinking",
            "messages": [{
                "role": "user",
                "content": "Hello, world!"
            }],
            "max_tokens": 1024
        });

        // Send request through handler
        // Verify geminiSettings in upstream request
        // Verify response format unchanged
        // Verify no errors from backend
    }
}
```

### Manual Testing Checklist

- [ ] **Request Inspection**:
  - [ ] Verify geminiSettings in serialized JSON
  - [ ] Confirm action=BLOCK, threshold=LOW
  - [ ] Check field ordering matches expectations

- [ ] **Backend Testing**:
  - [ ] Send request to Vertex AI
  - [ ] Verify no rejection or error
  - [ ] Check response quality (recitation policy active)

- [ ] **Compliance Testing**:
  - [ ] Compare with Antigravity baseline request
  - [ ] Verify identical geminiSettings structure
  - [ ] Confirm no fingerprinting signals

- [ ] **Regression Testing**:
  - [ ] Test requests without tools
  - [ ] Test requests without thinking
  - [ ] Test minimal requests
  - [ ] Verify all existing features work

---

## Definition of Done

### Code Quality
- [ ] ✅ geminiSettings added after toolConfig
- [ ] ✅ Hardcoded values: action="BLOCK", threshold="LOW"
- [ ] ✅ Debug logging added with [Gemini-Settings] prefix
- [ ] ✅ Code follows Rust best practices
- [ ] ✅ No clippy warnings
- [ ] ✅ Code formatted with rustfmt

### Testing
- [ ] ✅ All 5 unit tests passing
- [ ] ✅ Integration test passing
- [ ] ✅ Manual testing completed
- [ ] ✅ Backward compatibility validated
- [ ] ✅ No regressions in existing functionality

### Documentation
- [ ] ✅ Code comments added explaining purpose
- [ ] ✅ Debug logging messages clear
- [ ] ✅ Story marked as completed
- [ ] ✅ Gap Analysis updated

### Validation
- [ ] ✅ Gap #7 closed in Gap Analysis
- [ ] ✅ Compliance score updated: 96.67% → 98.33%
- [ ] ✅ All AC validated
- [ ] ✅ Manual testing against Vertex AI successful
- [ ] ✅ Request structure matches Antigravity baseline

### Integration
- [ ] ✅ Changes committed to feature branch
- [ ] ✅ Code review completed
- [ ] ✅ Merged to main branch
- [ ] ✅ Epic-003 updated with completion status

---

## Dependencies

### Upstream Dependencies
- ✅ **Story-003-01**: Model ID constant (completed)
- ✅ **Story-003-02**: API/Model Provider constants (completed)
- ✅ **Story-003-03**: Antigravity metadata (completed)
- ✅ **Story-003-09**: Flexible Tool Configuration Modes (completed)

**Why**: These stories establish the request structure foundation.

### Downstream Dependencies
- **Story-003-11**: Tool Mode Testing (will validate geminiSettings presence)
- **Story-003-12**: Compliance Monitoring Dashboard (will track compliance improvement)

**Impact**: Story-003-11 will test end-to-end request structure including geminiSettings.

---

## Risk Analysis

### Risk #1: Backend Rejection of Recitation Policy
**Severity**: LOW
**Probability**: LOW
**Impact**: Request rejected by Vertex AI if policy invalid

**Mitigation**:
- ✅ Use exact values from RE documentation (action=BLOCK, threshold=LOW)
- ✅ Manual testing with live Vertex AI endpoint
- ✅ Monitor backend responses for policy-related errors
- ✅ Gap Analysis confirms these values are standard

**Contingency**: If backend rejects, investigate upstream error and adjust values based on actual error message.

### Risk #2: Performance Impact
**Severity**: NEGLIGIBLE
**Probability**: VERY LOW
**Impact**: Minimal JSON serialization overhead

**Mitigation**:
- ✅ Simple hardcoded JSON object (no computation)
- ✅ No conditional logic or validation needed
- ✅ <1ms additional latency

**Measurement**: Performance benchmarking in Story-003-11.

### Risk #3: Breaking Existing Functionality
**Severity**: LOW
**Probability**: VERY LOW
**Impact**: Existing requests fail or behave differently

**Mitigation**:
- ✅ Add field without modifying existing logic
- ✅ Comprehensive unit tests for backward compatibility
- ✅ Integration testing with all request types
- ✅ No conditional logic - always add the same field

**Contingency**: If issues arise, add feature flag to disable geminiSettings temporarily.

---

## Implementation Notes

### Design Decisions

**1. Why Hardcoded Values (Not Configurable)?**
- ✅ **Anti-Detection**: Antigravity always uses BLOCK+LOW
- ✅ **Simplicity**: No need for configuration complexity
- ✅ **Compliance**: Exact match with RE specification
- ✅ **Safety**: Strictest anti-plagiarism policy by default

**2. Why Always Present (Not Conditional)?**
- ✅ **Protocol Requirement**: Gemini Protocol v1internal expects geminiSettings
- ✅ **Antigravity Baseline**: RE analysis shows 100% presence
- ✅ **Anti-Detection**: Missing field is a clear fingerprint
- ✅ **Consistency**: Same behavior for all requests

**3. Why After toolConfig?**
- ✅ **Logical Grouping**: Gemini-specific settings together
- ✅ **Field Ordering**: Matches observed Antigravity structure
- ✅ **Code Organization**: Standard fields, then protocol-specific

**4. Why Debug Level Logging?**
- ✅ **Not User-Facing**: Implementation detail
- ✅ **Troubleshooting**: Helps verify correct addition
- ✅ **Performance**: Can be disabled in production

### Performance Considerations

**Impact**: NEGLIGIBLE
- ✅ Simple JSON object creation: O(1)
- ✅ No network calls or database queries
- ✅ No conditional logic or validation
- ✅ Minimal memory footprint (~50 bytes)

**Measurement**: <0.1ms additional latency in request mapping.

### Anti-Detection Significance

**Why This Field is CRITICAL**:
1. **Always Present**: RE analysis confirms 100% presence in Antigravity
2. **Exact Values**: BLOCK+LOW matches Antigravity configuration
3. **Fingerprinting**: Missing field enables backend detection
4. **Protocol Compliance**: Required by Gemini Protocol v1internal
5. **Analytics Signal**: Backend tracks recitationPolicy presence and values

**With vs Without geminiSettings**:
```
WITHOUT geminiSettings:
  Backend sees: Missing expected field
  Risk: Flagged as non-Antigravity client
  Impact: Potential rate limiting or blocking

WITH geminiSettings (BLOCK+LOW):
  Backend sees: Standard Antigravity configuration
  Risk: None - indistinguishable from genuine client
  Impact: Normal request processing
```

---

## File Impact Analysis

### Modified Files

| File | Lines Changed | Change Type | Description |
|------|---------------|-------------|-------------|
| `request.rs` | +10 | Addition | Add geminiSettings after toolConfig |
| `request_tests.rs` | +150 | Addition | 5 unit tests |
| `claude_tests.rs` | +40 | Addition | 1 integration test |

**Total Changes**:
- **Production Code**: ~10 lines
- **Test Code**: ~190 lines
- **Test/Code Ratio**: 19:1 (very high test coverage for simple change)

### New Files
None (all changes to existing files)

---

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2026-01-10 | Story created with comprehensive analysis | BMad Master |

---

## References

### Gap Analysis
- **Document**: `docs/comparison/claude/claude-4-5-sonnet/current-implementation-thinking.md`
- **Section**: Lines 3472-3539 (Gap #7: Grounding Configuration)
- **Impact Table**: Lines 3508-3517
- **Required Changes**: Lines 3519-3538

### Current Implementation
- **Request Mapper**: `src-tauri/src/proxy/mappers/claude/request.rs:434-446`
- **Missing**: geminiSettings field

### Gemini Protocol
- **Field**: `geminiSettings.recitationPolicy`
- **Values**: action ("BLOCK"), threshold ("LOW")
- **Purpose**: Anti-plagiarism protection, content attribution

### Antigravity Baseline
- **Configuration**: geminiSettings always present
- **Policy**: action=BLOCK, threshold=LOW (100% of requests)
- **Anti-Detection**: Exact match required for compliance
