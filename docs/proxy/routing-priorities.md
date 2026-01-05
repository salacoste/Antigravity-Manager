# Routing Priorities (Recommended)

This document captures the current routing priorities we want the proxy to enforce. It is meant to be read together with `docs/proxy/routing.md`.

## Scope and assumptions
- Applies to requests handled by the Google account pool.
- Availability is driven by pool quota snapshots:
  - If a candidate model appears in any pool account with remaining percentage > 0, it is considered available.
  - If any account has unknown quota data, availability is treated as best-effort (unknown models may be considered available).
  - If all accounts report quotas and a model does not appear, it is treated as unavailable.
- z.ai is unaffected by these rules (Claude passthrough has its own mapping).

## Claude protocol (POST /v1/messages)
We prefer one-to-one Claude family routing, with thinking preserved when possible.

Priority by family:
- Opus:
  - Thinking enabled -> `claude-opus-4-5-thinking`
  - Thinking disabled -> `gemini-3-pro-high`
- Sonnet:
  - Thinking enabled -> `claude-sonnet-4-5-thinking`
  - Thinking disabled -> `claude-sonnet-4-5`
- Haiku:
  - Always -> `gemini-3-pro-high`

Availability gating:
- If the chosen target is unavailable, the resolver falls back to the next best candidate from config (family/series/custom), or to system defaults.

Thinking detection:
- `thinking.enabled` is derived from `thinking.type == "enabled"` in the Claude request.
  - Thinking may be auto-disabled when the latest assistant message includes tool_use without a matching thinking block, to avoid upstream 400s.

## OpenAI-like requests (POST /v1/chat/completions, /v1/completions, /v1/responses)
We route OpenAI models to the highest available Claude model, and then fall back in a strict order.

Thinking detection (OpenAI):
- If `thinking.type == "enabled"` -> thinking enabled.
- Else if `reasoning.effort` is present and not `"none"` -> thinking enabled.
- Else if model name contains `"thinking"` -> thinking enabled.
- Else if the model explicitly names a Claude family without thinking -> thinking disabled.
- Default: thinking enabled.

Priority chains (first available wins):
- OpenAI models (gpt/o*):
  - Thinking enabled:
    1) `claude-opus-4-5-thinking`
    2) `claude-sonnet-4-5-thinking`
    3) `gemini-3-pro-high`
    4) `claude-sonnet-4-5`
    5) `gemini-3-flash`
  - Thinking disabled:
    1) `gemini-3-pro-high`
    2) `gemini-3-flash`
- Claude models sent via OpenAI protocol:
  - `claude-opus-*` follows the Opus chain above.
  - `claude-sonnet-*` uses:
    - Thinking enabled: `claude-sonnet-4-5-thinking` -> `gemini-3-pro-high` -> `claude-sonnet-4-5` -> `gemini-3-flash`
    - Thinking disabled: `claude-sonnet-4-5` -> `claude-sonnet-4-5-thinking` -> `gemini-3-pro-high` -> `gemini-3-flash`
  - `claude-haiku-*` uses: `gemini-3-pro-high` -> `gemini-3-flash`

Custom mapping override:
- `proxy.custom_mapping` exact match always overrides the chains above.
  - Recommended: `gemini-3-pro-low -> gemini-3-flash` (when low-tier quotas are missing).

## Gemini protocol (POST /v1beta/*)
Gemini protocol requests are passed through the Google pool directly and are **not** remapped to Claude.

Availability notes:
- If the requested Gemini model is present in pool quotas, it is used as-is.
- If it is not present in quotas, the resolver may fall back to default Gemini routing (based on built-in mappings), or return an upstream error if the model is unsupported.

Recommended usage:
- For explicit Gemini usage, request `gemini-3-pro-high` or `gemini-3-flash` directly.
- Image generation should use `gemini-3-pro-image` (with optional `-2k/-4k` and aspect ratio suffixes).

## Recommended Anthropic mapping keys
These match the current routing intent:
```
claude-opus-family   -> claude-opus-4-5-thinking
claude-sonnet-family -> claude-sonnet-4-5-thinking
claude-haiku-family  -> gemini-3-pro-high
claude-4.5-series    -> claude-opus-4-5-thinking
claude-3.5-series    -> gemini-3-pro-high
```

## Operational guidance
- Refresh account quotas in the UI to keep availability decisions accurate.
- If a model never appears in quotas, it will always be treated as unavailable.
