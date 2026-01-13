# COMPARISON Files Reference Guide

**Purpose**: Quick reference for existing COMPARISON file format and structure
**Created**: 2026-01-13
**For**: Epic-020 Research Team - Understanding documentation standards

---

## 📚 Existing COMPARISON Files Overview

### Location Structure
```
docs/
├── comparison/
│   ├── MASTER-MODELS-TABLE.md (central registry)
│   ├── README.md (format guide)
│   ├── claude/
│   │   └── (Claude model comparisons)
│   ├── gemini/
│   │   └── (Gemini model comparisons)
│   └── openai/
│       └── (OpenAI model comparisons)
└── antigravity/
    └── workflows/
        └── models/
            └── (Additional model comparisons)
```

---

## 📊 Existing COMPARISON Files Summary

### Root Level Comparison Files
Located: `docs/comparison/`

| File Name | Purpose | Status |
|-----------|---------|--------|
| MASTER-MODELS-TABLE.md | Central registry of ALL models (54+) | ✅ Active |
| README.md | Format and structure guide | ✅ Active |
| ANTHROPIC_VS_GOOGLE_THINKING.md | Thinking mode comparison | ✅ Reference |
| claude-opus-4-5-COMPARISON.md | Claude Opus 4.5 detailed analysis | ✅ Complete |
| claude-4-5-sonnet-COMPARISON.md | Claude Sonnet 4.5 detailed analysis | ✅ Complete |

### Gemini Model Comparisons
Located: `docs/comparison/gemini/`

| Model Name | File Name | Type | Status | Notes |
|------------|-----------|------|--------|-------|
| Gemini 3 Pro High | gemini-3-pro-high-COMPARISON.md | Active | ✅ | 38-42 KB |
| Gemini 3 Pro Low | gemini-3-pro-low-COMPARISON.md | Active | ✅ | Under review |
| Gemini 3 Flash | gemini-3-flash-COMPARISON.md | Active | ✅ | 4 quality levels |
| Gemini 3 Pro Image | gemini-3-pro-image-COMPARISON.md | Active | ✅ | 21 variants |
| Gemini 2.5 Pro Thinking | gemini-2.5-pro-thinking-COMPARISON.md | Thinking | ✅ | Production |
| Gemini 2.5 Flash Lite Thinking | gemini-2.5-flash-lite-thinking-COMPARISON.md | Thinking | ❌ | Not supported |
| Gemini 2.0 Flash Exp | gemini-2.0-flash-exp-COMPARISON.md | Experimental | ✅ | Experimental |

### Claude Model Comparisons
Located: `docs/comparison/claude/`

| Model Name | File Name | Type | Status | Notes |
|------------|-----------|------|--------|-------|
| Claude Opus 4.5 | claude-opus-4-5-COMPARISON.md | Production | ✅ | 200K context |
| Claude Sonnet 4.5 | claude-4-5-sonnet-COMPARISON.md | Production | ✅ | Latest standard |

### OpenAI Model Comparisons
Located: `docs/comparison/openai/`

| Model Name | File Name | Type | Status | Notes |
|------------|-----------|------|--------|-------|
| GPT-4o | (in openai/) | Production | ✅ | N/A |
| GPT-4 Turbo | (in openai/) | Production | ✅ | N/A |

### Workflow Comparison Files
Located: `docs/antigravity/workflows/models/gemini/`

| File Name | Purpose | Status | Notes |
|-----------|---------|--------|-------|
| gemini-3-flash-COMPARISON.md | Workflow variant | ✅ | See also root level |
| gemini-3-pro-high-COMPARISON.md | Workflow variant | ✅ | See also root level |
| gemini-3-pro-low-COMPARISON.md | Workflow variant | ✅ | See also root level |
| gemini-3-pro-image-COMPARISON.md | Workflow variant | ✅ | See also root level |
| gemini-2.5-pro-thinking-COMPARISON.md | Workflow variant | ✅ | See also root level |
| gemini-2.5-flash-lite-thinking-COMPARISON.md | Workflow variant | ❌ | Not supported |

---

## 🎯 COMPARISON File Format Overview

### Standard Section Structure

Each COMPARISON file typically includes:

1. **Header & Metadata**
   - Model name and ID
   - Version information
   - Last updated date
   - Stability status

2. **Executive Summary**
   - Quick overview of model capabilities
   - Key use cases
   - Performance characteristics

3. **Feature Matrix**
   ```markdown
   | Feature | Supported | Quality | Notes |
   |---------|-----------|---------|-------|
   | Text Generation | ✅ | Excellent | Fast |
   | Tool Calling | ✅ | Good | Standard |
   | Vision | ❌ | N/A | Not available |
   ```

4. **Detailed Capabilities**
   - Text generation quality
   - Token limits and counting
   - Response modalities
   - Model-specific features

5. **Performance Characteristics**
   - Speed benchmarks
   - Cost analysis
   - Comparison with similar models
   - Use case recommendations

6. **Integration Notes**
   - API endpoint specifics
   - Request/response handling
   - Special parameters
   - Known limitations

7. **Comparison Section**
   - vs. Previous versions
   - vs. Competing models
   - Advantages/disadvantages
   - Best for / Worst for scenarios

