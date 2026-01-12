# Q2 2026 Team Allocation Plan - Optimal Parallel Execution

**Planning Date**: 2026-01-12
**Resource**: 2 Teams, 3 Developers Each (6 Total)
**Timeline**: 12 Weeks (March 17 - June 6, 2026)
**Objective**: Maximize velocity, minimize conflicts, zero blockers

---

## 🎯 Resource Configuration

### Team 1: Gemini Specialists
**Size**: 3 Developers
**Focus**: Gemini model family implementation
**Primary Skills**: Rust, Gemini API, thinking protocols
**Code Ownership**: `proxy/mappers/gemini/`, `proxy/handlers/gemini.rs`, Gemini tests

```yaml
team_1_developers:
  dev_1a: "Senior Developer (Story Lead)"
  dev_1b: "Mid-Level Developer (Implementation)"
  dev_1c: "Junior Developer (Testing & QA Support)"
```

### Team 2: Multi-Protocol Specialists
**Size**: 3 Developers
**Focus**: Cross-protocol features, Claude/OpenAI, security
**Primary Skills**: Rust, OpenAI/Anthropic APIs, security hardening
**Code Ownership**: `proxy/mappers/claude/`, `proxy/mappers/openai/`, `proxy/mappers/common/`, security features

```yaml
team_2_developers:
  dev_2a: "Senior Developer (Security & Architecture)"
  dev_2b: "Mid-Level Developer (Protocol Implementation)"
  dev_2c: "Junior Developer (Monitoring & Analytics)"
```

---

## 📊 Epic Allocation Strategy

### Allocation Principles

1. **Isolation First**: Epic-024 (Anti-Detection) runs FIRST to avoid merge conflicts
2. **Parallel Safe**: Epic-013 (Flash) parallel with Epic-024 (different code areas)
3. **Sequential Dependencies**: Epic-017/019 (Claude) AFTER Epic-024 (use detection patterns)
4. **Story-Level Parallelism**: 3 devs per team → 2-3 stories in parallel per sprint
5. **Code Conflict Matrix**: Each epic mapped to file paths for conflict detection

### Epic-to-Team Mapping

```yaml
team_1_epics:
  - epic_013: "Gemini 3 Flash Compliance (Phases 2+3)"
  - epic_015: "Gemini 2.5 Pro Optimization"

team_2_epics:
  - epic_024: "Anti-Detection Hardening (CRITICAL)"
  - epic_017: "Claude 4.5 Sonnet Standard Mode"
  - epic_019: "Claude 4.5 Opus Standard Mode"
```

---

## 🗓️ Week-by-Week Execution Plan

### **Weeks 1-2** (March 17-28) - Sprint 1

#### Team 1: Epic-013 - Gemini 3 Flash (4 Stories, 6-10 Days)

**Dev 1A (Story Lead)**:
```yaml
week_1:
  story: "Story-013-06: Cost Analytics Dashboard"
  effort: "2-3 days"
  files:
    - "src-tauri/src/proxy/monitor.rs" (NEW: analytics module)
    - "src-tauri/src/models/" (NEW: analytics types)
  deliverable: "Cost tracking API + level distribution analytics"

week_2:
  story: "Story-013-06 FINISH + Code Review"
  support: "Review Dev 1B/1C code"
  deliverable: "Analytics dashboard working, team code review"
```

**Dev 1B (Implementation)**:
```yaml
week_1:
  story: "Story-013-04: Error Logging Enhancement"
  effort: "1-2 days"
  files:
    - "src-tauri/src/modules/logger.rs" (MODIFY: structured logs)
    - "src-tauri/src/proxy/handlers/gemini.rs" (ADD: log calls)
    - "src-tauri/src/proxy/mappers/gemini/*" (ADD: log calls)
  deliverable: "Structured JSON logging for thinking errors"

week_2:
  story: "Story-013-05: Caching Integration (START)"
  effort: "2-3 days total"
  files:
    - "src-tauri/src/proxy/cache.rs" (NEW: cache module)
    - "src-tauri/src/proxy/handlers/gemini.rs" (MODIFY: cache integration)
  deliverable: "Cache module implemented, signature-based caching"
```

