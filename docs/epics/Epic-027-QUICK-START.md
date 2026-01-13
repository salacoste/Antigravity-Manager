# Epic-027: Upstream Integration - Quick Start

**Created**: 2026-01-14
**Purpose**: Fast reference for Epic-027 execution

---

## What Is This Epic?

**Objective**: Integrate 74 upstream commits (v3.3.21-v3.3.27) into our v3.5.0 codebase.

**Approach**: "Reverse Documentation"
1. ✅ First: Document upstream features as Stories (DONE)
2. Next: Execute merge and integrate code
3. Finally: Update Stories with actual implementation details

---

## 11 Stories Overview

| ID | Story | Priority | Risk | Effort |
|----|-------|----------|------|--------|
| 027-01 | Aggressive Context Scaling | HIGH | 🟢 LOW | 2h |
| 027-02 | Session Signature Caching | HIGH | 🟡 MED | 3h |
| 027-03 | Stable Session IDs | MED | 🟢 LOW | 1h |
| 027-04 | Headless Deployment | MED | 🟢 LOW | 2h |
| 027-05 | Post-Install Guidance | LOW | 🟢 LOW | 1h |
| 027-06 | Stream Error i18n | MED | 🟡 MED | 2h |
| 027-07 | Traditional Chinese | LOW | 🟢 LOW | 1h |
| 027-08 | MCP Compatibility | HIGH | 🟢 LOW | 2h |
| 027-09 | Update Notifications | MED | 🟢 LOW | 2h |
| 027-10 | Device Binding | HIGH | 🔴 HIGH | 3h |
| 027-11 | Smart Warmup | MED | 🟡 MED | 2h |

**Total**: 21 hours

---

## Critical Information

### ⚠️ Our Modules (Will NOT be deleted)

These modules exist ONLY in our codebase and will be preserved:
- `budget_detector.rs`, `budget_optimizer.rs`
- `model_selector.rs`
- `quota_cache.rs`, `quota_fetcher.rs`, `quota_manager.rs`, `quota_monitor.rs`
- `signature_cache.rs` (we have our own!)
- `thinking_quality.rs`, `weekly_tuner.rs`

**Strategy**: During merge, explicitly keep these files with `git add <file>`

### ✅ Upstream New Modules (Will be added)

New files from upstream to integrate:
- `modules/device.rs` - Device binding for fraud prevention (423 lines)
- `deploy/*.sh` - Headless deployment scripts
- `locales/zh-TW.json` - Traditional Chinese

**Strategy**: Copy from upstream with `git checkout upstream/main -- <file>`

### 🔴 High-Conflict Files (Manual merge required)

1. **proxy_db.rs** - Database schema
   - Our version: +1155 lines (budget, quota, signature cache tables)
   - Upstream: different structure
   - **Action**: Merge schemas carefully, test migrations

2. **signature_cache.rs** - Caching logic
   - We have our own implementation
   - Upstream has session-based caching
   - **Action**: Compare, merge best of both

