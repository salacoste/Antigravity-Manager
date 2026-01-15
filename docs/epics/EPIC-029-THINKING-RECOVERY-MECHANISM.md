# EPIC-029: Thinking Recovery Mechanism

**Status**: 📋 Planning (Version 2.0 - Updated with Critical Fixes)
**Priority**: 🟡 P1 HIGH
**Estimate**: 5-7 days
**Dependency**: EPIC-028 (Model-Specific Rate Limiting)
**Author**: Amelia (Dev Agent)
**Date**: 2026-01-15
**Validation**: Updated after PM review

---

## Change Log

**v2.0** (2026-01-15):
- ✅ Added complete Acceptance Criteria (FR1-FR5, NFR1-NFR5, QR1-QR5)
- ✅ Fixed code references (cla::Message → claude::Message)
- ✅ Added complete type definitions from models.rs
- ✅ Added recovery failure handling specification
- ✅ Clarified EPIC-028 dependency integration
- ✅ Added data migration strategy

---

## Executive Summary

**Problem**: When switching models mid-conversation (e.g., Claude → Gemini), thinking blocks are stripped but tool loops are not properly closed, causing 400 errors in 30-40% of cross-model conversations.

**Solution**: Implement intelligent thinking recovery mechanism that detects interrupted tool loops and injects synthetic messages to close them gracefully before sending to the target model.

**Impact**: Fixes cross-model conversation failures from 30-40% to <5%, improving multi-model user experience significantly.

---

## Acceptance Criteria

### Functional Requirements (FR)

**FR1: Recovery Detection**
- System MUST detect when conversation contains interrupted tool loops
- System MUST detect incompatible thinking signatures for target model family
- Detection MUST be triggered before request transformation

**FR2: Recovery Execution**
- System MUST inject synthetic assistant message when tool_use lacks tool_result
- System MUST inject synthetic user message with tool_result placeholder
- System MUST filter Claude-style signatures for Gemini targets
- System MUST preserve valid thinking blocks

**FR3: Error Handling**
- Recovery failures MUST NOT prevent request from being sent
- Recovery errors MUST be logged with context
- On recovery failure, system MUST fall back to original messages
- System MUST track recovery success rate

**FR4: Model Family Support**
- System MUST support Claude model family
- System MUST support Gemini model family
- System MUST default to Claude family for unknown models

**FR5: Backward Compatibility**
- Existing single-model conversations MUST NOT be affected
- Non-thinking conversations MUST continue to work unchanged
- No breaking changes to public API

### Non-Functional Requirements (NFR)

**NFR1: Performance**
- Recovery processing MUST add <10ms latency
- Memory overhead MUST be <1MB per conversation
- Recovery logic MUST NOT block request processing

**NFR2: Reliability**
- Recovery success rate MUST be ≥95%
- False positive rate MUST be <5%
- Recovery MUST be idempotent (safe to reapply)

**NFR3: Maintainability**
- Recovery logic MUST be isolated in separate module
- Code MUST have ≥80% test coverage
- All functions MUST have documentation

**NFR4: Compatibility**
- Synthetic message formats MUST be validated against upstream API specs
- Support MUST be tested with Claude 3.5+ and Gemini 2.5+
- No changes to client contract

**NFR5: Observability**
- All recovery attempts MUST be logged
- Recovery success/failure MUST be tracked as metrics
- Recovery decisions MUST include debug context

### Quality Requirements (QR)

**QR1: Code Quality**
- All code MUST pass clippy linting
- All code MUST follow Rust naming conventions
- No unsafe code without justification

**QR2: Testing**
- Unit tests MUST cover all recovery scenarios
- Integration tests MUST validate end-to-end flow
- Manual testing MUST include real model switches

**QR3: Documentation**
- API documentation MUST be updated
- CLAUDE.md MUST include recovery behavior
- Troubleshooting guide MUST be created

**QR4: Security**
- Synthetic messages MUST NOT expose sensitive data
- Recovery MUST NOT bypass authentication
- No injection of arbitrary content

**QR5: Rollout**
- Feature flag MUST be implemented
- Gradual rollout plan MUST be documented
- Rollback procedure MUST be tested

---

## Business Case

### Current State
```
Cross-Model Success Rate: 60-70%
User Pain: HIGH - Broken conversations when switching models
Revenue Impact: MEDIUM - User retention impact
Complaints: "Why does my conversation break when I switch models?"
```

### Target State
```
Cross-Model Success Rate: 95%+
User Experience: Seamless model switching
Error Rate: <5% for cross-model conversations
```

