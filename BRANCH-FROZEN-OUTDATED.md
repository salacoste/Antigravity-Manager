# FROZEN BRANCH - OUTDATED

This branch is frozen as OUTDATED.
Based on v3.3.14, main is now v3.5.0 (2+ months ahead).

Epic: Epic-001 (QUOTA-001) Quota-Aware Failover
Status: OUTDATED - Core functionality merged via other commits ⚠️
Base Version: v3.3.14
Main Version: v3.5.0

## What was merged:
- ✅ Quota-aware account selection (in main via Epic-001)
- ✅ z.ai compatibility improvements (in main via z.ai PRs)

## What was NOT merged (needs porting):
- quota_is_forbidden flag
- QuotaUnavailableReason enum  
- Per-model quota tracking (quota_models HashMap)

## Why not merged:
28 merge conflicts due to divergent codebase (v3.3.14 → v3.5.0).
Main contains newer implementations of core features.

Do not make new commits to this branch.
Use main branch for future development.
See docs/MAIN-MERGE-REPORT-2026-01-13.md for details.
