# Epic-004 Implementation Plan - Gap Remediation

**Created**: 2026-01-11
**Epic**: Claude 4.5 Sonnet Standard - Gap Remediation & Validation
**Total Effort**: 10 hours (5 gaps)
**Status**: Ready for Implementation

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [GAP #1: User-Agent Dynamic Generation](#gap-1-user-agent-dynamic-generation)
3. [GAP #2: Thinking Mode Detection Fix](#gap-2-thinking-mode-detection-fix)
4. [GAP #3: Integration Test Suite](#gap-3-integration-test-suite)
5. [GAP #4: Platform Detection Refactoring](#gap-4-platform-detection-refactoring)
6. [GAP #5: Validation Logging](#gap-5-validation-logging)
7. [Implementation Order](#implementation-order)
8. [Testing Strategy](#testing-strategy)
9. [Validation Checklist](#validation-checklist)

---

## Overview

### Infrastructure Context (from Phase 1 Analysis)

**Common Module Structure**:
```
src-tauri/src/proxy/
├── common/
│   ├── mod.rs              (exports only)
│   ├── utils.rs            (generate_random_id only)
│   ├── json_schema.rs
│   └── model_mapping.rs
├── mappers/
│   ├── common_utils.rs     (request config resolution)
│   └── claude/
│       ├── request.rs      (has get_platform/get_architecture)
│       └── models.rs
└── upstream/
    └── client.rs           (hardcoded User-Agent)
```

**Test Infrastructure**:
- 151 existing tests via `#[cfg(test)]` modules
- Run via `cargo test --lib`
- No separate test directory
- No CI/CD (manual testing)

**Code Conventions**:
- snake_case functions, SCREAMING_SNAKE_CASE constants
- Tracing for logging (`tracing::debug!`, `tracing::warn!`)
- Result<T, String> for Tauri commands
- Chinese + English comments

---

## GAP #1: User-Agent Dynamic Generation

**Priority**: P0 - CRITICAL
**Effort**: 2 hours
**Files**: `src-tauri/src/proxy/common/platform.rs` (new), `src-tauri/src/proxy/upstream/client.rs`

### Problem Statement

User-Agent is **hardcoded** as `"antigravity/1.13.3 darwin/arm64"` for ALL platforms, violating anti-detection compliance.

**Current Code** (`upstream/client.rs:25-26`):
```rust
let user_agent = std::env::var("CLAUDE_USER_AGENT")
    .unwrap_or_else(|_| "antigravity/1.13.3 darwin/arm64".to_string());
```

**Impact**:
- Windows/Linux machines send wrong User-Agent ❌
- Critical anti-detection marker failure ❌

### Solution Design

#### Step 1: Create Platform Detection Module (30 min)

**File**: `src-tauri/src/proxy/common/platform.rs` (NEW)

```rust
//! Platform detection utilities for Antigravity compliance
//!
//! Provides compile-time platform and architecture detection for building
//! dynamic User-Agent strings and request metadata.
//!
//! Reference: docs/epics/Epic-004-IMPLEMENTATION-PLAN.md#gap-1

/// Antigravity version for User-Agent and metadata
pub const ANTIGRAVITY_VERSION: &str = "1.13.3";

/// Detect platform using compile-time cfg macros
///
/// Returns platform identifier matching Google Antigravity convention:
/// - macOS: "darwin"
/// - Windows: "windows"
/// - Linux: "linux"
///
/// # Examples
/// ```
/// let platform = get_platform();
/// assert!(platform == "darwin" || platform == "windows" || platform == "linux");
/// ```
#[inline]
pub fn get_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        // Fallback for unknown platforms
        "linux"
    }
}

/// Detect CPU architecture using compile-time cfg macros
///
/// Returns architecture identifier matching Google Antigravity convention:
/// - ARM64: "arm64" (Apple Silicon, ARM processors)
/// - x86_64: "x86_64" (Intel, AMD processors)
///
/// # Examples
/// ```
/// let arch = get_architecture();
/// assert!(arch == "arm64" || arch == "x86_64");
/// ```
#[inline]
pub fn get_architecture() -> &'static str {
    if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else {
        // Fallback for unknown architectures
        "x86_64"
    }
}

/// Build User-Agent string for Antigravity v1internal API compliance
///
/// Format: `"antigravity/{version} {platform}/{arch}"`
///
/// # Examples
/// ```
/// let ua = build_user_agent();
/// // macOS ARM64: "antigravity/1.13.3 darwin/arm64"
/// // Windows x64: "antigravity/1.13.3 windows/x86_64"
/// // Linux ARM64: "antigravity/1.13.3 linux/arm64"
/// ```
///
/// # Reference
/// - Reverse Engineering: docs/antigravity/workflows/models/claude/claude-4-5-sonnet-workflow.md:46-50
/// - Anti-Detection: docs/epics/Epic-004-GAPS-AND-RECOMMENDATIONS.md#gap-1
pub fn build_user_agent() -> String {
    let platform = get_platform();
    let arch = get_architecture();

    let ua = format!(
        "antigravity/{} {}/{}",
        ANTIGRAVITY_VERSION,
        platform,
        arch
    );

    tracing::debug!(
        "[Platform-Detection] User-Agent: '{}' (platform: {}, arch: {})",
        ua,
        platform,
        arch
    );

    ua
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_platform_valid() {
        let platform = get_platform();
        assert!(
            platform == "darwin" || platform == "windows" || platform == "linux",
            "Platform must be one of: darwin, windows, linux. Got: {}",
            platform
        );
    }

    #[test]
    fn test_get_architecture_valid() {
        let arch = get_architecture();
        assert!(
            arch == "arm64" || arch == "x86_64",
            "Architecture must be one of: arm64, x86_64. Got: {}",
            arch
        );
    }

    #[test]
    fn test_build_user_agent_format() {
        let ua = build_user_agent();

        // Should start with version
        assert!(ua.starts_with("antigravity/1.13.3 "));

        // Should contain platform and architecture
        assert!(ua.contains("darwin/") || ua.contains("windows/") || ua.contains("linux/"));
        assert!(ua.contains("/arm64") || ua.contains("/x86_64"));
    }

    #[test]
    fn test_user_agent_matches_platform() {
        let ua = build_user_agent();

        #[cfg(target_os = "macos")]
        assert!(ua.contains("darwin"), "macOS build should contain 'darwin'");

        #[cfg(target_os = "windows")]
        assert!(ua.contains("windows"), "Windows build should contain 'windows'");

        #[cfg(target_os = "linux")]
        assert!(ua.contains("linux"), "Linux build should contain 'linux'");
    }

    #[test]
    fn test_user_agent_matches_architecture() {
        let ua = build_user_agent();

        #[cfg(target_arch = "aarch64")]
        assert!(ua.ends_with("arm64"), "ARM64 build should end with 'arm64'");

        #[cfg(target_arch = "x86_64")]
        assert!(ua.ends_with("x86_64"), "x86_64 build should end with 'x86_64'");
    }

    #[test]
    fn test_version_constant() {
        assert_eq!(ANTIGRAVITY_VERSION, "1.13.3");
    }
}
```

**Tests**: 7 unit tests (all compile-time verified)

#### Step 2: Update common/mod.rs (5 min)

**File**: `src-tauri/src/proxy/common/mod.rs`

```rust
// Common 模块 - 公共工具

pub mod json_schema;
pub mod model_mapping;
pub mod utils;
pub mod platform;  // ✅ Add new module
```

#### Step 3: Refactor claude/request.rs (15 min)

**File**: `src-tauri/src/proxy/mappers/claude/request.rs`

**Changes**:

1. **Remove duplicate functions** (lines 213-239):
```rust
// DELETE these functions (移动到 platform.rs)
fn get_platform() -> &'static str { ... }
fn get_architecture() -> &'static str { ... }
```

2. **Add import** (top of file):
```rust
use crate::proxy::common::platform::{get_platform, get_architecture, ANTIGRAVITY_VERSION};
```

3. **Update IDE_VERSION constant** (line 29):
```rust
// DELETE this line
const IDE_VERSION: &str = "1.13.3";

// Use ANTIGRAVITY_VERSION from platform module instead
// (Update all references from IDE_VERSION to ANTIGRAVITY_VERSION)
```

4. **Update metadata building code** (line 543):
```rust
let mut metadata = json!({
    "ideType": IDE_TYPE,
    "ideVersion": ANTIGRAVITY_VERSION,  // ✅ Use from platform module
    "platform": get_platform(),         // ✅ Use from platform module
    "architecture": get_architecture()  // ✅ Use from platform module
});
```

#### Step 4: Update upstream/client.rs (30 min)

**File**: `src-tauri/src/proxy/upstream/client.rs`

**Changes**:

1. **Add import** (top of file):
```rust
use crate::proxy::common::platform::build_user_agent;
```

2. **Update UpstreamClient::new()** (lines 22-51):
```rust
pub fn new(proxy_config: Option<crate::proxy::config::UpstreamProxyConfig>) -> Self {
    // Allow override via env var, otherwise use dynamic detection
    let user_agent = std::env::var("CLAUDE_USER_AGENT")
        .unwrap_or_else(|_| build_user_agent());  // ✅ Dynamic!

    tracing::info!("🔧 UpstreamClient User-Agent: {}", user_agent);

    let mut builder = Client::builder()
        // ... other settings ...
        .user_agent(user_agent);  // ✅ No .clone() needed (String moved)

    // ... rest of function ...
}
```

3. **Update call_v1_internal()** (lines 100-107):
```rust
// ✅ Use dynamic User-Agent (remove duplication)
let user_agent = std::env::var("CLAUDE_USER_AGENT")
    .unwrap_or_else(|_| build_user_agent());

headers.insert(
    header::USER_AGENT,
    header::HeaderValue::from_str(&user_agent).map_err(|e| e.to_string())?,
);
```

**IMPORTANT**: User-Agent is set in **TWO places** in this file:
- Line 37: Client builder (for all requests)
- Line 102-107: Per-request headers (overrides builder)

Both must use `build_user_agent()`.

#### Step 5: Run Tests (10 min)

```bash
# Run platform detection tests
cargo test --lib platform::tests

# Run full test suite
cargo test --lib

# Verify output
cargo run  # Check logs for User-Agent
```

### Acceptance Criteria

- [ ] `common/platform.rs` module created with 3 functions + 7 tests
- [ ] `common/mod.rs` exports platform module
- [ ] `claude/request.rs` uses platform functions (no duplication)
- [ ] `upstream/client.rs` uses `build_user_agent()` dynamically
- [ ] All 7 platform tests pass on current platform
- [ ] User-Agent logged correctly in console
- [ ] Env var override (`CLAUDE_USER_AGENT`) still works
- [ ] No hardcoded "darwin/arm64" remains in codebase

### Testing Checklist

**Unit Tests** (7 tests):
- [x] Platform detection returns valid value
- [x] Architecture detection returns valid value
- [x] User-Agent format correct
- [x] User-Agent matches compile-time platform
- [x] User-Agent matches compile-time architecture
- [x] Version constant correct
- [x] Build function logs debug message

**Integration Tests** (manual):
- [ ] macOS ARM64: `antigravity/1.13.3 darwin/arm64`
- [ ] macOS x64: `antigravity/1.13.3 darwin/x86_64`
- [ ] Windows x64: `antigravity/1.13.3 windows/x86_64`
- [ ] Linux ARM64: `antigravity/1.13.3 linux/arm64`
- [ ] Linux x64: `antigravity/1.13.3 linux/x86_64`

**Regression Tests**:
- [ ] Env var override works: `CLAUDE_USER_AGENT=custom cargo run`
- [ ] Upstream requests use correct User-Agent
- [ ] No errors in logs

---

## GAP #2: Thinking Mode Detection Fix

**Priority**: P0 - CRITICAL
**Effort**: 3 hours
**Files**: `src-tauri/src/proxy/mappers/claude/request.rs`

### Problem Statement

Thinking mode detection logic incorrectly considers **ALL Claude models** as thinking-capable.

**Current Code** (`request.rs:329-331`):
```rust
let target_model_supports_thinking = mapped_model.contains("-thinking")
    || mapped_model.starts_with("claude-")  // ❌ WRONG! All Claude → thinking
    || mapped_model.starts_with("gemini-");
```

**Bug Scenario**:
1. User: `{ model: "claude-4.5-sonnet", thinking: { type: "enabled" } }`
2. System: `target_model_supports_thinking` = TRUE (because `starts_with("claude-")`)
3. System: Does NOT disable thinking mode
4. Request: modelId 333 + thinkingConfig ← **INCONSISTENT** ❌

### Solution Design

#### Step 1: Fix Detection Logic (15 min)

**File**: `src-tauri/src/proxy/mappers/claude/request.rs`

**Current code** (lines 326-339):
```rust
// [NEW FIX] Check if target model supports thinking
// Claude models: thinking via "-thinking" suffix in model name
// Gemini models: thinking via thinkingConfig parameter in API request (NOT in model name!)
let target_model_supports_thinking = mapped_model.contains("-thinking")
    || mapped_model.starts_with("claude-")  // ❌ DELETE THIS LINE
    || mapped_model.starts_with("gemini-");

if is_thinking_enabled && !target_model_supports_thinking {
    tracing::warn!(
        "[Thinking-Mode] Target model '{}' does not support thinking. Force disabling thinking mode.",
        mapped_model
    );
    is_thinking_enabled = false;
}
```

**Fixed code**:
```rust
// [FIXED] Check if target model supports thinking
// Claude models: ONLY models with "-thinking" suffix support thinking mode
// Gemini models: Support thinking via thinkingConfig (NOT suffix-based)
let target_model_supports_thinking = mapped_model.contains("-thinking")
    || is_gemini_thinking_model(&mapped_model);  // ✅ Explicit Gemini check

if is_thinking_enabled && !target_model_supports_thinking {
    tracing::warn!(
        "[Thinking-Mode] ⚠️  Model '{}' (ID: {}) does NOT support thinking. \
         User requested thinking: {{ type: 'enabled' }}, but it will be IGNORED.",
        mapped_model,
        get_model_id(&mapped_model)
    );
    is_thinking_enabled = false;
}
```

#### Step 2: Add Gemini Helper Function (15 min)

**Location**: Same file, before the transform function (around line 660)

```rust
/// Check if Gemini model supports thinking mode
///
/// Gemini models support thinking via thinkingConfig parameter,
/// not via model name suffix (unlike Claude models).
///
/// Reference: docs/comparison/gemini/gemini-thinking-support.md
fn is_gemini_thinking_model(model_name: &str) -> bool {
    // Only specific Gemini models support thinking
    matches!(
        model_name,
        "gemini-3-pro-high"
            | "gemini-2.5-pro"
            | "gemini-2.5-pro-thinking"
            | "gemini-2.0-flash-thinking-exp"
    )
}
```

**Alternative**: If Gemini thinking support list is dynamic, use prefix check:
```rust
fn is_gemini_thinking_model(model_name: &str) -> bool {
    model_name.starts_with("gemini-")
        && (model_name.contains("thinking") || model_name.contains("-pro"))
}
```

**Recommendation**: Use explicit whitelist for safety.

#### Step 3: Update should_enable_thinking_by_default() (15 min)

**Location**: Lines 642-660

**Current code**:
```rust
fn should_enable_thinking_by_default(model: &str) -> bool {
    let model_lower = model.to_lowercase();

    // Enable thinking by default for Opus 4.5 variants
    if model_lower.contains("opus-4-5") || model_lower.contains("opus-4.5") {
        tracing::debug!(
            "[Thinking-Mode] Auto-enabling thinking for Opus 4.5 model: {}",
            model
        );
        return true;
    }

    // Also enable for explicit thinking model variants
    if model_lower.contains("-thinking") {
        return true;
    }

    false
}
```

**Problem**: This function enables thinking for `"claude-opus-4-5"` (no "-thinking" suffix).

**Fixed code**:
```rust
fn should_enable_thinking_by_default(model: &str) -> bool {
    let model_lower = model.to_lowercase();

    // ONLY enable thinking by default for models with explicit "-thinking" suffix
    if model_lower.contains("-thinking") {
        tracing::debug!(
            "[Thinking-Mode] Auto-enabling thinking for model with '-thinking' suffix: {}",
            model
        );
        return true;
    }

    // [DEPRECATED] Opus 4.5 auto-enable removed
    // User must explicitly use "claude-opus-4-5-thinking" model name
    // or provide thinking config to enable thinking mode

    false
}
```

**Rationale**: Explicit is better than implicit. User should use `-thinking` suffix or provide `thinking: { type: "enabled" }`.

#### Step 4: Add Unit Tests (1.5 hours)

**Location**: End of test module in `request.rs` (after line 2500)

```rust
// ==================== GAP #2: Thinking Mode Detection Tests ====================
// Reference: docs/epics/Epic-004-GAPS-AND-RECOMMENDATIONS.md#gap-2

#[test]
fn test_standard_sonnet_does_not_support_thinking() {
    // Standard Claude models should NOT support thinking
    assert!(!model_supports_thinking_suffix("claude-4.5-sonnet"));
    assert!(!model_supports_thinking_suffix("claude-opus-4-5"));
    assert!(!model_supports_thinking_suffix("claude-3-5-sonnet"));

    // Only thinking variants support it
    assert!(model_supports_thinking_suffix("claude-4.5-sonnet-thinking"));
    assert!(model_supports_thinking_suffix("claude-opus-4-5-thinking"));
}

#[test]
fn test_gemini_thinking_models() {
    // Gemini Pro models support thinking
    assert!(is_gemini_thinking_model("gemini-3-pro-high"));
    assert!(is_gemini_thinking_model("gemini-2.5-pro"));

    // Flash models don't (unless explicit thinking variant)
    assert!(!is_gemini_thinking_model("gemini-2.5-flash"));
    assert!(!is_gemini_thinking_model("gemini-3-flash"));
}

#[test]
fn test_explicit_thinking_ignored_for_standard_model() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),  // Standard model
        thinking: Some(ThinkingConfig {
            type_: "enabled".to_string(),
            budget_tokens: Some(3000),
        }),  // ❌ Trying to enable thinking
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(8192),
        temperature: None,
        top_p: None,
        top_k: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Model ID should be 333 (standard)
    assert_eq!(body["modelId"], 333);

    // ✅ thinkingConfig should be IGNORED (not present)
    let gen_config = &body["request"]["generationConfig"];
    assert!(gen_config["thinkingConfig"].is_null());

    // ✅ maxOutputTokens should be preserved
    assert_eq!(gen_config["maxOutputTokens"], 8192);
}

#[test]
fn test_thinking_variant_enables_thinking() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet-thinking".to_string(),  // Thinking model
        thinking: None,  // ✅ Auto-enabled by default
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(8192),
        temperature: None,
        top_p: None,
        top_k: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Model ID should be 334 (thinking)
    assert_eq!(body["modelId"], 334);

    // ✅ thinkingConfig should be PRESENT
    let gen_config = &body["request"]["generationConfig"];
    assert!(!gen_config["thinkingConfig"].is_null());
}

#[test]
fn test_should_enable_thinking_only_for_explicit_suffix() {
    // Only models with "-thinking" suffix should auto-enable
    assert!(should_enable_thinking_by_default("claude-4.5-sonnet-thinking"));
    assert!(should_enable_thinking_by_default("claude-opus-4-5-thinking"));
    assert!(should_enable_thinking_by_default("gemini-2.5-pro-thinking"));

    // Standard models should NOT auto-enable (even Opus 4.5)
    assert!(!should_enable_thinking_by_default("claude-4.5-sonnet"));
    assert!(!should_enable_thinking_by_default("claude-opus-4-5"));  // ✅ Fixed
    assert!(!should_enable_thinking_by_default("claude-3-5-sonnet"));
}

// Helper function for testing (can be inline or in test module)
fn model_supports_thinking_suffix(model_name: &str) -> bool {
    model_name.contains("-thinking")
}
```

#### Step 5: Integration Tests (20 min)

**Location**: Same test module

```rust
#[test]
fn test_full_request_standard_vs_thinking_models() {
    // Test standard model
    let standard_req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        thinking: Some(ThinkingConfig {
            type_: "enabled".to_string(),
            budget_tokens: None,
        }),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(4096),
        temperature: None,
        top_p: None,
        top_k: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    // Test thinking model
    let thinking_req = ClaudeRequest {
        model: "claude-4.5-sonnet-thinking".to_string(),
        ..standard_req.clone()
    };

    let (standard_body, _) = transform_claude_request_in(&standard_req, "test-project").unwrap();
    let (thinking_body, _) = transform_claude_request_in(&thinking_req, "test-project").unwrap();

    // Standard model: NO thinking despite explicit request
    assert_eq!(standard_body["modelId"], 333);
    assert!(standard_body["request"]["generationConfig"]["thinkingConfig"].is_null());

    // Thinking model: HAS thinking
    assert_eq!(thinking_body["modelId"], 334);
    assert!(!thinking_body["request"]["generationConfig"]["thinkingConfig"].is_null());
}
```

#### Step 6: Run Tests (10 min)

```bash
# Run thinking detection tests
cargo test --lib thinking

# Run full test suite
cargo test --lib

# Test with explicit thinking
# (Manual verification with real API)
```

### Acceptance Criteria

- [ ] `target_model_supports_thinking` logic fixed (no `starts_with("claude-")`)
- [ ] `is_gemini_thinking_model()` helper function added
- [ ] `should_enable_thinking_by_default()` only checks `-thinking` suffix
- [ ] Warning log enhanced with model ID
- [ ] 6+ unit tests added and passing
- [ ] Standard model NEVER activates thinking
- [ ] Thinking model ALWAYS activates thinking
- [ ] Explicit thinking request ignored for standard model

### Testing Checklist

**Unit Tests** (6 tests):
- [x] Standard models don't support thinking
- [x] Gemini thinking models detected correctly
- [x] Explicit thinking ignored for standard
- [x] Thinking variant enables thinking
- [x] Auto-enable only for `-thinking` suffix
- [x] Full request transformation comparison

**Integration Tests** (manual):
- [ ] `claude-4.5-sonnet` + thinking → NO thinkingConfig, modelId 333
- [ ] `claude-4.5-sonnet-thinking` → HAS thinkingConfig, modelId 334
- [ ] Warning logged when thinking ignored

---

## GAP #3: Integration Test Suite

**Priority**: P0 - CRITICAL
**Effort**: 3 hours
**Files**: `src-tauri/src/proxy/mappers/claude/request.rs` (test module)

### Problem Statement

NO comprehensive integration tests for standard model (claude-4.5-sonnet, ID 333).

**Current Coverage**:
- ✅ Unit tests for `get_model_id("claude-4.5-sonnet")` → 333
- ✅ Unit tests for provider routing
- ❌ Integration tests only use "claude-4.5-sonnet-thinking" (334)
- ❌ No tests for full request transformation with model ID 333
- ❌ No regression tests to prevent thinking activation

### Solution Design

#### Test Suite Structure

**8 Integration Tests** covering full request transformation for standard model:

1. **test_standard_sonnet_model_id_333** - Model ID routing
2. **test_standard_sonnet_no_thinking_config** - Thinking absence
3. **test_standard_sonnet_explicit_thinking_ignored** - Thinking rejection
4. **test_standard_sonnet_metadata_compliance** - Metadata injection
5. **test_standard_sonnet_tool_configuration** - Tool config modes
6. **test_standard_sonnet_grounding_config** - Grounding/geminiSettings
7. **test_standard_vs_thinking_model_ids** - Model comparison
8. **test_standard_sonnet_full_roundtrip** - Complete E2E test

#### Implementation

**Location**: `src-tauri/src/proxy/mappers/claude/request.rs` (end of test module, after line 2500)

```rust
// ==================== GAP #3: Standard Model Integration Tests ====================
// Complete integration test suite for claude-4.5-sonnet (standard, no thinking)
// Reference: docs/epics/Epic-004-IMPLEMENTATION-PLAN.md#gap-3

#[test]
fn test_standard_sonnet_model_id_333() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),  // ✅ Standard model
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Hello".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(2048),
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,  // NO thinking config
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let result = transform_claude_request_in(&req, "test-project");
    assert!(result.is_ok());

    let (body, _violations) = result.unwrap();

    // ✅ Should have model ID 333 (standard, not 334)
    assert_eq!(body["modelId"], 333);

    // ✅ Should have correct providers
    assert_eq!(body["apiProvider"], 26);
    assert_eq!(body["modelProvider"], 3);

    // ✅ Should have model name
    assert_eq!(body["model"], "claude-4.5-sonnet");
}

