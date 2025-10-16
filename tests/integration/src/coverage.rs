//! Test coverage analysis and reporting
//!
//! This module provides test coverage analysis capabilities to ensure
//! comprehensive testing of the Token ACL implementation.

use std::fs;

/// Test coverage metrics
#[derive(Debug, Clone)]
pub struct CoverageMetrics {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_assertions: usize,
    pub passed_assertions: usize,
    pub failed_assertions: usize,
    pub coverage_percentage: f64,
}

impl CoverageMetrics {
    /// Calculate coverage percentage
    pub fn calculate_coverage(&mut self) {
        if self.total_assertions > 0 {
            self.coverage_percentage =
                (self.passed_assertions as f64 / self.total_assertions as f64) * 100.0;
        } else {
            self.coverage_percentage = 0.0;
        }
    }
}

/// Test coverage analysis for different categories
#[derive(Debug, Clone)]
pub struct CoverageAnalysis {
    pub overall: CoverageMetrics,
    pub integration_tests: CoverageMetrics,
    pub core_logic_tests: CoverageMetrics,
    pub advanced_scenarios: CoverageMetrics,
    pub performance_tests: CoverageMetrics,
    pub security_tests: CoverageMetrics,
}

impl Default for CoverageAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl CoverageAnalysis {
    /// Create a new coverage analysis
    pub fn new() -> Self {
        Self {
            overall: CoverageMetrics {
                total_tests: 0,
                passed_tests: 0,
                failed_tests: 0,
                total_assertions: 0,
                passed_assertions: 0,
                failed_assertions: 0,
                coverage_percentage: 0.0,
            },
            integration_tests: CoverageMetrics {
                total_tests: 0,
                passed_tests: 0,
                failed_tests: 0,
                total_assertions: 0,
                passed_assertions: 0,
                failed_assertions: 0,
                coverage_percentage: 0.0,
            },
            core_logic_tests: CoverageMetrics {
                total_tests: 0,
                passed_tests: 0,
                failed_tests: 0,
                total_assertions: 0,
                passed_assertions: 0,
                failed_assertions: 0,
                coverage_percentage: 0.0,
            },
            advanced_scenarios: CoverageMetrics {
                total_tests: 0,
                passed_tests: 0,
                failed_tests: 0,
                total_assertions: 0,
                passed_assertions: 0,
                failed_assertions: 0,
                coverage_percentage: 0.0,
            },
            performance_tests: CoverageMetrics {
                total_tests: 0,
                passed_tests: 0,
                failed_tests: 0,
                total_assertions: 0,
                passed_assertions: 0,
                failed_assertions: 0,
                coverage_percentage: 0.0,
            },
            security_tests: CoverageMetrics {
                total_tests: 0,
                passed_tests: 0,
                failed_tests: 0,
                total_assertions: 0,
                passed_assertions: 0,
                failed_assertions: 0,
                coverage_percentage: 0.0,
            },
        }
    }

    /// Update overall metrics from category metrics
    pub fn update_overall(&mut self) {
        self.overall.total_tests = self.integration_tests.total_tests
            + self.core_logic_tests.total_tests
            + self.advanced_scenarios.total_tests
            + self.performance_tests.total_tests
            + self.security_tests.total_tests;

        self.overall.passed_tests = self.integration_tests.passed_tests
            + self.core_logic_tests.passed_tests
            + self.advanced_scenarios.passed_tests
            + self.performance_tests.passed_tests
            + self.security_tests.passed_tests;

        self.overall.failed_tests = self.integration_tests.failed_tests
            + self.core_logic_tests.failed_tests
            + self.advanced_scenarios.failed_tests
            + self.performance_tests.failed_tests
            + self.security_tests.failed_tests;

        self.overall.total_assertions = self.integration_tests.total_assertions
            + self.core_logic_tests.total_assertions
            + self.advanced_scenarios.total_assertions
            + self.performance_tests.total_assertions
            + self.security_tests.total_assertions;

        self.overall.passed_assertions = self.integration_tests.passed_assertions
            + self.core_logic_tests.passed_assertions
            + self.advanced_scenarios.passed_assertions
            + self.performance_tests.passed_assertions
            + self.security_tests.passed_assertions;

        self.overall.failed_assertions = self.integration_tests.failed_assertions
            + self.core_logic_tests.failed_assertions
            + self.advanced_scenarios.failed_assertions
            + self.performance_tests.failed_assertions
            + self.security_tests.failed_assertions;

        self.overall.calculate_coverage();
    }
}

