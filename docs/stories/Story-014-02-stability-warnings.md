# Story-014-02: Experimental Stability Warnings

**Epic**: Epic-014 - Gemini 2.0 Flash Experimental Audio Specialist (Team 1, Gemini Specialists)
**Priority**: P1 (HIGH - risk management)
**Effort**: 2 days
**Assignee**: Dev 1B (Team 1 Mid-Level Developer)
**Status**: ✅ READY FOR EXECUTION (Start: 2026-01-22)
**Created**: 2026-01-12

---

## Objective

Implement clear experimental status warnings in API responses, documentation, and dashboard to prevent production misuse, communicate deprecation timeline, and provide proactive migration guidance for gemini-2.0-flash-exp users.

---

## Business Context

### Problem Statement

Gemini 2.0 Flash Experimental is production-ready for audio transcription BUT experimental status creates risk:

```yaml
current_risks:
  no_warnings: "Users may deploy to production without awareness"
  breaking_changes: "Experimental model may change behavior unexpectedly"
  no_timeline: "No deprecation communication (users caught off-guard)"
  legal_exposure: "No SLA disclaimer (expectations mismatch)"

user_impact:
  production_breakage: "Breaking changes disrupt production services"
  migration_panic: "Last-minute scramble when deprecation announced"
  trust_erosion: "Users feel blindsided by changes"
```

### Success Metrics

**Primary KPI**: 100% experimental status visibility (all responses + docs)
**Risk Mitigation**: Clear deprecation timeline (Q2 2026 EOL estimate)
**User Confidence**: Migration guide available proactively
**Compliance**: Experimental SLA disclaimer in ToS

### Business Value

- **Risk mitigation**: Prevent production misuse ($0 cost vs potential litigation)
- **Trust building**: Transparent communication builds user confidence
- **Legal protection**: Clear SLA disclaimer protects company
- **User retention**: Proactive migration guidance prevents churn

---

## Acceptance Criteria

### AC1: Response Metadata (Experimental Flag)

**GIVEN** any API response from gemini-2.0-flash-exp
**WHEN** the response is returned to client
**THEN** it MUST include `experimental: true` metadata flag

**Response Schema Enhancement**:
```json
{
  "id": "chatcmpl-abc123",
  "object": "chat.completion",
  "created": 1705075200,
  "model": "gemini-2.0-flash-exp",
  "choices": [...],
  "usage": {...},
  "metadata": {
    "experimental": true,
    "experimental_warning": "This model is experimental and may be deprecated. Migration to gemini-2.5-flash recommended for production use.",
    "deprecation_timeline": "Q2 2026 (estimated)",
    "migration_guide_url": "https://docs.antigravity.tools/migration/gemini-2.0-flash-exp"
  }
}
```

**Implementation**:
```rust
// src-tauri/src/proxy/mappers/gemini/response.rs

#[derive(Debug, Clone, Serialize)]
pub struct ResponseMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_warning: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_timeline: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_guide_url: Option<String>,
}

impl GeminiResponseMapper {
    pub fn add_experimental_metadata(
        &self,
        mut response: OpenAiResponse,
        model: &str,
    ) -> OpenAiResponse {
        if model == "gemini-2.0-flash-exp" {
            response.metadata = Some(ResponseMetadata {
                experimental: Some(true),
                experimental_warning: Some(
                    "This model is experimental and may be deprecated. Migration to gemini-2.5-flash recommended for production use.".to_string()
                ),
                deprecation_timeline: Some("Q2 2026 (estimated)".to_string()),
                migration_guide_url: Some(
                    "https://docs.antigravity.tools/migration/gemini-2.0-flash-exp".to_string()
                ),
            });
        }
        response
    }
}
```

---

### AC2: Deprecation Timeline Documentation

**GIVEN** gemini-2.0-flash-exp documentation
**WHEN** users read model documentation
**THEN** deprecation timeline MUST be clearly communicated

**Documentation Structure**:
```markdown
# Gemini 2.0 Flash Experimental - Audio Transcription Specialist

⚠️ **EXPERIMENTAL MODEL - MIGRATION RECOMMENDED**

## Experimental Status

**Current Status**: ✅ Production-ready for audio transcription
**Deprecation Timeline**: Q2 2026 (estimated end-of-life)
**Migration Path**: gemini-2.5-flash (stable alternative)

### Why Experimental?

- Early access model with potential breaking changes
- Google may modify behavior without notice
- No production SLA or uptime guarantees
- Intended for testing and evaluation

### Should You Use This Model?

✅ **YES** if:
- You need 100% Whisper API compatibility NOW
- You're testing audio transcription features
- You can migrate to stable models within 6 months

❌ **NO** if:
- You're building production services
- You need long-term stability guarantees
- You require SLA-backed uptime

### Migration Timeline

**2026 Q1**: Experimental model stable, audio transcription 100% functional
**2026 Q2**: Deprecation warnings increase, migration guide published
**2026 Q3**: Model deprecated, automatic downgrade to gemini-2.5-flash
**2026 Q4**: Model removed from API

### Recommended Action

**Start migration planning NOW**: Review [migration guide](./MIGRATION-gemini-2.0-flash-exp-to-2.5-flash.md)
```

