# Epic-020 Research Documentation

**Project**: Antigravity Tools - Model IDs 314-327 Research Initiative
**Objective**: Discover, document, and create comparison files for 14 unknown model IDs
**Team**: Dev C (Junior Developer, Epic-020 Research Team)
**Start Date**: 2026-01-13
**Current Phase**: Day 1 - Morning (Template Preparation) ✅ COMPLETE

---

## 📚 Document Index

### Quick Navigation
**Just starting?** Start here:
1. [RESEARCH-QUICK-START.md](RESEARCH-QUICK-START.md) ← Begin here (5 min read)
2. [MODEL-IDS-314-327-TRACKING-MATRIX.md](MODEL-IDS-314-327-TRACKING-MATRIX.md) ← Main working doc
3. [EPIC-020-DAY1-MORNING-SUMMARY.md](EPIC-020-DAY1-MORNING-SUMMARY.md) ← Status update

### Complete Document Set

| Document | Purpose | Status | Size | Read Time |
|----------|---------|--------|------|-----------|
| **RESEARCH-QUICK-START.md** | Fast onboarding guide | ✅ Ready | 7.5 KB | 3 min |
| **MODEL-IDS-314-327-TRACKING-MATRIX.md** | Main tracking table | ✅ Ready | 9.7 KB | 5 min |
| **COMPARISON-FILES-REFERENCE.md** | Format standards guide | ✅ Ready | 9.5 KB | 7 min |
| **MODEL-IDS-314-327-DISCOVERY-TEMPLATE.md** | Full research template | ✅ Ready | 12 KB | 15 min |
| **EPIC-020-DAY1-MORNING-SUMMARY.md** | Day 1 morning recap | ✅ Ready | 8.5 KB | 5 min |
| **README.md** | This file | ✅ Ready | 2.5 KB | 3 min |

**Total**: 49.2 KB of comprehensive research documentation | 38 minutes to read all

---

## 🎯 Your Research Mission

### The Challenge
Research 14 mystery model IDs (314-327) in the Antigravity codebase:
- **What**: Identify what these models are
- **Where**: Find code references and usage
- **Why**: Understand if they're active, experimental, or deprecated
- **How**: Create documentation and COMPARISON files

### The Deliverables
By end of Epic-020:
- [x] **Phase 1** (Day 1 Morning): Template preparation ✅ COMPLETE
- [ ] **Phase 2** (Day 1 Afternoon): Initial research & tracking matrix completion
- [ ] **Phase 3** (Day 2): Feature detection and testing
- [ ] **Phase 4** (Day 3): COMPARISON file creation
- [ ] **Phase 5** (Day 4+): Integration and finalization

---

## 🚀 Getting Started (5 Minutes)

### Step 1: Read Quick Start (3 min)
```bash
cat /Users/r2d2/Documents/Code_Projects/00_mcp/Antigravity-Manager/docs/research/RESEARCH-QUICK-START.md
```

### Step 2: Understand Your Mission
- Research Model IDs 314-327 (14 total)
- Search codebase for references
- Document findings in tracking matrix
- Assign confidence levels

### Step 3: Execute First Search (2 min)
```bash
cd /Users/r2d2/Documents/Code_Projects/00_mcp/Antigravity-Manager
grep -r "314\|315\|316\|317\|318\|319\|320\|321\|322\|323\|324\|325\|326\|327" src-tauri/
```

---

## 📋 Document Guide

### 1. RESEARCH-QUICK-START.md
**For**: Getting productive immediately
**Contains**:
- 30-minute workflow
- Copy-paste commands
- Decision trees
- Pro tips
- Q&A

**When to read**: First thing (3 minutes)

---

### 2. MODEL-IDS-314-327-TRACKING-MATRIX.md
**For**: Tracking and organizing findings
**Contains**:
- Empty 14-row tracking table
- Column definitions
- Reference template structure
- Discovery checklist
- Search strategies

**When to read**: After quick-start, before research (5 minutes)
**When to use**: Continuously throughout afternoon research

---