**Dev 1C (Testing & QA)**:
```yaml
week_1:
  story: "Story-013-01: MEDIUM Level Test Coverage"
  effort: "1-2 days"
  files:
    - "tests/gemini_3/gemini_3_flash_openai_tests.rs" (ADD: 4+ tests)
    - "tests/gemini_3/gemini_3_flash_claude_tests.rs" (ADD: 4+ tests)
    - "tests/gemini_3/gemini_3_flash_gemini_tests.rs" (ADD: 4+ tests)
  deliverable: "10+ MEDIUM level tests across 3 protocols, 77/77+ passing"

week_2:
  story: "Epic-013 QA & Integration Testing"
  focus: "Regression testing, integration validation"
  deliverable: "All 4 stories validated, 77/77+ tests passing, Epic-013 COMPLETE"
```

**Team 1 Code Conflict Risk**: 🟢 LOW
- Dev 1A: NEW files (monitor.rs, analytics)
- Dev 1B: Gemini-specific handlers/mappers
- Dev 1C: Test files only
- **No overlapping files** ✅

---

#### Team 2: Epic-024 - Anti-Detection Hardening (4 Stories, 23-25 Hours)

**Epic-024 Story Breakdown** (estimated):
```yaml
story_024_01:
  title: "ideType Marker Addition (15+ Models)"
  effort: "8-10 hours"
  priority: "P0"

story_024_02:
  title: "apiProvider Field Completion"
  effort: "6-8 hours"
  priority: "P0"

story_024_03:
  title: "User-Agent Rotation Strategy"
  effort: "5-7 hours"
  priority: "P1"

story_024_04:
  title: "Detection Monitoring & Alerting"
  effort: "4-5 hours"
  priority: "P1"
```

**Dev 2A (Security Lead)**:
```yaml
week_1:
  story: "Story-024-01: ideType Marker Addition (Part 1)"
  effort: "4-5 days (Dev 2A + 2B pair)"
  files:
    - "src-tauri/src/proxy/mappers/claude/models.rs" (ADD: ideType to 5+ models)
    - "src-tauri/src/proxy/mappers/gemini/models.rs" (ADD: ideType to 5+ models)
    - "src-tauri/src/proxy/mappers/openai/models.rs" (ADD: ideType to 5+ models)
  deliverable: "15+ models with ideType: 'ANTIGRAVITY' markers"

week_2:
  story: "Story-024-03: User-Agent Rotation Strategy"
  effort: "3 days"
  files:
    - "src-tauri/src/proxy/upstream/client.rs" (MODIFY: rotation logic)
    - "src-tauri/src/proxy/config.rs" (ADD: user-agent pool config)
  deliverable: "User-agent rotation implemented, 10+ agents in pool"
```

**Dev 2B (Protocol Implementation)**:
```yaml
week_1:
  story: "Story-024-01: ideType Marker Addition (Part 2 - Pairing)"
  effort: "4-5 days (pair with Dev 2A)"
  files: "Same as Dev 2A (pair programming for critical security)"
  deliverable: "Joint delivery with Dev 2A"

week_2:
  story: "Story-024-02: apiProvider Field Completion"
  effort: "3 days"
  files:
    - "src-tauri/src/proxy/mappers/claude/request.rs" (ADD: apiProvider logic)
    - "src-tauri/src/proxy/mappers/gemini/request.rs" (ADD: apiProvider logic)
    - "src-tauri/src/models/" (ADD: apiProvider constants)
  deliverable: "All models have correct apiProvider IDs (26, 32, etc.)"
```

**Dev 2C (Monitoring & Analytics)**:
```yaml
week_1:
  story: "Story-024-04: Detection Monitoring & Alerting (Part 1)"
  effort: "2 days"
  files:
    - "src-tauri/src/proxy/monitor.rs" (ADD: detection metrics)
    - "src-tauri/src/modules/logger.rs" (ADD: detection alerts)
  deliverable: "Detection event logging, basic metrics collection"

week_2:
  story: "Story-024-04: Detection Monitoring & Alerting (Part 2)"
  effort: "2-3 days"
  files:
    - "src-tauri/src/proxy/monitor.rs" (ADD: alerting logic)
    - "tests/security/" (NEW: detection tests)
  deliverable: "Alert thresholds, notification system, monitoring tests"
```

