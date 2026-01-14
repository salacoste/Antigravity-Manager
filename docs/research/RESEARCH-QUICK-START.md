# Epic-020 Research Quick Start Guide

**For**: Dev Team C - Junior Developer onboarding
**Purpose**: Get productive in 5 minutes
**Duration**: Read time ~3 minutes

---

## 🎯 Your Mission (Today)

Research Model IDs **314-327** (14 models) to discover:
- What are these models?
- Where are they referenced in code?
- Are they active, experimental, or deprecated?
- Should we create documentation for them?

---

## 📋 What You'll Do Today

### Morning ✅ COMPLETE
1. ✅ Read discovery template
2. ✅ Understand tracking matrix structure
3. ✅ Review existing COMPARISON files format
4. ✅ Prepare search strategies

### Afternoon 🔄 YOUR TURN NOW
1. Search codebase for model IDs 314-327
2. Document findings in tracking matrix
3. Note code locations and contexts
4. Assign confidence levels

### Tomorrow 📋 NEXT PHASE
1. API endpoint testing
2. Feature detection for each model
3. Create COMPARISON files
4. Implementation recommendations

---

## 🚀 Quick Commands to Get Started

### Search for Model IDs in Code
```bash
# From project root:
cd /Users/r2d2/Documents/Code_Projects/00_mcp/Antigravity-Manager

# Search for any of the model IDs
grep -r "314\|315\|316\|317\|318\|319\|320\|321\|322\|323\|324\|325\|326\|327" src-tauri/

# Also search frontend
grep -r "314\|315\|316\|317\|318\|319\|320\|321\|322\|323\|324\|325\|326\|327" src/
```

### Key Files to Check
```
src-tauri/src/proxy/mappers/gemini/model_mapping.rs   ← Model definitions
src-tauri/src/proxy/mappers/openai/model_mapping.rs   ← Alternative format
src-tauri/src/proxy/mappers/claude/model_mapping.rs   ← Another variant
src-tauri/src/models/mod.rs                            ← Data structures
```

---

## 📚 Your Reference Documents

**You now have 3 guides:**

1. **MODEL-IDS-314-327-TRACKING-MATRIX.md** ← Main deliverable
   - Empty table ready for your findings
   - Search strategies and checklists
   - Progress tracking

2. **COMPARISON-FILES-REFERENCE.md** ← Format guide
   - How existing COMPARISON files are structured
   - Examples and standards
   - When to create new files

3. **MODEL-IDS-314-327-DISCOVERY-TEMPLATE.md** ← Full research template
   - Comprehensive research structure
   - API testing procedures
   - Feature detection tests
   - Implementation recommendations

---

## ⚡ 30-Minute Workflow

### Minute 1-5: Orient Yourself
- [ ] Read this Quick Start
- [ ] Open tracking matrix in editor
- [ ] Have discovery template available as reference

### Minute 6-25: Execute First Search
```bash
# Run grep command for models
grep -r "314\|315\|316\|317" src-tauri/ > findings.txt
grep -r "314\|315\|316\|317" src/ >> findings.txt

# Look at results
cat findings.txt | head -20
```

### Minute 26-30: Document First Finding
- [ ] Pick the first model found
- [ ] Note file location and line number
- [ ] Add to tracking matrix
- [ ] Record confidence level
- [ ] Note any observations

---

## 📊 What Success Looks Like

### By End of Afternoon
- [ ] All 14 models (314-327) have entries
- [ ] Most have at least "Unknown" or found status
- [ ] Code references documented (if found)
- [ ] Tracking matrix updated
- [ ] Ready for tomorrow's testing

### Confidence Levels Explained
- **High**: Found in code + official documentation + follows naming convention
- **Medium**: Found in code + some documentation + reasonable interpretation
- **Low**: Found in code but unclear or missing supporting evidence
- **Unknown**: Not found anywhere

---

## 🔍 Example Finding

Here's what a completed entry might look like:

```
Model ID: 314
Model Name: gemini-3-pro-standard
Code References:
  - src-tauri/src/proxy/mappers/gemini/model_mapping.rs:145
Log References: None found
Confidence: High
Status: Identified
Discovery Date: 2026-01-13
Notes: Found in current model mapping, appears to be production variant
```

