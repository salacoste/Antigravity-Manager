# Quota Exhaustion Failover - Epic & Design Notes

## Goal
Prevent client hangs and improve routing when model quota hits 0% on specific accounts. When a selected account is at 0% for the requested model, skip it and select another. If all accounts are at 0% for that model, fall back to the next mapped model, or return a clear error (429/503) and end streaming properly.

## Scope
Applies to proxy request handling for Claude/OpenAI/Gemini routes, including streaming responses and sticky session behavior. This is a design/spec document only; no code changes in this step.

## Current Architecture (Relevant Pieces)
- `TokenManager` loads accounts and their `quota.models` percentages from account JSON.
- `ModelAvailability` is a pool-wide summary used in routing decisions.
- Request handlers pick a model using routing rules + availability checks, then select a token (account) via `TokenManager.get_token`.
- Sticky sessions can bind a session to an account; 60s global reuse exists when no session_id.
- Streaming requests use SSE; errors must be explicitly emitted or the client will hang.

## Observed Problem
- A request can be sent to an account that has 0% quota for the requested model.
- Upstream can return error or stall; client may hang if SSE is not closed with an error.

## Desired Behavior (Agreed)
1) Account-level quota check:
   - If selected account has 0% for the requested model, do not send the request. Select another account.
2) All accounts at 0% for the requested model:
   - If routing rules allow a fallback model, use it.
   - Otherwise, return a clear error (429/503) and end streaming.
3) Streaming:
   - If refusing the request, send an SSE error event and terminate the stream.
4) Sticky sessions:
   - If the bound account is 0% for the model, unbind and pick another.

## Policy Decisions (Fixed)
- Low-quota threshold is global (5%) for all model families.
- Thinking is always a higher priority than quota savings (no forced thinking disable).
- Unknown quota accounts are not used for routing until quota is known.
- Quota refresh attempts for unknown accounts are throttled to <= 1/min.
- If all eligible accounts are unknown quota, requests are rejected (or fallback is attempted first if it has known quota).

## Unknown Quota (Definition + Policy)
Unknown quota is when `quota.models` is missing. This is not the same as disabled accounts (disabled/proxy_disabled files are skipped during load).

Root causes:
- Quota refresh failed or timed out; account JSON lacks `quota.models`.
- Upstream did not provide per-model percentages.
- Quota has not been fetched yet for the account.

Policy:
- Unknown quota accounts are not used for request routing.
- They remain in the pool and should be retried for quota fetch (throttled to <= 1/min).
- UI should surface this state clearly to operators/admins.
- If all accounts for a requested model are unknown quota, the request must not route to them.

## Proposed Design (High Level)
### A) Account-level quota gate
- Extend token selection logic to consider model-specific availability per account.
- Before sending request:
  - If account has model percentage <= 5%, skip it and choose another (unless all models are <= 5%).
  - If account has unknown quota, do not select it for routing.
  - Model name matching should use the same candidate expansion logic as routing (aliases, -thinking, -online).

### B) Fallback flow when model is exhausted
- If no account can serve the requested model:
  - Attempt model fallback using existing routing rules (same thinking preference).
  - Re-run account selection on fallback model.
  - If still none, return 429/503 with a clear error message.
  - If fallback model does not support thinking, disable thinking for fallback only.

### C) Streaming error termination
- For SSE requests, emit an error event and close the stream on rejection:
  - `event: error` with a JSON body describing the cause (e.g., "no accounts with quota for model").

### D) Routing order and decision points
1) Parse request and derive thinking preference.
2) Resolve target model via mapping rules (custom > family > system).
3) Apply account-level quota gate for the resolved model.
4) If no eligible account:
   - attempt fallback model resolution and repeat quota gate.
   - if still none, return 429/503 (SSE error for streaming).
5) Bind or re-bind sticky sessions based on the chosen account.

### D) Sticky session behavior
- If the session-bound account has 0% for the requested model:
  - Unbind session from that account.
  - Retry selection with next available account.

## Impact & Compatibility
- Preserves existing routing rules and model mappings.
- Adds a guardrail at account selection time, reducing upstream errors and client hangs.
- Requires careful ordering to avoid breaking:
  - thinking preference
  - current fallback rules
  - session stickiness
  - unknown quota quarantine logic
  - background task downgrade rules (must still obey quota gate)

