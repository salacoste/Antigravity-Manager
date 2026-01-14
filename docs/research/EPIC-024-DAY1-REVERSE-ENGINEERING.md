# Epic-024 Day 1: Reverse Engineering Analysis - gemini-2.5-flash

**Model ID**: 312
**Model Name**: `gemini-2.5-flash`
**Team**: Team 2 (Multi-Protocol Specialists)
**Date**: 2026-01-26
**Session**: Day 1 Morning (4 hours)

---

## 📊 Workflow Analysis Summary

**Source Document**: `docs/antigravity/workflows/models/gemini/gemini-2.5-flash-workflow.md`
**Document Size**: 21 KB (1,046 lines)
**Completeness**: ✅ **COMPREHENSIVE** - Full reverse engineering from Antigravity v1.13.3
**Last Updated**: 2026-01-10

---

## 🎯 Core Capabilities Matrix

### Model Characteristics

```yaml
model_id: 312
api_name: "gemini-2.5-flash"
provider: "Google Vertex AI"
api_provider: "API_PROVIDER_GOOGLE_GEMINI (24)"

capabilities:
  thinking_mode: ❌ DISABLED (use Model ID 313 for thinking)
  tool_use: ✅ ENABLED
  web_search: ✅ ENABLED (Google Search integration)
  multimodal: ❌ DISABLED (text-only)
  streaming: ✅ ENABLED (SSE format)

performance_profile:
  speed: "⚡⚡⚡ Very Fast"
  quality: "⭐⭐⭐ Good"
  cost: "$ Low"
  tier: "Flash (cost-optimized)"

token_limits:
  max_input_tokens: 1048576  # 1M tokens
  max_output_tokens: 65536   # 64K tokens
  context_window: 1048576    # 1M total
  thinking_budget: 0         # N/A (no thinking)
```

### Use Cases

**✅ Best For**:
- Quick code completions and suggestions
- General coding tasks with standard quality
- Fast response requirements
- Cost-sensitive applications
- High-volume workloads

**❌ Not Recommended For**:
- Complex reasoning tasks → Use `gemini-2.5-flash-thinking` (Model ID 313)
- Image processing → Use `gemini-2.5-flash-image-preview`
- Maximum quality → Use `gemini-2.5-pro` (Model ID 246)

---

## 🔍 Feature Extraction

### 1. Anti-Detection / Identity Compliance

**Compliance Requirements** (10 checkpoints):

```yaml
user_agent:
  format: "antigravity/{version} {platform}/{arch}"
  current_version: "1.13.3"
  platforms: [darwin/arm64, darwin/x64, linux/arm64, linux/x64, win32/x64]
  critical: "Outdated version WILL BE DETECTED"

request_id:
  pattern: "agent-{uuid4}"
  requirement: "ALWAYS prefix with 'agent-'"
  format: "RFC 4122 UUID v4"
  example: "agent-550e8400-e29b-41d4-a716-446655440000"

ide_type:
  value: "ANTIGRAVITY"
  location: "metadata.ideType in loadCodeAssist"
  purpose: "Project discovery + quota allocation"

system_instruction:
  identity: "You are Antigravity, a powerful agentic AI coding assistant designed by the Google Deepmind team working on Advanced Agentic Coding."
  critical: "Identity text MUST match exactly"

http_headers:
  required: ["Content-Type: application/json", "Authorization: Bearer {token}", "User-Agent: antigravity/1.13.3"]
  forbidden: ["X-Custom-*", "X-Forwarded-For", "Extra auth headers"]

endpoints:
  production: "https://cloudcode-pa.googleapis.com/v1internal"
  sandbox: "https://daily-cloudcode-pa.sandbox.googleapis.com/v1internal"
  fallback_strategy: "Production-first, sandbox on 429/404/408/5xx only"
```

**Detection Risks** (7 categories):
1. User-Agent mismatch or outdated version
2. Request ID format errors (missing `agent-` prefix)
3. Missing ideType metadata
4. System instruction differences
5. Header inconsistencies (extra/missing)
6. Endpoint pattern violations
7. Version/behavior mismatches

### 2. Request Construction

**Complete Request Structure**:

