# Epic-024/025 TEAM COORDINATION PLAN

**Date**: 2026-01-27
**Implementation Start**: Feb 1, 2026
**Duration**: 10 weeks (3 weeks Epic-024 + 7 weeks Epic-025 parallel)
**Teams**: Team 1 (3 developers) + Team 2 (3 developers)

---

## 🎯 Executive Summary

This document coordinates parallel development of Epic-024 (Flash Base - Model ID 312) and Epic-025 (Flash Thinking - Model ID 313) by two independent teams of 3 developers each. The plan ensures zero code conflicts through domain separation, interface contracts, and phased integration.

**Key Principle**: Epic-024 focuses on **base model infrastructure** (quota, routing), Epic-025 focuses on **thinking model features** (budget, quality).

---

## 👥 Team Composition

### Team 1 - Epic-025 (Flash Thinking Optimization)
**Branch**: `epic-025-flash-thinking`
**Duration**: 7 weeks (Feb 1 - Mar 21)
**Focus**: Model ID 313 thinking mode optimization

```yaml
developer_1_backend_lead:
  name: "Backend Lead"
  focus: "Budget optimization, thinking quality monitoring"
  stories: "025-01 (lead), 025-03, 025-04"
  files:
    - src-tauri/src/modules/budget_optimizer.rs (NEW)
    - src-tauri/src/modules/thinking_quality.rs (NEW)
    - src-tauri/src/proxy/handlers/gemini.rs (MODIFY - thinking paths only)

developer_2_cache_specialist:
  name: "Cache Specialist"
  focus: "Signature cache enhancement, LRU implementation"
  stories: "025-02 (lead)"
  files:
    - src-tauri/src/modules/signature_cache.rs (NEW)
    - src-tauri/src/proxy/providers/gemini_provider.rs (MODIFY - cache integration)

developer_3_frontend:
  name: "Frontend Lead"
  focus: "All Epic-025 UI components"
  stories: "All 025 frontend tasks"
  files:
    - src/pages/BudgetOptimizerPage.tsx (NEW)
    - src/pages/QualityDashboardPage.tsx (NEW)
    - src/components/budget/ (NEW directory)
    - src/components/quality/ (NEW directory)
```

### Team 2 - Epic-024 (Flash Base Optimization)
**Branch**: `epic-024-flash-base`
**Duration**: 3 weeks (Feb 1 - Feb 21)
**Focus**: Model ID 312 base model infrastructure

```yaml
developer_1_backend_lead:
  name: "Backend Lead"
  focus: "Quota monitoring, model selection"
  stories: "024-01, 024-02 (lead)"
  files:
    - src-tauri/src/modules/quota_monitor.rs (NEW)
    - src-tauri/src/modules/model_selector.rs (NEW)
    - src-tauri/src/modules/quota.rs (MODIFY - add monitoring hooks)

developer_2_integration:
  name: "Integration Specialist"
  focus: "Token manager integration, cost tracking"
  stories: "024-02 (support)"
  files:
    - src-tauri/src/proxy/token_manager.rs (MODIFY - add model selection hooks)
    - src-tauri/src/modules/cost_tracker.rs (NEW)

developer_3_frontend:
  name: "Frontend Lead"
  focus: "All Epic-024 UI components"
  stories: "All 024 frontend tasks"
  files:
    - src/pages/QuotaMonitoringPage.tsx (NEW)
    - src/components/quota/ (NEW directory)
    - src/components/alerts/ (NEW directory)
```

---

## 🗂️ Code Domain Separation

### Epic-024 Domain (Team 2)
**Exclusive Ownership** - No Epic-025 modifications:

```rust
// NEW files (Epic-024 only)
src-tauri/src/modules/quota_monitor.rs
src-tauri/src/modules/model_selector.rs
src-tauri/src/modules/cost_tracker.rs
src-tauri/src/commands/quota.rs (extend existing)

// MODIFIED files (Epic-024 only)
src-tauri/src/modules/quota.rs          // Add monitoring hooks
src-tauri/src/proxy/token_manager.rs    // Add model selection hooks (Section: Model Selection)
```

**Frontend Files**:
```typescript
// NEW directories (Epic-024 only)
src/pages/QuotaMonitoringPage.tsx
src/components/quota/
src/components/alerts/
src/stores/useQuotaStore.ts
```

### Epic-025 Domain (Team 1)
**Exclusive Ownership** - No Epic-024 modifications:

