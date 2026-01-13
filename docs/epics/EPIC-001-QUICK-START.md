# Epic-001: Proactive Quota Monitoring - Quick Start Guide

**Status**: Ready for Implementation
**Priority**: P0 CRITICAL
**Team Size**: 3 Developers
**Duration**: 2-3 weeks (15 working days)
**Story Points**: 22 total

---

## Executive Summary

**Problem**: 15-20% 429 error rate due to reactive quota management
**Solution**: Proactive quota monitoring with background sync and tier-aware prioritization
**Impact**: 429 errors <3%, account switching <500ms, API success >95%

---

## Quick Architecture Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌────────────────┐
│  QuotaFetcher   │────▶│  QuotaManager    │────▶│  TokenManager  │
│  (API Client)   │     │  (Orchestrator)  │     │  (Integration) │
└─────────────────┘     └──────────────────┘     └────────────────┘
         │                       │                         │
         │                       ▼                         │
         │              ┌──────────────────┐              │
         └─────────────▶│   QuotaCache     │◀─────────────┘
                        │  (DashMap/TTL)   │
                        └──────────────────┘
                                 │
                                 ▼
                        ┌──────────────────┐
                        │ Background Task  │
                        │  (5-min sync)    │
                        └──────────────────┘
```

---

## Team Assignments

### Developer 1 (API Specialist)
- **Primary**: QUOTA-001-01 (Quota API Integration, 5 days, 5 SP)
- **Secondary**: QUOTA-001-02 (Integration, 4 days, 5 SP)
- **Files**: `quota_fetcher.rs` (new), `quota_manager.rs` (primary)
- **Total**: 9 days, 10 SP

### Developer 2 (Background/Tiers Specialist)
- **Primary**: QUOTA-001-04 (Tier Detection, 3 days, 3 SP)
- **Secondary**: QUOTA-001-03 (Background Monitor, 3 days, 3 SP)
- **Files**: `quota_fetcher.rs` (tier section), `quota_manager.rs` (background section)
- **Total**: 6 days, 6 SP

### Developer 3 (Cache/Integration Specialist)
- **Primary**: QUOTA-001-05 (Quota Cache, 3 days, 3 SP)
- **Secondary**: QUOTA-001-02 (TokenManager Integration, 4 days, 5 SP)
- **Files**: `quota_cache.rs` (new), `token_manager.rs` (integration)
- **Total**: 7 days, 8 SP

---

## Implementation Phases

### Phase 1: Foundation (Week 1) - FULLY PARALLEL
- **QUOTA-001-01**: Quota API Integration (Dev 1) - 5 days
- **QUOTA-001-04**: Subscription Tier Detection (Dev 2) - 3 days
- **QUOTA-001-05**: Quota Cache Implementation (Dev 3) - 3 days
- **Zero Dependencies**: All three can start simultaneously

### Phase 2: Integration (Week 2)
- **QUOTA-001-02**: TokenManager Integration (Dev 1 + Dev 3) - 4 days
  - Depends on: QUOTA-001-01, QUOTA-001-05
- **QUOTA-001-03**: Background Monitoring (Dev 2) - 3 days
  - Depends on: QUOTA-001-01, QUOTA-001-04, QUOTA-001-05

### Phase 3: UI (Week 3)
- **QUOTA-001-06**: Dashboard Integration (All devs) - 3 days
  - Depends on: All previous stories

---

## Key API Endpoints

### fetchAvailableModels
```bash
POST https://cloudcode-pa.googleapis.com/v1internal:fetchAvailableModels
Authorization: Bearer {access_token}
Content-Type: application/json

{"project": "{project_id}"}
```

**Response**:
```json
{
  "models": {
    "gemini-2.5-flash": {
      "displayName": "Gemini 2.5 Flash",
      "quotaInfo": {
        "remainingFraction": 0.87,
        "resetTime": "2026-01-11T00:00:00Z"
      }
    }
  }
}
```

### loadCodeAssist
```bash
POST https://cloudcode-pa.googleapis.com/v1internal:loadCodeAssist
Authorization: Bearer {access_token}
```

**Response**:
```json
{
  "subscriptionTier": "ULTRA",
  "features": ["gemini-2.5-flash", "gemini-2.5-pro"]
}
```

---

## Core Data Structures

### QuotaInfo
```rust
pub struct QuotaInfo {
    pub remaining_fraction: f64,    // 0.0 - 1.0
    pub reset_time: DateTime<Utc>,  // UTC midnight
    pub display_name: String,
}
```

### SubscriptionTier
```rust
pub enum SubscriptionTier {
    FREE,    // Basic quota, daily reset
    PRO,     // Higher quota, daily reset
    ULTRA,   // Highest quota, daily reset + advanced features
}
```

### QuotaDecision
```rust
pub enum QuotaDecision {
    Allow,                          // Quota sufficient
    Deny(String),                   // Quota exhausted, reason
    SwitchAccount(String),          // Switch recommended, target account
}
```

---

## File Ownership Matrix

| File | Primary Owner | Secondary Owner | Conflict Risk |
|------|---------------|-----------------|---------------|
| `quota_fetcher.rs` | Dev 1 | Dev 2 (tier section) | LOW |
| `quota_cache.rs` | Dev 3 | None | ZERO |
| `quota_manager.rs` | Dev 1 | Dev 2 (background) | MEDIUM |
| `token_manager.rs` | Dev 3 | Dev 1 (integration) | MEDIUM |
| `mod.rs` | All | All (different sections) | LOW |

---

## Daily Coordination Rituals

### Morning Standup (09:00, 15 min)
1. Yesterday's progress (2 min each)
2. Today's plan (2 min each)
3. Blockers/dependencies (5 min)
4. Shared file coordination (4 min)

### End-of-Day Sync (17:00, 15 min)
1. Completed work review
2. Tomorrow's file reservations
3. Integration point verification
4. Blocker resolution

### Weekly Demo (Friday 16:00, 45 min)
- Live demonstration of completed features
- Integration testing review
- Next week planning

---

## Success Criteria

### Performance Metrics
| Metric | Before | Target | Test Method |
|--------|--------|--------|-------------|
| 429 Error Rate | 15-20% | <3% | 1000 concurrent requests |
| Account Switch Latency | 2-5s | <500ms | Timestamp measurement |
| API Success Rate | 70-80% | >95% | 500 sequential requests |
| Quota Sync Latency | N/A | <2s | Background task monitoring |

### Quality Gates
- ✅ All 112+ tests passing (including new quota tests)
- ✅ Integration tests with real Google APIs
- ✅ Background monitor runs for 24h without errors
- ✅ Cache TTL respected across concurrent requests
- ✅ Subscription tier detection 100% accurate

---

## Risk Mitigation

### HIGH RISK: token_manager.rs Conflicts
**Prevention**:
- Dev 1 owns `check_quota_before_request()` integration
- Dev 3 owns account selection logic
- Paired programming sessions (Days 6-7)
- Daily end-of-day merge checks

### MEDIUM RISK: quota_manager.rs Conflicts
**Prevention**:
- Dev 1: API orchestration methods (top of file)
- Dev 2: Background monitor logic (bottom of file)
- Daily standup coordination
- Clear section boundaries with comments

### LOW RISK: API Rate Limiting During Development
**Prevention**:
- Use 3 separate test accounts
- 5-minute intervals for background sync (not 1-minute)
- Mock responses for unit tests

---

## Testing Strategy

### Unit Tests (Dev Responsibility)
```rust
// quota_fetcher.rs (Dev 1)
#[tokio::test]
async fn test_fetch_available_models_success()
#[tokio::test]
async fn test_load_code_assist_tiers()
#[tokio::test]
async fn test_api_error_handling()