/// Test coverage requirements
#[derive(Debug, Clone)]
pub struct CoverageRequirements {
    pub minimum_test_coverage: f64,
    pub minimum_assertion_coverage: f64,
    pub critical_tests_required: usize,
    pub performance_tests_required: usize,
    pub security_tests_required: usize,
}

impl Default for CoverageRequirements {
    fn default() -> Self {
        Self {
            minimum_test_coverage: 95.0,
            minimum_assertion_coverage: 90.0,
            critical_tests_required: 10,
            performance_tests_required: 5,
            security_tests_required: 8,
        }
    }
}

/// Coverage analysis results
#[derive(Debug, Clone)]
pub struct CoverageResults {
    pub analysis: CoverageAnalysis,
    pub requirements: CoverageRequirements,
    pub meets_requirements: bool,
    pub recommendations: Vec<String>,
}

impl CoverageResults {
    /// Check if coverage meets requirements
    pub fn check_requirements(&mut self) {
        self.meets_requirements = true;
        self.recommendations.clear();

        // Check overall test coverage
        if self.analysis.overall.coverage_percentage < self.requirements.minimum_test_coverage {
            self.meets_requirements = false;
            self.recommendations.push(format!(
                "Overall test coverage ({:.1}%) is below minimum requirement ({:.1}%)",
                self.analysis.overall.coverage_percentage, self.requirements.minimum_test_coverage
            ));
        }

        // Check critical tests
        if self.analysis.core_logic_tests.total_tests < self.requirements.critical_tests_required {
            self.meets_requirements = false;
            self.recommendations.push(format!(
                "Critical tests ({}) are below minimum requirement ({})",
                self.analysis.core_logic_tests.total_tests,
                self.requirements.critical_tests_required
            ));
        }

        // Check performance tests
        if self.analysis.performance_tests.total_tests
            < self.requirements.performance_tests_required
        {
            self.meets_requirements = false;
            self.recommendations.push(format!(
                "Performance tests ({}) are below minimum requirement ({})",
                self.analysis.performance_tests.total_tests,
                self.requirements.performance_tests_required
            ));
        }

        // Check security tests
        if self.analysis.security_tests.total_tests < self.requirements.security_tests_required {
            self.meets_requirements = false;
            self.recommendations.push(format!(
                "Security tests ({}) are below minimum requirement ({})",
                self.analysis.security_tests.total_tests, self.requirements.security_tests_required
            ));
        }

        // Add positive recommendations
        if self.analysis.overall.coverage_percentage >= 95.0 {
            self.recommendations
                .push("Excellent test coverage! Consider adding edge case tests.".to_string());
        }

        if self.analysis.performance_tests.total_tests >= 10 {
            self.recommendations
                .push("Good performance test coverage. Consider adding stress tests.".to_string());
        }
    }
}

/// Coverage report generator
pub mod coverage_reporting {
    use super::*;

