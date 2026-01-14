# Q2 2026 Story Assignment Table - Developer Task Matrix

**Purpose**: Simple assignment table for Tech Leads to allocate stories to developers
**Date**: 2026-01-12
**Teams**: 2 Teams × 3 Developers = 6 Total

---

## 🎯 Quick Assignment Guide

### Team 1: Gemini Specialists

| Dev ID | Role | Sprint 1 (Weeks 1-2) | Sprint 2-3 (Weeks 3-6) |
|--------|------|----------------------|------------------------|
| **Dev 1A** | Senior Lead | Story 013-06 (2-3 days) | Stories 015-01, 015-04 |
| **Dev 1B** | Mid-Level | Stories 013-04, 013-05 | Stories 015-02, 015-03 |
| **Dev 1C** | Junior QA | Story 013-01 + QA | Story 015-03 + Testing |

### Team 2: Multi-Protocol Specialists

| Dev ID | Role | Sprint 1 (Weeks 1-3) | Sprint 2 (Week 4) | Sprint 3 (Weeks 5-6) |
|--------|------|----------------------|-------------------|----------------------|
| **Dev 2A** | Senior Security | Stories 024-01 (PAIR), 024-03 | Story 017-01 | Story 019-01 |
| **Dev 2B** | Mid-Level | Stories 024-01 (PAIR), 024-02 | Story 017-02 | Story 019-02 |
| **Dev 2C** | Junior Monitor | Story 024-04 | Story 017-03 | Story 019-03 |

---

## 📋 Detailed Story Assignment Matrix

### Sprint 1: Weeks 1-2 (March 17-28)

#### Team 1: Epic-013 (4 Stories)

| Story | Dev | Days | Priority | Dependencies | Files Modified |
|-------|-----|------|----------|--------------|----------------|
| **013-01** | Dev 1C | 1-2 | P1 | None | `tests/gemini_3/gemini_3_flash_*_tests.rs` (ADD 10+ tests) |
| **013-04** | Dev 1B | 1-2 | P2 | None | `modules/logger.rs`, `handlers/gemini.rs` (ADD structured logs) |
| **013-05** | Dev 1B | 2-3 | P3 | Story 013-04 | `proxy/cache.rs` (NEW), `handlers/gemini.rs` (MODIFY) |
| **013-06** | Dev 1A | 2-3 | P2 | None | `proxy/monitor.rs` (ADD analytics), `models/` (NEW types) |

**Week 1 Output**: Stories 013-01, 013-04, 013-06 COMPLETE
**Week 2 Output**: Story 013-05 COMPLETE, Epic-013 QA & Integration ✅

---

#### Team 2: Epic-024 (4 Stories)

| Story | Dev | Days | Priority | Dependencies | Files Modified |
|-------|-----|------|----------|--------------|----------------|
| **024-01** | Dev 2A + 2B (PAIR) | 4-5 | P0 | None | `mappers/*/models.rs` (ADD ideType to 15+ models) |
| **024-02** | Dev 2B | 3 | P0 | Story 024-01 | `mappers/*/request.rs` (ADD apiProvider logic) |
| **024-03** | Dev 2A | 3 | P1 | None | `upstream/client.rs` (ADD rotation), `config.rs` (NEW) |
| **024-04** | Dev 2C | 4-5 | P1 | None | `monitor.rs` (ADD detection metrics), `logger.rs` (ADD alerts) |

**Week 1 Output**: Story 024-01 60% (pair), Story 024-04 Part 1
**Week 2 Output**: Stories 024-01, 024-02, 024-03 COMPLETE, Story 024-04 Part 2
**Week 3 Output**: Story 024-04 COMPLETE, Epic-024 Validation ✅

---

### Sprint 2: Weeks 3-4 (March 31 - April 11)

#### Team 1: Epic-015 (3-4 Stories, TBD after discovery)

| Story | Dev | Days | Priority | Dependencies | Files Modified |
|-------|-----|------|----------|--------------|----------------|
| **015-01** | Dev 1A | 3-4 | P1 | Discovery Week 3 | TBD (Gemini Pro optimization) |
| **015-02** | Dev 1B | 3-4 | P1 | Discovery Week 3 | TBD (Gemini Pro features) |
| **015-03** | Dev 1C | 2-3 | P2 | Stories 015-01/02 | TBD (Pro testing) |

**Week 3 Output**: Discovery complete, Stories 015-01, 015-02 START
**Week 4 Output**: Stories 015-01, 015-02, 015-03 in progress

---

#### Team 2: Epic-024 Finish + Epic-017 (3 Stories)