**Implementation Location**: `docs/models/gemini-2.0-flash-exp.md` (NEW)

---

### AC3: Dashboard Warning Banner

**GIVEN** user is using gemini-2.0-flash-exp via API proxy
**WHEN** viewing API Proxy or Monitor dashboard pages
**THEN** prominent warning banner MUST be displayed

**Warning Banner Design**:
```tsx
// src/pages/ApiProxy.tsx

export function ExperimentalModelWarning() {
  const config = useConfigStore((state) => state.config);

  // Check if any accounts have gemini-2.0-flash-exp in model mapping
  const isUsingExperimental = useMemo(() => {
    return config.model_mapping?.some((mapping) =>
      mapping.target_model === "gemini-2.0-flash-exp"
    );
  }, [config]);

  if (!isUsingExperimental) return null;

  return (
    <div className="alert alert-warning shadow-lg mb-4">
      <div>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          className="stroke-current flex-shrink-0 h-6 w-6"
          fill="none"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
          />
        </svg>
        <div>
          <h3 className="font-bold">Experimental Model in Use</h3>
          <div className="text-sm">
            You're using <code>gemini-2.0-flash-exp</code> which is experimental.
            Expected deprecation: <strong>Q2 2026</strong>.{" "}
            <a
              href="/docs/migration/gemini-2.0-flash-exp"
              className="link link-primary"
            >
              View migration guide
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
```

**Display Locations**:
- `/api-proxy` page (configuration view)
- `/monitor` page (request monitoring view)
- `/dashboard` page (if experimental model has activity)

---

### AC4: Usage Analytics Tracking

**GIVEN** experimental model requests
**WHEN** tracking request metrics
**THEN** flag experimental model usage for analytics

**Analytics Schema**:
```rust
// src-tauri/src/proxy/monitor.rs

#[derive(Debug, Clone, Serialize)]
pub struct RequestMetrics {
    // Existing fields...
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_experimental: Option<bool>,
}

impl RequestMonitor {
    pub fn record_request(&self, model: &str, ...) {
        let is_experimental = model.contains("-exp") || model == "gemini-2.0-flash-exp";

        // Record with experimental flag
        self.metrics.lock().unwrap().push(RequestMetrics {
            model: model.to_string(),
            is_experimental: Some(is_experimental),
            // ... other fields
        });

        // Track experimental model usage separately
        if is_experimental {
            self.experimental_usage_counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn get_experimental_usage_stats(&self) -> ExperimentalUsageStats {
        ExperimentalUsageStats {
            total_requests: self.experimental_usage_counter.load(Ordering::Relaxed),
            models: self.get_experimental_models_used(),
            warning_acknowledged: false, // Future: user acknowledgment
        }
    }
}
```

---

### AC5: Migration Guide Reference

**GIVEN** experimental model warnings
**WHEN** user sees deprecation timeline
**THEN** clear link to migration guide MUST be provided

**Migration Guide Links**:
- Response metadata: `migration_guide_url` field
- Dashboard warning: "View migration guide" link
- Documentation: Prominent link in experimental status section
- API error responses: Include migration guide URL

**Implementation**: Story 014-03 creates comprehensive migration guide

---

### AC6: Comprehensive Test Coverage

**Unit Tests** (5+ tests minimum):
```yaml
Response Metadata Tests:
  - test_experimental_flag_added_to_response
  - test_experimental_warning_message_correct
  - test_deprecation_timeline_included
  - test_migration_url_present
  - test_non_experimental_model_no_metadata

Dashboard Warning Tests:
  - test_warning_banner_shown_when_experimental_active
  - test_warning_banner_hidden_when_no_experimental
  - test_migration_link_correct_url
```

**Integration Tests** (5+ tests minimum):
```yaml
End-to-End Warning Flow:
  - test_e2e_experimental_response_includes_metadata
  - test_e2e_dashboard_shows_warning_banner
  - test_e2e_analytics_tracks_experimental_usage
  - test_e2e_documentation_shows_deprecation_timeline
  - test_e2e_migration_guide_accessible
```

---

## Implementation Details

### Module Structure

```
src-tauri/src/proxy/
├── mappers/gemini/
│   └── response.rs                  (MODIFY - add experimental metadata)
├── handlers/
│   └── audio.rs                     (MODIFY - pass experimental flag)
└── monitor.rs                       (MODIFY - track experimental usage)

src/
├── pages/
│   ├── ApiProxy.tsx                 (MODIFY - add warning banner)
│   ├── Monitor.tsx                  (MODIFY - add warning banner)
│   └── Dashboard.tsx                (MODIFY - conditional warning)
└── components/common/
    └── ExperimentalModelWarning.tsx (NEW - reusable warning component)

docs/
└── models/
    └── gemini-2.0-flash-exp.md      (NEW - experimental status documentation)

tests/
└── audio/
    └── experimental_warnings_tests.rs (NEW - 10 tests)
```

