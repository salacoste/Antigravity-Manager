# STORY QUOTA-001-04: Subscription Tier Detection - Implementation Plan

**Developer**: Dev 2 (Background Tasks & Subscription Tier Specialist)
**Duration**: 3 days (24 hours)
**Priority**: P1
**Status**: 🚀 READY TO START

---

## Day 1: API Implementation & Data Structures (8 hours)

### Morning (4 hours): Core Data Structures
- ✅ Review existing `ProxyToken.subscription_tier` field
- ✅ Create `SubscriptionTier` enum with priority scoring
- ✅ Implement `SubscriptionInfo` and `TierInfo` structures
- ✅ Add tier detection to `quota_fetcher.rs` (coordinate with Dev 1)

### Afternoon (4 hours): API Integration
- ⏳ Implement `load_code_assist()` in `quota_fetcher.rs`
- ⏳ Implement `detect_subscription_tier()` method
- ⏳ Add tier caching with TTL (24 hours)
- ⏳ Write unit tests for API parsing

---

## Day 2: Prioritization Logic (8 hours)

### Morning (4 hours): Account Prioritizer Module
- ⏳ Create `src-tauri/src/proxy/account_prioritizer.rs`
- ⏳ Implement `prioritize_accounts()` function
- ⏳ Implement multi-factor comparison (tier → quota → 429 time)
- ⏳ Add helper functions for tier/429 comparison

### Afternoon (4 hours): Integration with TokenManager
- ⏳ Update `TokenManager.get_token()` to use prioritization
- ⏳ Add quota-aware account selection
- ⏳ Implement 429 time tracking enhancement
- ⏳ Write integration tests

---

## Day 3: Testing & Documentation (8 hours)

### Morning (4 hours): Comprehensive Testing
- ⏳ Unit tests for tier detection (FREE/PRO/ULTRA)
- ⏳ Unit tests for prioritization logic
- ⏳ Integration tests with mock API responses
- ⏳ Test with real Google API (3 accounts: FREE, PRO, ULTRA)

### Afternoon (4 hours): Quality & Documentation
- ⏳ Performance testing (<100ms tier detection)
- ⏳ Cache validation (TTL correctness)
- ⏳ Code formatting and clippy fixes
- ⏳ Documentation and comments
- ⏳ Final review and PR preparation

---

## Success Criteria

### Functional Requirements
- [x] ProxyToken has `subscription_tier` field (already exists)
- [ ] `detect_subscription_tier()` correctly detects FREE/PRO/ULTRA
- [ ] `detect_tiers_batch()` processes multiple accounts in parallel
- [ ] `prioritize_accounts()` sorts by tier → quota → 429 time
- [ ] Tier cache works with 24-hour TTL
- [ ] Integration with TokenManager complete

### Performance Requirements
- [ ] Tier detection: <100ms (cached), <500ms (API)
- [ ] Prioritization: <10ms for 10 accounts
- [ ] Batch detection: <2s for 10 accounts (parallel)

### Quality Gates
- [ ] All unit tests passing (minimum 6 tests)
- [ ] Code compiles without warnings
- [ ] `cargo fmt` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] Test coverage >80%

---

## File Ownership

**Primary Files** (Dev 2 owns):
- `src-tauri/src/proxy/account_prioritizer.rs` (NEW, 100% ownership)

**Shared Files** (coordinate with Dev 1):
- `src-tauri/src/proxy/quota_fetcher.rs` (Dev 1 primary, Dev 2 adds tier section)
  - Dev 1: Lines 1-200 (API client core, `fetch_available_models()`)
  - Dev 2: Lines 201-400 (tier detection section, `load_code_assist()`)

**Integration Files** (preparation for Phase 2):
- `src-tauri/src/proxy/token_manager.rs` (already has tier sorting, line 232-240)
  - Phase 2 will add quota-aware selection logic

---

## Coordination Points

### With Dev 1 (API Integration)
**Day 1-2**: Dev 1 creates `quota_fetcher.rs` basic structure
**Day 3**: Dev 2 adds tier detection methods to separate section

**Handoff Marker** in `quota_fetcher.rs`:
```rust
// ============================================================================
// DEV 2: Subscription Tier Detection Section
// TODO: Dev 2 will add load_code_assist() and tier detection logic here
// ============================================================================
```

### With Dev 3 (Cache)
**Day 4**: Use Dev 3's `QuotaCache` for tier caching
**Zero conflicts**: Different files

---

## Risk Mitigation

### Medium Risk: API Response Format Changes
**Mitigation**: Use defensive parsing with fallbacks
**Contingency**: Default to FREE tier if parsing fails

### Low Risk: Prioritization Performance
**Mitigation**: Use efficient sorting with early termination
**Contingency**: Add benchmarks to validate <10ms target

---

## Next Steps After Completion

**Phase 2 Integration** (Week 2):
- Integrate prioritization into `TokenManager.get_token()`
- Add quota-based account selection (with Dev 1's quota data)
- Implement background tier refresh (with Dev 2's background monitor)

---

**Commit Message Template**:
```
feat(epic-001): implement subscription tier detection and prioritization (QUOTA-001-04)

- Add SubscriptionTier enum with priority scoring
- Implement load_code_assist() API call for tier detection
- Create account_prioritizer module with multi-factor sorting
- Add tier caching with 24-hour TTL
- Integrate with TokenManager for tier-aware account selection

Co-authored-by: Claude Sonnet 4.5 <noreply@anthropic.com>
```