8. **Implementation Guidance**
   - When to use this model
   - When to avoid
   - Performance tuning tips
   - Code examples

---

## 📋 Key Formatting Standards

### Status Indicators
- ✅ Complete/Supported/Active
- ❌ Missing/Not Supported/Deprecated
- ⏳ In Progress/Planned
- ❓ Unknown/Unclear
- 🔴 High Priority/Critical
- 🟡 Medium Priority
- 🟢 Low Priority

### Confidence Levels
- **Confirmed**: Multiple sources verify
- **High**: Strong evidence, minor uncertainties
- **Medium**: Some evidence, notable gaps
- **Low**: Limited evidence, significant uncertainties
- **Unknown**: No evidence found

### Model Status Categories
- **Active**: Currently supported in production
- **Experimental**: Testing phase, subject to change
- **Deprecated**: Older version, avoid for new projects
- **Blocked**: Cannot be used (unavailable/unsupported)
- **Unknown**: Status unclear, needs investigation

---

## 🔍 Template Example Structure

Below is the typical structure observed in COMPARISON files:

```markdown
# Model Name COMPARISON

**Model ID**: [ID number]
**Official Name**: [Full API model name]
**Family**: [Model family, e.g., Gemini 3.0]
**Status**: [Active/Experimental/Deprecated]
**Last Updated**: [Date]

## Overview
[Brief description of model]

## Capabilities

| Capability | Status | Notes |
|------------|--------|-------|
| Feature 1 | ✅ | Description |
| Feature 2 | ❌ | Reason |

## Performance
- Speed: [Relative performance]
- Cost: [Cost characteristics]
- Quality: [Output quality assessment]

## Best For
- Use case 1
- Use case 2
- Use case 3

## Limitations
- Limitation 1
- Limitation 2

## Comparison
### vs. [Model A]
- Advantage: [A]
- Disadvantage: [A]

## Implementation
[Guidance for using this model]

## Resources
- Official docs: [Link]
- Examples: [Link]
```

---

## 📚 File Statistics

### Coverage Summary
```yaml
Total Models Documented: 40/54+ (74.1%)

By Provider:
  Gemini: 27/42+ (64.3%)
    Active: 17 documented
    Experimental: 12 documented

  Claude: 9/9 (100%)
    All production models documented

  OpenAI: 4/4 (100%)
    All major models documented

By Type:
  Base Workflows: 30 files
  Thinking Workflows: 9 files
  COMPARISON Metadata: 5+ files
  Total Files: 44+
```

### File Size Ranges
- Small (5-15 KB): Entry-level models or brief comparisons
- Medium (15-40 KB): Detailed single-model analysis
- Large (40-60 KB): Comprehensive with variants and testing

---

## 🎓 When to Create a COMPARISON File

Create a new COMPARISON file when:

✅ **DO Create** if the model:
- Has unique capabilities vs. existing models
- Is actively supported (not deprecated)
- Differs significantly from related models/aliases
- Documentation adds value for users
- User demand exists for this model

❌ **DON'T Create** if the model:
- Is deprecated or will be removed
- Is an alias of an existing documented model
- Has no public API availability
- Documentation is already comprehensive elsewhere

⚠️ **MAYBE Create** if:
- Model is experimental (document with ⏳ status)
- Model has limited differentiation (brief summary only)
- Status is unclear (mark with ❓ and flag for follow-up)

---

## 🚀 Reference for Model IDs 314-327 Research

### How This Applies to Your Research

1. **Format Consistency**: New COMPARISON files for 314-327 should follow the standards shown above
2. **Status Indication**: Use the status indicators consistently across your tracking matrix
3. **Evidence Required**: Like existing files, confidence levels should be justified by evidence
4. **Integration Point**: Findings will integrate into MASTER-MODELS-TABLE.md
5. **Priority Framework**: Use Priority scoring matrix (P0-P3) for implementation recommendations

### Expected Outcomes

For each model in 314-327 range that is:
- **Identified + Active** → Create full COMPARISON file (40-60 KB target)
- **Identified + Experimental** → Create brief COMPARISON file (15-25 KB target)
- **Identified + Deprecated** → Create summary only (5-10 KB target)
- **Unknown** → Document as "Not Found" in research notes

---

## 📖 Master Reference Documents

Key documents to consult during your research:

| Document | Purpose | Location |
|----------|---------|----------|
| MASTER-MODELS-TABLE.md | Central model registry | docs/comparison/ |
| MODEL-IDS-314-327-DISCOVERY-TEMPLATE.md | Research template | docs/research/ |
| ANTHROPIC_VS_GOOGLE_THINKING.md | Thinking mode guide | docs/comparison/ |
| README.md | Format guidelines | docs/comparison/ |

---

## ✅ Deliverables Checklist for Your Research

By end of Epic-020, complete these items:

- [ ] All 14 models (314-327) have entries in tracking matrix
- [ ] Code references documented with file paths and line numbers
- [ ] Confidence levels assigned with justification
- [ ] COMPARISON files created for priority models
- [ ] Integration recommendations provided
- [ ] Findings merged into MASTER-MODELS-TABLE.md
- [ ] New implementation stories drafted if needed

---

**Document Version**: 1.0
**Created**: 2026-01-13
**Reference Status**: ✅ Guide Complete
**For**: Epic-020 Research Team
