//! Test data generator for Adaptive Budget Complexity Classifier (Story-008-01)
//!
//! Generates realistic prompts across all complexity levels for testing classification accuracy.
//!
//! # Complexity Levels
//!
//! - **Simple**: Greetings, one-word responses, yes/no questions (2000-4000 tokens)
//! - **Moderate**: Multi-sentence queries, single-topic explanations (8000-12000 tokens)
//! - **Complex**: Multi-topic analysis, reasoning tasks (16000-24000 tokens)
//! - **Deep**: Research, architectural design, comprehensive analysis (24000-32000 tokens)
//!
//! # Usage
//!
//! ```rust
//! use fixtures::prompt_generator::PromptGenerator;
//!
//! let generator = PromptGenerator::new();
//! let simple_prompts = generator.generate_simple_prompts(50);
//! let complex_prompts = generator.generate_complex_prompts(50);
//! ```

use rand::seq::SliceRandom;
use rand::Rng;

/// Test data generator for prompt classification
pub struct PromptGenerator {
    rng: rand::rngs::ThreadRng,
}

impl PromptGenerator {
    /// Create new prompt generator
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Generate simple prompts (greetings, one-word, yes/no)
    ///
    /// Expected classification: ComplexityLevel::Simple
    /// Expected budget: 2000-4000 tokens
    pub fn generate_simple_prompts(&mut self, count: usize) -> Vec<String> {
        let templates = vec![
            // Greetings
            "hello",
            "hi",
            "hey",
            "good morning",
            "good afternoon",
            "good evening",
            "greetings",
            // One-word responses
            "yes",
            "no",
            "ok",
            "sure",
            "thanks",
            "bye",
            "goodbye",
            // Simple questions
            "what time is it?",
            "how are you?",
            "what's your name?",
            "where are you?",
            "who are you?",
            // Simple commands
            "help",
            "stop",
            "continue",
            "start",
            "pause",
            // Simple confirmations
            "understood",
            "got it",
            "I see",
            "okay",
            "alright",
            // Simple acknowledgments
            "thank you",
            "thanks a lot",
            "appreciate it",
            "great",
            "perfect",
            // Simple reactions
            "wow",
            "nice",
            "cool",
            "awesome",
            "interesting",
            // Simple status checks
            "status?",
            "ready?",
            "done?",
            "finished?",
            "complete?",
            // Simple affirmations
            "correct",
            "right",
            "exactly",
            "precisely",
            "absolutely",
        ];

        self.sample_with_variations(&templates, count)
    }

    /// Generate moderate prompts (multi-sentence, single-topic, factual)
    ///
    /// Expected classification: ComplexityLevel::Moderate
    /// Expected budget: 8000-12000 tokens
    pub fn generate_moderate_prompts(&mut self, count: usize) -> Vec<String> {
        let templates = vec![
            // Explanations
            "explain quantum computing",
            "what is machine learning?",
            "how does blockchain work?",
            "describe the water cycle",
            "what is photosynthesis?",
            "explain how the internet works",
            "what is artificial intelligence?",
            "describe the solar system",
            // Summaries
            "summarize the article about climate change",
            "give me an overview of World War II",
            "summarize the main points of this document",
            "what's the summary of this book?",
            "brief overview of the meeting",
            // Single-topic how-to
            "how do I use Git?",
            "how to bake a cake?",
            "how do I learn Python?",
            "how to start a blog?",
            "how do I install Docker?",
            // Factual queries
            "what's the capital of France?",
            "when was the Declaration of Independence signed?",
            "who invented the telephone?",
            "what is the speed of light?",
            "how many planets are in our solar system?",
            // Definitions
            "define recursion in programming",
            "what does API mean?",
            "define entropy in physics",
            "what is a neural network?",
            "explain the term 'sustainability'",
            // Comparisons (simple)
            "difference between HTTP and HTTPS",
            "compare Python and JavaScript",
            "Java vs C++ performance",
            "Mac vs Windows for developers",
            "SQL vs NoSQL databases",
            // Instructions
            "write a function to reverse a string",
            "create a simple TODO list app",
            "build a REST API endpoint",
            "design a basic login form",
            "implement a binary search algorithm",
            // Historical facts
            "tell me about the Renaissance",
            "what happened during the Industrial Revolution?",
            "who was Albert Einstein?",
            "describe ancient Rome",
            "explain the American Revolution",
        ];

        self.sample_with_variations(&templates, count)
    }