```rust
// NEW files (Epic-025 only)
src-tauri/src/modules/budget_optimizer.rs
src-tauri/src/modules/signature_cache.rs
src-tauri/src/modules/thinking_quality.rs
src-tauri/src/modules/budget_detector.rs

// MODIFIED files (Epic-025 only)
src-tauri/src/proxy/handlers/gemini.rs         // Add budget optimization (Section: Thinking Mode)
src-tauri/src/proxy/providers/gemini_provider.rs // Integrate signature cache (Section: Signature Management)
```

**Frontend Files**:
```typescript
// NEW directories (Epic-025 only)
src/pages/BudgetOptimizerPage.tsx
src/pages/QualityDashboardPage.tsx
src/components/budget/
src/components/quality/
src/stores/useBudgetStore.ts
src/stores/useQualityStore.ts
```

### Shared Infrastructure (COORDINATION REQUIRED)
**Pre-Allocated Sections** - Teams work in separate sections:

```rust
// src-tauri/src/proxy/handlers/gemini.rs
// Section 1: Base Model Handling (Epic-024 - Team 2)
// - Lines 1-500 (existing code)
// - Model routing logic
//
// Section 2: Thinking Mode Handling (Epic-025 - Team 1)
// - Lines 501+ (new code for thinking budget)
// - Budget optimization logic
// - Sufficiency detection

// src-tauri/src/proxy/token_manager.rs
// Section 1: Token Selection (Existing)
// - Lines 1-300
//
// Section 2: Model Selection Hooks (Epic-024 - Team 2)
// - Lines 301-400 (new code)
// - Complexity-based routing
//
// Section 3: Future Extensions (Reserved)
// - Lines 401+
```

---

## 🔄 Integration Points & Interfaces

### Interface 1: Model Selection Hook (Epic-024 → Token Manager)
**Owner**: Team 2
**Timeline**: Week 2 (Feb 8-14)

```rust
// src-tauri/src/modules/model_selector.rs (NEW - Team 2)
pub trait ModelSelector {
    /// Recommends model based on request complexity
    fn recommend_model(&self, request: &Request) -> ModelRecommendation;
}

pub struct ModelRecommendation {
    pub model_id: String,        // "312" or "313"
    pub confidence: f32,         // 0.0-1.0
    pub reasoning: String,
}

// Interface contract for Team 1 (no modifications needed)
```

### Interface 2: Budget Optimization Hook (Epic-025 → Gemini Handler)
**Owner**: Team 1
**Timeline**: Weeks 1-2 (Feb 1-14)

```rust
// src-tauri/src/modules/budget_optimizer.rs (NEW - Team 1)
pub trait BudgetOptimizer {
    /// Calculates optimal thinking budget
    fn optimize_budget(&self, request: &Request) -> BudgetAllocation;
}

pub struct BudgetAllocation {
    pub budget: u32,             // 4096, 12288, or 24576
    pub tier: ComplexityTier,    // Simple, Moderate, Complex
    pub confidence: f32,
}

// Interface contract for Team 2 (no modifications needed)
```

### Interface 3: Signature Cache Hook (Epic-025 → Provider)
**Owner**: Team 1
**Timeline**: Week 3 (Feb 15-21)

```rust
// src-tauri/src/modules/signature_cache.rs (NEW - Team 1)
pub trait SignatureCache {
    /// Caches signature with LRU eviction
    fn cache_signature(&mut self, key: String, signature: CachedSignature);

    /// Retrieves and validates cached signature
    fn get_signature(&self, key: &str) -> Option<CachedSignature>;
}

pub struct CachedSignature {
    pub signature: String,
    pub created_at: DateTime<Utc>,
    pub conversation_id: String,
}

// Interface contract for Team 2 (no modifications needed)
```

---

## 📅 Weekly Coordination Schedule

### Week 1 (Feb 1-7) - Kickoff & Foundation

**Monday Feb 1 - Kickoff Meeting** (1 hour, both teams):
- Review coordination plan
- Confirm interface contracts
- Establish communication channels (Slack, daily standups)
- Create GitHub issues for all stories

**Team 1 Tasks**:
```yaml
developer_1: "Start Story-025-01 (Budget Optimizer) - backend"
developer_2: "Design SignatureCache interface"
developer_3: "Create BudgetOptimizerPage.tsx skeleton"
deliverable: "ComplexityClassifier prototype"
branch: epic-025-flash-thinking
```

**Team 2 Tasks**:
```yaml
developer_1: "Start Story-024-01 (Quota Monitor) - backend"
developer_2: "Design ModelSelector interface"
developer_3: "Create QuotaMonitoringPage.tsx skeleton"
deliverable: "QuotaMonitor service with alerts"
branch: epic-024-flash-base
```

