// Enhanced logging utilities for debugging Claude model requests
// Use environment variable DEBUG_CLAUDE_REQUESTS=1 to enable

use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;

/// Check if enhanced debugging is enabled
pub fn is_debug_enabled() -> bool {
    std::env::var("DEBUG_CLAUDE_REQUESTS")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
}

/// Log request details before sending
pub fn log_request_details(
    url: &str,
    method: &str,
    headers: &reqwest::header::HeaderMap,
    body: &Value,
) {
    if !is_debug_enabled() {
        return;
    }

    tracing::warn!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    tracing::warn!("🔍 DEBUG: Upstream Request Details");
    tracing::warn!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    tracing::warn!("📍 URL: {}", url);
    tracing::warn!("🔧 Method: {}", method);

    // Log headers
    tracing::warn!("📋 Headers:");
    for (key, value) in headers.iter() {
        if let Ok(v) = value.to_str() {
            // Mask authorization token for security
            if key.as_str().to_lowercase() == "authorization" {
                let masked = if v.len() > 20 {
                    format!("{}...{}", &v[..10], &v[v.len() - 4..])
                } else {
                    "Bearer ***".to_string()
                };
                tracing::warn!("  {}: {}", key, masked);
            } else {
                tracing::warn!("  {} : {}", key, v);
            }
        }
    }

    // Log body (with sensitive data protection)
    tracing::warn!("📦 Body (structure):");
    if let Some(obj) = body.as_object() {
        for (key, value) in obj.iter() {
            match key.as_str() {
                "project" | "requestId" | "userAgent" | "requestType" | "model" => {
                    tracing::warn!("  {}: {:?}", key, value);
                }
                "request" => {
                    if let Some(req_obj) = value.as_object() {
                        tracing::warn!("  request:");
                        for (req_key, _req_val) in req_obj.iter() {
                            tracing::warn!("    - {}", req_key);
                        }
                    }
                }
                _ => {
                    tracing::warn!("  {}: <present>", key);
                }
            }
        }
    }

    // Log model if present
    if let Some(model) = body.get("model").and_then(|m| m.as_str()) {
        tracing::warn!("🤖 Model: {}", model);
    }

    tracing::warn!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

/// Log response details after receiving
pub async fn log_response_details(response: &Response, context: &str) {
    if !is_debug_enabled() {
        return;
    }

    tracing::warn!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    tracing::warn!("📥 DEBUG: Upstream Response Details ({})", context);
    tracing::warn!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    tracing::warn!("🔢 Status: {}", response.status());
    tracing::warn!("📋 Response Headers:");

    for (key, value) in response.headers().iter() {
        if let Ok(v) = value.to_str() {
            tracing::warn!("  {}: {}", key, v);
        }
    }

    tracing::warn!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

/// Log 429 error details with quota information
pub fn log_429_error_details(status: u16, body: &str, headers: &reqwest::header::HeaderMap) {
    if status != 429 {
        return;
    }

    tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    tracing::error!("❌ 429 ERROR: Rate Limit Details");
    tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Check for Retry-After header
    if let Some(retry_after) = headers.get("retry-after") {
        if let Ok(value) = retry_after.to_str() {
            tracing::error!("⏱️  Retry-After: {}", value);
        }
    }

    // Check for quota headers
    let quota_headers = vec![
        "x-goog-quota-user",
        "x-rate-limit-remaining",
        "x-rate-limit-reset",
        "x-quota-limit",
        "x-quota-remaining",
    ];

    tracing::error!("📊 Quota Headers:");
    for header_name in quota_headers {
        if let Some(value) = headers.get(header_name) {
            if let Ok(v) = value.to_str() {
                tracing::error!("  {}: {}", header_name, v);
            }
        }
    }

    // Parse error body
    tracing::error!("📄 Error Body:");
    if let Ok(json) = serde_json::from_str::<Value>(body) {
        if let Some(error) = json.get("error") {
            tracing::error!("  Type: {:?}", error.get("type"));
            tracing::error!("  Message: {:?}", error.get("message"));
            tracing::error!("  Code: {:?}", error.get("code"));

            // Check for specific quota information
            if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                if message.contains("Quota exceeded") {
                    tracing::error!("  ⚠️  QUOTA TYPE: {}", message);
                }
                if message.contains("aiplatform.googleapis.com") {
                    tracing::error!("  ⚠️  VERTEX AI QUOTA");
                }
                if message.contains("per_base_model") {
                    tracing::error!("  ⚠️  MODEL-SPECIFIC QUOTA (Claude/Anthropic)");
                }
            }
        }
    } else {
        tracing::error!("  Raw: {}", body);
    }

    tracing::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

/// Compare request with reference (for debugging)
pub fn compare_with_reference(current_ua: &str, current_request_type: &str) -> Vec<String> {
    let mut suggestions = Vec::new();

    // Reference values from working Google Antigravity
    let reference_version = "1.13.3";
    let reference_platforms = vec!["darwin/arm64", "darwin/amd64", "windows/amd64"];

    // Check version
    if !current_ua.contains(reference_version) {
        suggestions.push(format!(
            "⚠️  User-Agent version mismatch. Expected: {}, Current: {}",
            reference_version, current_ua
        ));
    }

    // Check platform
    let has_valid_platform = reference_platforms.iter().any(|p| current_ua.contains(p));
    if !has_valid_platform {
        suggestions.push(format!(
            "⚠️  User-Agent platform not in reference list: {:?}",
            reference_platforms
        ));
    }

    // Check format
    if !current_ua.starts_with("antigravity/") {
        suggestions.push("⚠️  User-Agent should start with 'antigravity/'".to_string());
    }

    // Check requestType
    if current_request_type != "agent" {
        suggestions.push(format!(
            "💡 Non-standard requestType: '{}' (standard is 'agent')",
            current_request_type
        ));
    }

    suggestions
}

/// Generate diagnostic report
pub fn generate_diagnostic_report(
    user_agent: &str,
    request_type: &str,
    model: &str,
    status_code: u16,
    additional_info: HashMap<String, String>,
) -> String {
    let mut report = String::new();

    report.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    report.push_str("🔬 DIAGNOSTIC REPORT\n");
    report.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    report.push_str(&format!("📅 Timestamp: {:?}\n", chrono::Utc::now()));
    report.push_str(&format!("🤖 Model: {}\n", model));
    report.push_str(&format!("🔧 User-Agent: {}\n", user_agent));
    report.push_str(&format!("📦 Request Type: {}\n", request_type));
    report.push_str(&format!("🔢 Status Code: {}\n", status_code));

    report.push_str("\n💡 Suggestions:\n");
    let suggestions = compare_with_reference(user_agent, request_type);
    if suggestions.is_empty() {
        report.push_str("  ✅ Configuration matches reference\n");
    } else {
        for suggestion in suggestions {
            report.push_str(&format!("  {}\n", suggestion));
        }
    }

    if !additional_info.is_empty() {
        report.push_str("\n📊 Additional Info:\n");
        for (key, value) in additional_info.iter() {
            report.push_str(&format!("  {}: {}\n", key, value));
        }
    }

    report.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_with_reference() {
        let suggestions = compare_with_reference("antigravity/1.11.9 windows/amd64", "agent");
        assert!(!suggestions.is_empty(), "Should detect version mismatch");

        let suggestions = compare_with_reference("antigravity/1.13.3 darwin/arm64", "agent");
        assert!(suggestions.is_empty(), "Should match reference");
    }

    #[test]
    fn test_diagnostic_report_generation() {
        let report = generate_diagnostic_report(
            "antigravity/1.13.3 darwin/arm64",
            "agent",
            "claude-sonnet-4-5",
            200,
            HashMap::new(),
        );
        assert!(report.contains("DIAGNOSTIC REPORT"));
        assert!(report.contains("claude-sonnet-4-5"));
    }
}
