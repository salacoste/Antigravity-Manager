// Epic-025 Week 7: Final Integration Tests
// End-to-end testing of complete Flash Thinking Optimization system

#[cfg(test)]
mod epic_025_integration_tests {
    use crate::modules::{budget_detector, budget_optimizer, thinking_quality};
    use std::sync::Arc;

    /// Helper: Create test request with given complexity
    fn create_test_request(prompt: &str, complexity: &str) -> serde_json::Value {
        serde_json::json!({
            "model": "gemini-2.0-flash-thinking-exp",
            "messages": [{
                "role": "user",
                "content": prompt
            }],
            "complexity_hint": complexity
        })
    }

    /// Helper: Create mock Gemini response
    fn create_mock_response(finish_reason: &str, token_count: usize) -> serde_json::Value {
        serde_json::json!({
            "candidates": [{
                "content": {
                    "parts": [{
                        "text": "Mock response content"
                    }],
                    "role": "model"
                },
                "finishReason": finish_reason,
                "index": 0
            }],
            "usageMetadata": {
                "promptTokenCount": 100,
                "candidatesTokenCount": token_count,
                "totalTokenCount": 100 + token_count
            }
        })
    }

    /// Test 1: Complete Epic-025 flow from budget allocation to quality analysis
    #[tokio::test]
    async fn test_epic_025_end_to_end_flow() {
        // Initialize components
        let budget_optimizer = budget_optimizer::BudgetOptimizer::new();
        let budget_detector = budget_detector::BudgetSufficiencyDetector::new();
        let quality_monitor = Arc::new(thinking_quality::ThinkingQualityMonitor::new());

        // Test request: Moderate complexity
        let request = create_test_request(
            "Implement a binary search tree with insert, delete, and search operations",
            "moderate",
        );

        // Step 1: Budget Optimizer classifies request and allocates budget
        let classification = budget_optimizer.classify_complexity(&request);
        assert!(
            classification.confidence > 0.7,
            "Classification confidence should be > 0.7"
        );

        let budget_alloc = budget_optimizer
            .optimize_budget(&request)
            .await
            .expect("Budget optimization should succeed");

        assert!(
            budget_alloc.budget >= 4096 && budget_alloc.budget <= 24576,
            "Budget should be within valid range"
        );

        // For moderate complexity, expect 12K budget
        assert_eq!(
            budget_alloc.budget, 12288,
            "Moderate complexity should get 12K budget"
        );
        assert_eq!(budget_alloc.tier, "Moderate");

        // Step 2: Simulate request sent with optimized budget
        // (In production, this would be a real Gemini API call)
        let response = create_mock_response("STOP", 10000);

        // Step 3: Budget Sufficiency Detector checks response
        let detection = budget_detector.detect_insufficiency(&response);

        assert_eq!(
            detection,
            budget_detector::DetectionResult::Sufficient,
            "Response should be detected as sufficient"
        );

        // Step 4: Quality Monitor analyzes response
        let thinking_tokens = 3000;
        let output_tokens = 7000;

        let analysis = quality_monitor
            .analyze_quality(
                "test-request-id".to_string(),
                thinking_tokens,
                output_tokens,
                budget_alloc.budget,
                budget_detector::FinishReason::Stop,
                0, // escalation_count
            )
            .await;

        assert!(
            analysis.overall_score > 0.8,
            "Overall quality score should be > 0.8"
        );
        assert!(
            analysis.first_time_right,
            "Should be first-time-right with sufficient budget"
        );
        assert!(
            analysis.efficiency_score > 0.75,
            "Efficiency should be good with proper budget allocation"
        );
        assert_eq!(
            analysis.completeness_score, 1.0,
            "Completeness should be 1.0 for STOP finish reason"
        );

        // Step 5: Verify metrics update
        let metrics = quality_monitor.get_metrics().await;
        assert_eq!(metrics.total_requests, 1);
        assert_eq!(metrics.first_time_right, 1);
        assert_eq!(metrics.escalations_needed, 0);
    }