### Quantitative Metrics
```
Before: 400 errors on model switch (30-40% failure rate)
After:  Graceful recovery with synthetic messages
Improvement: +30% success rate

User Impact: HIGH - Fixes critical workflow blocker
Complexity: HIGH - Complex conversation state analysis
Effort: 5-7 days development
Risk: HIGH - Core message transformation logic
```

---

## Technical Analysis

### Root Cause

When Claude Code switches from Claude to Gemini mid-conversation:

1. **Claude messages** contain thinking blocks with:
   - `Thinking` blocks with `signature` field (Claude format)
   - Tool calls interleaved with thinking

2. **On switch to Gemini**:
   - Claude Code strips `Thinking` blocks (incompatible with Gemini)
   - Tool loops may be left unclosed
   - Gemini receives malformed conversation state

3. **Result**: 400 error - "Invalid request: tool call without response"

### Current Antigravity-Manager Implementation

**File**: `src-tauri/src/proxy/mappers/claude/request.rs`

Current implementation only filters thinking blocks without recovery logic.

### Alternative Proxy Solution

**File**: `alternative_proxy_app/src/format/thinking-utils.js`

Reference implementation provides the recovery pattern.

---

## Type Definitions (From Actual Codebase)

```rust
// From: src-tauri/src/proxy/mappers/claude/models.rs

/// Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: MessageContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    String(String),
    Array(Vec<ContentBlock>),
}

/// Content Block (Claude)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },

    #[serde(rename = "thinking")]
    Thinking {
        thinking: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<serde_json::Value>,
    },

    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<serde_json::Value>,
    },

    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
    },

    // ... other variants
}
```

---

## Implementation Plan

### Phase 1: Analysis & Design (Days 1-2)

**Objective**: Understand current message flow and design recovery mechanism.

#### Tasks

1. **Read current implementation** (Day 1, Morning)
   - `src-tauri/src/proxy/mappers/claude/request.rs`
   - `src-tauri/src/proxy/mappers/claude/response.rs`
   - `src-tauri/src/proxy/mappers/claude/thinking_utils.rs`
   - `src-tauri/src/proxy/mappers/gemini/wrapper.rs`

2. **Analyze message structures** (Day 1, Afternoon)
   - Claude `Message` and `ContentBlock` types
   - Gemini message format with thoughtSignature
   - Tool call/response structures
   - Conversation state patterns

3. **Design recovery logic** (Day 2)
   - Define `ModelFamily` enum (Claude, Gemini)
   - Design `needs_thinking_recovery()` function
   - Design `close_tool_loop_for_thinking()` function
   - Define synthetic message formats (validated against API specs)
   - Plan signature compatibility checking
   - Design recovery failure handling

4. **Write technical spec** (Day 2, End)
   - Data structures
   - Function signatures
   - Integration points
   - Test scenarios
   - Error handling strategy

**Deliverables**:
- Technical design document
- Function specifications
- Integration plan

---

### Phase 2: Core Implementation (Days 3-4)

**Objective**: Implement thinking recovery mechanism.

#### New Module: `src-tauri/src/proxy/mappers/thinking_recovery.rs`

