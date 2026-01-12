# Story-012-02: Epic-008 Budget Pattern Integration

**Epic**: EPIC-012 (CI/CD Remediation)
**Priority**: P0 CRITICAL 🔥
**Effort**: 2-3 hours
**Owner**: Developer B (Epic-008 Budget Optimizer expert)
**Status**: READY
**Dependencies**: Story-012-01 complete

---

## 📋 Story Description

**CRITICAL INTEGRATION GAP**: Epic-008 Story-008-01 implemented complete database persistence infrastructure for budget pattern learning, but **NEVER wired it to production code**. This story connects all the pieces to achieve the advertised "learning budget optimizer" feature.

**Current State** (~85% complete):
- ✅ Database tables created (`budget_patterns`)
- ✅ Database functions implemented (`save_budget_pattern`, `load_budget_patterns`)
- ✅ BudgetOptimizer has `record_feedback()` method
- ✅ PatternStore has persistence methods
- ❌ **NOT wired to startup** (patterns never loaded)
- ❌ **NOT wired to runtime** (feedback never recorded)
- ❌ **NOT wired to periodic save** (patterns never persisted)

**Target State** (100% complete):
- ✅ Patterns loaded from database on proxy startup
- ✅ Feedback recorded after each request completion
- ✅ Patterns saved to database every 5 minutes
- ✅ Graceful degradation if database unavailable

---

## 🎯 Acceptance Criteria

### AC1: Patterns Loaded on Startup ✅
**Given** proxy server starts
**When** BudgetOptimizer initializes
**Then** existing patterns are loaded from database AND PatternStore is populated AND startup logs confirm restoration

**Verification**:
```bash
# Check logs for pattern loading
tail -f logs/proxy.log | grep "budget pattern"
# Expected: "Loaded 42 budget patterns from database"
```

**Code Location**: `src/proxy/server.rs` (startup sequence)

---

### AC2: Feedback Recorded After Requests ✅
**Given** request completes with thinking budget
**When** response is sent to client
**Then** `record_feedback()` is called with prompt, budget_used, quality_score AND feedback is processed asynchronously AND no performance degradation

**Verification**:
```bash
# Check logs for feedback recording
tail -f logs/proxy.log | grep "feedback recorded"
# Expected: "Budget feedback recorded: prompt_hash=abc123, budget=16000, quality=0.85"
```

**Code Location**:
- `src/proxy/handlers/openai.rs` (OpenAI completion handler)
- `src/proxy/handlers/claude.rs` (Claude message handler)

---

### AC3: Patterns Persisted Periodically ✅
**Given** proxy server running
**When** 5 minutes elapse
**Then** updated patterns are saved to database AND save operation is async (non-blocking) AND errors are logged but don't crash proxy

**Verification**:
```bash
# Check database for new patterns
sqlite3 antigravity.db "SELECT COUNT(*) FROM budget_patterns"
# Expected: Count increases over time as patterns learned
```

**Code Location**: `src/proxy/server.rs` (background task)

---

### AC4: Graceful Degradation on Database Failure ✅
**Given** database is unavailable (file locked, disk full, etc.)
**When** pattern load/save fails
**Then** proxy continues operating normally AND errors are logged AND in-memory patterns still used AND quality_score defaults applied

**Verification**:
```bash
# Simulate database failure
chmod 000 antigravity.db

# Check proxy still works
curl -X POST http://localhost:8045/v1/chat/completions \
  -d '{"model": "gemini-3-pro-high", "messages": [...]}'
# Expected: Request succeeds despite database unavailable
```

**Code Location**: Error handling in all database calls

---

### AC5: Integration Tests Pass ✅
**Given** integration tests created
**When** `cargo test --lib budget_pattern_integration` runs
**Then** all tests pass validating:
- Pattern loading from database
- Feedback recording lifecycle
- Pattern persistence
- Error handling

**Code Location**: `src/proxy/tests/budget_pattern_integration_tests.rs` (new file)

