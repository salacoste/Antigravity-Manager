# EPIC-020 Research Protocol: Model IDs 314-327 Investigation

**Status**: READY FOR RESEARCH
**Phase**: Discovery & Analysis (Week 1)
**Start Date**: 2026-01-22
**Duration**: 5 days (parallel with Epic-014)
**Team**: Tech Lead + 1 Developer (Team 1)
**Decision Point**: End of Week 1

---

## Research Phase Overview

### Objective
Conduct systematic investigation of 25 unknown model IDs (314-327 + additional gaps) to determine:
1. Model identification and capabilities
2. Documentation requirements (COMPARISON files)
3. Implementation effort estimation
4. Go/No-Go recommendation for Q2/Q3 implementation

### Scope
- **Primary**: Model IDs 314-327 (14 models)
- **Secondary**: Additional gaps identified in model_mapping.rs (~11 models)
- **Total**: ~25 unknown models requiring investigation
- **Current Coverage**: 72.2% (39/54+ documented models)
- **Target Coverage**: 98%+ (post-implementation)

### Research Approach
```
Discovery → Analysis → Testing → Documentation → Decision
```

**Phase 1**: Code analysis and pattern recognition
**Phase 2**: Live API validation and capability testing
**Phase 3**: Analysis, recommendations, and decision framework

---

## Week 1 Research Plan

### Daily Breakdown

#### **Day 1 (2026-01-22): Code Analysis & Pattern Recognition**

**Morning Session (4 hours)**:
- [ ] Scan `src-tauri/src/proxy/mappers/gemini/model_mapping.rs` for references
- [ ] Search codebase for model ID patterns (314-327)
- [ ] Check `upstream/client.rs` for model routing logic
- [ ] Analyze request/response mappers for model-specific handling

**Afternoon Session (4 hours)**:
- [ ] Review application logs for model ID occurrences
- [ ] Check API error logs for unhandled model references
- [ ] Document code-based evidence for each model ID
- [ ] Create initial model ID tracking matrix

**Deliverable**: Code analysis report with initial hypotheses

---

#### **Day 2 (2026-01-23): Model Identification & Reverse Engineering**

**Morning Session (4 hours)**:
- [ ] Reverse engineer model IDs from API request patterns
- [ ] Cross-reference with Google AI Studio model list
- [ ] Check Vertex AI API documentation for model variants
- [ ] Search Google Cloud release notes (2024-2025)

**Afternoon Session (4 hours)**:
- [ ] Test model ID resolution via upstream API
- [ ] Document API response patterns for each ID
- [ ] Identify model aliases and variants
- [ ] Categorize models by type (standard/vision/thinking/audio)

**Deliverable**: Model identification matrix with confidence scores

---

#### **Day 3 (2026-01-24): Live API Validation & Capability Testing**

**Morning Session (4 hours)**:
- [ ] Set up test harness for API validation
- [ ] Test each model ID (314-327) via live API
- [ ] Document supported features per model:
  - Standard text generation
  - System instructions
  - Tool/function calling
  - Vision capabilities
  - Thinking mode support
  - Audio input/output
  - Multimodal support

**Afternoon Session (4 hours)**:
- [ ] Benchmark response quality and speed
- [ ] Test rate limits and quota consumption
- [ ] Compare capabilities with existing documented models
- [ ] Identify unique features vs. aliases

**Deliverable**: Capability matrix with test results

---

#### **Day 4 (2026-01-25): COMPARISON Files & Documentation**

**Morning Session (4 hours)**:
- [ ] Determine which models warrant COMPARISON files
- [ ] Create COMPARISON files for high-value models
- [ ] Document model-specific features and limitations
- [ ] Cross-reference with MASTER-MODELS-TABLE.md

**Afternoon Session (4 hours)**:
- [ ] Validate COMPARISON file accuracy via API testing
- [ ] Document deprecation status for obsolete models
- [ ] Create model migration recommendations
- [ ] Update capability matrices with test evidence

**Deliverable**: Draft COMPARISON files (up to 7 new files)

---

#### **Day 5 (2026-01-26): Analysis & Go/No-Go Decision**

**Morning Session (4 hours)**:
- [ ] Synthesize research findings
- [ ] Size implementation effort for each scenario
- [ ] Calculate ROI for documentation completion
- [ ] Prepare decision framework presentation

**Afternoon Session (4 hours)**:
- [ ] Create implementation recommendations
- [ ] Document risk assessment and mitigation
- [ ] Prepare Epic-020 implementation stories (conditional)
- [ ] Present findings and recommendation to stakeholders

