# Story-005-01: Gemini 3.x Model ID Discovery & Implementation

**Story ID**: Story-005-01
**Epic**: [Epic-005](../epics/Epic-005-Gemini-3-Pro-High-Compliance.md) - Gemini 3 Pro High - Full Compliance & Documentation
**Priority**: P0 (Critical) - Architectural Parity
**Estimated Effort**: 3 hours
**Type**: CODE + Research
**Status**: To Do
**Created**: 2026-01-11
**Owner**: Engineering Team

**Sequential Position**: Story #1 of 8 (FIRST - Foundation)
**Blocks**: Monitoring improvements, Model ID visibility in dashboard

---

## User Story

**As a** system architect maintaining Antigravity compliance
**I want** Model ID constants for all Gemini 3.x models matching Claude pattern
**So that** we achieve architectural parity, enable granular monitoring, and maintain consistency with existing Claude model implementation

---

## Context

### Current Situation

**Architectural Inconsistency**:

**Claude Models** (✅ IMPLEMENTED):
```rust
// src-tauri/src/proxy/mappers/claude/request.rs:11-14
const CLAUDE_4_5_SONNET_THINKING_MODEL_ID: u32 = 334;
const CLAUDE_4_5_SONNET_MODEL_ID: u32 = 333;

// request.rs:177-184
fn get_model_id(model_name: &str) -> u32 {
    match model_name {
        "claude-4.5-sonnet-thinking" => 334,  // ✅ Returns explicit ID
        "claude-4.5-sonnet" => 333,           // ✅ Returns explicit ID
        _ => 0, // Unknown model
    }
}

// request.rs:578
"modelId": get_model_id(&config.final_model),  // ✅ Claude: 333 or 334
```

**Gemini Models** (❌ NOT IMPLEMENTED):
```rust
// request.rs:177-184
fn get_model_id(model_name: &str) -> u32 {
    match model_name {
        // ❌ NO gemini-3-pro-high mapping
        // ❌ NO gemini-3-pro-low mapping
        // ❌ NO gemini-3-flash mapping
        _ => 0, // Always returns 0 for Gemini
    }
}

// request.rs:578
"modelId": get_model_id("gemini-3-pro-high"),  // ❌ Returns 0 (unknown)
```

**Impact**:
- ❌ **Monitoring**: Cannot distinguish Gemini models by ID (only by name)
- ❌ **Quota Tracking**: Less granular than Claude models
- ❌ **Dashboard**: Shows "Model ID: 0" for all Gemini models
- ❌ **Consistency**: Architectural inconsistency with Claude pattern

### Expected Behavior

**Reference**: Epic-003, Epic-004 pattern (Model ID constants for compliance)

**Gemini 3.x Models** (SHOULD HAVE):
```rust
// Model ID constants (to be discovered)
const GEMINI_3_PRO_HIGH_MODEL_ID: u32 = ???;  // Network capture needed
const GEMINI_3_PRO_LOW_MODEL_ID: u32 = ???;   // Network capture needed
const GEMINI_3_FLASH_MODEL_ID: u32 = ???;     // Network capture needed

// Updated get_model_id() function
fn get_model_id(model_name: &str) -> u32 {
    match model_name {
        "claude-4.5-sonnet-thinking" => 334,
        "claude-4.5-sonnet" => 333,
        // 🆕 Add Gemini 3.x mappings
        "gemini-3-pro-high" => GEMINI_3_PRO_HIGH_MODEL_ID,
        "gemini-3-pro-low" => GEMINI_3_PRO_LOW_MODEL_ID,
        "gemini-3-flash" => GEMINI_3_FLASH_MODEL_ID,
        _ => 0,
    }
}
```

**Result**:
- ✅ **Monitoring**: Model ID visible in dashboard
- ✅ **Quota Tracking**: Granular per-model tracking
- ✅ **Consistency**: Matches Claude architectural pattern
- ✅ **Compliance**: Architectural parity achieved

### Gap Analysis

**Source**: `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-COMPARISON.md:231-310`