---

### AC6: Clippy Errors Eliminated ✅
**Given** integration complete
**When** `cargo clippy -- -D warnings` runs
**Then** 10 budget optimizer dead code errors eliminated (125 → 115 total)

**Errors Fixed**:
- `feedback_processor` field unused
- `record_feedback()` method unused
- `get_pattern_store()` method unused
- `save_pattern()` method unused
- `get_pattern()` method unused
- `load_from_db()` method unused
- `get_all_patterns()` method unused
- `FeedbackProcessor::process()` method unused
- `proxy_db::save_budget_pattern()` function unused
- `proxy_db::load_budget_patterns()` function unused

---

## 🛠️ Implementation Tasks

### Task 1: Wire Pattern Loading to Startup

**File**: `src/proxy/server.rs`
**Function**: `start_server()` or equivalent

**Current Code** (hypothetical):
```rust
pub async fn start_server(config: ProxyConfig) -> Result<(), String> {
    // Initialize budget optimizer
    let optimizer = BudgetOptimizer::new();

    // Start HTTP server
    // ...
}
```

**Updated Code**:
```rust
use crate::modules::proxy_db;

pub async fn start_server(config: ProxyConfig) -> Result<(), String> {
    // Initialize budget optimizer
    let optimizer = BudgetOptimizer::new();

    // Load saved patterns from database
    match proxy_db::load_budget_patterns() {
        Ok(patterns) => {
            if !patterns.is_empty() {
                match optimizer.get_pattern_store().write() {
                    Ok(mut store) => {
                        store.load_from_db(patterns.clone());
                        tracing::info!(
                            "Loaded {} budget patterns from database",
                            patterns.len()
                        );
                    }
                    Err(e) => {
                        tracing::warn!("Failed to acquire pattern store lock: {}", e);
                    }
                }
            } else {
                tracing::info!("No budget patterns found in database (first run)");
            }
        }
        Err(e) => {
            // Graceful degradation: Log error but continue startup
            tracing::warn!("Failed to load budget patterns from database: {}", e);
            tracing::info!("Budget optimizer will use default patterns only");
        }
    }

    // Start periodic pattern persistence task
    start_pattern_persistence_task(optimizer.clone());

    // Start HTTP server
    // ...
}
```

**Error Handling**:
- Database unavailable → Log warning, use defaults
- Empty database → Log info (first run), continue
- Lock acquisition fails → Log warning, retry next cycle

---

### Task 2: Create Periodic Persistence Task

**File**: `src/proxy/server.rs`
**New Function**: `start_pattern_persistence_task()`

**Implementation**:
```rust
use std::time::Duration;
use tokio::time::interval;

fn start_pattern_persistence_task(optimizer: Arc<BudgetOptimizer>) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(300)); // 5 minutes

        loop {
            interval.tick().await;

            tracing::debug!("Starting periodic budget pattern persistence...");

            // Get all patterns from PatternStore
            let patterns = match optimizer.get_pattern_store().read() {
                Ok(store) => store.get_all_patterns(),
                Err(e) => {
                    tracing::error!("Failed to read pattern store: {}", e);
                    continue; // Skip this cycle, try again in 5 minutes
                }
            };

            if patterns.is_empty() {
                tracing::debug!("No patterns to persist (empty store)");
                continue;
            }

            // Save each pattern to database (async, non-blocking)
            let mut saved_count = 0;
            let mut error_count = 0;

            for pattern in patterns {
                match proxy_db::save_budget_pattern(&pattern) {
                    Ok(_) => saved_count += 1,
                    Err(e) => {
                        error_count += 1;
                        tracing::warn!(
                            "Failed to save pattern {}: {}",
                            pattern.prompt_hash,
                            e
                        );
                    }
                }
            }

            if saved_count > 0 {
                tracing::info!(
                    "Persisted {} budget patterns to database ({} errors)",
                    saved_count,
                    error_count
                );
            }

            if error_count > 0 {
                tracing::warn!(
                    "Failed to persist {} patterns (database may be unavailable)",
                    error_count
                );
            }
        }
    });

    tracing::info!("Budget pattern persistence task started (5-minute interval)");
}
```

