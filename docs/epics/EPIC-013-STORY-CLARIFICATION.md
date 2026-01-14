# Epic-013 Story Clarification: Stories 013-02 & 013-03

**Date**: 2026-01-11
**Author**: Product Manager
**Purpose**: Clarify ambiguous stories identified in Epic-013 validation report
**Status**: DRAFT - Awaiting Product Owner approval

---

## Background

Epic-013 validation report (EPIC-013-VALIDATION-REPORT.md) identified **CRITICAL FINDING #2**: Stories 013-02 and 013-03 have ambiguous scope because:

1. **Story 013-02 (Safety Settings)**: COMPARISON shows "Content filter handling ✅ IMPLEMENTED" (line 283)
2. **Story 013-03 (Streaming)**: COMPARISON shows "TTFT ✅ IMPLEMENTED" (line 216)

**Question**: What specific enhancements are needed if features already implemented?

---

## Story 013-02: Safety Settings Enhancement

### Current Story Specification

```yaml
story_013_02:
  title: "Safety Settings Enhancement"
  priority: "P1"
  effort: "2 days"
  focus: "Content filtering + harm categories"
```

### Investigation Results

**COMPARISON Evidence** (lines 278-288):
```yaml
error_handling:
  content_filter_handling: "✅ IMPLEMENTED (line 283)"

  not_implemented:
    - "Thinking-specific errors (line 284)"
    - "Level fallback chain (line 285)"
```

**Conclusion**: Basic content filtering EXISTS. Enhancement likely means:
- Adding level-specific safety validation
- Enhancing error messages for safety rejections
- Adding fallback chain when safety blocks certain levels

### 🎯 Proposed Clarification Option A: Level-Specific Safety Validation

**Revised Story**:
```yaml
story_013_02:
  title: "Level-Specific Safety Error Handling"
  priority: "P1"
  effort: "2 days"
  focus: "Enhance safety error messages with level context"

  description: |
    When safety filters block requests, enhance error handling to:
    1. Include which thinking level was attempted (MINIMAL/LOW/MEDIUM/HIGH)
    2. Provide level-specific fallback suggestions
    3. Log level distribution in safety-blocked requests

  acceptance_criteria:
    ac_1_level_in_error:
      criterion: "Safety error responses include attempted thinking level"
      test: |
        Given: Request with MEDIUM level + violating content
        When: Safety filter blocks request
        Then: Error message states "Request blocked at MEDIUM level (10001-20000 tokens)"

      implementation: "Modify error handler to include thinkingLevel context"
      location: "handlers/gemini.rs error responses"

    ac_2_fallback_suggestion:
      criterion: "Error response suggests lower thinking level retry"
      test: |
        Given: Request blocked at HIGH level
        When: Safety filter triggers
        Then: Error suggests "Consider retrying with MEDIUM or LOW level"

      implementation: "Add fallback suggestion logic"
      location: "error handler middleware"

    ac_3_safety_metrics:
      criterion: "Monitor tracks safety blocks by level"
      test: |
        Given: 10 safety blocks (3 HIGH, 5 MEDIUM, 2 LOW)
        When: Query monitoring dashboard
        Then: Dashboard shows safety block distribution by level

      implementation: "Add level dimension to safety metrics"
      location: "monitor.rs"

  gap_closed: "Partial Gap 3 (TEST-001) - Error handling tests"

  dependencies:
    - "Epic-011: thinkingLevel API must be working"
    - "Story 013-01: MEDIUM level validation complete"
```

### 🎯 Proposed Clarification Option B: Remove Story (If Redundant)

**If Product Owner confirms basic safety is sufficient**:
```yaml
decision: "REMOVE Story 013-02"
rationale: "Content filtering already implemented (COMPARISON line 283)"

reallocate_effort:
  story_013_04: "+1 day (more comprehensive error logging)"
  integration_testing: "+1 day (additional safety test scenarios)"
```

---

## Story 013-03: Streaming Optimization

### Current Story Specification

```yaml
story_013_03:
  title: "Streaming Optimization"
  priority: "P1"
  effort: "1 day"
  focus: "TTFT + progressive rendering"
```

### Investigation Results

**COMPARISON Evidence** (lines 210-223):
```yaml
performance_characteristics:
  streaming_support: "✅ IMPLEMENTED (line 215)"
  ttft_optimization: "✅ IMPLEMENTED (line 216)"

  compliance: "100% (4/4 testable)"
```