**Team 2 Code Conflict Risk**: 🟡 MEDIUM (Week 1), 🟢 LOW (Week 2)
- Week 1: Dev 2A + 2B pair on same files (intentional - security critical)
- Week 2: Separate file ownership (client.rs vs request.rs vs monitor.rs)
- **Mitigation**: Pair programming Week 1, daily sync meetings

---

### **Weeks 3-4** (March 31 - April 11) - Sprint 2

#### Team 1: Epic-015 - Gemini 2.5 Pro Optimization (START)

**Epic-015 Estimated Scope**:
```yaml
effort: "2-3 weeks total"
stories: "3-4 stories (TBD - needs COMPARISON analysis)"
focus:
  - "Extended thinking optimization"
  - "Pro tier features"
  - "Cost efficiency improvements"
```

**Dev 1A**:
```yaml
weeks_3_4:
  activity: "Epic-015 Discovery & Planning"
  tasks:
    - "Read gemini-2-5-pro-thinking-COMPARISON.md"
    - "Identify P0/P1 gaps"
    - "Create story files (4 stories estimated)"
    - "Begin Story 015-01 implementation"
  files: "TBD after gap analysis"
```

**Dev 1B + 1C**:
```yaml
weeks_3_4:
  activity: "Epic-015 Parallel Implementation"
  coordination: "Dev 1B = Story 015-02, Dev 1C = Testing + Story 015-03"
  deliverable: "2-3 stories in parallel"
```

**Team 1 Notes**:
- Epic-013 COMPLETE end of Week 2
- Epic-015 starts Week 3 with clean slate
- Full team available (3 developers)

---

#### Team 2: Epic-024 FINISH + Epic-017 START

**Dev 2A (Security)**:
```yaml
week_3:
  story: "Epic-024 Final Validation & Security Audit"
  effort: "2 days"
  deliverable: "All 15+ models verified, security checklist complete"

  handoff: "Epic-017 Discovery (START)"
  task: "Read claude-4-5-sonnet-COMPARISON.md, analyze gaps"

week_4:
  story: "Epic-017-01: Claude Sonnet Standard - Core Implementation"
  effort: "3-4 days (9-11 hours total for Epic-017)"
  files:
    - "src-tauri/src/proxy/mappers/claude/models.rs" (ADD: sonnet standard)
    - "src-tauri/src/proxy/mappers/claude/request.rs" (MODIFY: standard mode)
  deliverable: "modelId 333, apiProvider 26, ideType markers"
```

**Dev 2B (Protocol)**:
```yaml
week_3:
  story: "Epic-024 Integration Testing"
  effort: "2 days"
  deliverable: "Cross-protocol tests, rotation validation"

  handoff: "Epic-017 Preparation"

week_4:
  story: "Epic-017-02: Tool Modes & Grounding Config"
  effort: "2-3 days"
  files:
    - "src-tauri/src/proxy/mappers/claude/request.rs" (ADD: tool modes AUTO/ANY/NONE)
    - "src-tauri/src/proxy/mappers/claude/grounding.rs" (NEW: grounding config)
  deliverable: "Flexible tool modes, grounding config for Sonnet standard"
```

**Dev 2C (Monitoring)**:
```yaml
week_3:
  story: "Epic-024 Monitoring Dashboard"
  effort: "2 days"
  deliverable: "Detection metrics dashboard, alerting UI"

week_4:
  story: "Epic-017-03: Testing & Documentation"
  effort: "2 days"
  files:
    - "tests/claude/" (ADD: sonnet standard tests)
    - "docs/comparison/" (UPDATE: compliance)
  deliverable: "20+ tests for Sonnet standard, 100% compliance"
```

**Epic-024 Completion**: End of Week 3 ✅
**Epic-017 Start**: Mid-Week 3
**Epic-017 Completion**: End of Week 4 ✅

---

### **Weeks 5-7** (April 14 - May 2) - Sprint 3