**Deliverable**: Research report with Go/No-Go recommendation

---

## Research Methodology

### 1. Code Analysis Protocol

**Tools & Techniques**:
```bash
# Search for model ID references
grep -r "gemini-2.0-flash-thinking" src-tauri/
rg "31[4-7]|32[0-7]" src-tauri/src/proxy/

# Check model mapping patterns
cat src-tauri/src/proxy/mappers/gemini/model_mapping.rs

# Analyze upstream client routing
cat src-tauri/src/proxy/upstream/client.rs
```

**Analysis Checklist**:
- [ ] Model ID enum definitions
- [ ] Routing logic for model selection
- [ ] Feature flag detection (thinking mode, tools, vision)
- [ ] Error handling for unknown models
- [ ] API version compatibility

---

### 2. Live API Validation Protocol

**Test Harness Setup**:
```rust
// Test each model ID
let test_models = vec![
    "gemini-2.0-flash-thinking-exp-01-21",
    "models/gemini-2.0-flash-thinking-exp-1219",
    // ... models 314-327
];

for model_id in test_models {
    test_model_capabilities(model_id).await;
}
```

**Capability Testing Matrix**:
| Model ID | Text | Vision | Tools | Thinking | Audio | Status |
|----------|------|--------|-------|----------|-------|--------|
| 314 | ? | ? | ? | ? | ? | Unknown |
| 315 | ? | ? | ? | ? | ? | Unknown |
| ... | ... | ... | ... | ... | ... | ... |

**Testing Checklist** (per model):
- [ ] Basic text generation (100-word prompt)
- [ ] System instructions support
- [ ] Tool/function calling (simple calculator)
- [ ] Vision input (image analysis)
- [ ] Thinking mode activation
- [ ] Audio input/output
- [ ] Rate limit behavior
- [ ] Error responses
- [ ] Response time benchmarks

---

### 3. Documentation Standards

**COMPARISON File Creation Criteria**:
- Model has unique capabilities vs. existing models
- Model is actively supported (not deprecated)
- Model differs significantly from aliases
- Documentation adds value for users

**Template Structure**:
```markdown
# Model Name Comparison

## Model Identification
- Model ID: gemini-xxx
- Internal ID: 314
- Status: Active/Deprecated

## Capabilities
- Text generation: Yes/No
- Vision: Yes/No
- Tools: Yes/No
- Thinking: Yes/No
- Audio: Yes/No

## Use Cases
...

## Limitations
...
```

---

## Decision Framework (4 Scenarios)

### Scenario A: Vertex AI Variants (Low Effort)

**Discovery**: Models 314-327 are aliases/variants of existing models
**Characteristics**:
- Same underlying model, different endpoint/naming
- No new features detected
- Simple aliasing required

**Implementation Estimate**: 1 week
- 3 days: Add model mapping aliases
- 2 days: Documentation updates

**Recommendation**: ✅ **IMPLEMENT** (Quick win, high ROI)

**Stories**:
- Story-020-01: Add model ID aliases to model_mapping.rs
- Story-020-02: Update MASTER-MODELS-TABLE.md with aliases

---

### Scenario B: New Unique Features (High Effort)

**Discovery**: Models have unique capabilities not in existing documentation
**Characteristics**:
- New thinking modes, vision capabilities, or audio features
- Requires new mappers or feature detection
- High documentation value

**Implementation Estimate**: 2-3 weeks per model family
- 1 week: Feature detection and mapper implementation
- 1 week: COMPARISON file creation (5-7 files)
- 3-5 days: Integration testing and validation

**Recommendation**: 🎯 **PRIORITIZE** by business value

**Decision Criteria**:
- User demand for model
- Feature uniqueness
- Performance advantages
- Documentation completeness impact

**Stories** (per high-value model):
- Story-020-0X: Implement model-specific feature detection
- Story-020-0Y: Create COMPARISON-MODEL-XXX.md
- Story-020-0Z: Integration testing and validation

---

### Scenario C: Deprecated Models (Minimal Effort)

**Discovery**: Models are deprecated or removed from API
**Characteristics**:
- API returns 404 or deprecation warnings
- No active user demand
- Historical reference only

**Implementation Estimate**: 3 days
- 2 days: Document deprecation status
- 1 day: Add migration recommendations

**Recommendation**: 📝 **DOCUMENT & SKIP** implementation

**Stories**:
- Story-020-01: Document deprecated model IDs
- Story-020-02: Add migration guide for users

---

### Scenario D: Mixed (Variable Effort)

