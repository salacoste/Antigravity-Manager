# Model IDs 314-327 Research Tracking Matrix

**Document Purpose**: Central tracking matrix for Epic-020 research on Model IDs 314-327
**Date Created**: 2026-01-13
**Team**: Epic-020 Research (Dev Team C)
**Status**: Template Ready - Awaiting Day 1 Afternoon Findings

---

## 📊 Quick Summary

| Metric | Count | Status |
|--------|-------|--------|
| **Total Models (314-327)** | 14 | Pending Investigation |
| **Models Identified** | 0 | Empty - Awaiting Findings |
| **Code References Found** | 0 | Empty - Awaiting Findings |
| **Documentation Created** | 0 | Empty - Awaiting Findings |
| **High Confidence** | 0 | Empty - Awaiting Findings |
| **Progress** | 0% | Starting |

---

## 🔍 Model ID Tracking Matrix (314-327)

**Column Definitions**:
- **Model ID**: Internal identifier (314-327 range)
- **Model Name**: Public-facing model name (e.g., gemini-3-pro-high)
- **Code References**: Files and line numbers where model appears in codebase
- **Log References**: Evidence from application logs (if found)
- **Confidence Level**: Research confidence (Low/Medium/High/Confirmed)
- **Status**: Research status (Identified/Unknown/Deprecated/Blocked)
- **Discovery Date**: When this model was identified
- **Notes**: Key observations and findings

| Model ID | Model Name | Code References | Log References | Confidence | Status | Discovery Date | Notes |
|----------|-----------|-----------------|----------------|------------|--------|-----------------|-------|
| 314 | — | — | — | — | Unknown | — | Awaiting discovery |
| 315 | — | — | — | — | Unknown | — | Awaiting discovery |
| 316 | — | — | — | — | Unknown | — | Awaiting discovery |
| 317 | — | — | — | — | Unknown | — | Awaiting discovery |
| 318 | — | — | — | — | Unknown | — | Awaiting discovery |
| 319 | — | — | — | — | Unknown | — | Awaiting discovery |
| 320 | — | — | — | — | Unknown | — | Awaiting discovery |
| 321 | — | — | — | — | Unknown | — | Awaiting discovery |
| 322 | — | — | — | — | Unknown | — | Awaiting discovery |
| 323 | — | — | — | — | Unknown | — | Awaiting discovery |
| 324 | — | — | — | — | Unknown | — | Awaiting discovery |
| 325 | — | — | — | — | Unknown | — | Awaiting discovery |
| 326 | — | — | — | — | Unknown | — | Awaiting discovery |
| 327 | — | — | — | — | Unknown | — | Awaiting discovery |

---

## 📋 Reference Template Structure

### For Each Model Discovery

When you complete research on a model, create an entry following this template:

```markdown
### Model [XXX] - [Model Name]

**Identification Status**: ✅ Confirmed / 🔄 In Progress / ❌ Unknown

**Model Name**: [e.g., gemini-3-pro-standard]
**Model Family**: [e.g., Gemini 3.0, Claude 4.5]
**Status**: [Active/Experimental/Deprecated]

**Code References**:
- File: `src-tauri/src/proxy/mappers/gemini/model_mapping.rs`
- Line: XXX
- Context: [Relevant code snippet or description]

**Log References**:
- [Log file location and sample entries if found]

**Confidence Assessment**:
- Found in official documentation: [Yes/No]
- Code reference verified: [Yes/No]
- API responds successfully: [Yes/No]
- Naming convention matches known pattern: [Yes/No]
- Community confirmation: [Yes/No]

**Overall Confidence**: [Low/Medium/High/Confirmed]

**Notes**: [Key findings and observations]
```

---

## 🎯 Discovery Checklist

For each model you investigate, verify these items:

### Basic Identification
- [ ] Model ID located in codebase
- [ ] Model name decoded/identified
- [ ] Official documentation found or confirmed missing
- [ ] API endpoint responds to test requests
- [ ] Naming convention analysis complete

### Code Analysis
- [ ] Grep search completed: `grep -r "314\|315\|316..." src-tauri/`
- [ ] File locations documented
- [ ] Line numbers recorded
- [ ] Context snippet captured
- [ ] Related code patterns analyzed

### Log Analysis
- [ ] Application logs searched for model references
- [ ] Error patterns examined if present
- [ ] Usage frequency estimated (if visible in logs)
- [ ] Deprecation warnings noted

### Confidence Evaluation
- [ ] Evidence matrix completed
- [ ] Uncertainties documented
- [ ] Alternative interpretations considered
- [ ] Confidence level assigned

---

## 🔧 Search Strategies

### Strategy 1: Direct Codebase Search
```bash
# Search for specific model IDs in source code
grep -r "314\|315\|316\|317\|318\|319\|320\|321\|322\|323\|324\|325\|326\|327" src-tauri/
grep -r "314\|315\|316\|317\|318\|319\|320\|321\|322\|323\|324\|325\|326\|327" src/
```

### Strategy 2: Model Mapping Analysis
```
Files to examine:
- src-tauri/src/proxy/mappers/gemini/model_mapping.rs
- src-tauri/src/proxy/mappers/openai/model_mapping.rs
- src-tauri/src/proxy/mappers/claude/model_mapping.rs
- src-tauri/src/models/mod.rs
```