#### Team 1: Epic-015 FINISH

```yaml
weeks_5_7:
  team_1_focus: "Complete Epic-015 Gemini 2.5 Pro Optimization"
  estimated_completion: "Week 6 end"

  dev_1a: "Story 015-04 + Final validation"
  dev_1b: "Story 015-03 + Integration testing"
  dev_1c: "Epic-015 comprehensive test suite"

  week_7_activity: "Buffer, code review, documentation cleanup"
```

---

#### Team 2: Epic-019 - Claude 4.5 Opus Standard Mode

**Epic-019 Details**:
```yaml
effort: "9-11 hours (same as Epic-017)"
code_reuse: "90% (identical 5 gaps as Sonnet)"
stories: "3 stories (mirror Epic-017 structure)"
  story_019_01: "Core Implementation (modelId, apiProvider, ideType)"
  story_019_02: "Tool Modes & Grounding Config"
  story_019_03: "Testing & Documentation"
```

**Dev 2A**:
```yaml
weeks_5_6:
  story: "Story-019-01: Opus Standard Core Implementation"
  effort: "3-4 days"
  files:
    - "src-tauri/src/proxy/mappers/claude/models.rs" (ADD: opus standard)
    - "src-tauri/src/proxy/mappers/claude/request.rs" (MODIFY: opus handling)
  deliverable: "Opus standard with modelId, apiProvider, ideType"

  code_reuse: "Copy Sonnet patterns from Epic-017, change model ID"

week_7:
  activity: "Epic-019 Final Validation"
  deliverable: "100% compliance for Claude 4.5 Opus standard"
```

**Dev 2B**:
```yaml
weeks_5_6:
  story: "Story-019-02: Tool Modes & Grounding (Opus)"
  effort: "2-3 days"
  files: "Same as Epic-017 (extend existing functions)"
  deliverable: "Opus tool modes + grounding config"

week_7:
  activity: "Cross-Model Integration Testing"
  deliverable: "Sonnet + Opus working together, no regressions"
```

**Dev 2C**:
```yaml
weeks_5_6:
  story: "Story-019-03: Opus Testing & Documentation"
  effort: "2 days"
  files:
    - "tests/claude/" (ADD: opus standard tests)
    - "docs/comparison/" (UPDATE: opus compliance)
  deliverable: "20+ tests, 100% compliance documentation"

week_7:
  activity: "Q2 Completion Report & Metrics"
  deliverable: "Q2 delivered epics summary, metrics, success criteria validation"
```

**Epic-019 Completion**: End of Week 6 ✅

---

## 🔀 Code Conflict Matrix

### File Ownership Map

```yaml
team_1_ownership:
  exclusive:
    - "src-tauri/src/proxy/mappers/gemini/**" ✅
    - "src-tauri/src/proxy/handlers/gemini.rs" ✅
    - "tests/gemini_3/**" ✅
    - "src-tauri/src/proxy/cache.rs" (NEW in Epic-013) ✅

  shared_write:
    - "src-tauri/src/proxy/monitor.rs" (Epic-013 + Epic-024) ⚠️
    - "src-tauri/src/modules/logger.rs" (Epic-013 + Epic-024) ⚠️

team_2_ownership:
  exclusive:
    - "src-tauri/src/proxy/mappers/claude/**" ✅
    - "src-tauri/src/proxy/mappers/openai/**" ✅
    - "src-tauri/src/proxy/handlers/claude.rs" ✅
    - "src-tauri/src/proxy/upstream/client.rs" (Epic-024) ✅
    - "tests/claude/**" ✅
    - "tests/security/**" (NEW in Epic-024) ✅

  shared_write:
    - "src-tauri/src/proxy/monitor.rs" (Epic-024) ⚠️
    - "src-tauri/src/modules/logger.rs" (Epic-024) ⚠️
    - "src-tauri/src/proxy/mappers/common/**" (used by both) ⚠️

shared_files_strategy:
  monitor_rs:
    mitigation: "Team 1 adds analytics module (lines 1-200), Team 2 adds detection module (lines 201-400)"
    conflict_risk: "🟢 LOW (separate functions)"

  logger_rs:
    mitigation: "Team 1 adds thinking log types, Team 2 adds security log types"
    conflict_risk: "🟢 LOW (different log categories)"

  common_mappers:
    mitigation: "Read-only for Epic-013/024, write access for Team 2 only in Epic-017/019"
    conflict_risk: "🟢 LOW (temporal separation)"
```