**Error Handling**:
- Pattern store lock fails → Skip cycle, log error
- Database write fails → Log warning, continue with other patterns
- Task panic → Tokio restarts task automatically

---

### Task 3: Wire Feedback Recording to OpenAI Handler

**File**: `src/proxy/handlers/openai.rs`
**Function**: OpenAI completion handler (after response sent)

**Current Code** (hypothetical):
```rust
pub async fn handle_openai_completion(
    req: OpenAIRequest,
    optimizer: Arc<BudgetOptimizer>,
) -> Result<Response, String> {
    // Process request
    let response = process_request(&req).await?;

    // Return response
    Ok(response)
}
```

**Updated Code**:
```rust
pub async fn handle_openai_completion(
    req: OpenAIRequest,
    optimizer: Arc<BudgetOptimizer>,
) -> Result<Response, String> {
    // Extract prompt for feedback
    let prompt = extract_prompt_from_request(&req);

    // Process request
    let response = process_request(&req).await?;

    // Extract budget and quality from response
    let budget_used = extract_budget_from_response(&response);
    let quality_score = calculate_quality_score(&response);

    // Record feedback asynchronously (non-blocking)
    let optimizer_clone = optimizer.clone();
    let prompt_clone = prompt.clone();
    tokio::spawn(async move {
        optimizer_clone.record_feedback(
            &prompt_clone,
            budget_used,
            quality_score
        );

        tracing::debug!(
            "Budget feedback recorded: prompt_hash={}, budget={}, quality={:.2}",
            hash_prompt(&prompt_clone),
            budget_used,
            quality_score
        );
    });

    // Return response (don't wait for feedback recording)
    Ok(response)
}

// Helper function: Calculate quality score from response
fn calculate_quality_score(response: &Response) -> f32 {
    // Heuristic: Response length, thinking depth, etc.
    // For now, use simple heuristic:
    let has_thinking = response.choices.iter().any(|c| c.thinking.is_some());
    if has_thinking {
        0.85 // Good quality (included thinking)
    } else {
        0.65 // Standard quality (no thinking)
    }
}

// Helper function: Extract budget from response
fn extract_budget_from_response(response: &Response) -> u32 {
    response.usage
        .as_ref()
        .and_then(|u| u.thinking_tokens)
        .unwrap_or(0) as u32
}
```

**Performance Considerations**:
- Feedback recording is **async** (tokio::spawn) → No latency impact
- Quality score calculation is **heuristic** → Fast, no complex computation
- Database writes are **batched** (periodic task) → No per-request overhead

---

### Task 4: Wire Feedback Recording to Claude Handler

**File**: `src/proxy/handlers/claude.rs`
**Function**: Claude message handler (after response sent)

**Implementation**: Similar to Task 3, adapted for Claude protocol

**Key Differences**:
- Claude uses `thinking` object in response, not `thinking_tokens`
- Quality score based on thinking depth (extended vs standard)

**Code** (similar structure to OpenAI handler):
```rust
pub async fn handle_claude_message(
    req: ClaudeRequest,
    optimizer: Arc<BudgetOptimizer>,
) -> Result<Response, String> {
    // Extract prompt
    let prompt = extract_prompt_from_claude_request(&req);

    // Process request
    let response = process_claude_request(&req).await?;

    // Extract budget and quality
    let budget_used = extract_thinking_budget_from_claude_response(&response);
    let quality_score = calculate_claude_quality_score(&response);

    // Record feedback asynchronously
    let optimizer_clone = optimizer.clone();
    let prompt_clone = prompt.clone();
    tokio::spawn(async move {
        optimizer_clone.record_feedback(
            &prompt_clone,
            budget_used,
            quality_score
        );
    });

    Ok(response)
}
```