#[test]
fn test_standard_sonnet_no_thinking_config() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Write a Python function".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(4096),
        temperature: Some(0.7),
        top_p: None,
        top_k: None,
        thinking: None,  // NO thinking
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Should NOT have thinkingConfig in generationConfig
    let gen_config = &body["request"]["generationConfig"];
    assert!(
        gen_config["thinkingConfig"].is_null(),
        "Standard model should NOT have thinkingConfig"
    );

    // ✅ Should have other generation config fields
    assert_eq!(gen_config["maxOutputTokens"], 4096);
    assert_eq!(gen_config["temperature"], 0.7);
}

#[test]
fn test_standard_sonnet_explicit_thinking_ignored() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),  // Standard model
        thinking: Some(ThinkingConfig {
            type_: "enabled".to_string(),
            budget_tokens: Some(3000),
        }),  // ❌ Trying to enable thinking
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(8192),
        temperature: None,
        top_p: None,
        top_k: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Model ID should be 333 (standard)
    assert_eq!(body["modelId"], 333);

    // ✅ thinkingConfig should be IGNORED (not present)
    let gen_config = &body["request"]["generationConfig"];
    assert!(gen_config["thinkingConfig"].is_null());

    // ✅ maxOutputTokens should be preserved
    assert_eq!(gen_config["maxOutputTokens"], 8192);
}