| Story | Dev | Days | Priority | Dependencies | Files Modified |
|-------|-----|------|----------|--------------|----------------|
| **024-VAL** | Dev 2A | 2 | P0 | All 024 stories | Security audit, validation |
| **024-TEST** | Dev 2B | 2 | P0 | All 024 stories | Integration tests |
| **024-DASH** | Dev 2C | 2 | P1 | Story 024-04 | Monitoring dashboard |
| **017-01** | Dev 2A | 3-4 | P1 | Epic-024 complete | `mappers/claude/models.rs` (ADD sonnet std) |
| **017-02** | Dev 2B | 2-3 | P1 | Story 017-01 | `mappers/claude/request.rs` (ADD tool modes) |
| **017-03** | Dev 2C | 2 | P2 | Story 017-02 | `tests/claude/` (ADD 20+ tests) |

**Week 3 Output**: Epic-024 COMPLETE ✅, Epic-017 Discovery
**Week 4 Output**: Epic-017 COMPLETE ✅ (all 3 stories)

---

### Sprint 3: Weeks 5-6 (April 14 - May 2)

#### Team 1: Epic-015 Finish

| Story | Dev | Days | Priority | Dependencies | Files Modified |
|-------|-----|------|----------|--------------|----------------|
| **015-04** | Dev 1A | 3-4 | P1 | Stories 015-01/02/03 | TBD (final Pro features) |
| **015-INT** | All Team 1 | 2 | P1 | All 015 stories | Integration testing, validation |

**Week 5 Output**: Story 015-04 in progress, 015-03 COMPLETE
**Week 6 Output**: Epic-015 COMPLETE ✅

---

#### Team 2: Epic-019 (3 Stories)

| Story | Dev | Days | Priority | Dependencies | Files Modified |
|-------|-----|------|----------|--------------|----------------|
| **019-01** | Dev 2A | 3-4 | P1 | Epic-017 (90% reuse) | `mappers/claude/models.rs` (ADD opus std) |
| **019-02** | Dev 2B | 2-3 | P1 | Story 019-01 | `mappers/claude/request.rs` (ADD tool modes opus) |
| **019-03** | Dev 2C | 2 | P2 | Story 019-02 | `tests/claude/` (ADD 20+ opus tests) |

**Week 5 Output**: Stories 019-01, 019-02 in progress
**Week 6 Output**: Epic-019 COMPLETE ✅ (all 3 stories)

---

## 🔧 Story Details Reference

### Epic-013 Stories (READY)

#### Story-013-01: MEDIUM Level Test Coverage
```yaml
assignee: Dev 1C (Junior QA)
effort: "1-2 days"
priority: P1
files:
  - "tests/gemini_3/gemini_3_flash_openai_tests.rs" (ADD 4+ tests)
  - "tests/gemini_3/gemini_3_flash_claude_tests.rs" (ADD 4+ tests)
  - "tests/gemini_3/gemini_3_flash_gemini_tests.rs" (ADD 4+ tests)
acceptance:
  - "10+ tests for MEDIUM level (10001-20000 tokens)"
  - "Validate budget boundaries"
  - "Verify Flash exclusivity (Pro downgrades to LOW)"
  - "All tests passing (77/77+ at 100%)"
documentation: "docs/stories/Story-013-01-MEDIUM-level-test-coverage.md"
```

#### Story-013-04: Error Logging Enhancement
```yaml
assignee: Dev 1B (Mid-Level)
effort: "1-2 days"
priority: P2
files:
  - "src-tauri/src/modules/logger.rs" (MODIFY: structured logs)
  - "src-tauri/src/proxy/handlers/gemini.rs" (ADD log calls)
  - "src-tauri/src/proxy/mappers/gemini/*" (ADD log calls)
acceptance:
  - "Structured JSON logging for thinking errors"
  - "Include context: model, level, budget, error type, timestamp"
  - "Enable 10x faster debugging with searchable logs"
documentation: "docs/stories/Story-013-04-error-logging-enhancement.md"
```

#### Story-013-05: Caching Integration
```yaml
assignee: Dev 1B (Mid-Level)
effort: "2-3 days"
priority: P3 (OPTIONAL)
files:
  - "src-tauri/src/proxy/cache.rs" (NEW module)
  - "src-tauri/src/proxy/handlers/gemini.rs" (MODIFY: cache integration)
acceptance:
  - "Signature-based response caching implemented"
  - "30-50% reduction in duplicate API calls"
  - "Cache hit rate metrics available"
dependencies: "Story 013-04 (logging setup)"
documentation: "docs/stories/Story-013-05-caching-integration.md"
```