---

### Task 5: Update BudgetOptimizer Constructor

**File**: `src/proxy/budget_optimizer.rs`
**Function**: `BudgetOptimizer::new()`

**Current Code**:
```rust
impl BudgetOptimizer {
    pub fn new() -> Self {
        Self {
            pattern_store: Arc::new(RwLock::new(PatternStore::new())),
            feedback_processor: FeedbackProcessor::new(),
            // ...
        }
    }
}
```

**Potential Update** (if constructor needs database access):
```rust
impl BudgetOptimizer {
    pub fn new() -> Self {
        Self {
            pattern_store: Arc::new(RwLock::new(PatternStore::new())),
            feedback_processor: FeedbackProcessor::new(),
            // ...
        }
    }

    /// Load patterns from database into this optimizer
    pub fn load_patterns_from_db(&self) -> Result<(), String> {
        let patterns = crate::modules::proxy_db::load_budget_patterns()?;
        let mut store = self.pattern_store.write()
            .map_err(|e| format!("Lock error: {}", e))?;
        store.load_from_db(patterns);
        Ok(())
    }
}
```

**Note**: This is OPTIONAL - can also load directly from server.rs as shown in Task 1

---

### Task 6: Create Integration Tests

**File**: `src/proxy/tests/budget_pattern_integration_tests.rs` (NEW)

**Test Suite**:
```rust
#[cfg(test)]
mod budget_pattern_integration_tests {
    use super::*;
    use crate::proxy::budget_optimizer::{BudgetOptimizer, BudgetPattern};
    use crate::modules::proxy_db;

    #[test]
    fn test_pattern_persistence_lifecycle() {
        // 1. Create optimizer and record feedback
        let optimizer = BudgetOptimizer::new();
        optimizer.record_feedback("test prompt", 16000, 0.85);

        // 2. Get patterns and save to database
        let patterns = optimizer.get_pattern_store()
            .read().unwrap()
            .get_all_patterns();

        assert!(!patterns.is_empty(), "Patterns should be created from feedback");

        for pattern in &patterns {
            proxy_db::save_budget_pattern(pattern).expect("Save should succeed");
        }

        // 3. Load patterns from database
        let loaded = proxy_db::load_budget_patterns().expect("Load should succeed");
        assert_eq!(loaded.len(), patterns.len(), "Should load same number of patterns");

        // 4. Create new optimizer and load patterns
        let new_optimizer = BudgetOptimizer::new();
        new_optimizer.get_pattern_store()
            .write().unwrap()
            .load_from_db(loaded);

        let restored = new_optimizer.get_pattern_store()
            .read().unwrap()
            .get_all_patterns();

        assert_eq!(restored.len(), patterns.len(), "Patterns should be restored");
    }

    #[test]
    fn test_feedback_recording() {
        let optimizer = BudgetOptimizer::new();

        // Record feedback for multiple prompts
        optimizer.record_feedback("prompt 1", 8000, 0.7);
        optimizer.record_feedback("prompt 2", 16000, 0.85);
        optimizer.record_feedback("prompt 3", 24000, 0.95);

        // Verify patterns created
        let patterns = optimizer.get_pattern_store()
            .read().unwrap()
            .get_all_patterns();

        assert_eq!(patterns.len(), 3, "Should have 3 patterns");
    }

    #[test]
    fn test_graceful_degradation_on_database_failure() {
        let optimizer = BudgetOptimizer::new();

        // Simulate database failure by loading from non-existent file
        let result = proxy_db::load_budget_patterns();

        // Should return error but not panic
        assert!(result.is_err() || result.unwrap().is_empty());

        // Optimizer should still work with defaults
        let budget = optimizer.calculate_budget("test prompt");
        assert!(budget > 0, "Should still calculate budgets");
    }

    #[test]
    fn test_pattern_store_methods() {
        let mut store = PatternStore::new();

        // Test save_pattern
        let pattern = BudgetPattern {
            prompt_hash: "hash123".to_string(),
            avg_budget: 16000,
            quality_score: 0.85,
            usage_count: 5,
        };
        store.save_pattern(pattern.clone());

        // Test get_pattern
        let retrieved = store.get_pattern("hash123");
        assert!(retrieved.is_some(), "Pattern should be retrievable");

        // Test get_all_patterns
        let all = store.get_all_patterns();
        assert_eq!(all.len(), 1, "Should have 1 pattern");
    }
}
```