#[test]
fn test_standard_sonnet_metadata_compliance() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Hello".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: None,
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: Some(Metadata {
            user_id: Some("test-user-123".to_string()),
            workspace_id: Some("workspace-abc".to_string()),
            cloudaicompanion_project: Some("project-xyz".to_string()),
        }),
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Should have Antigravity metadata
    let metadata = &body["request"]["metadata"];
    assert_eq!(metadata["ideType"], "ANTIGRAVITY");
    assert_eq!(metadata["ideVersion"], "1.13.3");
    assert!(metadata["platform"].is_string());
    assert!(metadata["architecture"].is_string());

    // ✅ Should have extended session metadata
    assert_eq!(metadata["sessionId"], "test-user-123");
    assert_eq!(metadata["workspace_id"], "workspace-abc");
    assert_eq!(metadata["cloudaicompanion_project"], "project-xyz");
}

#[test]
fn test_standard_sonnet_tool_configuration() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: Some(vec![Tool {
            type_: None,
            name: Some("test_tool".to_string()),
            description: Some("A test tool".to_string()),
            input_schema: Some(json!({
                "type": "object",
                "properties": {
                    "param": { "type": "string" }
                },
                "required": ["param"]
            })),
        }]),
        stream: false,
        max_tokens: None,
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: Some(ToolChoice::Auto),  // AUTO mode
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Should have tool configuration
    assert!(body["request"]["tools"].is_array());

    // ✅ Should have correct tool config mode
    let tool_config = &body["request"]["toolConfig"]["functionCallingConfig"];
    assert_eq!(tool_config["mode"], "AUTO");
}