## UI/Operator Feedback
- Mark accounts with unknown quota status in the UI.
- Provide a concise reason (e.g., "quota not yet fetched" or "quota refresh failed").
- Optionally include a last-attempt timestamp for quota refresh.

## Error Contract (Streaming + Non-Streaming)
Define explicit error payloads to prevent client hangs and standardize responses.

### Claude-compatible Errors
- **Non-streaming**: HTTP 429/503 with JSON body:
  ```json
  {
    "type": "error",
    "error": {
      "type": "overloaded_error",
      "message": "No available accounts for model: <model> (quota exhausted/unknown)."
    }
  }
  ```
- **Streaming (SSE)**: send `event: error` with JSON body and close:
  ```json
  {
    "type": "error",
    "error": {
      "type": "overloaded_error",
      "message": "No available accounts for model: <model> (quota exhausted/unknown)."
    }
  }
  ```

### OpenAI-compatible Errors
- **Non-streaming**: HTTP 429/503 with JSON body:
  ```json
  {
    "error": {
      "message": "No available accounts for model: <model> (quota exhausted/unknown).",
      "type": "insufficient_quota",
      "code": "quota_exhausted"
    }
  }
  ```
- **Streaming (SSE)**: send `event: error` with JSON body and close:
  ```json
  {
    "error": {
      "message": "No available accounts for model: <model> (quota exhausted/unknown).",
      "type": "insufficient_quota",
      "code": "quota_exhausted"
    }
  }
  ```

### Gemini-compatible Errors
- **Non-streaming**: HTTP 429/503 with JSON body:
  ```json
  {
    "error": {
      "code": 429,
      "status": "RESOURCE_EXHAUSTED",
      "message": "No available accounts for model: <model> (quota exhausted/unknown)."
    }
  }
  ```
- **Streaming (SSE)**: send `event: error` with JSON body and close:
  ```json
  {
    "error": {
      "code": 429,
      "status": "RESOURCE_EXHAUSTED",
      "message": "No available accounts for model: <model> (quota exhausted/unknown)."
    }
  }
  ```

## Telemetry/Logging (Recommended)
- Log when a token is skipped due to 0% model quota.
- Log when a token is skipped due to <= 5% quota in the presence of healthier alternatives.
- Log when fallback is triggered because all accounts are at 0%.
- Log when all accounts are unknown quota for a requested model.
- Log when an SSE error is sent.

## Epic
"Quota-aware account selection and safe failover"

### Acceptance Criteria
- Requests never get sent to accounts with 0% model quota.
- If all accounts are at 0% for a model, either fallback is used or a clear error is returned (429/503).
- Streaming requests do not hang; errors are sent explicitly.
- Sticky sessions are re-bound when their account is out of quota.
- Unknown quota accounts are never selected for routing.

## User Stories
1) **Account-level skip**
   - As a proxy user, if the selected account has 0% for the requested model, the proxy selects a different account without failing the request.
   - Acceptance: no upstream call made with account at 0% for that model.

2) **Model-level exhaustion fallback**
   - As a proxy user, when all accounts are 0% for the requested model, the proxy falls back to the next allowed model (keeping thinking preference).
   - Acceptance: fallback is used if available; otherwise error 429/503.

3) **SSE error handling**
   - As a streaming client, when no account can serve the request, I receive an error event and the stream closes promptly.
   - Acceptance: no hanging streams on quota exhaustion.

4) **Sticky session resilience**
   - As a user with sticky sessions, if my bound account is out of quota for the model, I am re-bound to a valid account.
   - Acceptance: session binding is removed and new account is selected.

5) **Unknown quota policy**
   - As an operator, I can see that an account's quota is unknown and it is temporarily not used for routing.
   - Acceptance: unknown quota accounts are excluded from routing and surfaced in UI, with refresh attempts throttled to <= 1/min.

## Implementation Notes (for later)
- Extend token selection to accept `requested_model` and enforce per-account quota.
- Add a small helper to evaluate account quota status for a model.
- Reuse existing model candidate expansion rules when checking quota (aliases, -thinking, -online).
- Add SSE error emission helper shared across handlers.
- Ensure fallback routing still respects `thinking` preference and existing mapping rules.
- Define a consistent SSE error payload for each protocol (Claude/OpenAI) to avoid client hangs.