**Test Coverage**:
- [ ] Pattern persistence lifecycle (create → save → load → restore)
- [ ] Feedback recording creates patterns
- [ ] Graceful degradation on database failure
- [ ] PatternStore methods work correctly
- [ ] Async operations don't block

---

## 📊 Impact Analysis

### Clippy Errors Fixed: 10

**Before**: 125 total errors
**After**: 115 total errors (10 eliminated)

**Fixed Errors**:
1. `feedback_processor` field unused → NOW USED in `record_feedback()`
2. `record_feedback()` method unused → NOW CALLED from handlers
3. `get_pattern_store()` method unused → NOW USED for loading/saving
4. `save_pattern()` method unused → NOW USED in `load_from_db()`
5. `get_pattern()` method unused → NOW USED for pattern retrieval
6. `load_from_db()` method unused → NOW CALLED on startup
7. `get_all_patterns()` method unused → NOW USED in periodic task
8. `FeedbackProcessor::process()` method unused → NOW CALLED from `record_feedback()`
9. `proxy_db::save_budget_pattern()` unused → NOW CALLED in periodic task
10. `proxy_db::load_budget_patterns()` unused → NOW CALLED on startup

### Epic-008 Completeness

**Before**: ~85% (core features work, persistence missing)
**After**: 100% (full learning budget optimizer with persistence)

**Advertised Features**:
- ✅ Adaptive budget optimization (ALREADY WORKING)
- ✅ Signature cache monitoring (ALREADY WORKING)
- ✅ **Pattern persistence** (NOW WIRED)
- ✅ **Feedback loop** (NOW WIRED)

### Performance Impact

**Latency**: ZERO (feedback recording is async)
**Memory**: +~100KB per 1000 patterns (negligible)
**Disk**: +~1MB per 10,000 patterns (acceptable)
**CPU**: Minimal (periodic saves every 5 minutes)

---

## 🧪 Testing Strategy

### Unit Tests

Run existing budget optimizer tests:
```bash
cargo test --lib budget_optimizer
cargo test --lib proxy_db
```

**Expected**: All tests pass (no regressions)

### Integration Tests

Run new integration tests:
```bash
cargo test --lib budget_pattern_integration
```

**Expected**: 5 integration tests pass

### Manual Testing

**Test 1: Pattern Persistence Lifecycle**
```bash
# 1. Start proxy
cargo run --bin proxy

# 2. Send requests with thinking
curl -X POST http://localhost:8045/v1/chat/completions \
  -d '{"model": "gemini-3-pro-high", "messages": [...], "thinking_budget": 16000}'

# 3. Wait 5 minutes for periodic save

# 4. Check database
sqlite3 antigravity.db "SELECT * FROM budget_patterns"
# Expected: Patterns saved

# 5. Restart proxy
# Expected: Logs show "Loaded N patterns from database"
```

**Test 2: Graceful Degradation**
```bash
# 1. Lock database
chmod 000 antigravity.db

# 2. Start proxy
# Expected: Logs show "Failed to load patterns" but proxy starts

# 3. Send requests
# Expected: Requests work with default budgets
```

---

## 📝 Code Review Checklist

### Before Submitting PR

