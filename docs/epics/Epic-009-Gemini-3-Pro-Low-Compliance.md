# Epic-009: Gemini 3 Pro Low - Compliance & Value Positioning

**Epic ID**: Epic-009
**Models**: `gemini-3-pro-low` + `gemini-3-pro-low-thinking`
**Priority**: P1 (High) - Completes Gemini 3 Pro Trilogy
**Compliance Target**: 82.1% → 100%
**Status**: 📋 PLANNED (After Epic-008)
**Created**: 2026-01-11
**Pattern**: Compliance + Quality Enhancement (like Epic-005, Epic-007)

---

## 📊 Executive Summary

### Current Status

```yaml
model_info:
  primary_model: "gemini-3-pro-low"
  thinking_variant: "gemini-3-pro-low-thinking"  # Predicted (parameter-based)
  tier: "Pro (Low Cost)"
  thinking_budget: 32000  # SAME AS HIGH TIER!
  cost_savings: "40-60% vs High tier"
  api_parameter: "thinkingLevel"

compliance_metrics:
  overall_compliance: "82.1%"
  features_analyzed: 28
  fully_implemented: 23
  partially_implemented: 2
  not_implemented: 3

gap_analysis:
  P0_critical: 2 🔴
    - "No Routing Aliases"
    - "Model ID Constant Missing"
  P1_high: 1 ⚠️
    - "Thinking Variant Naming"
  P2_medium: 2 📋
    - "Enhanced Error Recovery Documentation"
    - "Low Tier Specific Test Coverage"
  P3_low: 0

production_readiness:
  status: "✅ PRODUCTION READY (with caveats)"
  blocking_issues: 2  # Aliases + Model ID
  enhancement_opportunities: 3
  deployment_risk: "🟢 Low"
```

### Strategic Significance

**Why Epic-009?**

1. **Completes Gemini 3 Pro Trilogy**: High (96.4%) → Image (Epic-007, 100%) → Low (Epic-009, 100%)
2. **Value Proposition Discovery**: SAME 32000 token thinking budget as High tier!
3. **Cost Optimization**: 40-60% savings while maintaining full reasoning capability
4. **Strategic Milestone**: 100% Gemini 3.x Pro coverage

**Critical Discovery**:
```yaml
thinking_budget_equality:
  gemini_3_pro_low: 32000
  gemini_3_pro_high: 32000
  difference: 0

implication:
  - "Cost difference from BASE model quality, NOT thinking"
  - "Same reasoning depth as High tier"
  - "Low tier is UNDERVALUED in current positioning"

value_proposition:
  - "Equal thinking capability at 40-60% lower cost"
  - "Optimal for reasoning-heavy, non-critical tasks"
  - "Best for: Code analysis, data extraction, problem-solving"
```

**Business Value**:
- **Cost Efficiency**: 40-60% savings with equal thinking depth
- **Market Positioning**: "Cost-optimized reasoning specialist"
- **Use Case Expansion**: Beyond development/testing to production reasoning tasks
- **Competitive Edge**: Equal capability at lower cost vs competitors

**Technical Value**:
- **Consistency**: Same architecture as High tier (proven reliability)
- **Discoverability**: Routing aliases improve user experience
- **Monitoring**: Model ID enables granular quota tracking
- **Quality**: Comprehensive test coverage ensures reliability

---

## 🎯 Gap Analysis & Story Mapping

### Gap #1: No Routing Aliases (P0 Critical)

**Current State**:
```yaml
implementation: "Partial (25%)"
current_behavior:
  - Direct routing: "gemini-3-pro-low" → works ✅
  - No convenience aliases (unlike High tier)
  - Unknown models → High tier (quality over cost)
  - Users must use explicit model name

high_tier_comparison:
  high_tier_aliases:
    - "gemini-3-pro" → "gemini-3-pro-high"
    - "claude-haiku-4-5" → "gemini-3-pro-high"
  low_tier_aliases: []  # NONE!

limitations:
  - Reduced discoverability
  - No convenience shortcuts
  - Explicit opt-in only (by design)
  - May confuse cost-conscious users
```

**Desired State**:
```yaml
implementation: "Complete (100%)"
target_behavior:
  - Convenience aliases for easier access
  - Clear naming patterns
  - Improved discoverability
  - Maintains intentional opt-in

proposed_aliases:
  minimal_set:
    - "gemini-low" → "gemini-3-pro-low"
    - "gemini-3-low" → "gemini-3-pro-low"

  rationale:
    - Short and memorable
    - Consistent with tier naming
    - No ambiguity (explicit "low")
    - No Haiku fallback (preserve quality for unknown)

expected_benefits:
  discoverability: "+30%"
  user_experience: "Improved convenience"
  explicit_intent: "Maintained (no accidental routing)"
```

**Implementation Strategy** → Story-009-01

**Technical Specifications**:
```yaml
code_location: "src-tauri/src/proxy/common/model_mapping.rs:51,62-76"

changes_required:
  addition_1:
    line: "~65"
    code: 'm.insert("gemini-low", "gemini-3-pro-low");'

  addition_2:
    line: "~66"
    code: 'm.insert("gemini-3-low", "gemini-3-pro-low");'

validation:
  - No fallback to Low from unknown models (preserve quality)
  - Explicit aliases only (no auto-routing)
  - Documentation update required
```

---

### Gap #2: Model ID Constant Missing (P0 Critical)

**Current State**:
```yaml
implementation: "Not Implemented (0%)"
current_behavior:
  - No Model ID constant defined
  - get_model_id("gemini-3-pro-low") returns 0
  - Quota tracking uses model name strings
  - Monitoring relies on name-based tracking

claude_models_comparison:
  claude_4_5_sonnet: "const = 333 ✅"
  claude_4_5_sonnet_thinking: "const = 334 ✅"
  gemini_3_pro_low: "const = ??? ❌"
  gemini_3_pro_high: "const = ??? ❌"

limitations:
  - Incomplete quota attribution
  - No Model ID tracking in v1internal
  - Inconsistency with Claude models
  - Reduced monitoring granularity
```

