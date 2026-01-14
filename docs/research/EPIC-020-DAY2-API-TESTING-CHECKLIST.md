# Epic-020 Day 2 API Testing Checklist

**Document Purpose**: Comprehensive API testing plan for Model IDs 314-327
**Date**: 2026-01-13 (Day 1 Afternoon Preparation)
**Team**: Epic-020 Research (Dev A, Dev B, Dev C)
**Status**: Ready for Execution - Day 2 Morning
**Prepared By**: Dev C (Junior Developer)

---

## Executive Summary

After exhaustive Day 1 codebase and log analysis confirming that Model IDs 314-327 are NOT present in current code or production logs, Day 2 focuses on external API validation to determine whether these models are:
1. **Deprecated** (70% probability) - No longer available via Vertex AI/AI Studio
2. **Reserved** (25% probability) - Future/experimental models not yet released
3. **External-only** (5% probability) - Available via API but not integrated in Antigravity

This checklist provides a complete testing framework for systematic API validation.

---

## Pre-Testing Setup (Morning - 30 min)

### Tools & Environment Setup
- [ ] **Google Cloud SDK**: Verify `gcloud` CLI is installed and configured
  ```bash
  gcloud --version
  gcloud auth list  # Verify authentication
  ```
- [ ] **curl**: Verify curl is available
  ```bash
  curl --version
  ```
- [ ] **jq**: Optional but recommended for JSON parsing
  ```bash
  brew install jq  # macOS
  ```
- [ ] **API Keys Ready**: Ensure Vertex AI and AI Studio API keys are accessible
  - Vertex AI API key or Service Account JSON
  - Google AI Studio API key (for Gemini API)

### Test Environment Validation
- [ ] Confirm internet connectivity: `ping 8.8.8.8`
- [ ] Verify Google Cloud project is set: `gcloud config get-value project`
- [ ] Confirm Vertex AI API is enabled in Google Cloud Console
- [ ] Confirm generative-ai API is enabled for AI Studio access
- [ ] Create test log directory: `mkdir -p logs/day2-api-testing`

---

## Model ID Testing Matrix

### Models to Test: 314-327 (14 total)

| Model ID | Test Vertex AI | Test AI Studio | Priority | Status |
|----------|----------------|----------------|----------|--------|
| 314 | ✓ | ✓ | HIGH | Pending |
| 315 | ✓ | ✓ | HIGH | Pending |
| 316 | ✓ | ✓ | HIGH | Pending |
| 317 | ✓ | ✓ | HIGH | Pending |
| 318 | ✓ | ✓ | HIGH | Pending |
| 319 | ✓ | ✓ | HIGH | Pending |
| 320 | ✓ | ✓ | HIGH | Pending |
| 321 | ✓ | ✓ | HIGH | Pending |
| 322 | ✓ | ✓ | HIGH | Pending |
| 323 | ✓ | ✓ | HIGH | Pending |
| 324 | ✓ | ✓ | HIGH | Pending |
| 325 | ✓ | ✓ | HIGH | Pending |
| 326 | ✓ | ✓ | HIGH | Pending |
| 327 | ✓ | ✓ | HIGH | Pending |

---

## API Endpoint Testing Plan

### Test Strategy 1: Vertex AI Models Endpoint

**Purpose**: Determine if models exist in Google Cloud Vertex AI system

**Endpoint**: `https://us-central1-aiplatform.googleapis.com/v1/projects/{PROJECT_ID}/locations/us-central1/models`

**Testing Steps**:
1. [ ] List all available models in Vertex AI
2. [ ] Search response for model ID references (314-327)
3. [ ] Document available model names and IDs
4. [ ] Look for any Gemini 3.x variants

**Expected Responses**:
- **200 OK**: List of models returned (check if 314-327 present)
- **403 Forbidden**: API not enabled or insufficient permissions
- **404 Not Found**: Project or endpoint doesn't exist

**Documentation to Capture**:
```json
{
  "model_id": 314,
  "api_endpoint": "vertex_ai_list_models",
  "response_code": 200,
  "found_in_list": true/false,
  "model_name": "string or null",
  "status": "ACTIVE|DEPRECATED|UNKNOWN",
  "error_message": "string or null"
}
```

---

### Test Strategy 2: Direct Model Endpoint Testing

**Purpose**: Test if each model ID can be accessed directly via API

**Endpoint Pattern**: `https://us-central1-aiplatform.googleapis.com/v1/projects/{PROJECT_ID}/locations/us-central1/publishers/google/models/{MODEL_ID}`

**Testing Steps** (for each model 314-327):
1. [ ] Construct endpoint with model ID
2. [ ] Send GET request to model endpoint
3. [ ] Capture HTTP response code and headers
4. [ ] If successful (200), extract model metadata
5. [ ] If failed (4xx/5xx), capture error message

**Expected Responses**:
- **200 OK**: Model exists and is accessible
  - Extract: model capabilities, supported tasks, pricing info
