# Epic-001: Proactive Quota Monitoring - Implementation Plan

**Epic ID**: QUOTA-001
**Status**: READY TO START
**Priority**: 🔴 P0 CRITICAL
**Team**: 3 Developers
**Duration**: 2-3 weeks (Mar 22 - Apr 11, 2026)
**Prepared**: 2026-03-21

---

## 📊 Executive Summary

### Problem
- **15-20% 429 error rate** causing poor UX
- Reactive quota management (no proactive monitoring)
- Account switching takes 2-5 seconds
- No visibility into quota consumption

### Solution
Implement **proactive quota monitoring system** with:
- Real-time quota tracking via Google `fetchAvailableModels` API
- Pre-request quota validation
- Background monitoring (5-min intervals)
- Intelligent account selection with tier detection
- Quota health UI indicators

### Success Metrics
```yaml
target_improvements:
  - "429 Error Rate: 15-20% → <3%"
  - "Account Switch Latency: 2-5s → <500ms"
  - "API Success Rate: 70-80% → >95%"
```

---

## 📋 Story Overview & Dependencies

### All Stories (6 Total - 22 Story Points)

```yaml
phase_1_parallel (3 stories - 11 points):
  QUOTA-001-01: "Quota API Integration" (5 points, P0)
  QUOTA-001-04: "Subscription Tier Detection" (3 points, P1)
  QUOTA-001-05: "Quota Cache Implementation" (3 points, P0)

phase_2_integration (2 stories - 8 points):
  QUOTA-001-02: "Pre-Request Quota Validation" (3 points, P0)
  QUOTA-001-03: "Background Quota Monitoring" (5 points, P1)

phase_3_ui (1 story - 3 points):
  QUOTA-001-06: "Quota Health UI Indicators" (3 points, P2)
```

### Dependency Graph

```
     ┌─────────────────┐
     │  QUOTA-001-05   │
     │ Quota Cache     │◄────┐
     │  (Dev 3)        │     │
     └────────┬────────┘     │
              │              │
              ▼              │
     ┌─────────────────┐     │
     │  QUOTA-001-01   │     │
     │ Quota API       │─────┤
     │  (Dev 1)        │     │
     └────────┬────────┘     │
              │              │
     ┌────────┴────────┐     │
     │  QUOTA-001-04   │     │
     │ Tier Detection  │◄────┘
     │  (Dev 2)        │
     └────────┬────────┘
              │
       ┌──────┴──────┐
       ▼             ▼
  ┌─────────┐  ┌─────────┐
  │ 001-02  │  │ 001-03  │
  │Pre-Req  │  │Background│
  │(Dev 1+3)│  │(Dev 2+1) │
  └────┬────┘  └────┬─────┘
       └──────┬─────┘
              ▼
        ┌─────────┐
        │ 001-06  │
        │   UI    │
        │(Dev 3+2)│
        └─────────┘
```

**Key Dependencies**:
- ✅ Phase 1 stories are **FULLY PARALLEL** (no dependencies)
- 🔗 Phase 2 stories require Phase 1 completion
- 🔗 Phase 3 (UI) requires all previous phases

---

## 🎯 Phase 1: Parallel Foundation (Week 1 - 5 days)

### Dev 1: QUOTA-001-01 - Quota API Integration

**Duration**: 5 days (40 hours)
**Priority**: 🔴 P0

#### Deliverables

**1. New File: `src-tauri/src/proxy/quota_fetcher.rs`**
```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaInfo {
    /// Remaining quota as fraction (0.0-1.0)
    pub remaining_fraction: f64,

    /// UTC timestamp when quota resets
    pub reset_time: String,  // ISO 8601 format
}

#[derive(Debug, Clone)]
pub struct QuotaFetcher {
    client: Client,
    api_base: String,
}

impl QuotaFetcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            api_base: "https://cloudcode-pa.googleapis.com".to_string(),
        }
    }

    /// Fetch available models with quota info
    pub async fn fetch_available_models(
        &self,
        access_token: &str,
        project_id: &str,
    ) -> Result<HashMap<String, ModelQuotaInfo>, String> {
        let url = format!("{}/v1internal:fetchAvailableModels", self.api_base);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "antigravity/1.13.3 Darwin/arm64")
            .json(&serde_json::json!({
                "project": project_id
            }))
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        let data: FetchModelsResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(data.models)
    }
}

#[derive(Debug, Deserialize)]
struct FetchModelsResponse {
    models: HashMap<String, ModelQuotaInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelQuotaInfo {
    pub display_name: String,
    pub supported_generation_methods: Vec<String>,
    #[serde(rename = "quotaInfo")]
    pub quota_info: Option<QuotaInfo>,
}
```

