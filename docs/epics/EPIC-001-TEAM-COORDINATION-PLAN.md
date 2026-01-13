# Epic-001: Team Coordination Plan - Proactive Quota Monitoring

**Epic ID**: QUOTA-001
**Team Size**: 3 Developers
**Duration**: 2-3 weeks (Mar 22 - Apr 11, 2026)
**Coordination Strategy**: Parallel execution with zero-conflict design

---

## 👥 Team Structure & Assignments

### Developer 1: API Integration Lead

**Primary Responsibility**: Quota API client and manager core logic

**Stories Assigned**:
- 🔴 QUOTA-001-01: Quota API Integration (5 days, P0)
- 🔴 QUOTA-001-02: Pre-Request Quota Validation (1.5 days with Dev 3, P0)
- 🟡 QUOTA-001-03: Background Monitoring Configuration (1 day with Dev 2, P1)

**Total Effort**: 7.5 days

**Code Ownership**:
- `src-tauri/src/proxy/quota_fetcher.rs` (new, primary owner)
- `src-tauri/src/proxy/quota_manager.rs` (new, primary owner)
- `src-tauri/src/proxy/token_manager.rs` (modifications, shared with Dev 3)

---

### Developer 2: Background Services & Tier Detection Lead

**Primary Responsibility**: Background monitoring and subscription tier logic

**Stories Assigned**:
- 🟡 QUOTA-001-04: Subscription Tier Detection (3 days, P1)
- 🟡 QUOTA-001-03: Background Quota Monitoring (4 days with Dev 1, P1)
- 🟢 QUOTA-001-06: Tauri Commands for UI (1 day with Dev 3, P2)

**Total Effort**: 8 days

**Code Ownership**:
- `src-tauri/src/proxy/quota_fetcher.rs` (tier detection section, shared)
- `src-tauri/src/proxy/quota_manager.rs` (background monitor, shared)
- `src-tauri/src/commands/quota.rs` (new, primary owner)
- `src-tauri/src/models/app_config.rs` (quota config section)

---

### Developer 3: Cache & Integration Lead

**Primary Responsibility**: Cache implementation and TokenManager integration

**Stories Assigned**:
- 🔴 QUOTA-001-05: Quota Cache Implementation (3 days, P0)
- 🔴 QUOTA-001-02: Pre-Request Validation Integration (1.5 days with Dev 1, P0)
- 🟢 QUOTA-001-06: Quota Health UI Indicators (2 days, P2)

**Total Effort**: 6.5 days

**Code Ownership**:
- `src-tauri/src/proxy/quota_cache.rs` (new, primary owner)
- `src-tauri/src/proxy/token_manager.rs` (integration, shared with Dev 1)
- `src/components/quota/QuotaIndicator.tsx` (new, primary owner)
- `src/stores/useQuotaStore.ts` (modifications)

---

## 🗓️ Detailed Week-by-Week Plan

### Week 1 (Mar 22-28): Foundation Phase - PARALLEL EXECUTION

#### Day 1 (Mar 22) - Kickoff & Setup

**Morning (All Devs - 2 hours)**:
```yaml
09:00-10:00: "Team kickoff meeting"
  - Review Epic-001 scope and success metrics
  - Review implementation plan
  - Discuss technical architecture
  - Clarify questions

10:00-11:00: "Development environment setup"
  - Create epic-001-quota-monitoring branch
  - Review quota-apis.md reverse engineering docs
  - Set up testing infrastructure
```

**Afternoon (Parallel Work - 6 hours)**:
```yaml
dev_1:
  - "Create quota_fetcher.rs skeleton"
  - "Implement FetchModelsRequest/Response structs"
  - "Start fetch_available_models() implementation"

dev_2:
  - "Create subscription tier structures (TierInfo, SubscriptionInfo)"
  - "Start load_code_assist() API implementation"
  - "Review tier detection logic from quota-apis.md"

dev_3:
  - "Create quota_cache.rs skeleton"
  - "Implement CachedQuotaInfo with TTL"
  - "Start QuotaCache with DashMap"
```

**End of Day Sync (30 min)**:
- Share progress and blockers
- Review interfaces between modules
- Plan tomorrow's work

---

#### Day 2 (Mar 23) - Core Implementation

