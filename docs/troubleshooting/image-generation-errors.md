# Image Generation Error Troubleshooting Guide

This guide helps you troubleshoot errors when generating images using Gemini 3 Pro Image via the OpenAI-compatible `/v1/images/generations` and `/v1/images/edits` endpoints.

## Error Categories

All errors are categorized into four types for easier diagnosis:

- **USER_ERROR**: Client-side validation errors (400 Bad Request, invalid parameters)
- **API_ERROR**: Upstream API errors (429 Rate Limit, 503 Service Unavailable, quota issues)
- **SYSTEM_ERROR**: Internal server errors (500 Internal Server Error, unexpected exceptions)
- **NETWORK_ERROR**: Network errors (timeouts, connection failures)

## Common Errors

### IMG_QUOTA_EXHAUSTED

**Error Code**: `IMG_QUOTA_EXHAUSTED`
**Category**: `API_ERROR`
**HTTP Status**: 429

**Description**: Account quota exhausted for image generation

**Error Message Example**:
```
Rate limit exceeded: The API quota for image generation has been exhausted for this account. The system will automatically rotate to another account. If all accounts are exhausted, please wait for quota reset (typically 24 hours).
```

**Resolution**:
1. **Automatic Account Rotation**: The system automatically rotates to the next available account
2. **Wait for Quota Reset**: If all accounts exhausted, quota typically resets in 24 hours
3. **Add More Accounts**: Consider adding more Google accounts to increase quota pool
4. **Reduce Generation Rate**: Implement rate limiting in your application
5. **Check Dashboard**: Monitor quota status in the Antigravity Tools dashboard

**Log Query Examples**:
```bash
# Find all quota errors in the last 24 hours
grep "IMG_QUOTA_EXHAUSTED" logs/antigravity.log | tail -100

# Find quota errors for specific account
grep "IMG_QUOTA_EXHAUSTED" logs/antigravity.log | grep "account_email=user@example.com"
```

---

### IMG_SAFETY_BLOCKED

**Error Code**: `IMG_SAFETY_BLOCKED`
**Category**: `API_ERROR`
**HTTP Status**: 400

**Description**: Content blocked by safety filter

**Error Message Example**:
```
Content safety filter triggered: The generated image was blocked by content safety filters. Try adjusting your prompt or lowering the safety_threshold setting.
```

**Resolution**:
1. **Adjust Prompt**: Modify your prompt to avoid potentially unsafe content
2. **Lower Safety Threshold**: Configure safety threshold via:
   - **Environment Variable**: `GEMINI_IMAGE_SAFETY_THRESHOLD=OFF|LOW|MEDIUM|HIGH`
   - **Per-Request Parameter**: Include `"safety_threshold": "OFF"` in your request body
3. **Valid Threshold Values**:
   - `OFF`: No filtering (default for backward compatibility)
   - `LOW`: Block only high-risk content (`BLOCK_ONLY_HIGH`)
   - `MEDIUM`: Block medium and high-risk content (`BLOCK_MEDIUM_AND_ABOVE`)
   - `HIGH`: Block all except low-risk content (`BLOCK_LOW_AND_ABOVE`)
4. **Request-Level Override**: Per-request `safety_threshold` parameter takes priority over environment variable

**Example Request with Safety Override**:
```json
{
  "prompt": "A beautiful mountain landscape",
  "model": "gemini-3-pro-image",
  "size": "1024x1024",
  "safety_threshold": "LOW"
}
```

**Log Query Examples**:
```bash
# Find all safety filter blocks
grep "IMG_SAFETY_BLOCKED" logs/antigravity.log

# Find safety blocks with prompt hashes for correlation
grep "safety_threshold" logs/antigravity.log | grep "error_type=API_ERROR"
```

---

### IMG_MISSING_PROMPT

**Error Code**: `IMG_MISSING_PROMPT`
**Category**: `USER_ERROR`
**HTTP Status**: 400

**Description**: Request missing required 'prompt' field

**Error Message Example**:
```
Invalid request: Missing or empty prompt. Please provide a text description for image generation.
```

**Resolution**:
1. Ensure your request includes a `prompt` field
2. Verify the prompt is not empty
3. Check JSON formatting

**Example Valid Request**:
```json
{
  "prompt": "A serene mountain landscape at sunset",
  "model": "gemini-3-pro-image",
  "n": 1,
  "size": "1024x1024"
}
```

---

### IMG_SERVICE_UNAVAILABLE

**Error Code**: `IMG_SERVICE_UNAVAILABLE`
**Category**: `API_ERROR`
**HTTP Status**: 503

**Description**: Gemini API temporarily unavailable or all accounts rate-limited

**Error Message Example**:
```
Service unavailable: The Gemini API is temporarily unavailable or all accounts are rate-limited. The system will retry automatically.
```

