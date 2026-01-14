# Upstream Sync Plan - Antigravity Manager

**Date**: 2026-01-14
**Current Branch**: `epic-026-model-coverage`
**Target**: Sync with `upstream/main` (v3.3.27) while preserving all our work

## Executive Summary

### Current State
- **Our Version**: v3.5.0 (183 commits ahead)
- **Upstream Version**: v3.3.27 (74 commits ahead)
- **Total Divergence**: 257 commits, 1,215 files changed
- **Breaking Changes**: ❌ None detected in upstream

### Risk Assessment

#### 🟢 LOW RISK (Our new modules - no conflicts)
- `budget_detector.rs`, `budget_optimizer.rs`, `model_selector.rs`
- `quota_cache.rs`, `quota_fetcher.rs`, `quota_manager.rs`, `quota_monitor.rs`
- `signature_cache.rs`, `thinking_quality.rs`, `weekly_tuner.rs`
- All new commands: `budget.rs`, `quality.rs`, `quota_manager_commands.rs`
- New database modules: `audio_metrics.rs`
- Frontend: All quota-manager components, new pages (BudgetOptimizer, QualityDashboard)

**Action**: These files will be preserved as-is (only exist in our fork).

#### 🟡 MEDIUM RISK (Modified files - potential conflicts)
- `modules/account.rs` - Account management enhancements
- `modules/config.rs` - Configuration additions
- `modules/proxy_db.rs` - Database schema extensions (±1155 lines)
- `modules/logger.rs` - Logging enhancements
- `modules/i18n.rs` - Internationalization updates
- `proxy/account_prioritizer.rs` - Account selection logic
- `lib.rs` - Command registration changes

**Upstream Changes**:
- Session-based signature caching (we have our own implementation)
- Device binding for fraud prevention (new feature)
- Context scaling configuration (new feature)
- MCP compatibility enhancements
- Stream error i18n support

**Action**: Careful merge with manual conflict resolution.

#### 🟢 LOW RISK (Documentation conflicts)
- `docs/epics/*.md` - Our epic documentation
- README updates - Minor version differences

**Action**: Keep our documentation, optionally merge upstream README improvements.

## Upstream Key Features to Integrate

### 1. **Device Binding** (fraud prevention)
- **File**: `src-tauri/src/modules/device.rs` (upstream only)
- **Value**: Reduces account suspension risk
- **Integration**: Copy from upstream, connect to our account system

### 2. **Context Scaling Configuration**
- **Commits**: `c5e2f9c`, `f7522e8` - "aggressive context scaling"
- **Value**: Better performance for high-throughput scenarios
- **Integration**: Merge into our `config.rs` and proxy config

### 3. **MCP Compatibility Enhancements**
- **Commit**: `22dee5a` - "MCP 工具兼容性增强"
- **Value**: Better Claude Code integration
- **Integration**: Review and merge MCP-related changes

### 4. **Stream Error i18n Support**
- **Commit**: `c6d09eb` - "add i18n support for stream error messages"
- **Value**: Better user experience for non-English users
- **Integration**: Merge into our i18n system

### 5. **Automatic Update Notification System**
- **Commits**: `af0be0c`, `c9c028e`, `ac595b1`
- **Value**: User-friendly update notifications
- **Integration**: Review `update_checker.rs` changes

## Migration Strategy

### Phase 1: Preparation (2-3 hours)
1. **Backup current state**
   ```bash
   git checkout -b backup/pre-upstream-sync-2026-01-14
   git push origin backup/pre-upstream-sync-2026-01-14
   ```

2. **Create sync branch**
   ```bash
   git checkout main
   git checkout -b feat/upstream-sync-v3.3.27
   ```

3. **Review upstream commits in detail**
   ```bash
   git log --stat upstream/main ^main --since="2024-12-01"
   git diff upstream/main main -- src-tauri/src/modules/ src-tauri/src/proxy/
   ```

4. **Document potential conflicts**
   - Create conflict resolution guide
   - Identify merge strategy for each file

### Phase 2: Incremental Merge (4-6 hours)

#### Strategy: Cherry-pick vs. Full Merge
**Recommendation**: **Full merge with strategic rollback**
- More maintainable long-term
- Preserves git history
- Easier future syncs

#### Step-by-step Process

**2.1. Attempt Automatic Merge**
```bash
git merge --no-commit --no-ff upstream/main
```