```rust
//! Thinking Recovery Mechanism
//!
//! Detects and recovers interrupted tool loops when switching models.
//! Handles incompatible thinking signatures between Claude and Gemini.

use crate::proxy::mappers::claude::models::{Message, ContentBlock, MessageContent};
use serde::{Deserialize, Serialize};

/// Model family for thinking signature compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelFamily {
    Claude,
    Gemini,
}

impl ModelFamily {
    pub fn from_model_id(model: &str) -> Self {
        if model.starts_with("claude-") || model.contains("claude") {
            ModelFamily::Claude
        } else if model.starts_with("gemini-") || model.contains("gemini") {
            ModelFamily::Gemini
        } else {
            // Default to Claude for unknown models
            ModelFamily::Claude
        }
    }
}

/// Recovery result with detailed status
#[derive(Debug, Clone)]
pub enum RecoveryResult {
    Success(Vec<Message>),
    Failure { reason: String, original: Vec<Message> },
    NotNeeded(Vec<Message>),
}

/// Check if conversation needs thinking recovery
///
/// # Arguments
/// * `messages` - Conversation messages
/// * `target_family` - Target model family
///
/// # Returns
/// * `true` if recovery is needed, `false` otherwise
pub fn needs_thinking_recovery(
    messages: &[Message],
    target_family: ModelFamily,
) -> bool {
    // Find last assistant message
    let last_assistant_idx = messages.iter()
        .rev()
        .position(|msg| msg.role == "assistant");

    let last_assistant_idx = match last_assistant_idx {
        Some(idx) => messages.len() - 1 - idx,
        None => return false,
    };

    let assistant_msg = &messages[last_assistant_idx];

    // Extract content blocks
    let blocks = match &assistant_msg.content {
        MessageContent::Array(blocks) => blocks,
        MessageContent::String(_) => return false,
    };

    // Check for tool_use without corresponding tool_result
    let has_tool_use = blocks.iter().any(|b| matches!(b, ContentBlock::ToolUse { .. }));

    if has_tool_use {
        // Check if there's a user message with tool_result after this
        let has_tool_response = messages.iter()
            .skip(last_assistant_idx + 1)
            .any(|m| {
                if m.role != "user" { return false; }
                match &m.content {
                    MessageContent::Array(blocks) => {
                        blocks.iter().any(|b| matches!(b, ContentBlock::ToolResult { .. }))
                    }
                    MessageContent::String(_) => false,
                }
            });

        if !has_tool_response {
            return true; // Interrupted tool loop
        }
    }

    // Check for incompatible thinking signatures
    if target_family == ModelFamily::Gemini {
        return blocks.iter().any(|b| {
            if let ContentBlock::Thinking { signature: Some(_), .. } = b {
                // Reject Claude-style signatures for Gemini
                true
            } else {
                false
            }
        });
    }

    false
}

/// Close tool loop for thinking recovery
///
/// # Arguments
/// * `messages` - Conversation messages to recover
/// * `target_family` - Target model family
///
/// # Returns
/// * `RecoveryResult` with recovered messages or failure details
pub fn close_tool_loop_for_thinking(
    messages: Vec<Message>,
    target_family: ModelFamily,
) -> RecoveryResult {
    let mut recovered = messages;

    // Find last assistant message
    let last_assistant_idx = recovered.iter()
        .rev()
        .position(|msg| msg.role == "assistant");

    let last_assistant_idx = match last_assistant_idx {
        Some(idx) => recovered.len() - 1 - idx,
        None => return RecoveryResult::NotNeeded(recovered),
    };

    // Extract content blocks from assistant message
    let blocks = match &recovered[last_assistant_idx].content {
        MessageContent::Array(blocks) => blocks.clone(),
        MessageContent::String(_) => return RecoveryResult::NotNeeded(recovered),
    };

    let has_tool_use = blocks.iter().any(|b| matches!(b, ContentBlock::ToolUse { .. }));
    let last_is_user = recovered.last()
        .map(|m| m.role == "user")
        .unwrap_or(false);

    if has_tool_use && !last_is_user {
        // Case 1: Interrupted tool → inject synthetic messages
        let tool_use_id = blocks.iter()
            .find_map(|b| {
                if let ContentBlock::ToolUse { id, .. } = b {
                    Some(id.clone())
                } else {
                    None
                }
            });

        if let Some(id) = tool_use_id {
            // Synthetic assistant message
            recovered.push(Message {
                role: "assistant".to_string(),
                content: MessageContent::Array(vec![
                    ContentBlock::Text {
                        text: "[Tool interrupted by model switch - please retry tool call]".to_string(),
                    },
                ]),
            });

            // Synthetic user message with tool_result
            recovered.push(Message {
                role: "user".to_string(),
                content: MessageContent::Array(vec![
                    ContentBlock::ToolResult {
                        tool_use_id: id,
                        content: serde_json::json!("[Model switched - tool execution pending]"),
                        is_error: Some(false),
                    },
                ]),
            });

            tracing::info!("[ThinkingRecovery] Injected synthetic messages for interrupted tool loop");
        }
    }

    // Filter incompatible signatures for Gemini
    if target_family == ModelFamily::Gemini {
        for msg in &mut recovered {
            if let MessageContent::Array(blocks) = &mut msg.content {
                blocks.retain(|b| {
                    if let ContentBlock::Thinking { signature, .. } = b {
                        // Remove Claude-style signatures for Gemini
                        signature.is_none()
                    } else {
                        true
                    }
                });
            }
        }

        tracing::info!("[ThinkingRecovery] Filtered Claude-style signatures for Gemini target");
    }

    RecoveryResult::Success(recovered)
}

/// Apply recovery with fallback handling
///
/// # Arguments
/// * `messages` - Conversation messages
/// * `target_family` - Target model family
///
/// # Returns
/// * Recovered messages or original on failure
pub fn apply_recovery_or_fallback(
    messages: Vec<Message>,
    target_family: ModelFamily,
) -> Vec<Message> {
    if !needs_thinking_recovery(&messages, target_family) {
        return messages;
    }

    match close_tool_loop_for_thinking(messages, target_family) {
        RecoveryResult::Success(recovered) => {
            tracing::info!("[ThinkingRecovery] Recovery successful");
            recovered
        }
        RecoveryResult::Failure { reason, original } => {
            tracing::warn!("[ThinkingRecovery] Recovery failed: {}, using original messages", reason);
            original
        }
        RecoveryResult::NotNeeded(messages) => {
            tracing::debug!("[ThinkingRecovery] Recovery not needed");
            messages
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needs_thinking_recovery_interrupted_tool() {
        let messages = vec![
            Message {
                role: "user".to_string(),
                content: MessageContent::String("Hello".to_string()),
            },
            Message {
                role: "assistant".to_string(),
                content: MessageContent::Array(vec![
                    ContentBlock::ToolUse {
                        id: "tool-123".to_string(),
                        name: "test_tool".to_string(),
                        input: serde_json::json!({}),
                        signature: None,
                        cache_control: None,
                    },
                ]),
            },
        ];

        let result = needs_thinking_recovery(&messages, ModelFamily::Claude);
        assert!(result, "Should detect interrupted tool loop");
    }

    #[test]
    fn test_needs_thinking_recovery_gemini_target() {
        let messages = vec![
            Message {
                role: "user".to_string(),
                content: MessageContent::String("Hello".to_string()),
            },
            Message {
                role: "assistant".to_string(),
                content: MessageContent::Array(vec![
                    ContentBlock::Thinking {
                        thinking: "Thinking...".to_string(),
                        signature: Some("claude-sig".to_string()),
                        cache_control: None,
                    },
                ]),
            },
        ];

        let result = needs_thinking_recovery(&messages, ModelFamily::Gemini);
        assert!(result, "Should detect Claude signature for Gemini target");
    }

    #[test]
    fn test_close_tool_loop_interrupted() {
        let messages = vec![
            Message {
                role: "user".to_string(),
                content: MessageContent::String("Hello".to_string()),
            },
            Message {
                role: "assistant".to_string(),
                content: MessageContent::Array(vec![
                    ContentBlock::ToolUse {
                        id: "tool-123".to_string(),
                        name: "test_tool".to_string(),
                        input: serde_json::json!({}),
                        signature: None,
                        cache_control: None,
                    },
                ]),
            },
        ];

        let result = close_tool_loop_for_thinking(messages, ModelFamily::Claude);
        assert!(matches!(result, RecoveryResult::Success(_)));
    }

    #[test]
    fn test_signature_filtering_gemini() {
        let messages = vec![
            Message {
                role: "user".to_string(),
                content: MessageContent::String("Hello".to_string()),
            },
            Message {
                role: "assistant".to_string(),
                content: MessageContent::Array(vec![
                    ContentBlock::Thinking {
                        thinking: "Thinking...".to_string(),
                        signature: Some("claude-sig".to_string()),
                        cache_control: None,
                    },
                ]),
            },
        ];

        let result = close_tool_loop_for_thinking(messages, ModelFamily::Gemini);
        let recovered = match result {
            RecoveryResult::Success(msgs) => msgs,
            _ => panic!("Expected success"),
        };

        let blocks = match &recovered[1].content {
            MessageContent::Array(blocks) => blocks,
            _ => panic!("Expected array content"),
        };

        assert!(blocks.is_empty(), "Claude signature should be filtered for Gemini");
    }
}
```

