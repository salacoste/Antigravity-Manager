//! Gemini model detection utilities
//!
//! Provides functions to detect Gemini model versions and capabilities

/// Detects if a model is a Gemini 3.x model with thinking support
///
/// # Arguments
/// * `model` - The model name to check
///
/// # Returns
/// * `true` if the model is a Gemini 3.x model (excluding image models)
/// * `false` otherwise
///
/// # Examples
/// ```
/// use antigravity_tools_lib::proxy::mappers::common::gemini_detection::is_gemini_3_model;
///
/// assert!(is_gemini_3_model("gemini-3-flash"));
/// assert!(is_gemini_3_model("gemini-3-pro-high"));
/// assert!(is_gemini_3_model("gemini-3-pro-low"));
/// assert!(!is_gemini_3_model("gemini-3-pro-image"));  // Image excluded
/// assert!(!is_gemini_3_model("gemini-2.5-flash"));    // Gemini 2.5 excluded
/// ```
///
/// # Note
/// - All Gemini 3.x models use `thinkingLevel` API (not `thinkingBudget`)
/// - Image models don't support thinking mode, so they're excluded
/// - Gemini 2.5 models continue using `thinkingBudget` API
/// - Pattern matches "gemini-3." or "gemini-3-" to avoid false positives (e.g., "gemini-30")
pub fn is_gemini_3_model(model: &str) -> bool {
    // More precise pattern: gemini-3. (3.0, 3.1) or gemini-3- (3-flash, 3-pro)
    // Prevents false positives: gemini-30, gemini-300
    let is_gemini_3 = model.starts_with("gemini-3.") || model.starts_with("gemini-3-");
    let is_image = model.contains("image");

    is_gemini_3 && !is_image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_3_flash_detected() {
        assert!(
            is_gemini_3_model("gemini-3-flash"),
            "Flash should be detected as Gemini 3"
        );
    }

    #[test]
    fn test_gemini_3_pro_high_detected() {
        assert!(
            is_gemini_3_model("gemini-3-pro-high"),
            "Pro High should be detected as Gemini 3"
        );
    }

    #[test]
    fn test_gemini_3_pro_low_detected() {
        assert!(
            is_gemini_3_model("gemini-3-pro-low"),
            "Pro Low should be detected as Gemini 3"
        );
    }

    #[test]
    fn test_gemini_3_image_excluded() {
        assert!(
            !is_gemini_3_model("gemini-3-pro-image"),
            "Image model should NOT be detected (no thinking support)"
        );
    }

    #[test]
    fn test_gemini_2_5_flash_not_detected() {
        assert!(
            !is_gemini_3_model("gemini-2.5-flash"),
            "Gemini 2.5 Flash should NOT be detected"
        );
    }

    #[test]
    fn test_gemini_2_5_flash_thinking_not_detected() {
        assert!(
            !is_gemini_3_model("gemini-2.5-flash-thinking"),
            "Gemini 2.5 Flash Thinking should NOT be detected"
        );
    }

    #[test]
    fn test_gemini_2_5_pro_thinking_not_detected() {
        assert!(
            !is_gemini_3_model("gemini-2.5-pro-thinking"),
            "Gemini 2.5 Pro Thinking should NOT be detected"
        );
    }

    #[test]
    fn test_future_gemini_3_1_detected() {
        assert!(
            is_gemini_3_model("gemini-3.1-flash"),
            "Future Gemini 3.1 should be detected (forward compatibility)"
        );
    }

    #[test]
    fn test_non_gemini_model_not_detected() {
        assert!(
            !is_gemini_3_model("claude-sonnet-4-5"),
            "Non-Gemini model should NOT be detected"
        );
    }

    #[test]
    fn test_false_positive_prevention() {
        // Prevent false positives for future models like gemini-30, gemini-300
        assert!(
            !is_gemini_3_model("gemini-30-flash"),
            "gemini-30 should NOT be detected as Gemini 3"
        );
        assert!(
            !is_gemini_3_model("gemini-300-pro"),
            "gemini-300 should NOT be detected as Gemini 3"
        );
    }

    #[test]
    fn test_future_versions_detected() {
        // Future Gemini 3.x versions should be detected
        assert!(
            is_gemini_3_model("gemini-3.2-flash"),
            "gemini-3.2 should be detected"
        );
        assert!(
            is_gemini_3_model("gemini-3.5-pro"),
            "gemini-3.5 should be detected"
        );
    }
}