// quota_cache.rs (Dev 3)
#[test]
fn test_cache_set_and_get()
#[test]
fn test_cache_expiration()
#[test]
fn test_concurrent_access()

// quota_manager.rs (Dev 1 + Dev 2)
#[tokio::test]
async fn test_check_quota_allow()
#[tokio::test]
async fn test_check_quota_switch_account()
#[tokio::test]
async fn test_background_monitor_updates()
```

### Integration Tests (Week 3, All Devs)
```rust
#[tokio::test]
async fn test_end_to_end_quota_validation()
#[tokio::test]
async fn test_background_sync_integration()
#[tokio::test]
async fn test_tier_based_prioritization()
#[tokio::test]
async fn test_token_manager_integration()
```

---

## Command Quick Reference

### Run Tests
```bash
# All tests
cargo test

# Specific module
cargo test --lib quota_fetcher
cargo test --lib quota_cache
cargo test --lib quota_manager

# Integration tests
cargo test --test integration_quota
```

### Format & Lint
```bash
# Format
cargo fmt

# Lint
cargo clippy -- -D warnings

# Type check
cargo check
```

### Development Build
```bash
# Backend only
cd src-tauri && cargo build

# Full app
npm run tauri dev
```

---

## Key Documentation References

1. **Epic Document**: `docs/epics/Epic-001-Proactive-Quota-Monitoring.md`
2. **Implementation Plan**: `docs/epics/EPIC-001-IMPLEMENTATION-PLAN.md`
3. **Team Coordination**: `docs/epics/EPIC-001-TEAM-COORDINATION-PLAN.md`
4. **API Reverse Engineering**: `docs/antigravity/api/quota-apis.md`
5. **Comparison Docs**: `docs/comparison/MASTER-MODELS-TABLE.md`

---

## Day-by-Day Schedule Overview

### Week 1: Foundation (Days 1-5)
- **Day 1**: Environment setup, API client skeleton
- **Day 2**: fetchAvailableModels implementation
- **Day 3**: loadCodeAssist + cache structure
- **Day 4**: Tier detection + cache operations
- **Day 5**: API integration complete, cache complete

### Week 2: Integration (Days 6-10)
- **Day 6**: TokenManager integration start (paired)
- **Day 7**: Pre-request validation logic
- **Day 8**: Background monitor implementation
- **Day 9**: Integration testing
- **Day 10**: Bug fixes + optimization

### Week 3: UI & Polish (Days 11-15)
- **Day 11**: Dashboard quota widgets
- **Day 12**: Real-time UI updates
- **Day 13**: Comprehensive integration tests
- **Day 14**: Bug fixes + edge case handling
- **Day 15**: Demo preparation + documentation

---

## Emergency Contact & Escalation

### Blockers
1. **Technical Blocker**: Escalate in daily standup
2. **API Access Issue**: Contact operations team immediately
3. **Merge Conflict**: Request paired programming session
4. **Test Failure**: Coordinate with story owner

### Code Review Process
- Pull requests created at end of each story
- Minimum 1 reviewer from team
- All tests must pass before merge
- Review within 24 hours

---

## Success Celebration Criteria 🎉

- ✅ 429 error rate dropped to <3%
- ✅ All 6 stories completed and merged
- ✅ 112+ tests passing (including new quota tests)
- ✅ Dashboard shows real-time quota status
- ✅ Background monitor runs reliably for 7+ days
- ✅ Zero production incidents during first week

---

**Last Updated**: 2026-03-21
**Epic**: QUOTA-001 Proactive Quota Monitoring
**Team**: 3 Developers (API, Background/Tiers, Cache/Integration)
**Timeline**: 15 working days (2-3 weeks)