**Desired State**:
```yaml
implementation: "Complete (100%)"
target_behavior:
  - Model ID constant defined
  - get_model_id() returns actual ID
  - Consistent with Claude models
  - Enhanced quota tracking

discovery_required:
  method: "Network capture of v1internal requests"
  target: "Actual Model ID from Gemini API"
  effort: "1-2 hours"

expected_structure:
  const_definition:
    code: "const GEMINI_3_PRO_LOW_MODEL_ID: u32 = ???;"
    location: "src-tauri/src/proxy/mappers/claude/request.rs:11-24"

  mapping_addition:
    match_case: '"gemini-3-pro-low" => GEMINI_3_PRO_LOW_MODEL_ID'
    location: "get_model_id() function"

expected_benefits:
  quota_tracking: "Granular per-model attribution"
  monitoring: "Model ID based analytics"
  consistency: "Same pattern as Claude models"
```

**Implementation Strategy** → Story-009-02

**Technical Specifications**:
```yaml
discovery_steps:
  step_1: "Network capture during gemini-3-pro-low request"
  step_2: "Extract Model ID from v1internal request body"
  step_3: "Validate across multiple accounts"
  step_4: "Add constant and mapping"

code_changes:
  file: "src-tauri/src/proxy/mappers/claude/request.rs"

  addition_1:
    line: "After line 13"
    code: |
      const GEMINI_3_PRO_LOW_MODEL_ID: u32 = ???;  // Discovered via network
      const GEMINI_3_PRO_HIGH_MODEL_ID: u32 = ???;  // High tier (same task)

  addition_2:
    location: "get_model_id() match block"
    code: |
      "gemini-3-pro-low" => GEMINI_3_PRO_LOW_MODEL_ID,
      "gemini-3-pro-high" => GEMINI_3_PRO_HIGH_MODEL_ID,

note: "SAME GAP AS HIGH TIER - Fix together in single story"
```

---

### Gap #3: Thinking Variant Naming (P1 High)

**Current State**:
```yaml
implementation: "Not Implemented (0%)"
current_behavior:
  - Thinking via parameter: thinkingConfig.thinkingLevel
  - No explicit "gemini-3-pro-low-thinking" routing
  - Parameter-based activation (more flexible)

architectural_pattern:
  gemini_models: "Parameter-based (thinkingConfig)"
  claude_models: "Model name suffix (-thinking)"

limitations:
  - Inconsistency with Claude naming
  - No explicit thinking model name
  - May confuse users expecting -thinking suffix
```

**Desired State**:
```yaml
decision_required: true

option_1_accept_current:
  rationale: "Parameter-based activation is superior architecture"
  action: "Document why -thinking suffix not used for Gemini"
  benefits:
    - Cleaner architecture
    - More flexible (enable/disable per request)
    - No model name proliferation
  recommendation: "⭐ RECOMMENDED"

option_2_add_suffix_routing:
  rationale: "Consistency with Claude models"
  action: "Add routing: gemini-3-pro-low-thinking → gemini-3-pro-low"
  benefits:
    - Consistent naming pattern
    - Familiar to Claude users
    - Explicit thinking variant
  risk: "Model name proliferation, less flexible"

recommendation: "ACCEPT Option 1 (parameter-based is better)"
```

**Implementation Strategy** → Story-009-03

**Technical Specifications**:
```yaml
if_option_1_documentation_only:
  file: "docs/antigravity/workflows/models/gemini/gemini-3-pro-low.md"
  section: "Model Naming Architecture"
  content: |
    ## Why No -thinking Suffix?

    Gemini models use **parameter-based thinking activation** instead of
    model name suffixes. This is SUPERIOR to Claude's -thinking suffix:

    - More flexible (enable/disable per request)
    - Cleaner architecture (fewer model names)
    - Budget control via thinkingConfig parameter

    Use: gemini-3-pro-low + thinkingConfig.thinkingLevel = HIGH

if_option_2_implementation:
  code_location: "src-tauri/src/proxy/common/model_mapping.rs"
  addition:
    code: 'm.insert("gemini-3-pro-low-thinking", "gemini-3-pro-low");'
    line: "~67"
  note: "Routing only, actual thinking via parameter"
```

---

### Gap #4: Enhanced Error Recovery Documentation (P2 Medium)

**Current State**:
```yaml
implementation: "Partial (80%)"
current_behavior:
  - Rate limit recovery: ✅ Implemented (429 → retry)
  - Auth recovery: ✅ Implemented (401 → token refresh)
  - Safety filter: ✅ Implemented (env var config)
  - Web search conflict: ✅ Implemented (auto-route to Flash)
  - Signature error: ✅ Implemented (auto-disable thinking)
  - Corrupted signature retry: ⚠️ Partially documented

documented_errors: 6 types
implemented_errors: 5 fully + 1 partial

gap:
  corrupted_signature_retry:
    documented: "Story-004-02-retry-503-signature-errors.md"
    implementation: "Not visible in request.rs (may be in upstream)"
    status: "Partially documented"
```

**Desired State**:
```yaml
implementation: "Complete (100%)"
target_behavior:
  - All error types fully documented
  - Retry logic explicitly shown
  - Code references provided
  - Test coverage validated

corrupted_signature_retry:
  error_pattern: "Corrupted thought signature"
  retry_enabled: true
  max_retries: 3
  backoff: "Exponential (1s, 2s, 4s)"

  required_documentation:
    - Retry configuration
    - Backoff strategy
    - Code location reference
    - Test coverage proof

expected_benefits:
  clarity: "100% error handling transparency"
  confidence: "Validated retry behavior"
  debugging: "Clear troubleshooting path"
```

**Implementation Strategy** → Story-009-04

