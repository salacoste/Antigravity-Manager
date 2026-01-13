# EPIC-020: Model IDs 314-327 Investigation

**Status**: RESEARCH PHASE
**Priority**: P2 (Documentation Completeness)
**Start Date**: 2026-01-22 (Week 1 - Research Only)
**Team**: Tech Lead + 1 Developer (Team 1)
**Complexity**: HIGH (Unknown unknowns)

---

## Executive Summary

### Problem Statement

**Current Documentation Gap**: 72.2% coverage (39/54+ models documented)
- **Known Models**: 39 models with COMPARISON files
- **Unknown Models**: ~25 models (IDs 314-327 + additional gaps)
- **Target Coverage**: 98%+ for professional-grade documentation

**Business Impact**:
- Incomplete model catalog reduces user confidence
- Users may miss optimal model choices for their use cases
- Competitive disadvantage vs. well-documented AI tools
- Support burden from undocumented model questions

### Research Objective

Conduct 1-week investigation (5 days) to:
1. **Identify** all unknown model IDs (314-327 + gaps)
2. **Test** model capabilities via live API validation
3. **Document** findings in COMPARISON files (conditional)
4. **Recommend** implementation approach (Go/No-Go decision)

### Budget Commitment

**Phase 1 (Research)**: 1 week = LOW commitment
- No code implementation
- No breaking changes
- Reversible decision at week end

**Phase 2 (Implementation)**: CONDITIONAL on research findings
- Effort: 1-4 weeks (scenario-dependent)
- Timeline: Q2/Q3 2026
- Decision: Based on ROI analysis

---

## Problem Analysis

### Gap Identification

**Primary Gap: Model IDs 314-327**
```
Known IDs: 1-313, 328+
Unknown IDs: 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327
Total: 14 models
```

**Secondary Gaps: Additional Missing Models**
- Estimated ~11 additional models with non-sequential IDs
- Discovered via code analysis and API logs
- Total unknown: ~25 models

**Code Evidence**:
```rust
// src-tauri/src/proxy/mappers/gemini/model_mapping.rs
// Gap between lines where IDs 314-327 are referenced but not documented
```

### Current Documentation State

**MASTER-MODELS-TABLE.md Analysis**:
- Total rows: 54+ models
- Documented: 39 models with COMPARISON files
- Undocumented: 15+ models without detailed documentation
- Coverage: 72.2%

**COMPARISON Files Inventory**:
```
docs/comparison/
├── COMPARISON-GEMINI-1.5-FLASH.md
├── COMPARISON-GEMINI-1.5-PRO.md
├── COMPARISON-GEMINI-2.0-FLASH-EXP.md
├── COMPARISON-GEMINI-2.0-FLASH-THINKING-EXP.md
└── ... (39 total files)
```

**Missing COMPARISON Files**: ~15 files for unknown models

---

## Research Approach

### Phase 1: Code Analysis (Day 1-2)

**Objective**: Identify model IDs through codebase analysis

**Methodology**:
1. **Pattern Recognition**
   ```bash
   # Search for model ID references
   rg "31[4-7]|32[0-7]" src-tauri/src/proxy/
   grep -r "gemini.*thinking" src-tauri/
   ```

2. **Mapper Analysis**
   - Review `model_mapping.rs` for enum definitions
   - Check request/response mappers for model-specific logic
   - Identify feature flags (thinking, vision, tools, audio)

3. **Upstream Client Analysis**
   - Trace model routing in `upstream/client.rs`
   - Check error handling for unknown models
   - Identify API version compatibility

4. **Log Analysis**
   - Search application logs for model ID occurrences
   - Check error logs for unhandled model references
   - Identify user-requested models not in documentation

**Deliverable**: Model ID hypothesis list with code evidence

---

### Phase 2: Live API Validation (Day 3-4)

**Objective**: Test each model ID via live API calls

**Test Harness**:
```rust
// Test framework for model capability detection
#[tokio::test]
async fn test_model_capabilities() {
    let unknown_models = vec![314, 315, 316, /* ... */ 327];

    for model_id in unknown_models {
        let result = test_model_features(model_id).await;
        assert!(result.is_ok(), "Model {} validation failed", model_id);
    }
}

async fn test_model_features(model_id: u32) -> Result<ModelCapabilities> {
    // Test: Basic text generation
    // Test: System instructions
    // Test: Tool calling
    // Test: Vision input
    // Test: Thinking mode
    // Test: Audio input/output
}
```