```yaml
top_level:
  project: "GCP project ID (from loadCodeAssist or default)"
  requestId: "agent-{uuid4}"
  model: "gemini-2.5-flash"
  request: {...}  # Nested request object

request_object:
  contents: []  # Required - message history
  systemInstruction: {...}  # Recommended - Antigravity identity
  generationConfig: {...}  # Recommended - generation parameters
  safetySettings: []  # Recommended - set to OFF
  tools: []  # Optional - function declarations or googleSearch
  toolConfig: {...}  # Optional - function calling config

contents_structure:
  - role: "user" | "model"
    parts:
      - text: "Message text"
      - functionCall: {...}  # Tool calls
      - functionResponse: {...}  # Tool results

generation_config:
  maxOutputTokens: 8192  # Default, max 65536
  temperature: 0.7  # 0.0-1.0
  topP: 0.95  # 0.0-1.0
  topK: 40  # Integer
  stopSequences: ["<|user|>", "<|endoftext|>", "<|end_of_turn|>", "[DONE]", "\n\nHuman:"]
  thinkingConfig: ❌ NOT SUPPORTED (Model ID 312 has no thinking)

safety_settings:
  categories: [HARASSMENT, HATE_SPEECH, SEXUALLY_EXPLICIT, DANGEROUS_CONTENT]
  threshold: "OFF"  # Required to avoid false positives
```

### 3. Tool Use Capabilities

**Function Declarations**:

```yaml
tool_structure:
  tools:
    - functionDeclarations:
        - name: "function_name"
          description: "Function description"
          parameters:
            type: "object"
            properties: {...}
            required: ["param1", "param2"]

tool_config:
  functionCallingConfig:
    mode: "VALIDATED"  # Strict validation

tool_response_format:
  functionCall:
    name: "function_name"
    args: {param1: value1}
    id: "toolu_01ABC123"

tool_result_submission:
  functionResponse:
    name: "function_name"
    response: {result: "..."}
    id: "toolu_01ABC123"  # Must match functionCall.id
```

**Critical Limitations**:
- ❌ **Cannot mix** `functionDeclarations` with `googleSearch`
- ✅ Use `functionDeclarations` OR `googleSearch`, not both
- ✅ `toolConfig.functionCallingConfig.mode: "VALIDATED"` recommended

### 4. Web Search Integration

**Google Search Tool**:

```yaml
enablement:
  tools:
    - googleSearch: {}  # Empty object enables search

response_structure:
  groundingMetadata:
    webSearchQueries: ["search query text"]
    groundingAttributions:
      - segment: {startIndex: 0, endIndex: 45}
        confidenceScore: 0.95
        web:
          uri: "https://source.url"
          title: "Page title"

critical_limitation:
  mixing_tools: "❌ INVALID - Cannot combine googleSearch with functionDeclarations"
  valid_usage: "✅ Only googleSearch in tools array"
```

### 5. Streaming Support

**SSE (Server-Sent Events) Streaming**:

```yaml
endpoint:
  url: "https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse"
  method: "POST"

stream_format:
  line_format: "data: {json_chunk}\n\n"
  final_marker: "data: [DONE]"

parsing_logic:
  - Read lines starting with "data: "
  - Parse JSON from data payload
  - Extract text from candidates[0].content.parts[0].text
  - Accumulate until "[DONE]" marker
  - Final chunk contains usageMetadata

chunk_structure:
  candidates:
    - content:
        role: "model"
        parts:
          - text: "Incremental text chunk"
```

### 6. Response Handling

**Response Structure**:

```yaml
success_response:
  candidates:
    - content:
        role: "model"
        parts:
          - text: "Model response"
      finishReason: "STOP" | "MAX_TOKENS" | "SAFETY" | "RECITATION"
      safetyRatings: [...]

  usageMetadata:
    promptTokenCount: 45
    candidatesTokenCount: 123
    totalTokenCount: 168

finish_reasons:
  STOP: "Normal completion - use response"
  MAX_TOKENS: "Hit output limit - truncated"
  SAFETY: "Safety filter triggered - review content"
  RECITATION: "Recitation detected - retry with different prompt"
```

### 7. Quota Management

**Quota Tracking**:

```yaml
quota_type: "gemini_family"
shared_quota_pool:
  - gemini-2.5-flash (312)
  - gemini-2.5-flash-thinking (313)
  - gemini-2.5-flash-lite
  - gemini-2.5-flash-thinking-tools

quota_check_api:
  endpoint: "https://cloudcode-pa.googleapis.com/v1internal:fetchAvailableModels"
  request: {project: "project_id"}
  response:
    models:
      "gemini-2.5-flash":
        quotaInfo:
          remainingFraction: 0.87  # 87% remaining
          resetTime: "2026-01-11T00:00:00Z"  # UTC midnight

quota_exhaustion:
  error_code: 429
  status: "RESOURCE_EXHAUSTED"
  recovery:
    - "Wait until resetTime (UTC midnight)"
    - "Switch to alternative account"
    - "Use different model (if available)"
```

### 8. Error Handling