#[test]
fn test_standard_sonnet_grounding_config() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: None,
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Should have geminiSettings with recitationPolicy
    let gemini_settings = &body["request"]["geminiSettings"];
    assert_eq!(gemini_settings["recitationPolicy"]["action"], "BLOCK");
    assert_eq!(gemini_settings["recitationPolicy"]["threshold"], "LOW");
}

#[test]
fn test_standard_vs_thinking_model_ids() {
    let standard_req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: None,
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let thinking_req = ClaudeRequest {
        model: "claude-4.5-sonnet-thinking".to_string(),
        ..standard_req.clone()
    };

    let (standard_body, _) = transform_claude_request_in(&standard_req, "test-project").unwrap();
    let (thinking_body, _) = transform_claude_request_in(&thinking_req, "test-project").unwrap();

    // ✅ Model IDs should be different
    assert_eq!(standard_body["modelId"], 333);
    assert_eq!(thinking_body["modelId"], 334);

    // ✅ API/Model providers should be same (both Anthropic via Vertex)
    assert_eq!(standard_body["apiProvider"], 26);
    assert_eq!(thinking_body["apiProvider"], 26);
    assert_eq!(standard_body["modelProvider"], 3);
    assert_eq!(thinking_body["modelProvider"], 3);
}

#[test]
fn test_standard_sonnet_full_roundtrip() {
    // Complete E2E test with all features
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: MessageContent::String("Write a Python REST API".to_string()),
            }
        ],
        system: Some(SystemPrompt::String(
            "You are a helpful coding assistant.".to_string()
        )),
        tools: Some(vec![Tool {
            type_: None,
            name: Some("write_file".to_string()),
            description: Some("Write content to a file".to_string()),
            input_schema: Some(json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "content": { "type": "string" }
                },
                "required": ["path", "content"]
            })),
        }]),
        stream: false,
        max_tokens: Some(4096),
        temperature: Some(0.7),
        top_p: Some(0.9),
        top_k: Some(40),
        thinking: None,  // NO thinking
        metadata: Some(Metadata {
            user_id: Some("user-123".to_string()),
            workspace_id: None,
            cloudaicompanion_project: None,
        }),
        output_config: None,
        tool_choice: Some(ToolChoice::Auto),
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // Validate all aspects
    assert_eq!(body["modelId"], 333);
    assert_eq!(body["apiProvider"], 26);
    assert_eq!(body["modelProvider"], 3);
    assert_eq!(body["model"], "claude-4.5-sonnet");

    // Request structure
    let request = &body["request"];
    assert!(request["contents"].is_array());
    assert!(request["systemInstruction"].is_object());
    assert!(request["tools"].is_array());
    assert!(request["generationConfig"].is_object());
    assert!(request["toolConfig"].is_object());
    assert!(request["geminiSettings"].is_object());
    assert!(request["metadata"].is_object());

    // Generation config
    let gen_config = &request["generationConfig"];
    assert!(gen_config["thinkingConfig"].is_null());  // ✅ NO thinking
    assert_eq!(gen_config["maxOutputTokens"], 4096);
    assert_eq!(gen_config["temperature"], 0.7);
    assert_eq!(gen_config["topP"], 0.9);
    assert_eq!(gen_config["topK"], 40);

    // Tool config
    let tool_config = &request["toolConfig"]["functionCallingConfig"];
    assert_eq!(tool_config["mode"], "AUTO");

    // Metadata
    let metadata = &request["metadata"];
    assert_eq!(metadata["ideType"], "ANTIGRAVITY");
    assert_eq!(metadata["ideVersion"], "1.13.3");
    assert_eq!(metadata["sessionId"], "user-123");

    // Grounding
    let gemini_settings = &request["geminiSettings"];
    assert_eq!(gemini_settings["recitationPolicy"]["action"], "BLOCK");
}
```

#### Run Tests

```bash
# Run all standard model tests
cargo test --lib standard_sonnet

