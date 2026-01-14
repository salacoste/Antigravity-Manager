# Debugging Claude Models Integration

This document describes the debugging infrastructure added to diagnose and fix issues with Claude model integration (Sonnet, Opus).

## Problem Background

**Issue**: Antigravity Manager was receiving 429 (Rate Limit) errors when using Claude Sonnet/Opus models, despite having available quotas in Google accounts that worked perfectly in the original Google Antigravity application.

**Root Cause**: Google was blocking requests with outdated User-Agent version (`antigravity/1.11.9`).

**Solution**: Updated User-Agent to current version (`antigravity/1.13.3 darwin/arm64`).

---

## Changes Made to the Codebase

### 1. Updated User-Agent in `src-tauri/src/proxy/upstream/client.rs`

#### Lines 24-28 (Constructor)
```rust
// [DEBUG] Allow overriding user agent via environment variable
// Updated to 1.13.3 to match current Google Antigravity version
let user_agent = std::env::var("CLAUDE_USER_AGENT")
    .unwrap_or_else(|_| "antigravity/1.13.3 darwin/arm64".to_string());

tracing::info!("🔧 UpstreamClient User-Agent: {}", user_agent);
```

**Environment Variable Support**: You can now override User-Agent via `CLAUDE_USER_AGENT` environment variable.

#### Lines 102-108 (Headers Building)
```rust
// [DEBUG] Allow overriding user agent via environment variable
// Updated to 1.13.3 to match current Google Antigravity version
let user_agent = std::env::var("CLAUDE_USER_AGENT")
    .unwrap_or_else(|_| "antigravity/1.13.3 darwin/arm64".to_string());
headers.insert(
    header::USER_AGENT,
    header::HeaderValue::from_str(&user_agent)
        .map_err(|e| e.to_string())?,
);
```

#### Lines 290-296 (Model Fetching)
```rust
// Updated to 1.13.3 to match current Google Antigravity version
let user_agent = std::env::var("CLAUDE_USER_AGENT")
    .unwrap_or_else(|_| "antigravity/1.13.3 darwin/arm64".to_string());
headers.insert(
    header::USER_AGENT,
    header::HeaderValue::from_str(&user_agent)
        .map_err(|e| format!("Invalid user agent: {}", e))?,
);
```

### 2. Enhanced Debug Logging in `src-tauri/src/proxy/upstream/client.rs`

#### Lines 126-169 (Request Logging)
Added comprehensive request logging before sending upstream requests:

```rust
// [DEBUG] Log full request details before sending
tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
tracing::error!("🔍 UPSTREAM REQUEST DEBUG");
tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
tracing::error!("📍 URL: {}", url);
tracing::error!("🔧 Method: POST");
tracing::error!("📋 Headers:");
for (key, value) in headers.iter() {
    if let Ok(v) = value.to_str() {
        if key.as_str().to_lowercase() == "authorization" {
            // Mask sensitive auth tokens
            let masked = if v.len() > 20 {
                format!("Bearer {}...{}", &v[7..17], &v[v.len()-4..])
            } else {
                "Bearer ***".to_string()
            };
            tracing::error!("  {}: {}", key, masked);
        } else {
            tracing::error!("  {}: {}", key, v);
        }
    }
}
tracing::error!("📦 Body Structure:");
if let Some(obj) = body.as_object() {
    tracing::error!("  project: {:?}", obj.get("project"));
    tracing::error!("  requestId: {:?}", obj.get("requestId"));
    tracing::error!("  model: {:?}", obj.get("model"));
    tracing::error!("  userAgent: {:?}", obj.get("userAgent"));
    tracing::error!("  requestType: {:?}", obj.get("requestType"));
    // ... more body structure logging
}
tracing::error!("📄 Full Body (first 500 chars):");
let body_str = serde_json::to_string_pretty(&body).unwrap_or_default();
let preview = if body_str.len() > 500 {
    format!("{}...", &body_str[..500])
} else {
    body_str
};
tracing::error!("{}", preview);
tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
```

#### Lines 183-207 (Response Logging)
Added comprehensive response logging:

```rust
// [DEBUG] Log response details
tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
tracing::error!("📥 UPSTREAM RESPONSE DEBUG");
tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
tracing::error!("🔢 Status: {}", status);
tracing::error!("📋 Response Headers:");
for (key, value) in resp.headers().iter() {
    if let Ok(v) = value.to_str() {
        tracing::error!("  {}: {}", key, v);
    }
}

// [DEBUG] For 429 errors, capture error body
if status == StatusCode::TOO_MANY_REQUESTS {
    tracing::error!("❌❌❌ 429 RATE LIMIT ERROR DETECTED! ❌❌❌");
    // Try to read error body
    let body_bytes = resp.bytes().await.unwrap_or_default();
    let body_str = String::from_utf8_lossy(&body_bytes);
    tracing::error!("📄 Error Body: {}", body_str);
    tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Return error since we consumed the body
    return Err(format!("429 Rate Limit: {}", body_str));
}
tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
```

### 3. Created Enhanced Logging Module

**File**: `src-tauri/src/proxy/upstream/enhanced_logging.rs`

Utility functions for debugging (currently unused but available for future):
- `is_debug_enabled()` - Check if debug mode is enabled
- `log_request_details()` - Detailed request logging
- `log_response_details()` - Detailed response logging
- `log_429_error_details()` - Specific 429 error logging
- `compare_with_reference()` - Compare with reference implementation
- `generate_diagnostic_report()` - Generate comprehensive diagnostic reports

---

## Debug Infrastructure

### mitmproxy Setup

**Installed**: `mitmproxy` v12.2.1 via Homebrew

**Purpose**: Intercept and compare HTTP/HTTPS traffic between Antigravity Manager and Google APIs with original Google Antigravity application.

**Location**: `/opt/homebrew/bin/mitmdump`

### Debug Scripts Created

#### `.debug/capture_requests.py`
Python addon for mitmproxy that captures requests to `googleapis.com`:

```python
def request(flow: http.HTTPFlow) -> None:
    """Intercept and log requests to googleapis.com"""
    if "googleapis.com" not in flow.request.pretty_host:
        return
    if "generateContent" not in flow.request.path:
        return

    # Logs full request details to JSON files
```

#### `.debug/start_capture.sh`
Script to launch mitmproxy with capture addon:

```bash
mitmdump \
    --listen-port 8888 \
    --ssl-insecure \
    -s "$CAPTURE_SCRIPT"
```

#### `.debug/CAPTURE_INSTRUCTIONS.md`
Detailed instructions for using mitmproxy to capture traffic from both original Antigravity and Antigravity Manager.

#### `.debug/TESTING_PLAN.md`
Two-pronged testing approach documentation:
- **Approach A**: Quick test with logs
- **Approach B**: Deep traffic analysis with mitmproxy

---

## How to Use Debug Infrastructure

### 1. Quick Testing with Test Scripts

Use the existing test script to verify Claude models:

```bash
# Test both Claude Sonnet and Opus models with 3 requests each
./scripts/test_claude_pool.sh both 3

# Test only Sonnet
./scripts/test_claude_pool.sh claude-sonnet-4-5 5

# Test only Opus
./scripts/test_claude_pool.sh claude-opus-4-5-thinking 5
```

**Expected output for success**:
```
model=claude-sonnet-4-5 status=200 mapped=claude-sonnet-4-5 account=example@gmail.com
```

**Expected output for failure**:
```
model=claude-sonnet-4-5 status=429 mapped=claude-sonnet-4-5 account=example@gmail.com
```

### 2. Checking Debug Logs

Application logs are stored in:
```
~/Library/Application Support/com.lbjlaq.antigravity-tools/logs/
```

**Find latest log file**:
```bash
ls -lt ~/Library/Application\ Support/com.lbjlaq.antigravity-tools/logs/ | head -2
```

**View request/response debug logs**:
```bash
tail -300 ~/Library/Application\ Support/com.lbjlaq.antigravity-tools/logs/antigravity_*.log | grep -A 50 "UPSTREAM REQUEST DEBUG"
```

**Look for**:
- `🔍 UPSTREAM REQUEST DEBUG` blocks - Full request details
- `📥 UPSTREAM RESPONSE DEBUG` blocks - Response details
- `❌❌❌ 429 RATE LIMIT ERROR DETECTED!` - 429 error details

