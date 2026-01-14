# Model ID [XXX] Discovery Template

**Model ID**: [Internal ID number, e.g., 314]
**Discovery Date**: [YYYY-MM-DD]
**Researcher**: [Name]
**Confidence Level**: [Low/Medium/High/Confirmed]

---

## Model Identification

### Basic Information

**Model Name**: [e.g., gemini-2.0-flash-thinking-exp-01-21]
**Full API Path**: [e.g., models/gemini-2.0-flash-thinking-exp-01-21]
**Model Family**: [e.g., Gemini 2.0 Flash, Gemini 1.5 Pro (⚫ DEPRECATED)]
**Status**: [Active/Deprecated/Experimental/Unknown]
**Note**: ⚫ Gemini 1.5 family is DEPRECATED as of 2026-01-14

### Identification Sources

**Code References**:
```rust
// File: src-tauri/src/proxy/mappers/gemini/model_mapping.rs
// Line: XXX
// Context: [Paste relevant code snippet]
```

**API Documentation**:
- Google AI Studio: [Link or "Not found"]
- Vertex AI Docs: [Link or "Not found"]
- Release Notes: [Link or "Not found"]

**Community References**:
- GitHub Issues: [Links to relevant issues]
- Forum Discussions: [Links to discussions]
- Stack Overflow: [Links to questions]

### Confidence Assessment

**Confidence Level**: [Low/Medium/High/Confirmed]

**Evidence Supporting Identification**:
- [ ] Found in official Google documentation
- [ ] Code reference in model_mapping.rs
- [ ] API returns valid responses (not 404)
- [ ] Model name follows known naming convention
- [ ] Community confirmation (GitHub/forums)

**Uncertainties**:
- [List any unclear aspects or contradictory information]

---

## API Endpoint Testing

### Test Setup

**Test Date**: [YYYY-MM-DD]
**Test Environment**: [Production/Staging]
**API Version**: [e.g., v1beta]
**Test Account**: [Account tier: Free/Pro/Ultra]

### Basic Connectivity Test

**Endpoint**: `POST https://generativelanguage.googleapis.com/v1beta/models/[MODEL_NAME]:generateContent`

**Request**:
```json
{
  "contents": [{
    "parts": [{"text": "Hello, world! Please respond with a brief greeting."}]
  }]
}
```

**Response Status**: [200 OK / 404 Not Found / 403 Forbidden / Other]

**Response Sample**:
```json
{
  "candidates": [{
    "content": {
      "parts": [{"text": "[Model response here]"}]
    }
  }]
}
```

**Response Time**: [XXX ms]

**Test Result**: ✅ Success / ❌ Failure

**Notes**: [Any unusual behavior or errors]

---

## Feature Detection

### Standard Text Generation

**Test Prompt**: "Write a 50-word summary of artificial intelligence."

**Result**: ✅ Supported / ❌ Not Supported / ⚠️ Partial

**Quality Assessment**: [Excellent/Good/Fair/Poor]

**Response Time**: [XXX ms]

**Notes**: [Any quality or performance observations]

---

### System Instructions Support

**Test Request**:
```json
{
  "system_instruction": {
    "parts": [{"text": "You are a helpful assistant. Always respond in exactly 3 sentences."}]
  },
  "contents": [{
    "parts": [{"text": "What is the capital of France?"}]
  }]
}
```

**Result**: ✅ Supported / ❌ Not Supported / ⚠️ Partial

**Compliance**: [Did model follow system instructions? Yes/No/Partially]

**Notes**: [Observations about system instruction handling]

---

### Tool/Function Calling

**Test Request**:
```json
{
  "tools": [{
    "function_declarations": [{
      "name": "get_weather",
      "description": "Get current weather for a location",
      "parameters": {
        "type": "object",
        "properties": {
          "location": {"type": "string"}
        }
      }
    }]
  }],
  "contents": [{
    "parts": [{"text": "What's the weather in Tokyo?"}]
  }]
}
```

**Result**: ✅ Supported / ❌ Not Supported / ⚠️ Partial

**Tool Call Generated**: [Yes/No]

**Tool Call Format**: [Correct/Incorrect]

**Notes**: [Tool calling behavior observations]

---

### Vision Capabilities

**Test Request** (with image):
```json
{
  "contents": [{
    "parts": [
      {"text": "Describe this image in detail."},
      {
        "inline_data": {
          "mime_type": "image/jpeg",
          "data": "[base64_encoded_image]"
        }
      }
    ]
  }]
}
```

**Result**: ✅ Supported / ❌ Not Supported / ⚠️ Partial

**Image Analysis Quality**: [Excellent/Good/Fair/Poor/N/A]

**Supported Formats**: [JPEG/PNG/WebP/GIF/Unknown]

**Notes**: [Vision capability observations]

---

### Thinking Mode Support

