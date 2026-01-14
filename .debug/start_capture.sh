#!/bin/bash
# Start mitmproxy to capture Antigravity requests

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
CAPTURE_SCRIPT="$SCRIPT_DIR/capture_requests.py"
OUTPUT_DIR="$SCRIPT_DIR/captures"

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 Starting mitmproxy capture"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📂 Output: $OUTPUT_DIR"
echo "🔧 Script: $CAPTURE_SCRIPT"
echo ""
echo "⚙️  Configure your application to use proxy:"
echo "   HTTP Proxy: 127.0.0.1:8888"
echo "   HTTPS Proxy: 127.0.0.1:8888"
echo ""
echo "   Or set environment variables:"
echo "   export HTTP_PROXY=http://127.0.0.1:8888"
echo "   export HTTPS_PROXY=http://127.0.0.1:8888"
echo ""
echo "Press Ctrl+C to stop capturing"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Start mitmdump with our capture script
mitmdump \
    --listen-port 8888 \
    --ssl-insecure \
    -s "$CAPTURE_SCRIPT" \
    "$@"