    /// Generate complex prompts (multi-topic, analysis, reasoning)
    ///
    /// Expected classification: ComplexityLevel::Complex
    /// Expected budget: 16000-24000 tokens
    pub fn generate_complex_prompts(&mut self, count: usize) -> Vec<String> {
        let templates = vec![
            // Multi-topic analysis
            "analyze the pros and cons of microservices vs monolithic architecture",
            "compare React, Vue, and Angular for enterprise applications",
            "evaluate the trade-offs between SQL and NoSQL databases for e-commerce",
            "assess the impact of AI on job markets across different industries",
            "analyze the relationship between climate change and economic development",
            // Reasoning tasks
            "explain why functional programming is gaining popularity and when to use it",
            "discuss the ethical implications of autonomous vehicles",
            "reason about the best approach for scaling a web application to 1M users",
            "analyze how microservices affect team organization and communication",
            "evaluate whether Kubernetes is necessary for small startups",
            // System design (moderate complexity)
            "design a URL shortening service like bit.ly",
            "design a notification system for a social media app",
            "create a caching strategy for an e-commerce platform",
            "design a rate limiting system for an API",
            "architect a real-time chat application",
            // Problem-solving
            "how would you optimize a slow-loading web page with 100+ images?",
            "debug a memory leak in a Node.js application",
            "solve the N+1 query problem in a Django application",
            "improve the performance of a slow database query",
            "reduce the bundle size of a React application by 50%",
            // Strategic analysis
            "should we migrate from monolith to microservices? Analyze the decision.",
            "evaluate the cost-benefit of moving to cloud vs on-premise infrastructure",
            "analyze whether to build or buy a CRM system",
            "assess the risk of adopting a new JavaScript framework",
            "evaluate the ROI of implementing automated testing",
            // Technical comparisons
            "compare REST, GraphQL, and gRPC for API design with examples",
            "analyze Docker vs Kubernetes vs serverless for deployment",
            "evaluate PostgreSQL, MySQL, and MongoDB for different use cases",
            "compare Redis, Memcached, and in-memory caching strategies",
            "analyze AWS, Azure, and GCP for startup deployment",
            // Code review scenarios
            "review this code and suggest improvements for performance and maintainability",
            "analyze this architecture diagram and identify potential bottlenecks",
            "evaluate this database schema for scalability issues",
            "review this API design and suggest security improvements",
            "analyze this test suite and recommend coverage improvements",
            // Debugging scenarios
            "investigate why our API response time increased from 100ms to 500ms",
            "debug why our Redis cache hit rate dropped from 80% to 30%",
            "analyze why our database CPU usage spiked to 90%",
            "troubleshoot intermittent 502 errors in production",
            "investigate memory leaks causing weekly server restarts",
        ];

        self.sample_with_variations(&templates, count)
    }