```yaml
gap: "No Model ID constant for gemini-3-pro-high"
impact: "MEDIUM - Affects monitoring granularity"
current_behavior:
  - get_model_id("gemini-3-pro-high") returns 0 (unknown)
  - Quota tracking still works (uses model name, not ID)
  - Monitoring relies on model name strings

why_missing:
  possible_reason_1: "Gemini models use different ID system vs Claude"
  possible_reason_2: "Model ID auto-determined by upstream API"
  possible_reason_3: "Not yet implemented (future enhancement)"

recommendation: "INVESTIGATE AND IMPLEMENT"
priority: "MEDIUM → P0 (upgraded for architectural parity)"
effort: "LOW - Once Model ID discovered via network capture"
```

**Priority Justification**:
- COMPARISON says MEDIUM, but upgrading to **P0** for architectural consistency
- All Claude models have explicit IDs (333, 334)
- Gemini 3.x is PRIMARY production model (visible in UI)
- Monitoring dashboard should show consistent Model IDs

---

## Reference Documentation

### Primary Sources

1. **Gap Analysis**: `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-COMPARISON.md`
   - Lines 231-310: Model ID System gap analysis
   - Lines 635-647: Critical Gap Summary
   - Lines 775-789: Action Plan

2. **Epic-003 Pattern**: `docs/stories/Story-003-01-model-id-constant.md`
   - Claude 4.5 Sonnet Thinking Model ID: 334
   - Implementation pattern for constants

3. **Epic-004 Pattern**: `docs/stories/Story-004-01-model-provider-constants.md`
   - Claude 4.5 Sonnet Standard Model ID: 333
   - Shared implementation pattern

### Code References

**Current Implementation**:
- `src-tauri/src/proxy/mappers/claude/request.rs:11-24` - Model ID constants (Claude only)
- `src-tauri/src/proxy/mappers/claude/request.rs:177-184` - get_model_id() function
- `src-tauri/src/proxy/mappers/claude/request.rs:578` - Model ID usage in request body
- `src-tauri/src/proxy/mappers/claude/request.rs:2054-2067` - Model ID unit tests

**Related Files**:
- `src-tauri/src/proxy/common/model_mapping.rs:52-75` - Model name routing
- `src-tauri/src/proxy/monitor.rs` - Monitoring and stats collection

### Workflow Documents

- `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-workflow.md`
- `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-thinking-workflow.md`
- `docs/comparison/MASTER-MODELS-TABLE.md` - Model inventory

---

## Technical Details

### Implementation Architecture

**Three-Step Implementation**:

1. **Discovery Phase**: Network capture to find actual Model IDs
2. **Constants Phase**: Add constants matching Claude pattern
3. **Integration Phase**: Update get_model_id() mapping and tests

### Step 1: Model ID Discovery (Research - 1 hour)

**Objective**: Discover actual Model IDs used by Antigravity v1.13.3 for Gemini 3.x models

**Method 1: Network Capture (Recommended)**

```bash
# Using mitmproxy to capture Antigravity HTTPS traffic
mitmproxy --mode regular --listen-port 8080

# Configure Antigravity to use proxy
export HTTPS_PROXY=http://localhost:8080

# Make requests with each model and capture payload
# Look for "modelId" field in v1internal request body
```

**Expected Payload Structure**:
```json
{
  "request": {...},
  "model": "gemini-3-pro-high",
  "modelId": ???,  // <-- This value to discover
  "apiProvider": 0,
  "modelProvider": 1,
  "userAgent": "antigravity"
}
```

**Method 2: Reverse Engineering Docs**

Check if Model IDs already documented in:
- `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-workflow.md`
- Antigravity extension code analysis
- Protocol Buffer message definitions

**Method 3: Code Pattern Analysis**

Analyze model ID numbering pattern:
```yaml
claude_models:
  claude-4.5-sonnet: 333
  claude-4.5-sonnet-thinking: 334

gemini_models_hypothesis:
  gemini-3-pro-high: 3XX?  # Pattern suggests 300+ range
  gemini-3-pro-low: 3XX?
  gemini-3-flash: 3XX?
```

**Discovery Deliverables**:
- ✅ Model ID for `gemini-3-pro-high` (base variant)
- ✅ Model ID for `gemini-3-pro-low` (base variant)
- ✅ Model ID for `gemini-3-flash` (base variant)
- ✅ Evidence: network capture screenshots or documentation references
- ✅ Validation: Multiple requests confirm same Model ID