#### Tasks Breakdown

**Day 1-2: Core API Client**
- [ ] Create `quota_fetcher.rs` module
- [ ] Implement `fetch_available_models()` API call
- [ ] Add data structures (`QuotaInfo`, `ModelQuotaInfo`)
- [ ] Handle authentication (Bearer token)
- [ ] Parse JSON response

**Day 3: Error Handling**
- [ ] Handle 401 (token expired) → trigger refresh
- [ ] Handle 403 (forbidden) → mark account blocked
- [ ] Handle 429 (rate limited) → exponential backoff
- [ ] Handle 500 (server error) → retry logic

**Day 4: Unit Tests**
- [ ] Test `fetch_available_models()` with mock responses
- [ ] Test error handling (401, 403, 429, 500)
- [ ] Test JSON parsing edge cases
- [ ] Test timeout handling

**Day 5: Integration Tests**
- [ ] Test with real API (dev account)
- [ ] Verify quota data accuracy
- [ ] Performance testing (<200ms API call)
- [ ] Documentation

#### Success Criteria
- ✅ API client successfully fetches quota data
- ✅ Error handling covers all HTTP status codes
- ✅ Unit test coverage >80%
- ✅ API response time <200ms (p95)

---

### Dev 2: QUOTA-001-04 - Subscription Tier Detection

**Duration**: 3 days (24 hours)
**Priority**: 🟡 P1

#### Deliverables

**1. Extend `src-tauri/src/proxy/quota_fetcher.rs`**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    pub cloudaicompanion_project: String,
    pub current_tier: Option<TierInfo>,
    pub paid_tier: Option<TierInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierInfo {
    pub id: String,  // "FREE", "PRO", "ULTRA"
    pub quota_tier: String,
    pub name: String,
    pub slug: String,
}

impl QuotaFetcher {
    /// Load code assist to get subscription tier
    pub async fn load_code_assist(
        &self,
        access_token: &str,
    ) -> Result<SubscriptionInfo, String> {
        let url = format!("{}/v1internal:loadCodeAssist", self.api_base);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "antigravity/windows/amd64")
            .json(&serde_json::json!({
                "metadata": {
                    "ideType": "ANTIGRAVITY"
                }
            }))
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Extract effective subscription tier (prioritize paid_tier)
    pub fn get_subscription_tier(info: &SubscriptionInfo) -> String {
        info.paid_tier
            .as_ref()
            .or(info.current_tier.as_ref())
            .map(|t| t.id.clone())
            .unwrap_or_else(|| "FREE".to_string())
    }
}
```

**2. Tier-Based Account Prioritization**
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubscriptionTier {
    Free = 0,
    Pro = 1,
    Ultra = 2,
}

impl From<&str> for SubscriptionTier {
    fn from(s: &str) -> Self {
        match s {
            "ULTRA" => SubscriptionTier::Ultra,
            "PRO" => SubscriptionTier::Pro,
            _ => SubscriptionTier::Free,
        }
    }
}

pub fn prioritize_accounts(
    accounts: Vec<ProxyToken>,
    tiers: &HashMap<String, SubscriptionTier>,
) -> Vec<ProxyToken> {
    let mut sorted = accounts;
    sorted.sort_by(|a, b| {
        let tier_a = tiers.get(&a.account_id).unwrap_or(&SubscriptionTier::Free);
        let tier_b = tiers.get(&b.account_id).unwrap_or(&SubscriptionTier::Free);
        tier_b.cmp(tier_a)  // Higher tier first
    });
    sorted
}
```

#### Tasks Breakdown