- [ ] Pattern loading wired to startup (`server.rs`)
- [ ] Periodic persistence task created and started
- [ ] Feedback recording wired to OpenAI handler
- [ ] Feedback recording wired to Claude handler
- [ ] Error handling for all database operations
- [ ] Integration tests created and passing
- [ ] `cargo clippy -- -D warnings` shows 10 fewer errors
- [ ] `cargo test --lib` passes (279 + 5 new tests)
- [ ] Manual testing completed successfully

### Code Review Focus

**Reviewer Should Verify**:
1. Async operations don't block request handling
2. Error handling is comprehensive (graceful degradation)
3. Pattern store locking is safe (no deadlocks)
4. Periodic task doesn't leak memory
5. Integration tests are comprehensive
6. Performance impact is minimal

---

## 📈 Definition of Done

### Code Complete ✅
- [ ] Pattern loading on startup implemented
- [ ] Periodic persistence task implemented
- [ ] Feedback recording in OpenAI handler
- [ ] Feedback recording in Claude handler
- [ ] Error handling complete

### Testing Complete ✅
- [ ] 5 integration tests passing
- [ ] All existing tests passing (279/279)
- [ ] Manual testing successful
- [ ] Clippy errors reduced by 10 (125 → 115)

### Documentation Complete ✅
- [ ] Code comments explain integration points
- [ ] Error handling documented
- [ ] Performance impact documented

### Deployment Ready ✅
- [ ] PR approved by Epic-008 original author
- [ ] Changes merged to epic-012 branch
- [ ] Epic-008 now 100% complete
- [ ] Ready for Story-012-03

---

## 🚀 Rollback Plan

**If Integration Breaks Production**:

**Scenario 1**: Performance degradation
- **Action**: Disable periodic saves (comment out task)
- **Workaround**: Manual pattern dumps on shutdown only
- **Fix**: Optimize save logic, increase interval

**Scenario 2**: Database locking issues
- **Action**: Add `#[allow(dead_code)]` to problematic methods
- **Workaround**: Defer persistence to Epic-013
- **Fix**: Implement better lock strategy

**Scenario 3**: Memory leak in periodic task
- **Action**: Kill periodic task, restart proxy
- **Workaround**: Manual pattern saves only
- **Fix**: Fix task cleanup logic

**Risk**: **MODERATE** (integration complex, but well-tested)

---

## 📞 Handoff

### From Developer B (Story Owner)

**When Complete**:
1. Create PR: `feat(epic-012): wire Epic-008 budget pattern persistence (Story-012-02)`
2. Tag Project Manager for review
3. Provide clippy error count BEFORE/AFTER
4. Confirm integration tests passing
5. Provide manual testing evidence (logs, database snapshots)

**PR Description Template**:
```markdown
## Story-012-02: Epic-008 Budget Pattern Integration

**Epic-008 Completeness**: 85% → 100% ✅

### Changes
- ✅ Wired pattern loading to proxy startup
- ✅ Created periodic persistence task (5-minute interval)
- ✅ Wired feedback recording to OpenAI handler
- ✅ Wired feedback recording to Claude handler
- ✅ Added comprehensive error handling
- ✅ Created 5 integration tests

### Clippy Errors Fixed
- Before: 125 errors
- After: 115 errors
- **Reduction**: 10 errors eliminated

### Testing
- ✅ 284 tests passing (279 existing + 5 new integration tests)
- ✅ Manual testing: Pattern persistence lifecycle verified
- ✅ Manual testing: Graceful degradation verified
- ✅ Performance impact: ZERO latency (async operations)

### Epic-008 Now FULLY COMPLETE
- ✅ Adaptive budget optimization
- ✅ Signature cache monitoring
- ✅ Pattern persistence (NOW WIRED)
- ✅ Feedback loop (NOW WIRED)

### Next Steps
Ready for Story-012-03 (Cache Metrics Restoration)
```

---

**Story Owner**: Developer B
**Reviewer**: Project Manager + Developer A
**Estimated Duration**: 2-3 hours
**Status**: READY FOR IMPLEMENTATION
**Priority**: P0 CRITICAL 🔥