**Test Request**:
```json
{
  "contents": [{
    "parts": [{"text": "Solve this logic puzzle step by step: If all roses are flowers and some flowers fade quickly, what can we conclude?"}]
  }],
  "generationConfig": {
    "response_modalities": ["TEXT"],
    "thinking_mode": "enabled"
  }
}
```

**Result**: ✅ Supported / ❌ Not Supported / ⚠️ Partial

**Thinking Process Visible**: [Yes/No]

**Response Quality**: [Excellent/Good/Fair/Poor]

**Notes**: [Thinking mode behavior observations]

---

### Audio Input Support

**Test Request** (with audio):
```json
{
  "contents": [{
    "parts": [
      {"text": "Transcribe this audio."},
      {
        "inline_data": {
          "mime_type": "audio/wav",
          "data": "[base64_encoded_audio]"
        }
      }
    ]
  }]
}
```

**Result**: ✅ Supported / ❌ Not Supported / ⚠️ Partial

**Transcription Quality**: [Excellent/Good/Fair/Poor/N/A]

**Supported Formats**: [WAV/MP3/FLAC/Unknown]

**Notes**: [Audio input observations]

---

### Audio Output Support

**Test Request**:
```json
{
  "contents": [{
    "parts": [{"text": "Say 'Hello, world!' in a friendly tone."}]
  }],
  "generationConfig": {
    "response_modalities": ["AUDIO"]
  }
}
```

**Result**: ✅ Supported / ❌ Not Supported / ⚠️ Partial

**Audio Quality**: [Excellent/Good/Fair/Poor/N/A]

**Voice Options**: [Available/Not Available/Unknown]

**Notes**: [Audio output observations]

---

## Advanced Testing

### Rate Limiting Behavior

**Test Method**: Send 20 rapid requests

**Rate Limit Encountered**: [Yes/No]
**Limit Threshold**: [XXX requests per minute]
**Error Code**: [429/Other]
**Retry-After Header**: [XXX seconds / Not provided]

**Notes**: [Rate limit behavior observations]

---

### Error Response Analysis

**Test Scenarios**:
1. **Invalid API Key**: [Error response]
2. **Malformed Request**: [Error response]
3. **Exceeded Token Limit**: [Error response]
4. **Unsupported Parameter**: [Error response]

**Error Handling Quality**: [Excellent/Good/Fair/Poor]

**Notes**: [Error message quality and helpfulness]

---

### Performance Benchmarking

**Test Setup**: 10 requests with 500-token prompts

| Metric | Value | Unit |
|--------|-------|------|
| Mean Response Time | XXX | ms |
| Median (p50) | XXX | ms |
| 95th Percentile (p95) | XXX | ms |
| 99th Percentile (p99) | XXX | ms |
| Tokens per Second | XXX | tokens/s |
| First Token Latency | XXX | ms |

**Comparison with Similar Models**:
- [Model A]: [Faster/Slower/Similar]
- [Model B]: [Faster/Slower/Similar]

**Notes**: [Performance observations]

---

## Code Reference Analysis

### Model Mapping Analysis

**File**: `src-tauri/src/proxy/mappers/gemini/model_mapping.rs`

**Code Snippet**:
```rust
// Paste relevant code showing model ID usage
```

**Mapping Logic**: [Describe how this model ID is currently handled]

**Feature Flags**: [List any feature flags associated with this model]

---

### Request Mapper Analysis

**File**: `src-tauri/src/proxy/mappers/gemini/request.rs`

**Special Handling**: [Yes/No]

**Details**: [Describe any model-specific request mapping]

---

### Response Mapper Analysis

**File**: `src-tauri/src/proxy/mappers/gemini/response.rs`

**Special Handling**: [Yes/No]

**Details**: [Describe any model-specific response mapping]

---

## Capability Matrix Summary

| Feature | Supported | Quality | Performance | Notes |
|---------|-----------|---------|-------------|-------|
| Text Generation | ✅/❌ | Excellent/Good/Fair/Poor | Fast/Medium/Slow | ... |
| System Instructions | ✅/❌ | Excellent/Good/Fair/Poor | Fast/Medium/Slow | ... |
| Tool Calling | ✅/❌ | Excellent/Good/Fair/Poor | Fast/Medium/Slow | ... |
| Vision Input | ✅/❌ | Excellent/Good/Fair/Poor | Fast/Medium/Slow | ... |
| Thinking Mode | ✅/❌ | Excellent/Good/Fair/Poor | Fast/Medium/Slow | ... |
| Audio Input | ✅/❌ | Excellent/Good/Fair/Poor | Fast/Medium/Slow | ... |
| Audio Output | ✅/❌ | Excellent/Good/Fair/Poor | Fast/Medium/Slow | ... |

**Overall Assessment**: [Excellent/Good/Fair/Poor]

---

## Comparison with Existing Models

### Similar Models