**Day 1: API Implementation**
- [ ] Implement `load_code_assist()` API call
- [ ] Add data structures (`SubscriptionInfo`, `TierInfo`)
- [ ] Implement tier extraction logic
- [ ] Unit tests with mock responses

**Day 2: Prioritization Logic**
- [ ] Create `SubscriptionTier` enum
- [ ] Implement `prioritize_accounts()` function
- [ ] Add tier comparison logic
- [ ] Unit tests for prioritization

**Day 3: Integration & Testing**
- [ ] Test with real API (Free/Pro accounts)
- [ ] Verify tier detection accuracy
- [ ] Integration tests
- [ ] Documentation

#### Success Criteria
- ✅ Tier detection works for Free/Pro accounts
- ✅ Account prioritization based on tier
- ✅ Unit test coverage >80%
- ✅ Tier detection <100ms (cached)

---

### Dev 3: QUOTA-001-05 - Quota Cache Implementation

**Duration**: 3 days (24 hours)
**Priority**: 🔴 P0

#### Deliverables

**1. New File: `src-tauri/src/proxy/quota_cache.rs`**
```rust
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CachedQuotaInfo {
    pub quota_info: QuotaInfo,
    pub cached_at: Instant,
    pub ttl: Duration,
}

impl CachedQuotaInfo {
    pub fn new(quota_info: QuotaInfo, ttl: Duration) -> Self {
        Self {
            quota_info,
            cached_at: Instant::now(),
            ttl,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.cached_at.elapsed() < self.ttl
    }

    pub fn is_stale(&self) -> bool {
        !self.is_valid()
    }
}

pub struct QuotaCache {
    /// Cache: "account_id:model_name" -> CachedQuotaInfo
    cache: Arc<DashMap<String, CachedQuotaInfo>>,

    /// Default TTL for quota entries
    default_ttl: Duration,
}

impl QuotaCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            default_ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// Get quota info from cache
    pub fn get(&self, account_id: &str, model: &str) -> Option<QuotaInfo> {
        let key = format!("{}:{}", account_id, model);

        let entry = self.cache.get(&key)?;

        if entry.is_valid() {
            Some(entry.quota_info.clone())
        } else {
            // Remove stale entry
            drop(entry);
            self.cache.remove(&key);
            None
        }
    }

    /// Set quota info in cache
    pub fn set(&self, account_id: &str, model: &str, quota_info: QuotaInfo) {
        let key = format!("{}:{}", account_id, model);
        let cached = CachedQuotaInfo::new(quota_info, self.default_ttl);
        self.cache.insert(key, cached);
    }

    /// Batch update quotas for all models
    pub fn set_all(&self, account_id: &str, quotas: HashMap<String, QuotaInfo>) {
        for (model, quota_info) in quotas {
            self.set(account_id, &model, quota_info);
        }
    }

    /// Clear all cache entries for account
    pub fn clear_account(&self, account_id: &str) {
        self.cache.retain(|key, _| !key.starts_with(&format!("{}:", account_id)));
    }

    /// Clear entire cache
    pub fn clear_all(&self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let total = self.cache.len();
        let valid = self.cache.iter().filter(|entry| entry.is_valid()).count();
        let stale = total - valid;

        CacheStats {
            total_entries: total,
            valid_entries: valid,
            stale_entries: stale,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub valid_entries: usize,
    pub stale_entries: usize,
}
```

#### Tasks Breakdown

**Day 1: Core Cache Implementation**
- [ ] Create `quota_cache.rs` module
- [ ] Implement `QuotaCache` with DashMap
- [ ] Add `CachedQuotaInfo` with TTL tracking
- [ ] Implement get/set operations

**Day 2: Advanced Operations**
- [ ] Implement `set_all()` for batch updates
- [ ] Implement `clear_account()` for account removal
- [ ] Implement `stats()` for cache metrics
- [ ] Add stale entry cleanup

**Day 3: Testing & Optimization**
- [ ] Unit tests for cache operations
- [ ] Test TTL expiration logic
- [ ] Test concurrent access (thread safety)
- [ ] Performance benchmarks (<10ms operations)
- [ ] Documentation

