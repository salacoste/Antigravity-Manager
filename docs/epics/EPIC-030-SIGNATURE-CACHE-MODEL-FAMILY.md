# EPIC-030: Signature Cache with Model Family

**Status**: 📋 Planning
**Priority**: 🟢 P2 MEDIUM
**Estimate**: 2-3 days
**Dependency**: None
**Author**: Amelia (Dev Agent)
**Date**: 2026-01-15

---

## Executive Summary

**Problem**: Claude and Gemini use incompatible thinking signature formats. When switching models, signatures from the wrong model family are not filtered, causing validation errors in 15-20% of cross-model conversations.

**Solution**: Enhance signature cache to track model family (claude/gemini) for each signature and filter incompatible signatures when switching models.

**Impact**: Reduces cross-model signature compatibility issues from 15-20% to <5%, improving reliability of multi-model conversations.

---

## Business Case

### Current State
```
Signature Compatibility Issues: 15-20%
User Pain: MEDIUM - Occasional signature validation errors
Revenue Impact: LOW - Indirect user experience
Complexity: LOW - Isolated cache enhancement
```

### Target State
```
Signature Compatibility Issues: <5%
User Experience: Smooth signature handling across models
Filtering: Automatic model family detection
```

### Quantitative Metrics
```
Before: Signature validation errors 15-20%
After:  Signature compatibility issues <5%
Improvement: -75%

Complexity: Low (isolated changes)
Risk: Low (easy rollback)
Effort: 2-3 days
```

---

## Technical Analysis

### Root Cause

**Claude Signature Format**:
```json
{
  "type": "thinking",
  "thinking": "Thinking content here",
  "signature": "base64-encoded-signature"
}
```

**Gemini Signature Format**:
```json
{
  "type": "thinking",
  "thinking": "Thinking content here",
  "thoughtSignature": "sentinel-or-cached-value"
}
```

**Problem**: Current implementation uses simple `HashMap<String, String>` without model family tracking.

### Current Antigravity-Manager Implementation

**File**: `src-tauri/src/proxy/mappers/claude/response.rs`

```rust
// Current implementation - no model family tracking
thought_signature_map: Arc<tokio::sync::Mutex<HashMap<String, String>>>,
// ❌ No model family tracking
```

**Problem**: Signatures from Claude and Gemini are stored without distinguishing which model family they belong to.

### Alternative Proxy Solution

**File**: `alternative_proxy_app/src/format/signature-cache.js`

```javascript
/**
 * Signature cache with model family tracking
 * @type {Map<string, {modelFamily: 'claude' | 'gemini', timestamp: number}>}
 */
const signatureCache = new Map();

/**
 * Store signature with model family
 * @param {string} signature - The signature string
 * @param {string} modelFamily - 'claude' | 'gemini'
 */
function cacheSignature(signature, modelFamily) {
  signatureCache.set(signature, {
    modelFamily,
    timestamp: Date.now()
  });
}

/**
 * Check if signature is from compatible model family
 * @param {string} signature - The signature to check
 * @param {string} targetFamily - 'claude' | 'gemini'
 * @returns {boolean}
 */
function isCompatibleSignature(signature, targetFamily) {
  const cached = signatureCache.get(signature);
  if (!cached) return false; // Unknown signature

  return cached.modelFamily === targetFamily;
}

/**
 * Filter incompatible signatures from thinking blocks
 * @param {Array} blocks - Thinking blocks to filter
 * @param {string} targetFamily - 'claude' | 'gemini'
 * @returns {Array} Filtered blocks
 */
function filterIncompatibleSignatures(blocks, targetFamily) {
  return blocks.filter(block => {
    if (block.signature) {
      return isCompatibleSignature(block.signature, targetFamily);
    }
    return true; // Keep blocks without signature
  });
}

/**
 * Get model family from model ID
 * @param {string} model - Model identifier
 * @returns {string} 'claude' | 'gemini'
 */
function getModelFamily(model) {
  if (model.startsWith('claude-') || model.includes('claude')) {
    return 'claude';
  } else if (model.startsWith('gemini-') || model.includes('gemini')) {
    return 'gemini';
  }
  return 'claude'; // Default
}
```