**Capability Matrix Template**:
| Model ID | Text | Vision | Tools | Thinking | Audio | Status | Performance |
|----------|------|--------|-------|----------|-------|--------|-------------|
| 314 | ✅ | ❌ | ✅ | ❌ | ❌ | Active | Fast |
| 315 | ✅ | ✅ | ✅ | ✅ | ❌ | Active | Slow |
| ... | ... | ... | ... | ... | ... | ... | ... |

**Testing Checklist** (per model):
- [ ] Basic text generation (100-word prompt)
- [ ] System instructions support
- [ ] Tool/function calling (calculator test)
- [ ] Vision input (image analysis)
- [ ] Thinking mode activation
- [ ] Audio input (speech-to-text)
- [ ] Audio output (text-to-speech)
- [ ] Rate limit behavior (429 handling)
- [ ] Error responses (400, 404, 500)
- [ ] Response time benchmarks (p50, p95, p99)

**Deliverable**: Capability matrix with test evidence

---

### Phase 3: Documentation & Decision (Day 5)

**Objective**: Synthesize findings and recommend next steps

**Analysis Framework**:
1. **Categorization**: Classify models by scenario (A/B/C/D)
2. **ROI Calculation**: Effort vs. value for each model
3. **Prioritization**: Rank models by business value
4. **Decision**: Go/No-Go recommendation

**COMPARISON File Creation**:
- Create files for high-value, unique models only
- Follow established template structure
- Include test evidence and benchmarks
- Cross-reference with MASTER-MODELS-TABLE.md

**Deliverable**: Implementation recommendation report

---

## Four Potential Scenarios

### Scenario A: Vertex AI Variants (Low Effort)

**Hypothesis**: Models 314-327 are aliases or variants of existing models

**Characteristics**:
- Same underlying model, different endpoint/naming convention
- Example: `gemini-1.5-pro` vs. `models/gemini-1.5-pro`
- No new features or capabilities
- Simple aliasing in model_mapping.rs required

**Evidence Indicators**:
- Model responses identical to known models
- Same rate limits and quota consumption
- No unique features detected in API testing
- Google documentation mentions "alias" or "variant"

**Implementation Effort**: **1 week**
- 3 days: Add model mapping aliases
  ```rust
  // model_mapping.rs
  pub fn normalize_model_name(model: &str) -> &str {
      match model {
          "gemini-2.0-flash-thinking-exp-01-21" => "gemini-2.0-flash-thinking-exp",
          "models/gemini-314" => "gemini-1.5-pro-002",
          // ... additional aliases
          _ => model,
      }
  }
  ```
- 2 days: Update MASTER-MODELS-TABLE.md with alias references
- 0 days: No COMPARISON files needed (reference existing docs)

**ROI**: **VERY HIGH** (minimal effort, completes documentation)

**Recommendation**: ✅ **IMPLEMENT IMMEDIATELY**

**Stories**:
- Story-020-01: Add model ID aliases to model_mapping.rs (3 days)
- Story-020-02: Update MASTER-MODELS-TABLE.md with alias references (2 days)

---

### Scenario B: New Unique Features (High Effort)

**Hypothesis**: Models have unique capabilities not in existing documentation

**Characteristics**:
- New thinking modes (extended reasoning, chain-of-thought)
- Enhanced vision capabilities (OCR, video analysis)
- Advanced audio features (real-time transcription)
- Unique tool calling patterns
- Performance advantages (faster, cheaper)

**Evidence Indicators**:
- API responses differ significantly from known models
- New API parameters or response fields detected
- Unique error codes or rate limit behavior
- Google documentation highlights new features
- User demand for specific models

**Implementation Effort**: **2-3 weeks per model family**
- 1 week: Feature detection and mapper implementation
  - Update request mappers for new parameters
  - Update response mappers for new fields
  - Add feature flag detection
  - Implement rate limit handling
- 1 week: COMPARISON file creation (5-7 files)
  - Comprehensive capability testing
  - Use case documentation
  - Performance benchmarking
  - Limitation documentation
- 3-5 days: Integration testing and validation
  - End-to-end testing with real workloads
  - Regression testing with existing models
  - Documentation accuracy validation

