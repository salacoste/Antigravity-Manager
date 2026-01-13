# Model ID 246 Architecture Clarification

**Document Type**: Technical Clarification
**Date**: 2026-01-26
**Epic**: Epic-020 Model IDs Investigation
**Status**: REFERENCE DOCUMENT

---

## Executive Summary

**Model ID 246** belongs to:

✅ **CORRECT**: `gemini-2.5-pro` (BASE model, WITHOUT thinking mode)

**Key Architecture Point**: Thinking mode is activated via API parameter `thinkingBudget`, NOT a separate Model ID.

---

## Technical Architecture

### Model ID 246 - Single ID, Dual Mode

```yaml
model_id: 246
model_name: "gemini-2.5-pro"  # ALWAYS this name

# Base usage (WITHOUT thinking)
generationConfig: {}

# With thinking mode (SAME Model ID 246!)
generationConfig:
  thinkingConfig:
    thinkingBudget: 16000  # Parameter, NOT separate Model ID
```

---

## Why Confusion Occurred

### Epic-008 Statement (lines 4-5):
```markdown
Model: gemini-2.5-pro-thinking
Model ID: 246
```

**This is TECHNICALLY INACCURATE** because:
- Model ID 246 = `gemini-2.5-pro` (base model)
- "Thinking" is enabled via API parameter, not a separate Model ID

### Correct Understanding:
- **Model Name**: `gemini-2.5-pro` (always)
- **Model ID**: 246 (always)
- **Thinking Mode**: Activated via `thinkingConfig.thinkingBudget` parameter
- **Documentation Name**: `gemini-2.5-pro-thinking` (workflow/COMPARISON convenience name)

---

## Architecture Comparison Across Models

| Model Family | Base ID | Thinking ID | Architecture |
|--------------|---------|-------------|--------------|
| **Gemini 2.5 Pro** | **246** | **246** | **Parameter-based** (ONE ID) |
| Gemini 2.5 Flash | 312 | 313 | Separate IDs (TWO IDs) |
| Claude Sonnet 4.5 | 333 | 334 | Separate IDs (TWO IDs) |
| Gemini 3.x | 0 | 0 | Name-based routing (ONE ID) |

### Key Differences:

**Gemini 2.5 Pro Architecture** (Parameter-Based):
- ✅ ONE Model ID (246) for both base and thinking
- ✅ Thinking activated via `thinkingBudget` parameter
- ✅ Similar to Gemini 3.x architecture (parameter-based, not ID-based)

**Gemini 2.5 Flash Architecture** (ID-Based):
- ✅ TWO separate Model IDs (312 base, 313 thinking)
- ✅ Each mode has dedicated ID
- ✅ Similar to Claude architecture

**Why This Matters**:
- Affects how we document models
- Affects how we understand Model ID allocation
- Affects Epic-020 pivot opportunity analysis

---

## Documentation Implications

### Current Documentation State:

**MASTER-MODELS-TABLE.md** (Section 1.2 & 1.3):

**Section 1.2**: Gemini 2.5 Production
```markdown
| 5 | gemini-2.5-pro | - | ✅ | ❌ | ❌ | DONE | - | v1.0, no thinking |
```

**Section 1.3**: Gemini 2.5 Thinking Variants
```markdown
| 1 | gemini-2.5-pro-thinking | - | ❌ N/A | ✅ | ✅ 33KB | ✅ COMPLETE | - | ✅ Epic-015 |
```

**Architecture Reality**:
- Row 5 (`gemini-2.5-pro`): Model ID 246, base mode (no thinkingBudget)
- Row 114 (`gemini-2.5-pro-thinking`): SAME Model ID 246, thinking mode (with thinkingBudget)

**Documentation Convention**: We separate them for clarity (base workflow vs. thinking workflow), but they share the SAME Model ID.

---

## Epic-020 Pivot Opportunity Correction

### Original Analysis (Epic-020 Day 2):
```yaml
gemini_2_5_models_to_document:
  - 246: gemini-2.5-pro-thinking
  - 312: gemini-2.5-flash (base)
  - 313: gemini-2.5-flash-thinking

coverage_improvement: +5.5% (3 models)
```

### Corrected Analysis (Epic-020 Day 3):
```yaml
model_id_246:
  model_name: "gemini-2.5-pro"  # BASE model
  base_workflow: ✅ DOCUMENTED (gemini-2.5-pro-workflow.md)
  thinking_workflow: ✅ DOCUMENTED (gemini-2.5-pro-thinking-workflow.md)
  comparison: ✅ DOCUMENTED (gemini-2.5-pro-thinking-COMPARISON.md, Epic-015)
  architecture: "Parameter-based (ONE Model ID for both modes)"
  status: "FULLY DOCUMENTED ✅"

model_id_312:
  model_name: "gemini-2.5-flash"  # BASE model (likely)
  base_workflow: ✅ DOCUMENTED (gemini-2.5-flash-workflow.md)
  comparison: ❌ NOT DOCUMENTED
  status: "Base documented, COMPARISON missing"

model_id_313:
  model_name: "gemini-2.5-flash-thinking"
  thinking_workflow: ✅ DOCUMENTED (gemini-2.5-flash-thinking-workflow.md)
  comparison: ❌ NOT DOCUMENTED
  status: "Thinking documented, COMPARISON missing"

pivot_opportunity:
  models_needing_comparison: 2 (312 base, 313 thinking)
  coverage_improvement: "+~2% (not +5.5%)"
  effort: "8-12 hours (2 COMPARISON files)"
  roi: "MEDIUM (not HIGH as initially estimated)"
```

