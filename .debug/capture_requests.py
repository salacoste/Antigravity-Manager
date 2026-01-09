#!/usr/bin/env python3
"""
mitmproxy addon for capturing Antigravity requests to Google APIs
Usage: mitmdump -s capture_requests.py
"""

import json
import base64
from datetime import datetime
from mitmproxy import http
import os

# Create output directory
OUTPUT_DIR = "/Users/r2d2/Documents/Code_Projects/00_mcp/Antigravity-Manager/.debug/captures"
os.makedirs(OUTPUT_DIR, exist_ok=True)

request_counter = 0

def request(flow: http.HTTPFlow) -> None:
    """Intercept and log requests to googleapis.com"""
    global request_counter

    # Only capture requests to googleapis.com
    if "googleapis.com" not in flow.request.pretty_host:
        return

    # Only capture Claude-related requests (containing streamGenerateContent or generateContent)
    if "generateContent" not in flow.request.path:
        return

    request_counter += 1
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")

    # Extract request details
    request_data = {
        "timestamp": timestamp,
        "counter": request_counter,
        "method": flow.request.method,
        "url": flow.request.pretty_url,
        "host": flow.request.pretty_host,
        "path": flow.request.path,
        "headers": dict(flow.request.headers),
        "body": None,
        "body_raw": None,
    }

    # Try to parse body as JSON
    if flow.request.content:
        try:
            request_data["body"] = json.loads(flow.request.content.decode('utf-8', errors='replace'))
        except:
            request_data["body_raw"] = base64.b64encode(flow.request.content).decode('utf-8')

    # Save to file
    filename = f"{OUTPUT_DIR}/request_{timestamp}_{request_counter:03d}.json"
    with open(filename, 'w') as f:
        json.dump(request_data, f, indent=2, ensure_ascii=False)

    print(f"✓ Captured request #{request_counter}: {flow.request.method} {flow.request.path}")
    print(f"  Saved to: {filename}")

    # Print key information
    if "user-agent" in request_data["headers"]:
        print(f"  User-Agent: {request_data['headers']['user-agent']}")

    if request_data["body"] and "model" in request_data["body"]:
        print(f"  Model: {request_data['body']['model']}")

    if request_data["body"] and "requestType" in request_data["body"]:
        print(f"  Request Type: {request_data['body']['requestType']}")


def response(flow: http.HTTPFlow) -> None:
    """Intercept and log responses from googleapis.com"""
    global request_counter

    # Only capture responses from googleapis.com
    if "googleapis.com" not in flow.request.pretty_host:
        return

    if "generateContent" not in flow.request.path:
        return

    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")

    # Extract response details
    response_data = {
        "timestamp": timestamp,
        "counter": request_counter,
        "status_code": flow.response.status_code,
        "headers": dict(flow.response.headers),
        "body": None,
        "body_raw": None,
    }

    # Try to parse body as JSON
    if flow.response.content:
        try:
            response_data["body"] = json.loads(flow.response.content.decode('utf-8', errors='replace'))
        except:
            # For SSE streams, just capture first 1000 bytes
            response_data["body_raw"] = flow.response.content[:1000].decode('utf-8', errors='replace')

    # Save to file
    filename = f"{OUTPUT_DIR}/response_{timestamp}_{request_counter:03d}.json"
    with open(filename, 'w') as f:
        json.dump(response_data, f, indent=2, ensure_ascii=False)

    status_emoji = "✅" if flow.response.status_code == 200 else "❌"
    print(f"{status_emoji} Response #{request_counter}: {flow.response.status_code}")
    print(f"  Saved to: {filename}")

    # Print error details if present
    if flow.response.status_code == 429:
        print(f"  ⚠️  429 ERROR - Rate Limited!")
        if response_data["body"] and "error" in response_data["body"]:
            print(f"  Error: {response_data['body']['error']}")


print("=" * 60)
print("🔍 Antigravity Request Capture Started")
print("=" * 60)
print(f"Output directory: {OUTPUT_DIR}")
print("Waiting for requests to googleapis.com...")
print("=" * 60)