**ROI**: **VARIABLE** (depends on user demand and feature uniqueness)

**Recommendation**: 🎯 **PRIORITIZE BY BUSINESS VALUE**

**Decision Criteria**:
| Criterion | Weight | Measurement |
|-----------|--------|-------------|
| User Demand | 40% | GitHub issues, forum requests |
| Feature Uniqueness | 30% | No overlap with existing models |
| Performance Advantage | 20% | Speed/cost improvements >20% |
| Documentation Value | 10% | Fills critical knowledge gaps |

**Stories** (per high-value model):
- Story-020-0X: Implement model-specific feature detection (5 days)
- Story-020-0Y: Create COMPARISON-MODEL-XXX.md (3 days)
- Story-020-0Z: Integration testing and validation (2 days)

---

### Scenario C: Deprecated Models (Minimal Effort)

**Hypothesis**: Models are deprecated or removed from Google's API

**Characteristics**:
- API returns 404 (model not found) or 410 (gone)
- Google documentation marks as "deprecated" or "legacy"
- No active user demand or support requests
- Models were experimental or preview-only
- Replaced by newer model versions

**Evidence Indicators**:
- API error: "Model not found" or "Model deprecated"
- Google Cloud release notes mention deprecation
- Zero usage in application logs (past 90 days)
- No GitHub issues or forum questions
- Documentation states "use [newer model] instead"

**Implementation Effort**: **3 days**
- 2 days: Document deprecation status
  - Add deprecation notices to MASTER-MODELS-TABLE.md
  - Create brief deprecation notes in README
  - Link to replacement models
- 1 day: Migration guide for users
  - Document model equivalents
  - Provide migration examples
  - Update error messages for deprecated models

**ROI**: **MODERATE** (minimal effort, completes documentation, prevents confusion)

**Recommendation**: 📝 **DOCUMENT & SKIP** implementation

**Stories**:
- Story-020-01: Document deprecated model IDs in MASTER-MODELS-TABLE.md (2 days)
- Story-020-02: Create migration guide for deprecated models (1 day)

---

### Scenario D: Mixed (Variable Effort)

**Hypothesis**: Models fall into multiple categories (A, B, C)

**Characteristics**:
- Some models are simple aliases (Scenario A)
- Some models have unique features (Scenario B)
- Some models are deprecated (Scenario C)
- Requires case-by-case analysis

**Example Distribution**:
```
Model 314: Alias of gemini-1.5-pro-002 → Scenario A (1 day)
Model 315: New thinking mode variant → Scenario B (2 weeks)
Model 316: Deprecated experimental → Scenario C (0.5 days)
Model 317: Alias of gemini-2.0-flash → Scenario A (1 day)
Model 318: Unique vision model → Scenario B (2 weeks)
...
Total: ~14 models × mixed effort = 1-4 weeks
```

**Implementation Effort**: **1-4 weeks** (depends on distribution)
- Aliases: 1 week total (all aliases together)
- Unique models: 2-3 weeks each (prioritized)
- Deprecated: 3 days total (batch documentation)

**ROI**: **VARIABLE** (cherry-pick high-value models)

**Recommendation**: 🎯 **CHERRY-PICK** high-value models, document rest

**Prioritization Matrix**:
| Model | Category | Effort | User Demand | Priority | Action |
|-------|----------|--------|-------------|----------|--------|
| 314 | Alias | Low | High | P0 | Implement Week 1 |
| 315 | Unique | High | High | P1 | Implement Q2 |
| 316 | Deprecated | Min | Low | P3 | Document only |
| 317 | Alias | Low | Medium | P1 | Implement Week 1 |
| 318 | Unique | High | Medium | P2 | Implement Q3 |
| ... | ... | ... | ... | ... | ... |

**Stories** (phased approach):
- **Phase 1 (Week 1)**: Implement all P0 aliases (1 week)
  - Story-020-01: Add P0 model aliases
  - Story-020-02: Update documentation for aliases
- **Phase 2 (Q2 2026)**: Implement P1 unique models (2-3 weeks)
  - Story-020-03: Implement model 315 unique features
  - Story-020-04: Create COMPARISON files for P1 models
- **Phase 3 (Q3 2026)**: Implement P2 unique models (2-3 weeks)
  - Story-020-05: Implement model 318 unique features
  - Story-020-06: Create COMPARISON files for P2 models