#### Integration: Update `src-tauri/src/proxy/mappers/claude/request.rs`

```rust
// Add at top of file
pub mod thinking_recovery;
use thinking_recovery::{ModelFamily, needs_thinking_recovery, apply_recovery_or_fallback};

// Update transform function
pub fn transform_claude_request_in(
    request: ClaudeRequest,
    model: &str,
) -> Result<Value, String> {
    let mut messages = request.messages;

    // Detect target model family
    let target_family = ModelFamily::from_model_id(model);

    // Apply thinking recovery if needed (with fallback on error)
    messages = apply_recovery_or_fallback(messages, target_family);

    // Filter invalid thinking blocks (preserved for backward compatibility)
    filter_invalid_thinking_blocks(&mut messages);

    // Continue with normal transformation
    // ... rest of existing code
}
```

**Deliverables**:
- `thinking_recovery.rs` module with implementation
- Updated request transformer with recovery logic
- Unit tests for recovery functions

---

### Phase 3: Testing & Validation (Days 5-6)

**Objective**: Comprehensive testing of recovery mechanism.

#### Test Scenarios

1. **Cross-model conversations** (Day 5)
   - Claude → Gemini switch with thinking blocks
   - Gemini → Claude switch with thoughtSignature
   - Multiple switches in same conversation