#### Story-013-06: Cost Analytics Dashboard
```yaml
assignee: Dev 1A (Senior Lead)
effort: "2-3 days"
priority: P2
files:
  - "src-tauri/src/proxy/monitor.rs" (ADD analytics module)
  - "src-tauri/src/models/" (ADD analytics types)
acceptance:
  - "Track request distribution across 4 thinking levels"
  - "Calculate cost per level based on token usage"
  - "Provide dashboard/API for cost analytics"
documentation: "docs/stories/Story-013-06-cost-analytics.md"
```

---

### Epic-024 Stories (TO BE CREATED)

#### Story-024-01: ideType Marker Addition
```yaml
assignee: Dev 2A + Dev 2B (PAIR PROGRAMMING)
effort: "4-5 days"
priority: P0 (CRITICAL)
files:
  - "src-tauri/src/proxy/mappers/claude/models.rs" (ADD ideType to 5+ models)
  - "src-tauri/src/proxy/mappers/gemini/models.rs" (ADD ideType to 5+ models)
  - "src-tauri/src/proxy/mappers/openai/models.rs" (ADD ideType to 5+ models)
acceptance:
  - "15+ models with ideType: 'ANTIGRAVITY' markers"
  - "All Claude/Gemini/OpenAI models covered"
  - "100% model protection from detection"
rationale: "PAIR PROGRAMMING (security critical, requires review)"
```

#### Story-024-02: apiProvider Field Completion
```yaml
assignee: Dev 2B (Mid-Level)
effort: "3 days"
priority: P0
files:
  - "src-tauri/src/proxy/mappers/claude/request.rs" (ADD apiProvider logic)
  - "src-tauri/src/proxy/mappers/gemini/request.rs" (ADD apiProvider logic)
  - "src-tauri/src/models/" (ADD apiProvider constants)
acceptance:
  - "All models have correct apiProvider IDs (e.g., 26 = ANTHROPIC_VERTEX)"
  - "Dynamic apiProvider selection based on model"
  - "Tests validate apiProvider correctness"
dependencies: "Story 024-01 (model markers complete)"
```

#### Story-024-03: User-Agent Rotation Strategy
```yaml
assignee: Dev 2A (Senior Security)
effort: "3 days"
priority: P1
files:
  - "src-tauri/src/proxy/upstream/client.rs" (MODIFY: rotation logic)
  - "src-tauri/src/proxy/config.rs" (ADD: user-agent pool config)
acceptance:
  - "User-agent rotation implemented (10+ agents in pool)"
  - "Random selection per request"
  - "Configurable rotation strategy"
  - "Metrics track rotation effectiveness"
```

#### Story-024-04: Detection Monitoring & Alerting
```yaml
assignee: Dev 2C (Junior Monitoring)
effort: "4-5 days (split across Week 1-2)"
priority: P1
files:
  - "src-tauri/src/proxy/monitor.rs" (ADD detection metrics)
  - "src-tauri/src/modules/logger.rs" (ADD detection alerts)
  - "tests/security/" (NEW: detection tests)
acceptance:
  - "Detection event logging (all detection attempts captured)"
  - "Alert thresholds configurable"
  - "Notification system operational"
  - "Monitoring dashboard shows detection stats"
```

---

### Epic-017 Stories (TO BE CREATED)

#### Story-017-01: Claude Sonnet Standard Core
```yaml
assignee: Dev 2A (Senior)
effort: "3-4 days"
priority: P1
files:
  - "src-tauri/src/proxy/mappers/claude/models.rs" (ADD sonnet standard)
  - "src-tauri/src/proxy/mappers/claude/request.rs" (MODIFY standard mode)
acceptance:
  - "modelId: 333 implemented"
  - "apiProvider: 26 (ANTHROPIC_VERTEX)"
  - "ideType: 'ANTIGRAVITY' marker"
  - "Sonnet standard model working (100% core features)"
dependencies: "Epic-024 complete (detection patterns available)"
```

#### Story-017-02: Tool Modes & Grounding Config
```yaml
assignee: Dev 2B (Mid-Level)
effort: "2-3 days"
priority: P1
files:
  - "src-tauri/src/proxy/mappers/claude/request.rs" (ADD tool modes)
  - "src-tauri/src/proxy/mappers/claude/grounding.rs" (NEW grounding config)
acceptance:
  - "Flexible tool modes: AUTO/ANY/NONE"
  - "Grounding config for Sonnet standard"
  - "Tool mode validation working"
dependencies: "Story 017-01 (core implementation)"
```

#### Story-017-03: Testing & Documentation
```yaml
assignee: Dev 2C (Junior)
effort: "2 days"
priority: P2
files:
  - "tests/claude/" (ADD sonnet standard tests)
  - "docs/comparison/claude-4-5-sonnet-COMPARISON.md" (UPDATE compliance)
acceptance:
  - "20+ tests for Sonnet standard mode"
  - "100% compliance validation"
  - "Documentation updated"
dependencies: "Stories 017-01, 017-02 (features complete)"
```

