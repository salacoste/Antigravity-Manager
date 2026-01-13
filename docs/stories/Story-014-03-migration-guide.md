# Story-014-03: Migration Guide Creation

**Epic**: Epic-014 - Gemini 2.0 Flash Experimental Audio Specialist (Team 1, Gemini Specialists)
**Priority**: P2 (MEDIUM - future-proofing)
**Effort**: 2 days
**Assignee**: Dev 1A or Dev 1B (shared)
**Status**: ✅ READY FOR EXECUTION (Start: 2026-01-22)
**Created**: 2026-01-12

---

## Objective

Create comprehensive migration documentation guiding users from gemini-2.0-flash-exp to gemini-2.5-flash, including step-by-step instructions, feature comparison, breaking changes, code examples, testing checklist, and FAQ.

---

## Business Context

### Problem Statement

Users currently have NO migration guidance:

```yaml
current_gaps:
  no_documentation: "No migration path documented"
  no_comparison: "Users don't know differences between models"
  no_examples: "No before/after code samples"
  no_checklist: "No validation process for migration"

user_risks:
  surprise_breakage: "Breaking changes discovered during migration"
  data_loss: "Incorrect migration causes transcription quality issues"
  downtime: "Trial-and-error migration causes service disruption"
  abandoned_migration: "Users give up, stay on experimental model"
```

### Success Metrics

**Primary KPI**: <5% of users stuck on experimental model after Q2 2026
**User Satisfaction**: 90%+ successful migrations without support tickets
**Documentation Quality**: 95%+ users find guide helpful (survey)
**Completeness**: All breaking changes documented with workarounds

### Business Value

- **User retention**: Smooth upgrade prevents churn ($0 cost vs lost users)
- **Support efficiency**: Self-service docs reduce support load by 80%
- **Competitive advantage**: Better migration support than competitors
- **Brand trust**: Proactive documentation builds user confidence

---

## Acceptance Criteria

### AC1: Step-by-Step Migration Process

**GIVEN** user wants to migrate from gemini-2.0-flash-exp
**WHEN** reading migration guide
**THEN** clear step-by-step instructions MUST be provided

**Migration Steps**:
```markdown
# Migration Guide: gemini-2.0-flash-exp → gemini-2.5-flash

## Step 1: Assess Current Usage (15 minutes)

**1.1 Review Model Mapping Configuration**
- Open Antigravity Tools → API Proxy → Model Mapping
- Identify all mappings using `gemini-2.0-flash-exp`
- Note which endpoints/clients depend on this model

**1.2 Check Audio Transcription Volume**
- Open Monitor page → Filter by model: gemini-2.0-flash-exp
- Note: Average daily requests, peak hours, file formats used
- Estimate: Migration testing time needed (1 hour per 100 requests/day)

**1.3 Document Current Configuration**
```json
// Current model mapping (BEFORE)
{
  "source_pattern": "whisper-1",
  "target_model": "gemini-2.0-flash-exp",
  "target_provider": "gemini"
}
```

---

## Step 2: Review Breaking Changes (10 minutes)

**2.1 Thinking Budget Reduction**
- **BEFORE**: gemini-2.0-flash-exp supports 32K thinking budget
- **AFTER**: gemini-2.5-flash max 24K thinking budget
- **Action**: If using thinking mode >24K, adjust budget to 24000

**2.2 Audio Transcription (NO BREAKING CHANGES)**
- ✅ All audio formats still supported (mp3, wav, m4a, ogg, flac, aiff)
- ✅ 15MB file size limit unchanged
- ✅ Base64 encoding unchanged
- ✅ Response format unchanged

**2.3 Web Search (NEW FEATURE)**
- ✅ gemini-2.5-flash has native web search (gemini-2.0-flash-exp doesn't)
- ✅ No action needed (automatic enhancement)

---

## Step 3: Update Configuration (5 minutes)

**3.1 Update Model Mapping**
```json
// Updated model mapping (AFTER)
{
  "source_pattern": "whisper-1",
  "target_model": "gemini-2.5-flash",  // ← CHANGED
  "target_provider": "gemini"
}
```

**3.2 Update Thinking Budget (if applicable)**
```json
// BEFORE (gemini-2.0-flash-exp)
{
  "thinking": {
    "budget": 32000  // Max: 32K
  }
}