    /// Generate comprehensive coverage report
    pub fn generate_coverage_report(
        results: &CoverageResults,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut report = String::new();

        // Header
        report.push_str("# Token ACL Test Coverage Report\n\n");
        report.push_str(&format!(
            "**Generated**: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Overall status
        let status = if results.meets_requirements {
            "✅ **COVERAGE REQUIREMENTS MET**"
        } else {
            "❌ **COVERAGE REQUIREMENTS NOT MET**"
        };
        report.push_str(&format!("{}\n\n", status));

        // Overall metrics
        report.push_str("## Overall Coverage Metrics\n\n");
        report.push_str(&format!(
            "- **Total Tests**: {}\n",
            results.analysis.overall.total_tests
        ));
        report.push_str(&format!(
            "- **Passed Tests**: {} ({}%)\n",
            results.analysis.overall.passed_tests,
            if results.analysis.overall.total_tests > 0 {
                (results.analysis.overall.passed_tests * 100) / results.analysis.overall.total_tests
            } else {
                0
            }
        ));
        report.push_str(&format!(
            "- **Failed Tests**: {}\n",
            results.analysis.overall.failed_tests
        ));
        report.push_str(&format!(
            "- **Total Assertions**: {}\n",
            results.analysis.overall.total_assertions
        ));
        report.push_str(&format!(
            "- **Coverage Percentage**: {:.1}%\n\n",
            results.analysis.overall.coverage_percentage
        ));

        // Category breakdown
        report.push_str("## Coverage by Category\n\n");
        report.push_str("| Category | Tests | Passed | Failed | Assertions | Coverage |\n");
        report.push_str("|----------|-------|--------|--------|------------|----------|\n");

        let categories = vec![
            ("Integration Tests", &results.analysis.integration_tests),
            ("Core Logic Tests", &results.analysis.core_logic_tests),
            ("Advanced Scenarios", &results.analysis.advanced_scenarios),
            ("Performance Tests", &results.analysis.performance_tests),
            ("Security Tests", &results.analysis.security_tests),
        ];

        for (name, metrics) in categories {
            report.push_str(&format!(
                "| {} | {} | {} | {} | {} | {:.1}% |\n",
                name,
                metrics.total_tests,
                metrics.passed_tests,
                metrics.failed_tests,
                metrics.total_assertions,
                metrics.coverage_percentage
            ));
        }

        // Requirements check
        report.push_str("\n## Requirements Check\n\n");
        report.push_str(&format!(
            "- **Minimum Test Coverage**: {:.1}% (Required: {:.1}%)\n",
            results.analysis.overall.coverage_percentage,
            results.requirements.minimum_test_coverage
        ));
        report.push_str(&format!(
            "- **Critical Tests**: {} (Required: {})\n",
            results.analysis.core_logic_tests.total_tests,
            results.requirements.critical_tests_required
        ));
        report.push_str(&format!(
            "- **Performance Tests**: {} (Required: {})\n",
            results.analysis.performance_tests.total_tests,
            results.requirements.performance_tests_required
        ));
        report.push_str(&format!(
            "- **Security Tests**: {} (Required: {})\n\n",
            results.analysis.security_tests.total_tests,
            results.requirements.security_tests_required
        ));

        // Recommendations
        report.push_str("## Recommendations\n\n");
        if results.recommendations.is_empty() {
            report.push_str(
                "✅ All coverage requirements are met. No additional recommendations.\n\n",
            );
        } else {
            for (i, recommendation) in results.recommendations.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", i + 1, recommendation));
            }
            report.push('\n');
        }

        // Coverage trends (placeholder for future implementation)
        report.push_str("## Coverage Trends\n\n");
        report.push_str("*Coverage trend analysis will be available in future versions.*\n\n");

        // Write to file
        fs::create_dir_all("../../tests/reports").ok();
        fs::write(output_path, &report)?;

        Ok(())
    }
}

/// Coverage analysis utilities
pub mod coverage_utils {
    use super::*;
    use crate::TestResultReport;