**Fallback**: If Model IDs cannot be discovered, document as "0 (name-based routing)" with rationale.

---

### Step 2: Add Model ID Constants (Implementation - 1 hour)

**File**: `src-tauri/src/proxy/mappers/claude/request.rs`

**Location**: After line 14 (after Claude constants)

**Code to Add**:

```rust
// Model ID constants from Google Antigravity v1.13.3
// Reference: docs/antigravity/workflows/models/claude/claude-4.5-sonnet-thinking-workflow.md:161-166
const CLAUDE_4_5_SONNET_THINKING_MODEL_ID: u32 = 334;
const CLAUDE_4_5_SONNET_MODEL_ID: u32 = 333;

// 🆕 Gemini 3.x Model ID constants
// Reference: docs/stories/Story-005-01-gemini-model-id-constants.md
// Discovered via: [Network capture / Documentation / Method used]
const GEMINI_3_PRO_HIGH_MODEL_ID: u32 = [DISCOVERED_VALUE];
const GEMINI_3_PRO_LOW_MODEL_ID: u32 = [DISCOVERED_VALUE];
const GEMINI_3_FLASH_MODEL_ID: u32 = [DISCOVERED_VALUE];

// API Provider constants
// (existing code continues...)
```

**Update get_model_id() Function**:

```rust
// src-tauri/src/proxy/mappers/claude/request.rs:177-184
fn get_model_id(model_name: &str) -> u32 {
    match model_name {
        // Claude models
        "claude-4.5-sonnet-thinking" => CLAUDE_4_5_SONNET_THINKING_MODEL_ID,
        "claude-4.5-sonnet" => CLAUDE_4_5_SONNET_MODEL_ID,

        // 🆕 Gemini 3.x models (Story-005-01)
        "gemini-3-pro-high" => GEMINI_3_PRO_HIGH_MODEL_ID,
        "gemini-3-pro-low" => GEMINI_3_PRO_LOW_MODEL_ID,
        "gemini-3-flash" => GEMINI_3_FLASH_MODEL_ID,

        // Unknown model
        _ => 0,
    }
}
```

**Lines of Code**:
- Constants: ~6 lines (3 constants + comments)
- Function update: ~3 lines (3 new match arms)
- **Total**: ~9 lines

---

### Step 3: Add Integration Tests (Testing - 1 hour)

**File**: `src-tauri/src/proxy/mappers/claude/request.rs`

**Location**: After line 2067 (after existing Model ID tests)

**Tests to Add**:

```rust
// ==================== Story-005-01: Gemini 3.x Model ID Tests ====================

/// AC-1: Test get_model_id() for gemini-3-pro-high
#[test]
fn test_get_model_id_gemini_3_pro_high() {
    let model_id = get_model_id("gemini-3-pro-high");
    assert_ne!(model_id, 0, "Gemini 3 Pro High should return non-zero Model ID");
    assert_eq!(
        model_id,
        GEMINI_3_PRO_HIGH_MODEL_ID,
        "Model ID should match constant"
    );
}

/// AC-2: Test get_model_id() for gemini-3-pro-low
#[test]
fn test_get_model_id_gemini_3_pro_low() {
    let model_id = get_model_id("gemini-3-pro-low");
    assert_ne!(model_id, 0, "Gemini 3 Pro Low should return non-zero Model ID");
    assert_eq!(
        model_id,
        GEMINI_3_PRO_LOW_MODEL_ID,
        "Model ID should match constant"
    );
}

/// AC-3: Test get_model_id() for gemini-3-flash
#[test]
fn test_get_model_id_gemini_3_flash() {
    let model_id = get_model_id("gemini-3-flash");
    assert_ne!(model_id, 0, "Gemini 3 Flash should return non-zero Model ID");
    assert_eq!(
        model_id,
        GEMINI_3_FLASH_MODEL_ID,
        "Model ID should match constant"
    );
}

/// AC-4: Integration test - Model ID included in request body
#[tokio::test]
async fn test_gemini_request_includes_model_id() {
    let req = ClaudeRequest {
        model: "gemini-3-pro-high".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        max_tokens: Some(8192),
        temperature: None,
        top_p: None,
        top_k: None,
        stop_sequences: None,
        stream: Some(false),
        tools: None,
        tool_choice: None,
        thinking: None,
        metadata: None,
        output_config: None,
    };

    let result = transform_claude_request_in(&req, "test-project").await;
    assert!(result.is_ok(), "Request transformation should succeed");

    let (body, _violations) = result.unwrap();

    // Verify modelId field exists and is non-zero
    let model_id = body["modelId"].as_u64().expect("modelId should be present");
    assert_ne!(
        model_id, 0,
        "Model ID should be non-zero for gemini-3-pro-high"
    );
    assert_eq!(
        model_id,
        GEMINI_3_PRO_HIGH_MODEL_ID as u64,
        "Model ID should match constant"
    );
}

/// AC-5: Test all three Gemini 3.x models return distinct IDs
#[test]
fn test_gemini_3x_distinct_model_ids() {
    let high_id = get_model_id("gemini-3-pro-high");
    let low_id = get_model_id("gemini-3-pro-low");
    let flash_id = get_model_id("gemini-3-flash");

    // All should be non-zero
    assert_ne!(high_id, 0, "High should have Model ID");
    assert_ne!(low_id, 0, "Low should have Model ID");
    assert_ne!(flash_id, 0, "Flash should have Model ID");

    // All should be distinct
    assert_ne!(high_id, low_id, "High and Low should have different IDs");
    assert_ne!(high_id, flash_id, "High and Flash should have different IDs");
    assert_ne!(low_id, flash_id, "Low and Flash should have different IDs");
}
```

