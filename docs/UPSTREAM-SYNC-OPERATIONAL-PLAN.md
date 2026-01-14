# Upstream Sync - Operational Execution Plan

**Date**: 2026-01-14
**Executor**: Amelia (Developer Agent)
**Target**: Sync upstream v3.3.27 → our v3.5.0 (preserve all features)

---

## PHASE 1: BACKUP & PREPARATION 🛡️

### Stage 1.1: Create Backup Branch ✅
**Objective**: Save current state before any changes

**Commands**:
```bash
git checkout main
git checkout -b backup/pre-upstream-sync-2026-01-14
git push origin backup/pre-upstream-sync-2026-01-14
```

**Success Criteria**:
- ✅ Backup branch created
- ✅ Backup pushed to origin
- ✅ Verify: `git log -1` shows latest commit

**Checkpoint 1.1**: STOP → Verify backup exists remotely

---

### Stage 1.2: Create Working Branch ✅
**Objective**: Create isolated branch for merge work

**Commands**:
```bash
git checkout main
git checkout -b feat/upstream-sync-v3.3.27
```

**Success Criteria**:
- ✅ Working branch created from main
- ✅ Current branch: `feat/upstream-sync-v3.3.27`

**Checkpoint 1.2**: Confirm branch created

---

### Stage 1.3: Verify Current State ✅
**Objective**: Document starting point

**Commands**:
```bash
# Current version
grep '"version"' package.json
grep '^version' src-tauri/Cargo.toml

# Build status
cargo build --manifest-path src-tauri/Cargo.toml
npm run build

# Test status
cargo test --manifest-path src-tauri/Cargo.toml
```

**Success Criteria**:
- ✅ Version confirmed: 3.5.0
- ✅ Current code builds successfully
- ✅ All tests pass

**Checkpoint 1.3**: STOP → Confirm all builds and tests pass

---

### Stage 1.4: Analyze Upstream Changes in Detail 📊
**Objective**: Understand what we're merging

**Commands**:
```bash
# List all changed files
git diff --name-status upstream/main main > /tmp/upstream-diff-files.txt

# Categorize changes
echo "=== NEW FILES (upstream only) ===" > /tmp/upstream-analysis.txt
git diff --name-only --diff-filter=A upstream/main main >> /tmp/upstream-analysis.txt

echo "\n=== MODIFIED FILES (both changed) ===" >> /tmp/upstream-analysis.txt
git diff --name-only --diff-filter=M upstream/main main >> /tmp/upstream-analysis.txt

echo "\n=== DELETED FILES (removed in upstream) ===" >> /tmp/upstream-analysis.txt
git diff --name-only --diff-filter=D upstream/main main >> /tmp/upstream-analysis.txt

# Review upstream commits
git log --oneline --graph upstream/main ^main --since="2024-12-01" > /tmp/upstream-commits.txt
```

**Success Criteria**:
- ✅ File analysis complete
- ✅ Commit list generated
- ✅ No unexpected file deletions

**Checkpoint 1.4**: Review analysis files before proceeding

---

## PHASE 2: MERGE ANALYSIS 🔍

### Stage 2.1: Attempt Dry-Run Merge ✅
**Objective**: Simulate merge to identify all conflicts

**Commands**:
```bash
# Create temporary branch for dry-run
git checkout -b temp/merge-dryrun
git merge --no-commit --no-ff upstream/main

# Capture conflict list
git status | grep "both modified" > /tmp/merge-conflicts.txt
git diff --name-only --diff-filter=U >> /tmp/merge-conflicts.txt

# Abort dry-run
git merge --abort
git checkout feat/upstream-sync-v3.3.27
git branch -D temp/merge-dryrun
```

**Success Criteria**:
- ✅ Conflict list generated
- ✅ Dry-run aborted cleanly
- ✅ Back on working branch

**Checkpoint 2.1**: STOP → Review conflict list

---

### Stage 2.2: Categorize Conflicts 📋
**Objective**: Create conflict resolution strategy

**File Categories**:

#### 🟢 Category A: Simple Version Conflicts
- `package.json` → Keep 3.5.0
- `src-tauri/Cargo.toml` → Keep 3.5.0
- `src-tauri/tauri.conf.json` → Keep 3.5.0

**Strategy**: `git checkout --ours <file>`

#### 🟡 Category B: Additive Merges (Our new files)
**These files DON'T exist in upstream - ZERO conflicts**:
- `src-tauri/src/modules/budget_detector.rs`
- `src-tauri/src/modules/budget_optimizer.rs`
- `src-tauri/src/modules/model_selector.rs`
- `src-tauri/src/modules/quota_cache.rs`
- `src-tauri/src/modules/quota_fetcher.rs`
- `src-tauri/src/modules/quota_manager.rs`
- `src-tauri/src/modules/quota_monitor.rs`
- `src-tauri/src/modules/signature_cache.rs`
- `src-tauri/src/modules/thinking_quality.rs`
- `src-tauri/src/modules/weekly_tuner.rs`
- `src-tauri/src/commands/budget.rs`
- `src-tauri/src/commands/quality.rs`
- `src-tauri/src/commands/quota_manager_commands.rs`
- All frontend quota/budget components

**Strategy**: Auto-merge (no conflicts expected)

#### 🔴 Category C: Complex Manual Merges
**Priority 1 (Critical):**
1. `src-tauri/src/lib.rs` - Command registration
2. `src-tauri/src/modules/mod.rs` - Module exports
3. `src-tauri/src/modules/proxy_db.rs` - Database schema

**Priority 2 (Important):**
4. `src-tauri/src/modules/config.rs` - Config struct
5. `src-tauri/src/modules/logger.rs` - Logging setup
6. `src-tauri/src/modules/i18n.rs` - Translations

**Priority 3 (Low):**
7. `src-tauri/src/modules/account.rs`
8. `src-tauri/src/modules/oauth.rs`
9. `src-tauri/src/modules/process.rs`
10. `src-tauri/src/proxy/account_prioritizer.rs`

**Strategy**: Line-by-line manual merge with testing after each file

#### 🟢 Category D: Upstream New Files (Copy as-is)
- `src-tauri/src/modules/device.rs` (device binding)
- Any new upstream files

**Strategy**: `git checkout upstream/main -- <file>`

#### 📄 Category E: Documentation (Keep ours)
- `docs/epics/*.md`
- `docs/stories/*.md`
- Most README sections

**Strategy**: `git checkout --ours docs/`

**Checkpoint 2.2**: Confirm categorization is complete

---

### Stage 2.3: Create Conflict Resolution Checklist ✅
**Objective**: Track progress during merge

**Checklist File**: `/tmp/merge-checklist.md`

**Checkpoint 2.3**: Review checklist before starting merge

---

## PHASE 3: CONFLICT RESOLUTION 🔧

### Stage 3.1: Execute Actual Merge ⚠️
**Objective**: Start the real merge

**Commands**:
```bash
git checkout feat/upstream-sync-v3.3.27
git merge --no-commit --no-ff upstream/main
```

**Expected Output**: Merge conflicts in 10-20 files

**Success Criteria**:
- ✅ Merge initiated
- ✅ Conflicts identified
- ✅ Working tree ready for resolution

**Checkpoint 3.1**: STOP → Confirm merge started, conflicts visible

---

### Stage 3.2: Resolve Category A (Simple Versions) ✅
**Objective**: Quick wins - version conflicts

**Commands**:
```bash
# Keep our versions
git checkout --ours package.json
git checkout --ours src-tauri/Cargo.toml
git checkout --ours src-tauri/tauri.conf.json

# Stage resolved files
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json

# Verify
git diff --staged package.json | grep '"version"'
```

**Success Criteria**:
- ✅ Version 3.5.0 preserved in all files
- ✅ Files staged

**Checkpoint 3.2**: Version conflicts resolved

---

### Stage 3.3: Resolve Category D (New Upstream Files) ✅
**Objective**: Add upstream's new features

