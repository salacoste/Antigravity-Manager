//! API Provider Constants Module
//!
//! Story-024-02: Centralized API provider constants for upstream routing
//! These values are injected into the apiProvider field of all upstream requests
//! to ensure correct routing through Google Vertex AI proxy infrastructure.
//!
//! CRITICAL: These constants must match Google's internal routing table values.
//! Incorrect values will cause routing failures and 400/404 errors upstream.

/// Anthropic API via Google Vertex AI
/// Used for all Claude models (claude-4.5-sonnet, claude-opus-4-5, etc.)
pub const ANTHROPIC_VERTEX: u32 = 26;

/// Google Gemini API via Google Vertex AI
/// Used for all Gemini models (gemini-3-flash, gemini-2.5-pro-thinking, etc.)
pub const GOOGLE_VERTEX: u32 = 32;

/// OpenAI API (direct)
/// Used for OpenAI models when not using Azure endpoint
pub const OPENAI: u32 = 1;

/// OpenAI API via Azure
/// Used for OpenAI models when AZURE_OPENAI_ENDPOINT is configured
pub const OPENAI_AZURE: u32 = 2;

/// Helper function to get provider name for logging
///
/// # Arguments
/// * `provider_id` - The numeric provider ID
///
/// # Returns
/// Human-readable provider name for logging/debugging
pub fn provider_name(provider_id: u32) -> &'static str {
    match provider_id {
        ANTHROPIC_VERTEX => "ANTHROPIC_VERTEX",
        GOOGLE_VERTEX => "GOOGLE_VERTEX",
        OPENAI => "OPENAI",
        OPENAI_AZURE => "OPENAI_AZURE",
        _ => "UNKNOWN",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_vertex_constant() {
        assert_eq!(ANTHROPIC_VERTEX, 26, "ANTHROPIC_VERTEX must be 26");
    }

    #[test]
    fn test_google_vertex_constant() {
        assert_eq!(GOOGLE_VERTEX, 32, "GOOGLE_VERTEX must be 32");
    }

    #[test]
    fn test_openai_constant() {
        assert_eq!(OPENAI, 1, "OPENAI must be 1");
    }

    #[test]
    fn test_openai_azure_constant() {
        assert_eq!(OPENAI_AZURE, 2, "OPENAI_AZURE must be 2");
    }

    #[test]
    fn test_provider_name_anthropic_vertex() {
        assert_eq!(
            provider_name(ANTHROPIC_VERTEX),
            "ANTHROPIC_VERTEX",
            "provider_name(26) should return ANTHROPIC_VERTEX"
        );
    }

    #[test]
    fn test_provider_name_google_vertex() {
        assert_eq!(
            provider_name(GOOGLE_VERTEX),
            "GOOGLE_VERTEX",
            "provider_name(32) should return GOOGLE_VERTEX"
        );
    }

    #[test]
    fn test_provider_name_openai() {
        assert_eq!(
            provider_name(OPENAI),
            "OPENAI",
            "provider_name(1) should return OPENAI"
        );
    }

    #[test]
    fn test_provider_name_openai_azure() {
        assert_eq!(
            provider_name(OPENAI_AZURE),
            "OPENAI_AZURE",
            "provider_name(2) should return OPENAI_AZURE"
        );
    }

    #[test]
    fn test_provider_name_unknown() {
        assert_eq!(
            provider_name(999),
            "UNKNOWN",
            "provider_name(999) should return UNKNOWN"
        );
        assert_eq!(
            provider_name(0),
            "UNKNOWN",
            "provider_name(0) should return UNKNOWN"
        );
    }

    #[test]
    fn test_all_constants_unique() {
        let mut values = vec![ANTHROPIC_VERTEX, GOOGLE_VERTEX, OPENAI, OPENAI_AZURE];
        values.sort();
        values.dedup();
        assert_eq!(values.len(), 4, "All API provider constants must be unique");
    }

    #[test]
    fn test_constants_not_zero() {
        assert_ne!(ANTHROPIC_VERTEX, 0, "ANTHROPIC_VERTEX must not be 0");
        assert_ne!(GOOGLE_VERTEX, 0, "GOOGLE_VERTEX must not be 0");
        assert_ne!(OPENAI, 0, "OPENAI must not be 0");
        assert_ne!(OPENAI_AZURE, 0, "OPENAI_AZURE must not be 0");
    }
}
