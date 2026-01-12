# Agent Best Practices: Token Limits & File Management

**Audience**: All AI agents (PM agents, QA agents, dev agents, analyst agents)
**Purpose**: Guidelines for handling token limits and creating manageable documentation
**Last Updated**: 2026-01-11

---

## 🚨 Token Limit Handling

### Problem Statement

AI agents have output token limits (e.g., 8192 tokens for some models). When creating comprehensive reports or documentation, agents may exceed these limits, resulting in:
- API errors: `max_output_tokens exceeded`
- Incomplete deliverables
- Failed task execution
- Lost work and wasted time

### ✅ Solution: Incremental File Creation Strategy

**NEVER attempt to generate large reports in a single response.**

Instead, use **incremental append strategy**:

```yaml
strategy:
  approach: "Build reports incrementally using multiple small writes"

  step_1_create_header:
    action: "Create file with header and executive summary"
    command: "cat > file.md << 'EOF' ... EOF"
    max_size: "~2000 tokens"

  step_2_append_sections:
    action: "Append each major section separately"
    command: "cat >> file.md << 'EOF' ... EOF"
    max_size: "~2000 tokens per append"
    repeat: "For each section"

  step_3_verify:
    action: "Read file to verify completeness"
    command: "wc -l file.md && head -20 file.md && tail -20 file.md"
```

### Example: Creating Large Validation Report

**❌ BAD - Single Write (Will Fail)**:
```bash
# This will fail with max_output_tokens error
cat > EPIC-013-VALIDATION-REPORT.md << 'EOF'
[50KB of content in one go]
EOF
```

**✅ GOOD - Incremental Append**:
```bash
# Step 1: Create header + executive summary
cat > EPIC-013-VALIDATION-REPORT.md << 'EOF'
# Epic-013 Validation Report
## Executive Summary
[2KB content]
EOF

# Step 2: Append story validations
cat >> EPIC-013-VALIDATION-REPORT.md << 'EOF'
## Story-by-Story Validation
### Story 013-01
[2KB content]
EOF

# Step 3: Append more stories
cat >> EPIC-013-VALIDATION-REPORT.md << 'EOF'
### Story 013-02
[2KB content]
EOF

# Continue for all sections...
```

---

## 📚 Reading Large Files in Chunks

### When Reading Files

**NEVER read entire large files at once.**

Use the `Read` tool with `offset` and `limit` parameters:

```yaml
reading_strategy:
  step_1_assess_size:
    action: "Check file size first"
    command: "wc -l large-file.md"
    decision: "If >500 lines, read in chunks"

  step_2_read_header:
    tool: "Read"
    parameters:
      file_path: "/path/to/large-file.md"
      offset: 0
      limit: 100
    purpose: "Understand structure"

  step_3_read_specific_sections:
    tool: "Read"
    parameters:
      file_path: "/path/to/large-file.md"
      offset: 200
      limit: 100
    purpose: "Read specific section"

  step_4_read_tail:
    tool: "Read"
    parameters:
      file_path: "/path/to/large-file.md"
      offset: -50
      limit: 50
    purpose: "Check end of file"
```

### Example: Reading COMPARISON File

```yaml
# COMPARISON files are typically 800-1000 lines

step_1_executive_summary:
  offset: 0
  limit: 50
  content: "Executive summary, compliance scores"

step_2_gap_analysis:
  offset: 350
  limit: 100
  content: "Gap 1 (CRITICAL-001) details"

step_3_roadmap:
  offset: 550
  limit: 100
  content: "Implementation roadmap"
```

---

## 🔗 Multi-File Strategy for Large Reports

### When to Split Reports

Split large reports into multiple linked files when:
- Total content exceeds 30KB
- Report has distinct sections (e.g., validation, findings, recommendations)
- Content will be referenced independently
- Multiple audiences (devs vs. QA vs. PM)

### Linking Strategy

**Main Report** (index file):
```markdown
# Epic-013 Validation Report

**Report Structure**:
- [Executive Summary](./epic-013-validation-summary.md)
- [Story-by-Story Validation](./epic-013-story-validation.md)
- [Gap Analysis](./epic-013-gap-analysis.md)
- [Findings & Recommendations](./epic-013-findings.md)
- [Reference Documentation](./epic-013-references.md)

## Quick Links
- [Critical Findings](./epic-013-findings.md#critical-findings)
- [COMPARISON Cross-Reference](./epic-013-references.md#comparison-file)
```

**Individual Files**:
```markdown
# Epic-013: Story-by-Story Validation

**Parent Report**: [Epic-013 Validation Report](./epic-013-validation-report.md)
**Previous**: [Executive Summary](./epic-013-validation-summary.md)
**Next**: [Gap Analysis](./epic-013-gap-analysis.md)

## Story 013-01: MEDIUM Level Testing
[Content]

## Story 013-02: Safety Settings Enhancement
[Content]
```

### File Size Guidelines