**Friday Feb 7 - Week 1 Review** (30 min, both teams):
- Demo progress (Team 2: quota alerts, Team 1: classifier)
- Resolve any interface questions
- Plan Week 2 integration

---

### Week 2 (Feb 8-14) - Parallel Development

**Team 1 Tasks**:
```yaml
developer_1: "Continue Story-025-01 - BudgetOptimizer integration"
developer_2: "Start Story-025-02 prep (design cache schema)"
developer_3: "Complete BudgetOptimizerWidget"
deliverable: "Budget optimization working in gemini.rs (Section 2)"
branch: epic-025-flash-thinking
```

**Team 2 Tasks**:
```yaml
developer_1: "Start Story-024-02 (Adaptive Selection) - ComplexityAnalyzer"
developer_2: "Integrate ModelSelector into token_manager.rs (Section 2)"
developer_3: "Create ModelSelectorWidget"
deliverable: "Model selection integrated into proxy flow"
branch: epic-024-flash-base
```

**Friday Feb 14 - Week 2 Review** (30 min, both teams):
- Demo: Team 1 shows budget optimization results
- Demo: Team 2 shows model selection routing
- Confirm no code conflicts in shared files
- Plan Week 3 final push

---

### Week 3 (Feb 15-21) - Epic-024 Completion & Epic-025 Continues

**Team 1 Tasks**:
```yaml
developer_1: "Start Story-025-03 prep (Budget Detector design)"
developer_2: "Start Story-025-02 (Signature Cache) - full implementation"
developer_3: "Complete quality monitoring UI prep"
deliverable: "SignatureCache LRU working, Story-025-02 complete"
branch: epic-025-flash-thinking
```

**Team 2 Tasks** (FINAL WEEK):
```yaml
developer_1: "Complete Story-024-02 - testing & docs"
developer_2: "Cost tracking dashboard finalization"
developer_3: "Complete all Epic-024 UI components"
deliverable: "Epic-024 COMPLETE, ready for merge"
branch: epic-024-flash-base
```

**Friday Feb 21 - Epic-024 Completion** (1 hour, both teams):
- **Team 2 Demo**: Full Epic-024 walkthrough
- **Code Review**: Team 1 reviews Epic-024 changes
- **Merge Epic-024**: Merge `epic-024-flash-base` → `main`
- **Team 1 Rebase**: Rebase `epic-025-flash-thinking` onto new `main`
- **Team 2 Status**: Team 2 becomes available for next work

---

### Weeks 4-7 (Feb 22 - Mar 21) - Epic-025 Solo Execution

**Week 4 (Feb 22-28)**:
```yaml
team_1: "Story-025-03 (Budget Sufficiency Detection)"
focus: "finishReason monitoring, auto-escalation"
deliverable: "100% response completeness validated"
```

**Weeks 5-7 (Mar 1-21)**:
```yaml
team_1: "Story-025-04 (Thinking Quality Monitoring)"
focus: "Quality scoring, feedback aggregation, trend analysis"
deliverable: "Complete quality dashboard with 2 weeks of data"
```

**Friday Mar 21 - Epic-025 Completion**:
- **Team 1 Demo**: Full Epic-025 walkthrough
- **Final Testing**: Integration tests with Epic-024 features
- **Merge Epic-025**: Merge `epic-025-flash-thinking` → `main`
- **Celebration**: Both epics complete, 30-45% cost savings achieved! 🎉

---

## 🚨 Conflict Prevention Strategy

### Git Workflow
```bash
# Team 2 workflow (Epic-024)
git checkout epic-024-flash-base
git pull origin main  # Daily rebase
# Make changes to Epic-024 domain files only
git add <epic-024-files>
git commit -m "feat(epic-024): description"
git push origin epic-024-flash-base

# Team 1 workflow (Epic-025)
git checkout epic-025-flash-thinking
git pull origin main  # Daily rebase
# Make changes to Epic-025 domain files only
git add <epic-025-files>
git commit -m "feat(epic-025): description"
git push origin epic-025-flash-thinking
```

### Shared File Modification Protocol
**For files in both domains** (gemini.rs, token_manager.rs):

1. **Section Comments**: Add clear section markers
   ```rust
   // ========== EPIC-024: Model Selection (Team 2) ==========
   // Lines 301-400 reserved for model selection logic
   // DO NOT MODIFY: Team 1 (Epic-025)

   // ========== EPIC-025: Budget Optimization (Team 1) ==========
   // Lines 501-700 reserved for budget optimization
   // DO NOT MODIFY: Team 2 (Epic-024)
   ```