### Conflict Mitigation Strategy

1. **Temporal Separation**:
   - Epic-024 (Team 2) writes to `monitor.rs` Week 1-3
   - Epic-013 (Team 1) writes to `monitor.rs` Week 1-2
   - **Solution**: Different sections of file, merge end of Week 2

2. **Functional Separation**:
   - Team 1: Analytics functions (`track_level_distribution()`, `calculate_cost()`)
   - Team 2: Detection functions (`log_detection_event()`, `check_rotation()`)
   - **No overlapping function names** ✅

3. **Daily Sync**:
   - 15-min standup daily between Team 1 Lead (Dev 1A) and Team 2 Lead (Dev 2A)
   - Shared Slack channel for merge coordination
   - PR review cross-team for shared files

---

## 📈 Velocity & Risk Analysis

### Team 1 Velocity

```yaml
sprint_1: "Epic-013 (4 stories, 6-10 days) with 3 devs"
  velocity: "1.2-1.7 stories/week/dev"
  completion: "Week 2 end ✅"

sprint_2_3: "Epic-015 (3-4 stories, 2-3 weeks) with 3 devs"
  velocity: "1.0-1.3 stories/week/dev"
  completion: "Week 6 end ✅"

total_output: "7-8 stories in 6 weeks"
utilization: "85-90% (includes buffer)"
```

### Team 2 Velocity

```yaml
sprint_1: "Epic-024 (4 stories, 23-25 hours = 3-3.5 weeks) with 3 devs"
  velocity: "Pair programming Week 1 (security critical)"
  completion: "Week 3 end ✅"

sprint_2: "Epic-017 (3 stories, 9-11 hours = 1.5 weeks) with 3 devs"
  velocity: "Fast turnaround (code reuse patterns established)"
  completion: "Week 4 end ✅"

sprint_3: "Epic-019 (3 stories, 9-11 hours = 1.5 weeks) with 3 devs"
  velocity: "90% code reuse from Epic-017"
  completion: "Week 6 end ✅"

total_output: "10 stories in 6 weeks"
utilization: "90-95% (high-priority critical path)"
```

### Risk Assessment

```yaml
team_1_risks:
  - risk: "Epic-015 scope uncertainty (COMPARISON analysis needed)"
    probability: "MEDIUM"
    mitigation: "Buffer Week 7, can defer 1 story to Q3"
    impact: "LOW (Epic-015 is optimization, not critical)"

team_2_risks:
  - risk: "Epic-024 security critical, can't rush"
    probability: "LOW"
    mitigation: "Pair programming, extra week if needed (Week 4 buffer)"
    impact: "MEDIUM (blocks Epic-017/019 if delayed)"

  - risk: "Claude models (017/019) depend on Epic-024 patterns"
    probability: "LOW"
    mitigation: "Epic-024 complete Week 3, 1-week buffer before Epic-017"
    impact: "LOW (clear dependency, no parallel work)"

merge_conflicts:
  probability: "LOW (10-15%)"
  mitigation: "Daily sync, separate file sections, cross-team PR reviews"
  impact: "VERY LOW (2-4 hours to resolve if occurs)"
```

---

## ✅ Success Criteria & Checkpoints

### Weekly Checkpoints

```yaml
week_2_checkpoint:
  team_1: "Epic-013 COMPLETE, 77/77+ tests passing ✅"
  team_2: "Epic-024 60% complete, ideType markers done ✅"

week_3_checkpoint:
  team_1: "Epic-015 discovery complete, 2+ stories in progress ✅"
  team_2: "Epic-024 COMPLETE, Epic-017 discovery started ✅"

week_4_checkpoint:
  team_1: "Epic-015 50%+ complete ✅"
  team_2: "Epic-017 COMPLETE, 100% compliance ✅"

week_6_checkpoint:
  team_1: "Epic-015 COMPLETE ✅"
  team_2: "Epic-019 COMPLETE ✅"
```