**Lines of Code**:
- Unit tests: ~60 lines (3 tests × ~12 lines)
- Integration test: ~45 lines
- Validation test: ~18 lines
- **Total**: ~123 lines of test code

---

## Acceptance Criteria

### AC-1: Model ID Discovery Evidence

**Requirement**: Discover Model IDs for all 3 Gemini 3.x models with verifiable evidence

**Verification**:
```yaml
evidence_required:
  gemini_3_pro_high:
    model_id: "Numeric value discovered"
    source: "Network capture OR documentation reference"
    validation: "Minimum 3 requests show same ID"

  gemini_3_pro_low:
    model_id: "Numeric value discovered"
    source: "Same as above"
    validation: "Minimum 3 requests show same ID"

  gemini_3_flash:
    model_id: "Numeric value discovered"
    source: "Same as above"
    validation: "Minimum 3 requests show same ID"

documentation:
  - Screenshots of network capture showing modelId field
  - OR documentation references with page/line numbers
  - OR code analysis showing Model ID derivation
```

**Pass Criteria**:
- ✅ All 3 Model IDs discovered
- ✅ Evidence documented (screenshots OR docs)
- ✅ Model IDs validated (consistent across multiple requests)
- ✅ Fallback plan documented if IDs not discoverable

---

### AC-2: Constants Added Matching Claude Pattern

**Requirement**: Add 3 Model ID constants following exact Claude pattern

**File**: `src-tauri/src/proxy/mappers/claude/request.rs`
**Location**: After line 14

**Exact Format**:
```rust
// 🆕 Gemini 3.x Model ID constants
// Reference: docs/stories/Story-005-01-gemini-model-id-constants.md
// Discovered via: [Network capture date YYYY-MM-DD]
const GEMINI_3_PRO_HIGH_MODEL_ID: u32 = [DISCOVERED_VALUE];
const GEMINI_3_PRO_LOW_MODEL_ID: u32 = [DISCOVERED_VALUE];
const GEMINI_3_FLASH_MODEL_ID: u32 = [DISCOVERED_VALUE];
```

**Pass Criteria**:
- ✅ Constants added at correct location (after line 14)
- ✅ Naming convention matches Claude pattern (SCREAMING_SNAKE_CASE)
- ✅ Type is `u32` (matching Claude constants)
- ✅ Comments include discovery method and date
- ✅ Reference to this story document in comments

---

### AC-3: get_model_id() Returns Non-Zero for Gemini 3.x

**Requirement**: Update get_model_id() function to return discovered IDs

**File**: `src-tauri/src/proxy/mappers/claude/request.rs`
**Location**: Lines 177-184 (function body)