**Technical Specifications**:
```yaml
investigation_required:
  search_locations:
    - "src-tauri/src/proxy/upstream/"
    - "src-tauri/src/proxy/handlers/"
    - "src-tauri/src/proxy/rate_limit.rs"

  search_patterns:
    - "corrupted"
    - "signature"
    - "retry"
    - "503"

documentation_updates:
  file: "docs/antigravity/workflows/models/gemini/gemini-3-pro-low.md"
  section: "Error Handling → Type 6: Corrupted Signature Retry"
  content:
    - Error detection logic
    - Retry configuration
    - Backoff strategy
    - Code references
    - Test coverage
```

---

### Gap #5: Low Tier Specific Test Coverage (P2 Medium)

**Current State**:
```yaml
implementation: "Partial (70%)"
current_behavior:
  - Shared tests with High tier (budget clamping)
  - No Low tier specific test cases
  - Relies on shared logic validation

test_coverage:
  budget_limits: "✅ Covered (32000 max via High tier test)"
  openai_injection: "✅ Covered (ends_with('-low') check)"
  direct_routing: "⚠️ No explicit Low tier test"
  alias_routing: "❌ No tests (aliases don't exist yet)"
  tier_equality: "❌ No test validating Low = High budget"

limitations:
  - No explicit Low tier validation
  - No tier comparison tests
  - Reduced confidence in Low tier routing
```

**Desired State**:
```yaml
implementation: "Complete (100%)"
target_behavior:
  - Explicit Low tier test cases
  - Budget equality validation
  - Alias routing tests (after Story-009-01)
  - Tier comparison tests

proposed_tests:
  test_1_direct_routing:
    name: "test_gemini_3_pro_low_routing()"
    validates: "gemini-3-pro-low → correct routing"

  test_2_budget_equality:
    name: "test_gemini_3_pro_low_thinking_budget_same_as_high()"
    validates: "Low tier 32000 = High tier 32000"

  test_3_openai_injection:
    name: "test_openai_auto_injection_gemini_3_pro_low()"
    validates: "ends_with('-low') → 16000 default budget"

  test_4_alias_routing:
    name: "test_gemini_low_aliases()"
    validates: "gemini-low, gemini-3-low → gemini-3-pro-low"

expected_benefits:
  confidence: "+30% (explicit validation)"
  regression_prevention: "Low tier specific issues caught"
  documentation: "Tests serve as usage examples"
```

**Implementation Strategy** → Story-009-05

**Technical Specifications**:
```yaml
code_location: "src-tauri/src/proxy/tests/thinking_models.rs"

test_additions:
  test_1:
    location: "After line 403"
    code: |
      #[test]
      fn test_gemini_3_pro_low_routing() {
          let req = create_basic_request("gemini-3-pro-low", false);
          let result = transform_claude_request_in(&req, "test-project");
          assert!(result.is_ok());

          let body = result.unwrap();
          let model = body["request"]["model"].as_str().unwrap();
          assert_eq!(model, "gemini-3-pro-low");
      }

  test_2:
    location: "After test_1"
    code: |
      #[test]
      fn test_gemini_3_pro_low_thinking_budget_same_as_high() {
          let mut req_low = create_basic_request("gemini-3-pro-low", true);
          let mut req_high = create_basic_request("gemini-3-pro-high", true);

          req_low.thinking = Some(ThinkingConfig {
              type_: "enabled".to_string(),
              budget_tokens: Some(64000),  // Exceeds 32000 max
          });
          req_high.thinking = req_low.thinking.clone();

          let result_low = transform_claude_request_in(&req_low, "test");
          let result_high = transform_claude_request_in(&req_high, "test");

          let budget_low = result_low.unwrap()["request"]["generationConfig"]
              ["thinkingConfig"]["thinkingBudget"].as_i64().unwrap();
          let budget_high = result_high.unwrap()["request"]["generationConfig"]
              ["thinkingConfig"]["thinkingBudget"].as_i64().unwrap();

          // CRITICAL: Both tiers have SAME budget limit
          assert_eq!(budget_low, budget_high);
          assert_eq!(budget_low, 32000);
      }

coverage_target: "≥90% for Low tier specific code paths"
```

---

## 📋 Story Breakdown

### Story-009-01: Routing Aliases for Discoverability

**Priority**: P0 (Critical)
**Type**: Feature Addition
**Effort**: 3 hours
**Assignee**: Backend Developer

**Objective**: Add convenience routing aliases for gemini-3-pro-low to improve discoverability and user experience while maintaining intentional opt-in architecture.

#### Acceptance Criteria

**AC1: Alias Addition**
```yaml
GIVEN: Model mapping configuration
WHEN: Aliases are added to routing table
THEN:
  - "gemini-low" → "gemini-3-pro-low" ✅
  - "gemini-3-low" → "gemini-3-pro-low" ✅
  - Aliases are case-sensitive
  - No ambiguity with other models
```

**AC2: Routing Validation**
```yaml
GIVEN: User requests with alias names
WHEN: Request is routed
THEN:
  - "gemini-low" routes to gemini-3-pro-low
  - "gemini-3-low" routes to gemini-3-pro-low
  - Direct "gemini-3-pro-low" still works
  - Unknown models → High tier (no change)
```

**AC3: No Fallback to Low**
```yaml
GIVEN: Unknown model name
WHEN: Fallback routing is applied
THEN:
  - Unknown → "gemini-3-pro-high" (quality preserved)
  - NO automatic routing TO Low tier
  - Low tier requires explicit opt-in
  - Quality-over-cost default maintained
```

**AC4: Documentation Update**
```yaml
GIVEN: New aliases added
WHEN: Documentation is updated
THEN:
  - All aliases listed in docs
  - Usage examples provided
  - Rationale explained (explicit opt-in)
  - No Haiku fallback documented
```

#### Technical Implementation

**Code Location**: `src-tauri/src/proxy/common/model_mapping.rs:51,62-76`

**Changes**:
```rust
// Add after line 64 (after gemini-3-pro-high aliases)
m.insert("gemini-low", "gemini-3-pro-low");
m.insert("gemini-3-low", "gemini-3-pro-low");

// NO Haiku fallback to Low (preserve quality)
// NO gemini-pro → Low (preserve intentionality)
```