#### Success Criteria
- ✅ Cache operations are thread-safe
- ✅ TTL expiration works correctly
- ✅ Cache operations <10ms (p99)
- ✅ Unit test coverage >90%

---

## 🔗 Phase 2: Integration (Week 2 - 5 days)

### Dev 1 + Dev 3: QUOTA-001-02 - Pre-Request Quota Validation

**Duration**: 3 days (48 hours combined)
**Priority**: 🔴 P0

#### Deliverables

**1. New File: `src-tauri/src/proxy/quota_manager.rs`**
```rust
use crate::proxy::quota_cache::QuotaCache;
use crate::proxy::quota_fetcher::QuotaFetcher;

pub struct QuotaManager {
    cache: Arc<QuotaCache>,
    fetcher: Arc<QuotaFetcher>,
    tier_cache: Arc<DashMap<String, SubscriptionTier>>,
}

impl QuotaManager {
    pub fn new(cache_ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(QuotaCache::new(cache_ttl_seconds)),
            fetcher: Arc::new(QuotaFetcher::new()),
            tier_cache: Arc::new(DashMap::new()),
        }
    }

    /// Check quota before making request
    pub async fn check_quota(
        &self,
        account_id: &str,
        model: &str,
        access_token: &str,
        project_id: &str,
    ) -> Result<QuotaDecision, String> {
        // Try cache first
        if let Some(quota) = self.cache.get(account_id, model) {
            return Ok(self.make_decision(&quota));
        }

        // Cache miss - fetch from API
        let quotas = self.fetcher
            .fetch_available_models(access_token, project_id)
            .await?;

        // Update cache for all models
        let quota_map: HashMap<String, QuotaInfo> = quotas
            .into_iter()
            .filter_map(|(name, info)| {
                info.quota_info.map(|qi| (name, qi))
            })
            .collect();

        self.cache.set_all(account_id, quota_map.clone());

        // Get specific model quota
        let quota = quota_map.get(model)
            .ok_or_else(|| format!("Model {} not found in quota data", model))?;

        Ok(self.make_decision(quota))
    }

    /// Make decision based on remaining fraction
    fn make_decision(&self, quota: &QuotaInfo) -> QuotaDecision {
        if quota.remaining_fraction >= 0.10 {
            QuotaDecision::Proceed
        } else if quota.remaining_fraction > 0.0 {
            QuotaDecision::LowQuota {
                remaining: quota.remaining_fraction,
                reset_time: quota.reset_time.clone(),
            }
        } else {
            QuotaDecision::Exhausted {
                reset_time: quota.reset_time.clone(),
            }
        }
    }
}

#[derive(Debug)]
pub enum QuotaDecision {
    /// Quota is healthy (>10% remaining)
    Proceed,

    /// Quota is low (0-10% remaining)
    LowQuota {
        remaining: f64,
        reset_time: String,
    },

    /// Quota is exhausted (0% remaining)
    Exhausted {
        reset_time: String,
    },
}
```

**2. Integration with TokenManager** (`src-tauri/src/proxy/token_manager.rs`):
```rust
impl TokenManager {
    /// Get token with quota validation
    pub async fn get_token_with_quota(
        &self,
        model: &str,
    ) -> Result<ProxyToken, String> {
        let mut attempts = 0;
        let max_attempts = self.tokens.len();

        while attempts < max_attempts {
            attempts += 1;

            // Select account (prioritize by tier)
            let token = self.select_account_by_tier()?;

            // Check quota before using
            let quota_decision = self.quota_manager
                .check_quota(
                    &token.account_id,
                    model,
                    &token.access_token,
                    &token.project_id,
                )
                .await?;

            match quota_decision {
                QuotaDecision::Proceed => {
                    tracing::info!(
                        "[Quota-OK] Account {} has sufficient quota for {}",
                        token.account_id, model
                    );
                    return Ok(token);
                }
                QuotaDecision::LowQuota { remaining, .. } => {
                    tracing::warn!(
                        "[Quota-Low] Account {} has {}% quota remaining",
                        token.account_id, (remaining * 100.0).round()
                    );
                    // Continue to next account
                    continue;
                }
                QuotaDecision::Exhausted { reset_time } => {
                    tracing::error!(
                        "[Quota-Exhausted] Account {} quota exhausted, resets at {}",
                        token.account_id, reset_time
                    );
                    // Continue to next account
                    continue;
                }
            }
        }

        Err("All accounts have insufficient quota".to_string())
    }
}
```