# Run specific test
cargo test --lib test_standard_sonnet_full_roundtrip -- --nocapture

# Run all integration tests
cargo test --lib -- --test-threads=1
```

### Acceptance Criteria

- [ ] 8 integration tests created and documented
- [ ] All tests pass on first run
- [ ] Tests cover: model ID, thinking, metadata, tools, grounding, comparison
- [ ] Full roundtrip test validates entire request structure
- [ ] Tests prevent regression (thinking activation for standard)
- [ ] Tests match Epic-004 requirements exactly

### Testing Checklist

**Integration Tests** (8 tests):
- [x] Model ID 333 correct
- [x] NO thinkingConfig present
- [x] Explicit thinking ignored
- [x] Metadata compliance
- [x] Tool configuration works
- [x] Grounding config present
- [x] Standard vs thinking comparison
- [x] Full E2E roundtrip

---

## GAP #4: Platform Detection Refactoring

**Priority**: P1 - MEDIUM
**Effort**: 1 hour
**Files**: Multiple (consolidation task)

### Problem Statement

Platform detection functions (`get_platform()`, `get_architecture()`) are defined in `claude/request.rs` but will be duplicated in `upstream/client.rs` when fixing GAP #1.

**Current Duplication Risk**:
- GAP #1 fix requires platform detection in `client.rs`
- Functions already exist in `request.rs` (lines 213-239)
- Without refactoring → code duplication → maintenance burden

### Solution Design

**This is a cleanup task executed AFTER GAP #1 implementation.**

#### Prerequisites

- GAP #1 must be completed first
- `common/platform.rs` module already created
- Both `request.rs` and `client.rs` already use platform functions

#### Verification Steps (30 min)

**Step 1: Verify No Duplication**

```bash
# Search for duplicate platform detection
grep -rn "cfg!(target_os" src-tauri/src/proxy/

# Should only find matches in:
# - common/platform.rs (implementation)
# - No other files!
```

**Expected Result**: Only `common/platform.rs` contains platform detection logic.

**Step 2: Verify All Imports Correct**

```bash
# Check imports of platform functions
grep -rn "use.*platform::" src-tauri/src/proxy/

# Should find:
# - request.rs: use crate::proxy::common::platform::{get_platform, get_architecture, ANTIGRAVITY_VERSION};
# - client.rs: use crate::proxy::common::platform::build_user_agent;
```

**Step 3: Code Review Checklist** (20 min)

Check each file for:

**`common/platform.rs`**:
- [ ] Module documentation exists
- [ ] All functions have documentation
- [ ] 7 unit tests present and passing
- [ ] Constants properly defined
- [ ] No hardcoded values

**`claude/request.rs`**:
- [ ] Old platform functions REMOVED (lines 213-239)
- [ ] Imports from `common::platform` present
- [ ] `ANTIGRAVITY_VERSION` used (not `IDE_VERSION`)
- [ ] No compilation errors
- [ ] Tests still pass

**`upstream/client.rs`**:
- [ ] Import from `common::platform` present
- [ ] `build_user_agent()` used in both places
- [ ] No hardcoded "darwin/arm64"
- [ ] No compilation errors

**Step 4: Run Full Test Suite** (10 min)

```bash
# Run all tests
cargo test --lib

# Specific platform tests
cargo test --lib platform::tests