**Testing**:
```rust
#[test]
fn test_gemini_low_aliases() {
    let aliases = vec!["gemini-low", "gemini-3-low"];

    for alias in aliases {
        let mapped = map_claude_model_to_gemini(alias);
        assert_eq!(mapped, "gemini-3-pro-low");
    }
}

#[test]
fn test_unknown_models_to_high_not_low() {
    let unknown = map_claude_model_to_gemini("unknown-model");
    assert_eq!(unknown, "gemini-3-pro-high");  // Quality over cost
}
```

**Documentation**:
- Update `gemini-3-pro-low.md` with alias list
- Add usage examples
- Explain explicit opt-in philosophy

---

### Story-009-02: Model ID Discovery and Integration

**Priority**: P0 (Critical)
**Type**: Investigation + Implementation
**Effort**: 2 hours
**Assignee**: Backend Developer

**Objective**: Discover actual Gemini 3 Pro Low Model ID via network capture and add constant to code for consistent quota tracking and monitoring.

#### Acceptance Criteria

**AC1: Model ID Discovery**
```yaml
GIVEN: Network capture tools ready
WHEN: gemini-3-pro-low request is made
THEN:
  - Model ID extracted from v1internal request
  - Validated across ≥3 different accounts
  - Documented with discovery method
  - Confidence ≥95%
```

**AC2: Constant Definition**
```yaml
GIVEN: Model ID discovered
WHEN: Constant is added to code
THEN:
  - Constant defined in request.rs
  - Naming follows Claude pattern
  - Value matches discovered ID
  - Comment includes discovery date
```

**AC3: Mapping Integration**
```yaml
GIVEN: Constant defined
WHEN: get_model_id() is called with "gemini-3-pro-low"
THEN:
  - Returns actual Model ID (not 0)
  - Matches Claude model pattern
  - Works for base and thinking variants
  - Quota tracking improved
```

**AC4: High Tier Integration**
```yaml
GIVEN: Low tier Model ID discovered
WHEN: High tier Model ID is also discovered
THEN:
  - Both Low and High constants defined
  - Both mappings added to get_model_id()
  - Consistency validated
  - SAME TASK executed together
```

#### Technical Implementation

**Discovery Method**:
```yaml
step_1_setup:
  tool: "Chrome DevTools Network tab"
  filter: "XHR, v1internal"

step_2_request:
  model: "gemini-3-pro-low"
  account: "Use ≥3 different accounts"

step_3_capture:
  request_body: "Look for model_id or modelId field"
  location: "v1internal request payload"

step_4_validation:
  method: "Compare across accounts"
  confidence: "≥95% (same ID across accounts)"
```

**Code Changes**:
```rust
// src-tauri/src/proxy/mappers/claude/request.rs:11-24

// Add after line 13
const GEMINI_3_PRO_LOW_MODEL_ID: u32 = ???;  // Discovered 2026-01-XX via network
const GEMINI_3_PRO_HIGH_MODEL_ID: u32 = ???;  // Discovered 2026-01-XX via network

// Update get_model_id() match block
fn get_model_id(model_name: &str) -> u32 {
    match model_name {
        "claude-4.5-sonnet-thinking" => CLAUDE_4_5_SONNET_THINKING_MODEL_ID,
        "claude-4.5-sonnet" => CLAUDE_4_5_SONNET_MODEL_ID,
        "gemini-3-pro-low" => GEMINI_3_PRO_LOW_MODEL_ID,
        "gemini-3-pro-high" => GEMINI_3_PRO_HIGH_MODEL_ID,
        _ => 0, // Unknown model
    }
}
```

**Testing**:
```rust
#[test]
fn test_gemini_3_pro_low_model_id() {
    let id = get_model_id("gemini-3-pro-low");
    assert_ne!(id, 0);  // Not unknown
    assert!(id > 0);    // Valid ID
}

#[test]
fn test_gemini_3_pro_tier_ids_different() {
    let low_id = get_model_id("gemini-3-pro-low");
    let high_id = get_model_id("gemini-3-pro-high");
    assert_ne!(low_id, high_id);  // Different tiers, different IDs
}
```

---

### Story-009-03: Thinking Variant Naming Decision

**Priority**: P1 (High)
**Type**: Architecture Decision + Documentation
**Effort**: 2 hours
**Assignee**: Backend Developer + Technical Writer

**Objective**: Decide on thinking variant naming strategy and document architectural rationale for parameter-based vs model-name-based thinking activation.

#### Acceptance Criteria

**AC1: Decision Made**
```yaml
GIVEN: Two architectural options
WHEN: Team reviews pros/cons
THEN:
  - Decision documented (Option 1 or 2)
  - Rationale explained
  - Trade-offs analyzed
  - Future implications considered
```

**AC2: If Option 1 (Accept Parameter-Based) - RECOMMENDED**
```yaml
GIVEN: Parameter-based activation accepted
WHEN: Documentation is updated
THEN:
  - Architectural rationale explained
  - Benefits over model-name suffix documented
  - Usage examples provided
  - Consistency with Gemini API emphasized
```

**AC3: If Option 2 (Add -thinking Suffix)**
```yaml
GIVEN: Model name suffix routing added
WHEN: Implementation is complete
THEN:
  - Routing added: gemini-3-pro-low-thinking → gemini-3-pro-low
  - Tests validate routing
  - Documentation updated
  - Parameter-based still primary activation method
```

#### Technical Implementation

**Option 1: Accept Parameter-Based (RECOMMENDED)**