---

### Epic-019 Stories (TO BE CREATED)

#### Story-019-01: Claude Opus Standard Core
```yaml
assignee: Dev 2A (Senior)
effort: "3-4 days"
priority: P1
files:
  - "src-tauri/src/proxy/mappers/claude/models.rs" (ADD opus standard)
  - "src-tauri/src/proxy/mappers/claude/request.rs" (MODIFY opus handling)
acceptance:
  - "modelId implementation (opus standard)"
  - "apiProvider: 26 (ANTHROPIC_VERTEX)"
  - "ideType: 'ANTIGRAVITY' marker"
  - "90% code reuse from Epic-017 (copy-paste with model ID change)"
dependencies: "Epic-017 complete (pattern established)"
code_reuse: "Copy Sonnet patterns, change model ID to Opus"
```

#### Story-019-02: Tool Modes & Grounding (Opus)
```yaml
assignee: Dev 2B (Mid-Level)
effort: "2-3 days"
priority: P1
files:
  - "src-tauri/src/proxy/mappers/claude/request.rs" (EXTEND tool modes)
  - "src-tauri/src/proxy/mappers/claude/grounding.rs" (EXTEND grounding)
acceptance:
  - "Opus tool modes: AUTO/ANY/NONE"
  - "Grounding config for Opus standard"
  - "Same features as Sonnet (identical 5 gaps)"
dependencies: "Story 019-01 (core implementation)"
code_reuse: "Extend existing functions from Epic-017"
```

#### Story-019-03: Testing & Documentation (Opus)
```yaml
assignee: Dev 2C (Junior)
effort: "2 days"
priority: P2
files:
  - "tests/claude/" (ADD opus standard tests)
  - "docs/comparison/claude-opus-4-5-COMPARISON.md" (CREATE/UPDATE)
acceptance:
  - "20+ tests for Opus standard mode"
  - "100% compliance validation"
  - "Cross-model integration tests (Sonnet + Opus working together)"
dependencies: "Stories 019-01, 019-02 (features complete)"
```

---

## 🔀 Code Conflict Prevention

### Shared Files Strategy

| File | Team 1 Usage | Team 2 Usage | Conflict Mitigation |
|------|--------------|--------------|---------------------|
| **monitor.rs** | Analytics module (lines 1-200) | Detection module (lines 201-400) | **Separate sections**, merge end of Week 2 |
| **logger.rs** | Thinking log types | Security log types | **Different log categories**, parallel safe |
| **mappers/common/** | Read-only | Read-only (024), Write (017/019) | **Temporal separation**, Epic-024 first |

### Daily Coordination

```yaml
daily_standup:
  time: "9:30 AM"
  duration: "15 minutes"
  attendees: "Dev 1A (Team 1 Lead) + Dev 2A (Team 2 Lead)"
  agenda:
    - "Yesterday's progress"
    - "Today's focus"
    - "Blockers & shared file coordination"

merge_coordination:
  channel: "#team-merge-sync (Slack)"
  protocol: "Tag @team-lead before PR merge on shared files"
  review: "Cross-team PR review mandatory for monitor.rs, logger.rs"
```

---

## ✅ Assignment Checklist

### Week 1 Day 1 (Immediate)

- [ ] **Assign Developer Names**:
  - [ ] Dev 1A = _____________ (Team 1 Lead, Gemini Senior)
  - [ ] Dev 1B = _____________ (Team 1 Mid-Level)
  - [ ] Dev 1C = _____________ (Team 1 Junior QA)
  - [ ] Dev 2A = _____________ (Team 2 Lead, Security Senior)
  - [ ] Dev 2B = _____________ (Team 2 Mid-Level)
  - [ ] Dev 2C = _____________ (Team 2 Junior Monitoring)

- [ ] **Create Missing Story Files**:
  - [ ] Epic-024 stories (024-01, 024-02, 024-03, 024-04)
  - [ ] Epic-015 stories (after discovery Week 3)
  - [ ] Epic-017 stories (017-01, 017-02, 017-03)
  - [ ] Epic-019 stories (019-01, 019-02, 019-03)

- [ ] **Communication Setup**:
  - [ ] Daily standup calendar invite (9:30 AM)
  - [ ] Weekly demo calendar invite (Friday 3 PM)
  - [ ] Slack channel #team-merge-sync created

- [ ] **Begin Execution**:
  - [ ] Team 1: Epic-013 kickoff (already handed off)
  - [ ] Team 2: Epic-024 kickoff, pair programming setup

---

**Status**: ✅ READY FOR TEAM ASSIGNMENT
**Created**: 2026-01-12
**Next Action**: Fill in developer names, create Epic-024 story files