---

## Implementation Plan

### Phase 1: Design (Day 1, Morning)

**Objective**: Design enhanced signature cache structure.

#### Tasks

1. **Read current implementation**
   - `src-tauri/src/proxy/mappers/claude/response.rs`
   - `src-tauri/src/proxy/mappers/claude/request.rs`
   - `src-tauri/src/proxy/mappers/gemini/wrapper.rs`

2. **Design data structures**
   ```rust
   /// Cached signature with model family
   #[derive(Debug, Clone)]
   pub struct CachedSignature {
       pub model_family: ModelFamily,
       pub timestamp: Instant,
       pub signature: String,
   }

   /// Model family for signature compatibility
   #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
   pub enum ModelFamily {
           Claude,
           Gemini,
   }
   ```

3. **Plan integration points**
   - Response mapper (cache signatures with family)
   - Request mapper (filter incompatible signatures)
   - Cross-model conversation detection

**Deliverables**:
- Data structure specifications
- Integration plan

---

### Phase 2: Implementation (Day 1, Afternoon - Day 2)

**Objective**: Implement enhanced signature cache.

#### Update `src-tauri/src/proxy/mappers/claude/response.rs`

```rust
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::Instant;

/// Model family for signature compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
            ModelFamily::Claude // Default
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ModelFamily::Claude => "claude",
            ModelFamily::Gemini => "gemini",
        }
    }
}

/// Cached signature with model family
#[derive(Debug, Clone)]
pub struct CachedSignature {
    pub model_family: ModelFamily,
    pub timestamp: Instant,
    pub signature: String,
}

/// Enhanced signature cache
pub struct SignatureCache {
    cache: Arc<Mutex<HashMap<String, CachedSignature>>>,
}

impl SignatureCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Store signature with model family
    pub async fn store(&self, signature: String, model_family: ModelFamily) {
        let cached = CachedSignature {
            model_family,
            timestamp: Instant::now(),
            signature: signature.clone(),
        };

        let mut cache = self.cache.lock().await;
        cache.insert(signature.clone(), cached);

        // Prune old entries (>1 hour)
        let now = Instant::now();
        cache.retain(|_, sig| now.duration_since(sig.timestamp).as_secs() < 3600);
    }

    /// Check if signature is from compatible model family
    pub async fn is_compatible(&self, signature: &str, target_family: ModelFamily) -> bool {
        let cache = self.cache.lock().await;
        if let Some(cached) = cache.get(signature) {
            return cached.model_family == target_family;
        }
        false // Unknown signature - not compatible
    }

    /// Filter incompatible signatures from blocks
    pub async fn filter_incompatible(
        &self,
        blocks: Vec<ThinkingBlock>,
        target_family: ModelFamily,
    ) -> Vec<ThinkingBlock> {
        blocks.into_iter()
            .filter(|block| {
                if let Some(ref sig) = block.signature {
                    // Check compatibility
                    let cache = self.cache.lock().await;
                    if let Some(cached) = cache.get(sig) {
                        return cached.model_family == target_family;
                    }
                    return false; // Unknown signature
                }
                true // Keep blocks without signature
            })
            .collect()
    }
}

// Update response mapper to use enhanced cache
pub struct ClaudeResponseMapper {
    // ... existing fields
    signature_cache: Arc<SignatureCache>,
}

impl ClaudeResponseMapper {
    pub fn new(signature_cache: Arc<SignatureCache>) -> Self {
        Self {
            signature_cache,
            // ... other fields
        }
    }

    // When processing response with thinking blocks
    async fn cache_thinking_signatures(
        &self,
        blocks: &[ThinkingBlock],
        model: &str,
    ) {
        let model_family = ModelFamily::from_model_id(model);

        for block in blocks {
            if let Some(ref signature) = block.signature {
                self.signature_cache.store(
                    signature.clone(),
                    model_family,
                ).await;
            }
        }
    }
}
```