**Documentation Addition** (`gemini-3-pro-low.md`):
```markdown
## Model Naming Architecture

### Why No -thinking Suffix?

Gemini models use **parameter-based thinking activation** instead of model name
suffixes. This architectural pattern is SUPERIOR to Claude's -thinking suffix:

**Advantages**:
- **Flexibility**: Enable/disable thinking per request without changing model
- **Cleaner Architecture**: Fewer model names, less routing complexity
- **Budget Control**: Fine-grained control via thinkingConfig.thinkingLevel
- **API Consistency**: Matches Gemini API native parameter structure

**Usage Pattern**:
```json
{
  "model": "gemini-3-pro-low",
  "thinking": {
    "type": "enabled",
    "budget_tokens": 16000
  }
}
```

**vs Claude Pattern**:
```json
{
  "model": "gemini-3-pro-low-thinking"  // ❌ NOT USED
}
```

**Recommendation**: Use parameter-based activation for maximum flexibility.
```

**Option 2: Add -thinking Suffix**

**Code Addition** (`model_mapping.rs`):
```rust
// Add after Low tier aliases
m.insert("gemini-3-pro-low-thinking", "gemini-3-pro-low");

// Note: Routing only. Actual thinking activation still via parameter.
```

**Test Addition**:
```rust
#[test]
fn test_gemini_3_pro_low_thinking_suffix_routing() {
    let mapped = map_claude_model_to_gemini("gemini-3-pro-low-thinking");
    assert_eq!(mapped, "gemini-3-pro-low");
}
```

---

### Story-009-04: Error Recovery Documentation Enhancement

**Priority**: P2 (Medium)
**Type**: Investigation + Documentation
**Effort**: 3 hours
**Assignee**: Backend Developer + Technical Writer

**Objective**: Investigate corrupted signature retry logic, document implementation details, and ensure complete error handling transparency.

#### Acceptance Criteria

**AC1: Retry Logic Investigation**
```yaml
GIVEN: Corrupted signature retry documented in Story-004-02
WHEN: Codebase is searched for implementation
THEN:
  - Retry logic location identified
  - Configuration parameters documented
  - Backoff strategy validated
  - Test coverage confirmed
```

**AC2: Documentation Completeness**
```yaml
GIVEN: Retry logic found and analyzed
WHEN: Documentation is updated
THEN:
  - Error type 6 fully documented
  - Code references provided
  - Retry configuration explained
  - Backoff strategy detailed
  - Test coverage referenced
```

**AC3: All Error Types Documented**
```yaml
GIVEN: 6 error types exist
WHEN: Documentation review is complete
THEN:
  - All 6 error types have complete docs
  - Code references for each type
  - Test coverage for each type
  - 100% error handling transparency
```

#### Technical Implementation

**Investigation Steps**:
```yaml
step_1_search_patterns:
  files:
    - "src-tauri/src/proxy/upstream/"
    - "src-tauri/src/proxy/handlers/"
    - "src-tauri/src/proxy/rate_limit.rs"
  keywords:
    - "corrupted"
    - "signature"
    - "retry"
    - "503"
    - "backoff"

step_2_validate_story:
  reference: "docs/stories/Story-004-02-retry-503-signature-errors.md"
  extract: "Retry configuration, max attempts, backoff strategy"

step_3_test_coverage:
  search: "test_*signature*retry"
  validate: "Retry behavior tested"
```

**Documentation Template** (`gemini-3-pro-low.md` Error Handling section):
```markdown
### Type 6: Corrupted Signature Retry

**Error Pattern**: "Corrupted thought signature"

**Detection Logic**:
```rust
// Reference: [CODE_LOCATION]
if response.contains("corrupted") && response.contains("signature") {
    return Err(SignatureCorrupted);
}
```

**Retry Configuration**:
- **Enabled**: Yes (automatic)
- **Max Retries**: 3
- **Backoff Strategy**: Exponential (1s, 2s, 4s)
- **Total Max Wait**: 7 seconds

**Recovery Flow**:
1. Detect corrupted signature error (503 or error message)
2. Wait with exponential backoff
3. Retry request with same account
4. If 3 attempts fail → rotate account
5. Log retry attempts for monitoring

**Code References**:
- Detection: `[FILE:LINE]`
- Retry logic: `[FILE:LINE]`
- Tests: `[TEST_NAME]`

**Test Coverage**:
```rust
#[test]
fn test_corrupted_signature_retry() { ... }
```
```

---

### Story-009-05: Low Tier Specific Test Suite

**Priority**: P2 (Medium)
**Type**: Test Implementation
**Effort**: 2 hours
**Assignee**: Backend Developer

**Objective**: Add comprehensive test coverage specifically for gemini-3-pro-low to validate routing, budget equality, and tier-specific behavior.

#### Acceptance Criteria

**AC1: Direct Routing Test**
```yaml
GIVEN: gemini-3-pro-low model name
WHEN: Routing is performed
THEN:
  - Routes to gemini-3-pro-low (not High)
  - No fallback applied
  - Test passes
```

**AC2: Budget Equality Test**
```yaml
GIVEN: Low and High tier with same thinking config
WHEN: Budget clamping is applied
THEN:
  - Both tiers clamp to 32000 (SAME limit)
  - Budget equality validated
  - Test passes
```

**AC3: OpenAI Auto-Injection Test**
```yaml
GIVEN: OpenAI protocol request for gemini-3-pro-low
WHEN: Auto-injection logic runs
THEN:
  - ends_with("-low") detected correctly
  - 16000 default budget applied
  - thinkingConfig injected
  - Test passes
```

**AC4: Alias Routing Test**
```yaml
GIVEN: Aliases "gemini-low", "gemini-3-low"
WHEN: Routing is performed
THEN:
  - Both aliases route to gemini-3-pro-low
  - No fallback applied
  - Test passes
```

**AC5: Test Coverage Metrics**
```yaml
GIVEN: New tests added
WHEN: Coverage is measured
THEN:
  - Low tier specific code ≥90% covered
  - No regression in existing tests
  - All tests pass
```

#### Technical Implementation

**Test Additions** (`src-tauri/src/proxy/tests/thinking_models.rs`):