**Commands**:
```bash
# Check if device.rs exists in upstream
git show upstream/main:src-tauri/src/modules/device.rs > /dev/null 2>&1

# If exists, copy it
if [ $? -eq 0 ]; then
  git checkout upstream/main -- src-tauri/src/modules/device.rs
  git add src-tauri/src/modules/device.rs
fi

# Find any other new files in upstream
git diff --name-only --diff-filter=A main upstream/main -- src-tauri/src/
```

**Success Criteria**:
- ✅ New upstream files identified
- ✅ Files copied and staged
- ✅ No compilation errors

**Checkpoint 3.3**: New upstream files added

---

### Stage 3.4: Resolve Category E (Documentation) ✅
**Objective**: Keep all our documentation

**Commands**:
```bash
# Keep our entire docs directory
git checkout --ours docs/
git add docs/

# Optional: Cherry-pick useful upstream README sections
# (Manual review required)
```

**Success Criteria**:
- ✅ All our epic documentation preserved
- ✅ docs/ staged

**Checkpoint 3.4**: Documentation resolved

---

### Stage 3.5: Resolve Category C - Priority 1 (Critical Files) 🔴

#### Stage 3.5.1: lib.rs - Command Registration
**Objective**: Merge both sets of Tauri commands

**File**: `src-tauri/src/lib.rs`

**Conflict Areas**:
1. Command imports
2. `invoke_handler![]` macro
3. Plugin initialization

**Resolution Strategy**:
```rust
// MERGE APPROACH:
// 1. Keep all our command imports
// 2. Add any new upstream command imports
// 3. Combine both command lists in invoke_handler!
// 4. Merge plugin setup (keep both sets)

// Example:
invoke_handler![
    // Our commands (keep all)
    commands::budget::get_budget_status,
    commands::quality::get_quality_metrics,
    commands::quota_manager_commands::start_quota_monitor,
    // ... all our commands ...

    // Upstream commands (add new ones)
    commands::device::get_device_info,
    commands::update_checker::check_for_updates,
    // ... new upstream commands ...
]
```

**Steps**:
1. Open file in editor
2. Locate conflict markers `<<<<<<<`, `=======`, `>>>>>>>`
3. Manually merge both sections
4. Remove conflict markers
5. Build test: `cargo check --manifest-path src-tauri/Cargo.toml`

**Success Criteria**:
- ✅ All commands from both branches included
- ✅ No duplicate command entries
- ✅ Compiles without errors
- ✅ Conflict markers removed

**Checkpoint 3.5.1**: STOP → Test build after lib.rs resolution

---

#### Stage 3.5.2: modules/mod.rs - Module Exports
**Objective**: Export all modules from both branches

**File**: `src-tauri/src/modules/mod.rs`

**Conflict Areas**:
1. `pub mod` declarations
2. `pub use` re-exports

**Resolution Strategy**:
```rust
// Combine both lists alphabetically
pub mod account;
pub mod budget_detector;      // Ours
pub mod budget_optimizer;     // Ours
pub mod config;
pub mod db;
pub mod device;               // Upstream new
// ... etc
```

**Steps**:
1. List all our modules
2. List all upstream modules
3. Merge alphabetically
4. Remove duplicates
5. Build test

**Success Criteria**:
- ✅ All modules exported
- ✅ Alphabetical order
- ✅ Compiles successfully

**Checkpoint 3.5.2**: modules/mod.rs resolved and builds

---

#### Stage 3.5.3: modules/proxy_db.rs - Database Schema 🔴🔴🔴
**Objective**: Merge database schemas (MOST COMPLEX)

**File**: `src-tauri/src/modules/proxy_db.rs`

**Conflict Areas**:
1. Table CREATE statements
2. Index definitions
3. SQL queries
4. Migration logic