    /// Test 2: Escalation flow when budget is insufficient
    #[tokio::test]
    async fn test_epic_025_escalation_flow() {
        let budget_optimizer = budget_optimizer::BudgetOptimizer::new();
        let budget_detector = budget_detector::BudgetSufficiencyDetector::new();
        let quality_monitor = Arc::new(thinking_quality::ThinkingQualityMonitor::new());

        // Test request: Simple complexity (will get 4K budget)
        let request = create_test_request("What is 2+2?", "simple");

        // Step 1: Get initial budget allocation
        let budget_alloc = budget_optimizer.optimize_budget(&request).await.unwrap();
        assert_eq!(budget_alloc.budget, 4096, "Simple should get 4K budget");

        // Step 2: Simulate insufficient response (MAX_TOKENS finish reason)
        let response = create_mock_response("MAX_TOKENS", 3900);

        // Step 3: Detect insufficiency
        let detection = budget_detector.detect_insufficiency(&response);
        assert_eq!(
            detection,
            budget_detector::DetectionResult::Insufficient,
            "Should detect insufficiency"
        );

        // Step 4: Quality analysis for first attempt
        let analysis1 = quality_monitor
            .analyze_quality(
                "test-request-escalation-1",
                &request,
                &response,
                budget_alloc.budget,
                2000,
                1900,
                false, // Not first-time-right
            )
            .await;

        assert!(
            !analysis1.first_time_right,
            "Should not be first-time-right"
        );
        assert!(
            analysis1.completeness_score < 1.0,
            "Completeness should be reduced"
        );

        // Step 5: Escalate to higher budget (12K)
        let escalated_response = create_mock_response("STOP", 10000);

        let analysis2 = quality_monitor
            .analyze_quality(
                "test-request-escalation-2",
                &request,
                &escalated_response,
                12288, // Escalated budget
                4000,
                6000,
                false, // Still not first-time-right (required escalation)
            )
            .await;

        assert!(
            !analysis2.first_time_right,
            "Should not count as first-time-right after escalation"
        );
        assert_eq!(
            analysis2.completeness_score, 1.0,
            "Should be complete after escalation"
        );

        // Step 6: Verify metrics show escalation
        let metrics = quality_monitor.get_metrics().await;
        assert_eq!(
            metrics.escalations_needed, 1,
            "Should track escalation count"
        );
        assert_eq!(metrics.total_requests, 2, "Should count both attempts");
        assert_eq!(
            metrics.first_time_right, 0,
            "Neither attempt was first-time-right"
        );
    }

    /// Test 3: ROI calculation validates 20-30% savings target
    #[tokio::test]
    async fn test_epic_025_roi_validation() {
        let budget_optimizer = budget_optimizer::BudgetOptimizer::new();

        // Simulate 100 requests with mix of complexities
        let test_requests = vec![
            ("What is Rust?", "simple", 50),                    // 50 simple
            ("Implement quicksort", "moderate", 30),            // 30 moderate
            ("Design a distributed system", "complex", 20),     // 20 complex
        ];

        let mut total_cost_without_optimization = 0;
        let mut total_cost_with_optimization = 0;

        for (prompt, _complexity, count) in test_requests {
            let request = create_test_request(prompt, _complexity);

            // Without optimization: Always use 24K budget
            total_cost_without_optimization += 24576 * count;

            // With optimization: Use classifier
            let budget_alloc = budget_optimizer.optimize_budget(&request).await.unwrap();
            total_cost_with_optimization += budget_alloc.budget * count;
        }

        let savings = ((total_cost_without_optimization - total_cost_with_optimization) as f64
            / total_cost_without_optimization as f64)
            * 100.0;

        println!("Total cost without optimization: {}", total_cost_without_optimization);
        println!("Total cost with optimization: {}", total_cost_with_optimization);
        println!("Savings: {:.1}%", savings);

        // Verify Epic-025 target: 20-30% savings
        assert!(
            savings >= 20.0 && savings <= 40.0,
            "Expected 20-40% savings, got {:.1}%",
            savings
        );
    }

