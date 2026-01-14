//! Test suite for OpenAI reasoning_effort field mapping to Gemini thinkingLevel
//!
//! Tests verify:
//! - reasoning_effort "low" → "LOW"
//! - reasoning_effort "medium" → "MEDIUM" (Flash only) or "LOW" (Pro)
//! - reasoning_effort "high" → "HIGH"
//! - Invalid reasoning_effort → model defaults
//! - No reasoning_effort → model defaults

#[cfg(test)]
mod tests {
    use crate::proxy::mappers::openai::models::{OpenAIContent, OpenAIMessage, OpenAIRequest};
    use crate::proxy::mappers::openai::request::transform_openai_request;

    fn create_test_request(model: &str, reasoning_effort: Option<String>) -> OpenAIRequest {
        OpenAIRequest {
            model: model.to_string(),
            messages: vec![OpenAIMessage {
                role: "user".to_string(),
                content: Some(OpenAIContent::String("Test message".to_string())),
                reasoning_content: None,
                tool_calls: None,
                tool_call_id: None,
                name: None,
            }],
            prompt: None,
            stream: false,
            n: None,
            max_tokens: Some(8192),
            temperature: None,
            top_p: None,
            stop: None,
            response_format: None,
            tools: None,
            tool_choice: None,
            parallel_tool_calls: None,
            reasoning_effort,
            instructions: None,
            input: None,
        }
    }

    // ==================================================================================
    // REASONING_EFFORT MAPPING TESTS
    // ==================================================================================

    #[test]
    fn test_reasoning_effort_low_to_low() {
        let req = create_test_request("gemini-3-flash", Some("low".to_string()));
        let result = transform_openai_request(&req, "test-project", &req.model);
        assert!(result.is_ok());

        let body = result.unwrap();
        let thinking_level =
            &body["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"];

        assert_eq!(
            thinking_level.as_str().unwrap(),
            "LOW",
            "reasoning_effort 'low' should map to LOW"
        );
    }

    #[test]
    fn test_reasoning_effort_medium_flash() {
        let req = create_test_request("gemini-3-flash", Some("medium".to_string()));
        let result = transform_openai_request(&req, "test-project", &req.model);
        assert!(result.is_ok());

        let body = result.unwrap();
        let thinking_level =
            &body["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"];

        assert_eq!(
            thinking_level.as_str().unwrap(),
            "MEDIUM",
            "Flash should support MEDIUM level"
        );
    }

    #[test]
    fn test_reasoning_effort_medium_pro_downgrade() {
        // Pro models don't support MEDIUM, should downgrade to LOW
        let req = create_test_request("gemini-3-pro-high", Some("medium".to_string()));
        let result = transform_openai_request(&req, "test-project", &req.model);
        assert!(result.is_ok());

        let body = result.unwrap();
        let thinking_level =
            &body["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"];

        assert_eq!(
            thinking_level.as_str().unwrap(),
            "LOW",
            "Pro should downgrade MEDIUM to LOW"
        );
    }

    #[test]
    fn test_reasoning_effort_high_to_high() {
        let req = create_test_request("gemini-3-pro-low", Some("high".to_string()));
        let result = transform_openai_request(&req, "test-project", &req.model);
        assert!(result.is_ok());

        let body = result.unwrap();
        let thinking_level =
            &body["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"];

        assert_eq!(
            thinking_level.as_str().unwrap(),
            "HIGH",
            "reasoning_effort 'high' should map to HIGH"
        );
    }

    #[test]
    fn test_reasoning_effort_invalid_uses_defaults() {
        // Invalid reasoning_effort should fallback to model defaults
        let req = create_test_request("gemini-3-flash", Some("ultra".to_string()));
        let result = transform_openai_request(&req, "test-project", &req.model);
        assert!(result.is_ok());

        let body = result.unwrap();
        let thinking_level =
            &body["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"];

        assert_eq!(
            thinking_level.as_str().unwrap(),
            "MEDIUM",
            "Invalid effort should use Flash default (MEDIUM)"
        );
    }

    #[test]
    fn test_no_reasoning_effort_uses_defaults() {
        // No reasoning_effort should use model defaults
        let flash_req = create_test_request("gemini-3-flash", None);
        let flash_result = transform_openai_request(&flash_req, "test-project", &flash_req.model);
        assert!(flash_result.is_ok());

        let flash_body = flash_result.unwrap();
        let flash_level =
            &flash_body["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"];

        assert_eq!(
            flash_level.as_str().unwrap(),
            "MEDIUM",
            "Flash default should be MEDIUM"
        );

        let pro_req = create_test_request("gemini-3-pro-high", None);
        let pro_result = transform_openai_request(&pro_req, "test-project", &pro_req.model);
        assert!(pro_result.is_ok());

        let pro_body = pro_result.unwrap();
        let pro_level = &pro_body["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"];

        assert_eq!(
            pro_level.as_str().unwrap(),
            "HIGH",
            "Pro default should be HIGH"
        );
    }

    #[test]
    fn test_reasoning_effort_case_insensitive() {
        // Test case insensitivity
        let test_cases = vec!["LOW", "Low", "LoW", "low"];

        for effort in test_cases {
            let req = create_test_request("gemini-3-flash", Some(effort.to_string()));
            let result = transform_openai_request(&req, "test-project", &req.model);
            assert!(result.is_ok());

            let body = result.unwrap();
            let thinking_level =
                &body["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"];

            assert_eq!(
                thinking_level.as_str().unwrap(),
                "LOW",
                "reasoning_effort should be case-insensitive: {}",
                effort
            );
        }
    }
}
