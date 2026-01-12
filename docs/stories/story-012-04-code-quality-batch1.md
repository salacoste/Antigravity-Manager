# Story-012-04: Code Quality Batch 1 (Patterns & Idioms)

**Epic**: EPIC-012
**Priority**: P1 HIGH
**Effort**: 2-3 hours
**Owner**: Developer D
**Dependencies**: Stories 012-01, 012-02, 012-03 complete

---

## 📋 Description

Fix idiomatic Rust pattern violations (33 errors) using automated tools and manual review.

---

## 🎯 Tasks

### Task 1: Simplify `.map_or()` → `.unwrap_or()` (11 errors)

**Auto-fix**:
```bash
cargo clippy --fix -- -A clippy::all -W clippy::map_unwrap_or
```

**Before**: `.map_or(default, |x| x)`
**After**: `.unwrap_or(default)`

---

### Task 2: Collapse Nested `if` Statements (7 errors)

**Auto-fix**:
```bash
cargo clippy --fix -- -A clippy::all -W clippy::collapsible_if
```

**Before**:
```rust
if condition1 {
    if condition2 {
        // code
    }
}
```

**After**:
```rust
if condition1 && condition2 {
    // code
}
```

---

### Task 3: Use `.first()` Instead of `.get(0)` (15 errors)

**Auto-fix**:
```bash
cargo clippy --fix -- -A clippy::all -W clippy::get_first
```

**Before**: `arr.get(0)`
**After**: `arr.first()`

---

## 📊 Impact

**Clippy**: 33 errors eliminated (114 → 81)
**Readability**: Improved (more idiomatic code)
**Risk**: LOW (automated fixes)

---

**Approach**: Run `cargo clippy --fix`, test, commit incrementally
**Status**: READY