2. **Interface-Based Extension**: Modify via trait implementations, not direct code changes
   ```rust
   // Good: Team 1 adds new trait implementation
   impl BudgetOptimizer for GeminiBudgetOptimizer { ... }

   // Bad: Team 1 modifies existing TokenManager methods
   ```

3. **Daily Sync**: Both teams check for conflicts daily
   ```bash
   git fetch origin main
   git merge origin/main  # Resolve any conflicts immediately
   ```

### Communication Channels

**Slack Channels**:
- `#epic-024-team2` - Team 2 internal
- `#epic-025-team1` - Team 1 internal
- `#epic-024-025-coordination` - Cross-team sync (both teams)

**Daily Standups** (15 min, async in Slack):
- What did you complete yesterday?
- What are you working on today?
- Any blockers or potential conflicts?

**Weekly Syncs** (30-60 min, both teams):
- Friday afternoons (Feb 7, 14, 21)
- Demo progress, review interfaces, resolve issues

---

## 📊 Database Schema Coordination

### Team 2 - Epic-024 Tables (Exclusive)
```sql
-- Created by Team 2, no Epic-025 modifications
CREATE TABLE quota_history (
    id INTEGER PRIMARY KEY,
    account_id TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    tokens_used INTEGER NOT NULL,
    tokens_remaining INTEGER NOT NULL,
    reset_time DATETIME
);

CREATE TABLE quota_alerts (
    id INTEGER PRIMARY KEY,
    account_id TEXT NOT NULL,
    threshold REAL NOT NULL,
    triggered_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    resolved_at DATETIME,
    mitigation_action TEXT
);
```

### Team 1 - Epic-025 Tables (Exclusive)
```sql
-- Created by Team 1, no Epic-024 modifications
CREATE TABLE budget_classifications (
    id INTEGER PRIMARY KEY,
    request_id TEXT UNIQUE NOT NULL,
    complexity_tier TEXT NOT NULL,
    assigned_budget INTEGER NOT NULL,
    confidence REAL NOT NULL,
    features TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE thinking_quality_metrics (
    id INTEGER PRIMARY KEY,
    request_id TEXT UNIQUE NOT NULL,
    thinking_tokens INTEGER NOT NULL,
    output_tokens INTEGER NOT NULL,
    efficiency_score REAL NOT NULL,
    quality_score REAL NOT NULL,
    user_rating REAL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Migration Files**:
- `migrations/2026-02-01-epic-024-quota-monitoring.sql` (Team 2)
- `migrations/2026-02-01-epic-025-budget-optimization.sql` (Team 1)

---

## 🧪 Testing Strategy

### Unit Tests (Per Team)
**Team 2 - Epic-024**:
```rust
#[cfg(test)]
mod quota_monitor_tests {
    #[test]
    fn test_alert_threshold_80_percent() { ... }

    #[test]
    fn test_auto_mitigation_account_switch() { ... }
}

#[cfg(test)]
mod model_selector_tests {
    #[test]
    fn test_complexity_classification_simple() { ... }

    #[test]
    fn test_recommend_base_model_312() { ... }
}
```

**Team 1 - Epic-025**:
```rust
#[cfg(test)]
mod budget_optimizer_tests {
    #[test]
    fn test_budget_allocation_simple_4k() { ... }

    #[test]
    fn test_cost_savings_calculation() { ... }
}

#[cfg(test)]
mod signature_cache_tests {
    #[test]
    fn test_lru_eviction() { ... }

    #[test]
    fn test_signature_validation() { ... }
}
```

### Integration Tests (Cross-Team)
**Week 3 (After Epic-024 merge)**:
```rust
#[test]
fn test_epic_024_025_integration() {
    // Team 1 + Team 2 collaboration
    // Test: Model selection (024) + Budget optimization (025) work together
    let request = create_complex_request();

    // Epic-024: Recommend model
    let model_rec = model_selector.recommend_model(&request);
    assert_eq!(model_rec.model_id, "313"); // Recommends thinking model

    // Epic-025: Optimize budget
    let budget_alloc = budget_optimizer.optimize_budget(&request);
    assert_eq!(budget_alloc.budget, 12288); // Allocates moderate budget
}
```

---

## ✅ Success Criteria

### Team 2 - Epic-024 (3 weeks)
```yaml
story_024_01_complete:
  - [ ] QuotaMonitor service working
  - [ ] Multi-threshold alerts (80%, 90%, 95%)
  - [ ] Auto-mitigation tested
  - [ ] UI dashboard integrated
  - [ ] ≥80% test coverage

