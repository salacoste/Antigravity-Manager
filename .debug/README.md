# Debug Tools

This directory contains tools for debugging HTTP/HTTPS traffic between Antigravity Manager and Google Cloud APIs.

## Quick Start

### Start Traffic Capture
```bash
./start_capture.sh
```

This launches `mitmproxy` on port 8888 to intercept and log requests to `googleapis.com`.

### Configure Application to Use Proxy
```bash
export HTTP_PROXY=http://127.0.0.1:8888
export HTTPS_PROXY=http://127.0.0.1:8888

# Then run your application
cd ..
npm run tauri dev
```

### View Captured Traffic
Captured requests and responses are saved in `./captures/` directory as JSON files:
- `request_TIMESTAMP_XXX.json`
- `response_TIMESTAMP_XXX.json`

## Files

### `capture_requests.py`
Python addon for mitmproxy that:
- Filters requests to `googleapis.com`
- Focuses on `generateContent` API calls
- Logs full request/response details to JSON files
- Masks sensitive authorization tokens

### `start_capture.sh`
Wrapper script that:
- Creates `./captures/` directory
- Launches `mitmdump` with `capture_requests.py` addon
- Listens on port 8888
- Uses `--ssl-insecure` for self-signed certificates

### `CAPTURE_INSTRUCTIONS.md`
Detailed step-by-step instructions for:
- Capturing traffic from original Google Antigravity
- Capturing traffic from Antigravity Manager
- Comparing the two to find differences

### `TESTING_PLAN.md`
Testing strategies document including:
- Approach A: Quick test with application logs
- Approach B: Deep analysis with mitmproxy
- Diagnostic workflows

## Requirements

- **mitmproxy**: Install via `brew install mitmproxy`
- **Python 3.x**: Required for mitmproxy addon script

## Use Cases

### 1. Compare Requests
When you need to see exact differences between working and non-working implementations:

1. Capture traffic from working Google Antigravity
2. Capture traffic from Antigravity Manager
3. Compare the JSON files to identify differences in:
   - Headers (User-Agent, X-Goog-*, etc.)
   - Request body structure
   - Model names and formats
   - Query parameters

### 2. Debug 429 Errors
When receiving rate limit errors:

1. Start capture
2. Make failing request
3. Examine response JSON for:
   - Error message details
   - Quota type being limited
   - Suggested retry delays

### 3. Validate Fixes
After implementing a fix:

1. Capture before-fix traffic
2. Apply fix
3. Capture after-fix traffic
4. Compare to verify changes

## Security Notes

⚠️ **Sensitive Data**: Captured files may contain access tokens and request data.

- Files in `./captures/` are gitignored
- Authorization tokens are partially masked in logs
- Do not commit captured traffic to version control
- Clear `./captures/` directory after debugging

## Documentation

For comprehensive debugging guide, see:
[`../docs/debugging-claude-models.md`](../docs/debugging-claude-models.md)