#### Tasks Breakdown

**Day 1: QuotaManager (Dev 1)**
- [ ] Create `quota_manager.rs`
- [ ] Implement `check_quota()` with cache-first strategy
- [ ] Implement `make_decision()` threshold logic
- [ ] Unit tests

**Day 2: TokenManager Integration (Dev 1 + Dev 3)**
- [ ] Add QuotaManager to TokenManager
- [ ] Modify `get_token()` to call `check_quota()`
- [ ] Implement account fallback logic
- [ ] Integration tests

**Day 3: Testing & Refinement (Dev 1 + Dev 3)**
- [ ] End-to-end tests (pre-request validation)
- [ ] Test account switching on low quota
- [ ] Performance testing (<50ms pre-check)
- [ ] Documentation

#### Success Criteria
- ✅ Pre-request quota check <50ms (cached)
- ✅ Automatic account switching on low quota
- ✅ All P0 scenarios tested
- ✅ Integration test coverage >75%

---

### Dev 2 + Dev 1: QUOTA-001-03 - Background Quota Monitoring

**Duration**: 5 days (80 hours combined)
**Priority**: 🟡 P1

#### Deliverables

**1. Background Monitor Task** (in `quota_manager.rs`):
```rust
impl QuotaManager {
    /// Start background monitoring task
    pub fn start_background_monitor(
        self: Arc<Self>,
        tokens: Arc<DashMap<String, ProxyToken>>,
        interval_seconds: u64,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                Duration::from_secs(interval_seconds)
            );

            loop {
                interval.tick().await;

                tracing::info!("[Quota-Monitor] Starting quota refresh cycle");

                let start = Instant::now();
                let results = self.refresh_all_quotas(&tokens).await;
                let duration = start.elapsed();

                tracing::info!(
                    "[Quota-Monitor] Refresh cycle complete: {} accounts in {:?}",
                    results.len(), duration
                );
            }
        })
    }

    /// Refresh quotas for all accounts
    async fn refresh_all_quotas(
        &self,
        tokens: &DashMap<String, ProxyToken>,
    ) -> Vec<RefreshResult> {
        let mut tasks = vec![];

        for entry in tokens.iter() {
            let token = entry.value().clone();
            let fetcher = self.fetcher.clone();
            let cache = self.cache.clone();

            let task = tokio::spawn(async move {
                match fetcher
                    .fetch_available_models(&token.access_token, &token.project_id)
                    .await
                {
                    Ok(quotas) => {
                        let quota_map: HashMap<String, QuotaInfo> = quotas
                            .into_iter()
                            .filter_map(|(name, info)| {
                                info.quota_info.map(|qi| (name, qi))
                            })
                            .collect();

                        cache.set_all(&token.account_id, quota_map.clone());

                        RefreshResult::Success {
                            account_id: token.account_id,
                            model_count: quota_map.len(),
                        }
                    }
                    Err(e) => RefreshResult::Failed {
                        account_id: token.account_id,
                        error: e,
                    },
                }
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        let results = futures::future::join_all(tasks).await;
        results.into_iter().filter_map(|r| r.ok()).collect()
    }
}

#[derive(Debug)]
enum RefreshResult {
    Success {
        account_id: String,
        model_count: usize,
    },
    Failed {
        account_id: String,
        error: String,
    },
}
```

