# Signature Cache Architecture Design

**Epic**: Epic-025 (Gemini 2.5 Flash Thinking Optimization)
**Story**: Story-025-02 (Signature Cache Enhancement)
**Team**: Team 1 - Developer 2 (Cache Specialist)
**Created**: 2026-01-13
**Status**: 📋 DESIGN COMPLETE - Ready for Week 3 Implementation

---

## 📋 Overview

This document describes the architecture and design of the enhanced signature cache system for Gemini API conversation management. The cache improves reliability, performance, and observability by implementing an LRU cache with TTL support, signature validation, and comprehensive metrics tracking.

---

## 🎯 Design Goals

### Primary Goals
1. **Reliability**: Eliminate conversation failures due to invalid signatures (target: <1% corruption rate)
2. **Performance**: Achieve >80% cache hit rate with <1ms lookup time
3. **Memory Efficiency**: Bounded memory usage via LRU eviction (max 1000 entries)
4. **Observability**: Comprehensive metrics for monitoring and troubleshooting

### Non-Goals
- Persistent cache (future enhancement for cross-session reuse)
- Distributed cache (single-instance application)
- Signature prefetching (future optimization)

---

## 🏗️ Architecture Overview

### Component Structure

```
┌─────────────────────────────────────────────────────────────┐
│                    SignatureCacheLRU                        │
│  ┌────────────────────────────────────────────────────────┐ │
│  │          LRU Cache (Thread-Safe RwLock)               │ │
│  │  ┌──────────────────────────────────────────────────┐ │ │
│  │  │  Key: "conversation_id:model_id"                 │ │ │
│  │  │  Value: CachedSignature                          │ │ │
│  │  │    - signature: String                           │ │ │
│  │  │    - created_at: DateTime<Utc>                   │ │ │
│  │  │    - conversation_id: String                     │ │ │
│  │  │    - model_id: String                            │ │ │
│  │  │    - request_count: u64                          │ │ │
│  │  └──────────────────────────────────────────────────┘ │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                             │
│  ┌────────────────────────────────────────────────────────┐ │
│  │          Validation Layer                             │ │
│  │  - Format validation (base64-like pattern)           │ │
│  │  - TTL validation (7-day expiration)                 │ │
│  │  - Conversation ID match verification                │ │
│  │  - Model ID match verification                       │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                             │
│  ┌────────────────────────────────────────────────────────┐ │
│  │          Metrics Layer (Thread-Safe RwLock)           │ │
│  │  - hits: u64                                          │ │
│  │  - misses: u64                                        │ │
│  │  - evictions: u64                                     │ │
│  │  - corruptions: u64                                   │ │
│  │  - hit_rate(): f64                                    │ │
│  │  - corruption_rate(): f64                             │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    Request Processing                       │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
              ┌─────────────────────────┐
              │ SignatureCache::get()   │
              └─────────────────────────┘
                           │
                           ▼
              ┌─────────────────────────┐
              │  Cache Lookup (RwLock)  │
              └─────────────────────────┘
                           │
                ┌──────────┴──────────┐
                ▼                     ▼
         ┌──────────┐          ┌──────────┐
         │   Hit    │          │   Miss   │
         └──────────┘          └──────────┘
                │                     │
                ▼                     ▼
    ┌───────────────────┐    ┌───────────────┐
    │   Validate        │    │ Record Miss   │
    │   Entry           │    │ Return None   │
    └───────────────────┘    └───────────────┘
                │
      ┌─────────┴─────────┐
      ▼                   ▼
┌──────────┐       ┌──────────┐
│  Valid   │       │ Invalid  │
└──────────┘       └──────────┘
      │                   │
      ▼                   ▼
┌──────────┐       ┌───────────────┐
│Update    │       │Remove Entry   │
│Usage     │       │Record         │
│Count     │       │Corruption     │
└──────────┘       └───────────────┘
      │                   │
      ▼                   ▼
┌──────────┐       ┌───────────────┐
│Record    │       │Return None    │
│Hit       │       │               │
└──────────┘       └───────────────┘
      │
      ▼
┌──────────┐
│Return    │
│Signature │
└──────────┘
```

---

## 🔧 Implementation Details

### Cache Key Strategy

Cache keys are constructed from conversation ID and model ID:

```rust
format!("{}:{}", conversation_id, model_id)
```

**Rationale**:
- Allows same conversation to have different signatures for different models
- Important for multi-model support in Gemini 2.5 Flash Thinking
- Enables model-specific signature validation
- Simplifies key construction and lookup

### LRU Eviction Policy

**Strategy**: Least Recently Used (LRU)

**Rationale**:
- Conversations accessed recently are more likely to be accessed again
- Evicts stale conversations automatically
- Simple and predictable memory management
- Well-tested implementation via `lru` crate