- **404 Not Found**: Model doesn't exist (likely deprecated or not released)
- **403 Forbidden**: Model exists but access denied
- **400 Bad Request**: Invalid model ID format

**Documentation to Capture**:
```json
{
  "model_id": 314,
  "endpoint": "vertex_ai_direct_model",
  "response_code": 404,
  "response_time_ms": 245,
  "status": "NOT_FOUND",
  "error_message": "The model `314` was not found.",
  "headers": {
    "content-type": "application/json",
    "date": "2026-01-14"
  },
  "timestamp": "2026-01-14T10:30:00Z"
}
```

---

### Test Strategy 3: Chat Completion API Testing

**Purpose**: Test if models can handle actual API requests

**Endpoint**: `https://generativelanguage.googleapis.com/v1beta/models/{MODEL_ID}:generateContent` (AI Studio)

**Testing Steps** (for each model 314-327):
1. [ ] Construct request with model ID
2. [ ] Send minimal test request (e.g., "Hello")
3. [ ] Capture response or error
4. [ ] Document response time and error details

**Test Request**:
```json
{
  "model": "models/314",
  "contents": {
    "parts": [
      {
        "text": "Hello, can you respond?"
      }
    ]
  }
}
```

**Expected Responses**:
- **200 OK + Response**: Model is active and working
- **404 Not Found**: Model doesn't exist or ID is invalid
- **429 Too Many Requests**: Model exists but rate limited
- **400 Bad Request**: Model ID format not recognized
- **403 Forbidden**: Access denied or model deprecated

**Documentation to Capture**:
```json
{
  "model_id": 314,
  "test_type": "chat_completion",
  "request": "Hello, can you respond?",
  "response_code": 404,
  "response_time_ms": 128,
  "status": "MODEL_NOT_FOUND",
  "error_detail": "Resource not found",
  "timestamp": "2026-01-14T10:31:00Z"
}
```

---

### Test Strategy 4: Models List Endpoint (AI Studio)

**Purpose**: Get list of all available models from Google AI Studio

**Endpoint**: `https://generativelanguage.googleapis.com/v1beta/models`

**Testing Steps**:
1. [ ] Query models list endpoint
2. [ ] Parse response for all model IDs
3. [ ] Search specifically for 314-327 range
4. [ ] Extract model names and capabilities
5. [ ] Note any Gemini 3.x variants

**Expected Response**: List of ~20-50 models with metadata

**Documentation to Capture**:
```json
{
  "endpoint": "ai_studio_list_models",
  "response_code": 200,
  "total_models_returned": 35,
  "models_314_327_found": 0,
  "model_ids_found": [246, 312, 313, 333, 334, 335, 336],
  "gap_analysis": "No models in 314-327 range found"
}
```

---

### Test Strategy 5: Google Cloud Documentation Check

**Purpose**: Cross-reference API responses with official documentation

**Resources to Check**:
- [ ] Vertex AI Models Docs: https://cloud.google.com/python/docs/reference/aiplatform/latest/google.cloud.aiplatform.gapic.v1.types
- [ ] AI Studio Docs: https://ai.google.dev/models
- [ ] Release Notes: https://cloud.google.com/vertex-ai/docs/release-notes
- [ ] Gemini API Changelog: https://ai.google.dev/release-notes

**Documentation Checklist**:
- [ ] Any mention of model ID range 314-327
- [ ] Any deprecation notices for this range
- [ ] Release timeline for potential new models
- [ ] Breaking changes or removed models list

---

## API Testing Checklist - Individual Tests

### For Each Model (314-327):

**Model ID: 314**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 315**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 316**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 317**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 318**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 319**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 320**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 321**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 322**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 323**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 324**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 325**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 326**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

**Model ID: 327**
- [ ] Vertex AI endpoint test: ___ (response code)
- [ ] Direct model endpoint test: ___ (response code)
- [ ] Chat completion test: ___ (response code)
- [ ] Error message captured: ___
- [ ] Documentation notes: ___
- [ ] Hypothesis confirmed: [ ] Deprecated [ ] Reserved [ ] Active [ ] Unknown

---

## Success Criteria - Hypothesis Validation

### Hypothesis 1: Deprecated Models (70% probability)
**Confirms if**:
- ✅ All model endpoints return 404 Not Found
- ✅ Error messages contain "deprecated", "removed", or "no longer available"
- ✅ No models appear in current model list endpoints
- ✅ Google Cloud release notes mention removal of 314-327 range

**Action if Confirmed**: Document as deprecated, update MASTER-MODELS-TABLE

---

### Hypothesis 2: Reserved/Future Models (25% probability)
**Confirms if**:
- ✅ Some models return 404 (not yet released)
- ✅ Some models return 403 (access denied/experimental)
- ✅ Error messages reference "experimental" or "not available yet"
- ✅ Google I/O announcements mention future releases in this range