**Discovery**: Combination of scenarios A, B, and C
**Characteristics**:
- Some aliases (low effort)
- Some unique features (high effort)
- Some deprecated (minimal effort)

**Implementation Estimate**: 1-4 weeks (depends on mix)
- Aliases: 1 week total
- Unique models: 2-3 weeks per model
- Deprecated: 3 days total

**Recommendation**: 🎯 **CHERRY-PICK** high-value models

**Prioritization Matrix**:
| Model | Type | Effort | Value | Priority |
|-------|------|--------|-------|----------|
| 314 | Alias | Low | High | P0 |
| 315 | Unique | High | High | P1 |
| 316 | Deprecated | Min | Low | P3 |
| ... | ... | ... | ... | ... |

**Stories** (prioritized):
- Story-020-01: Implement P0 aliases (1 week)
- Story-020-02: Implement P1 unique models (2-3 weeks)
- Story-020-03: Document P3 deprecated models (3 days)

---

## Research Deliverables

### Primary Outputs

1. **Model Identification Report** (`docs/research/MODEL-IDS-314-327-FINDINGS.md`)
   - Complete list of identified models
   - Confidence scores per model
   - Evidence supporting identification

2. **Capability Matrix** (Spreadsheet or Markdown table)
   - Feature support per model
   - Performance benchmarks
   - Comparison with existing models

3. **COMPARISON Files** (Conditional, up to 7 files)
   - `docs/comparison/COMPARISON-GEMINI-2.0-FLASH-THINKING-EXP-01-21.md`
   - `docs/comparison/COMPARISON-GEMINI-[MODEL-NAME].md`
   - Only for high-value, unique models

4. **Implementation Recommendation** (`docs/epics/EPIC-020-IMPLEMENTATION-RECOMMENDATION.md`)
   - Scenario classification (A/B/C/D)
   - Effort estimation per scenario
   - ROI analysis
   - Go/No-Go decision with justification

### Secondary Outputs

5. **Epic-020 Implementation Stories** (Conditional)
   - Story breakdown based on scenario
   - Effort estimates per story
   - Dependency mapping

6. **Risk Assessment Document**
   - Technical risks (API changes, deprecation)
   - Resource risks (developer availability)
   - Business risks (low user demand)

---

## Success Criteria

### Research Phase Success

- [x] **Completeness**: All 25 model IDs identified or confirmed deprecated
- [x] **Accuracy**: Capability matrix validated via live API testing
- [x] **Documentation**: COMPARISON files created for valuable models
- [x] **Decision**: Clear Go/No-Go recommendation with evidence
- [x] **Sizing**: Implementation effort estimated per scenario
- [x] **Timeline**: Research completed within 5-day sprint

### Quality Gates

1. **Identification Confidence**: ≥90% confidence per model ID
2. **Test Coverage**: All active models tested via live API
3. **Documentation Quality**: COMPARISON files follow established template
4. **Decision Justification**: ROI analysis supports recommendation
5. **Stakeholder Buy-in**: Research findings presented and approved

---

## Risk Assessment

### High-Risk Factors

**1. API Instability**
- **Risk**: Model IDs change during research week
- **Mitigation**: Daily API validation, snapshot testing
- **Impact**: Medium (requires re-testing)

**2. Insufficient Documentation**
- **Risk**: Google provides minimal model documentation
- **Mitigation**: Reverse engineering via API testing
- **Impact**: High (extends research timeline)

**3. Deprecated Models**
- **Risk**: Many models are obsolete (Scenario C)
- **Mitigation**: Document deprecation, focus on active models
- **Impact**: Low (minimal implementation required)

### Medium-Risk Factors

**4. Resource Contention**
- **Risk**: 1 developer from Team 1 diverted from Epic-014
- **Mitigation**: Research runs in parallel (non-blocking)
- **Impact**: Low (Epic-014 has 2 other developers)

**5. Unclear Business Value**
- **Risk**: Models don't add significant value
- **Mitigation**: ROI analysis, user demand assessment
- **Impact**: Medium (may result in No-Go decision)

---

## Next Steps Based on Findings

### If Scenario A (Vertex Variants) → IMPLEMENT

**Timeline**: 1 week (2026-01-29 to 2026-02-05)
**Effort**: 40 developer-hours
**Team**: Same research team continues

**Stories**:
1. Story-020-01: Add model aliases (3 days)
2. Story-020-02: Update documentation (2 days)

**Success**: Documentation coverage → 98%+

---

### If Scenario B (New Features) → PRIORITIZE

**Timeline**: 2-3 weeks per model (staggered Q2/Q3)
**Effort**: 80-120 hours per model
**Team**: Assign based on priority