**Dev 1 (8 hours)**:
```yaml
tasks:
  - "Complete fetch_available_models() implementation"
  - "Add authentication headers (Bearer token)"
  - "Parse JSON response to HashMap<String, ModelQuotaInfo>"
  - "Add logging (info, debug, error)"

deliverables:
  - "Working API client (untested)"
  - "Data structures complete"
```

**Dev 2 (8 hours)**:
```yaml
tasks:
  - "Complete load_code_assist() implementation"
  - "Implement get_subscription_tier() extraction logic"
  - "Create SubscriptionTier enum (Free/Pro/Ultra)"
  - "Start prioritize_accounts() function"

deliverables:
  - "Tier detection API client"
  - "Account prioritization skeleton"
```

**Dev 3 (8 hours)**:
```yaml
tasks:
  - "Complete QuotaCache get/set operations"
  - "Implement TTL validation (is_valid, is_stale)"
  - "Add batch operations (set_all)"
  - "Implement clear_account, clear_all"

deliverables:
  - "Functional quota cache"
  - "Thread-safe operations with DashMap"
```

**End of Day Sync**: Review interfaces, ensure compatibility

---

#### Day 3 (Mar 24) - Error Handling & Testing

**Dev 1 (8 hours)**:
```yaml
tasks:
  - "Implement error handling (401, 403, 429, 500)"
  - "Add retry logic for transient errors"
  - "Add timeout handling"
  - "Write unit tests for API client"

deliverables:
  - "Robust error handling"
  - "Unit tests (mock responses)"
```

**Dev 2 (8 hours)**:
```yaml
tasks:
  - "Complete prioritize_accounts() with tier sorting"
  - "Write unit tests for tier detection"
  - "Write unit tests for account prioritization"
  - "Integration testing with mock API"

deliverables:
  - "QUOTA-001-04 COMPLETE ✅"
  - "Unit tests passing"
```

**Dev 3 (8 hours)**:
```yaml
tasks:
  - "Write unit tests for cache operations"
  - "Test TTL expiration logic"
  - "Test concurrent access (thread safety)"
  - "Performance benchmarks (<10ms operations)"

deliverables:
  - "QUOTA-001-05 COMPLETE ✅"
  - "Unit tests passing, performance validated"
```

**Milestone**: ✅ Dev 2 and Dev 3 complete their Phase 1 stories!

---

#### Day 4 (Mar 25) - Integration Prep

**Dev 1 (8 hours)**:
```yaml
tasks:
  - "Complete unit tests for quota_fetcher"
  - "Integration tests with real API (dev account)"
  - "Performance testing (<200ms API call)"
  - "Code review and documentation"

deliverables:
  - "QUOTA-001-01 COMPLETE ✅"
  - "Integration tests passing"
```

**Dev 2 (8 hours)**:
```yaml
tasks:
  - "Start QUOTA-001-03: Background monitoring skeleton"
  - "Create background task with tokio::spawn"
  - "Implement interval-based refresh (5 minutes)"
  - "Add refresh_all_quotas() skeleton"

deliverables:
  - "Background task structure ready"
```

**Dev 3 (8 hours)**:
```yaml
tasks:
  - "Start QUOTA-001-02: Create quota_manager.rs"
  - "Implement QuotaManager struct"
  - "Implement check_quota() with cache-first strategy"
  - "Implement make_decision() threshold logic"

deliverables:
  - "QuotaManager core logic ready"
```

**Milestone**: ✅ All Phase 1 stories COMPLETE! Ready for integration.

---

#### Day 5 (Mar 26) - Integration Work

**Dev 1 + Dev 3 (Paired - 8 hours)**:
```yaml
tasks:
  - "Integrate QuotaManager into TokenManager"
  - "Modify get_token() to call check_quota()"
  - "Implement account fallback logic"
  - "Add quota-based account switching"

deliverables:
  - "Pre-request quota validation working"
```

**Dev 2 (8 hours)**:
```yaml
tasks:
  - "Complete refresh_all_quotas() with parallelization"
  - "Add error handling for background task"
  - "Add retry logic for failed refreshes"
  - "Unit tests for background monitor"

deliverables:
  - "Background refresh logic complete"
```

**End of Week 1 Sync (1 hour)**:
- Demo progress to Product Owner
- Review integration points
- Plan Week 2 work

---