**Undocumented Feature** (lines 334-348):
```yaml
auto_stream_conversion:
  description: "Non-streaming requests converted to streaming at upstream"
  benefit: "Reduces 429 errors"
  user_facing: false
  documented: NO ❌

  recommendation: "Document in workflow as optimization"
```

**Conclusion**: TTFT and streaming ALREADY OPTIMIZED. Enhancement likely means:
- **Option A**: Document auto-stream conversion feature
- **Option B**: Further optimize TTFT with explicit metrics target
- **Option C**: Add progressive rendering enhancements

### 🎯 Proposed Clarification Option A: Document Auto-Stream Feature

**Revised Story**:
```yaml
story_013_03:
  title: "Document Auto-Stream Conversion"
  priority: "P1"
  effort: "1 day"
  focus: "Document undocumented optimization feature"

  description: |
    Auto-stream conversion is an undocumented feature that converts
    non-streaming requests to streaming at upstream, then collects results.
    This reduces 429 rate limit errors.

    Task: Document this feature in workflow documentation and ensure
    it works correctly for all 4 thinking levels.

  acceptance_criteria:
    ac_1_documentation:
      criterion: "Auto-stream feature documented in workflow"
      test: |
        Given: New developer reads gemini-3-flash workflow
        When: Looking for rate limit optimization strategies
        Then: Auto-stream conversion is explained with code references

      implementation: "Add section to workflow documentation"
      location: "docs/antigravity/workflows/models/gemini/"

    ac_2_level_compatibility:
      criterion: "Auto-stream works for all 4 thinking levels"
      test: |
        Given: Non-streaming request with MEDIUM level
        When: Sent to upstream
        Then: Request converted to streaming, MEDIUM level preserved

      implementation: "Verify auto-stream preserves thinkingLevel"
      location: "handlers/gemini.rs, server.rs"

    ac_3_metrics_tracking:
      criterion: "Monitor tracks auto-stream conversion rate"
      test: |
        Given: 100 requests (60 non-streaming, 40 streaming)
        When: Check monitoring dashboard
        Then: Dashboard shows "60% auto-converted to streaming"

      implementation: "Add auto-stream metric"
      location: "monitor.rs"

  gap_closed: "Documentation gap for undocumented feature (COMPARISON lines 334-348)"

  dependencies:
    - "Epic-011: thinkingLevel API must be working"
    - "Story 013-01: MEDIUM level must be functional"
```

### 🎯 Proposed Clarification Option B: TTFT Performance Target

**Revised Story**:
```yaml
story_013_03:
  title: "TTFT Performance Optimization"
  priority: "P1"
  effort: "2 days"  # Increased from 1 day
  focus: "Measurable TTFT improvement for thinking mode"

  description: |
    Current TTFT is optimized but not measured for thinking mode.
    Establish baseline and improve TTFT by 15-20% for all thinking levels.

  acceptance_criteria:
    ac_1_baseline_measurement:
      criterion: "TTFT baseline established for all 4 levels"
      test: |
        Given: 100 requests per level (MINIMAL, LOW, MEDIUM, HIGH)
        When: Measure P50, P95, P99 TTFT
        Then: Baseline documented in performance report

      implementation: "Add TTFT tracking per thinking level"
      location: "monitor.rs"

      baseline_targets:
        minimal: "P50 <150ms, P95 <300ms, P99 <500ms"
        low: "P50 <200ms, P95 <400ms, P99 <700ms"
        medium: "P50 <250ms, P95 <500ms, P99 <900ms"
        high: "P50 <300ms, P95 <600ms, P99 <1100ms"

    ac_2_optimization_implementation:
      criterion: "TTFT improved by 15-20% for all levels"
      test: |
        Given: Baseline TTFT measurements
        When: Optimization implemented
        Then: New TTFT is 15-20% faster (e.g., MEDIUM P95: 500ms → 400-425ms)

      implementation: "Optimize request preprocessing and level mapping"
      location: "handlers/gemini.rs, mappers/gemini/"

    ac_3_progressive_rendering:
      criterion: "First token arrives within target TTFT"
      test: |
        Given: Streaming request with MEDIUM level
        When: Request sent
        Then: First token received within 250ms P50 target

      implementation: "Ensure streaming starts immediately"
      location: "streaming.rs"

  gap_closed: "Performance measurement gap (quantified TTFT targets)"

  dependencies:
    - "Epic-011: thinkingLevel API must be working"
    - "Monitoring infrastructure: P50/P95/P99 tracking"
```