```rust
/// Test direct routing for gemini-3-pro-low
#[test]
fn test_gemini_3_pro_low_routing() {
    let req = create_basic_request("gemini-3-pro-low", false);
    let result = transform_claude_request_in(&req, "test-project");
    assert!(result.is_ok());

    let body = result.unwrap();
    let model = body["request"]["model"].as_str().unwrap();
    assert_eq!(
        model, "gemini-3-pro-low",
        "Direct routing should preserve Low tier model name"
    );
}

/// Test that Low and High tiers have SAME thinking budget limit
#[test]
fn test_gemini_3_pro_low_thinking_budget_same_as_high() {
    let mut req_low = create_basic_request("gemini-3-pro-low", true);
    let mut req_high = create_basic_request("gemini-3-pro-high", true);

    // Both request 64000 (exceeds 32000 max)
    req_low.thinking = Some(ThinkingConfig {
        type_: "enabled".to_string(),
        budget_tokens: Some(64000),
    });
    req_high.thinking = req_low.thinking.clone();

    let result_low = transform_claude_request_in(&req_low, "test");
    let result_high = transform_claude_request_in(&req_high, "test");

    let budget_low = result_low.unwrap()["request"]["generationConfig"]
        ["thinkingConfig"]["thinkingBudget"].as_i64().unwrap();
    let budget_high = result_high.unwrap()["request"]["generationConfig"]
        ["thinkingConfig"]["thinkingBudget"].as_i64().unwrap();

    // CRITICAL: Both tiers MUST have SAME budget limit (32000)
    assert_eq!(
        budget_low, budget_high,
        "Low and High tiers should have IDENTICAL thinking budget limits"
    );
    assert_eq!(
        budget_low, 32000,
        "Gemini 3 Pro (Low and High) max budget should be 32000"
    );
}

/// Test OpenAI protocol auto-injection for gemini-3-pro-low
#[test]
fn test_openai_auto_injection_gemini_3_pro_low() {
    // Simulate OpenAI protocol request
    let openai_req = OpenAIRequest {
        model: "gemini-3-pro-low".to_string(),
        messages: vec![/* ... */],
        // No explicit thinking config (should auto-inject)
    };

    let result = transform_openai_request(&openai_req);
    assert!(result.is_ok());

    let body = result.unwrap();
    let thinking_config = &body["request"]["generationConfig"]["thinkingConfig"];

    // Validate auto-injection
    assert!(thinking_config.is_object());
    assert_eq!(thinking_config["includeThoughts"].as_bool().unwrap(), true);
    assert_eq!(
        thinking_config["thinkingBudget"].as_i64().unwrap(),
        16000,  // Default for OpenAI protocol
        "OpenAI protocol should auto-inject 16000 thinking budget for Low tier"
    );
}

/// Test alias routing for gemini-low and gemini-3-low
#[test]
fn test_gemini_low_aliases() {
    let aliases = vec!["gemini-low", "gemini-3-low"];

    for alias in aliases {
        let req = create_basic_request(alias, false);
        let result = transform_claude_request_in(&req, "test");
        assert!(result.is_ok());

        let body = result.unwrap();
        let model = body["request"]["model"].as_str().unwrap();
        assert_eq!(
            model, "gemini-3-pro-low",
            "Alias '{}' should route to gemini-3-pro-low", alias
        );
    }
}
```

---

### Story-009-06: Integration, Documentation & Deployment

**Priority**: P1 (High)
**Type**: Integration + Documentation + Deployment
**Effort**: 2 days
**Assignee**: Backend Developer + Technical Writer

**Objective**: Integrate all Epic-009 features, complete comprehensive documentation, validate deployment readiness, and emphasize critical value proposition (budget equality).

#### Acceptance Criteria

**AC1: Feature Integration**
```yaml
GIVEN: All 5 previous stories completed
WHEN: Features are integrated
THEN:
  - No conflicts between features
  - All tests pass (unit + integration)
  - Code review approved
  - No regressions introduced
```

**AC2: Documentation Completeness**
```yaml
documentation_requirements:
  user_guide:
    - "Gemini 3 Pro Low: Complete Guide"
    - "Budget Equality with High Tier (EMPHASIZED)"
    - "Cost Optimization Strategies"
    - "When to Use Low vs High"

  technical_docs:
    - "Architecture: Routing and Aliases"
    - "Model ID Discovery Process"
    - "Error Handling: All 6 Types"
    - "Test Coverage Report"

  value_proposition:
    - "SAME 32000 Token Thinking Budget" (CRITICAL)
    - "40-60% Cost Savings"
    - "Use Case Expansion"
    - "Cost-Optimized Reasoning Specialist"
```

**AC3: Value Proposition Emphasis**
```yaml
GIVEN: Budget equality is CRITICAL discovery
WHEN: Documentation is written
THEN:
  - Budget equality in executive summary
  - Comparison matrix (Low vs High)
  - Use case recommendations
  - Cost-quality tradeoff analysis
  - Positioning as "reasoning specialist"
```

**AC4: Deployment Readiness**
```yaml
GIVEN: Epic-009 ready for production
WHEN: Deployment checklist is reviewed
THEN:
  - All acceptance criteria met (100%)
  - Code review approved
  - Documentation complete
  - Test coverage ≥90%
  - Migration plan validated
  - Rollback plan tested
  - Monitoring configured
```

#### Implementation Tasks

**Integration Tasks**:
1. Merge all 5 story branches
2. Resolve merge conflicts (if any)
3. Run full test suite
4. Validate end-to-end workflows
5. Performance benchmarking
6. Code review with senior developers

**Documentation Tasks**:
1. **CRITICAL**: Emphasize budget equality in all docs
2. Create comprehensive user guide
3. Update technical architecture docs
4. Generate API reference (Rust doc comments)
5. Create deployment guide
6. Update CHANGELOG.md
7. Create release notes highlighting value proposition

**Value Proposition Documentation** (Sample):