    /// Analyze test results and generate coverage metrics
    pub fn analyze_test_results(results: &[TestResultReport]) -> CoverageAnalysis {
        let mut analysis = CoverageAnalysis::new();

        for result in results {
            // Categorize tests based on name patterns
            let category = categorize_test(&result.name);

            match category {
                TestCategory::Integration => {
                    analysis.integration_tests.total_tests += 1;
                    if result.passed {
                        analysis.integration_tests.passed_tests += 1;
                        analysis.integration_tests.passed_assertions += result.assertions_run;
                    } else {
                        analysis.integration_tests.failed_tests += 1;
                        analysis.integration_tests.failed_assertions += result.assertions_run;
                    }
                    analysis.integration_tests.total_assertions += result.assertions_run;
                }
                TestCategory::CoreLogic => {
                    analysis.core_logic_tests.total_tests += 1;
                    if result.passed {
                        analysis.core_logic_tests.passed_tests += 1;
                        analysis.core_logic_tests.passed_assertions += result.assertions_run;
                    } else {
                        analysis.core_logic_tests.failed_tests += 1;
                        analysis.core_logic_tests.failed_assertions += result.assertions_run;
                    }
                    analysis.core_logic_tests.total_assertions += result.assertions_run;
                }
                TestCategory::AdvancedScenarios => {
                    analysis.advanced_scenarios.total_tests += 1;
                    if result.passed {
                        analysis.advanced_scenarios.passed_tests += 1;
                        analysis.advanced_scenarios.passed_assertions += result.assertions_run;
                    } else {
                        analysis.advanced_scenarios.failed_tests += 1;
                        analysis.advanced_scenarios.failed_assertions += result.assertions_run;
                    }
                    analysis.advanced_scenarios.total_assertions += result.assertions_run;
                }
                TestCategory::Performance => {
                    analysis.performance_tests.total_tests += 1;
                    if result.passed {
                        analysis.performance_tests.passed_tests += 1;
                        analysis.performance_tests.passed_assertions += result.assertions_run;
                    } else {
                        analysis.performance_tests.failed_tests += 1;
                        analysis.performance_tests.failed_assertions += result.assertions_run;
                    }
                    analysis.performance_tests.total_assertions += result.assertions_run;
                }
                TestCategory::Security => {
                    analysis.security_tests.total_tests += 1;
                    if result.passed {
                        analysis.security_tests.passed_tests += 1;
                        analysis.security_tests.passed_assertions += result.assertions_run;
                    } else {
                        analysis.security_tests.failed_tests += 1;
                        analysis.security_tests.failed_assertions += result.assertions_run;
                    }
                    analysis.security_tests.total_assertions += result.assertions_run;
                }
            }
        }

        // Calculate coverage percentages
        analysis.integration_tests.calculate_coverage();
        analysis.core_logic_tests.calculate_coverage();
        analysis.advanced_scenarios.calculate_coverage();
        analysis.performance_tests.calculate_coverage();
        analysis.security_tests.calculate_coverage();

        // Update overall metrics
        analysis.update_overall();

        analysis
    }

    /// Test categories for coverage analysis
    #[derive(Debug, Clone, PartialEq)]
    pub enum TestCategory {
        Integration,
        CoreLogic,
        AdvancedScenarios,
        Performance,
        Security,
    }

    /// Categorize a test based on its name
    pub fn categorize_test(test_name: &str) -> TestCategory {
        let name_lower = test_name.to_lowercase();

        if name_lower.contains("pda")
            || name_lower.contains("discriminator")
            || name_lower.contains("mintconfig")
        {
            TestCategory::Integration
        } else if name_lower.contains("famp")
            || name_lower.contains("permission")
            || name_lower.contains("de-escalation")
        {
            TestCategory::CoreLogic
        } else if name_lower.contains("kyc")
            || name_lower.contains("sanctions")
            || name_lower.contains("geo")
            || name_lower.contains("rwa")
        {
            TestCategory::AdvancedScenarios
        } else if name_lower.contains("benchmark") || name_lower.contains("performance") {
            TestCategory::Performance
        } else if name_lower.contains("security")
            || name_lower.contains("attack")
            || name_lower.contains("vulnerability")
        {
            TestCategory::Security
        } else {
            TestCategory::Integration // Default category
        }
    }
}
