#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/test_claude_pool.sh <model|both> [count]

Examples:
  scripts/test_claude_pool.sh claude-sonnet-4-5 5
  scripts/test_claude_pool.sh claude-opus-4-5-thinking 5
  scripts/test_claude_pool.sh both 5

Config resolution:
  - Reads ~/.antigravity_tools/gui_config.json for proxy port/api_key unless overridden:
      PROXY_PORT=8045 PROXY_API_KEY=sk-... scripts/test_claude_pool.sh both 3
USAGE
}

MODEL="${1:-}"
COUNT="${2:-5}"

if [[ -z "$MODEL" || "$MODEL" == "-h" || "$MODEL" == "--help" ]]; then
  usage
  exit 0
fi

CONFIG_PATH="${CONFIG_PATH:-$HOME/.antigravity_tools/gui_config.json}"

if [[ -z "${PROXY_PORT:-}" ]]; then
  if [[ -f "$CONFIG_PATH" ]]; then
    PROXY_PORT="$(jq -r '.proxy.port // empty' "$CONFIG_PATH" 2>/dev/null || true)"
  fi
fi
if [[ -z "${PROXY_API_KEY:-}" ]]; then
  if [[ -f "$CONFIG_PATH" ]]; then
    PROXY_API_KEY="$(jq -r '.proxy.api_key // empty' "$CONFIG_PATH" 2>/dev/null || true)"
  fi
fi

if [[ -z "${PROXY_PORT:-}" || -z "${PROXY_API_KEY:-}" ]]; then
  echo "Missing PROXY_PORT/PROXY_API_KEY and couldn't read them from $CONFIG_PATH" >&2
  exit 2
fi

BASE_URL="${PROXY_BASE_URL:-http://127.0.0.1:${PROXY_PORT}}"
URL="${BASE_URL%/}/v1/messages"

payload_for_model() {
  local model="$1"
  cat <<JSON
{"model":"$model","max_tokens":96,"messages":[{"role":"user","content":"Ping. Reply with: OK"}]}
JSON
}

one() {
  local model="$1"
  local hdr tmp
  hdr="$(mktemp)"
  tmp="$(mktemp)"

  local code
  code="$(curl -sS -D "$hdr" -o "$tmp" -w '%{http_code}' \
    -H "Authorization: Bearer ${PROXY_API_KEY}" \
    -H "Content-Type: application/json" \
    -d "$(payload_for_model "$model")" \
    "$URL" || true)"

  local email mapped provider
  email="$(rg -i '^x-account-email:' "$hdr" | head -n 1 | sed -E 's/^[^:]+:[[:space:]]*//I' || true)"
  mapped="$(rg -i '^x-mapped-model:' "$hdr" | head -n 1 | sed -E 's/^[^:]+:[[:space:]]*//I' || true)"
  provider="$(rg -i '^x-antigravity-provider:' "$hdr" | head -n 1 | sed -E 's/^[^:]+:[[:space:]]*//I' || true)"

  printf "model=%s status=%s" "$model" "$code"
  [[ -n "$mapped" ]] && printf " mapped=%s" "$mapped"
  [[ -n "$provider" ]] && printf " provider=%s" "$provider"
  [[ -n "$email" ]] && printf " account=%s" "$email"
  printf "\n"

  if [[ "$code" -ge 400 ]]; then
    echo "---- error body (first 300 chars) ----"
    head -c 300 "$tmp" || true
    echo
    echo "-------------------------------------"
  fi

  rm -f "$hdr" "$tmp"
}

models=()
if [[ "$MODEL" == "both" ]]; then
  models=("claude-sonnet-4-5" "claude-opus-4-5-thinking")
else
  models=("$MODEL")
fi

for m in "${models[@]}"; do
  echo "== Testing $m ($COUNT requests) =="
  for ((i=1; i<=COUNT; i++)); do
    one "$m"
    sleep 0.2
  done
done