**Updated Function**:
```rust
fn get_model_id(model_name: &str) -> u32 {
    match model_name {
        // Claude models
        "claude-4.5-sonnet-thinking" => CLAUDE_4_5_SONNET_THINKING_MODEL_ID,
        "claude-4.5-sonnet" => CLAUDE_4_5_SONNET_MODEL_ID,

        // 🆕 Gemini 3.x models (Story-005-01)
        "gemini-3-pro-high" => GEMINI_3_PRO_HIGH_MODEL_ID,
        "gemini-3-pro-low" => GEMINI_3_PRO_LOW_MODEL_ID,
        "gemini-3-flash" => GEMINI_3_FLASH_MODEL_ID,

        // Unknown model
        _ => 0,
    }
}
```

**Pass Criteria**:
- ✅ Function updated with 3 new match arms
- ✅ All Gemini 3.x models return non-zero
- ✅ Unknown models still return 0
- ✅ Comment references Story-005-01
- ✅ Formatting matches existing code style

---

### AC-4: Monitoring Dashboard Shows Model ID

**Requirement**: Model ID visible in monitoring UI for Gemini 3.x models

**Current Behavior**:
```typescript
// src/components/proxy/ProxyMonitor.tsx
// Shows: "Model ID: 0" for all Gemini models
```

**Expected Behavior**:
```typescript
// After Story-005-01:
// gemini-3-pro-high → "Model ID: [DISCOVERED_VALUE]"
// gemini-3-pro-low → "Model ID: [DISCOVERED_VALUE]"
// gemini-3-flash → "Model ID: [DISCOVERED_VALUE]"
```

**Verification Steps**:
1. Start proxy server
2. Make request with gemini-3-pro-high
3. Open Monitor page
4. Verify Model ID shows non-zero value
5. Repeat for gemini-3-pro-low and gemini-3-flash

**Pass Criteria**:
- ✅ Dashboard displays non-zero Model ID for gemini-3-pro-high
- ✅ Dashboard displays non-zero Model ID for gemini-3-pro-low
- ✅ Dashboard displays non-zero Model ID for gemini-3-flash
- ✅ Model IDs match constants defined in AC-2
- ✅ No regression: Claude models still show 333/334

---

### AC-5: 100% Test Coverage for Model ID Mapping

**Requirement**: Comprehensive test coverage for all new Model ID mappings

**Files**: `src-tauri/src/proxy/mappers/claude/request.rs`

**Tests Required** (5 tests minimum):

1. **Unit Test**: get_model_id("gemini-3-pro-high") returns correct ID
2. **Unit Test**: get_model_id("gemini-3-pro-low") returns correct ID
3. **Unit Test**: get_model_id("gemini-3-flash") returns correct ID
4. **Integration Test**: Request body includes correct Model ID
5. **Validation Test**: All 3 models have distinct IDs

**Pass Criteria**:
- ✅ All 5 tests written and passing
- ✅ Tests location: After line 2067 (existing Model ID tests)
- ✅ Test names follow pattern: `test_get_model_id_gemini_3_*`
- ✅ `cargo test` passes with 100% success
- ✅ Coverage report shows ≥90% for get_model_id() function

**Test Execution**:
```bash
# Run all Model ID tests
cargo test test_get_model_id

# Expected output:
# test test_get_model_id_sonnet_thinking ... ok
# test test_get_model_id_sonnet ... ok
# test test_get_model_id_unknown ... ok
# test test_get_model_id_gemini_3_pro_high ... ok  # 🆕
# test test_get_model_id_gemini_3_pro_low ... ok   # 🆕
# test test_get_model_id_gemini_3_flash ... ok     # 🆕
# test test_gemini_request_includes_model_id ... ok # 🆕
# test test_gemini_3x_distinct_model_ids ... ok     # 🆕
```

---

## Implementation Steps

### Phase 1: Discovery (1 hour)

**Step 1.1**: Set up network capture environment
- Install mitmproxy or use Chrome DevTools
- Configure HTTPS interception
- Prepare capture filter for v1internal requests

**Step 1.2**: Capture Antigravity requests
- Launch actual Antigravity v1.13.3
- Make requests with gemini-3-pro-high
- Make requests with gemini-3-pro-low
- Make requests with gemini-3-flash

**Step 1.3**: Extract Model IDs
- Parse captured JSON payloads
- Extract "modelId" field values
- Validate consistency across multiple captures
- Document evidence (screenshots, payloads)