- **Phase 4 (Any time)**: Document P3 deprecated models (3 days)
  - Story-020-07: Document all deprecated models

---

## Business Case

### Documentation Completeness Impact

**Current State**:
- 72.2% documentation coverage
- Professional-grade tools typically achieve 95%+ coverage
- User complaints about missing model information
- Competitive disadvantage in AI tool market

**Target State**:
- 98%+ documentation coverage
- Comprehensive model catalog
- Reduced support burden
- Competitive parity or advantage

**Value Proposition**:
1. **User Confidence**: Complete catalog builds trust
2. **Optimal Model Selection**: Users choose best models for tasks
3. **Reduced Support**: Fewer "which model?" questions
4. **Competitive Edge**: Superior documentation vs. alternatives
5. **Professional Image**: Completeness signals quality

### Cost-Benefit Analysis

**Research Cost** (Week 1):
- Tech Lead: 20 hours × $150/hour = $3,000
- Developer: 40 hours × $100/hour = $4,000
- **Total**: $7,000

**Implementation Cost** (Scenario-dependent):
- **Scenario A**: 40 hours × $100/hour = $4,000
- **Scenario B**: 120 hours × $100/hour × N models = $12,000-$36,000
- **Scenario C**: 24 hours × $100/hour = $2,400
- **Scenario D**: Variable ($4,000-$40,000)

**Estimated Benefit**:
- Reduced support tickets: ~10 hours/month × $100/hour = $1,200/month
- User retention improvement: 2% (est. $5,000/month value)
- Competitive positioning: Intangible (high value)
- **Annual Benefit**: ~$75,000

**ROI**:
- **Best Case** (Scenario A): $4,000 cost → $75,000 benefit = **1,775% ROI**
- **Worst Case** (Scenario B, all models): $40,000 cost → $75,000 benefit = **88% ROI**
- **Most Likely** (Scenario D, cherry-picked): $15,000 cost → $75,000 benefit = **400% ROI**

**Payback Period**: 2-6 months

---

## Risk Assessment

### High-Risk Factors

**1. Uncertain Model Status (Probability: 60%)**
- **Risk**: Many models may be deprecated or experimental
- **Impact**: HIGH (wasted effort on obsolete models)
- **Mitigation**: Research phase validates status before implementation
- **Contingency**: Document deprecated models, skip implementation

**2. Insufficient Google Documentation (Probability: 70%)**
- **Risk**: Google provides minimal documentation for some models
- **Impact**: HIGH (difficult to create accurate COMPARISON files)
- **Mitigation**: Reverse engineering via API testing, community resources
- **Contingency**: Best-effort documentation with caveats

**3. API Changes During Research (Probability: 20%)**
- **Risk**: Google deprecates/changes models during investigation
- **Impact**: MEDIUM (requires re-testing, updated docs)
- **Mitigation**: Daily API validation, snapshot testing
- **Contingency**: Update documentation with change history

### Medium-Risk Factors

**4. Resource Contention (Probability: 30%)**
- **Risk**: Developer from Team 1 diverted from Epic-014
- **Impact**: MEDIUM (Epic-014 may slow slightly)
- **Mitigation**: Research runs in parallel, non-blocking
- **Contingency**: Extend Epic-014 by 1-2 days if needed

**5. Low Business Value (Probability: 40%)**
- **Risk**: Models don't add significant value for users
- **Impact**: MEDIUM (implementation may not be justified)
- **Mitigation**: ROI analysis, user demand assessment
- **Contingency**: No-Go decision, document findings only

**6. Implementation Complexity (Probability: 50%)**
- **Risk**: Unique models require extensive mapper changes
- **Impact**: MEDIUM (effort exceeds estimates)
- **Mitigation**: Phased implementation, prioritize by value
- **Contingency**: Defer low-value models to backlog

### Low-Risk Factors

**7. Testing Infrastructure Gaps (Probability: 20%)**
- **Risk**: Insufficient test coverage for new models
- **Impact**: LOW (existing test harness is comprehensive)
- **Mitigation**: Extend test harness during research
- **Contingency**: Manual testing fallback

---

## Success Metrics

### Research Phase Metrics