**Phased Approach**:
- **Phase 1**: P0 models (highest value) - Q2 2026
- **Phase 2**: P1 models (medium value) - Q3 2026
- **Phase 3**: P2 models (low value) - Backlog

**Stories**: Created per model based on priority matrix

---

### If Scenario C (Deprecated) → DOCUMENT & SKIP

**Timeline**: 3 days (2026-01-29 to 2026-01-31)
**Effort**: 24 developer-hours
**Team**: Tech Lead only

**Stories**:
1. Story-020-01: Document deprecated models (2 days)
2. Story-020-02: Create migration guide (1 day)

**Outcome**: Documentation complete, no implementation needed

---

### If Scenario D (Mixed) → CHERRY-PICK

**Timeline**: Variable (1-4 weeks based on selection)
**Effort**: 40-160 hours
**Team**: Flexible assignment

**Decision Process**:
1. Prioritize by ROI matrix
2. Implement aliases first (quick wins)
3. Queue unique models by priority
4. Document deprecated models

**Stories**: Hybrid approach combining scenarios A, B, C

---

## Research Team Coordination

### Team Composition

**Tech Lead** (50% allocation):
- Research strategy and methodology
- Code analysis and pattern recognition
- Decision framework and recommendations
- Stakeholder presentation

**Developer** (Team 1, 100% allocation):
- Live API testing and validation
- Capability matrix creation
- COMPARISON file authoring
- Test harness implementation

### Communication Plan

**Daily Standups**: 15 minutes
- Progress update
- Blockers identification
- Next-day planning

**Mid-Week Review** (Day 3, 2026-01-24):
- Preliminary findings
- Scenario likelihood assessment
- Adjust research focus if needed

**End-of-Week Presentation** (Day 5, 2026-01-26):
- Research findings
- Go/No-Go recommendation
- Implementation roadmap (if applicable)
- Stakeholder Q&A

### Collaboration with Epic-014

**Non-Blocking Approach**:
- Epic-020 research runs in parallel
- No shared code conflicts (different modules)
- Developer from Team 1 (Epic-014 has 2 others)
- Research can inform Epic-014 model selection

**Synergies**:
- Both epics focus on Gemini model improvements
- Shared learning on model capabilities
- Potential for unified testing infrastructure

---

## Documentation Standards

### File Naming Conventions

**Research Files**:
- `MODEL-IDS-314-327-FINDINGS.md`
- `MODEL-IDS-CAPABILITY-MATRIX.md`
- `EPIC-020-IMPLEMENTATION-RECOMMENDATION.md`

**COMPARISON Files**:
- `COMPARISON-GEMINI-2.0-FLASH-THINKING-EXP-01-21.md`
- `COMPARISON-GEMINI-[MODEL-NAME].md`

### Version Control

**Branch Strategy**:
```bash
# Research branch
git checkout -b research/epic-020-model-ids-investigation

# Daily commits
git commit -m "docs(epic-020): Day 1 - Code analysis findings"
git commit -m "docs(epic-020): Day 2 - Model identification matrix"
# ...
```

**PR at End of Week**:
- Title: `docs: Epic-020 Research Phase - Model IDs 314-327 Investigation`
- Description: Research findings, recommendations, decision
- Reviewers: Product Owner, Tech Lead, Stakeholders

---

## Appendix

### Reference Materials

**Code Locations**:
- Model mapping: `src-tauri/src/proxy/mappers/gemini/model_mapping.rs`
- Upstream client: `src-tauri/src/proxy/upstream/client.rs`
- Request mappers: `src-tauri/src/proxy/mappers/gemini/request.rs`
- Response mappers: `src-tauri/src/proxy/mappers/gemini/response.rs`

**Documentation References**:
- Master table: `docs/comparison/MASTER-MODELS-TABLE.md`
- Existing COMPARISON files: `docs/comparison/COMPARISON-*.md`
- Epic templates: `docs/epics/EPIC-*-TEMPLATE.md`

**External Resources**:
- Google AI Studio: https://aistudio.google.com
- Vertex AI Docs: https://cloud.google.com/vertex-ai/docs
- Gemini API Reference: https://ai.google.dev/api

### Research Tools

**API Testing**:
- Postman/Insomnia collections
- Custom Rust test harness
- curl scripts for quick validation

**Data Collection**:
- Spreadsheet for capability matrix
- Markdown templates for documentation
- JSON files for test results

---

**Last Updated**: 2026-01-12
**Next Review**: End of Day 5 (2026-01-26)
**Status**: READY FOR EXECUTION