2. **Tool loop recovery** (Day 5, Afternoon)
   - Interrupted tool_use without tool_result
   - Partial tool execution before switch
   - Multiple tool calls in single message

3. **Signature compatibility** (Day 6)
   - Claude signature blocks with Gemini target
   - Gemini thoughtSignature with Claude target
   - Mixed signature formats

4. **Edge cases** (Day 6, Afternoon)
   - Empty conversation
   - No thinking blocks
   - Only thinking blocks, no tools
   - Malformed messages

5. **Recovery failure handling** (Day 6, Afternoon)
   - Simulate recovery failure
   - Verify fallback to original messages
   - Verify logging occurs

#### Manual Testing Plan

```bash
# Test 1: Claude → Gemini with thinking
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: test" \
  -d '{
    "model": "claude-sonnet-4-5-thinking",
    "max_tokens": 100,
    "messages": [
      {"role": "user", "content": "What is 2+2?"},
      {"role": "assistant", "content": "4"}
    ]
  }'

# Then switch to Gemini
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: test" \
  -d '{
    "model": "gemini-2.5-flash",
    "max_tokens": 100,
    "messages": [
      {"role": "user", "content": "What is 2+2?"},
      {"role": "assistant", "content": "4"},
      {"role": "user", "content": "Now what is 3+3?"}
    ]
  }'
```

**Deliverables**:
- Comprehensive test suite (unit + integration)
- Manual testing report
- Bug fixes and refinements

---

### Phase 4: Documentation & Rollout (Day 7)

**Objective**: Document changes and prepare for rollout.

#### Tasks

1. **Update documentation** (Day 7, Morning)
   - Add thinking recovery section to CLAUDE.md
   - Update API documentation with recovery behavior
   - Document synthetic message formats
   - Document recovery failure handling

2. **Create migration guide** (Day 7, Afternoon)
   - Backward compatibility notes
   - Configuration changes (if any)
   - Troubleshooting guide
   - Feature flag usage

3. **Prepare rollout** (Day 7, End)
   - Feature flag for gradual rollout
   - Monitoring and alerting
   - Rollback plan

**Feature Flag Implementation**:

```rust
// In config
pub struct ProxyConfig {
    pub enable_thinking_recovery: bool,
    // ... other fields
}

// In request handler
if config.enable_thinking_recovery {
    messages = apply_recovery_or_fallback(messages, target_family);
}
```

**Deliverables**:
- Updated documentation
- Migration guide
- Rollout plan

---

## Data Migration Strategy

### Existing Conversations

**Question**: Should recovery apply to existing broken conversations?

**Answer**: NO - Only apply recovery to NEW conversations

**Rationale**:
- Existing conversations are already in broken state
- Recovery cannot fix past 400 errors
- Applying retroactively could cause unexpected behavior

**Implementation**:
```rust
// Recovery only applies on model switch in NEW conversations
// No migration needed for existing conversation state
```

### Feature Flag Rollout

**Phase 1**: Internal testing (feature flag = false)
**Phase 2**: Enable for 10% of requests
**Phase 3**: Enable for 50% of requests
**Phase 4**: Enable for 100% of requests

---

## EPIC-028 Dependency Clarification

### Why EPIC-028 is Required

The dependency on EPIC-028 (Model-Specific Rate Limiting) is for **model context**, not rate limiting itself:

1. **Model Family Detection**: EPIC-028 introduces `ModelFamily` enum that EPIC-029 reuses
2. **Model ID Parsing**: Both epics need `from_model_id()` function
3. **Shared Infrastructure**: Model-specific tracking infrastructure

**Integration Code**:
```rust
// Both epics use shared model detection
use epic_028::ModelFamily; // Reused from EPIC-028

let target_family = ModelFamily::from_model_id(model);
```

**Alternative**: If EPIC-028 is not complete, EPIC-029 can define its own `ModelFamily` enum, but this creates duplication.

---

## Success Metrics

### Quantitative