---

## 💡 Pro Tips

### Tip 1: Search Patterns
- Try partial searches first: `grep -r "31[0-9]" src-tauri/`
- Then narrow down: `grep "314" src-tauri/`
- Check both snake_case and kebab-case variants

### Tip 2: Code Context
When you find a reference, look at:
- Variable names around it
- Comments explaining the model
- Related code in same file
- Test files that might use it

### Tip 3: Naming Conventions
- If ID is 314, look for strings containing "314"
- Also look for hex equivalents or aliases
- Check for comments with model names
- Review enum definitions

### Tip 4: Multiple Locations
One model might appear in:
- model_mapping.rs (mapping logic)
- request.rs (request transformation)
- response.rs (response handling)
- tests/ (test cases)
- All of the above = Higher confidence!

---

## 🚦 Decision Tree

When you find a model reference, ask:

```
Found a reference to model 314?
├─ Is it in official Google/Anthropic/OpenAI docs?
│  └─ YES: Mark Confidence = High
│  └─ NO: Continue below
├─ Does it follow known naming convention?
│  └─ YES: Mark Confidence = Medium→High
│  └─ NO: Continue below
├─ Is it referenced in multiple files?
│  └─ YES: Mark Confidence = Medium
│  └─ NO: Mark Confidence = Low
└─ No references anywhere?
   └─ Mark Status = Unknown
```

---

## ❓ Common Questions

**Q: What if I can't find a model?**
A: That's valuable! Mark it as "Unknown" - negative evidence is still evidence.

**Q: How specific should I be?**
A: Include file path AND line number. Example: `src-tauri/src/proxy/mappers/gemini/model_mapping.rs:145`

**Q: What counts as "Code References"?**
A: Any mention in Rust/TypeScript source code (not comments unless critical).

**Q: Should I test the API?**
A: No - save that for tomorrow's phase. Just find references today.

**Q: What if I find conflicting information?**
A: Document both! Mark confidence as "Low" and note the conflict in observations.

---

## 📞 Getting Help

### Resources Available
- **Discovery Template**: Full reference with examples
- **Tracking Matrix**: Pre-formatted table with guidelines
- **COMPARISON Reference**: Examples of final output format

### When Stuck
1. Re-read the "Search Strategies" in the tracking matrix
2. Check if the model might be under a different name (alias)
3. Review examples in COMPARISON-FILES-REFERENCE.md
4. Look at how similar model ranges (280-313, 328+) are handled

---

## ✅ Next Steps

**Right Now:**
1. Open terminal
2. Navigate to project root
3. Run first grep command
4. Report back findings to tracking matrix

**This Afternoon:**
1. Complete all 14 model searches
2. Update tracking matrix with 100% coverage (even if "Unknown")
3. Prepare observations for team review

**Tomorrow Morning:**
1. Review findings with tech lead
2. Begin API testing phase
3. Start drafting COMPARISON files

---

## 📝 Your Tracking Documents

All documents are in: `/Users/r2d2/Documents/Code_Projects/00_mcp/Antigravity-Manager/docs/research/`

- `MODEL-IDS-314-327-TRACKING-MATRIX.md` ← Main working doc
- `COMPARISON-FILES-REFERENCE.md` ← Format guide
- `MODEL-IDS-314-327-DISCOVERY-TEMPLATE.md` ← Full template
- `RESEARCH-QUICK-START.md` ← This file

---

## 🎯 Success Criteria

You're done for the day when:
- [ ] All 14 models (314-327) have status entries
- [ ] Code references documented (or "Not found" noted)
- [ ] Confidence levels assigned to each
- [ ] Tracking matrix is 100% complete
- [ ] Observations documented
- [ ] Ready for tomorrow's testing phase

---

**You've got this! Questions? Check the Discovery Template or Comparison Reference.**

*Good luck, Dev C! - Your Tech Lead*

---

**Document Version**: 1.0
**Created**: 2026-01-13
**Status**: ✅ Ready to Use