### Response Metadata Integration

```rust
// src-tauri/src/proxy/handlers/audio.rs

pub async fn handle_audio_transcription(
    file_data: Vec<u8>,
    filename: String,
    config: &ProxyConfig,
) -> Result<TranscriptionResponse, HandlerError> {
    // ... existing validation logic ...

    // Call Gemini API
    let mut response = upstream_client
        .transcribe_audio(&gemini_request)
        .await?;

    // Add experimental metadata to response
    if gemini_request.model == "gemini-2.0-flash-exp" {
        response = add_experimental_metadata(response);
    }

    Ok(response)
}

fn add_experimental_metadata(mut response: TranscriptionResponse) -> TranscriptionResponse {
    response.metadata = Some(ResponseMetadata {
        experimental: Some(true),
        experimental_warning: Some(
            "This model is experimental and may be deprecated. Migration to gemini-2.5-flash recommended for production use.".to_string()
        ),
        deprecation_timeline: Some("Q2 2026 (estimated)".to_string()),
        migration_guide_url: Some(
            "https://docs.antigravity.tools/migration/gemini-2.0-flash-exp".to_string()
        ),
    });
    response
}
```

---

## Test Strategy

### Phase 1: Unit Testing (Day 1 Morning)
**Focus**: Response metadata and warning banner logic

```bash
cargo test --package antigravity_tools_lib experimental_warnings
```

---

### Phase 2: Integration Testing (Day 1 Afternoon)
**Focus**: End-to-end warning flow

```bash
cargo test --package antigravity_tools_lib handlers::audio::experimental
npm test -- ExperimentalModelWarning
```

---

### Phase 3: UI Testing (Day 2)
**Focus**: Dashboard warning banner display

**Manual Testing**:
1. Configure model mapping with gemini-2.0-flash-exp
2. Verify warning banner appears on `/api-proxy`
3. Verify warning banner appears on `/monitor`
4. Verify migration guide link works
5. Verify warning disappears when experimental model removed

---

## Dependencies

### Internal Dependencies
- `src-tauri/src/proxy/mappers/gemini/response.rs` - STABLE (add metadata)
- `src-tauri/src/proxy/handlers/audio.rs` - STABLE (integrate metadata)
- `src-tauri/src/proxy/monitor.rs` - STABLE (track experimental usage)

### External Dependencies
- Story 014-03 (Migration Guide) - creates migration documentation

---

## Success Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Experimental visibility | 100% | All responses include metadata |
| Dashboard warning | 100% | Banner shows when experimental active |
| Documentation clarity | 95%+ user comprehension | User survey |
| Test coverage | 100% | `cargo tarpaulin` |

---

## Definition of Done

### Code Complete
- ✅ Response metadata added to all gemini-2.0-flash-exp responses
- ✅ Dashboard warning banner implemented
- ✅ Analytics tracks experimental usage
- ✅ Documentation created with deprecation timeline

### Testing Complete
- ✅ 5+ unit tests passing
- ✅ 5+ integration tests passing
- ✅ UI tests passing (manual verification)

### Quality Gates Passed
- ✅ 100% experimental visibility (all responses)
- ✅ Clear deprecation timeline (Q2 2026)
- ✅ Migration guide linked prominently

### Documentation Complete
- ✅ `docs/models/gemini-2.0-flash-exp.md` created
- ✅ Deprecation timeline documented
- ✅ Migration guide reference added

---

## Risk Assessment

**Risk 1**: User confusion (experimental status unclear)
- **Impact**: MEDIUM (user trust issues)
- **Probability**: LOW
- **Mitigation**: Clear, prominent warnings in multiple locations

**Risk 2**: Deprecation timeline inaccurate (Google changes plans)
- **Impact**: LOW (just update docs)
- **Probability**: MEDIUM
- **Mitigation**: Mark timeline as "estimated", monitor Google announcements

**Risk 3**: Warning fatigue (users ignore warnings)
- **Impact**: MEDIUM (users unprepared for deprecation)
- **Probability**: MEDIUM
- **Mitigation**: Escalate warnings as deprecation approaches

---

## Future Enhancements

- User acknowledgment system (dismiss warning after reading)
- Email notifications as deprecation approaches
- Automatic migration suggestion (switch to gemini-2.5-flash)
- A/B testing of warning message effectiveness

---

**Story Status**: ✅ READY FOR EXECUTION
**Assignee**: Dev 1B (Team 1 Mid-Level Developer)
**Start Date**: 2026-01-22 (Day 1)
**Expected Completion**: 2026-01-23 (Day 2)
