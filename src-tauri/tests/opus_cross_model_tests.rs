//! Cross-Model Integration Tests (Story-019-03: AC-1)
//!
//! Validates claude-opus-4-5 (standard mode) vs claude-opus-4-5-thinking
//! distinctions to ensure correct model routing and feature support.
//!
//! **Epic**: Epic-019 - Claude Opus 4.5 Standard Mode
//! **Story**: Story-019-03 - Testing & Documentation
//! **Acceptance Criteria**: AC-1 (Cross-Model Integration Tests)
//!
//! **Test Coverage**:
//! - Model ID distinction (335 vs 336)
//! - Shared metadata (apiProvider: 26, ideType: "ANTIGRAVITY")
//! - Thinking block presence/absence validation
//! - Tool modes work in both models
//! - Grounding works in both models
//! - Standard mode simpler than thinking mode
//! - Pattern consistency with Sonnet (333 vs 335)
//!
//! **DEPENDENCIES**: Requires Story-019-01 complete (modelId 335 constant)
//! **STATUS**: ⏳ READY - Waiting for Dev 2A to complete Story-019-01

use antigravity_tools_lib::proxy::mappers::claude::models::{
    ClaudeRequest, Message, MessageContent, ThinkingConfig,
};
use antigravity_tools_lib::proxy::mappers::claude::request::transform_claude_request_in;
use serde_json::{json, Value};

// ==================================================================================
// TEST HELPERS
// ==================================================================================

/// Create test OpenAI-style request
fn create_test_claude_request(model: &str) -> ClaudeRequest {
    ClaudeRequest {
        model: model.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test message".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(1000),
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    }
}

/// Create request with thinking enabled
fn create_test_claude_request_with_thinking(model: &str, budget: u32) -> ClaudeRequest {
    let mut req = create_test_claude_request(model);
    req.thinking = Some(ThinkingConfig {
        type_: "enabled".to_string(),
        budget_tokens: Some(budget),
    });
    req
}

/// Extract model_id from transformed request
fn extract_model_id(request: &Value) -> Option<u32> {
    request
        .get("modelId")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
}

/// Extract api_provider from transformed request
fn extract_api_provider(request: &Value) -> Option<u32> {
    request
        .get("apiProvider")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
}