**Step 1.4**: Validate discoveries
- Confirm Model IDs are numeric (u32)
- Confirm Model IDs are distinct
- Confirm Model IDs consistent across requests
- Document fallback plan if discovery fails

---

### Phase 2: Implementation (1 hour)

**Step 2.1**: Add constants to request.rs
- Open `src-tauri/src/proxy/mappers/claude/request.rs`
- Navigate to line 14
- Add 3 Gemini constants with comments
- Reference discovery evidence

**Step 2.2**: Update get_model_id() function
- Navigate to lines 177-184
- Add 3 new match arms for Gemini models
- Add Story-005-01 comment
- Verify formatting matches existing style

**Step 2.3**: Code review
- Run `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Verify no compilation errors
- Review for consistency with Claude pattern

---

### Phase 3: Testing (1 hour)

**Step 3.1**: Write unit tests
- Add test_get_model_id_gemini_3_pro_high
- Add test_get_model_id_gemini_3_pro_low
- Add test_get_model_id_gemini_3_flash
- Add test_gemini_3x_distinct_model_ids

**Step 3.2**: Write integration test
- Add test_gemini_request_includes_model_id
- Verify modelId field in transformed request
- Test all 3 Gemini models

**Step 3.3**: Execute tests
```bash
cargo test test_get_model_id --lib
cargo test test_gemini_request_includes_model_id
```

**Step 3.4**: Verify monitoring
- Start proxy server: `cargo run`
- Make test request with gemini-3-pro-high
- Check Monitor page for Model ID display
- Verify non-zero ID shown

---

## Definition of Done

Story-005-01 is **COMPLETE** when:

### Code Requirements
- ✅ 3 Model ID constants added to request.rs (lines 15-20)
- ✅ get_model_id() function updated with 3 new mappings
- ✅ Code formatted with `cargo fmt`
- ✅ Linting passed with `cargo clippy`
- ✅ No compilation errors

### Testing Requirements
- ✅ 5 unit/integration tests added
- ✅ All tests passing (`cargo test`)
- ✅ Test coverage ≥90% for get_model_id()
- ✅ Manual verification: Model ID visible in dashboard

### Documentation Requirements
- ✅ Discovery evidence documented (screenshots OR references)
- ✅ Constants include discovery method in comments
- ✅ Code comments reference this story document
- ✅ Story status updated to "✅ IMPLEMENTED"

### Quality Gates
- ✅ Code review passed
- ✅ No regression in existing tests
- ✅ Monitoring dashboard verified manually
- ✅ Discovery method documented for future models

---

## Dependencies

### Upstream Dependencies (Must Exist)
- ✅ Epic-003: Pattern for Model ID constants (334)
- ✅ Epic-004: Pattern for Model ID constants (333)
- ✅ COMPARISON document: Gap analysis for Model ID

### Downstream Dependencies (Will Benefit)
- ⏳ Story-005-02: Profile Presets (may reference Model ID)
- ⏳ Future monitoring enhancements (will use Model ID for filtering)
- ⏳ Epic-006: Gemini 3 Pro Low (will reuse discovery method)
- ⏳ Epic-007: Gemini 3 Flash (will reuse discovery method)

### Sequential Constraint
- **MUST BE FIRST**: Foundation для architectural parity
- **BLOCKS**: Story-005-02, Story-005-03 (optional dependency)
- **Next Story**: Story-005-02 (Profile Presets UI)

---

## Risks & Mitigations

### Risk 1: Model ID Cannot Be Discovered 🔴

**Risk**: Network capture may not reveal Model ID or IDs may not exist

**Probability**: MEDIUM (30%)

**Impact**: HIGH - Cannot achieve architectural parity

**Mitigation Strategy**:
```yaml
option_1_retry_capture:
  action: "Try multiple capture methods"
  methods:
    - mitmproxy with SSL interception
    - Chrome DevTools Network tab
    - Wireshark with Protocol Buffer dissector

option_2_reverse_engineering:
  action: "Search Antigravity extension code"
  locations:
    - Extension JavaScript files (de-minified)
    - Protocol Buffer message definitions
    - Model configuration constants

option_3_documentation_search:
  action: "Search workflow documents"
  files:
    - gemini-3-pro-high-workflow.md
    - Antigravity API documentation
    - MASTER-MODELS-TABLE.md