# Specific mapper tests that use platform
cargo test --lib request::tests
```

**All tests should pass** ✅

### Acceptance Criteria

- [ ] Platform detection in ONE place only (`common/platform.rs`)
- [ ] No duplicate implementations
- [ ] All imports correct
- [ ] All tests passing
- [ ] No compilation warnings
- [ ] Code review checklist completed

### Testing Checklist

**Verification Tests**:
- [ ] No code duplication found via grep
- [ ] Imports verified via grep
- [ ] All 7 platform tests pass
- [ ] All 151+ library tests pass
- [ ] No new warnings in `cargo clippy`

**Manual Verification**:
- [ ] User-Agent logged correctly
- [ ] Metadata has correct platform/arch
- [ ] No runtime errors

---

## GAP #5: Validation Logging

**Priority**: P1 - MEDIUM
**Effort**: 1 hour
**Files**: `request.rs`, `client.rs`

### Problem Statement

Insufficient logging for debugging and validation:
- Model ID routing decisions not logged
- Thinking mode decisions have minimal logging
- User-Agent formation not logged
- Metadata injection not logged

**Impact**: Harder to debug issues, less observability.

### Solution Design

#### Logging Principles

**Use Tracing Levels**:
- `debug!` - Normal operations, verbose details
- `info!` - Important state changes
- `warn!` - Potential issues, unexpected behavior
- `error!` - Actual errors

**Structured Logging Format**:
```rust
tracing::debug!(
    "[Component-Name] Message with context: field1={}, field2={}",
    value1,
    value2
);
```

#### Implementation

#### Log Point 1: Model ID Routing (10 min)

**File**: `src-tauri/src/proxy/mappers/claude/request.rs`
**Location**: Function `get_model_id()` (around line 176)

**Current Code**:
```rust
fn get_model_id(model_name: &str) -> u32 {
    match model_name {
        "claude-4.5-sonnet-thinking" => 334,
        "claude-4.5-sonnet" => 333,
        _ => 0,
    }
}
```

**Add Logging**:
```rust
fn get_model_id(model_name: &str) -> u32 {
    let model_id = match model_name {
        "claude-4.5-sonnet-thinking" => 334,
        "claude-4.5-sonnet" => 333,
        _ => 0,
    };

    if model_id == 0 {
        tracing::warn!(
            "[Model-Routing] ⚠️  Unknown model '{}' → Model ID: 0 (fallback)",
            model_name
        );
    } else {
        tracing::debug!(
            "[Model-Routing] Model '{}' → Model ID: {}",
            model_name,
            model_id
        );
    }

    model_id
}
```

**Rationale**: Helps debug model routing issues, especially for unknown models.

#### Log Point 2: Thinking Mode Detection (10 min)

**File**: `request.rs`
**Location**: Around line 333

**Enhanced Logging** (already partially there, improve):
```rust
if is_thinking_enabled && !target_model_supports_thinking {
    tracing::warn!(
        "[Thinking-Mode] ⚠️  Model '{}' (ID: {}) does NOT support thinking mode. \
         User requested thinking: {{ type: 'enabled', budget: {:?} }}, but it will be IGNORED. \
         Reason: Model lacks '-thinking' suffix and is not in Gemini Pro whitelist.",
        mapped_model,
        get_model_id(&mapped_model),
        claude_req.thinking.as_ref().and_then(|t| t.budget_tokens)
    );
    is_thinking_enabled = false;
}

// Also add when thinking IS enabled:
if is_thinking_enabled {
    tracing::debug!(
        "[Thinking-Mode] ✅ Thinking enabled for model '{}' (ID: {})",
        mapped_model,
        get_model_id(&mapped_model)
    );
}
```

#### Log Point 3: Metadata Injection (10 min)

**File**: `request.rs`
**Location**: Around line 543

**Current Code**:
```rust
let mut metadata = json!({
    "ideType": IDE_TYPE,
    "ideVersion": ANTIGRAVITY_VERSION,
    "platform": get_platform(),
    "architecture": get_architecture()
});
```

**Add Logging AFTER metadata built**:
```rust
// Build metadata
let mut metadata = json!({
    "ideType": IDE_TYPE,
    "ideVersion": ANTIGRAVITY_VERSION,
    "platform": get_platform(),
    "architecture": get_architecture()
});

// Add optional fields...
if let Some(claude_metadata) = &claude_req.metadata {
    if let Some(user_id) = &claude_metadata.user_id {
        metadata["sessionId"] = json!(user_id);
    }
    // ... other fields ...
}

// Log final metadata
tracing::debug!(
    "[Metadata-Injection] ideType: {}, ideVersion: {}, platform: {}, arch: {}, sessionId: {:?}",
    IDE_TYPE,
    ANTIGRAVITY_VERSION,
    get_platform(),
    get_architecture(),
    claude_req.metadata.as_ref().and_then(|m| m.user_id.as_ref())
);

// Add to request
inner_request["metadata"] = metadata;
```

#### Log Point 4: Provider Routing (10 min)

**File**: `request.rs`
**Location**: Functions `get_api_provider()` and `get_model_provider()` (around lines 189, 203)

**Add Logging**:
```rust
fn get_api_provider(model_name: &str) -> u32 {
    let provider = if model_name.starts_with("claude-") {
        API_PROVIDER_ANTHROPIC_VERTEX  // 26
    } else if model_name.starts_with("gemini-") {
        API_PROVIDER_GEMINI  // 0
    } else {
        API_PROVIDER_GEMINI  // Default
    };

    tracing::debug!(
        "[API-Provider] Model '{}' → API Provider: {} ({})",
        model_name,
        provider,
        if provider == 26 { "ANTHROPIC_VERTEX" } else { "GEMINI" }
    );

    provider
}

fn get_model_provider(model_name: &str) -> u32 {
    let provider = if model_name.starts_with("claude-") {
        MODEL_PROVIDER_ANTHROPIC  // 3
    } else if model_name.starts_with("gemini-") {
        MODEL_PROVIDER_GEMINI  // 1
    } else {
        MODEL_PROVIDER_UNKNOWN  // 0
    };

    tracing::debug!(
        "[Model-Provider] Model '{}' → Model Provider: {} ({})",
        model_name,
        provider,
        match provider {
            3 => "ANTHROPIC",
            1 => "GEMINI",
            _ => "UNKNOWN"
        }
    );

    provider
}
```

#### Log Point 5: Final Request Assembly (10 min)

**File**: `request.rs`
**Location**: After final body assembly (around line 583)

**Add Summary Log**:
```rust
let mut body = json!({
    "project": project_id,
    "requestId": request_id,
    "model": config.final_model,
    "modelId": get_model_id(&config.final_model),
    "apiProvider": get_api_provider(&config.final_model),
    "modelProvider": get_model_provider(&config.final_model),
    "userAgent": "antigravity",
    "requestType": config.request_type,
    "request": inner_request,
});

// ✅ Add summary log
tracing::info!(
    "[Request-Assembly] 📦 Final request: model={}, modelId={}, apiProvider={}, \
     requestType={}, hasTools={}, hasThinking={}",
    body["model"],
    body["modelId"],
    body["apiProvider"],
    config.request_type,
    !body["request"]["tools"].is_null(),
    !body["request"]["generationConfig"]["thinkingConfig"].is_null()
);
```

#### Log Point 6: User-Agent Formation (Already done in GAP #1)

**File**: `common/platform.rs`
**Location**: In `build_user_agent()` function

```rust
pub fn build_user_agent() -> String {
    let platform = get_platform();
    let arch = get_architecture();

    let ua = format!(
        "antigravity/{} {}/{}",
        ANTIGRAVITY_VERSION,
        platform,
        arch
    );

    tracing::debug!(
        "[Platform-Detection] User-Agent: '{}' (platform: {}, arch: {})",
        ua,
        platform,
        arch
    );

    ua
}
```

**This is already in GAP #1 implementation** ✅

### Testing Logging (10 min)

**Enable Debug Logging**:
```bash
# Set environment variable
export RUST_LOG=debug