**Resolution Strategy**:
```sql
-- APPROACH:
-- 1. Compare schemas side-by-side
-- 2. Merge table definitions (keep all columns from both)
-- 3. Combine indexes (no duplicates)
-- 4. Update queries to use merged schema
-- 5. Test database operations

-- Example:
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,

    -- Our columns
    budget_level TEXT,
    quality_score REAL,
    signature_cache_hit INTEGER DEFAULT 0,

    -- Upstream columns
    device_id TEXT,
    context_scale INTEGER DEFAULT 1,

    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
```

**Steps**:
1. Extract our schema to `/tmp/our_schema.sql`
2. Extract upstream schema to `/tmp/upstream_schema.sql`
3. Create merged schema in `/tmp/merged_schema.sql`
4. Apply merged schema to proxy_db.rs
5. Update all queries to reference new columns
6. Test: `cargo test --manifest-path src-tauri/Cargo.toml db::`

**Success Criteria**:
- ✅ All tables merged (no data loss)
- ✅ All indexes present
- ✅ Queries updated
- ✅ Database tests pass

**Checkpoint 3.5.3**: STOP → Critical - Test database operations thoroughly

---

### Stage 3.6: Resolve Category C - Priority 2 (Important Files) 🟡

#### Stage 3.6.1: modules/config.rs - Configuration
**Objective**: Merge config structs

**Conflict Areas**:
1. `Config` struct fields
2. Default implementations
3. Serialization attributes

**Resolution Strategy**: Additive merge - keep all fields from both

**Checkpoint 3.6.1**: config.rs resolved

---

#### Stage 3.6.2: modules/logger.rs - Logging
**Objective**: Merge logging configurations

**Resolution Strategy**: Combine filtering and formatting logic

**Checkpoint 3.6.2**: logger.rs resolved

---

#### Stage 3.6.3: modules/i18n.rs + Frontend Locales
**Objective**: Merge translation systems

**Files**:
- `src-tauri/src/modules/i18n.rs`
- `src/locales/en.json`
- `src/locales/zh.json`

**Resolution Strategy**: Merge translation keys (combine both sets)

**Checkpoint 3.6.3**: i18n resolved

---

### Stage 3.7: Resolve Category C - Priority 3 (Low Priority) 🟢
**Objective**: Resolve remaining code conflicts

**Files**:
- `modules/account.rs`
- `modules/oauth.rs`
- `modules/process.rs`
- `proxy/account_prioritizer.rs`

**Strategy**: Case-by-case merge, prefer our logic if upstream changes are minor

**Checkpoint 3.7**: All conflicts resolved

---

### Stage 3.8: Final Conflict Check ✅
**Objective**: Ensure no conflicts remain

**Commands**:
```bash
# Check for conflict markers
grep -r "<<<<<<< HEAD" src-tauri/src/ src/
grep -r "=======" src-tauri/src/ src/
grep -r ">>>>>>>" src-tauri/src/ src/

# Check git status
git status | grep "Unmerged"
```

**Success Criteria**:
- ✅ No conflict markers found
- ✅ No unmerged files
- ✅ All files staged or ignored

**Checkpoint 3.8**: STOP → Confirm zero conflicts remain

---

## PHASE 4: TESTING & VALIDATION 🧪

### Stage 4.1: Backend Build Test ✅
**Objective**: Ensure Rust code compiles

**Commands**:
```bash
cd src-tauri
cargo clean
cargo build
cargo build --release
```

**Success Criteria**:
- ✅ Debug build succeeds
- ✅ Release build succeeds
- ✅ Zero compilation errors
- ✅ Zero clippy warnings

**Checkpoint 4.1**: STOP → Backend must build cleanly

---

### Stage 4.2: Backend Unit Tests ✅
**Objective**: Verify all tests pass

**Commands**:
```bash
cd src-tauri
cargo test --lib
cargo test --bins
cargo test -- --nocapture  # See output
```

**Expected**: All tests pass (may have timing flakes, retry if needed)

**Success Criteria**:
- ✅ 90%+ tests pass
- ✅ Critical tests pass (database, proxy, quota)
- ✅ No new test failures vs. pre-merge

**Checkpoint 4.2**: STOP → Review any test failures