```yaml
file_size_limits:
  optimal_size: "10-20KB"
    reason: "Easy to read in single pass, good for agents"

  acceptable_size: "20-50KB"
    reason: "May require chunked reading, but manageable"

  too_large: ">50KB"
    reason: "Should be split into multiple files"
    action: "Split by logical sections"

  maximum_size: "100KB"
    reason: "Hard limit - MUST split if exceeded"
```

---

## 📋 Best Practices Summary

### DO ✅

1. **Create files incrementally** using multiple `cat >>` appends
2. **Read large files in chunks** using `offset` and `limit`
3. **Split reports >30KB** into multiple linked files
4. **Use descriptive filenames** that indicate content
5. **Add navigation links** between related files
6. **Verify file creation** after each append (check line count)
7. **Use consistent section markers** for easy chunked reading

### DON'T ❌

1. **Don't generate >8KB content** in single response
2. **Don't read entire large files** without offset/limit
3. **Don't create monolithic reports** >50KB
4. **Don't use relative paths** without context
5. **Don't skip verification** after file operations
6. **Don't lose progress** - save incrementally

---

## 🎯 Practical Examples

### Example 1: PM Agent Validation Report

**Scenario**: Creating Epic-013 validation report (expected size: 60KB)

**Strategy**:
```yaml
approach: "Incremental append with 10 sections"

sections:
  1_header_executive:
    size: "~3KB"
    command: "cat > report.md"

  2_epic_overview:
    size: "~5KB"
    command: "cat >> report.md"

  3_story_validation_p1:
    size: "~8KB"
    command: "cat >> report.md"

  4_story_validation_p2:
    size: "~8KB"
    command: "cat >> report.md"

  5_gap_analysis:
    size: "~10KB"
    command: "cat >> report.md"

  6_findings:
    size: "~8KB"
    command: "cat >> report.md"

  7_recommendations:
    size: "~8KB"
    command: "cat >> report.md"

  8_business_impact:
    size: "~5KB"
    command: "cat >> report.md"

  9_references:
    size: "~3KB"
    command: "cat >> report.md"

  10_appendix:
    size: "~2KB"
    command: "cat >> report.md"

result: "10 incremental appends, each <8KB, total 60KB report"
```

### Example 2: Analyst Agent Reading COMPARISON File

**Scenario**: Analyzing gemini-3-flash-COMPARISON.md (855 lines)

**Strategy**:
```yaml
approach: "Chunked reading with targeted sections"

phase_1_overview:
  tool: "Read"
  offset: 0
  limit: 50
  purpose: "Executive summary, compliance scores"

phase_2_gaps:
  gap_1:
    offset: 380
    limit: 50
    purpose: "Gap 1 (CRITICAL-001) details"

  gap_2:
    offset: 435
    limit: 35
    purpose: "Gap 2 (IMPL-002) details"

  gap_3:
    offset: 472
    limit: 42
    purpose: "Gap 3 (TEST-001) details"

  gap_4:
    offset: 515
    limit: 40
    purpose: "Gap 4 (OPT-001) details"

phase_3_roadmap:
  offset: 560
  limit: 70
  purpose: "Implementation roadmap (Phases 1-3)"

result: "Full analysis with 6 targeted reads, no full file scan"
```

### Example 3: QA Agent Multi-File Test Report

**Scenario**: Creating comprehensive test report for Epic-008

**Strategy**:
```yaml
approach: "Multi-file with cross-linking"

file_structure:
  epic-008-test-report.md:
    size: "5KB (index)"
    content: "Executive summary, links to details"

  epic-008-test-plan.md:
    size: "15KB"
    content: "Test strategy, scope, approach"

  epic-008-test-results.md:
    size: "25KB"
    content: "Test execution results, evidence"

  epic-008-test-issues.md:
    size: "12KB"
    content: "Defects, blockers, risks"

  epic-008-test-metrics.md:
    size: "8KB"
    content: "Coverage, pass rates, trends"

benefits:
  - "Each file independently readable"
  - "Easy to update specific sections"
  - "Clear separation of concerns"
  - "Agents can read relevant sections only"
```

---

## 🔍 Troubleshooting

### Issue: "API Error: max_output_tokens exceeded"

**Solution**:
1. **Stop immediately** - don't retry with same approach
2. **Switch to incremental strategy** - use `cat >>` appends
3. **Target 2-3KB per append** - aim for small chunks
4. **Verify after each append** - check file grows correctly
5. **Resume from last checkpoint** - don't start over

### Issue: "File too large to read"

**Solution**:
1. **Check file size** - `wc -l file.md`
2. **Read in chunks** - use offset/limit parameters
3. **Use grep for search** - `grep "pattern" file.md` instead of full read
4. **Extract sections** - `sed -n '100,200p' file.md`

### Issue: "Lost progress due to error"

**Solution**:
1. **Save incrementally** - commit after each major section
2. **Use checkpoints** - verify file after each append
3. **Document progress** - use todo list to track sections
4. **Resume capability** - design tasks to be resumable

---

## 📊 Performance Guidelines

### Token Budgets by Agent Type