**Resolution**:
1. **Automatic Retry**: The system retries automatically with exponential backoff
2. **Check API Status**: Verify Gemini API status at [Google Cloud Status](https://status.cloud.google.com/)
3. **Add More Accounts**: Increase account pool for better availability
4. **Wait and Retry**: Temporary service issues typically resolve within minutes

**Log Query Examples**:
```bash
# Find all service unavailable errors
grep "IMG_SERVICE_UNAVAILABLE" logs/antigravity.log

# Check retry patterns
grep "503" logs/antigravity.log | grep "generation_time_ms"
```

---

### IMG_NETWORK_ERROR

**Error Code**: `IMG_NETWORK_ERROR`
**Category**: `NETWORK_ERROR`
**HTTP Status**: N/A (connection failure)

**Description**: Network connectivity issue to upstream Gemini API

**Error Message Example**:
```
Network error: Unable to connect to upstream API. This may be a temporary connectivity issue. The system will retry automatically.
```

**Resolution**:
1. **Automatic Retry**: The system retries automatically
2. **Check Network**: Verify internet connectivity
3. **Check Firewall**: Ensure firewall allows outbound HTTPS to Google APIs
4. **Check Proxy Settings**: Verify upstream proxy configuration if used
5. **DNS Issues**: Verify DNS resolution for `*.googleapis.com`

**Log Query Examples**:
```bash
# Find all network errors
grep "NETWORK_ERROR" logs/antigravity.log

# Find connection timeouts
grep "timeout" logs/antigravity.log | grep "error_type=NETWORK_ERROR"
```

---

## Log Query Examples

### Find Errors by Model

```bash
# Find all errors for gemini-3-pro-image
grep "model=gemini-3-pro-image" logs/antigravity.log | grep "ERROR"

# Find errors for specific model version
grep "model=gemini-3-pro-image" logs/antigravity.log | grep "error_type"
```

### Find Errors by Account

```bash
# Find all errors for specific account
grep "account_email=user@example.com" logs/antigravity.log | grep "ERROR"

# Find quota errors by account
grep "account_email=user@example.com" logs/antigravity.log | grep "IMG_QUOTA_EXHAUSTED"
```

### Find Errors by Time Range

```bash
# Find errors in last hour
grep "$(date -u -v-1H '+%Y-%m-%d %H')" logs/antigravity.log | grep "error_type"

# Find errors today
grep "$(date -u '+%Y-%m-%d')" logs/antigravity.log | grep "error_type"
```

### Find Errors by Operation

```bash
# Find all image generation errors
grep "operation=image_gen" logs/antigravity.log | grep "error_type"

# Find all image edit errors
grep "operation=image_edit" logs/antigravity.log | grep "error_type"
```

### Analyze Error Patterns

```bash
# Count errors by category
grep "error_type" logs/antigravity.log | cut -d'=' -f2 | cut -d' ' -f1 | sort | uniq -c

# Find slow generations (>10 seconds)
grep "generation_time_ms" logs/antigravity.log | awk -F'generation_time_ms=' '{print $2}' | awk '{if ($1 > 10000) print $0}'

# Find errors with specific prompt hash (for correlation)
grep "prompt_hash=a1b2c3d4e5f6g7h8" logs/antigravity.log | grep "error_type"
```

## Log Field Reference

All error logs include these structured fields:

| Field | Description | Example |
|-------|-------------|---------|
| `error_type` | Error category | `API_ERROR`, `USER_ERROR`, `SYSTEM_ERROR`, `NETWORK_ERROR` |
| `account_email` | Account used for generation | `user@example.com` |
| `model` | Model ID | `gemini-3-pro-image` |
| `prompt_hash` | SHA256 hash of prompt (first 16 chars) | `a1b2c3d4e5f6g7h8` |
| `generation_time_ms` | Time from request to error (milliseconds) | `1234` |
| `status_code` | HTTP status code | `429`, `400`, `500` |
| `aspect_ratio` | Image aspect ratio (generations only) | `1:1`, `16:9`, `9:16` |
| `quality` | Image quality setting (generations only) | `standard`, `hd` |
| `style` | Image style setting (generations only) | `vivid`, `natural` |
| `operation` | Operation type (edits only) | `image_edit` |
| `size` | Image size requested | `1024x1024` |
| `n` | Number of images requested | `1`, `2`, `4` |
| `safety_threshold` | Safety filter threshold | `OFF`, `BLOCK_ONLY_HIGH`, `BLOCK_MEDIUM_AND_ABOVE` |
| `task_index` | Task index (for concurrent requests) | `0`, `1`, `2` |

## Privacy and Security

**Prompt Hashing**: The system uses SHA256 hashing to log prompts without exposing sensitive content:

- **Privacy-Preserving**: Prompts are hashed (first 16 characters of SHA256 digest)
- **Correlation**: Same prompt → same hash, enabling error correlation
- **No PII**: Original prompts never appear in logs
- **Example**: `"A beautiful sunset"` → hash: `a1b2c3d4e5f6g7h8`

**Secure Logging**:
- Account emails are logged for operational purposes but should be protected
- Logs should be stored with appropriate access controls
- Consider log rotation and retention policies
- Sensitive data (API keys, tokens) are never logged

## Getting Help

If you encounter persistent errors:

1. **Check Logs**: Review structured logs for detailed error information
2. **Check Status**: Verify Gemini API status and your account quotas
3. **Review Configuration**: Ensure safety settings and model mappings are correct
4. **Contact Support**: Provide error logs and prompt hashes for investigation

## Additional Resources

- [Gemini 3 Pro Image Workflow Documentation](../antigravity/workflows/models/gemini/gemini-3-pro-image-workflow.md)
- [API Proxy Configuration Guide](../api-proxy-configuration.md)
- [Google Cloud Status Dashboard](https://status.cloud.google.com/)