### Week 2 (Mar 29 - Apr 4): Integration & Testing

#### Day 1 (Mar 29) - Configuration & Lifecycle

**Dev 1 (8 hours)**:
```yaml
tasks:
  - "Add QuotaConfig to AppConfig"
  - "Implement config loading in proxy startup"
  - "Add QuotaManager initialization"
  - "Start background monitor on startup"

deliverables:
  - "Configuration system ready"
```

**Dev 2 (8 hours)**:
```yaml
tasks:
  - "Add task lifecycle management (start/stop)"
  - "Implement graceful shutdown for monitor"
  - "Add monitoring metrics (success/fail counts)"
  - "Integration tests"

deliverables:
  - "Background task lifecycle complete"
```

**Dev 3 (8 hours)**:
```yaml
tasks:
  - "Integration testing for pre-request validation"
  - "Test account switching scenarios"
  - "Test low quota warnings"
  - "Performance testing (<50ms pre-check)"

deliverables:
  - "QUOTA-001-02 Integration tests passing"
```

---

#### Day 2 (Mar 30) - End-to-End Testing

**All Devs (Parallel - 8 hours each)**:
```yaml
dev_1:
  - "E2E test: Fetch quota → Cache → Pre-request check"
  - "E2E test: API error handling"
  - "Performance profiling"

dev_2:
  - "E2E test: Background monitoring updates cache"
  - "Stability testing (4-hour run)"
  - "CPU/memory profiling"

dev_3:
  - "E2E test: Account switching on low quota"
  - "E2E test: All accounts exhausted scenario"
  - "Integration with existing TokenManager"

milestone: "All core E2E tests passing"
```

---

#### Day 3 (Mar 31) - Bug Fixes & Optimization

**All Devs (8 hours each)**:
```yaml
focus: "Address bugs found in E2E testing"

common_tasks:
  - Fix integration issues
  - Performance optimization
  - Code review
  - Refactoring

deliverables:
  - "QUOTA-001-03 COMPLETE ✅"
  - "All integration tests passing"
```

---

#### Day 4-5 (Apr 1-2) - 48-Hour Stability Test

**Dev 1 (16 hours)**:
```yaml
tasks:
  - "Start 48-hour stability test"
  - "Monitor CPU usage (<1% target)"
  - "Monitor memory usage (<10MB target)"
  - "Check for memory leaks"
  - "Performance profiling"

deliverables:
  - "Stability test results"
  - "Performance benchmark report"
```

**Dev 2 + Dev 3 (16 hours each)**:
```yaml
tasks:
  - "Start QUOTA-001-06 UI work"
  - "Create QuotaIndicator.tsx component"
  - "Add Tauri commands (get_quota_info)"
  - "Integrate into existing account UI"

deliverables:
  - "UI components ready"
```

**Milestone**: ✅ Phase 2 Complete - Core functionality stable

---

### Week 3 (Apr 5-11): UI & Final Polish

#### Day 1-2 (Apr 5-6) - UI Development

**Dev 2 (16 hours)**:
```yaml
tasks:
  - "Implement get_quota_info Tauri command"
  - "Implement get_all_account_quotas command"
  - "Add Tauri events for quota updates"
  - "Unit tests for commands"

deliverables:
  - "Backend commands ready"
```

**Dev 3 (16 hours)**:
```yaml
tasks:
  - "Complete QuotaIndicator component"
  - "Add color-coded health indicators"
  - "Format reset time display (local timezone)"
  - "Add real-time updates (1-minute refresh)"
  - "Integrate into AccountCard components"

deliverables:
  - "QUOTA-001-06 COMPLETE ✅"
  - "UI displaying quota health"
```

**Dev 1 (16 hours)**:
```yaml
tasks:
  - "Monitor 48-hour stability test"
  - "Collect performance metrics"
  - "Fix any issues found"
  - "Prepare stability report"

deliverables:
  - "48-hour stability PASSED ✅"
```

---

#### Day 3-5 (Apr 7-11) - Documentation, Testing & Deployment Prep

**All Devs (24 hours each)**:
```yaml
day_3:
  all_devs: "Final integration testing, bug fixes"

day_4:
  dev_1: "Technical documentation (ADR, API reference)"
  dev_2: "User documentation (README, guides)"
  dev_3: "UI testing, screenshots, user guide"

day_5:
  all_devs: "Beta testing preparation, deployment checklist"

final_deliverables:
  - "All 6 stories COMPLETE ✅"
  - "All tests passing (unit + integration)"
  - "Documentation complete"
  - "Beta release candidate ready"
```

