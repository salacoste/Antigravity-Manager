// Budget Optimization Commands - Story-025-01
// Tauri commands for budget allocation and metrics tracking

use crate::modules::budget_optimizer::{BudgetAllocation, BudgetOptimizer, OptimizationMetrics};
use serde_json::Value;
use std::sync::Arc;
use tauri::State;

/// Shared budget optimizer state
pub struct BudgetOptimizerState {
    pub optimizer: Arc<BudgetOptimizer>,
}

impl BudgetOptimizerState {
    pub fn new() -> Self {
        Self {
            optimizer: Arc::new(BudgetOptimizer::new()),
        }
    }
}

impl Default for BudgetOptimizerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Request structure for budget allocation
#[derive(serde::Deserialize)]
pub struct AllocateBudgetRequest {
    pub request_text: String,
    pub messages: Vec<Value>,
}

/// Allocate budget for a request
#[tauri::command]
pub async fn allocate_budget(
    state: State<'_, BudgetOptimizerState>,
    request: AllocateBudgetRequest,
) -> Result<BudgetAllocation, String> {
    let allocation = state
        .optimizer
        .allocate_budget(&request.request_text, &request.messages)
        .await;

    Ok(allocation)
}

/// Get current optimization metrics
#[tauri::command]
pub async fn get_budget_metrics(
    state: State<'_, BudgetOptimizerState>,
) -> Result<OptimizationMetrics, String> {
    let metrics = state.optimizer.get_metrics().await;
    Ok(metrics)
}

/// Reset optimization metrics (useful for testing)
#[tauri::command]
pub async fn reset_budget_metrics(state: State<'_, BudgetOptimizerState>) -> Result<(), String> {
    state.optimizer.reset_metrics().await;
    Ok(())
}

/// Test budget allocation with sample data
#[tauri::command]
pub async fn test_budget_allocation(
    state: State<'_, BudgetOptimizerState>,
) -> Result<Vec<BudgetAllocation>, String> {
    let test_cases = vec![
        ("What is the weather today?", vec![]),
        (
            "First, analyze the data. Then, process it. Finally, generate a report.",
            vec![],
        ),
        (
            r#"Here is the code:
```rust
async fn main() {
    println!("Hello");
}
```
Please refactor it using proper async patterns."#,
            vec![],
        ),
    ];

    let mut results = Vec::new();
    for (text, messages) in test_cases {
        let allocation = state.optimizer.allocate_budget(text, &messages).await;
        results.push(allocation);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_budget_optimizer_state() {
        let state = BudgetOptimizerState::new();

        // Initial metrics should be zero
        let metrics = state.optimizer.get_metrics().await;
        assert_eq!(metrics.total_requests, 0);

        // Allocate budget
        let allocation = state
            .optimizer
            .allocate_budget("test request", &vec![])
            .await;

        assert!(allocation.budget > 0);

        // Metrics should be updated
        let metrics = state.optimizer.get_metrics().await;
        assert_eq!(metrics.total_requests, 1);
    }
}