```markdown
# Gemini 3 Pro Low: Cost-Optimized Reasoning Specialist

## 🌟 Critical Discovery: Budget Equality

**Gemini 3 Pro Low has the SAME 32000 token thinking budget as High tier!**

```yaml
thinking_capability:
  gemini_3_pro_low: 32000 tokens
  gemini_3_pro_high: 32000 tokens
  difference: 0

cost_difference:
  source: "Base model quality, NOT thinking budget"
  savings: "40-60%"
  reasoning_capability: "IDENTICAL"
```

## When to Use Low vs High

### Use Low Tier For:
- **Code Analysis**: Reasoning depth > eloquence
- **Data Extraction**: Logic > presentation quality
- **Problem Solving**: Capability > polish
- **Multi-Step Reasoning**: Depth > output quality
- **Cost Optimization**: Same thinking, lower cost

### Use High Tier For:
- **Customer-Facing**: Quality presentation matters
- **Production Critical**: Maximum quality required
- **Brand Consistency**: Premium output needed

## Value Proposition

**Low tier is NOT inferior** - it's a **cost-optimized reasoning specialist**:
- ✅ Same 32000 token thinking budget
- ✅ Same reasoning depth
- ✅ 40-60% cost savings
- ✅ Optimal for reasoning-heavy, non-critical tasks
```

**Testing Tasks**:
1. Run full test suite (unit + integration + E2E)
2. Performance testing (no regression)
3. Load testing (routing under stress)
4. Regression testing (existing functionality)
5. Documentation validation (examples work)

**Deployment Tasks**:
1. Create deployment checklist
2. Plan phased rollout
3. Configure monitoring alerts
4. Prepare rollback plan
5. Schedule deployment window

#### Success Metrics

```yaml
integration_quality:
  merge_conflicts: "0 unresolved"
  test_pass_rate: "100%"
  code_review: "✅ Approved"

documentation_quality:
  completeness: "100%"
  value_proposition: "✅ EMPHASIZED"
  accuracy: "Validated"

deployment_readiness:
  checklist: "100% complete"
  rollback: "✅ Tested"
  monitoring: "✅ Configured"
```

---

## 🗓️ Implementation Timeline

### Overview

```yaml
total_duration: "1-2 weeks"
story_count: 6
parallel_execution: "Possible for Stories 009-01, 009-02, 009-04"
sequential_requirements:
  - "Story-009-03 after team decision"
  - "Story-009-05 after Story-009-01 (alias tests)"
  - "Story-009-06 must be last (integration)"

estimated_breakdown:
  Story-009-01: "3 hours (aliases)"
  Story-009-02: "2 hours (Model ID)"
  Story-009-03: "2 hours (decision + docs)"
  Story-009-04: "3 hours (error docs)"
  Story-009-05: "2 hours (tests)"
  Story-009-06: "2 days (integration + docs)"
```

### Phase 1: Parallel Critical Fixes (Day 1)

**Story-009-01: Routing Aliases** (Developer A, 3h)
- Morning: Add aliases to model_mapping.rs
- Afternoon: Test validation, documentation

**Story-009-02: Model ID Discovery** (Developer B, 2h)
- Morning: Network capture, ID discovery
- Afternoon: Constant addition, mapping integration

**Story-009-04: Error Recovery Docs** (Technical Writer, 3h)
- Morning: Code investigation
- Afternoon: Documentation writing

### Phase 2: Sequential Enhancements (Day 2)

**Story-009-03: Thinking Variant Decision** (Team, 2h)
- Morning: Team discussion, decision
- (If Option 1): Documentation update only

**Story-009-05: Test Suite** (Developer A, 2h)
- Afternoon: Add 4 test cases
- Validate coverage ≥90%

### Phase 3: Integration & Documentation (Days 3-4)

**Story-009-06: Integration** (Full Team, 2 days)
- Day 3 Morning: Feature integration, conflict resolution
- Day 3 Afternoon: Full test suite, performance validation
- Day 4 Morning: Documentation completion (VALUE PROPOSITION)
- Day 4 Afternoon: Deployment preparation, final review

---

## 📊 Success Metrics & Validation

### Quantitative Metrics

```yaml
compliance_target: "100%"
gap_resolution: "5/5 gaps (100%)"

cost_efficiency:
  cost_savings: "40-60% vs High tier"
  thinking_capability: "100% equal (32000 tokens)"
  value_ratio: "1.6x - 2.5x better cost/capability"

quality:
  test_coverage: "≥90%"
  code_review: "Approved"
  documentation: "100% complete"

discoverability:
  alias_adoption: "≥20% of Low tier requests"
  user_confusion: "<5% support tickets"

monitoring:
  model_id_tracking: "100% attribution"
  quota_visibility: "Per-tier granularity"
```

### Qualitative Metrics

```yaml
user_experience:
  - "Easier to discover Low tier (aliases)"
  - "Clear value proposition (budget equality)"
  - "Confidence in tier selection (docs)"

developer_experience:
  - "Well-documented features"
  - "Clear architectural decisions"
  - "Comprehensive test coverage"

business_value:
  - "Cost optimization opportunities"
  - "Market positioning clarity"
  - "Use case expansion potential"
```

### Validation Criteria

**Before Production Deployment**:
1. All 6 stories completed (100%)
2. Test coverage ≥90%
3. Performance benchmarks passed (no regression)
4. Documentation reviewed and approved
5. Value proposition emphasized in all docs
6. Rollback plan tested
7. Monitoring alerts configured

---

## 🔗 Dependencies & References

### Epic Dependencies

**Prerequisite**: Epic-008 completion
- Epic-008 must be fully deployed
- Team resources available
- No blocking issues

**Strategic Context**: Completes Gemini 3 Pro Trilogy
- Epic-005: Gemini 3 Pro High (96.4% → 100%) ✅
- Epic-007: Gemini 3 Pro Image (86.7% → 100%) 🚧 IN PROGRESS
- Epic-009: Gemini 3 Pro Low (82.1% → 100%) ← THIS EPIC

