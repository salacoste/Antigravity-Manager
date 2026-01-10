# Story Epic-003-05: Implement JWT Signature Validation

**Story ID**: Epic-003-05
**Epic**: [Epic-003 - Claude 4.5 Sonnet Thinking Compliance](../epics/Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md)
**Priority**: P0 (Critical) - 🔐 **LAST P0 STORY - Completes Critical Phase**
**Estimate**: 3 story points (2 hours)
**Status**: To Do
**Assignee**: TBD

---

## User Story

**As a** API Proxy developer
**I want** to implement JWT format validation for thinking block signatures
**So that** only valid server-generated JWT signatures are accepted, preventing forged or invalid signatures

---

## Context

Current signature validation only checks **length** (≥10 characters), which is **too permissive** and could accept arbitrary strings. According to reverse engineering, thinking block signatures are **JWT tokens** (JSON Web Tokens) with format `header.payload.signature`.

**Security Risk**: MEDIUM - Current implementation could accept forged signatures.

**Current Validation** (Too Permissive):
```rust
const MIN_SIGNATURE_LENGTH: usize = 10;

fn has_valid_signature(block: &ContentBlock) -> bool {
    match block {
        ContentBlock::Thinking { signature, .. } => {
            signature.as_ref().map_or(false, |s| s.len() >= MIN_SIGNATURE_LENGTH)
        }
        _ => true
    }
}
```

**Issues**:
- ✅ Checks signature exists
- ✅ Checks minimum length (10 chars)
- ❌ No JWT structure validation
- ❌ No signature authenticity check
- ❌ Could accept arbitrary strings like "1234567890"

**Expected Validation** (JWT Format):
```rust
const MIN_SIGNATURE_LENGTH: usize = 100;  // JWT tokens ~200+ chars
const JWT_PARTS: usize = 3;  // header.payload.signature

fn is_valid_jwt_format(signature: &str) -> bool {
    let parts: Vec<&str> = signature.split('.').collect();

    if parts.len() != JWT_PARTS {
        return false;
    }

    // Each part should be base64-encoded
    for part in parts {
        if part.is_empty() || !is_base64_compatible(part) {
            return false;
        }
    }

    true
}
```

**Example Valid JWT**:
```
eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnb29nbGUuY29tIiwic3ViIjoidGhpbmtpbmciLCJleHAiOjE3MDUwODAwMDB9.MEUCIQDx...
```

---

## Reference Documents

**Primary**:
1. `docs/comparison/claude/claude-4-5-sonnet/current-implementation-thinking.md`
   - Lines 2999-3125: Gap Analysis #3 (Signature Validation)
   - Lines 3066-3125: JWT validation implementation example

2. `docs/technical-specs/antigravity-workflow-compliance-gap-analysis.md`
   - Thinking Signature Validation section

**Current Implementation**:
- `src-tauri/src/proxy/handlers/claude.rs` (lines 40-53)

---

## Technical Details

### Implementation Steps

**File**: `src-tauri/src/proxy/handlers/claude.rs`

**Step 1 - Update Constants**:
```rust
// Increase minimum length for JWT tokens
const MIN_SIGNATURE_LENGTH: usize = 100;  // JWT tokens are ~200+ chars
const JWT_PARTS: usize = 3;  // header.payload.signature
```

**Step 2 - Add JWT Validation Helper**:
```rust
/// Validate JWT format (header.payload.signature)
/// Each part should be base64url-encoded (alphanumeric + - _)
/// Reference: docs/comparison/claude/claude-4-5-sonnet/current-implementation-thinking.md:3073-3092
fn is_valid_jwt_format(signature: &str) -> bool {
    // Split into parts
    let parts: Vec<&str> = signature.split('.').collect();

    // Must have exactly 3 parts
    if parts.len() != JWT_PARTS {
        return false;
    }

    // Each part must be non-empty and base64url-compatible
    for part in parts {
        if part.is_empty() {
            return false;
        }

        // Base64url uses alphanumeric + - _ characters
        if !part.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return false;
        }
    }

    true
}
```

**Step 3 - Update has_valid_signature Function**:
```rust
/// Validate thinking block signature
/// Checks both length and JWT format structure
fn has_valid_signature(block: &ContentBlock) -> bool {
    match block {
        ContentBlock::Thinking { signature, thinking, .. } => {
            // Empty thinking + any valid signature = valid (trailing signature case)
            if thinking.is_empty() && signature.is_some() {
                return signature.as_ref().map_or(false, |s| is_valid_jwt_format(s));
            }

            // Has content + valid JWT signature
            signature.as_ref().map_or(false, |s| {
                s.len() >= MIN_SIGNATURE_LENGTH && is_valid_jwt_format(s)
            })
        }
        _ => true
    }
}
```

**Step 4 - Add Warning Logging** (optional but recommended):
```rust
// In filter_invalid_thinking_blocks or similar function
if let Some(sig) = signature {
    if !is_valid_jwt_format(sig) {
        tracing::warn!(
            "[Signature-Validation] Invalid JWT format: length={}, parts={}",
            sig.len(),
            sig.split('.').count()
        );
    }
}
```