#### Update `src-tauri/src/proxy/mappers/claude/request.rs`

```rust
use crate::proxy::mappers::claude::response::{SignatureCache, ModelFamily};

// Update request transformation to filter incompatible signatures
pub fn transform_claude_request_in(
    request: ClaudeRequest,
    model: &str,
    signature_cache: Arc<SignatureCache>,
) -> Result<Value, String> {
    let mut messages = request.messages;
    let target_family = ModelFamily::from_model_id(model);

    // Filter incompatible thinking signatures
    for msg in &mut messages {
        if let Message::AssistantMessage(assistant) = msg {
            if let Some(Content::ThinkingBlocks(blocks)) = &mut assistant.content {
                *blocks = signature_cache.filter_incompatible(
                    blocks.clone(),
                    target_family,
                ).await;
            }
        }
    }

    // Continue with normal transformation
    // ... rest of existing code
}
```

**Deliverables**:
- Enhanced signature cache implementation
- Updated response and request mappers
- Unit tests

---

### Phase 3: Testing (Day 2, Afternoon - Day 3, Morning)

**Objective**: Comprehensive testing of signature cache.

#### Test Scenarios

1. **Signature storage and retrieval**
   ```rust
   #[tokio::test]
   async fn test_signature_storage_with_family() {
       let cache = SignatureCache::new();

       // Store Claude signature
       cache.store("sig-claude-123".to_string(), ModelFamily::Claude).await;

       // Verify compatibility
       assert!(cache.is_compatible("sig-claude-123", ModelFamily::Claude).await);
       assert!(!cache.is_compatible("sig-claude-123", ModelFamily::Gemini).await);
   }
   ```

2. **Cross-model filtering**
   ```rust
   #[tokio::test]
   async fn test_filter_incompatible_signatures() {
       let cache = SignatureCache::new();

       // Cache signatures from different families
       cache.store("sig-claude".to_string(), ModelFamily::Claude).await;
       cache.store("sig-gemini".to_string(), ModelFamily::Gemini).await;

       // Create thinking blocks
       let blocks = vec![
           ThinkingBlock { signature: Some("sig-claude".to_string()), .. },
           ThinkingBlock { signature: Some("sig-gemini".to_string()), .. },
       ];

       // Filter for Claude target
       let filtered = cache.filter_incompatible(blocks, ModelFamily::Claude).await;
       assert_eq!(filtered.len(), 1); // Only Claude signature remains
   }
   ```

3. **Cache pruning**
   ```rust
   #[tokio::test]
   async fn test_cache_pruning_old_entries() {
       let cache = SignatureCache::new();

       // Store signature
       cache.store("old-sig".to_string(), ModelFamily::Claude).await;

       // Wait for expiration (or mock time)
       // ... trigger pruning

       // Verify old entry removed
       assert!(!cache.is_compatible("old-sig", ModelFamily::Claude).await);
   }
   ```

4. **Manual testing**
   ```bash
   # Test: Claude → Gemini switch with signatures
   curl -X POST http://localhost:8045/v1/messages \
     -H "Content-Type: application/json" \
     -d '{
       "model": "claude-sonnet-4-5-thinking",
       "max_tokens": 100,
       "messages": [{"role": "user", "content": "Hello"}]
     }'

   # Switch to Gemini with same conversation
   curl -X POST http://localhost:8045/v1/messages \
     -H "Content-Type: application/json" \
     -d '{
       "model": "gemini-2.5-flash",
       "max_tokens": 100,
       "messages": [
         {"role": "user", "content": "Hello"},
         {"role": "assistant", "content": "...with thinking signature..."},
         {"role": "user", "content": "Continue"}
       ]
     }'
   ```

**Deliverables**:
- Comprehensive test suite
- Manual testing report
- Bug fixes