### Conclusion:
- Model ID 246 is FULLY documented (both modes) ✅
- Model IDs 312/313 have workflows but lack COMPARISON files
- Pivot opportunity smaller than expected (2 COMPARISOs vs. 3 models)

---

## Recommended Actions

### 1. Epic-008 Clarification Note

**File**: `docs/epics/Epic-008-Gemini-2.5-Pro-Thinking-Optimization.md`

**Add after line 5**:
```markdown
**Model ID**: 246

**⚠️ IMPORTANT ARCHITECTURE NOTE**:
Model ID 246 refers to the BASE model `gemini-2.5-pro`. Thinking mode is activated
via the API parameter `thinkingConfig.thinkingBudget`, NOT a separate Model ID.
This is a parameter-based architecture, unlike Gemini 2.5 Flash (which uses
separate IDs 312 for base and 313 for thinking).

When this document refers to "gemini-2.5-pro-thinking", it means the base model
(ID 246) WITH the `thinkingBudget` parameter configured.
```

### 2. MASTER-MODELS-TABLE.md Annotation

**Add footnote to Section 1.2/1.3**:
```markdown
**Model ID Architecture Note**:
- Gemini 2.5 Pro: Uses ONE Model ID (246) for both base and thinking modes
  - Base mode: `gemini-2.5-pro` (no thinkingBudget parameter)
  - Thinking mode: `gemini-2.5-pro-thinking` (with thinkingBudget parameter)
- Gemini 2.5 Flash: Uses TWO separate Model IDs (312 base, 313 thinking)
```

### 3. MODEL-ID-ALLOCATION-REFERENCE.md Update

**Add to Active Model IDs section**:
```markdown
### Parameter-Based vs. ID-Based Architecture

**Parameter-Based Models** (ONE Model ID):
- Model ID 246 (`gemini-2.5-pro`): Base + Thinking (via thinkingBudget)
- Model ID 0 (Gemini 3.x): All modes (via thinkingLevel parameter)

**ID-Based Models** (SEPARATE Model IDs):
- Model IDs 312/313 (`gemini-2.5-flash`): 312 = Base, 313 = Thinking
- Model IDs 333/334 (`claude-sonnet-4-5`): 333 = Base, 334 = Thinking
```

---

## Verification Checklist

### Code Verification (Already Done):
- [x] Searched codebase for Model ID 246 references
- [x] Found reference in `request.rs:29` comment
- [x] Confirmed parameter-based architecture in Epic-008

### Documentation Verification:
- [x] MASTER-MODELS-TABLE.md shows both `gemini-2.5-pro` and `gemini-2.5-pro-thinking`
- [x] Epic-015 COMPARISON file exists for thinking mode
- [x] Base workflow exists for gemini-2.5-pro

### Architecture Understanding:
- [x] Model ID 246 = BASE model (`gemini-2.5-pro`)
- [x] Thinking = parameter activation, not separate ID
- [x] Compared with Flash (312/313 separate IDs)
- [x] Compared with Claude (333/334 separate IDs)
- [x] Compared with Gemini 3.x (0 parameter-based)

---

## Impact on Epic-020

### Pivot Opportunity Re-Assessment:

**Before Clarification**:
- Expected: 3 new models (246, 312, 313)
- Coverage: +5.5%
- Effort: 1-2 days

**After Clarification**:
- Reality: Model 246 FULLY documented (both modes) ✅
- Models 312/313: Workflows ✅, COMPARISOs ❌
- Coverage gain: +~2% (2 COMPARISON files)
- Effort: 8-12 hours (not 1-2 days)
- ROI: MEDIUM (not HIGH)

### Epic-020 Final Decision:

**CLOSE Epic-020** as COMPLETE with following outcomes:
1. ✅ Model IDs 314-327 confirmed DEPRECATED (99.5% confidence)
2. ✅ Documentation coverage clarified (40/40 real models)
3. ✅ Model ID architecture understood (parameter-based vs. ID-based)
4. ✅ Pivot opportunity evaluated and corrected (2 COMPARISOs, not 3 models)

**Optional Follow-Up** (Backlog):
- Create COMPARISON for `gemini-2.5-flash` (Model ID 312 base)
- Create COMPARISON for `gemini-2.5-flash-thinking` (Model ID 313)
- Effort: 4-6 hours each (total 8-12 hours)
- Priority: LOW (workflows already documented)

---

## Summary

**Model ID 246 = `gemini-2.5-pro` (BASE)**
- Thinking activated via `thinkingBudget` parameter
- NOT a separate Model ID for thinking variant
- Parameter-based architecture (like Gemini 3.x)
- Fully documented: Base workflow ✅, Thinking workflow ✅, COMPARISON ✅ (Epic-015)

**Key Takeaway**: Understanding Model ID architecture is critical for accurate documentation and coverage analysis. Parameter-based models (246, 0) use ONE ID with parameter activation, while ID-based models (312/313, 333/334) use SEPARATE IDs for each mode.

---

**Document Status**: ✅ REFERENCE COMPLETE
**Created By**: Tech Lead (Epic-020)
**Date**: 2026-01-26
**Related Documents**:
- docs/epics/Epic-008-Gemini-2.5-Pro-Thinking-Optimization.md
- docs/research/EPIC-020-COMPLETION-REPORT.md
- docs/research/MODEL-ID-ALLOCATION-REFERENCE.md
- docs/comparison/MASTER-MODELS-TABLE.md