---

## Acceptance Criteria

**Given** a thinking block with a signature
**When** the signature is validated
**Then** it must have exactly 3 parts separated by dots (header.payload.signature)
**And** each part must be non-empty
**And** each part must contain only base64url-compatible characters (alphanumeric + - _)
**And** the total length must be ≥100 characters

**Given** a thinking block with empty thinking and valid signature
**When** the signature is validated
**Then** it is accepted (trailing signature case)

**Given** a thinking block with invalid signature format
**When** the signature is validated
**Then** it is rejected and logged as warning

---

## Testing Requirements

### Unit Tests

```rust
// JWT Format Validation Tests
#[test]
fn test_valid_jwt_format() {
    let valid_jwt = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnb29nbGUuY29tIiwic3ViIjoidGhpbmtpbmciLCJleHAiOjE3MDUwODAwMDB9.MEUCIQDxyz123";
    assert!(is_valid_jwt_format(valid_jwt));
}

#[test]
fn test_invalid_jwt_too_few_parts() {
    let invalid = "header.payload";  // Only 2 parts
    assert!(!is_valid_jwt_format(invalid));
}

#[test]
fn test_invalid_jwt_too_many_parts() {
    let invalid = "header.payload.signature.extra";  // 4 parts
    assert!(!is_valid_jwt_format(invalid));
}

#[test]
fn test_invalid_jwt_empty_part() {
    let invalid = "header..signature";  // Empty payload
    assert!(!is_valid_jwt_format(invalid));
}

#[test]
fn test_invalid_jwt_invalid_characters() {
    let invalid = "header@.payload!.signature#";  // Special chars
    assert!(!is_valid_jwt_format(invalid));
}

// Signature Validation Tests
#[test]
fn test_signature_too_short() {
    let short_jwt = "abc.def.ghi";  // Only 11 chars
    let block = ContentBlock::Thinking {
        thinking: "Some thought".to_string(),
        signature: Some(short_jwt.to_string()),
    };
    assert!(!has_valid_signature(&block));
}

#[test]
fn test_signature_valid_jwt() {
    let valid_jwt = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnb29nbGUuY29tIiwic3ViIjoidGhpbmtpbmciLCJleHAiOjE3MDUwODAwMDB9.MEUCIQDxyz123";
    let block = ContentBlock::Thinking {
        thinking: "Some thought".to_string(),
        signature: Some(valid_jwt.to_string()),
    };
    assert!(has_valid_signature(&block));
}

#[test]
fn test_empty_thinking_with_valid_signature() {
    let valid_jwt = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnb29nbGUuY29tIiwic3ViIjoidGhpbmtpbmciLCJleHAiOjE3MDUwODAwMDB9.MEUCIQDxyz123";
    let block = ContentBlock::Thinking {
        thinking: "".to_string(),  // Empty thinking
        signature: Some(valid_jwt.to_string()),
    };
    assert!(has_valid_signature(&block));  // Should still be valid
}

#[test]
fn test_arbitrary_string_rejected() {
    let block = ContentBlock::Thinking {
        thinking: "Some thought".to_string(),
        signature: Some("1234567890".to_string()),  // Just a string
    };
    assert!(!has_valid_signature(&block));
}
```

### Integration Tests

```rust
#[test]
fn test_filter_blocks_with_invalid_signatures() {
    let mut messages = vec![
        ClaudeMessage {
            role: "user",
            content: vec![
                ContentBlock::Thinking {
                    thinking: "Thought".to_string(),
                    signature: Some("invalid-sig".to_string()),
                }
            ]
        }
    ];

    filter_invalid_thinking_blocks(&mut messages);

    // Invalid signature block should be filtered out
    assert_eq!(messages[0].content.len(), 0);
}

#[test]
fn test_valid_jwt_signature_preserved() {
    let valid_jwt = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnb29nbGUuY29tIiwic3ViIjoidGhpbmtpbmciLCJleHAiOjE3MDUwODAwMDB9.MEUCIQDxyz123";
    let mut messages = vec![
        ClaudeMessage {
            role: "user",
            content: vec![
                ContentBlock::Thinking {
                    thinking: "Valid thought".to_string(),
                    signature: Some(valid_jwt.to_string()),
                }
            ]
        }
    ];

    filter_invalid_thinking_blocks(&mut messages);

    // Valid JWT signature should be preserved
    assert_eq!(messages[0].content.len(), 1);
}
```

### Manual Validation

- [ ] Test with real thinking blocks from v1internal API
- [ ] Verify real JWT signatures are accepted
- [ ] Verify short/invalid signatures are rejected
- [ ] Check logs for warning messages on invalid signatures
- [ ] Confirm no false positives (valid signatures rejected)

---

## Definition of Done

**Code**:
- [ ] MIN_SIGNATURE_LENGTH increased from 10 to 100
- [ ] JWT_PARTS constant added (value: 3)
- [ ] `is_valid_jwt_format()` helper function implemented
- [ ] `has_valid_signature()` updated to use JWT validation
- [ ] Warning logging added for invalid signatures (optional)
- [ ] Code reviewed and approved