    /// Generate deep prompts (research, architectural, comprehensive)
    ///
    /// Expected classification: ComplexityLevel::Deep
    /// Expected budget: 24000-32000 tokens
    pub fn generate_deep_prompts(&mut self, count: usize) -> Vec<String> {
        let templates = vec![
            // System design (high complexity)
            "design a distributed tracing system for a microservices architecture with 100+ services",
            "architect a globally distributed e-commerce platform handling 1M transactions/day",
            "design a real-time analytics platform processing 10TB of data per day",
            "create a disaster recovery plan for a multi-region cloud infrastructure",
            "design an event-driven architecture for a financial trading platform",
            // Research analysis
            "analyze this 10-page research paper on neural network architectures and summarize key findings",
            "review this academic paper on distributed consensus algorithms and explain practical applications",
            "analyze the state of quantum computing research and predict future developments",
            "comprehensive review of blockchain scalability solutions across multiple papers",
            "deep dive into the evolution of database indexing strategies over the past 20 years",
            // Architectural reviews
            "conduct a comprehensive security audit of our entire system architecture",
            "perform a cost optimization analysis of our AWS infrastructure with detailed recommendations",
            "analyze our entire codebase for technical debt and create a 6-month remediation plan",
            "evaluate our current testing strategy and propose a comprehensive quality assurance framework",
            "review our DevOps practices and design a modern CI/CD pipeline",
            // Strategic planning
            "create a 3-year technical roadmap for migrating a legacy monolith to modern microservices",
            "design a comprehensive data governance framework for a healthcare organization",
            "develop a security strategy for a fintech startup handling sensitive financial data",
            "create a scalability plan to grow from 10K to 10M users over 2 years",
            "design an observability strategy for a complex distributed system",
            // Multi-domain analysis
            "analyze the technical, business, and organizational challenges of adopting Kubernetes enterprise-wide",
            "evaluate the feasibility of implementing real-time machine learning in production at scale",
            "assess the impact of GDPR and CCPA on our data architecture and recommend changes",
            "analyze the trade-offs between build speed, bundle size, and runtime performance for our frontend",
            "comprehensive analysis of our current architecture vs event-driven architecture",
            // Performance optimization (deep)
            "conduct a comprehensive performance audit of our application from frontend to database with detailed recommendations",
            "analyze and optimize our entire CI/CD pipeline reducing build time from 30min to 5min",
            "deep dive into our database performance bottlenecks and propose a sharding strategy",
            "comprehensive caching strategy covering CDN, application, and database layers",
            "end-to-end latency analysis from client to database with optimization recommendations",
            // Legacy system analysis
            "analyze our 10-year-old PHP monolith and create a detailed modernization strategy",
            "evaluate the feasibility of migrating from Oracle to PostgreSQL with minimal downtime",
            "assess the risks and create a plan for migrating 500K users to a new authentication system",
            "analyze our legacy ETL pipeline and design a modern data platform",
            "create a comprehensive plan to migrate from self-hosted to cloud infrastructure",
            // Complex debugging
            "investigate and resolve a complex race condition causing data inconsistency across 5 microservices",
            "debug a memory leak that only manifests after 7 days of continuous operation",
            "analyze cascading failures in our distributed system and implement circuit breakers",
            "troubleshoot a complex authentication issue involving SAML, OAuth, and legacy sessions",
            "investigate why our ML model accuracy dropped from 95% to 75% in production",
            // Business + Technical analysis
            "analyze the total cost of ownership for building vs buying a data warehouse solution",
            "evaluate the business and technical impact of adopting GraphQL federation across 50+ teams",
            "comprehensive analysis of moving from REST to event-driven architecture: cost, timeline, risks",
            "assess the ROI and implementation plan for implementing zero-trust security architecture",
            "analyze the business case for investing in developer productivity tooling and automation",
        ];

        self.sample_with_variations(&templates, count)
    }

    /// Generate edge case prompts for robustness testing
    pub fn generate_edge_case_prompts(&mut self) -> Vec<String> {
        vec![
            // Empty and whitespace
            String::new(),
            " ".to_string(),
            "   ".to_string(),
            "\n".to_string(),
            "\t".to_string(),
            // Very short
            "a".to_string(),
            "ab".to_string(),
            // Special characters
            "!@#$%^&*()".to_string(),
            "????".to_string(),
            "「こんにちは」".to_string(),
            "مرحبا".to_string(),
            "🚀🎯💡".to_string(),
            // Very long (10K+ characters)
            "a".repeat(10_000),
            "This is a very long prompt. ".repeat(500), // ~15K chars
            // Mixed complexity
            "hi\n\nCan you explain quantum computing in detail and compare it with classical computing across multiple dimensions including performance, cost, and use cases?".to_string(),
            // Code snippets
            "```rust\nfn main() { println!(\"Hello\"); }\n```".to_string(),
            // URLs and markdown
            "[link](https://example.com) **bold** *italic*".to_string(),
        ]
    }