3. **i18n.rs + locales/*.json** - Translations
   - We added quota/budget keys
   - Upstream added stream error keys + zh-TW
   - **Action**: Merge all translation keys

4. **account.rs** - Account management
   - Upstream: +703 lines (device binding integration)
   - Our version: quota monitoring integration
   - **Action**: Merge both features

---

## Execution Flow

### Phase 1: Backup (Stage 1.1-1.4) - 30-45 min
- Create backup branch
- Verify current state builds
- Analyze upstream changes

### Phase 2: Merge Analysis (Stage 2.1-2.3) - 30-45 min
- Dry-run merge
- Categorize conflicts
- Create resolution checklist

### Phase 3: Conflict Resolution (Stage 3.1-3.8) - 3-5 hours ⚠️
- Execute merge
- Resolve conflicts per category
- Test after each critical file
- **Story Documentation**: Update each Story file during resolution

### Phase 4: Testing (Stage 4.1-4.6) - 2-3 hours ⚠️
- Backend + frontend builds
- All tests pass
- Manual integration testing
- **Story Documentation**: Document test results in Stories

### Phase 5: Finalization (Stage 5.1-5.9) - 1-2 hours
- Cleanup
- Create Story-027-XX.md files
- Commit with Epic-027 reference
- Tag release v3.5.1

---

## Story Documentation Protocol

**During Merge**: As each feature is integrated, update the corresponding Story file.

### Story File Template

Create: `docs/stories/Story-027-XX-<feature-name>.md`

**Sections to Add**:
1. **Implementation Details**: Actual files modified, conflict resolution approach
2. **Testing Results**: Test commands, pass/fail status, performance metrics
3. **Integration Notes**: How feature works with our existing code
4. **Acceptance Criteria**: Check off each criterion as completed

**Example**:
```markdown
# Story-027-01: Aggressive Context Scaling

## Implementation Details

**Files Modified**:
- src-tauri/src/modules/config.rs: Added `ContextScaling` struct
- src-tauri/src/proxy/server.rs: Applied scaling logic to request handler
- src/pages/Settings.tsx: Added UI controls for scaling config

**Conflict Resolution**:
- config.rs: Merged our config fields with upstream scaling fields
- No conflicts in other files (additive changes)

**Code Changes**:
```rust
pub struct ContextScaling {
    pub enabled: bool,
    pub max_tokens: u32,
    pub scale_factor: f32,
}
```

## Testing Results

**Build Test**: ✅ PASS
```bash
cargo build --manifest-path src-tauri/Cargo.toml
# Build succeeded in 45s
```

**Unit Tests**: ✅ PASS
```bash
cargo test config::
# 5 tests passed
```

**Integration Test**: ✅ PASS
- Started application
- Navigated to Settings → Proxy Configuration
- Context Scaling toggle visible
- Changed max_tokens, saved successfully
- Restarted app, settings persisted

**Performance**:
- Response time with scaling: 250ms (baseline: 280ms)
- Memory usage: +2MB (acceptable)

## Integration Notes

- Compatible with our proxy system
- No conflicts with quota monitoring
- Scaling applies before our budget detection

## Acceptance Criteria

- [x] Context scaling config struct defined
- [x] Scaling applied to proxy requests
- [x] UI controls for scaling settings
- [x] Documentation in Settings page

**Status**: ✅ COMPLETE
**Integration Date**: 2026-01-15
**Implemented By**: Amelia (Developer Agent)
```

---

## Key Commands

### Quick Reference

```bash
# Backup
git checkout -b backup/pre-upstream-sync-2026-01-14

# Start merge
git checkout -b feat/upstream-sync-v3.3.27
git merge --no-commit --no-ff upstream/main

# Keep our files (no conflicts)
git add src-tauri/src/modules/budget_*.rs
git add src-tauri/src/modules/quota_*.rs

# Copy upstream new files
git checkout upstream/main -- src-tauri/src/modules/device.rs
git add src-tauri/src/modules/device.rs

# Resolve conflicts manually
# Edit files, then:
git add <resolved-file>

# Build test
cargo build --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml

# Frontend test
npm run build

# Commit
git commit -m "feat(epic-027): integrate upstream v3.3.27 features

Stories implemented:
- Story-027-01: Context scaling
- Story-027-02: Session signature caching
- ... (list all)

Refs: Epic-027"
```

---

## Success Checklist

**Before Merge**:
- [ ] Epic-027 documentation reviewed
- [ ] All 11 Stories understood
- [ ] Operational plan ready

**During Merge**:
- [ ] All conflicts resolved
- [ ] No conflict markers remain
- [ ] Backend builds successfully
- [ ] All tests pass
- [ ] Frontend builds successfully

**After Merge**:
- [ ] All 11 Stories have implementation docs
- [ ] Story-027-XX.md files created
- [ ] Epic-027 completion report written
- [ ] All Epic-001 to Epic-026 features work
- [ ] Release tagged v3.5.1

---

## Emergency Contacts

**Rollback Command**:
```bash
git reset --hard backup/pre-upstream-sync-2026-01-14
```

**Critical Stop Points**:
1. After proxy_db.rs merge → Test database operations
2. After signature_cache.rs merge → Test cache hit rate
3. After device.rs integration → Test account operations
4. After full merge → Run complete test suite

---

## Next Action

**Ready to start?**

Execute: `git checkout main && git checkout -b backup/pre-upstream-sync-2026-01-14`

Then follow: `docs/UPSTREAM-SYNC-OPERATIONAL-PLAN.md`

Reference this file for Story documentation protocol.

---

**Epic-027 Status**: 📋 Documentation Complete, Ready to Execute
**Operational Plan**: docs/UPSTREAM-SYNC-OPERATIONAL-PLAN.md
**Full Epic Details**: docs/epics/Epic-027-Upstream-Integration.md