**Most Similar Model**: [e.g., gemini-1.5-pro-002 (⚫ DEPRECATED)]
**Note**: If comparing to Gemini 1.5 models, note they are DEPRECATED as of 2026-01-14

**Key Differences**:
1. [Difference 1]
2. [Difference 2]
3. [Difference 3]

**Advantages Over Similar Models**:
- [Advantage 1]
- [Advantage 2]

**Disadvantages vs. Similar Models**:
- [Disadvantage 1]
- [Disadvantage 2]

---

### Unique Features

**Features Not Found in Other Models**:
1. [Unique feature 1]
2. [Unique feature 2]
3. [Unique feature 3]

**Business Value of Unique Features**: [High/Medium/Low]

**Justification**: [Explain why these features are valuable]

---

## COMPARISON File Recommendation

### Should COMPARISON File Be Created?

**Recommendation**: ✅ Yes / ❌ No / ⚠️ Maybe

**Justification**:
- [Reason 1]
- [Reason 2]
- [Reason 3]

**Decision Criteria**:
- [ ] Model has unique capabilities vs. existing models
- [ ] Model is actively supported (not deprecated)
- [ ] Model differs significantly from aliases
- [ ] Documentation adds value for users
- [ ] User demand exists for this model

---

### Draft COMPARISON File Content

**If COMPARISON file is recommended, draft key sections below:**

#### Use Cases
[List 3-5 primary use cases for this model]

#### Limitations
[List 3-5 key limitations users should know]

#### Performance Characteristics
[Describe speed, cost, quality trade-offs]

#### Best Practices
[List 3-5 best practices for using this model]

---

## Implementation Recommendations

### Scenario Classification

**Scenario**: [A: Vertex Variant / B: New Features / C: Deprecated / D: Mixed]

**Confidence**: [Low/Medium/High]

**Justification**: [Explain classification]

---

### Implementation Effort Estimate

**Estimated Effort**: [Hours/Days]

**Breakdown**:
- Model mapping changes: [X hours]
- Request mapper updates: [X hours]
- Response mapper updates: [X hours]
- Feature detection: [X hours]
- Testing: [X hours]
- Documentation: [X hours]

**Total**: [XX hours / X days]

---

### Implementation Priority

**Priority**: [P0: Critical / P1: High / P2: Medium / P3: Low]

**Priority Factors**:
| Factor | Score (1-5) | Weight | Notes |
|--------|-------------|--------|-------|
| User Demand | X | 40% | ... |
| Feature Uniqueness | X | 30% | ... |
| Performance Advantage | X | 20% | ... |
| Documentation Value | X | 10% | ... |
| **Weighted Score** | **X.X** | **100%** | ... |

**Priority Threshold**:
- P0: Score ≥4.5 (implement immediately)
- P1: Score 3.5-4.4 (implement Q2)
- P2: Score 2.5-3.4 (implement Q3)
- P3: Score <2.5 (backlog/document only)

---

### Implementation Stories

**If implementation is recommended, draft stories:**

#### Story 1: Model Mapping
- **Title**: Add model [XXX] to model_mapping.rs
- **Effort**: [X days]
- **Tasks**:
  - [ ] Add model enum entry
  - [ ] Add normalization logic
  - [ ] Add feature flags
  - [ ] Update tests

#### Story 2: Feature Detection
- **Title**: Implement feature detection for model [XXX]
- **Effort**: [X days]
- **Tasks**:
  - [ ] Detect thinking mode support
  - [ ] Detect vision capabilities
  - [ ] Detect audio support
  - [ ] Update feature detection logic

#### Story 3: Documentation
- **Title**: Create COMPARISON file for model [XXX]
- **Effort**: [X days]
- **Tasks**:
  - [ ] Write use cases
  - [ ] Document limitations
  - [ ] Add performance benchmarks
  - [ ] Cross-reference with MASTER-MODELS-TABLE.md

---

## Research Notes

### Observations

[Free-form notes about interesting findings, unexpected behavior, etc.]

---

### Questions for Follow-up

1. [Question 1]
2. [Question 2]
3. [Question 3]

---

### Blockers

[List any blockers encountered during research]

---

### Next Steps

- [ ] [Next action 1]
- [ ] [Next action 2]
- [ ] [Next action 3]

---

## Appendix

### Test Scripts

**Test Script Location**: [Path to test script]

**Test Script**:
```bash
# Paste test script used for validation
```

---

### Raw API Responses

**Sample Request/Response 1**:
```json
{
  "request": {...},
  "response": {...}
}
```

**Sample Request/Response 2**:
```json
{
  "request": {...},
  "response": {...}
}
```

---

### Related Documentation

- Google AI Studio: [Link]
- Vertex AI Docs: [Link]
- Community Discussion: [Link]
- GitHub Issue: [Link]

---

**Template Version**: 1.0
**Last Updated**: 2026-01-12
**Research Status**: [Not Started / In Progress / Complete]