story_024_02_complete:
  - [ ] ComplexityAnalyzer working
  - [ ] Model routing tested (312 vs 313)
  - [ ] 10-15% cost savings validated
  - [ ] UI selector widget complete
  - [ ] Integration tests passing

epic_024_merge_ready:
  - [ ] All tests passing (unit + integration)
  - [ ] Code review approved by Team 1
  - [ ] Documentation complete
  - [ ] No conflicts with main branch
  - [ ] Performance benchmarks met
```

### Team 1 - Epic-025 (7 weeks)
```yaml
story_025_01_complete:
  - [ ] ComplexityClassifier working (>80% accuracy)
  - [ ] BudgetOptimizer integrated
  - [ ] 20-30% cost savings validated
  - [ ] Quality preservation >95%
  - [ ] Dashboard with savings metrics

story_025_02_complete:
  - [ ] SignatureCache LRU working
  - [ ] >80% cache hit rate
  - [ ] Auto-regeneration tested
  - [ ] Metrics dashboard complete

story_025_03_complete:
  - [ ] finishReason monitoring working
  - [ ] Auto-escalation tested (4K→12K→24K)
  - [ ] 100% completeness validated
  - [ ] Escalation dashboard complete

story_025_04_complete:
  - [ ] Quality scoring working
  - [ ] First-time-right rate >90%
  - [ ] Weekly tuning process documented
  - [ ] Trend analysis dashboard complete

epic_025_merge_ready:
  - [ ] All tests passing (unit + integration)
  - [ ] Integration with Epic-024 validated
  - [ ] Code review approved
  - [ ] Documentation complete
  - [ ] 30-45% combined savings achieved
```

---

## 🔧 Development Environment Setup

### Prerequisites (Both Teams)
```bash
# Rust toolchain
rustc --version  # 1.70+
cargo --version

# Node.js
node --version  # v18+
npm --version

# Tauri CLI
cargo install tauri-cli

# Dependencies
cd src-tauri
cargo build  # Verify Rust builds
cd ..
npm install  # Verify Node modules
```

### Branch Setup
```bash
# Team 2
git checkout epic-024-flash-base
git pull origin main  # Start from latest main

# Team 1
git checkout epic-025-flash-thinking
git pull origin main  # Start from latest main
```

### IDE Configuration
**VS Code Extensions**:
- rust-analyzer (Rust support)
- Tauri (Tauri integration)
- ESLint + Prettier (TypeScript formatting)
- GitLens (Git visualization)

**Recommended Settings**:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "files.exclude": {
    "**/node_modules": true,
    "**/target": true
  }
}
```

---

## 📚 Reference Documentation

### Epic-024 Resources (Team 2)
1. `EPIC-024-IMPLEMENTATION-PLAN.md` - Detailed implementation guide
2. `gemini-2.5-flash-COMPARISON.md` - Model 312 analysis
3. `gemini-2.5-flash-workflow.md` - Base model workflow

### Epic-025 Resources (Team 1)
1. `EPIC-025-IMPLEMENTATION-PLAN.md` - Detailed implementation guide
2. `gemini-2.5-flash-thinking-COMPARISON.md` - Model 313 analysis
3. `gemini-2.5-flash-thinking-workflow.md` - Thinking mode workflow
4. `Epic-015-Gemini-2.5-Pro-Thinking-Optimization.md` - Proven patterns (16.4% savings)

### Shared Resources (Both Teams)
1. `EPIC-024-025-PREP-PHASE-COMPLETE.md` - This coordination plan
2. `MASTER-MODELS-TABLE.md` - Model catalog
3. Tauri documentation: https://tauri.app/v2/

---

## 🚀 Ready to Start!

**Status**: ✅ COORDINATION PLAN COMPLETE

**Next Actions**:
1. ✅ Git branches created (`epic-024-flash-base`, `epic-025-flash-thinking`)
2. ✅ Domain separation defined (zero conflict zones)
3. ✅ Interface contracts established
4. ✅ Weekly schedule planned
5. ✅ Communication channels ready

**Feb 1, 2026 Kickoff**:
- 9:00 AM: Team 2 standup (Epic-024)
- 10:00 AM: Team 1 standup (Epic-025)
- 11:00 AM: Combined kickoff meeting (both teams)

---

**Document Version**: 1.0
**Author**: Tech Lead
**Date**: 2026-01-27
**Status**: ✅ APPROVED AND READY FOR EXECUTION