---

### Stage 4.3: Frontend Build Test ✅
**Objective**: Ensure React app compiles

**Commands**:
```bash
npm install  # In case of new dependencies
npm run build
```

**Success Criteria**:
- ✅ TypeScript compilation succeeds
- ✅ Vite build succeeds
- ✅ No build errors

**Checkpoint 4.3**: Frontend must build cleanly

---

### Stage 4.4: Code Quality Checks ✅
**Objective**: Maintain code standards

**Commands**:
```bash
# Rust
cd src-tauri
cargo clippy -- -D warnings
cargo fmt -- --check

# TypeScript (if available)
cd ..
npx tsc --noEmit
```

**Success Criteria**:
- ✅ Zero clippy warnings
- ✅ Code formatted correctly
- ✅ TypeScript type-checks pass

**Checkpoint 4.4**: Code quality verified

---

### Stage 4.5: Integration Testing 🎮
**Objective**: Manual functional testing

**Test Scenarios**:

#### Test 4.5.1: Application Launch
```bash
npm run tauri dev
```
- ✅ Application starts without crashes
- ✅ No console errors
- ✅ UI renders correctly

#### Test 4.5.2: Account Management
- ✅ View accounts list
- ✅ OAuth authorization flow works
- ✅ Account switching works
- ✅ Quota display updates

#### Test 4.5.3: Proxy Server
- ✅ Start proxy server
- ✅ Test OpenAI endpoint: `curl http://localhost:8045/v1/chat/completions`
- ✅ Test Claude endpoint: `curl http://localhost:8045/v1/messages`
- ✅ Test Gemini endpoint
- ✅ Account rotation works
- ✅ Stop proxy server

#### Test 4.5.4: Quota Monitoring (Our Feature)
- ✅ Quota monitor starts
- ✅ Quota data fetches successfully
- ✅ UI displays quota information
- ✅ Alerts trigger when thresholds met

#### Test 4.5.5: Budget Optimizer (Our Feature)
- ✅ Budget page loads
- ✅ Budget detection works
- ✅ Auto-adjustment functions
- ✅ Charts render correctly

#### Test 4.5.6: New Upstream Features
- ✅ Device binding (if integrated)
- ✅ Context scaling config
- ✅ Update checker notification

**Checkpoint 4.5**: STOP → All critical features must work

---

### Stage 4.6: Database Migration Test ✅
**Objective**: Ensure DB migrations don't break existing data

**Commands**:
```bash
# Backup existing database
cp ~/.local/share/com.lbjlaq.antigravity-tools/antigravity.db \
   ~/.local/share/com.lbjlaq.antigravity-tools/antigravity.db.backup

# Launch app (should auto-migrate)
npm run tauri dev

# Check migration logs
tail -100 ~/.local/share/com.lbjlaq.antigravity-tools/logs/app.log
```

**Success Criteria**:
- ✅ Database opens successfully
- ✅ Migration runs without errors
- ✅ Existing data preserved
- ✅ New columns/tables created

**Checkpoint 4.6**: Database integrity verified

---

## PHASE 5: FINALIZATION 🎉

### Stage 5.1: Cleanup 🧹
**Objective**: Remove artifacts and prepare for commit

**Commands**:
```bash
# Remove backup files
find . -name "*.orig" -delete
find . -name "*.bak" -delete

# Clean temp files
rm -f /tmp/upstream-*.txt
rm -f /tmp/merge-*.txt
rm -f /tmp/our_schema.sql
rm -f /tmp/upstream_schema.sql
rm -f /tmp/merged_schema.sql

# Verify clean state
git status
```

**Success Criteria**:
- ✅ No .orig or .bak files
- ✅ Git status shows only intended changes
- ✅ No temp files in repo

**Checkpoint 5.1**: Clean working tree

---

### Stage 5.2: Update Documentation 📚
**Objective**: Document the merge