### 🎯 Proposed Clarification Option C: Remove Story (If Sufficient)

**If Product Owner confirms current streaming is sufficient**:
```yaml
decision: "REMOVE Story 013-03"
rationale: "Streaming and TTFT already optimized (COMPARISON line 220: 100% compliance)"

reallocate_effort:
  integration_testing: "+1 day (add streaming performance tests)"
```

---

## Recommendation Matrix

### Story 013-02: Safety Settings

| Option | Title | Effort | Pros | Cons | Recommendation |
|--------|-------|--------|------|------|----------------|
| **A** | Level-Specific Safety | 2 days | Adds value, measurable | Requires Epic-011 | ⭐ **RECOMMENDED** |
| **B** | Remove Story | 0 days | Avoids redundant work | Less comprehensive error handling | If basic safety sufficient |

**Recommended**: **Option A** - Adds meaningful enhancement (level context in errors)

### Story 013-03: Streaming

| Option | Title | Effort | Pros | Cons | Recommendation |
|--------|-------|--------|------|------|----------------|
| **A** | Document Auto-Stream | 1 day | Low effort, closes doc gap | Not technical enhancement | ⭐ **RECOMMENDED** |
| **B** | TTFT Optimization | 2 days | Measurable improvement | Requires more effort | If performance critical |
| **C** | Remove Story | 0 days | Avoids redundant work | Undocumented feature remains | Not recommended |

**Recommended**: **Option A** - Documents important undocumented feature (auto-stream conversion)

---

## Proposed Updated Epic-013 Stories

### If Recommendations Accepted

```yaml
epic_013_phase_2_stories_revised:

  story_013_01:
    title: "MEDIUM Level Testing & Validation"
    priority: "P1"
    effort: "2 days"
    status: "UNCHANGED ✅"

  story_013_02:
    title: "Level-Specific Safety Error Handling"  # REVISED
    priority: "P1"
    effort: "2 days"
    focus: "Safety errors with level context and fallback suggestions"
    status: "CLARIFIED ✅"

  story_013_03:
    title: "Document Auto-Stream Conversion"  # REVISED
    priority: "P1"
    effort: "1 day"
    focus: "Document undocumented rate limit optimization"
    status: "CLARIFIED ✅"

phase_2_total: "5 days (unchanged)"
```

---

## Next Steps

### 1. Product Owner Review (Priority: HIGH)

**Questions for PO**:

**Story 013-02**:
- [ ] Accept Option A (Level-Specific Safety Error Handling)?
- [ ] OR Remove story as redundant?
- [ ] Any additional safety requirements?

**Story 013-03**:
- [ ] Accept Option A (Document Auto-Stream Conversion)?
- [ ] OR Option B (TTFT Optimization with metrics)?
- [ ] OR Remove story as sufficient?

### 2. Update Epic-013 Specification

**If PO approves recommendations**:
- Update FUTURE-EPICS-ROADMAP-Q2-2026.md lines 251-262
- Add detailed acceptance criteria
- Update Epic-013 validation report with clarifications

### 3. Re-validate Epic-013

**After clarifications**:
- Update validation report to mark CRITICAL-002 as RESOLVED
- Confirm stories now have clear, measurable acceptance criteria
- Verify compliance target achievable (68.8% → 95%+)

---

## Approval Checklist

- [ ] Product Owner reviewed clarification options
- [ ] Story 013-02 decision: Option A / Option B / Other
- [ ] Story 013-03 decision: Option A / Option B / Option C / Other
- [ ] Acceptance criteria approved and measurable
- [ ] Epic-013 specification updated
- [ ] Validation report updated (CRITICAL-002 RESOLVED)
- [ ] Development team briefed on clarified scope

---

**Status**: 🟡 **AWAITING PRODUCT OWNER APPROVAL**

**Blocker for**: Epic-013 start date (2026-04-21)

**Estimated Review Time**: 30 minutes

**Document Version**: 1.0 (DRAFT)