option_4_accept_limitation:
  action: "Document as 'Model ID: 0 (name-based routing)'"
  rationale: "Gemini may use name-based routing, not ID-based"
  impact: "Accept architectural difference, update documentation"

fallback_decision: "Use Option 4 if Options 1-3 fail after 2 hours of research"
```

---

### Risk 2: Model IDs Are Not Distinct 🟡

**Risk**: All Gemini models share same Model ID

**Probability**: LOW (10%)

**Impact**: MEDIUM - Cannot distinguish models by ID

**Mitigation**:
- Document shared Model ID pattern
- Continue using model name for distinction
- Update monitoring to rely on name, not ID
- Accept architectural difference from Claude

---

### Risk 3: Constants Break Existing Tests 🟢

**Risk**: Adding constants causes compilation or test failures

**Probability**: LOW (5%)

**Impact**: LOW - Easy to fix

**Mitigation**:
- Run full test suite before and after changes
- Review all usages of get_model_id()
- Ensure unknown models still return 0
- Verify no breaking changes to existing functionality

---

## Testing Strategy

### Test Pyramid

```yaml
unit_tests:
  count: 4
  focus: "get_model_id() function logic"
  coverage: "All 3 Gemini models + distinct IDs validation"

integration_tests:
  count: 1
  focus: "modelId field in request body"
  coverage: "End-to-end transformation including Model ID"

manual_tests:
  count: 1
  focus: "Dashboard Model ID display"
  coverage: "UI verification for all 3 models"
```

### Test Execution Plan

**Pre-Implementation**:
```bash
# Baseline - existing tests should pass
cargo test --lib
# Expected: All passing, no Gemini Model ID tests
```

**Post-Implementation**:
```bash
# Run new Model ID tests
cargo test test_get_model_id_gemini
# Expected: 4 new tests passing

# Run integration test
cargo test test_gemini_request_includes_model_id
# Expected: 1 test passing

# Run full suite
cargo test --lib
# Expected: All tests passing (no regressions)
```

**Manual Verification**:
```bash
# Start proxy
cargo run

# In separate terminal, make test request
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemini-3-pro-high",
    "messages": [{"role": "user", "content": "Hello"}],
    "max_tokens": 1024
  }'

# Check Monitor page
# Verify: "Model ID: [NON_ZERO_VALUE]"
```

---

## Success Metrics

### Compliance Metrics

```yaml
before_story:
  architectural_parity: "83% (Claude has IDs, Gemini returns 0)"
  monitoring_granularity: "Name-based only"
  dashboard_consistency: "Inconsistent (0 for Gemini, 333/334 for Claude)"

after_story:
  architectural_parity: "100% (All models have explicit IDs)"
  monitoring_granularity: "ID-based and name-based"
  dashboard_consistency: "Consistent (all models show IDs)"

improvement:
  architectural: "+17%"
  monitoring: "ID-based filtering enabled"
  consistency: "100% pattern match with Claude"
```

### Code Quality Metrics

```yaml
lines_of_code:
  constants: ~9 lines
  function_update: ~3 lines
  tests: ~123 lines
  total: ~135 lines

test_coverage:
  before: "N/A (no Gemini Model ID tests)"
  after: "≥90% (5 comprehensive tests)"

code_quality:
  cargo_fmt: "Must pass"
  cargo_clippy: "0 warnings"
  compilation: "0 errors"