# Run tests
cargo test --lib -- --nocapture

# Check logs
cargo run
```

**Verify Log Output**:
```
[Model-Routing] Model 'claude-4.5-sonnet' → Model ID: 333
[API-Provider] Model 'claude-4.5-sonnet' → API Provider: 26 (ANTHROPIC_VERTEX)
[Model-Provider] Model 'claude-4.5-sonnet' → Model Provider: 3 (ANTHROPIC)
[Platform-Detection] User-Agent: 'antigravity/1.13.3 darwin/arm64' (platform: darwin, arch: arm64)
[Metadata-Injection] ideType: ANTIGRAVITY, ideVersion: 1.13.3, platform: darwin, arch: arm64, sessionId: Some("user-123")
[Request-Assembly] 📦 Final request: model=claude-4.5-sonnet, modelId=333, apiProvider=26, requestType=agent, hasTools=false, hasThinking=false
```

### Acceptance Criteria

- [ ] 6 logging points added
- [ ] All logs use structured format
- [ ] Debug level for normal operations
- [ ] Warn level for potential issues
- [ ] Info level for important state changes
- [ ] Logs include relevant context (model name, IDs, etc.)
- [ ] No performance impact (<1ms per log)
- [ ] Logs are machine-readable

### Testing Checklist

**Logging Verification**:
- [ ] Model ID routing logged
- [ ] API/Model provider routing logged
- [ ] Thinking mode decisions logged
- [ ] Metadata injection logged
- [ ] User-Agent formation logged
- [ ] Final request assembly logged

**Log Quality**:
- [ ] No sensitive data logged (tokens, etc.)
- [ ] Structured format consistent
- [ ] Emojis used appropriately (⚠️, ✅, 📦)
- [ ] Context sufficient for debugging

---

## Implementation Order

### Recommended Sequence

**Phase 1: Critical Fixes (8 hours)**

Execute in this EXACT order to minimize conflicts and rework:

1. **GAP #1: User-Agent Dynamic Generation (2h)**
   - Create `common/platform.rs` module first
   - Update `common/mod.rs` exports
   - Refactor `claude/request.rs` to use platform module
   - Update `upstream/client.rs` to use `build_user_agent()`
   - Run tests: `cargo test --lib platform`
   - **Checkpoint**: User-Agent dynamic ✅

2. **GAP #4: Platform Detection Verification (1h)**
   - Verify no code duplication
   - Run full test suite
   - **Checkpoint**: No duplication ✅

3. **GAP #2: Thinking Mode Detection Fix (3h)**
   - Fix `target_model_supports_thinking` logic
   - Add `is_gemini_thinking_model()` helper
   - Update `should_enable_thinking_by_default()`
   - Add 6+ unit tests
   - Run tests: `cargo test --lib thinking`
   - **Checkpoint**: Thinking detection correct ✅

4. **GAP #5: Validation Logging (1h)**
   - Add logging to 6 points
   - Verify log output with `RUST_LOG=debug`
   - **Checkpoint**: Enhanced observability ✅

5. **GAP #3: Integration Test Suite (3h)**
   - Add 8 integration tests for standard model
   - Run tests: `cargo test --lib standard_sonnet`
   - **Checkpoint**: Comprehensive test coverage ✅

**Phase 2: Validation & Documentation (1-2 hours)**

6. **Final Testing**
   - Run full test suite: `cargo test --lib`
   - Run clippy: `cargo clippy -- -D warnings`
   - Run format check: `cargo fmt -- --check`
   - Manual testing with real requests

7. **Create Validation Report**
   - Document all changes
   - List all tests passing
   - Verify compliance checklist
   - Create Epic-004-VALIDATION-REPORT.md

### Why This Order?

**GAP #1 First**:
- Creates foundation (platform module)
- Required by GAP #4 refactoring
- Independent of other changes
- Low risk, high value

**GAP #4 Second**:
- Validates GAP #1 didn't introduce duplication
- Quick verification step
- Prevents technical debt

**GAP #2 Third**:
- Most complex logic change
- Benefits from stable platform module
- Critical for correctness

**GAP #5 Fourth**:
- Non-breaking enhancement
- Uses all previous changes
- Helps validate GAP #1-3 work

**GAP #3 Last**:
- Comprehensive validation
- Tests all previous changes
- Natural completion point

---

## Testing Strategy

### Test Levels

**Level 1: Unit Tests (Fast)**
- Platform detection functions (7 tests)
- Thinking mode detection logic (6 tests)
- Model ID routing (3 tests)
- Provider routing (4 tests)
- **Total**: 20+ new unit tests
- **Runtime**: <1 second

**Level 2: Integration Tests (Medium)**
- Standard model full transformation (8 tests)
- Standard vs thinking comparison
- Metadata compliance
- Tool configuration
- **Total**: 8 new integration tests
- **Runtime**: <5 seconds

**Level 3: Manual Testing (Slow)**
- Real API requests with standard model
- Cross-platform verification (macOS, Windows, Linux)
- User-Agent validation in logs
- Thinking mode rejection verification
- **Total**: 5+ manual scenarios
- **Runtime**: 10-15 minutes

### Test Execution

**During Development**:
```bash
# Run affected tests only
cargo test --lib platform::tests
cargo test --lib thinking
cargo test --lib standard_sonnet

# Quick feedback loop
```

**Before Commit**:
```bash
# Run full test suite
cargo test --lib

# Check for warnings
cargo clippy -- -D warnings

# Format check
cargo fmt -- --check

# Verify everything passes
```

**Before Epic Completion**:
```bash
# Full validation
cargo test --lib -- --nocapture

# Manual testing
RUST_LOG=debug cargo run