```yaml
pm_agent:
  input_budget: "50K-80K tokens (COMPARISON files + epic specs)"
  output_budget: "6-7K tokens per response (safety margin)"
  strategy: "Incremental file creation (10-12 sections)"

qa_agent:
  input_budget: "30K-50K tokens (test specs + results)"
  output_budget: "6-7K tokens per response"
  strategy: "Multi-file reports (plan, results, issues, metrics)"

analyst_agent:
  input_budget: "60K-100K tokens (multiple COMPARISON files)"
  output_budget: "6-7K tokens per response"
  strategy: "Chunked reading + incremental analysis"

dev_agent:
  input_budget: "40K-60K tokens (code + specs)"
  output_budget: "6-7K tokens per response"
  strategy: "File-by-file implementation + incremental commits"
```

### Optimal Section Sizes

```yaml
section_sizes:
  executive_summary: "2-3KB (concise overview)"
  story_validation: "1.5-2KB per story"
  gap_analysis: "2-3KB per gap"
  findings: "1-2KB per finding"
  recommendations: "1-2KB per recommendation"
  appendix: "1-2KB (quick reference)"

total_report:
  small: "10-20KB (single file OK)"
  medium: "20-40KB (consider splitting)"
  large: "40-80KB (should split into 3-5 files)"
  very_large: ">80KB (must split into 5-10 files)"
```

---

## 🎓 Learning from Epic-014 Validation Failure

### What Happened

**PM Agent #2** attempted to create Epic-014 validation report:
- **Attempt 1**: Generate full 60KB report → max_output_tokens error
- **Attempt 2**: Generate in smaller sections → still too large, error
- **Attempt 3**: Generate even smaller → still exceeded limit
- **Attempt 4**: Final attempt → error
- **Result**: Agent completed but NO FILE CREATED

### What Should Have Happened

**Correct Approach** (what PM Agent #1 did successfully):
```yaml
epic_013_validation_success:
  strategy: "Incremental append"

  sections_created:
    1_header: "cat > file.md (3KB)"
    2_overview: "cat >> file.md (5KB)"
    3_stories_01_03: "cat >> file.md (8KB)"
    4_stories_04_06: "cat >> file.md (8KB)"
    5_gap_analysis: "cat >> file.md (10KB)"
    6_findings: "cat >> file.md (8KB)"
    7_recommendations: "cat >> file.md (8KB)"
    8_business: "cat >> file.md (5KB)"
    9_timeline: "cat >> file.md (5KB)"
    10_verdict: "cat >> file.md (6KB)"

  total_appends: 10
  result: "62KB report successfully created"
  failures: 0
```

### Key Lessons

1. **Plan sections before writing** - know your structure
2. **Target 5-8KB per append** - safe zone well below 8192 limit
3. **Verify after each section** - catch issues early
4. **Use consistent markers** - easy to resume if interrupted
5. **Document progress** - todo list with sections completed

---

## 📝 Template: Incremental Report Creation

```bash
#!/bin/bash
# Template for creating large reports incrementally

REPORT_FILE="epic-XXX-validation-report.md"

# Section 1: Header + Executive Summary (3KB)
cat > "$REPORT_FILE" << 'EOF'
# Epic-XXX Validation Report

## Executive Summary
[Summary content ~3KB]
EOF

echo "✅ Section 1 created ($(wc -l < "$REPORT_FILE") lines)"

# Section 2: Epic Overview (5KB)
cat >> "$REPORT_FILE" << 'EOF'

## Epic Overview Validation
[Overview content ~5KB]
EOF

echo "✅ Section 2 appended ($(wc -l < "$REPORT_FILE") lines)"

# Section 3: Story Validation Part 1 (8KB)
cat >> "$REPORT_FILE" << 'EOF'

## Story-by-Story Validation

### Story XXX-01
[Story validation ~8KB]
EOF

echo "✅ Section 3 appended ($(wc -l < "$REPORT_FILE") lines)"

# Continue for all sections...
# Section 4, 5, 6, etc.

# Final verification
echo "📊 Final report: $(wc -l < "$REPORT_FILE") lines, $(du -h "$REPORT_FILE" | cut -f1)"
```

---

## 🚀 Quick Reference Card

```yaml
token_limit_cheat_sheet:

  creating_files:
    ❌ wrong: "cat > file.md [50KB]"
    ✅ right: "cat > file.md [3KB], then cat >> file.md [3KB] x10"

  reading_files:
    ❌ wrong: "Read file.md (no limits)"
    ✅ right: "Read file.md offset=0 limit=100, then offset=100 limit=100"

  report_size:
    ❌ wrong: "Create 100KB monolithic report"
    ✅ right: "Create 5 linked 20KB reports"

  section_size:
    ❌ wrong: "Write 15KB section in one go"
    ✅ right: "Write 3x 5KB subsections incrementally"

  verification:
    ❌ wrong: "Assume file created successfully"
    ✅ right: "wc -l file.md after each append"
```

---

**Remember**: Small, incremental, linked files are ALWAYS better than large monolithic documents for both humans and AI agents!

---

**Document Version**: 1.0
**Last Updated**: 2026-01-11
**Feedback**: If you discover additional best practices, update this document