**Milestone**: 🎉 EPIC-001 COMPLETE - READY FOR BETA DEPLOYMENT

---

## 🔄 Daily Coordination Rituals

### Daily Standup (15 minutes @ 09:00)

**Format**:
```yaml
each_developer_shares:
  - "Yesterday: What I completed"
  - "Today: What I'm working on"
  - "Blockers: Any issues or dependencies"

team_lead_actions:
  - "Resolve blockers"
  - "Adjust assignments if needed"
  - "Update progress tracker"
```

### End of Day Sync (15 minutes @ 17:00)

**Format**:
```yaml
each_developer_shares:
  - "Completed tasks"
  - "Code ready for review"
  - "Tomorrow's plan"

team_actions:
  - "Cross-review interfaces"
  - "Verify no integration conflicts"
  - "Update shared progress board"
```

### Weekly Demo (Friday @ 16:00 - 1 hour)

**Format**:
```yaml
week_1_demo:
  - "Demo quota API fetching"
  - "Demo cache operations"
  - "Demo tier detection"
  - "Review Week 2 plan"

week_2_demo:
  - "Demo pre-request quota validation"
  - "Demo background monitoring"
  - "Demo account switching"
  - "Review Week 3 plan"

week_3_demo:
  - "Demo complete system"
  - "Show UI quota indicators"
  - "Present metrics (429 rate improvement)"
  - "Beta deployment readiness"
```

---

## 🛡️ Zero-Conflict Strategy

### File Ownership Matrix

| File | Primary Owner | Secondary | Conflicts Risk |
|------|---------------|-----------|----------------|
| `quota_fetcher.rs` | Dev 1 | Dev 2 (tier section) | 🟢 LOW |
| `quota_cache.rs` | Dev 3 | - | 🟢 ZERO |
| `quota_manager.rs` | Dev 1 | Dev 2 (bg monitor) | 🟡 MEDIUM |
| `token_manager.rs` | Dev 3 | Dev 1 (integration) | 🟡 MEDIUM |
| `commands/quota.rs` | Dev 2 | Dev 3 (UI commands) | 🟢 LOW |
| `app_config.rs` | Dev 2 | - | 🟢 LOW |
| `QuotaIndicator.tsx` | Dev 3 | - | 🟢 ZERO |

### Conflict Mitigation

**Medium Risk Files** (`quota_manager.rs`, `token_manager.rs`):

```yaml
strategy:
  - "Section-based development"
  - "Daily interface reviews"
  - "Paired programming for critical sections"

quota_manager_rs:
  dev_1_section: "// Section 1: QuotaManager core + check_quota()"
  dev_2_section: "// Section 2: Background monitoring + refresh_all_quotas()"

token_manager_rs:
  dev_1_section: "// Section 1: Quota decision logic"
  dev_3_section: "// Section 2: Token selection integration"

coordination:
  - "Daily sync on shared file changes"
  - "Commit message prefix: [Dev1], [Dev2], [Dev3]"
  - "Cross-review PRs before merge"
```

---

## 📊 Progress Tracking

### Story Completion Checklist

#### Phase 1 (Week 1)
- [ ] **QUOTA-001-01** (Dev 1): Quota API Integration
  - [ ] Day 1: API client skeleton
  - [ ] Day 2: Core implementation
  - [ ] Day 3: Error handling
  - [ ] Day 4: Unit tests
  - [ ] Day 5: Integration tests
  - [ ] ✅ COMPLETE

- [ ] **QUOTA-001-04** (Dev 2): Subscription Tier Detection
  - [ ] Day 1: Tier API implementation
  - [ ] Day 2: Prioritization logic
  - [ ] Day 3: Testing & integration
  - [ ] ✅ COMPLETE

- [ ] **QUOTA-001-05** (Dev 3): Quota Cache Implementation
  - [ ] Day 1: Core cache operations
  - [ ] Day 2: Advanced features
  - [ ] Day 3: Testing & optimization
  - [ ] ✅ COMPLETE