**Files to Update**:
1. `CLAUDE.md` - Update version to 3.5.1 (or keep 3.5.0)
2. `docs/UPSTREAM-SYNC-COMPLETION-REPORT.md` - Create completion report
3. `CHANGELOG.md` - Add merge entry (if exists)

**Checkpoint 5.2**: Documentation updated

---

### Stage 5.3: Commit Merge 💾
**Objective**: Save all changes

**Commands**:
```bash
# Stage all resolved files
git add .

# Verify what's being committed
git status
git diff --staged --stat

# Commit with detailed message
git commit -m "feat: sync with upstream v3.3.27 while preserving all v3.5.0 features

INTEGRATED UPSTREAM CHANGES:
- Device binding for fraud prevention (device.rs)
- Context scaling configuration
- MCP compatibility enhancements
- Stream error i18n support
- Update notification system improvements
- Session-based caching enhancements

PRESERVED ALL v3.5.0 FEATURES:
- Complete quota management system (Epic-001)
  - Quota monitor, cache, fetcher modules
  - Background monitoring and alerts
  - Tier detection and prioritization
- Budget optimizer (Epic-024, Epic-025)
  - Budget detector and optimizer modules
  - Adaptive budget adjustment
  - Quality monitoring dashboard
- Model coverage system (Epic-026)
  - Model selector module
  - 92.6% model coverage achieved
- Signature cache and thinking quality monitoring
- Audio metrics tracking
- All 183 commits from our development branch

CONFLICTS RESOLVED:
- src-tauri/src/lib.rs (command registration merged)
- src-tauri/src/modules/mod.rs (all modules exported)
- src-tauri/src/modules/config.rs (config structs merged)
- src-tauri/src/modules/proxy_db.rs (database schema merged)
- src-tauri/src/modules/logger.rs (logging config combined)
- src-tauri/src/modules/i18n.rs (translations merged)
- package.json, Cargo.toml (version 3.5.0 maintained)

TESTING COMPLETED:
- ✅ All Rust unit tests pass
- ✅ Backend builds cleanly (debug + release)
- ✅ Frontend builds successfully
- ✅ Zero clippy warnings
- ✅ Manual integration testing passed
- ✅ Database migration verified
- ✅ All Epic-001 to Epic-026 features functional

UPSTREAM COMMITS: 74 commits from lbjlaq/Antigravity-Manager
OUR COMMITS: 183 commits preserved
MERGE BASE: $(git merge-base upstream/main main)
UPSTREAM HEAD: upstream/main@d143c16
OUR HEAD: origin/main@379ec3f

Breaking Changes: NONE
Risk Level: LOW (all tests pass, features preserved)
Review Required: YES (before merge to main)

Refs: #upstream-sync, Epic-001, Epic-024, Epic-025, Epic-026"
```

**Success Criteria**:
- ✅ Commit created successfully
- ✅ Commit message detailed and clear

**Checkpoint 5.3**: Merge committed locally

---

### Stage 5.4: Create Completion Report 📊
**Objective**: Document what was done

**File**: `docs/UPSTREAM-SYNC-COMPLETION-REPORT.md`

**Contents**:
- Execution timeline
- Conflicts encountered and resolution
- Test results
- Features integrated
- Features preserved
- Known issues (if any)
- Recommendations

**Checkpoint 5.4**: Completion report created

---

### Stage 5.5: Push to Remote 🚀
**Objective**: Backup merge work

**Commands**:
```bash
# Push feature branch
git push origin feat/upstream-sync-v3.3.27

# Verify push
git log origin/feat/upstream-sync-v3.3.27 -1
```

**Success Criteria**:
- ✅ Branch pushed successfully
- ✅ Remote branch matches local

**Checkpoint 5.5**: Remote backup created

---

### Stage 5.6: Create Pull Request (Optional) 📝
**Objective**: Prepare for team review

**If using PRs**:
- Create PR: `feat/upstream-sync-v3.3.27` → `main`
- Include completion report in PR description
- Tag reviewers
- Wait for approval

**If merging directly**:
- Skip to Stage 5.7

**Checkpoint 5.6**: PR created or skipped

---