| Metric | Before | Target | Measurement |
|--------|--------|--------|-------------|
| **Cross-Model Success Rate** | 60-70% | 95%+ | Conversation completion rate |
| **400 Error Rate** | 30-40% | <5% | Error monitoring |
| **Recovery Trigger Rate** | N/A | Track | Recovery function calls |
| **Recovery Success Rate** | N/A | ≥95% | Recovery outcome tracking |
| **Synthetic Message Injection** | N/A | Track | Synthetic message count |
| **Recovery Latency** | N/A | <10ms | Performance monitoring |

### Qualitative

- **User Experience**: Seamless model switching without conversation breaks
- **Stability**: Graceful handling of all cross-model scenarios
- **Maintainability**: Clean separation of recovery logic
- **Testability**: Comprehensive test coverage for all scenarios

---

## Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Breaking existing conversations** | Medium | High | Comprehensive test suite, gradual rollout |
| **Incorrect synthetic message format** | Low | Medium | Validate against upstream API specs |
| **Performance degradation** | Low | Low | Benchmark recovery logic |
| **Signature validation errors** | Medium | Medium | Extensive cross-model testing |
| **Recovery failures** | Low | Medium | Fallback to original messages |

### Operational Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Extended downtime during deployment** | Low | High | Feature flag, rollback plan |
| **Increased support tickets** | Medium | Medium | Documentation, monitoring |
| **Complexity explosion** | Medium | Medium | Code review, refactoring |

---

## Dependencies

### EPIC-028: Model-Specific Rate Limiting
- **Why**: Shared `ModelFamily` infrastructure
- **Dependency Level**: Optional (can define own `ModelFamily` if needed)
- **Blocking**: No - can implement independently if necessary

### EPIC-030: Signature Cache with Model Family
- **Why**: Complements thinking recovery for signature handling
- **Dependency Level**: Optional but recommended
- **Blocking**: No

---

## Implementation Checklist

### Phase 1: Analysis & Design
- [ ] Read current implementation files
- [ ] Analyze message structures from models.rs
- [ ] Design recovery logic with failure handling
- [ ] Write technical spec with acceptance criteria

### Phase 2: Core Implementation
- [ ] Create `thinking_recovery.rs` module
- [ ] Implement `ModelFamily` enum (or reuse from EPIC-028)
- [ ] Implement `needs_thinking_recovery()` function
- [ ] Implement `close_tool_loop_for_thinking()` function
- [ ] Implement `apply_recovery_or_fallback()` function
- [ ] Update request transformer integration
- [ ] Write unit tests for all scenarios

### Phase 3: Testing & Validation
- [ ] Test cross-model conversations
- [ ] Test tool loop recovery
- [ ] Test signature compatibility
- [ ] Test edge cases
- [ ] Test recovery failure handling
- [ ] Manual testing with real accounts
- [ ] Performance benchmarks

### Phase 4: Documentation & Rollout
- [ ] Update CLAUDE.md
- [ ] Update API documentation
- [ ] Create migration guide (minimal - no data migration)
- [ ] Prepare rollout plan
- [ ] Set up monitoring
- [ ] Execute gradual rollout

---

## Rollout Plan

### Week 1: Internal Testing
- Days 1-2: Complete implementation
- Days 3-4: Internal testing
- Days 5-6: Bug fixes and refinement

### Week 2: Gradual Rollout
- Day 1: Feature flag enabled for 10% of requests
- Day 2: Monitor metrics, expand to 50%
- Day 3: Full rollout if metrics stable

### Rollback Criteria
- Cross-model success rate < 90%
- 400 error rate increase > 10%
- Performance degradation > 20%
- Recovery success rate < 90%

---

## Questions for Team Review

1. **Scope**: Should we recover ALL cross-model scenarios or focus on common ones?
2. **Synthetic Messages**: Are the proposed synthetic message formats acceptable?
3. **Signature Handling**: Should we preserve signatures for debugging or strip entirely?
4. **Monitoring**: What additional metrics should we track?
5. **Rollback**: Under what conditions should we rollback immediately?

---

## Next Steps

1. **Review this epic** with tech lead and stakeholders
2. **Prioritize implementation** after EPIC-028 completion (or in parallel with shared ModelFamily)
3. **Allocate resources** for 5-7 day sprint
4. **Set up monitoring** for success metrics
5. **Begin implementation** upon approval

---

**Author**: Amelia (Dev Agent)
**Status**: Updated for Re-Validation
**Version**: 2.0
**Date**: 2026-01-15