// AFTER (gemini-2.5-flash)
{
  "thinking": {
    "budget": 24000  // Max: 24K (reduced)
  }
}
```

**3.3 Save Configuration**
- Click "Save Configuration" in API Proxy page
- Restart proxy service (or wait for auto-reload)

---

## Step 4: Test Migration (30 minutes)

**4.1 Audio Transcription Quality Test**
```bash
# Test with sample audio file (15 seconds, clear speech)
curl -X POST http://localhost:8045/v1/audio/transcriptions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -F file="@test_audio.mp3" \
  -F model="whisper-1"

# Expected: Same transcription quality as gemini-2.0-flash-exp
```

**4.2 Thinking Mode Test (if applicable)**
```bash
# Test with thinking budget
curl -X POST http://localhost:8045/v1/chat/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemini-2.5-flash",
    "messages": [{"role": "user", "content": "Analyze market trends..."}],
    "thinking": {"budget": 24000}
  }'

# Expected: Thinking blocks in response (budget ≤24K)
```

**4.3 Multi-Format Audio Test**
```yaml
Test Files:
  - test_mp3.mp3 (30s, music + speech)
  - test_wav.wav (15s, clean speech)
  - test_m4a.m4a (45s, podcast excerpt)

Expected Results:
  - All formats transcribe successfully
  - Quality equivalent to gemini-2.0-flash-exp
  - No errors or warnings
```

---

## Step 5: Monitor Production (7 days)

**5.1 Monitor Error Rates**
- Open Monitor page → Filter by model: gemini-2.5-flash
- Check: Error rate <0.5% (same as baseline)
- Alert: If error rate >1%, investigate immediately

**5.2 Monitor Transcription Quality**
- Sample 20 transcriptions per day (random selection)
- Compare: gemini-2.0-flash-exp vs gemini-2.5-flash
- Threshold: <2% quality degradation (subjective)

**5.3 Monitor Performance**
- Check: Response times (P50, P95, P99)
- Expected: Similar to gemini-2.0-flash-exp (±10%)
- Alert: If P95 >2x baseline, investigate

---

## Step 6: Complete Migration (Day 8)

**6.1 Remove Experimental Model Mapping**
- Delete all model mappings using gemini-2.0-flash-exp
- Update documentation to reflect gemini-2.5-flash

**6.2 Update Client Configurations (if any)**
- Notify downstream clients of model change
- Update client-side model references (if hardcoded)

**6.3 Celebrate! 🎉**
- Migration complete
- Production-stable model in use
- No more experimental warnings
```

---

### AC2: Feature Comparison Table

**GIVEN** user evaluating migration
**WHEN** comparing gemini-2.0-flash-exp vs gemini-2.5-flash
**THEN** comprehensive feature comparison MUST be provided

**Comparison Table**:
```markdown
## Feature Comparison: gemini-2.0-flash-exp vs gemini-2.5-flash

| Feature | gemini-2.0-flash-exp | gemini-2.5-flash | Migration Impact |
|---------|---------------------|-----------------|------------------|
| **Audio Transcription** | ✅ 100% Whisper compatibility | ✅ 100% Whisper compatibility | ✅ NO CHANGE |
| Audio Formats | mp3, wav, m4a, ogg, flac, aiff | mp3, wav, m4a, ogg, flac, aiff | ✅ NO CHANGE |
| File Size Limit | 15MB | 15MB | ✅ NO CHANGE |
| Base64 Encoding | ✅ Supported | ✅ Supported | ✅ NO CHANGE |
| Language Detection | ✅ Automatic | ✅ Automatic | ✅ NO CHANGE |
| **Thinking Mode** | ✅ 1-32K budget | ✅ 1-24K budget | ⚠️ BREAKING CHANGE |
| Max Thinking Budget | 32,000 tokens | 24,000 tokens | ⚠️ Reduce budget if >24K |
| Thinking Block Streaming | ✅ Supported | ✅ Supported | ✅ NO CHANGE |
| Thinking Exclusion Billing | ✅ Excluded | ✅ Excluded | ✅ NO CHANGE |
| **Web Search** | ❌ NOT SUPPORTED | ✅ SUPPORTED | ✅ NEW FEATURE |
| Google Search Tool | ❌ Not available | ✅ Available | ✅ Upgrade benefit |
| Grounding Search | ❌ Not available | ✅ Available | ✅ Upgrade benefit |
| **Vision** | ❌ NOT SUPPORTED | ✅ SUPPORTED | ✅ NEW FEATURE |
| Image Analysis | ❌ Not available | ✅ Available | ✅ Upgrade benefit |
| Multi-Modal | ❌ Audio only | ✅ Audio + Image + Text | ✅ Upgrade benefit |
| **Stability** | ⚠️ EXPERIMENTAL | ✅ STABLE | ✅ Production-grade |
| SLA Guarantee | ❌ Best-effort | ✅ Production SLA | ✅ Upgrade benefit |
| Breaking Changes Risk | ⚠️ HIGH | ✅ LOW | ✅ Reduced risk |
| Deprecation Risk | ⚠️ Q2 2026 | ✅ Long-term stable | ✅ Future-proof |
| **Pricing** | $0.075/1M input | $0.075/1M input | ✅ NO CHANGE |
| Input Tokens | $0.075/1M | $0.075/1M | ✅ NO CHANGE |
| Output Tokens | $0.30/1M | $0.30/1M | ✅ NO CHANGE |
| Thinking Tokens | $0.35/1M | $0.35/1M | ✅ NO CHANGE |

### Summary

**Audio Transcription Users**: ✅ NO BREAKING CHANGES (seamless migration)
**Thinking Mode Users**: ⚠️ Reduce budget from 32K → 24K if applicable
**Web Search/Vision Users**: ✅ NEW FEATURES (upgrade benefit)
**Production Users**: ✅ STABILITY UPGRADE (experimental → stable)
```