**Common Errors** (6 categories):

```yaml
invalid_request_400:
  causes: ["Empty contents", "Invalid role alternation", "Missing fields"]
  fix: "Validate request structure, merge adjacent same-role messages"

authentication_401:
  causes: ["Invalid token", "Expired token"]
  recovery: "Refresh OAuth token and retry"

permission_denied_403:
  causes: ["Vertex AI API not enabled", "Insufficient permissions"]
  fix: "Enable Vertex AI API in GCP project"

quota_exceeded_429:
  causes: ["Daily quota exhausted"]
  recovery: "Wait until resetTime or switch account"

server_errors_500_503:
  causes: ["Internal server error", "Service unavailable"]
  recovery: "Exponential backoff retry (3 attempts)"
```

---

## 📋 API Behavior Patterns

### Authentication Flow

```yaml
oauth_2.0:
  required_scopes:
    - https://www.googleapis.com/auth/cloud-platform
    - https://www.googleapis.com/auth/userinfo.email

  token_lifecycle:
    - Obtain: "OAuth authorization flow"
    - Validate: "Check expires_at before each request"
    - Refresh: "Use refresh_token when expired"
    - Retry: "On 401, refresh and retry once"
```

### Project Discovery

```yaml
load_code_assist_api:
  endpoint: "https://cloudcode-pa.googleapis.com/v1internal:loadCodeAssist"
  request:
    metadata:
      ideType: "ANTIGRAVITY"

  response:
    cloudaicompanionProject: "bamboo-precept-lgxtn"  # Use this project ID

  fallback: "bamboo-precept-lgxtn"  # Default if discovery fails
```

### Request Optimization

```yaml
message_merging:
  purpose: "Avoid role alternation errors"
  logic: "Merge adjacent same-role messages"
  example:
    before: [{role: "user", parts: [p1]}, {role: "user", parts: [p2]}]
    after: [{role: "user", parts: [p1, p2]}]

retry_strategy:
  max_retries: 3
  backoff: "Exponential (2^n seconds)"
  retry_codes: [429, 500, 502, 503, 504]
  no_retry_codes: [400, 401, 403]
```

---

## 🎯 Capability Comparison Points

### vs. gemini-2.5-flash-thinking (Model ID 313)

**Key Differences**:

```yaml
model_312_base:
  thinking_mode: ❌ DISABLED
  thinking_budget: 0
  speed: "⚡⚡⚡ Very Fast"
  cost: "$ Low"
  use_case: "Fast completions, standard quality"

model_313_thinking:
  thinking_mode: ✅ ENABLED
  thinking_budget: 24576  # Separate token budget
  speed: "⚡⚡ Fast (slower than base)"
  cost: "$$ Medium"
  use_case: "Complex reasoning, multi-step problems"

architecture:
  id_based: "Separate Model IDs (312 base, 313 thinking)"
  not_parameter_based: "Unlike Pro (246), which uses thinkingBudget parameter"
```

### vs. gemini-2.5-pro (Model ID 246)

```yaml
model_312_flash:
  tier: "Flash (cost-optimized)"
  speed: "⚡⚡⚡ Very Fast"
  quality: "⭐⭐⭐ Good"
  cost: "$ Low"

model_246_pro:
  tier: "Pro (quality-optimized)"
  speed: "⚡⚡ Fast"
  quality: "⭐⭐⭐⭐⭐ Excellent"
  cost: "$$$ High"

trade_offs:
  flash: "50-70% cheaper, 20-30% faster, good for most tasks"
  pro: "Higher quality, better reasoning, recommended for critical tasks"
```

---

## 🔧 Implementation Patterns

### Best Practices (4 categories)

**1. Performance Optimization**:
```yaml
streaming:
  use_case: "Long responses (>500 tokens)"
  benefit: "Faster perceived performance"
  endpoint: ":streamGenerateContent?alt=sse"

output_limits:
  recommendation: "Set maxOutputTokens based on expected length"
  fast_responses: 2048
  standard: 8192
  comprehensive: 16384
```

**2. Error Handling**:
```yaml
retry_logic:
  max_attempts: 3
  backoff: "2^n * 1000ms"
  retry_codes: [429, 500, 502, 503]

token_refresh:
  trigger: "401 UNAUTHENTICATED error"
  action: "Refresh OAuth token, retry once"
```

**3. Quota Management**:
```yaml
pre_check:
  api: "fetchAvailableModels"
  threshold: "remainingFraction < 0.1 → warn user"

monitoring:
  track: "usageMetadata.totalTokenCount per request"
  aggregate: "Daily total vs. quota limit"
```