### Strategy 3: Configuration Search
```
Files to check:
- Configuration files in data_dir/config.json
- JSON account files in data_dir/accounts/
- Environment-specific configurations
```

### Strategy 4: Log Analysis
```
Search locations:
- data_dir/logs/ for rotating log files
- Application console output during proxy operations
- Error messages and debug traces
```

---

## 📊 Related Reference Documents

### Existing Documentation
- **MASTER-MODELS-TABLE.md**: Comprehensive table of all models (54+)
  - Path: `docs/comparison/MASTER-MODELS-TABLE.md`
  - Shows current model ID ranges documented
  - Reference for status formatting and confidence levels

- **Discovery Template**: Full research template
  - Path: `docs/research/MODEL-IDS-314-327-DISCOVERY-TEMPLATE.md`
  - Use for detailed investigation of each model

### Model Comparison Files
- **Claude Comparisons**: `docs/comparison/claude/`
- **Gemini Comparisons**: `docs/comparison/gemini/`
- **OpenAI Comparisons**: `docs/comparison/openai/`

### Code Reference Locations
- **Model Mapping**: `src-tauri/src/proxy/mappers/*/model_mapping.rs`
- **Request Mappers**: `src-tauri/src/proxy/mappers/*/request.rs`
- **Response Mappers**: `src-tauri/src/proxy/mappers/*/response.rs`
- **Model Types**: `src-tauri/src/models/mod.rs`

---

## 📈 Progress Tracking

### Day 1 Milestone Targets

**Morning (Template Preparation - COMPLETE ✅)**
- ✅ Read and understood discovery template
- ✅ Created tracking matrix structure
- ✅ Prepared search strategies and checklists
- ✅ Document ready for afternoon findings

**Afternoon (Initial Discoveries - PENDING)**
- [ ] Execute codebase searches for 314-327 range
- [ ] Identify which models are referenced in code
- [ ] Document code locations and contexts
- [ ] Assign initial confidence levels
- [ ] Update this tracking matrix with findings

**Day 2 (Detailed Research - PENDING)**
- [ ] API endpoint testing for identified models
- [ ] Feature detection for each model
- [ ] Comparison analysis with existing models
- [ ] Complete COMPARISON files for priority models
- [ ] Implementation recommendations

---

## 🎓 Format Examples

### Example 1: Identified Model (High Confidence)

| Model ID | Model Name | Code References | Log References | Confidence | Status | Discovery Date | Notes |
|----------|-----------|-----------------|----------------|------------|--------|-----------------|-------|
| 314 | gemini-3-pro-standard | model_mapping.rs:145 | audit.log:2026-01-13 | High | Identified | 2026-01-13 | Found in current model mapping, active in production |

### Example 2: Unknown Model (Low Confidence)

| Model ID | Model Name | Code References | Log References | Confidence | Status | Discovery Date | Notes |
|----------|-----------|-----------------|----------------|------------|--------|-----------------|-------|
| 327 | [Unknown] | Not found | Not found | Low | Unknown | — | No references in code or logs as of 2026-01-13 |

### Example 3: Deprecated Model

| Model ID | Model Name | Code References | Log References | Confidence | Status | Discovery Date | Notes |
|----------|-----------|-----------------|----------------|------------|--------|-----------------|-------|
| 320 | gemini-2.5-flash-deprecated | model_mapping.rs:98 | None | High | Deprecated | 2026-01-13 | Marked as deprecated in mapping, no recent log entries |

---

## 📝 Notes for Researchers

### Important Reminders
1. **Confidence Levels Matter**: Don't mark as "Confirmed" without strong evidence
2. **Document Everything**: Even "Not Found" is valuable evidence
3. **Reference Context**: Always include line numbers and file paths
4. **Log Analysis**: Check logs for actual usage patterns
5. **Cross-Reference**: Compare findings across code and documentation

### Common Pitfalls to Avoid
- ❌ Assuming a model ID maps to a specific model name without verification
- ❌ Missing variant models (e.g., -thinking, -exp, -image versions)
- ❌ Forgetting to check all three mappers (gemini, openai, claude)
- ❌ Overlooking feature flags or conditional compilation
- ❌ Not documenting the search strategy used

### Success Criteria
- ✅ Each model has a clear status (Identified, Unknown, or Deprecated)
- ✅ Code references include file paths and line numbers
- ✅ Confidence levels are justified by evidence
- ✅ Notes explain key findings succinctly
- ✅ All 14 models have entries (even if "Unknown")

---

## 🚀 Next Steps After Matrix is Complete

1. **Analyze Patterns**: Look for naming conventions and model family groupings
2. **Prioritize Implementation**: Use confidence levels to prioritize COMPARISON file creation
3. **Create Implementation Stories**: For models ready for documentation
4. **Validate Findings**: Cross-reference with team members
5. **Update MASTER-MODELS-TABLE**: Integrate findings into main documentation

---

**Document Version**: 1.0
**Created**: 2026-01-13
**Last Updated**: 2026-01-13
**Status**: ✅ Ready for Day 1 Afternoon Research