#### Phase 2 (Week 1-2)
- [ ] **QUOTA-001-02** (Dev 1 + Dev 3): Pre-Request Quota Validation
  - [ ] Day 4-5 (Week 1): QuotaManager + TokenManager integration
  - [ ] Day 1 (Week 2): Integration testing
  - [ ] ✅ COMPLETE

- [ ] **QUOTA-001-03** (Dev 2 + Dev 1): Background Quota Monitoring
  - [ ] Day 4-5 (Week 1): Background task implementation
  - [ ] Day 1-2 (Week 2): Configuration + lifecycle
  - [ ] Day 3 (Week 2): Testing
  - [ ] ✅ COMPLETE

#### Phase 3 (Week 2-3)
- [ ] **QUOTA-001-06** (Dev 3 + Dev 2): Quota Health UI Indicators
  - [ ] Day 4-5 (Week 2): UI components + Tauri commands
  - [ ] Day 1-2 (Week 3): Integration + testing
  - [ ] ✅ COMPLETE

---

### Testing Checklist

#### Unit Tests (Throughout)
- [ ] quota_fetcher.rs: >80% coverage
- [ ] quota_cache.rs: >90% coverage
- [ ] quota_manager.rs: >80% coverage
- [ ] token_manager.rs: >80% coverage (quota sections)
- [ ] commands/quota.rs: >75% coverage

#### Integration Tests (Week 2)
- [ ] End-to-end quota fetch → cache → validation
- [ ] Account switching on low quota
- [ ] Background monitoring updates cache
- [ ] UI displays current quota
- [ ] Error scenarios (403, 429, timeout)

#### Performance Tests (Week 2)
- [ ] Quota check latency <50ms (cached, p95)
- [ ] Quota check latency <200ms (API, p95)
- [ ] Cache operations <10ms (p99)
- [ ] Background task CPU <1% average
- [ ] Background task memory <10MB

#### Stability Tests (Week 2-3)
- [ ] 48-hour stability run (no crashes)
- [ ] No memory leaks detected
- [ ] CPU usage stable
- [ ] Cache size bounded (<1000 entries)

---

## 🎯 Success Criteria - Epic Level

### Pre-Launch (Week 3)

**Must Have (P0)**:
- [ ] All 6 stories completed
- [ ] 429 error rate <5% (beta target)
- [ ] Pre-request quota validation functional
- [ ] Background monitoring runs 48+ hours stable
- [ ] No performance regression (<5% latency increase)
- [ ] Unit test coverage >80%
- [ ] Integration tests passing

**Should Have (P1)**:
- [ ] Subscription tier detection working
- [ ] Quota cache hit rate >80%
- [ ] Documentation complete
- [ ] Beta deployment ready

### Post-Launch (30 days)

**Quantitative**:
- [ ] 429 Error Rate: <3%
- [ ] Account Switch Latency: <500ms
- [ ] API Success Rate: >95%
- [ ] User-reported issues: <5/week

**Qualitative**:
- [ ] Positive user feedback
- [ ] No critical bugs
- [ ] Engineering team satisfied with code quality

---

## 📞 Communication Channels

### Primary Channels

```yaml
daily_communication:
  - "Slack: #epic-001-quota-monitoring"
  - "Standups: 09:00 daily (15 min)"
  - "End of day sync: 17:00 daily (15 min)"

code_review:
  - "GitHub PRs: Require 1 approval from other dev"
  - "Review SLA: <4 hours during work day"

blockers:
  - "Post in Slack immediately"
  - "Tag @team-lead for urgent issues"

documentation:
  - "Update progress in epic-001-progress-tracker.md daily"
  - "Document decisions in ADRs"
```

---

## 🚨 Risk Management

### Critical Risks

**Risk 1: API Rate Limiting on Quota Checks**
```yaml
probability: "Medium"
impact: "Medium"
mitigation:
  - "5-min cache TTL (reduces API calls by 99%)"
  - "Batch quota checks where possible"
contingency:
  - "Increase cache TTL to 10 minutes"
  - "Reduce background refresh frequency to 10 minutes"

owner: "Dev 1"
monitoring: "Track API call rate in logs"
```