    /// Generate prompts with known expected budgets for validation
    pub fn generate_validation_set(&mut self) -> Vec<(String, u32)> {
        vec![
            // Simple (2000-4000)
            ("hello".to_string(), 3000),
            ("thanks".to_string(), 2500),
            ("yes".to_string(), 2000),
            // Moderate (8000-12000)
            ("explain machine learning".to_string(), 10000),
            ("summarize this article".to_string(), 9000),
            ("what is Rust?".to_string(), 8500),
            // Complex (16000-24000)
            ("analyze microservices vs monolith architecture".to_string(), 20000),
            ("compare React and Vue with examples".to_string(), 18000),
            ("debug slow API performance".to_string(), 17000),
            // Deep (24000-32000)
            ("design distributed tracing for 100+ microservices".to_string(), 30000),
            ("comprehensive security audit with recommendations".to_string(), 28000),
            ("3-year technical roadmap for modernization".to_string(), 32000),
        ]
    }

    /// Helper: Sample prompts with variations
    fn sample_with_variations(&mut self, templates: &[&str], count: usize) -> Vec<String> {
        let mut results = Vec::with_capacity(count);
        let mut used_indices = std::collections::HashSet::new();

        while results.len() < count {
            let idx = self.rng.gen_range(0..templates.len());
            if !used_indices.contains(&idx) {
                used_indices.insert(idx);
                let base = templates[idx].to_string();

                // Add variations
                let variations = vec![
                    base.clone(),
                    format!("{}?", base),
                    format!("Could you {}", base),
                    format!("Please {}", base),
                    format!("{} Thank you.", base),
                    base.to_uppercase(),
                    base.to_lowercase(),
                ];

                let variation = variations.choose(&mut self.rng).unwrap().clone();
                results.push(variation);
            }

            // If we've used all templates, reset and allow reuse with different variations
            if used_indices.len() == templates.len() && results.len() < count {
                used_indices.clear();
            }
        }

        results.truncate(count);
        results
    }
}

impl Default for PromptGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_simple_prompts() {
        let mut gen = PromptGenerator::new();
        let prompts = gen.generate_simple_prompts(50);
        assert_eq!(prompts.len(), 50);
        // All prompts should be relatively short
        for prompt in &prompts {
            assert!(
                prompt.len() < 100,
                "Simple prompt too long: {} ({})",
                prompt,
                prompt.len()
            );
        }
    }

    #[test]
    fn test_generate_moderate_prompts() {
        let mut gen = PromptGenerator::new();
        let prompts = gen.generate_moderate_prompts(50);
        assert_eq!(prompts.len(), 50);
        // Moderate prompts should be longer than simple
        for prompt in &prompts {
            assert!(
                prompt.len() > 10,
                "Moderate prompt too short: {}",
                prompt
            );
        }
    }

    #[test]
    fn test_generate_complex_prompts() {
        let mut gen = PromptGenerator::new();
        let prompts = gen.generate_complex_prompts(50);
        assert_eq!(prompts.len(), 50);
        // Complex prompts should be substantial
        for prompt in &prompts {
            assert!(prompt.len() > 30, "Complex prompt too short: {}", prompt);
        }
    }

    #[test]
    fn test_generate_deep_prompts() {
        let mut gen = PromptGenerator::new();
        let prompts = gen.generate_deep_prompts(50);
        assert_eq!(prompts.len(), 50);
        // Deep prompts should be very detailed
        for prompt in &prompts {
            assert!(prompt.len() > 50, "Deep prompt too short: {}", prompt);
        }
    }

    #[test]
    fn test_edge_cases() {
        let mut gen = PromptGenerator::new();
        let edge_cases = gen.generate_edge_case_prompts();
        assert!(!edge_cases.is_empty());
        // Should include empty strings
        assert!(edge_cases.iter().any(|s| s.is_empty()));
        // Should include very long strings
        assert!(edge_cases.iter().any(|s| s.len() > 5000));
    }

    #[test]
    fn test_validation_set() {
        let mut gen = PromptGenerator::new();
        let validation_set = gen.generate_validation_set();
        assert_eq!(validation_set.len(), 12); // 3 per complexity level
        // Verify budget ranges
        for (prompt, expected_budget) in &validation_set {
            assert!(!prompt.is_empty());
            assert!(*expected_budget >= 2000 && *expected_budget <= 32000);
        }
    }
}