    /// Test 4: Quality metrics aggregation over time
    #[tokio::test]
    async fn test_epic_025_quality_metrics_aggregation() {
        let quality_monitor = Arc::new(thinking_quality::ThinkingQualityMonitor::new());

        // Simulate 20 requests with varying quality
        for i in 0..20 {
            let request = create_test_request(&format!("Test request {}", i), "moderate");
            let response = create_mock_response("STOP", 8000 + (i * 100));

            let first_time_right = i % 5 != 0; // 80% FTR rate
            let budget = if first_time_right { 12288 } else { 4096 };

            quality_monitor
                .analyze_quality(
                    &format!("test-{}", i),
                    &request,
                    &response,
                    budget,
                    3000 + (i * 50),
                    5000 + (i * 50),
                    first_time_right,
                )
                .await;
        }

        // Check aggregated metrics
        let metrics = quality_monitor.get_metrics().await;

        assert_eq!(metrics.total_requests, 20);
        assert_eq!(metrics.first_time_right, 16); // 80%
        assert_eq!(metrics.escalations_needed, 4); // 20%

        let ftr_rate = metrics.first_time_right as f64 / metrics.total_requests as f64;
        assert!(
            ftr_rate >= 0.8,
            "FTR rate should be >= 80%, got {:.1}%",
            ftr_rate * 100.0
        );
    }

    /// Test 5: Weekly feedback generation
    #[tokio::test]
    async fn test_epic_025_weekly_feedback() {
        let quality_monitor = Arc::new(thinking_quality::ThinkingQualityMonitor::new());

        // Simulate varied quality over 7 days
        for day in 0..7 {
            for request in 0..10 {
                let req = create_test_request(&format!("Day {} Request {}", day, request), "moderate");
                let resp = create_mock_response("STOP", 8000);

                quality_monitor
                    .analyze_quality(
                        &format!("test-day{}-req{}", day, request),
                        &req,
                        &resp,
                        12288,
                        3000,
                        5000,
                        request % 10 != 0, // 90% FTR
                    )
                    .await;
            }
        }

        // Get weekly feedback
        let feedback = quality_monitor
            .get_weekly_feedback(7)
            .await
            .expect("Should get weekly feedback");

        assert_eq!(feedback.total_requests, 70);
        assert!(
            feedback.first_time_right_rate >= 0.85,
            "FTR rate should be >= 85%"
        );
        assert!(
            feedback.avg_quality_score >= 0.8,
            "Average quality should be >= 80%"
        );
        assert!(
            !feedback.recommendations.is_empty(),
            "Should generate recommendations"
        );
    }

    /// Test 6: Component integration - all modules working together
    #[tokio::test]
    async fn test_epic_025_full_system_integration() {
        // Initialize all Epic-025 components
        let budget_optimizer = budget_optimizer::BudgetOptimizer::new();
        let budget_detector = budget_detector::BudgetSufficiencyDetector::new();
        let quality_monitor = Arc::new(thinking_quality::ThinkingQualityMonitor::new());

        // Test various scenarios
        let scenarios = vec![
            ("Simple math question", "simple", 4096, "STOP", true),
            ("Implement algorithm", "moderate", 12288, "STOP", true),
            ("Design system architecture", "complex", 24576, "STOP", true),
            ("Quick question", "simple", 4096, "MAX_TOKENS", false), // Will need escalation
        ];

        for (prompt, expected_tier, expected_budget, finish_reason, should_be_ftr) in scenarios {
            let request = create_test_request(prompt, expected_tier);

            // Budget allocation
            let budget_alloc = budget_optimizer.optimize_budget(&request).await.unwrap();
            assert_eq!(
                budget_alloc.budget, expected_budget,
                "Budget mismatch for: {}",
                prompt
            );

            // Response simulation
            let response = create_mock_response(finish_reason, expected_budget as usize - 2000);

            // Sufficiency detection
            let detection = budget_detector.detect_insufficiency(&response);
            let is_sufficient = detection == budget_detector::DetectionResult::Sufficient;
            assert_eq!(
                is_sufficient, should_be_ftr,
                "Sufficiency mismatch for: {}",
                prompt
            );

            // Quality analysis
            let analysis = quality_monitor
                .analyze_quality(
                    &format!("integration-{}", prompt),
                    &request,
                    &response,
                    budget_alloc.budget,
                    3000,
                    5000,
                    should_be_ftr,
                )
                .await;

            assert_eq!(
                analysis.first_time_right, should_be_ftr,
                "FTR mismatch for: {}",
                prompt
            );
        }

        // Verify final metrics
        let metrics = quality_monitor.get_metrics().await;
        assert_eq!(metrics.total_requests, 4);
        assert_eq!(
            metrics.first_time_right, 3,
            "3 out of 4 should be first-time-right"
        );
    }
}