### 3. COMPARISON-FILES-REFERENCE.md
**For**: Understanding documentation format
**Contains**:
- Overview of 40+ existing COMPARISON files
- Format standards and structure
- Examples and templates
- When to create files
- File statistics

**When to read**: Before creating documentation (7 minutes)

---

### 4. MODEL-IDS-314-327-DISCOVERY-TEMPLATE.md
**For**: Comprehensive research procedures
**Contains**:
- Full research template (60+ pages)
- Model identification procedures
- API endpoint testing
- Feature detection tests
- Implementation guidance

**When to read**: Tomorrow for Day 2 testing (15 minutes)
**When to use**: Reference during detailed investigation

---

### 5. EPIC-020-DAY1-MORNING-SUMMARY.md
**For**: Understanding what's been completed
**Contains**:
- Morning phase summary
- Deliverables overview
- Reference materials
- Quality checklist
- Success metrics

**When to read**: Now or end of day for recap (5 minutes)

---

## ✅ Daily Workflow Templates

### Day 1 Afternoon (Research Phase)
```
Hour 1: Searching
  └─ Execute grep commands for model IDs
  └─ Document all found references
  └─ Note file paths and line numbers

Hour 2: Organization
  └─ Update tracking matrix with findings
  └─ Assign confidence levels
  └─ Note observations

Hour 3: Preparation
  └─ Review findings for patterns
  └─ Identify priority models for tomorrow
  └─ Prepare observation summary
```

### Day 2 (Testing Phase)
```
Morning: API Testing
  └─ Test identified models via API
  └─ Verify active vs. experimental
  └─ Document response patterns

Afternoon: Feature Detection
  └─ Test vision, thinking, audio features
  └─ Benchmark performance
  └─ Complete discovery template
```

### Day 3+ (Documentation Phase)
```
Create COMPARISON Files
  └─ High priority models first
  └─ Follow format standards
  └─ Include all sections
  └─ Cross-reference with other models

Update MASTER-MODELS-TABLE
  └─ Add new model entries
  └─ Update statistics
  └─ Link to new COMPARISON files
```

---

## 🔍 Key Files to Search

### Codebase Locations
```
/src-tauri/src/proxy/mappers/gemini/model_mapping.rs     ← Model definitions
/src-tauri/src/proxy/mappers/openai/model_mapping.rs     ← Alternative format
/src-tauri/src/proxy/mappers/claude/model_mapping.rs     ← Another variant
/src-tauri/src/models/mod.rs                              ← Data structures
```

### Search Command Quick Reference
```bash
# Search all Rust code
grep -r "314\|315\|316\|317\|318\|319\|320\|321\|322\|323\|324\|325\|326\|327" src-tauri/

# Search frontend code
grep -r "314\|315\|316\|317\|318\|319\|320\|321\|322\|323\|324\|325\|326\|327" src/

# Focus on model mapping
grep -r "314\|315\|316\|317\|318\|319\|320\|321\|322\|323\|324\|325\|326\|327" src-tauri/src/proxy/mappers/

# Count occurrences
grep -r "314\|315\|316\|317\|318\|319\|320\|321\|322\|323\|324\|325\|326\|327" src-tauri/ | wc -l
```

---

## 📊 Reference Standards

### Confidence Levels
- **Confirmed**: Multiple sources verify the model's identity
- **High**: Strong evidence from code + documentation + naming conventions
- **Medium**: Code reference + some supporting evidence
- **Low**: Limited evidence, significant uncertainties remain
- **Unknown**: No evidence found

### Status Categories
- **Identified**: Model found and documented
- **Unknown**: No references found in codebase
- **Deprecated**: Older version, marked for removal
- **Blocked**: Cannot be used (unavailable or unsupported)
- **Experimental**: Testing phase, subject to change

### Priority Levels (for implementation)
- **P0**: Critical - implement immediately
- **P1**: High - implement in Q2
- **P2**: Medium - implement in Q3
- **P3**: Low - backlog or document only

---

## 🎓 Success Criteria