**After Epic-009**:
```yaml
gemini_3_pro_series: "100% COMPLETE"
gemini_3_pro_models:
  - gemini-3-pro-high: "100% (96.4% base + Epic-005)"
  - gemini-3-pro-image: "100% (Epic-007)"
  - gemini-3-pro-low: "100% (Epic-009)"

strategic_milestone: "Gemini 3.x Pro series fully optimized"
```

### Reference Documents

**Analysis Documents**:
- [gemini-3-pro-low-COMPARISON.md](../antigravity/workflows/models/gemini/gemini-3-pro-low-COMPARISON.md) - Gap analysis source (1406 lines)
- [Epic-007-SELECTION-ANALYSIS.md](../epic/Epic-007-SELECTION-ANALYSIS.md) - Selection rationale

**Similar Patterns**:
- [Epic-005-Gemini-3-Pro-High-Compliance.md](Epic-005-Gemini-3-Pro-High-Compliance.md) - Same compliance pattern
- [Epic-007-Gemini-3-Pro-Image-Compliance.md](Epic-007-Gemini-3-Pro-Image-Compliance.md) - Feature completion pattern

**Code References**:
- `src-tauri/src/proxy/common/model_mapping.rs:51,62-76` - Routing logic
- `src-tauri/src/proxy/mappers/claude/request.rs:11-24` - Model ID constants
- `src-tauri/src/proxy/mappers/claude/request.rs:277-362,604-656` - Thinking logic
- `src-tauri/src/proxy/mappers/openai/request.rs:246-272` - Auto-injection
- `src-tauri/src/proxy/tests/thinking_models.rs:381-403` - Test suite

---

## 🎯 Strategic Alignment

### Epic-009 in Project Context

**Gemini 3.x Series Completion**:
```yaml
gemini_3_series_progress:
  pro_high: "✅ 100% (Epic-005)"
  pro_image: "🚧 Epic-007 (in progress)"
  pro_low: "📋 Epic-009 (this epic)"
  flash: "⏳ TODO (68.8%, API incompatibility)"

milestone_after_epic_009:
  status: "Gemini 3.x Pro series = 100% COMPLETE"
  coverage: "All primary Pro models fully optimized"
  strategic_value: "Complete tier offering"
```

**Critical Value Proposition**:
```yaml
budget_equality_impact:
  discovery: "Low = High thinking budget (32000)"
  repositioning: "Cost-optimized reasoning specialist"
  market_impact: "Undervalued tier becomes competitive advantage"

use_case_expansion:
  current: "Development, testing, internal tools"
  expanded: "Production reasoning, code analysis, data extraction"
  value: "40-60% savings with equal capability"
```

### Next Epic (After Epic-009)

**Planned**: TBD (Q2 2026 options)
- gemini-3-flash (68.8%, API migration required, risky)
- gemini-2.0-flash-exp (76.5%, audio focus)
- gemini-2.5-flash-thinking (requires COMPARISON creation)

**After Gemini 3 Pro Trilogy Complete**:
- Strategic review of Gemini 2.x opportunities
- User demand analysis
- API stability assessment

---

## 📝 Notes & Considerations

### Technical Debt

**None created** - This epic addresses existing gaps without introducing debt.

**Considerations**:
- Model ID discovery may require multiple attempts (network timing)
- Alias naming should be validated with users before final decision
- Test suite may expand as edge cases are discovered

### Risk Assessment

```yaml
risk_level: "🟢 Low"

identified_risks:
  - risk: "Model ID discovery fails"
    mitigation: "Use multiple accounts, different network conditions"
    probability: "Low"
    impact: "Medium"

  - risk: "Alias naming conflicts"
    mitigation: "Conservative alias choices, documentation"
    probability: "Very Low"
    impact: "Low"

  - risk: "Test coverage insufficient"
    mitigation: "Add tests iteratively, coverage monitoring"
    probability: "Low"
    impact: "Medium"
```

### Critical Value Proposition Communication

**MOST IMPORTANT**: Budget equality must be emphasized in ALL documentation:

```yaml
critical_messaging:
  primary: "SAME 32000 token thinking budget as High tier"
  secondary: "40-60% cost savings from base model, NOT thinking reduction"
  tertiary: "Cost-optimized reasoning specialist, not inferior High"

communication_channels:
  - Epic documentation (this document)
  - User guide
  - API documentation
  - Marketing materials
  - Release notes
  - Support knowledge base

impact: "This discovery changes market positioning for Low tier"
```

### Future Enhancements

**Post-Epic-009 Opportunities**:
1. **Usage Analytics**: Track Low vs High tier adoption patterns
2. **Cost Optimization Dashboard**: Real-time savings visualization
3. **Tier Recommendation Engine**: Suggest optimal tier per use case
4. **A/B Testing**: Validate quality differences in production
5. **Expanded Alias Set**: Based on user feedback and usage patterns

---

## ✅ Definition of Done

**Epic-009 is considered DONE when**:

1. ✅ All 6 stories completed with 100% acceptance criteria met
2. ✅ Compliance increased from 82.1% to 100%
3. ✅ Routing aliases added and tested
4. ✅ Model ID discovered and integrated
5. ✅ Thinking variant naming decision documented
6. ✅ Error recovery fully documented (all 6 types)
7. ✅ Test coverage ≥90% for Low tier specific code
8. ✅ Documentation complete with VALUE PROPOSITION emphasized
9. ✅ Code review approved by senior developers
10. ✅ All tests passing (unit + integration)
11. ✅ Performance validated (no regression)
12. ✅ Deployment guide ready
13. ✅ Production deployment successful
14. ✅ Gemini 3 Pro Trilogy = 100% COMPLETE

---

**Document Status**: ✅ COMPLETE
**Next Step**: Team review and Epic-009 activation after Epic-008 completion
**Estimated Start**: After Epic-008 (~2026-02-11)
**Estimated Completion**: ~2026-02-25 (2 weeks from Epic-008 end)

**Strategic Milestone**: After Epic-009 → Gemini 3.x Pro series 100% complete