### Stage 5.7: Merge to Main 🎯
**Objective**: Apply changes to main branch

**Commands**:
```bash
# Switch to main
git checkout main

# Merge feature branch (no-ff to preserve history)
git merge --no-ff feat/upstream-sync-v3.3.27

# Push to origin
git push origin main

# Verify
git log --graph --oneline -10
```

**Success Criteria**:
- ✅ Main branch updated
- ✅ Merge commit created
- ✅ Remote main updated

**Checkpoint 5.7**: STOP → Final verification before tagging

---

### Stage 5.8: Tag Release 🏷️
**Objective**: Mark this milestone

**Commands**:
```bash
# Create annotated tag
git tag -a v3.5.1 -m "Release v3.5.1: Upstream sync + all v3.5.0 features

This release syncs with upstream v3.3.27 while preserving all our v3.5.0 features:
- Complete quota management system
- Budget optimizer
- Model coverage (92.6%)
- Signature cache
- All Epic-001 to Epic-026 features

New from upstream:
- Device binding
- Context scaling
- MCP enhancements
- Update notifications"

# Push tag
git push origin v3.5.1

# Verify
git tag -l -n5 v3.5.1
```

**Success Criteria**:
- ✅ Tag created
- ✅ Tag pushed to remote
- ✅ Tag message detailed

**Checkpoint 5.8**: Release tagged

---

### Stage 5.9: Final Validation ✅
**Objective**: Ensure main branch is healthy

**Commands**:
```bash
# Fresh clone test (optional but recommended)
cd /tmp
git clone https://github.com/salacoste/Antigravity-Manager.git test-clone
cd test-clone
npm install
cargo build --manifest-path src-tauri/Cargo.toml
npm run build

# If successful, delete test clone
cd ..
rm -rf test-clone
```

**Success Criteria**:
- ✅ Fresh clone builds successfully
- ✅ No missing dependencies
- ✅ Application runs correctly

**Checkpoint 5.9**: Main branch validated

---

## PHASE 6: POST-MERGE MONITORING 👀

### Stage 6.1: Monitor for Issues (24-48 hours)
**Objective**: Catch any edge cases

**Activities**:
- Watch for build failures in CI/CD
- Monitor user reports (if applicable)
- Check logs for new warnings/errors
- Test on different platforms (macOS, Windows, Linux)

**Checkpoint 6.1**: No critical issues reported

---

### Stage 6.2: Cleanup Branches 🧹
**Objective**: Remove temporary branches

**Commands**:
```bash
# After confirming everything works (wait 24-48h)
git branch -d feat/upstream-sync-v3.3.27
git push origin --delete feat/upstream-sync-v3.3.27

# Keep backup branch for 30 days, then delete
# (Set reminder to delete backup/pre-upstream-sync-2026-01-14 after 2026-02-14)
```

**Checkpoint 6.2**: Cleanup complete

---

## ROLLBACK PLAN 🚨

**If critical issues discovered at any stage:**

### Emergency Rollback
```bash
# Rollback main branch
git checkout main
git reset --hard backup/pre-upstream-sync-2026-01-14
git push origin main --force

# Notify team
echo "ROLLBACK: Critical issue in upstream sync. Reverted to pre-sync state."
```

### Partial Rollback (Revert Merge Commit)
```bash
# Find merge commit
git log --oneline --merges -5

# Revert the merge (keep parent 1 = our work)
git revert -m 1 <merge-commit-hash>
git push origin main
```

---

## EXECUTION CHECKLIST ✅

**Before Starting**:
- [ ] Review entire plan
- [ ] Ensure 4-6 hours of uninterrupted time
- [ ] Notify team (if applicable)
- [ ] Current code builds and tests pass

**Phase 1 - Preparation**:
- [ ] 1.1: Backup branch created and pushed
- [ ] 1.2: Working branch created
- [ ] 1.3: Current state verified (builds + tests pass)
- [ ] 1.4: Upstream changes analyzed