**2.2. Review Conflicts**
```bash
git status
# Expected conflicts in:
# - src-tauri/src/modules/mod.rs (module registration)
# - src-tauri/src/modules/config.rs (config fields)
# - src-tauri/src/modules/proxy_db.rs (database schema)
# - src-tauri/src/lib.rs (command registration)
# - package.json (version conflict)
# - Cargo.toml (version + dependencies)
```

**2.3. Resolve Conflicts by Category**

##### A. Version Conflicts (Simple)
- `package.json`: Keep `3.5.0`
- `Cargo.toml`: Keep `3.5.0`
- `src-tauri/tauri.conf.json`: Keep `3.5.0`

##### B. Dependency Conflicts (Additive)
- `Cargo.toml`: Merge both sets of dependencies
- `package.json`: Keep `recharts: ^3.6.0` (our version is newer)

##### C. Code Conflicts (Manual Resolution)

**`src-tauri/src/lib.rs`**
- **Conflict**: Command registration
- **Resolution**: Merge both sets of commands (use `<<<<<<< ======= >>>>>>>` markers)
- **Our additions**: budget, quality, quota_manager commands
- **Upstream additions**: device binding, update_checker enhancements

**`src-tauri/src/modules/mod.rs`**
- **Conflict**: Module exports
- **Resolution**: Combine both module lists
- **Our additions**: ~15 new modules
- **Upstream additions**: device binding, update_checker improvements

**`src-tauri/src/modules/config.rs`**
- **Conflict**: Config struct fields
- **Resolution**: Merge struct definitions, preserve serde attributes
- **Our additions**: Budget optimizer config, quota cache config, quality monitoring
- **Upstream additions**: Context scaling config, device binding settings

**`src-tauri/src/modules/proxy_db.rs`**
- **Conflict**: Database schema, SQL queries
- **Resolution**: **Most complex** - merge table definitions
- **Strategy**:
  1. Compare schema changes side-by-side
  2. Merge table CREATE statements
  3. Combine index definitions
  4. Update migration logic in `migration.rs`
- **Our additions**: Budget tables, quality metrics, signature cache
- **Upstream additions**: Device tracking, session management

**`src-tauri/src/modules/logger.rs`**
- **Conflict**: Logging configuration
- **Resolution**: Merge log filtering and formatting logic
- **Our additions**: Structured logging for quota monitoring
- **Upstream additions**: Enhanced filtering, performance improvements

**`src-tauri/src/modules/i18n.rs`**
- **Conflict**: Translation keys and language support
- **Resolution**: Merge translation files (`src/locales/*.json`)
- **Our additions**: Quota monitoring UI strings, budget optimizer strings
- **Upstream additions**: Stream error messages, device binding UI

##### D. Documentation Conflicts (Strategic Keep/Discard)
- **Strategy**: Keep ALL our epics documentation
- **Action**: Accept "ours" for entire `docs/` directory
- **Optional**: Manually merge useful upstream README sections

```bash
# Keep all our docs
git checkout --ours docs/
git add docs/
```

**2.4. Copy New Upstream Files**
```bash
# New files that don't exist in our repo
git checkout upstream/main -- src-tauri/src/modules/device.rs
git add src-tauri/src/modules/device.rs
```

**2.5. Test After Each Major Conflict Resolution**
```bash
# After resolving lib.rs, mod.rs
cargo build --manifest-path src-tauri/Cargo.toml

# After resolving proxy_db.rs
cargo test --manifest-path src-tauri/Cargo.toml db::

# After resolving frontend conflicts
npm run build
```

### Phase 3: Integration & Testing (3-4 hours)

**3.1. Build Tests**
```bash
# Backend
cd src-tauri
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check

# Frontend
npm install  # In case of new dependencies
npm run build
```

**3.2. Functional Tests**
- Launch application: `npm run tauri dev`
- Test account management (OAuth flow)
- Test proxy server start/stop
- Test quota monitoring features
- Test new upstream features (device binding, context scaling)

**3.3. Integration Tests**
- Full proxy request flow (OpenAI/Claude/Gemini endpoints)
- Account rotation logic
- Quota cache updates
- Budget optimizer functionality

**3.4. Regression Tests**
- All Epic-001 to Epic-026 features still work
- No broken UI components
- Database migrations work correctly