---

### AC3: Breaking Changes Documentation

**GIVEN** user migrating to gemini-2.5-flash
**WHEN** encountering breaking changes
**THEN** clear documentation with workarounds MUST be provided

**Breaking Changes**:
```markdown
## Breaking Changes & Workarounds

### 🚨 Breaking Change #1: Thinking Budget Reduction (32K → 24K)

**Impact**: If you're using thinking budgets >24K, requests will fail

**Detection**:
```json
// Error response
{
  "error": {
    "message": "Thinking budget 32000 exceeds maximum 24000 for gemini-2.5-flash",
    "type": "invalid_request_error",
    "code": "thinking_budget_exceeded"
  }
}
```

**Workaround**:
```json
// BEFORE (gemini-2.0-flash-exp)
{
  "thinking": {"budget": 32000}  // ❌ Too high for gemini-2.5-flash
}

// AFTER (gemini-2.5-flash)
{
  "thinking": {"budget": 24000}  // ✅ Max allowed
}
```

**Quality Impact**: Minimal (<2% for most use cases)
- 24K budget sufficient for 95% of reasoning tasks
- For ultra-complex tasks, use gemini-2.5-pro (32K budget)

---

### ✅ Non-Breaking Changes (Safe Migrations)

**Audio Transcription**: 100% backward compatible
- All formats work identically
- No code changes needed
- No quality degradation

**Web Search**: NEW feature (automatic enhancement)
- gemini-2.5-flash has web search (gemini-2.0-flash-exp doesn't)
- No action needed (works automatically if requested)

**Vision**: NEW feature (automatic enhancement)
- gemini-2.5-flash supports images (gemini-2.0-flash-exp doesn't)
- No action needed (works automatically if images provided)
```

---

### AC4: Code Examples (Before/After)

**GIVEN** user implementing migration
**WHEN** writing code
**THEN** before/after code examples MUST be provided

**Code Examples**:
```markdown
## Code Migration Examples

### Example 1: Audio Transcription (No Changes Needed)

**BEFORE (gemini-2.0-flash-exp)**:
```python
import openai

client = openai.OpenAI(
    base_url="http://localhost:8045/v1",
    api_key="your_api_key"
)

# Audio transcription
with open("audio.mp3", "rb") as audio_file:
    transcript = client.audio.transcriptions.create(
        model="whisper-1",  # Maps to gemini-2.0-flash-exp
        file=audio_file
    )
print(transcript.text)
```

**AFTER (gemini-2.5-flash)**:
```python
import openai

client = openai.OpenAI(
    base_url="http://localhost:8045/v1",
    api_key="your_api_key"
)

# Audio transcription (IDENTICAL CODE)
with open("audio.mp3", "rb") as audio_file:
    transcript = client.audio.transcriptions.create(
        model="whisper-1",  # Now maps to gemini-2.5-flash
        file=audio_file
    )
print(transcript.text)  # ✅ Same output quality
```

---

### Example 2: Thinking Mode (Budget Adjustment Required)

**BEFORE (gemini-2.0-flash-exp)**:
```typescript
import OpenAI from "openai";

const client = new OpenAI({
  baseURL: "http://localhost:8045/v1",
  apiKey: process.env.OPENAI_API_KEY,
});