**Completeness**:
- [ ] 100% of model IDs 314-327 identified or confirmed non-existent
- [ ] 100% of additional gaps identified
- [ ] ≥90% confidence in model identification

**Accuracy**:
- [ ] All active models tested via live API
- [ ] Capability matrix validated with test evidence
- [ ] Performance benchmarks collected (p50, p95, p99)

**Documentation Quality**:
- [ ] COMPARISON files follow established template
- [ ] Test evidence included for all claims
- [ ] Cross-references to MASTER-MODELS-TABLE.md accurate

**Decision Quality**:
- [ ] Scenario classification clear and justified
- [ ] Effort estimates based on evidence
- [ ] ROI analysis supports recommendation
- [ ] Stakeholder buy-in achieved

### Implementation Phase Metrics (Conditional)

**Coverage**:
- [ ] Documentation coverage ≥98% (54/55 models)
- [ ] All active models have COMPARISON files
- [ ] Deprecated models documented with migration guides

**Quality**:
- [ ] All COMPARISON files pass quality review
- [ ] Test coverage ≥80% for new model features
- [ ] Zero regressions in existing model handling

**Performance**:
- [ ] Model selection latency <50ms
- [ ] API request success rate ≥99.5%
- [ ] Rate limit handling functional for all models

---

## Timeline & Milestones

### Week 1: Research Phase (2026-01-22 to 2026-01-26)

**Day 1 (2026-01-22)**: Code Analysis
- [x] Milestone: Model ID hypothesis list

**Day 2 (2026-01-23)**: Model Identification
- [x] Milestone: Model identification matrix with confidence scores

**Day 3 (2026-01-24)**: Live API Validation
- [x] Milestone: Capability matrix with test results

**Day 4 (2026-01-25)**: Documentation
- [x] Milestone: Draft COMPARISON files (conditional)

**Day 5 (2026-01-26)**: Decision
- [x] Milestone: Implementation recommendation report
- [x] Milestone: Go/No-Go decision with stakeholder approval

### Post-Research: Implementation (Conditional)

**Scenario A (1 week)**: 2026-01-29 to 2026-02-05
**Scenario B (2-3 weeks per model)**: Q2/Q3 2026 (staggered)
**Scenario C (3 days)**: 2026-01-29 to 2026-01-31
**Scenario D (1-4 weeks)**: Phased approach based on priority

---

## Stakeholder Communication

### Research Phase Updates

**Daily Standups** (15 minutes):
- Progress against daily milestones
- Blockers and risks
- Preliminary findings

**Mid-Week Review** (Day 3):
- Scenario likelihood assessment
- Early ROI indicators
- Adjust research focus if needed

**End-of-Week Presentation** (Day 5):
- Comprehensive research findings
- Go/No-Go recommendation
- Implementation roadmap (if applicable)
- Q&A session

### Decision Framework Presentation

**Content**:
1. Research methodology and findings
2. Model categorization (Scenarios A/B/C/D)
3. Effort estimates and ROI analysis
4. Risk assessment and mitigation
5. Recommendation with justification
6. Implementation roadmap (if Go decision)

**Attendees**:
- Product Owner
- Tech Lead
- Team Leads (Team 1, Team 2)
- Stakeholders (optional)

**Outcome**: Approved Go/No-Go decision

---

## Next Steps

### If Go Decision (Implementation Approved)

1. **Create Implementation Epic** (EPIC-020-IMPLEMENTATION.md)
2. **Break Down Stories** (based on scenario)
3. **Assign Teams** (prioritized by value)
4. **Schedule Sprints** (Q2/Q3 2026)
5. **Track Progress** (Jira/GitHub/Status docs)

### If No-Go Decision (Document Only)

1. **Finalize Research Report** (archive findings)
2. **Update MASTER-MODELS-TABLE.md** (note deprecated models)
3. **Close Epic** (document decision rationale)
4. **Monitor User Demand** (revisit if demand increases)

### If Partial Go (Cherry-Pick)

1. **Prioritize Models** (ROI matrix)
2. **Phase Implementation** (P0 → P1 → P2)
3. **Track Value Realization** (measure benefits)
4. **Iterate** (adjust priorities based on feedback)

---

**Last Updated**: 2026-01-12
**Next Review**: 2026-01-26 (End of Research Phase)
**Status**: READY FOR RESEARCH EXECUTION