### Q2 Final Deliverables (Week 6 End)

```yaml
team_1_delivered:
  - epic_013: "Gemini 3 Flash 95%+ compliance ✅"
  - epic_015: "Gemini 2.5 Pro optimization ✅"
  total_stories: "7-8 stories"
  total_effort: "4-5 weeks"

team_2_delivered:
  - epic_024: "Anti-Detection hardening (ALL models protected) ✅"
  - epic_017: "Claude 4.5 Sonnet standard mode (100%) ✅"
  - epic_019: "Claude 4.5 Opus standard mode (100%) ✅"
  total_stories: "10 stories"
  total_effort: "6 weeks"

combined_value:
  epics_completed: 5
  total_stories: "17-18 stories"
  critical_wins:
    - "🛡️ Anti-Detection: ALL models protected from service unavailability"
    - "🚀 Claude 4.5: Sonnet + Opus standard modes (high-demand revenue models)"
    - "⚡ Gemini 3.x: Flash 95%+, Pro optimization (market completeness)"

  business_impact:
    risk_mitigation: "P0 CRITICAL - Detection protection deployed"
    revenue_growth: "P1 HIGH - Claude 4.5 completeness unlocks premium tier"
    product_maturity: "95%+ compliance across flagship models"
```

---

## 🚀 Execution Recommendations

### Team Leads Responsibilities

**Dev 1A (Team 1 Lead)**:
- Daily sync with Team 2 Lead (Dev 2A) on shared files
- Epic-015 discovery and story creation (Week 3)
- Code review coordination
- Weekly velocity tracking

**Dev 2A (Team 2 Lead)**:
- Security audit ownership (Epic-024)
- Cross-epic dependency management (024 → 017 → 019)
- Pair programming coordination (Week 1)
- Daily sync with Team 1 Lead

### Communication Protocol

```yaml
daily_standup:
  time: "9:30 AM daily"
  duration: "15 minutes"
  attendees: "Dev 1A + Dev 2A (leads) + optional team members"
  focus: "Blockers, shared file coordination, velocity check"

weekly_demo:
  time: "Friday 3 PM"
  duration: "30 minutes"
  attendees: "All 6 developers + Product Manager (Ivan)"
  focus: "Sprint progress, checkpoint validation, next week planning"

merge_coordination:
  channel: "#team-merge-sync (Slack)"
  notification: "Tag @team-lead before PR merge on shared files"
  review: "Cross-team PR review mandatory for monitor.rs, logger.rs"
```

### Buffer Utilization

```yaml
week_7_buffer:
  team_1: "Documentation, code quality, Epic-015 polish"
  team_2: "Q2 metrics report, Epic-019 validation, Q3 planning prep"

week_8_10_buffer:
  purpose: "Q3 preparation, vacation coverage, technical debt cleanup"
  flexible: "Can start Epic-014 (Audio) if teams ready early"
```

---

## 📋 Next Steps

1. **Immediate** (Week 1 Start):
   - [ ] Assign developers to teams (Team 1 vs Team 2)
   - [ ] Create Epic-024 story files (4 stories detailed)
   - [ ] Team 1: Kick off Epic-013 (already handed off)
   - [ ] Team 2: Kick off Epic-024 (pair programming setup)

2. **Week 3** (Epic-015 Start):
   - [ ] Team 1 Lead: Complete Epic-015 discovery
   - [ ] Create Epic-015 story files (3-4 stories)
   - [ ] Validate Epic-024 completion before Epic-017 start

3. **Ongoing**:
   - [ ] Daily standup (leads)
   - [ ] Weekly demo (all devs + PM)
   - [ ] Merge coordination on shared files
   - [ ] Velocity tracking and checkpoint validation

---

**Status**: ✅ READY FOR EXECUTION
**Approval Required**: Product Manager (Ivan)
**Next Action**: Assign developers to Team 1 vs Team 2, create Epic-024 story files

---

**Document Version**: 1.0 FINAL
**Created**: 2026-01-12
**Last Updated**: 2026-01-12