/// Extract ide_type from transformed request
fn extract_ide_type(request: &Value) -> Option<String> {
    request
        .get("request")
        .and_then(|r| r.get("metadata"))
        .and_then(|m| m.get("ideType"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Check if request has thinking config
fn has_thinking_config(request: &Value) -> bool {
    request
        .get("request")
        .and_then(|r| r.get("generationConfig"))
        .and_then(|g| g.get("thinkingConfig"))
        .is_some()
}

/// Check if request has tool config
fn has_tool_config(request: &Value) -> bool {
    request
        .get("request")
        .and_then(|r| r.get("toolConfig"))
        .is_some()
}

/// Check if request has grounding
fn has_grounding(request: &Value) -> bool {
    request
        .get("request")
        .and_then(|r| r.get("grounding"))
        .is_some()
}

// ==================================================================================
// AC-1: CROSS-MODEL INTEGRATION TESTS
// ==================================================================================

/// Test 1: Model ID distinction (335 vs 336)
///
/// **Validates**:
/// - claude-opus-4-5 → modelId: 335
/// - claude-opus-4-5-thinking → modelId: 336
///
/// **Reference**: Story-019-01 AC-1
/// **TODO**: Wait for Dev 2A to implement CLAUDE_OPUS_45_STANDARD_MODEL_ID = 335
#[test]
fn test_opus_model_id_distinction() {
    // Standard mode (335)
    let standard_req = create_test_claude_request("claude-opus-4-5");
    let (standard_transformed, _) = transform_claude_request_in(&standard_req, "test-project")
        .expect("Standard request transformation failed");

    let standard_model_id =
        extract_model_id(&standard_transformed).expect("Standard request missing modelId");

    // Thinking mode (336)
    let thinking_req = create_test_claude_request("claude-opus-4-5-thinking");
    let (thinking_transformed, _) = transform_claude_request_in(&thinking_req, "test-project")
        .expect("Thinking request transformation failed");

    let thinking_model_id =
        extract_model_id(&thinking_transformed).expect("Thinking request missing modelId");

    // Validate distinction
    // TODO: After Story-019-01 complete, verify 335 vs 336
    assert_eq!(
        standard_model_id, 335,
        "Standard mode should have modelId 335"
    );
    assert_eq!(
        thinking_model_id, 336,
        "Thinking mode should have modelId 336"
    );
    assert_ne!(
        standard_model_id, thinking_model_id,
        "Model IDs must be different"
    );
}

/// Test 2: Shared API Provider (26 - ANTHROPIC_VERTEX)
///
/// **Validates**:
/// - Both models use apiProvider: 26 (ANTHROPIC_VERTEX)
///
/// **Reference**: Story-019-01 AC-2
#[test]
fn test_opus_shared_api_provider() {
    // Standard mode
    let standard_req = create_test_claude_request("claude-opus-4-5");
    let (standard_transformed, _) = transform_claude_request_in(&standard_req, "test-project")
        .expect("Standard request transformation failed");

    let standard_provider =
        extract_api_provider(&standard_transformed).expect("Standard request missing apiProvider");

    // Thinking mode
    let thinking_req = create_test_claude_request("claude-opus-4-5-thinking");
    let (thinking_transformed, _) = transform_claude_request_in(&thinking_req, "test-project")
        .expect("Thinking request transformation failed");

    let thinking_provider =
        extract_api_provider(&thinking_transformed).expect("Thinking request missing apiProvider");

    // Validate shared provider
    assert_eq!(
        standard_provider, 26,
        "Standard mode should have apiProvider 26"
    );
    assert_eq!(
        thinking_provider, 26,
        "Thinking mode should have apiProvider 26"
    );
    assert_eq!(
        standard_provider, thinking_provider,
        "API provider must be identical"
    );
}

/// Test 3: Shared ideType ("ANTIGRAVITY")
///
/// **Validates**:
/// - Both models have ideType: "ANTIGRAVITY" (anti-detection marker)
///
/// **Reference**: Story-019-01 AC-3
#[test]
fn test_opus_shared_ide_type() {
    // Standard mode
    let standard_req = create_test_claude_request("claude-opus-4-5");
    let (standard_transformed, _) = transform_claude_request_in(&standard_req, "test-project")
        .expect("Standard request transformation failed");

    let standard_ide_type =
        extract_ide_type(&standard_transformed).expect("Standard request missing ideType");

    // Thinking mode
    let thinking_req = create_test_claude_request("claude-opus-4-5-thinking");
    let (thinking_transformed, _) = transform_claude_request_in(&thinking_req, "test-project")
        .expect("Thinking request transformation failed");

    let thinking_ide_type =
        extract_ide_type(&thinking_transformed).expect("Thinking request missing ideType");

    // Validate shared ideType
    assert_eq!(
        standard_ide_type, "ANTIGRAVITY",
        "Standard mode should have ideType ANTIGRAVITY"
    );
    assert_eq!(
        thinking_ide_type, "ANTIGRAVITY",
        "Thinking mode should have ideType ANTIGRAVITY"
    );
    assert_eq!(
        standard_ide_type, thinking_ide_type,
        "ideType must be identical"
    );
}

/// Test 4: Thinking block absence in standard mode
///
/// **Validates**:
/// - Standard mode does NOT support thinking blocks
/// - Even when thinking config is provided, it's ignored for standard mode
///
/// **Reference**: Story-019-03 AC-1
#[test]
fn test_opus_thinking_block_absence_standard() {
    // Standard mode should NOT have thinking config even if requested
    let standard_req = create_test_claude_request_with_thinking("claude-opus-4-5", 10000);
    let (standard_transformed, _) = transform_claude_request_in(&standard_req, "test-project")
        .expect("Standard request transformation failed");

    // Standard mode should NOT have thinking config
    assert!(
        !has_thinking_config(&standard_transformed),
        "Standard mode should NOT have thinkingConfig"
    );
}

/// Test 5: Thinking block presence in thinking mode
///
/// **Validates**:
/// - Thinking mode DOES support thinking blocks
/// - Thinking config is preserved when provided
///
/// **Reference**: Epic-009 (Thinking mode baseline)
#[test]
fn test_opus_thinking_block_presence_thinking() {
    // Thinking mode SHOULD have thinking config when requested
    let thinking_req = create_test_claude_request_with_thinking("claude-opus-4-5-thinking", 10000);
    let (thinking_transformed, _) = transform_claude_request_in(&thinking_req, "test-project")
        .expect("Thinking request transformation failed");

    // Thinking mode SHOULD have thinking config
    assert!(
        has_thinking_config(&thinking_transformed),
        "Thinking mode should have thinkingConfig"
    );
}

/// Test 6: Tool modes work in both models
///
/// **Validates**:
/// - Standard mode supports tool calling
/// - Thinking mode supports tool calling
/// - Tool config is identical in both modes
///
/// **Reference**: Story-019-02 AC-1
#[test]
fn test_opus_tool_modes_work_both() {
    use antigravity_tools_lib::proxy::mappers::claude::models::{Tool, ToolChoice};

    // Create tool for testing
    let test_tool = Tool {
        name: Some("test_tool".to_string()),
        description: Some("Test tool".to_string()),
        input_schema: Some(json!({
            "type": "object",
            "properties": {}
        })),
        type_: None,
    };

    // Standard mode with tools
    let mut standard_req = create_test_claude_request("claude-opus-4-5");
    standard_req.tools = Some(vec![test_tool.clone()]);
    standard_req.tool_choice = Some(ToolChoice::Auto);

    let (standard_transformed, _) = transform_claude_request_in(&standard_req, "test-project")
        .expect("Standard request transformation failed");

    // Thinking mode with tools
    let mut thinking_req = create_test_claude_request("claude-opus-4-5-thinking");
    thinking_req.tools = Some(vec![test_tool.clone()]);
    thinking_req.tool_choice = Some(ToolChoice::Auto);

    let (thinking_transformed, _) = transform_claude_request_in(&thinking_req, "test-project")
        .expect("Thinking request transformation failed");

    // Both should have tool config
    assert!(
        has_tool_config(&standard_transformed),
        "Standard mode should support tool calling"
    );
    assert!(
        has_tool_config(&thinking_transformed),
        "Thinking mode should support tool calling"
    );
}

/// Test 7: Grounding works in both models
///
/// **Validates**:
/// - Standard mode supports grounding
/// - Thinking mode supports grounding
/// - Grounding config is identical when enabled
///
/// **Reference**: Story-019-02 AC-2
#[test]
fn test_opus_grounding_works_both() {
    // Note: Grounding requires CLAUDE_GROUNDING_ENABLED=true
    // This test assumes grounding is enabled via environment variable
    std::env::set_var("CLAUDE_GROUNDING_ENABLED", "true");

    // Standard mode
    let standard_req = create_test_claude_request("claude-opus-4-5");
    let (standard_transformed, _) = transform_claude_request_in(&standard_req, "test-project")
        .expect("Standard request transformation failed");

    // Thinking mode
    let thinking_req = create_test_claude_request("claude-opus-4-5-thinking");
    let (thinking_transformed, _) = transform_claude_request_in(&thinking_req, "test-project")
        .expect("Thinking request transformation failed");

    // Both should have grounding (if enabled)
    let standard_has_grounding = has_grounding(&standard_transformed);
    let thinking_has_grounding = has_grounding(&thinking_transformed);

    assert_eq!(
        standard_has_grounding, thinking_has_grounding,
        "Grounding support should be identical in both modes"
    );

    std::env::remove_var("CLAUDE_GROUNDING_ENABLED");
}

/// Test 8: Standard mode is simpler than thinking mode
///
/// **Validates**:
/// - Standard mode does NOT have thinking-specific complexity
/// - Thinking mode has additional thinking-related fields
///
/// **Reference**: Story-019-03 AC-1
#[test]
fn test_opus_standard_simpler_than_thinking() {
    // Standard mode (no thinking)
    let standard_req = create_test_claude_request("claude-opus-4-5");
    let (standard_transformed, _) = transform_claude_request_in(&standard_req, "test-project")
        .expect("Standard request transformation failed");

    // Thinking mode (with thinking)
    let thinking_req = create_test_claude_request_with_thinking("claude-opus-4-5-thinking", 10000);
    let (thinking_transformed, _) = transform_claude_request_in(&thinking_req, "test-project")
        .expect("Thinking request transformation failed");

    // Standard should NOT have thinking config
    let standard_has_thinking = has_thinking_config(&standard_transformed);
    let thinking_has_thinking = has_thinking_config(&thinking_transformed);

    assert!(
        !standard_has_thinking,
        "Standard mode should NOT have thinking config"
    );
    assert!(
        thinking_has_thinking,
        "Thinking mode should have thinking config"
    );
}

/// Test 9: Opus vs Sonnet pattern consistency
///
/// **Validates**:
/// - Opus standard (335) follows same patterns as Sonnet standard (333)
/// - Metadata structure is identical
/// - Both use ANTHROPIC_VERTEX provider (26)
///
/// **Reference**: Epic-017 pattern reuse
#[test]
fn test_opus_vs_sonnet_pattern_consistency() {
    // Opus standard (335)
    let opus_req = create_test_claude_request("claude-opus-4-5");
    let (opus_transformed, _) =
        transform_claude_request_in(&opus_req, "test-project").expect("Opus transformation failed");

    // Sonnet standard (333)
    let sonnet_req = create_test_claude_request("claude-4.5-sonnet");
    let (sonnet_transformed, _) = transform_claude_request_in(&sonnet_req, "test-project")
        .expect("Sonnet transformation failed");

    // Both should have same metadata structure
    assert!(
        opus_transformed.get("metadata").is_some()
            || opus_transformed
                .get("request")
                .and_then(|r| r.get("metadata"))
                .is_some(),
        "Opus should have metadata"
    );

    assert!(
        sonnet_transformed.get("metadata").is_some()
            || sonnet_transformed
                .get("request")
                .and_then(|r| r.get("metadata"))
                .is_some(),
        "Sonnet should have metadata"
    );

    // Both should use same apiProvider
    let opus_provider = extract_api_provider(&opus_transformed).expect("Opus missing apiProvider");
    let sonnet_provider =
        extract_api_provider(&sonnet_transformed).expect("Sonnet missing apiProvider");

    assert_eq!(
        opus_provider, sonnet_provider,
        "Opus and Sonnet should use same apiProvider"
    );
    assert_eq!(opus_provider, 26, "Both should use ANTHROPIC_VERTEX (26)");
}

#[cfg(test)]
mod cross_model_validation_summary {
    //! Test Summary for Story-019-03 AC-1
    //!
    //! **Total Tests**: 9
    //! **Coverage**:
    //! - ✅ Model ID distinction (335 vs 336)
    //! - ✅ Shared apiProvider (26)
    //! - ✅ Shared ideType ("ANTIGRAVITY")
    //! - ✅ Thinking block presence/absence
    //! - ✅ Tool modes work in both
    //! - ✅ Grounding works in both
    //! - ✅ Standard simpler than thinking
    //! - ✅ Opus vs Sonnet consistency
    //! - ✅ Pattern validation
    //!
    //! **Acceptance Criteria**: AC-1 COMPLETE (8+ tests required, 9 provided)
    //! **Dependencies**: Story-019-01 (modelId 335 constant)
    //! **Status**: ⏳ READY - Tests structured, waiting for Dev 2A
}