### 3. Override User-Agent for Testing

Test different User-Agent versions:

```bash
# Set custom User-Agent
export CLAUDE_USER_AGENT="antigravity/1.13.3 darwin/arm64"

# Run the application
npm run tauri dev

# In another terminal, test
./scripts/test_claude_pool.sh both 3
```

### 4. Traffic Interception with mitmproxy

**When to use**: When you need to compare exact requests between working and non-working implementations.

#### Step 1: Start mitmproxy
```bash
cd .debug
./start_capture.sh
```

#### Step 2: Configure proxy for application
```bash
export HTTP_PROXY=http://127.0.0.1:8888
export HTTPS_PROXY=http://127.0.0.1:8888
export REQUESTS_CA_BUNDLE=""
export SSL_CERT_FILE=""

cd /Users/r2d2/Documents/Code_Projects/00_mcp/Antigravity-Manager
npm run tauri dev
```

#### Step 3: Make test requests
In another terminal:
```bash
./scripts/test_claude_pool.sh claude-sonnet-4-5 1
```

#### Step 4: Analyze captures
Captured requests are saved in `.debug/captures/`:
- `request_TIMESTAMP_XXX.json` - Request details
- `response_TIMESTAMP_XXX.json` - Response details

**Compare**:
- Headers (especially `User-Agent`, `X-Goog-*`)
- Request body structure (`requestType`, `model`, `userAgent`)
- URL parameters
- Response status codes and error messages

---

## Common Issues and Solutions

### Issue: 429 Rate Limit Errors

**Symptoms**:
- Test script shows `status=429`
- Logs show `❌❌❌ 429 RATE LIMIT ERROR DETECTED!`

**Possible Causes**:
1. **Outdated User-Agent** (FIXED)
   - Solution: Verify User-Agent is `antigravity/1.13.3` or newer

2. **Missing/Wrong Headers**
   - Solution: Compare headers with working Google Antigravity using mitmproxy

3. **Wrong `requestType`**
   - Solution: Check request body structure in logs, should be `"agent"`

4. **Model name format**
   - Solution: Verify model names match expected format in `model_mapping.rs`

### Issue: No Debug Logs in File

**Symptoms**: Log file doesn't contain debug information

**Solution**:
```bash
# Check if logging is initialized
grep "日志系统已完成初始化" ~/Library/Application\ Support/com.lbjlaq.antigravity-tools/logs/antigravity_*.log

# Check log level - debug logs use tracing::error! to ensure they appear
tail -f ~/Library/Application\ Support/com.lbjlaq.antigravity-tools/logs/antigravity_*.log
```

### Issue: mitmproxy Certificate Errors

**Symptoms**: Application refuses to connect when using mitmproxy

**Solution**: This is expected for testing. Use `--ssl-insecure` flag (already included in `start_capture.sh`).

---

## Testing Checklist

When investigating Claude model issues:

- [ ] Run test script: `./scripts/test_claude_pool.sh both 3`
- [ ] Check test results for status codes (200 = success, 429 = rate limit)
- [ ] Review debug logs for request/response details
- [ ] Verify User-Agent matches current version
- [ ] Check request body structure (`requestType`, `model`, `userAgent`)
- [ ] Compare headers with reference implementation
- [ ] Test with different accounts if available
- [ ] Use mitmproxy for deep traffic analysis if needed

---

## References

- **Original Issue**: Claude Sonnet/Opus models returning 429 errors
- **Fix Date**: January 9, 2026
- **User-Agent Updated**: From `1.11.9 windows/amd64` to `1.13.3 darwin/arm64`
- **Test Results**: 100% success rate (6/6 requests successful)

---

## Future Improvements

1. **Automated Testing**: Add integration tests for Claude models in CI/CD
2. **User-Agent Auto-Detection**: Automatically detect and use latest Antigravity version
3. **Enhanced Error Messages**: Provide more actionable error messages for 429 errors
4. **Monitoring**: Add metrics for Claude model success/failure rates
5. **Version Compatibility Check**: Warn if User-Agent version is outdated