const response = await client.chat.completions.create({
  model: "gemini-2.0-flash-exp",
  messages: [
    { role: "user", content: "Design a distributed caching architecture..." }
  ],
  thinking: {
    budget: 32000  // ❌ Exceeds gemini-2.5-flash max
  }
});
```

**AFTER (gemini-2.5-flash)**:
```typescript
import OpenAI from "openai";

const client = new OpenAI({
  baseURL: "http://localhost:8045/v1",
  apiKey: process.env.OPENAI_API_KEY,
});

const response = await client.chat.completions.create({
  model: "gemini-2.5-flash",  // ✅ Updated model
  messages: [
    { role: "user", content: "Design a distributed caching architecture..." }
  ],
  thinking: {
    budget: 24000  // ✅ Adjusted to max allowed
  }
});
```
```

---

### AC5: Testing Checklist

**GIVEN** user completed migration
**WHEN** validating migration success
**THEN** comprehensive testing checklist MUST be provided

**Testing Checklist**:
```markdown
## Migration Validation Checklist

### Audio Transcription Tests

- [ ] **Test 1: MP3 Format** (30s, music + speech)
  - [ ] Upload test_mp3.mp3 via Whisper API
  - [ ] Verify transcription accuracy (compare with gemini-2.0-flash-exp baseline)
  - [ ] Check response time <3 seconds

- [ ] **Test 2: WAV Format** (15s, clean speech)
  - [ ] Upload test_wav.wav via Whisper API
  - [ ] Verify transcription accuracy
  - [ ] Check response time <2 seconds

- [ ] **Test 3: M4A Format** (45s, podcast excerpt)
  - [ ] Upload test_m4a.m4a via Whisper API
  - [ ] Verify transcription accuracy
  - [ ] Check response time <4 seconds

- [ ] **Test 4: Long Audio** (10 minutes, conference call)
  - [ ] Upload long_audio.mp3 via Whisper API
  - [ ] Verify transcription accuracy (spot-check 5 random segments)
  - [ ] Check no timeout errors

- [ ] **Test 5: Multi-Language** (30s, Spanish)
  - [ ] Upload spanish_audio.mp3 via Whisper API
  - [ ] Verify language detection automatic
  - [ ] Verify transcription accuracy (native speaker validation)

### Thinking Mode Tests (if applicable)

- [ ] **Test 6: Thinking Budget ≤24K**
  - [ ] Send request with thinking.budget = 24000
  - [ ] Verify thinking blocks in response
  - [ ] Check no errors or warnings

- [ ] **Test 7: Thinking Budget >24K (Error Handling)**
  - [ ] Send request with thinking.budget = 32000
  - [ ] Verify error response: "thinking_budget_exceeded"
  - [ ] Retry with thinking.budget = 24000 (should succeed)

### Production Monitoring (7 days)

- [ ] **Day 1-7: Error Rate Monitoring**
  - [ ] Check error rate <0.5% daily
  - [ ] Alert if error rate >1%

- [ ] **Day 1-7: Quality Monitoring**
  - [ ] Sample 20 transcriptions per day
  - [ ] Compare quality with gemini-2.0-flash-exp baseline
  - [ ] Threshold: <2% quality degradation

- [ ] **Day 1-7: Performance Monitoring**
  - [ ] Check response times (P50, P95, P99)
  - [ ] Expected: ±10% vs gemini-2.0-flash-exp baseline
  - [ ] Alert if P95 >2x baseline
```

---

### AC6: FAQ

**GIVEN** user has migration questions
**WHEN** consulting migration guide
**THEN** comprehensive FAQ MUST be provided