**Phase 2 - Merge Analysis**:
- [ ] 2.1: Dry-run merge completed
- [ ] 2.2: Conflicts categorized
- [ ] 2.3: Checklist created

**Phase 3 - Conflict Resolution**:
- [ ] 3.1: Actual merge initiated
- [ ] 3.2: Version conflicts resolved
- [ ] 3.3: New upstream files added
- [ ] 3.4: Documentation resolved
- [ ] 3.5.1: lib.rs resolved and builds
- [ ] 3.5.2: modules/mod.rs resolved and builds
- [ ] 3.5.3: proxy_db.rs resolved and DB tests pass ⚠️ CRITICAL
- [ ] 3.6: Priority 2 files resolved
- [ ] 3.7: Priority 3 files resolved
- [ ] 3.8: Zero conflicts remain

**Phase 4 - Testing**:
- [ ] 4.1: Backend builds (debug + release)
- [ ] 4.2: All tests pass
- [ ] 4.3: Frontend builds
- [ ] 4.4: Code quality checks pass
- [ ] 4.5: Manual integration testing complete ⚠️ CRITICAL
- [ ] 4.6: Database migration verified

**Phase 5 - Finalization**:
- [ ] 5.1: Cleanup complete
- [ ] 5.2: Documentation updated
- [ ] 5.3: Merge committed
- [ ] 5.4: Completion report created
- [ ] 5.5: Pushed to remote
- [ ] 5.6: PR created (if needed)
- [ ] 5.7: Merged to main
- [ ] 5.8: Release tagged
- [ ] 5.9: Main branch validated

**Phase 6 - Post-Merge**:
- [ ] 6.1: 24-48h monitoring (no issues)
- [ ] 6.2: Branches cleaned up

---

## STOP POINTS 🛑

**Mandatory stops** where we MUST verify before proceeding:

1. **After Stage 1.1**: Verify backup exists remotely
2. **After Stage 1.3**: Confirm all builds and tests pass
3. **After Stage 2.1**: Review conflict list
4. **After Stage 3.1**: Confirm merge started
5. **After Stage 3.5.1**: Test build after lib.rs
6. **After Stage 3.5.3**: Test database operations ⚠️⚠️⚠️
7. **After Stage 3.8**: Confirm zero conflicts remain
8. **After Stage 4.1**: Backend must build cleanly
9. **After Stage 4.2**: Review any test failures
10. **After Stage 4.5**: All critical features must work ⚠️⚠️⚠️
11. **After Stage 5.7**: Final verification before tagging

---

## ESTIMATED TIMELINE ⏱️

| Phase | Stages | Estimated Time | Critical? |
|-------|--------|---------------|-----------|
| Phase 1 | 1.1-1.4 | 30-45 min | 🟡 Important |
| Phase 2 | 2.1-2.3 | 30-45 min | 🟢 Analysis |
| Phase 3 | 3.1-3.8 | 3-5 hours | 🔴 CRITICAL |
| Phase 4 | 4.1-4.6 | 2-3 hours | 🔴 CRITICAL |
| Phase 5 | 5.1-5.9 | 1-2 hours | 🟡 Important |
| Phase 6 | 6.1-6.2 | 24-48 hours | 🟢 Monitoring |
| **TOTAL** | **21 stages** | **7-11 hours active + 24-48h monitoring** | |

**Best Practice**: Execute Phase 1-3 in one session, Phase 4-5 in another (with breaks).

---

## SUCCESS CRITERIA ✅

**Must achieve ALL of these**:
1. ✅ All tests pass (backend + frontend)
2. ✅ Application builds without errors
3. ✅ All Epic-001 to Epic-026 features work
4. ✅ New upstream features integrated
5. ✅ No performance regressions
6. ✅ Database migrations successful
7. ✅ Zero merge conflicts remain
8. ✅ Documentation updated
9. ✅ Version 3.5.0+ maintained

---

**Ready to Execute?**

Next: **Phase 1 - Stage 1.1: Create Backup Branch**

Confirm to proceed → "начинаем Phase 1" or "go"