**Action if Confirmed**: Mark as reserved, set target release date if known

---

### Hypothesis 3: External-Only Models (5% probability)
**Confirms if**:
- ✅ Some models return 200 and accept requests
- ✅ Models respond successfully to API calls
- ✅ Models are documented in Vertex AI or AI Studio
- ✅ No errors related to model ID validity

**Action if Confirmed**: Prioritize for integration in Antigravity, create COMPARISON files

---

## Response Documentation Template

For each test, capture response in this format:

```markdown
### Model ID: 314
**Date Tested**: 2026-01-14
**Tester**: [Dev Name]

#### Test 1: Vertex AI List Models
- **Endpoint**: `/v1/projects/{PROJECT_ID}/locations/us-central1/models`
- **Response Code**: 200
- **Model Found**: NO
- **Time**: 245ms
- **Notes**: Model 314 not present in list

#### Test 2: Direct Model Endpoint
- **Endpoint**: `/v1/projects/{PROJECT_ID}/locations/us-central1/publishers/google/models/314`
- **Response Code**: 404
- **Error Message**: "Resource not found"
- **Time**: 128ms
- **Notes**: Confirms model doesn't exist

#### Test 3: Chat Completion API
- **Endpoint**: `/v1beta/models/314:generateContent`
- **Response Code**: 404
- **Error Message**: "The model `314` was not found."
- **Time**: 156ms
- **Notes**: Model not accessible via AI Studio

#### Summary
- **Status**: NOT FOUND
- **Hypothesis**: Deprecated (LIKELY) or Reserved (POSSIBLE)
- **Confidence**: HIGH (3/3 tests confirm absence)
- **Next Step**: Document as deprecated pending release notes verification
```

---

## Documentation and Logging

### Test Harness Output Format

All test results should be logged to: `logs/day2-api-testing/model-tests-{date}.json`

```json
{
  "test_date": "2026-01-14",
  "test_duration_minutes": 120,
  "tester": "Dev A, Dev B, Dev C",
  "models_tested": 14,
  "summary": {
    "total_tests": 56,
    "passed": 42,
    "failed": 0,
    "not_found": 56,
    "errors": 0
  },
  "hypothesis_validation": {
    "deprecated": "56/56 tests return 404 - CONFIRMED",
    "reserved": "0/56 tests return 403 - NOT CONFIRMED",
    "external_only": "0/56 tests return 200 - NOT CONFIRMED"
  },
  "models": [
    {
      "model_id": 314,
      "status": "NOT_FOUND",
      "tests": {
        "vertex_ai_list": 404,
        "direct_endpoint": 404,
        "chat_completion": 404
      }
    }
    // ... more models
  ]
}
```

---

## Contingency Plans

### If APIs Are Not Accessible
- [ ] Verify API keys and authentication
- [ ] Check Google Cloud project quota limits
- [ ] Verify network connectivity to Google APIs
- [ ] Check firewall/proxy restrictions
- [ ] Use alternative: Query public documentation instead

### If Unexpected Models Are Found
- [ ] Document model name and capabilities
- [ ] Create detailed COMPARISON file
- [ ] Evaluate integration feasibility
- [ ] Prioritize for implementation discussion

### If Tests Take Longer Than Expected
- [ ] Reduce scope to critical models (314, 320, 325)
- [ ] Parallelize testing across team members
- [ ] Move non-critical tests to Day 3
- [ ] Document completion percentage

---

## Team Coordination

### Morning Session (Dev A)
- Execute Vertex AI endpoint tests (Tests 1-2)
- Focus on models 314-320
- Document findings in shared log

### Midday Session (Dev B)
- Execute AI Studio endpoint tests (Tests 3-4)
- Focus on models 321-327
- Cross-reference with documentation

### Afternoon Session (Dev C + Team)
- Consolidate all findings
- Update tracking matrix
- Prepare Day 2 findings report
- Validate hypotheses

---

## Post-Testing Deliverables

By end of Day 2, complete:

- [ ] **Updated Tracking Matrix**: All 14 models with API test results
- [ ] **Day 2 Findings Report**: Summary of hypothesis validation
- [ ] **Test Results Log**: Complete JSON with all API responses
- [ ] **Hypothesis Confirmation**: Clear determination of which scenario applies
- [ ] **Next Steps Recommendation**: Implementation or documentation actions

---

## Notes for Success

1. **Be Systematic**: Test all models consistently, don't skip any
2. **Document Failures**: Even "404 Not Found" is valuable evidence
3. **Capture Timestamps**: Record when each test was run
4. **Error Messages Matter**: Full error messages help identify patterns
5. **Communication**: Update team on findings throughout the day

---

**Prepared By**: Dev C (Junior Developer)
**Reviewed By**: Tech Lead
**Date Prepared**: 2026-01-13 (Day 1 Afternoon)
**Ready for Execution**: 2026-01-14 (Day 2 Morning)
**Version**: 1.0