**2. Configuration** (`src-tauri/src/models/app_config.rs`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaConfig {
    /// Enable background quota monitoring
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Refresh interval in seconds
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval_seconds: u64,  // Default: 300 (5 minutes)

    /// Cache TTL in seconds
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_seconds: u64,  // Default: 300 (5 minutes)
}

fn default_true() -> bool { true }
fn default_refresh_interval() -> u64 { 300 }
fn default_cache_ttl() -> u64 { 300 }
```

#### Tasks Breakdown

**Day 1-2: Background Task (Dev 2)**
- [ ] Implement `start_background_monitor()`
- [ ] Implement `refresh_all_quotas()` with parallelization
- [ ] Add error handling and retry logic
- [ ] Unit tests with mock fetcher

**Day 3: Configuration (Dev 1)**
- [ ] Add `QuotaConfig` to `AppConfig`
- [ ] Add configuration UI settings
- [ ] Load config on proxy startup
- [ ] Save config changes

**Day 4: Integration (Dev 1 + Dev 2)**
- [ ] Start monitor task in proxy server startup
- [ ] Add task lifecycle management (start/stop)
- [ ] Add monitoring metrics
- [ ] Integration tests

**Day 5: Stability Testing (Dev 1 + Dev 2)**
- [ ] 48-hour stability test
- [ ] Memory leak detection
- [ ] CPU usage profiling (<1% target)
- [ ] Documentation

#### Success Criteria
- ✅ Background task runs for 48+ hours without issues
- ✅ CPU usage <1% average
- ✅ Memory footprint <10MB
- ✅ Refresh cycle completes in <30s for 10 accounts

---

## 🎨 Phase 3: UI & Polish (Week 3 - 2 days)

### Dev 3 + Dev 2: QUOTA-001-06 - Quota Health UI Indicators

**Duration**: 2 days (32 hours combined)
**Priority**: 🟢 P2

#### Deliverables

**1. Quota Display Component** (`src/components/quota/QuotaIndicator.tsx`):
```typescript
interface QuotaIndicatorProps {
  accountId: string;
  model: string;
}

export const QuotaIndicator: React.FC<QuotaIndicatorProps> = ({ accountId, model }) => {
  const [quotaInfo, setQuotaInfo] = useState<QuotaInfo | null>(null);

  useEffect(() => {
    const fetchQuota = async () => {
      const quota = await invoke('get_quota_info', { accountId, model });
      setQuotaInfo(quota);
    };

    fetchQuota();
    const interval = setInterval(fetchQuota, 60000); // 1 minute
    return () => clearInterval(interval);
  }, [accountId, model]);

  if (!quotaInfo) return null;

  const percentage = (quotaInfo.remainingFraction * 100).toFixed(0);
  const healthColor = getHealthColor(quotaInfo.remainingFraction);

  return (
    <div className="quota-indicator">
      <div className={`badge ${healthColor}`}>
        {percentage}% quota
      </div>
      <div className="text-xs text-gray-500">
        Resets: {formatResetTime(quotaInfo.resetTime)}
      </div>
    </div>
  );
};

function getHealthColor(remaining: number): string {
  if (remaining >= 0.5) return 'badge-success';
  if (remaining >= 0.2) return 'badge-warning';
  return 'badge-error';
}

function formatResetTime(resetTime: string): string {
  const date = new Date(resetTime);
  const now = new Date();
  const hoursUntil = Math.floor((date.getTime() - now.getTime()) / (1000 * 60 * 60));

  if (hoursUntil < 1) return 'in <1h';
  if (hoursUntil < 24) return `in ${hoursUntil}h`;
  return date.toLocaleDateString();
}
```

**2. Tauri Command** (`src-tauri/src/commands/quota.rs`):
```rust
#[tauri::command]
pub async fn get_quota_info(
    account_id: String,
    model: String,
    quota_manager: tauri::State<'_, Arc<QuotaManager>>,
) -> Result<Option<QuotaInfo>, String> {
    Ok(quota_manager.cache.get(&account_id, &model))
}

#[tauri::command]
pub async fn get_all_account_quotas(
    quota_manager: tauri::State<'_, Arc<QuotaManager>>,
) -> Result<HashMap<String, HashMap<String, QuotaInfo>>, String> {
    // Return all cached quotas organized by account
    let stats = quota_manager.cache.stats();
    // ... implementation
    Ok(result)
}
```

#### Tasks Breakdown

**Day 1: UI Components (Dev 3)**
- [ ] Create `QuotaIndicator.tsx` component
- [ ] Add color-coded health indicators
- [ ] Format reset time display
- [ ] Add to account cards in UI

**Day 2: Tauri Commands & Integration (Dev 2 + Dev 3)**
- [ ] Implement `get_quota_info` command
- [ ] Implement `get_all_account_quotas` command
- [ ] Add real-time updates via Tauri events
- [ ] UI testing

#### Success Criteria
- ✅ Quota indicators visible in account UI
- ✅ Color coding reflects quota health
- ✅ Reset time displayed in local timezone
- ✅ UI updates every minute

---

## 🧪 Testing Strategy

### Unit Tests (Dev 1, 2, 3 - Throughout)

**Coverage Target**: >80% for all new code

**Test Areas**:
```yaml
quota_fetcher:
  - API call success/failure
  - JSON parsing
  - Error handling (401, 403, 429, 500)
  - Timeout handling

quota_cache:
  - Get/set operations
  - TTL expiration
  - Concurrent access
  - Cache statistics

quota_manager:
  - Quota decision logic
  - Cache-first strategy
  - Account prioritization
  - Background monitoring

token_manager:
  - Pre-request quota check
  - Account switching
  - Fallback logic
```

### Integration Tests (Dev 1 + Dev 2 + Dev 3 - Week 2-3)

**Scenarios**:
```yaml
end_to_end:
  - Fetch quota from API → Cache → Pre-request check
  - Low quota triggers account switch
  - Background monitoring updates cache
  - UI displays current quota

error_scenarios:
  - API returns 429 → retry with backoff
  - API returns 403 → skip account
  - All accounts exhausted → error message
  - Network timeout → fallback to cache
```

### Performance Tests (Dev 1 - Week 2)

**Benchmarks**:
```yaml
quota_check_latency:
  - Cached: <50ms (p95)
  - API fetch: <200ms (p95)

background_monitoring:
  - CPU usage: <1% average
  - Memory: <10MB
  - Refresh cycle: <30s for 10 accounts

cache_operations:
  - Get: <10ms (p99)
  - Set: <10ms (p99)
```

### Manual Testing (All Devs - Week 3)

**Test Plan**:
```yaml
functional:
  - [ ] Verify quota fetch from Google API
  - [ ] Test account switching on low quota
  - [ ] Verify background monitoring (48h run)
  - [ ] Test UI quota indicators
  - [ ] Test with Free/Pro tier accounts

stress_testing:
  - [ ] 100 concurrent requests
  - [ ] 10 accounts monitored simultaneously
  - [ ] Cache with 1000+ entries
  - [ ] 48-hour stability run
```

---

## 📊 Success Metrics & Validation

### Pre-Launch Criteria (Week 3)

**Must Have (P0)**:
- [ ] 429 error rate <5% (beta target)
- [ ] Pre-request quota validation functional
- [ ] Background monitoring runs for 48+ hours
- [ ] No performance regression (<5% latency increase)
- [ ] All P0 stories complete
- [ ] Unit test coverage >80%

**Should Have (P1)**:
- [ ] Subscription tier detection working
- [ ] Quota cache hit rate >80%
- [ ] All integration tests passing
- [ ] Documentation complete

### Post-Launch Success (30 days)

**Quantitative Metrics**:
```yaml
primary_kpis:
  - "429 Error Rate: <3%"
  - "Account Switch Latency: <500ms"
  - "API Success Rate: >95%"

secondary_kpis:
  - "Quota fetch success rate: >99%"
  - "Cache hit rate: >90%"
  - "Background task uptime: >99.5%"
```

---

## 📅 Timeline & Milestones

### Week 1: Parallel Foundation (Mar 22-28)
```yaml
day_1:
  dev_1: "QUOTA-001-01 Day 1 (API client core)"
  dev_2: "QUOTA-001-04 Day 1 (Tier detection API)"
  dev_3: "QUOTA-001-05 Day 1 (Cache core)"

day_2:
  dev_1: "QUOTA-001-01 Day 2 (API client core)"
  dev_2: "QUOTA-001-04 Day 2 (Prioritization logic)"
  dev_3: "QUOTA-001-05 Day 2 (Cache advanced)"

day_3:
  dev_1: "QUOTA-001-01 Day 3 (Error handling)"
  dev_2: "QUOTA-001-04 Day 3 (Integration & testing)"
  dev_3: "QUOTA-001-05 Day 3 (Testing & optimization)"

day_4:
  dev_1: "QUOTA-001-01 Day 4 (Unit tests)"
  dev_2: "Start QUOTA-001-03 (Background monitor)"
  dev_3: "Start QUOTA-001-02 (Pre-request validation)"

day_5:
  dev_1: "QUOTA-001-01 Day 5 (Integration tests)"
  dev_2: "QUOTA-001-03 Day 2 (Background monitor)"
  dev_3: "QUOTA-001-02 Day 2 (Pre-request validation)"

milestone_week_1: "✅ Phase 1 Complete - Foundation Ready"
```

### Week 2: Integration (Mar 29 - Apr 4)
```yaml
day_1-2:
  dev_1: "QUOTA-001-02 (QuotaManager)"
  dev_2: "QUOTA-001-03 (Background task)"
  dev_3: "QUOTA-001-02 (TokenManager integration)"

day_3-4:
  dev_1: "QUOTA-001-03 (Configuration)"
  dev_2: "QUOTA-001-03 (Integration)"
  dev_3: "QUOTA-001-02 (Testing)"

day_5:
  all_devs: "Integration testing & bug fixes"

milestone_week_2: "✅ Phase 2 Complete - Core Functionality Ready"
```

### Week 3: UI & Stabilization (Apr 5-11)
```yaml
day_1-2:
  dev_2: "QUOTA-001-06 (Tauri commands)"
  dev_3: "QUOTA-001-06 (UI components)"
  dev_1: "Performance testing & optimization"

day_3-5:
  all_devs: "48-hour stability testing, bug fixes, documentation"

milestone_week_3: "✅ Phase 3 Complete - EPIC-001 READY FOR PRODUCTION"
```

---

## 🚀 Deployment Plan

### Beta Testing (Week 3)
```yaml
participants: "5-10 select users"
deployment: "Beta channel"
success_criteria:
  - "429 error rate <5%"
  - "No crashes or critical bugs"
  - "Positive user feedback"
```

### General Availability (Week 4)
```yaml
release: "v3.4.0"
rollout: "All users"
success_criteria:
  - "429 error rate <3%"
  - "Background monitoring stable"
  - "<5 user-reported issues per week"
```

---

## 📚 Documentation

### Required Documentation
```yaml
technical:
  - [ ] QuotaManager API reference
  - [ ] Architecture decision record (ADR)
  - [ ] Performance tuning guide

user:
  - [ ] Update README with quota monitoring
  - [ ] Quota management user guide
  - [ ] UI indicators documentation
  - [ ] Troubleshooting guide
```

---

## 🎯 Risk Mitigation

### High-Risk Items
```yaml
risk_1_api_rate_limiting:
  probability: "Medium"
  impact: "Medium"
  mitigation: "5-min cache TTL, batch requests"
  contingency: "Increase cache TTL to 10 minutes"

risk_2_quota_api_unavailable:
  probability: "Low"
  impact: "High"
  mitigation: "Fallback to reactive 429 handling"
  contingency: "Continue with existing rate limit tracking"
```

---

## 📝 Related Documents

**Epic Documentation**:
- [Epic-001: Proactive Quota Monitoring](./Epic-001-Proactive-Quota-Monitoring.md)
- [Quota APIs Reverse Engineering](../antigravity/api/quota-apis.md)

**Story Documents**:
- [Story QUOTA-001-01](../stories/QUOTA-001-01-quota-api-integration.md)
- [Story QUOTA-001-02](../stories/QUOTA-001-02-pre-request-validation.md)
- [Story QUOTA-001-03](../stories/QUOTA-001-03-background-monitoring.md)
- [Story QUOTA-001-04](../stories/QUOTA-001-04-subscription-tier.md)
- [Story QUOTA-001-05](../stories/QUOTA-001-05-quota-cache.md)
- [Story QUOTA-001-06](../stories/QUOTA-001-06-ui-indicators.md)

---

**Plan Version**: 1.0
**Prepared By**: Product & Engineering Team
**Status**: ✅ READY FOR TEAM KICKOFF