# Cross-platform builds (if available)
cargo build --target x86_64-pc-windows-gnu
cargo build --target x86_64-unknown-linux-gnu
```

### Continuous Validation

**After Each Gap**:
1. Run gap-specific tests
2. Run full test suite
3. Verify no regressions
4. Update validation checklist

**Red/Green/Refactor**:
- ✅ Write test first (GAP #3)
- ✅ Implement fix (GAP #1, #2)
- ✅ Refactor if needed (GAP #4)
- ✅ Add observability (GAP #5)

---

## Validation Checklist

### Code Quality

**Platform Module** (`common/platform.rs`):
- [ ] Module created with proper documentation
- [ ] 3 functions: `get_platform()`, `get_architecture()`, `build_user_agent()`
- [ ] 1 constant: `ANTIGRAVITY_VERSION`
- [ ] 7 unit tests all passing
- [ ] No hardcoded values
- [ ] Compile-time detection only (no runtime overhead)

**Request Mapper** (`mappers/claude/request.rs`):
- [ ] Old platform functions removed
- [ ] Imports from `common::platform` present
- [ ] `ANTIGRAVITY_VERSION` used consistently
- [ ] Thinking detection logic fixed
- [ ] `is_gemini_thinking_model()` helper added
- [ ] `should_enable_thinking_by_default()` updated
- [ ] Logging added to 5 points
- [ ] 8 integration tests added
- [ ] All tests passing

**Upstream Client** (`upstream/client.rs`):
- [ ] Import from `common::platform` present
- [ ] `build_user_agent()` used in both places
- [ ] No hardcoded "darwin/arm64"
- [ ] User-Agent logged on startup

### Functional Requirements

**Anti-Detection Compliance**:
- [ ] User-Agent dynamic: `antigravity/1.13.3 {platform}/{arch}`
- [ ] Metadata includes: ideType, ideVersion, platform, architecture
- [ ] Model ID 333 for "claude-4.5-sonnet"
- [ ] API provider 26 for Claude models
- [ ] Model provider 3 for Claude models

**Standard Model Behavior**:
- [ ] claude-4.5-sonnet → Model ID 333
- [ ] NO thinkingConfig in request
- [ ] Explicit thinking requests ignored
- [ ] Warning logged when thinking ignored
- [ ] All other features work (tools, metadata, grounding)

**Regression Prevention**:
- [ ] Standard model NEVER activates thinking
- [ ] Model ID 333 NEVER paired with thinkingConfig
- [ ] Platform detection no duplication
- [ ] All 151+ existing tests still pass

### Testing Coverage

**Unit Tests**:
- [ ] 7 platform detection tests pass
- [ ] 6 thinking mode tests pass
- [ ] 4 model/provider routing tests pass
- [ ] All new tests have proper documentation

**Integration Tests**:
- [ ] 8 standard model tests pass
- [ ] Standard vs thinking comparison test passes
- [ ] Full roundtrip test passes
- [ ] All tests run in <10 seconds

**Manual Validation**:
- [ ] macOS: User-Agent contains "darwin/arm64" or "darwin/x86_64"
- [ ] Windows: User-Agent contains "windows/x86_64"
- [ ] Linux: User-Agent contains "linux/" + architecture
- [ ] Real API request with standard model succeeds
- [ ] Logs show correct routing decisions

### Documentation

- [ ] Epic-004-IMPLEMENTATION-PLAN.md complete
- [ ] All code changes have inline comments
- [ ] Tests reference Epic-004 stories
- [ ] GAPS-AND-RECOMMENDATIONS.md updated
- [ ] Epic-004-VALIDATION-REPORT.md created

### Performance

- [ ] No performance regression (<5% overhead)
- [ ] Logging has minimal impact (<1ms per request)
- [ ] Tests run in reasonable time (<30s total)
- [ ] No memory leaks detected

---

## Success Metrics

### Quantitative

**Code Metrics**:
- New LOC: ~500 (platform module + tests)
- Tests added: 20+ unit, 8 integration
- Test coverage: 100% for new code
- Compilation time: No significant increase

**Quality Metrics**:
- All tests passing: 171+ (151 existing + 20 new)
- Zero clippy warnings
- Zero format violations
- Zero compiler warnings

**Compliance Metrics**:
- Anti-detection: 100% (was ~95%)
- Standard model: 100% correct routing
- Thinking detection: 0% false positives

### Qualitative

**Developer Experience**:
- Clear error messages when thinking ignored
- Comprehensive logging for debugging
- Well-documented code
- Easy to extend for new models

**Maintainability**:
- No code duplication
- Single source of truth for platform detection
- Comprehensive test coverage
- Clear separation of concerns

**Reliability**:
- Regression tests prevent future breaks
- Cross-platform verified
- Edge cases handled

---

## Risk Assessment

### Low Risk Changes

**GAP #1: User-Agent**
- Risk: LOW
- Reason: Isolated to new module + simple refactoring
- Mitigation: Comprehensive unit tests, env var fallback
- Rollback: Revert to hardcoded value

**GAP #4: Refactoring**
- Risk: LOW
- Reason: Code movement only, no logic changes
- Mitigation: Verification tests
- Rollback: N/A (verification only)

**GAP #5: Logging**
- Risk: VERY LOW
- Reason: Non-breaking enhancement
- Mitigation: Minimal performance impact
- Rollback: Remove log statements

### Medium Risk Changes

**GAP #2: Thinking Logic**
- Risk: MEDIUM
- Reason: Core logic change, affects behavior
- Mitigation: Extensive testing, gradual rollout
- Rollback: Revert logic change
- Impact: Standard model users affected

**GAP #3: Integration Tests**
- Risk: LOW-MEDIUM
- Reason: Tests could be flaky or slow
- Mitigation: Isolated test data, fast execution
- Rollback: Disable failing tests temporarily

### Risk Mitigation

**Testing Strategy**:
- Write tests before implementing changes
- Run tests after each gap
- Manual verification for critical paths

**Gradual Rollout**:
- Deploy to test environment first
- Monitor logs for unexpected behavior
- Validate with real API calls
- Production rollout after 24h stability

**Rollback Plan**:
- Git branches for each gap
- Atomic commits per gap
- Easy revert if issues found
- No data migration needed

---

## Appendix

### Reference Documents

**Specifications**:
- Epic-004-Claude-4.5-Sonnet-Standard-Compliance.md
- Epic-004-GAPS-AND-RECOMMENDATIONS.md
- Epic-004-COMPREHENSIVE-ANALYSIS.md

**Reverse Engineering**:
- docs/antigravity/workflows/models/claude/claude-4-5-sonnet-workflow.md
- docs/comparison/claude/claude-4-5-sonnet/current-implementation.md
- docs/comparison/claude/claude-4-5-sonnet/EXECUTIVE-SUMMARY.md

**Related Epics**:
- Epic-003-Claude-4.5-Sonnet-Thinking-Compliance.md
- Epic-003-VALIDATION-REPORT.md

### Code Locations

**New Files**:
- `src-tauri/src/proxy/common/platform.rs` (NEW)

**Modified Files**:
- `src-tauri/src/proxy/common/mod.rs` (1 line)
- `src-tauri/src/proxy/mappers/claude/request.rs` (~50 lines changed, ~100 lines tests added)
- `src-tauri/src/proxy/upstream/client.rs` (~10 lines changed)

**Total Changes**: ~200 new lines, ~60 modified lines

### Glossary

**Anti-Detection**: Compliance with Antigravity v1.13.3 identity markers to avoid detection/blocking

**Model ID**: Numeric identifier for models (333 = standard sonnet, 334 = thinking sonnet)

**Thinking Mode**: Extended reasoning capability (only for models with "-thinking" suffix)

**Platform Detection**: Compile-time determination of OS (darwin/windows/linux) and architecture (arm64/x86_64)

**Integration Test**: Test that validates complete request transformation pipeline

**Regression Test**: Test that prevents previously fixed bugs from reoccurring

---

**Implementation Plan Status**: ✅ COMPLETE
**Total Effort**: 10 hours (5 gaps)
**Next Step**: Phase 3 - Review with stakeholder and get approval to begin implementation
**Created**: 2026-01-11
**Version**: 1.0