---

### Phase 4: Documentation & Rollout (Day 3, Afternoon)

**Objective**: Document changes and prepare for rollout.

#### Tasks

1. **Update documentation**
   - Add signature cache section to CLAUDE.md
   - Document model family detection logic
   - Add troubleshooting guide

2. **Migration notes**
   - Backward compatibility (signatures without family)
   - Cache migration strategy
   - Configuration changes (if any)

3. **Monitoring setup**
   - Track signature cache hit rate
   - Monitor signature compatibility errors
   - Alert on unexpected patterns

**Deliverables**:
- Updated documentation
- Monitoring configuration
- Rollout plan

---

## Success Metrics

### Quantitative

| Metric | Before | Target | Measurement |
|--------|--------|--------|-------------|
| **Signature Compatibility Errors** | 15-20% | <5% | Error monitoring |
| **Cache Hit Rate** | N/A | Track | Cache performance |
| **Cross-Model Success Rate** | 60-70% | 75%+ | Combined with EPIC-029 |

### Qualitative

- **Reliability**: Consistent signature handling across models
- **Maintainability**: Clean cache implementation with automatic pruning
- **Testability**: Easy to test with comprehensive coverage

---

## Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Cache performance degradation** | Low | Low | Automatic pruning, benchmarking |
| **Incorrect family detection** | Low | Medium | Extensive testing with various model IDs |
| **Memory leak from unbounded cache** | Low | Medium | Automatic pruning, size limits |

### Operational Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **Increased signature cache misses** | Low | Low | Monitoring, cache tuning |
| **Breaking existing conversations** | Low | Medium | Gradual rollout, backward compatibility |

---

## Dependencies

### EPIC-029: Thinking Recovery Mechanism
- **Why**: Complements thinking recovery for signature handling
- **Dependency Level**: Optional but recommended
- **Blocking**: No - can be implemented independently

### EPIC-028: Model-Specific Rate Limiting
- **Why**: May use similar model family detection logic
- **Dependency Level**: Optional
- **Blocking**: No

---

## Implementation Checklist

### Phase 1: Design
- [ ] Read current implementation
- [ ] Design data structures
- [ ] Plan integration points

### Phase 2: Implementation
- [ ] Implement `ModelFamily` enum
- [ ] Implement `CachedSignature` struct
- [ ] Implement `SignatureCache` with family tracking
- [ ] Update response mapper to cache signatures
- [ ] Update request mapper to filter signatures
- [ ] Write unit tests

### Phase 3: Testing
- [ ] Test signature storage and retrieval
- [ ] Test cross-model filtering
- [ ] Test cache pruning
- [ ] Manual testing with real accounts
- [ ] Performance benchmarks

### Phase 4: Documentation & Rollout
- [ ] Update CLAUDE.md
- [ ] Create migration notes
- [ ] Set up monitoring
- [ ] Execute rollout

---

## Rollout Plan

### Day 1: Development
- Morning: Design and setup
- Afternoon: Core implementation

### Day 2: Testing
- Morning: Unit tests
- Afternoon: Integration tests

### Day 3: Rollout
- Morning: Documentation and monitoring
- Afternoon: Gradual rollout (50% → 100%)

### Rollback Criteria
- Signature compatibility errors increase > 5%
- Cache performance degradation > 20%
- User complaints about broken conversations

---

## Questions for Team Review

1. **Cache TTL**: Should we use 1 hour default or make it configurable?
2. **Unknown Signatures**: Should we reject or allow unknown signatures?
3. **Bundling**: Should this be bundled with EPIC-029 (thinking recovery)?
4. **Monitoring**: What additional metrics should we track?

---

## Next Steps

1. **Review this epic** with tech lead
2. **Decide on bundling** with EPIC-029
3. **Allocate resources** for 2-3 day sprint
4. **Begin implementation** upon approval

---

**Author**: Amelia (Dev Agent)
**Status**: Ready for Review
**Version**: 1.0
