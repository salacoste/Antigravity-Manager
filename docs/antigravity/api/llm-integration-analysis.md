# Antigravity LLM Integration Analysis

**Focus**: LLM communication layer only
**Goal**: Understand API endpoints, payload structures, model routing

## Analysis Scope

### In Scope ✅
- API endpoints for LLM requests
- Request/response payload structures
- Model naming and routing logic
- Authentication for LLM APIs
- Streaming vs non-streaming implementations
- Error handling and retry logic

### Out of Scope ❌
- UI components
- Settings/preferences
- File system operations
- Editor integrations
- Non-LLM features

## Discovery Strategy

### Phase 1: Static Code Analysis (Current)
Search for LLM-specific patterns in `extension.js`:
- Model names (gemini-pro, gemini-ultra, etc.)
- API method names (generateContent, streamGenerateContent)
- Google AI endpoints
- OpenAI-compatible endpoints
- Anthropic Claude endpoints

### Phase 2: Network Traffic Capture (Next)
Intercept actual API calls to observe:
- Real endpoint URLs
- Complete request headers
- Full payload structures
- Response formats
- Authentication tokens

### Phase 3: Runtime Analysis
Debug live requests to understand:
- Request construction logic
- Model selection algorithm
- Token management
- Rate limiting

## Code Search Patterns

### 1. Model Names
```bash
# Search for Gemini model references
grep -i 'gemini-pro\|gemini-ultra\|gemini-flash' extension.js

# Search for other LLM providers
grep -i 'gpt-4\|claude\|llama' extension.js

# Generic model patterns
grep -o 'models?/[a-z0-9-]*' extension.js
```

### 2. API Endpoints
```bash
# Google AI endpoints
grep -o 'generativelanguage\.googleapis\.com[^"]*' extension.js

# Generic AI API patterns
grep -i 'generateContent\|chat/completions\|messages' extension.js
```

### 3. Request Structures
```bash
# Look for request building patterns
grep -i 'contents\|messages\|prompt' extension.js | head -20

# Look for configuration objects
grep -i 'temperature\|maxTokens\|topP' extension.js | head -20
```

## Expected API Architecture

### Hypothesis: Google AI API (Primary)

**Base URL**: `https://generativelanguage.googleapis.com/v1beta/`

**Endpoints**:
```
POST /v1beta/models/{model}:generateContent
POST /v1beta/models/{model}:streamGenerateContent
POST /v1beta/models/{model}:countTokens
GET  /v1beta/models/{model}
GET  /v1beta/models
```

**Request Structure** (Expected):
```json
{
  "contents": [
    {
      "role": "user",
      "parts": [
        { "text": "User message here" }
      ]
    }
  ],
  "generationConfig": {
    "temperature": 0.7,
    "topK": 40,
    "topP": 0.95,
    "maxOutputTokens": 2048,
    "stopSequences": []
  },
  "safetySettings": [
    {
      "category": "HARM_CATEGORY_HARASSMENT",
      "threshold": "BLOCK_MEDIUM_AND_ABOVE"
    }
  ]
}
```

**Response Structure** (Expected):
```json
{
  "candidates": [
    {
      "content": {
        "parts": [
          { "text": "AI response here" }
        ],
        "role": "model"
      },
      "finishReason": "STOP",
      "safetyRatings": [...]
    }
  ],
  "usageMetadata": {
    "promptTokenCount": 10,
    "candidatesTokenCount": 20,
    "totalTokenCount": 30
  }
}
```

### Hypothesis: OpenAI-Compatible Proxy

Some applications use OpenAI format internally and convert to Google AI:

**Endpoint**: `/v1/chat/completions`

**Request**:
```json
{
  "model": "gemini-pro",
  "messages": [
    {
      "role": "user",
      "content": "User message"
    }
  ],
  "temperature": 0.7,
  "max_tokens": 2048,
  "stream": true
}
```

## Findings Log

### 2026-01-09: Initial Search

**Search 1**: Direct URL patterns
```bash
grep -o '"https://[^"]*generativelanguage[^"]*"' extension.js
```
**Result**: _Pending_

**Search 2**: Model references
```bash
grep -o 'gemini-[a-z0-9-]*' extension.js | sort -u
```
**Result**: _Pending_

**Search 3**: API method names
```bash
grep -o '[a-zA-Z]*generateContent' extension.js | sort -u
```
**Result**: _Pending_

## Next Steps

1. **Beautify Code**: Make extension.js readable
2. **Pattern Search**: Run targeted searches for LLM patterns
3. **Network Capture**: Set up mitmproxy to intercept live traffic
4. **Request Logging**: Enable verbose logging if available
5. **API Documentation**: Document all discovered endpoints

## Tools & Setup

### mitmproxy Setup
```bash
# Install
brew install mitmproxy

# Run proxy
mitmproxy -p 8080

# Configure Antigravity to use proxy (if needed)
# Environment variables or app settings
```

### Chrome DevTools
```bash
# Launch with debugging
/Applications/Antigravity.app/Contents/MacOS/Antigravity --remote-debugging-port=9222

# Access at
http://localhost:9222
```

## Documentation Targets

Once discovered, document in:
- `api/endpoints.md` - All LLM API endpoints
- `api/payloads.md` - Request/response structures
- `api/models.md` - Available models and capabilities
- `api/authentication.md` - Auth flow for LLM APIs

## Success Criteria

Analysis complete when we can answer:
- ✅ Which API endpoints are used for each model?
- ✅ What is the exact request payload structure?
- ✅ How are streaming responses handled?
- ✅ What authentication method is used?
- ✅ How are errors and rate limits handled?
- ✅ Can we replicate these requests programmatically?