**Risk 2: Quota API Unavailable**
```yaml
probability: "Low"
impact: "High"
mitigation:
  - "Fallback to reactive 429 handling"
  - "Graceful degradation (use cached data)"
contingency:
  - "Continue with existing rate limit tracking"
  - "Disable background monitoring if persistent failures"

owner: "Dev 1"
monitoring: "Alert on API error rate >10%"
```

**Risk 3: Integration Conflicts in TokenManager**
```yaml
probability: "Medium"
impact: "Medium"
mitigation:
  - "Paired programming (Dev 1 + Dev 3)"
  - "Daily interface reviews"
  - "Section-based development"
contingency:
  - "Use feature flag to enable/disable quota validation"
  - "Implement as separate code path initially"

owner: "Dev 1 + Dev 3"
monitoring: "Daily code review of shared files"
```

---

## 📈 Metrics & Monitoring

### Development Metrics (Track Daily)

```yaml
velocity:
  - "Story points completed per day"
  - "Target: 1.5-2 points/day (22 points / 15 days)"

quality:
  - "Unit test coverage: >80%"
  - "Integration test coverage: >75%"
  - "Code review turnaround: <4 hours"

blockers:
  - "Count of blockers per day"
  - "Average resolution time: <8 hours"
```

### Production Metrics (Post-Launch)

```yaml
primary_kpis:
  - "429 Error Rate: Track daily"
  - "Account Switch Latency: p50/p95/p99"
  - "API Success Rate: Track hourly"

secondary_kpis:
  - "Quota fetch success rate"
  - "Cache hit rate"
  - "Background task uptime"
  - "Quota refresh cycle duration"
```

---

## 🎯 Definition of Done

### Story Level
```yaml
code:
  - [ ] Implementation complete
  - [ ] Code reviewed and approved
  - [ ] No lint errors or warnings

testing:
  - [ ] Unit tests passing
  - [ ] Integration tests passing
  - [ ] Test coverage >80%

documentation:
  - [ ] Code comments added
  - [ ] API documentation updated
  - [ ] User guide updated (if user-facing)
```

### Epic Level
```yaml
functionality:
  - [ ] All 6 stories complete
  - [ ] All acceptance criteria met
  - [ ] No P0/P1 bugs

quality:
  - [ ] 429 error rate <5% (beta)
  - [ ] Performance targets met
  - [ ] Stability test passed (48h)

deployment:
  - [ ] Beta release candidate ready
  - [ ] Documentation complete
  - [ ] Rollback plan documented
```

---

## 📋 Pre-Implementation Checklist

### Before Starting (Mar 22)

**Team Preparation**:
- [ ] All devs read Epic-001-Proactive-Quota-Monitoring.md
- [ ] All devs read quota-apis.md (reverse engineering)
- [ ] All devs read EPIC-001-IMPLEMENTATION-PLAN.md (this doc)
- [ ] All devs understand dependency graph

**Environment Setup**:
- [ ] Create epic-001-quota-monitoring branch
- [ ] Set up testing infrastructure
- [ ] Configure development accounts (Free + Pro tier)
- [ ] Set up monitoring/logging tools

**Technical Prep**:
- [ ] Review TokenManager current implementation
- [ ] Review existing quota.rs in modules (account quota)
- [ ] Understand ProxyToken structure
- [ ] Review DashMap documentation

**Communication**:
- [ ] Create #epic-001-quota-monitoring Slack channel
- [ ] Schedule daily standups (09:00)
- [ ] Schedule weekly demos (Friday 16:00)
- [ ] Set up progress tracking board

---

## ✅ Handoff Criteria for Next Epic

**When Epic-001 is complete**:
```yaml
deliverables:
  - [ ] All 6 stories implemented and tested
  - [ ] 429 error rate <3% validated
  - [ ] Documentation complete
  - [ ] Deployed to beta channel
  - [ ] Beta testing positive feedback

team_readiness:
  - [ ] Code merged to main branch
  - [ ] Team available for next epic
  - [ ] Lessons learned documented
  - [ ] Technical debt items logged

next_epic_prep:
  - [ ] Team 1 remains for next Gemini work
  - [ ] Identify next P0/P1 epic from roadmap
  - [ ] Begin prep phase for next epic
```

---

**Coordination Plan Version**: 1.0
**Prepared By**: Engineering Team Lead
**Status**: ✅ READY FOR KICKOFF (Mar 22, 2026)
**Expected Completion**: Apr 11, 2026