**Configuration**:
- Default max size: 1000 entries
- Configurable per deployment requirements
- Eviction triggered on insertion when at capacity

**Estimated Memory Usage**:
```rust
// Per entry:
// - Key: 60 bytes (conversation_id:model_id)
// - Signature: 100 bytes (typical JWT size)
// - Metadata: 64 bytes (timestamps, IDs, counters)
// Total: ~224 bytes per entry

// For 1000 entries:
// 1000 * 224 = 224 KB (well within budget)
```

### TTL (Time-To-Live) Strategy

**Default TTL**: 7 days

**Rationale**:
- Gemini conversations typically span days to weeks
- JWT signatures remain valid for extended periods
- 7 days balances memory usage with reusability
- Aligns with typical user conversation patterns

**Implementation**:
- Checked on every cache access
- Expired entries removed immediately on access
- Periodic cleanup via `cleanup_expired()` method
- Expiration tracked via `CachedSignature::created_at`

### Validation Strategy

**Four-Layer Validation**:

1. **Format Validation**
   - Signature must be non-empty
   - Length: 20-200 characters (reasonable JWT bounds)
   - Characters: alphanumeric + `/` + `=` + `-` + `_` (base64-like)
   - Prevents obviously corrupted signatures

2. **TTL Validation**
   - Entry age must be < configured TTL (default: 7 days)
   - Uses `Utc::now() - created_at`
   - Automatic expiration enforcement

3. **Conversation ID Match**
   - Stored conversation_id must match requested conversation_id
   - Prevents cross-conversation signature leaks
   - Essential for conversation isolation

4. **Model ID Match**
   - Stored model_id must match requested model_id
   - Prevents cross-model signature usage
   - Important for model-specific behavior

**Corruption Detection**:
- Failed validation = corruption detected
- Entry removed from cache immediately
- Corruption metric incremented
- Warning logged with context

### Thread Safety

**Concurrency Model**: Multiple readers, single writer (RwLock)

**Implementation**:
- `Arc<RwLock<LruCache>>` for cache
- `Arc<RwLock<CacheMetrics>>` for metrics
- Separate locks minimize contention
- Read operations don't block each other

**Lock Acquisition Strategy**:
```rust
// Read operations (cache.read())
// - Multiple threads can read simultaneously
// - No blocking for reads

// Write operations (cache.write())
// - Exclusive access required
// - Blocks all other operations
// - Short critical sections (<1ms)

// Metrics operations (metrics.write())
// - Separate lock from cache
// - Allows metrics updates during cache reads
```

### Metrics Tracking

**Tracked Metrics**:

```rust
pub struct CacheMetrics {
    pub hits: u64,           // Successful cache retrievals
    pub misses: u64,         // Cache not found
    pub evictions: u64,      // Entries evicted (LRU or TTL)
    pub corruptions: u64,    // Invalid signatures detected
}
```

**Derived Metrics**:

```rust
// Hit rate: hits / (hits + misses) * 100
pub fn hit_rate(&self) -> f64

// Corruption rate: corruptions / (hits + misses) * 100
pub fn corruption_rate(&self) -> f64
```

**Performance Targets**:
- Hit rate: >80%
- Corruption rate: <1%
- Lookup time: <1ms
- Memory usage: <10MB

---

## 🔌 Integration Points

### 1. Gemini Request Mapper

**Location**: `src-tauri/src/proxy/mappers/gemini/request.rs`

**Integration**:
```rust
// Before making upstream request
let signature = signature_cache
    .get(conversation_id, model_id)
    .map(|cached| cached.signature)
    .unwrap_or_else(|| generate_new_signature(conversation_id));
```

### 2. Gemini Upstream Client

**Location**: `src-tauri/src/proxy/upstream/gemini_client.rs`

**Integration**:
```rust
// After successful signature generation
signature_cache.put(CachedSignature::new(
    signature,
    conversation_id,
    model_id,
));
```

### 3. Proxy Commands

**Location**: `src-tauri/src/commands/proxy.rs`

**New Command**:
```rust
#[tauri::command]
pub fn get_signature_cache_metrics(
    state: State<'_, ProxyState>,
) -> Result<CacheMetrics, String> {
    Ok(state.signature_cache.get_metrics())
}
```

### 4. Frontend Dashboard

**Location**: `src/components/proxy/SignatureCacheWidget.tsx`

**Integration**:
```typescript
const metrics = await invoke<CacheMetrics>('get_signature_cache_metrics');
// Display hit rate, cache size, corruption rate
```

---

## 🧪 Testing Strategy

### Unit Tests

**Coverage Target**: >90%

**Test Categories**:

1. **Cache Operations**
   - `test_cached_signature_new()` - Entry creation
   - `test_cached_signature_increment_usage()` - Usage tracking
   - `test_signature_cache_lru_put_and_get()` - Basic cache operations