**FAQ**:
```markdown
## Frequently Asked Questions

**Q1: Will audio transcription quality degrade after migration?**
A1: No. gemini-2.5-flash has identical audio transcription quality to gemini-2.0-flash-exp. Both models use the same underlying audio processing pipeline. Expected quality: 100% equivalent.

**Q2: Do I need to change my client code?**
A2: For audio transcription only: NO code changes needed. For thinking mode >24K budget: YES, reduce budget to 24000.

**Q3: What happens if I forget to reduce thinking budget?**
A3: You'll receive error: "thinking_budget_exceeded". Simply retry with budget ≤24000.

**Q4: Can I migrate gradually (some requests to exp, some to 2.5-flash)?**
A4: Yes. Use multiple model mappings or route based on client identifier. Test gemini-2.5-flash in parallel before full migration.

**Q5: Will my API key still work?**
A5: Yes. No authentication changes. Same API key, same endpoints.

**Q6: How long does migration take?**
A6: Typical timeline: 1 hour (testing) + 7 days (monitoring) = 8 days total.

**Q7: What if I encounter issues during migration?**
A7: Revert immediately to gemini-2.0-flash-exp (change model mapping back). Open support ticket with error details.

**Q8: When should I migrate?**
A8: Recommended: Q1 2026 (before Q2 2026 deprecation). Latest: Q2 2026 (before model removed).

**Q9: What's the rollback process if migration fails?**
A9: Change model mapping back to gemini-2.0-flash-exp → Save configuration → Restart proxy. Rollback time: <5 minutes.

**Q10: Does gemini-2.5-flash support all the same audio formats?**
A10: Yes. MP3, WAV, M4A, OGG, FLAC, AIFF all supported identically.
```

---

## Implementation Details

### Documentation Structure

```
docs/
└── guides/
    └── MIGRATION-gemini-2.0-flash-exp-to-2.5-flash.md  (NEW - comprehensive guide)
        ├── Step-by-Step Migration Process (6 steps)
        ├── Feature Comparison Table
        ├── Breaking Changes Documentation
        ├── Code Examples (Before/After)
        ├── Testing Checklist
        └── FAQ
```

### Documentation Deliverable

**File**: `docs/guides/MIGRATION-gemini-2.0-flash-exp-to-2.5-flash.md`
**Length**: ~5000 words
**Sections**: 6 main sections + FAQ
**Code Examples**: 4+ before/after examples
**Test Cases**: 10+ validation tests

---

## Test Strategy

### Phase 1: Documentation Review (Day 1)
**Focus**: Content accuracy and completeness

**Review Checklist**:
- [ ] All breaking changes documented
- [ ] Code examples tested and working
- [ ] Testing checklist comprehensive
- [ ] FAQ covers common questions
- [ ] Links to other documentation correct

---

### Phase 2: User Testing (Day 2)
**Focus**: Documentation usability

**Test with 3 users**:
- Dev 1C (internal QA - first-time migrator)
- External beta tester (audio transcription heavy user)
- External beta tester (thinking mode user)

**Feedback Collection**:
- Time to complete migration
- Sections most/least helpful
- Missing information
- Confusing instructions

---

## Dependencies

### Internal Dependencies
- Story 014-02 (Experimental Warnings) - provides migration guide URL
- Existing documentation structure (`docs/guides/`)

### External Dependencies
- None (documentation only, no code changes)

---

## Success Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Successful migrations | 95%+ | User survey post-migration |
| Support tickets | <5% of migrations | Support ticket tracking |
| Documentation helpfulness | 90%+ rating | User survey |
| Time to migrate | <1 hour (excluding monitoring) | User timing |

---

## Definition of Done

### Documentation Complete
- ✅ Migration guide created (5000+ words)
- ✅ All 6 main sections complete
- ✅ 4+ code examples tested
- ✅ 10+ testing checklist items
- ✅ FAQ with 10+ questions

### Review Complete
- ✅ Technical review by Dev 1A (accuracy)
- ✅ User testing with 3 testers (usability)
- ✅ Links verified and working

### Integration Complete
- ✅ Linked from Story 014-02 (experimental warnings)
- ✅ Linked from model documentation
- ✅ Linked from dashboard warning banner

---

## Risk Assessment

**Risk 1**: Documentation incomplete (missing edge cases)
- **Impact**: MEDIUM (user confusion)
- **Probability**: LOW
- **Mitigation**: User testing reveals gaps before release

**Risk 2**: Code examples outdated
- **Impact**: HIGH (users copy broken code)
- **Probability**: LOW
- **Mitigation**: Test all examples before publishing

**Risk 3**: Google changes gemini-2.5-flash features
- **Impact**: MEDIUM (docs become inaccurate)
- **Probability**: MEDIUM
- **Mitigation**: Regular documentation reviews, update as needed

---

## Future Enhancements

- Video walkthrough (screen recording of migration)
- Interactive migration wizard (web-based tool)
- Automated migration script (one-click migration)
- Migration validation API (test endpoint)

---

**Story Status**: ✅ READY FOR EXECUTION
**Assignee**: Dev 1A or Dev 1B (shared)
**Start Date**: 2026-01-22 (Days 1-2 in parallel with other stories)
**Expected Completion**: 2026-01-23 (Day 2)