```

---

## Documentation Updates

### Files to Update

**1. This Story Document**: `docs/stories/Story-005-01-gemini-model-id-constants.md`
- Add discovery evidence section
- Document actual Model IDs discovered
- Update status to ✅ IMPLEMENTED

**2. Workflow Documents**: (Optional - can be Story-005-04)
- `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-workflow.md`
- Update Model ID: TBD → [DISCOVERED_VALUE]
- Add Model ID to Request Example section

**3. COMPARISON Document**: `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-COMPARISON.md`
- Update Gap Analysis section (lines 231-310)
- Change status: ❌ NOT IMPLEMENTED → ✅ IMPLEMENTED
- Add discovered Model IDs to analysis

**4. Code Comments**: `src-tauri/src/proxy/mappers/claude/request.rs`
- Add comments referencing Story-005-01
- Document discovery method and date
- Reference workflow documentation

---

## Cross-References

### Related Stories

**Epic-003**:
- Story-003-01: Model ID Constant (pattern for 334)
- Story-003-02: API/Model Provider Constants (pattern for providers)

**Epic-004**:
- Story-004-01: Model Provider Constants (pattern for 333)

**Epic-005** (This Epic):
- Story-005-02: Profile Presets (may reference Model ID)
- Story-005-08: Configuration Profiles Docs (references Model ID in monitoring)

### Related Epics (Future)

- Epic-006: Gemini 3 Pro Low (will reuse discovery method)
- Epic-007: Gemini 3 Flash (will reuse discovery method)
- Epic-00X: Gemini 2.5 models (may need different ID discovery approach)

---

## Implementation Notes

### Discovery Method Options

**Option 1: Network Capture** (Recommended)
```bash
# mitmproxy setup
mitmproxy --mode regular --listen-port 8080

# Configure Antigravity
export HTTPS_PROXY=http://localhost:8080

# Launch Antigravity and make requests
# Look for v1internal request payloads
# Extract modelId field
```

**Option 2: Extension Code Analysis**
```bash
# Search de-minified code
grep -r "modelId.*gemini-3" docs/antigravity/deminified/

# Look for model ID assignment logic
# May find constants or ID derivation algorithm
```

**Option 3: Documentation Search**
```bash
# Search workflow documents
grep -r "Model ID" docs/antigravity/workflows/models/gemini/

# Check if IDs already documented
# May be in metadata or configuration sections
```

### Code Style Guidelines

**Follow Existing Patterns**:
```rust
// Constant naming: SCREAMING_SNAKE_CASE with _MODEL_ID suffix
const GEMINI_3_PRO_HIGH_MODEL_ID: u32 = ???;

// Comment style: Multi-line with reference
// 🆕 Gemini 3.x Model ID constants
// Reference: docs/stories/Story-005-01-gemini-model-id-constants.md

// Match arm style: Indented, aligned
"gemini-3-pro-high" => GEMINI_3_PRO_HIGH_MODEL_ID,
```

### Common Pitfalls

**Avoid**:
- ❌ Hardcoding Model IDs without discovery evidence
- ❌ Using different naming convention than Claude constants
- ❌ Forgetting to update tests
- ❌ Breaking existing get_model_id() behavior for unknown models

**Ensure**:
- ✅ Discovery evidence documented before implementation
- ✅ Constants match Claude naming pattern exactly
- ✅ All tests updated and passing
- ✅ Unknown models still return 0 (fallback behavior)

---

## Questions & Answers

### Q1: What if Model IDs are not discoverable?

**A**: Use fallback strategy (Option 4 in Risk Mitigation):
- Document as "Model ID: 0 (name-based routing)"
- Add comment explaining Gemini uses different architecture
- Update COMPARISON document with rationale
- Accept architectural difference from Claude
- Story still considered COMPLETE (documented limitation)

### Q2: Should we discover Model IDs for thinking variants too?

**A**: NO - For Gemini 3.x, thinking is parameter-based (not model-based):
- `gemini-3-pro-high` + `thinkingConfig` → thinking enabled
- No separate `gemini-3-pro-high-thinking` model ID
- Single Model ID serves both thinking and non-thinking modes

### Q3: What if discovered Model IDs conflict with existing IDs?

**A**: Document conflict and investigate:
- Check if IDs overlap with Claude (333, 334)
- Verify Model IDs are truly distinct
- If conflict, investigate whether multiple models share IDs
- Update constants with conflict resolution strategy

### Q4: Should this story update the monitoring dashboard UI?

**A**: NO - Dashboard auto-updates when Model ID becomes non-zero:
- No UI code changes required
- Dashboard already displays modelId field
- Simply changing backend function enables display
- Manual verification in AC-4 confirms UI works

---

## Story Status

**Current Status**: To Do
**Next Story**: Story-005-02 (Profile Presets UI Implementation)
**Epic Progress**: 0/8 stories complete (0%)

---

**Created**: 2026-01-11
**Last Updated**: 2026-01-11
**Story Points**: 3
**Estimated Hours**: 3 (1h discovery + 1h implementation + 1h testing)