### By End of Day 1 Afternoon
- [ ] All 14 models (314-327) have entries
- [ ] Code references documented (or "Not found" noted)
- [ ] Confidence levels assigned to each model
- [ ] Tracking matrix 100% complete
- [ ] Observations documented
- [ ] Ready for Day 2 testing phase

### By End of Day 2
- [ ] Feature detection complete for all models
- [ ] API testing results documented
- [ ] Capability matrices filled
- [ ] COMPARISON file recommendations prepared

### By End of Epic-020
- [ ] All identified models have COMPARISON files
- [ ] MASTER-MODELS-TABLE updated
- [ ] Implementation stories drafted
- [ ] 100% documentation coverage for identified models

---

## 🔗 Related Documentation

### In This Directory
- All files documented above

### In Parent Directories
- `docs/comparison/MASTER-MODELS-TABLE.md` - Central model registry
- `docs/comparison/README.md` - Format guidelines
- `docs/comparison/claude/` - Claude model comparisons
- `docs/comparison/gemini/` - Gemini model comparisons
- `docs/comparison/openai/` - OpenAI model comparisons

### External References
- [Google AI Studio](https://aistudio.google.com/)
- [Vertex AI Documentation](https://cloud.google.com/vertex-ai/docs)
- [Anthropic Claude Docs](https://docs.anthropic.com/)
- [OpenAI API Reference](https://platform.openai.com/docs/)

---

## 💡 Pro Tips for Researchers

1. **Search Systematically**: Use all search strategies (code, config, logs, comments)
2. **Document Everything**: Even "not found" is valuable evidence
3. **Follow Patterns**: Look at how similar model IDs (280-313, 328+) are handled
4. **Multiple Evidence**: Higher confidence when found in multiple locations
5. **Ask Questions**: If unclear, mark as "Low" confidence and document the uncertainty

---

## ❓ Frequently Asked Questions

**Q: What if I don't find a model?**
A: That's valid research! Mark it as "Unknown" status. Negative evidence is still evidence.

**Q: How detailed should my notes be?**
A: Include file path, line number, and context. The more specific the better.

**Q: Should I test the API today?**
A: No - save API testing for Day 2. Today is research and documentation only.

**Q: What if models have multiple names?**
A: Document all aliases found. This helps us understand naming conventions.

**Q: How confident should I be?**
A: Start conservatively. You can revise tomorrow after API testing.

---

## 📞 Support Resources

### If Stuck
1. Review the **RESEARCH-QUICK-START.md** tips section
2. Check the **COMPARISON-FILES-REFERENCE.md** examples
3. Look at how existing similar models are handled
4. Review the discovery template for procedures

### Quick Reference
- **Tracking Matrix**: 9.7 KB, complete with examples
- **Discovery Template**: 12 KB, comprehensive procedures
- **COMPARISON Reference**: 9.5 KB, format standards
- **Quick Start**: 7.5 KB, immediate productivity

---

## 🎯 Next Steps

### Right Now
1. Read RESEARCH-QUICK-START.md (3 min)
2. Open the tracking matrix in your editor
3. Run first grep command
4. Start documenting findings

### This Afternoon
1. Complete systematic searches for all 14 models
2. Document all findings in tracking matrix
3. Update confidence levels
4. Prepare observation summary

### Tomorrow
1. Begin API testing phase
2. Complete feature detection
3. Start COMPARISON file creation
4. Prepare implementation recommendations

---

## 📝 Document Maintenance

- **Created**: 2026-01-13
- **Last Updated**: 2026-01-13
- **Version**: 1.0
- **Status**: ✅ All documentation complete and ready
- **Next Update**: After Day 1 afternoon research

---

## ✨ Summary

You now have everything needed to research Model IDs 314-327:
- ✅ Clear mission and objectives
- ✅ Systematic tracking infrastructure
- ✅ Search strategies and procedures
- ✅ Format standards and examples
- ✅ Quick-start guide for immediate productivity
- ✅ Comprehensive reference materials

**You're ready to start researching. Good luck!**

---

**Questions? Check the Quick Start guide or consult the full Discovery Template.**

*Last revised: 2026-01-13*