**4. Request Optimization**:
```yaml
message_merging:
  benefit: "Avoid 400 errors from role alternation"
  implementation: "Merge adjacent same-role messages"

conversation_pruning:
  trigger: "Context approaching 1M tokens"
  strategy: "Keep recent N turns + system instruction"
```

---

## 📊 Documentation Completeness Assessment

### Workflow Coverage

```yaml
sections:
  anti_detection: ✅ COMPLETE (6 identity markers + 7 risks)
  model_overview: ✅ COMPLETE (capabilities + use cases)
  request_workflow: ✅ COMPLETE (6-step flow + validation)
  response_workflow: ✅ COMPLETE (structure + streaming)
  quota_behavior: ✅ COMPLETE (tracking + exhaustion)
  error_handling: ✅ COMPLETE (6 error types + recovery)
  tool_use: ✅ COMPLETE (declarations + responses + limitations)
  web_search: ✅ COMPLETE (enablement + response + limitations)
  examples: ✅ COMPLETE (2 complete examples)
  best_practices: ✅ COMPLETE (4 categories)

total_sections: 10
documented_sections: 10
coverage: "100%"
```

### Technical Depth

```yaml
api_specification:
  endpoints: ✅ Fully documented
  request_structure: ✅ Complete JSON examples
  response_structure: ✅ Complete with field descriptions
  error_codes: ✅ All major errors covered

authentication:
  oauth_flow: ✅ Token lifecycle documented
  scopes: ✅ Required scopes listed
  refresh_logic: ✅ Refresh strategy detailed

anti_detection:
  identity_markers: ✅ All 6 markers documented
  detection_risks: ✅ All 7 risks identified
  compliance_checklist: ✅ Actionable checklist provided

tool_integration:
  function_declarations: ✅ Complete structure
  tool_responses: ✅ Response format documented
  limitations: ✅ Critical limitations noted
  web_search: ✅ Separate documentation
```

---

## 🎯 Preparation for COMPARISON File

### Key Comparison Dimensions Identified

**1. Architecture Differences**:
- ID-based vs. parameter-based thinking activation
- Separate Model IDs (312/313) vs. single ID with parameter (246)

**2. Performance Trade-offs**:
- Flash tier (speed/cost) vs. Pro tier (quality)
- Base Flash (fastest) vs. Flash Thinking (moderate) vs. Pro (slowest)

**3. Use Case Segmentation**:
- Cost-conscious workloads (Flash base)
- Complex reasoning with budget constraints (Flash thinking)
- Quality-critical tasks (Pro)

**4. Quota Architecture**:
- Shared quota pool (Flash family)
- Separate Pro quota
- Reset timing and frequency

**5. Tool Use Patterns**:
- Standard tool use (base Flash)
- Tool use with thinking signatures (Flash thinking)
- Web search limitations (both)

### COMPARISON File Structure Preview

```yaml
sections_to_create:
  1_model_identification: "Clear differentiation (312 vs. 313 vs. 246)"
  2_capabilities_comparison: "Feature matrix with checkmarks"
  3_architecture_differences: "ID-based vs. parameter-based"
  4_performance_benchmarks: "Speed/quality/cost trade-offs"
  5_use_case_mapping: "When to use each model"
  6_quota_behavior: "Shared vs. separate quotas"
  7_tool_integration: "Standard vs. thinking-enhanced"
  8_migration_guides: "Switching between Flash models"
  9_gap_analysis: "Missing features and improvements"
  10_implementation_examples: "Side-by-side code comparisons"

estimated_size: "~30 KB"
estimated_effort: "8-12 hours (Day 2-3)"
```

---

## ✅ Day 1 Morning Deliverables

**Completed**:
- ✅ Analyzed 21 KB workflow document (1,046 lines)
- ✅ Extracted all features and capabilities (8 major categories)
- ✅ Documented API behavior patterns (authentication, project discovery, optimization)
- ✅ Created capability matrix (10 sections, 100% coverage)
- ✅ Identified comparison dimensions (5 key areas)
- ✅ Prepared COMPARISON file structure (10 sections planned)

**Next Steps** (Day 1 Afternoon):
- Compare with gemini-2.5-pro (Model ID 246) - parameter-based architecture
- Compare with gemini-2.5-flash-thinking (Model ID 313) - thinking mode differences
- Identify unique Flash base capabilities
- Document differences from other Flash variants

---

**Document Status**: ✅ **DAY 1 MORNING COMPLETE**
**Created By**: Team 2 (Multi-Protocol Specialists)
**Date**: 2026-01-26
**Epic**: Epic-024
**Phase**: Prep Phase - Reverse Engineering