**Testing**:
- [ ] 10 unit tests written and passing (JWT format + signature validation)
- [ ] 2 integration tests passing (filter invalid + preserve valid)
- [ ] Test coverage ≥80% for new code
- [ ] Manual validation with real API responses
- [ ] No false positives in testing

**Compliance**:
- [ ] **Compliance validation**: JWT validation matches RE spec (3 parts, base64url)
- [ ] **Performance benchmarking**: JWT validation adds <10ms overhead
- [ ] **Anti-detection validation**: Invalid signatures filtered out
- [ ] **Request format validation**: Only server-generated JWTs accepted
- [ ] **Error handling validation**: Invalid formats logged and rejected gracefully

**Documentation**:
- [ ] Code comments reference RE spec line numbers (2999-3125)
- [ ] Implementation notes explain JWT structure (header.payload.signature)
- [ ] Warning about false positives if min length too restrictive
- [ ] Base64url character set documented (alphanumeric + - _)

---

## Dependencies

**Depends On**:
- Epic-003-01: Model ID (request structure)
- Epic-003-02: Providers (complete request format)
- Epic-003-03: Metadata (anti-detection compliance)

**Completes**:
- 🎯 **FR3**: JWT Signature Validation
- 🎉 **P0 PHASE**: All critical compliance stories (Stories #1-5)

**Enables**:
- ✅ Critical compliance phase COMPLETE
- ⏭️ Ready for P1 phase (validation warnings and logging)

---

## Implementation Notes

**Why JWT Format?**
- Thinking block signatures are server-generated JWT tokens
- Structure: `header.payload.signature` (3 parts separated by dots)
- Each part is base64url-encoded
- Total length ~200+ characters

**JWT Components**:
```
eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9  ← Header (algorithm, type)
.
eyJpc3MiOiJnb29nbGUuY29tIiwic3ViIjoidGhpbmtpbmciLCJleHAiOjE3MDUwODAwMDB9  ← Payload (claims)
.
MEUCIQDx...  ← Signature (cryptographic signature)
```

**Base64url Character Set**:
- Alphanumeric: `a-z`, `A-Z`, `0-9`
- URL-safe: `-` (minus), `_` (underscore)
- No padding `=` typically

**Security Improvement**:
- **Before**: Any string ≥10 chars accepted (e.g., "1234567890")
- **After**: Only valid JWT structure accepted
- **Risk Reduction**: Prevents forged or malformed signatures

**Backward Compatibility**:
- Real server-generated signatures are valid JWTs → no breaking change
- Only rejects invalid/forged signatures → correct behavior
- Empty thinking + signature still works → trailing signature case preserved

**Performance**:
- String split and character validation
- O(n) where n = signature length (~200 chars)
- Negligible overhead (<10ms per signature)

---

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| False positives (valid signatures rejected) | Low | High | Extensive testing with real API responses |
| Performance degradation | Very Low | Low | Simple string operations, <10ms overhead |
| Breaking existing functionality | Low | Medium | Integration tests, manual validation |

**Mitigation Plan**:
1. Test with multiple real JWT signatures from v1internal API
2. Monitor logs for rejected signatures during testing
3. Validate no performance degradation in benchmarks
4. Rollback capability if false positives detected

---

## Edge Cases

**Case 1: Empty Thinking + Signature** (Trailing Signature)
```rust
{
  "thinking": "",
  "signature": "valid.jwt.token"
}
```
**Behavior**: Should be accepted (trailing signature case)

**Case 2: Very Short JWT-like String**
```rust
{
  "thinking": "Some thought",
  "signature": "a.b.c"  // 5 chars, valid structure but too short
}
```
**Behavior**: Rejected (length < 100)

**Case 3: Long String Without Dots**
```rust
{
  "thinking": "Some thought",
  "signature": "aaaaaaaa..." // 200 chars but no dots
}
```
**Behavior**: Rejected (not 3 parts)

**Case 4: Valid Structure, Invalid Characters**
```rust
{
  "thinking": "Some thought",
  "signature": "header@invalid.payload!.signature#"
}
```
**Behavior**: Rejected (invalid base64url characters)

---

## Success Criteria (P0 Phase Completion)

After this story is complete:

**All P0 Stories Done** (8 hours total):
- ✅ Story #1: Model ID (1.5h)
- ✅ Story #2: API/Model Providers (1.5h)
- ✅ Story #3: ideType ANTIGRAVITY (2h)
- ✅ Story #4: Extended Metadata (1h)
- ✅ Story #5: JWT Validation (2h) ← **THIS STORY**

**Compliance Achievement**:
- Current: 75-80%
- After P0: **90%** (target achieved)

**Next Phase**: P1 (Validation & Logging - 3.5 hours)

---

## Related Stories

**Previous** (P0):
- Epic-003-04: Extended Session Metadata

**Next** (P1 Phase):
- Epic-003-06: Budget Constraint Warnings (0.5h)
- Epic-003-07: Position Enforcement Logging (1h)

---

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2026-01-10 | Story created | BMad Master |
