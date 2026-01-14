# Story-007-03: Enhanced Error Logging - Progress Report

**Story**: Story-007-03
**Epic**: Epic-007-Gemini-3-Pro-Image-Compliance
**Developer**: Developer A (Backend Specialist)
**Started**: 2026-01-11
**Priority**: P2
**Estimated Effort**: 1 day
**Branch**: `epic-007-gemini-pro-image`

---

## 📋 Story Overview

### Objective
Implement comprehensive error logging for Gemini 3 Pro Image generation with structured fields, error categorization, and user-friendly messages to improve debuggability and operations.

### Dependencies
- **Builds on**: Story-007-02 (Safety Settings Configuration) ✅ COMPLETE
- **Required by**: Story-007-04 (Caching Layer - weak dependency)
- **Parallel with**: Story-007-01 (E2E Testing)

---

## ✅ Acceptance Criteria

### AC-1: Structured Error Logging
- [ ] All error logs include structured fields (request_id, model, account, etc.)
- [ ] Error categorization helper (`categorize_error()`)
- [ ] Prompt hashing helper (`hash_prompt()`) for privacy
- [ ] Generation time tracking
- [ ] Retry count tracking

### AC-2: Error Categorization
- [ ] USER_ERROR: Client validation errors (400 Bad Request)
- [ ] API_ERROR: Upstream API errors (429, 503, etc.)
- [ ] SYSTEM_ERROR: Internal server errors (500)
- [ ] NETWORK_ERROR: Connection/timeout errors

### AC-3: User-Friendly Error Messages
- [ ] Clear error messages for safety filter blocks
- [ ] Helpful resolution suggestions
- [ ] Error code mapping

### AC-4: Testing
- [ ] Unit tests for error helpers (categorization, hashing)
- [ ] Integration tests with mocked error scenarios
- [ ] Error logging verification

### AC-5: Documentation
- [ ] Error handling documentation
- [ ] Troubleshooting guide with common errors
- [ ] Error code reference
- [ ] Log query examples

---

## 🚀 Implementation Plan

### Phase 1: Helper Functions (Morning)
- [ ] Create `src-tauri/src/proxy/errors.rs` module
- [ ] Implement `categorize_error(status_code: u16, error_text: &str) -> &str`
- [ ] Implement `hash_prompt(prompt: &str) -> String` (SHA256, first 16 chars)
- [ ] Implement `format_error_message(category: &str, details: &str) -> String`
- [ ] Unit tests for helpers

### Phase 2: Structured Logging Integration (Afternoon)
- [ ] Update `handle_images_generations()` error blocks
- [ ] Update `handle_images_edits()` error blocks
- [ ] Add generation time tracking (start/end timestamps)
- [ ] Add retry count tracking
- [ ] Add all structured fields per execution plan

### Phase 3: Testing & Documentation (Evening)
- [ ] Write 5+ unit tests for error scenarios
- [ ] Update integration tests with error validation
- [ ] Create troubleshooting guide
- [ ] Document error code mappings
- [ ] Create Grafana query examples

---

## 📊 Progress Tracking

### Files Modified
- [ ] `src-tauri/src/proxy/errors.rs` (new, ~200 lines)
- [ ] `src-tauri/src/proxy/handlers/openai.rs` (update error handling)
- [ ] `src-tauri/tests/error_logging_tests.rs` (new, ~150 lines)

### Tests Status
- Unit tests: 0/5
- Integration tests: 0/2
- All tests passing: ⏳

### Code Quality
- [ ] `cargo clippy` - no warnings
- [ ] `cargo fmt --check` - formatting correct
- [ ] Code review ready

---

## 🔍 Technical Implementation Notes

### Structured Fields (Per Execution Plan)

**Required Fields**:
```rust
error!(
    error_type = "quota|client|server|network",
    retry_count = 2,
    account_email = account,
    model = "gemini-3-pro-image-4k-16x9",
    prompt_hash = hash_prompt(&prompt),
    generation_time_ms = duration.as_millis(),
    "Error message here"
);
```

**Optional Fields** (when available):
```rust
    resolution = "4K",
    aspect_ratio = "16:9",
    quality = "hd",
    style = "vivid",
    n = 4,
    safety_threshold = "MEDIUM",
```

### Error Categorization Logic

```rust
fn categorize_error(status: u16, error_text: &str) -> &'static str {
    match status {
        400 => "USER_ERROR",
        401 | 403 => if error_text.contains("QUOTA") { "API_ERROR" } else { "USER_ERROR" },
        429 => "API_ERROR",
        500..=599 => "SYSTEM_ERROR",
        _ => if error_text.contains("timeout") { "NETWORK_ERROR" } else { "SYSTEM_ERROR" }
    }
}
```

### Privacy: Prompt Hashing

```rust
use sha2::{Sha256, Digest};

fn hash_prompt(prompt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prompt.as_bytes());
    let result = hasher.finalize();
    // First 16 characters of hex digest
    format!("{:x}", result)[..16].to_string()
}
```

---

## 🐛 Issues & Blockers

### Current Blockers
- None

### Risks Identified
- None

### Open Questions
- None

---

## 📝 Daily Log

### 2026-01-11 (Day 1)

**Morning Session**:
- Analyzed Story-007-03 requirements from Epic-007-TEAM-EXECUTION-PLAN.md
- Examined existing error handling in `openai.rs`
- Identified 8 error! macro call sites in image handlers
- Created progress report document

**Afternoon Session**:
- TBD

**Evening Session**:
- TBD

**Blockers**: None
**Next Steps**: Implement error categorization helpers

---

## ✅ Completion Criteria

### Before Marking Complete
- [ ] All acceptance criteria met (AC-1 through AC-5)
- [ ] All tests passing (unit + integration)
- [ ] No clippy warnings
- [ ] Formatting correct
- [ ] Documentation complete
- [ ] Code review ready
- [ ] Completion report created

---

**Status**: 🔄 IN PROGRESS
**Last Updated**: 2026-01-11 (Start of Day 1)
**Next Update**: After Phase 1 completion