### Phase 4: Cleanup & Documentation (1-2 hours)

**4.1. Clean up merge artifacts**
```bash
# Remove backup files, conflict markers
find . -name "*.orig" -delete
git status  # Ensure clean state
```

**4.2. Update documentation**
```bash
# Update CLAUDE.md with new version
# Document upstream features we integrated
# Update CHANGELOG.md (if exists)
```

**4.3. Commit merge**
```bash
git add .
git commit -m "feat: sync with upstream v3.3.27 while preserving all v3.5.0 features

Integrated upstream changes:
- Device binding for fraud prevention
- Context scaling configuration
- MCP compatibility enhancements
- Stream error i18n support
- Update notification system

Preserved all our features:
- Complete quota management system (Epic-001)
- Budget optimizer (Epic-024, Epic-025)
- Model coverage system (Epic-026)
- Signature cache and thinking quality monitoring
- All epic-specific enhancements

Conflicts resolved in:
- src-tauri/src/lib.rs (command registration)
- src-tauri/src/modules/mod.rs (module exports)
- src-tauri/src/modules/config.rs (config struct)
- src-tauri/src/modules/proxy_db.rs (database schema)
- src-tauri/src/modules/logger.rs (logging config)

Refs: upstream/main@d143c16, origin/main@379ec3f"
```

**4.4. Push and create PR**
```bash
git push origin feat/upstream-sync-v3.3.27

# Create PR with detailed description:
# - List of upstream features integrated
# - Conflict resolution approach
# - Testing results
# - Migration notes for team
```

### Phase 5: Post-Merge Validation (1-2 hours)

**5.1. Extended testing**
- Run full test suite multiple times
- Test on different platforms (macOS, Windows, Linux if applicable)
- Stress test proxy server with concurrent requests

**5.2. Monitor for issues**
- Check logs for new warnings/errors
- Verify all database migrations successful
- Ensure no memory leaks or performance degradation

**5.3. Merge to main**
```bash
# After PR approval
git checkout main
git merge --no-ff feat/upstream-sync-v3.3.27
git push origin main

# Tag the release
git tag -a v3.5.1 -m "Release v3.5.1: Upstream sync + all v3.5.0 features"
git push origin v3.5.1
```

## Rollback Plan

If critical issues discovered:

```bash
# Rollback to pre-sync state
git checkout main
git reset --hard backup/pre-upstream-sync-2026-01-14
git push origin main --force  # Use with caution

# Or revert the merge commit
git revert -m 1 <merge-commit-hash>
```

## Key Principles

1. **Preserve ALL our work** - Every feature from Epic-001 to Epic-026
2. **Merge, don't replace** - Combine upstream improvements with our additions
3. **Test incrementally** - Don't wait until the end to discover issues
4. **Document decisions** - Record why we chose specific conflict resolutions
5. **Maintain version 3.5.0** - We're ahead of upstream, keep our version number

## Expected Timeline

| Phase | Duration | Confidence |
|-------|----------|-----------|
| Phase 1: Preparation | 2-3 hours | High |
| Phase 2: Incremental Merge | 4-6 hours | Medium |
| Phase 3: Integration & Testing | 3-4 hours | Medium |
| Phase 4: Cleanup & Documentation | 1-2 hours | High |
| Phase 5: Post-Merge Validation | 1-2 hours | High |
| **Total** | **11-17 hours** | **Medium-High** |

## Success Criteria

- ✅ All tests pass (backend + frontend)
- ✅ Application builds without errors
- ✅ All Epic-001 to Epic-026 features functional
- ✅ New upstream features integrated and working
- ✅ No performance regressions
- ✅ Database migrations successful
- ✅ Documentation updated

## Risk Mitigation

- **Backup branch created** before any changes
- **Incremental testing** after each major conflict resolution
- **Rollback plan** documented and ready
- **Conflict resolution documented** for future reference
- **Team review** before merging to main

## Next Steps

1. **Get approval** for this plan from team/lead
2. **Schedule dedicated time** (1-2 days, minimize interruptions)
3. **Execute Phase 1** (backup and preparation)
4. **Begin Phase 2** with careful conflict resolution
5. **Continuous communication** - share progress and blockers

---

**Document Version**: 1.0
**Author**: Amelia (Developer Agent)
**Review Required**: Yes - before execution
**Estimated Completion**: 2026-01-16 (2 working days)