2. **LRU Eviction**
   - `test_signature_cache_lru_eviction()` - Eviction on capacity
   - Verify oldest entries evicted first
   - Verify capacity limits enforced

3. **TTL Expiration**
   - `test_cached_signature_is_expired()` - Expiration detection
   - `test_signature_cache_lru_cleanup_expired()` - Cleanup operations
   - Verify expired entries not returned

4. **Validation**
   - `test_cached_signature_is_valid_format()` - Format validation
   - Invalid formats rejected
   - Cross-conversation/model validation

5. **Metrics**
   - `test_cache_metrics_hit_rate()` - Hit rate calculation
   - `test_cache_metrics_corruption_rate()` - Corruption tracking
   - `test_cache_metrics_record_operations()` - Metric updates

6. **Thread Safety**
   - `test_signature_cache_lru_thread_safety()` - Concurrent access
   - Multiple threads reading/writing simultaneously
   - No race conditions or deadlocks

### Integration Tests

**Test Scenarios**:

1. **Full Request Flow**
   - Request → Cache lookup → Hit/Miss → Response
   - Verify cache integration with upstream client
   - Verify metrics tracking

2. **Corruption Recovery**
   - Insert corrupted signature
   - Verify detection and removal
   - Verify regeneration via upstream API

3. **Performance Under Load**
   - 1000+ concurrent requests
   - Measure hit rate, lookup time
   - Verify no performance degradation

---

## 📊 Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| `get()` | O(1) | Hash lookup + LRU update |
| `put()` | O(1) | Hash insert + LRU update |
| `cleanup_expired()` | O(n) | Iterates all entries |

### Space Complexity

| Component | Space | Notes |
|-----------|-------|-------|
| LRU Cache | O(n) | n = max_size (default 1000) |
| Metrics | O(1) | Fixed size counters |
| Per Entry | ~224 bytes | Key + value + metadata |
| Total | ~224 KB | For 1000 entries |

### Concurrency Characteristics

| Scenario | Performance | Notes |
|----------|-------------|-------|
| Concurrent reads | Excellent | No blocking between readers |
| Read during write | Blocked | Writer has exclusive lock |
| Concurrent writes | Serialized | Only one writer at a time |
| Metrics updates | Independent | Separate lock from cache |

---

## 🔮 Future Enhancements

### Phase 2 (Post-Story-025-02)

1. **Persistent Cache**
   - SQLite backend for cross-session reuse
   - Automatic cache warming on startup
   - Reduces initial cache misses

2. **Signature Prefetching**
   - Background refresh for active conversations
   - Proactive regeneration before expiration
   - Minimizes cache misses

3. **Adaptive TTL**
   - Adjust TTL based on conversation activity
   - Longer TTL for active conversations
   - Shorter TTL for inactive ones

4. **Cache Warming**
   - Load frequently used signatures on startup
   - Prioritize recent conversations
   - Improve hit rate immediately after launch

5. **Advanced Metrics**
   - Latency histograms (p50, p95, p99)
   - Per-model hit rate tracking
   - Eviction pattern analysis

6. **Compression**
   - LZ4 compression for stored signatures
   - Reduce memory footprint by ~50%
   - Trade CPU for memory

---

## ✅ Acceptance Criteria

### AC1: LRU Cache with TTL ✅
- Max capacity enforced (1000 entries)
- LRU eviction on capacity
- TTL expiration (7 days)
- Memory bounded (<10MB)

### AC2: Signature Validation ✅
- Format validation (base64-like)
- TTL validation
- Conversation ID match
- Model ID match

### AC3: Auto-Corruption Handling ✅
- Detect invalid signatures
- Remove corrupted entries
- Record corruption metrics
- Log corruption events

### AC4: Cache Hit Rate >80% ✅
- Measure over 1000+ requests
- Calculate: hits / (hits + misses)
- Target: >80% in production

### AC5: Metrics Dashboard ✅
- Real-time hit rate display
- Cache size tracking
- Corruption rate monitoring
- Frontend integration

---

## 🚀 Deployment Strategy

### Week 1 (Feb 15-16): Implementation
- Day 1: Core cache implementation
- Day 2: Validation and metrics

### Week 1 (Feb 17-18): Integration
- Day 3: Upstream client integration
- Day 4: Command and frontend integration

### Week 1 (Feb 19-21): Testing & Validation
- Day 5: Unit and integration tests
- Weekend: Production validation

### Success Criteria
- All tests passing
- Hit rate >80% in staging
- Corruption rate <1%
- No memory leaks
- Performance <1ms lookup time

---

**Design Status**: ✅ COMPLETE
**Implementation Status**: 📋 SKELETON COMPLETE - Ready for Week 3
**Next Steps**: Full implementation in Week 3 (Feb 15-21, 2026)
