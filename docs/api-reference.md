# Token ACL Testing Suite - API Reference

This document provides comprehensive API documentation for the Token ACL Testing Suite.

## Core Modules

### `common` Module

The `common` module provides shared utilities and data structures used across all test modules.

#### `TestResultReport`

```rust
pub struct TestResultReport {
    pub name: String,
    pub passed: bool,
    pub error: Option<String>,
    pub assertions_run: usize,
}
```

**Purpose**: Standardized test result reporting structure.

**Methods**:
- `success(name: &str, assertions: usize) -> Self` - Create a successful test result
- `failure(name: &str, error: String) -> Self` - Create a failed test result

**Example**:
```rust
use token_acl_integration_tests::TestResultReport;

let result = TestResultReport::success("My Test", 5);
assert!(result.passed);
assert_eq!(result.assertions_run, 5);
```

#### `utils` Module

Utility functions for common test operations.

**Functions**:
- `create_test_keypair(seed: &[u8]) -> Keypair` - Generate deterministic test keypairs
- `verify_pda_derivation(seeds: &[&[u8]], program_id: &Pubkey, expected_pda: &Pubkey) -> bool` - Verify PDA derivation
- `is_default_pubkey(pubkey: &Pubkey) -> bool` - Check if pubkey is default (all zeros)
- `is_valid_discriminator(discriminator: &[u8]) -> bool` - Validate discriminator format

**Example**:
```rust
use token_acl_integration_tests::utils;

let keypair = utils::create_test_keypair(b"test-seed");
let is_valid = utils::is_valid_discriminator(&[8, 175, 169, 129, 137, 74, 61, 241]);
assert!(is_valid);
```

#### `assertions` Module

Common test assertions with standardized error handling.

**Functions**:
- `assert_true(condition: bool, test_name: &str, error_message: &str) -> Result<(), TestResultReport>`
- `assert_eq<T>(actual: &T, expected: &T, test_name: &str, error_message: &str) -> Result<(), TestResultReport>`
- `assert_not_default_pubkey(pubkey: &Pubkey, test_name: &str, field_name: &str) -> Result<(), TestResultReport>`

**Example**:
```rust
use token_acl_integration_tests::assertions;

let result = assertions::assert_true(
    true,
    "My Test",
    "Condition should be true"
);
assert!(result.is_ok());
```

#### `reporting` Module

Test result aggregation and reporting functionality.

**Functions**:
- `generate_test_report(results: &[TestResultReport], title: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>>`

**Example**:
```rust
use token_acl_integration_tests::reporting;

let results = vec![
    TestResultReport::success("Test 1", 3),
    TestResultReport::success("Test 2", 5),
];

reporting::generate_test_report(
    &results,
    "My Test Suite",
    "reports/my_tests.md"
)?;
```

### `fixtures` Module

Test data, mock objects, and fixtures for consistent testing scenarios.

#### `test_data` Module

Standard test data and constants.

**Constants**:
- `THAW_DISCRIMINATOR: [u8; 8]` - Standard thaw discriminator from sRFC 37
- `FREEZE_DISCRIMINATOR: [u8; 8]` - Standard freeze discriminator from sRFC 37
- `MINT_CONFIG_SEED: &[u8]` - Standard seed for mint config PDA derivation

**Functions**:
- `create_test_mint_config(mint: Pubkey, authority: Pubkey, gating_program: Pubkey) -> TestMintConfig`

**Example**:
```rust
use token_acl_integration_tests::fixtures::test_data;

let discriminator = test_data::THAW_DISCRIMINATOR;
let seed = test_data::MINT_CONFIG_SEED;
```

#### `scenarios` Module

Test scenarios and use cases for real-world testing.

**Structs**:
- `KYCScenario` - KYC allowlist testing scenarios
- `SanctionsScenario` - Sanctions list testing scenarios
- `GeoBlockingScenario` - Geo-blocking testing scenarios

**Example**:
```rust
use token_acl_integration_tests::fixtures::scenarios;

let kyc_scenario = scenarios::KYCScenario::new_valid_user(user_pubkey);
assert!(kyc_scenario.kyc_complete);
```

### `benchmarks` Module

Performance benchmarking and optimization utilities.

#### `BenchmarkRunner`

```rust
pub struct BenchmarkRunner {
    name: String,
    iterations: usize,
    warmup_iterations: usize,
}
```

**Methods**:
- `new(name: &str) -> Self` - Create a new benchmark runner
- `iterations(self, iterations: usize) -> Self` - Set number of iterations
- `warmup_iterations(self, warmup_iterations: usize) -> Self` - Set warmup iterations
- `run<F>(self, benchmark_fn: F) -> BenchmarkResult` - Run the benchmark

**Example**:
```rust
use token_acl_integration_tests::benchmarks::BenchmarkRunner;

let result = BenchmarkRunner::new("My Benchmark")
    .iterations(1000)
    .warmup_iterations(100)
    .run(|| {
        // Benchmark code here
        Ok(())
    });

println!("Average time: {:?}", result.avg_duration);
```

#### `performance_benchmarks` Module

Pre-built performance benchmarks for common operations.

**Functions**:
- `benchmark_pda_derivation() -> BenchmarkResult`
- `benchmark_discriminator_validation() -> BenchmarkResult`
- `benchmark_serialization() -> BenchmarkResult`
- `benchmark_account_validation() -> BenchmarkResult`
- `run_all_benchmarks() -> Vec<BenchmarkResult>`

**Example**:
```rust
use token_acl_integration_tests::benchmarks::performance_benchmarks;

let results = performance_benchmarks::run_all_benchmarks();
for result in results {
    println!("{}: {:?}", result.name, result.avg_duration);
}
```

### `logging` Module

Enhanced logging and error handling utilities.

#### `Logger`

```rust
pub struct Logger {
    level: LogLevel,
    entries: Vec<LogEntry>,
}
```

**Methods**:
- `new(level: LogLevel) -> Self` - Create a new logger
- `trace(module: &str, message: &str)` - Log trace message
- `debug(module: &str, message: &str)` - Log debug message
- `info(module: &str, message: &str)` - Log info message
- `warn(module: &str, message: &str)` - Log warning message
- `error(module: &str, message: &str)` - Log error message
- `export_to_file(path: &str) -> Result<(), Box<dyn std::error::Error>>` - Export logs to file

**Example**:
```rust
use token_acl_integration_tests::logging::{Logger, LogLevel};

let mut logger = Logger::new(LogLevel::Info);
logger.info("test_module", "Starting test execution");
logger.error("test_module", "Test failed with error");
```

#### `TestError` Enum

```rust
pub enum TestError {
    TestFailure(String),
    AssertionFailure(String),
    SetupError(String),
    CleanupError(String),
    PerformanceError(String),
    ConfigurationError(String),
}
```

**Example**:
```rust
use token_acl_integration_tests::logging::TestError;

let error = TestError::TestFailure("Something went wrong".to_string());
println!("Error: {}", error);
```

### `coverage` Module

Test coverage analysis and reporting.

#### `CoverageAnalysis`

```rust
pub struct CoverageAnalysis {
    pub overall: CoverageMetrics,
    pub integration_tests: CoverageMetrics,
    pub core_logic_tests: CoverageMetrics,
    pub advanced_scenarios: CoverageMetrics,
    pub performance_tests: CoverageMetrics,
    pub security_tests: CoverageMetrics,
}
```

**Methods**:
- `new() -> Self` - Create new coverage analysis
- `update_overall()` - Update overall metrics from category metrics

#### `CoverageRequirements`

```rust
pub struct CoverageRequirements {
    pub minimum_test_coverage: f64,
    pub minimum_assertion_coverage: f64,
    pub critical_tests_required: usize,
    pub performance_tests_required: usize,
    pub security_tests_required: usize,
}
```

**Example**:
```rust
use token_acl_integration_tests::coverage::{CoverageAnalysis, CoverageRequirements};

let mut analysis = CoverageAnalysis::new();
let requirements = CoverageRequirements::default();
// ... populate analysis with test results
analysis.update_overall();
```

## Usage Examples

### Basic Test Structure

```rust
use token_acl_integration_tests::{
    TestResultReport,
    utils,
    assertions,
    reporting,
};

#[test]
fn my_test() {
    let report = run_my_test();
    assert!(report.passed, "Test failed: {:?}", report.error);
}

fn run_my_test() -> TestResultReport {
    let test_name = "My Test";
    let mut assertion_count = 0;
    
    // Test logic here
    assertion_count += 1;
    if !some_condition {
        return TestResultReport::failure(test_name, "Condition failed".to_string());
    }
    
    TestResultReport::success(test_name, assertion_count)
}
```

### Performance Benchmarking

```rust
use token_acl_integration_tests::benchmarks::{BenchmarkRunner, performance_benchmarks};

#[test]
fn benchmark_my_operation() {
    let result = BenchmarkRunner::new("My Operation")
        .iterations(1000)
        .run(|| {
            // Operation to benchmark
            Ok(())
        });
    
    assert!(result.success);
    assert!(result.avg_duration.as_micros() < 1000); // Should be fast
}
```

### Coverage Analysis

```rust
use token_acl_integration_tests::coverage::{coverage_utils, coverage_reporting};

#[test]
fn analyze_coverage() {
    let test_results = vec![
        TestResultReport::success("Test 1", 3),
        TestResultReport::success("Test 2", 5),
    ];
    
    let analysis = coverage_utils::analyze_test_results(&test_results);
    let requirements = CoverageRequirements::default();
    
    let mut results = CoverageResults {
        analysis,
        requirements,
        meets_requirements: false,
        recommendations: Vec::new(),
    };
    
    results.check_requirements();
    
    coverage_reporting::generate_coverage_report(
        &results,
        "reports/coverage.md"
    )?;
}
```

## Best Practices

1. **Use Shared Utilities**: Always use the shared utilities from the `common` module instead of duplicating code.

2. **Structured Logging**: Use the logging module for consistent, structured logging across all tests.

3. **Performance Testing**: Include performance benchmarks for critical operations to ensure they meet requirements.

4. **Coverage Analysis**: Regularly run coverage analysis to ensure comprehensive test coverage.

5. **Error Handling**: Use the standardized error types and handling utilities for consistent error reporting.

6. **Test Organization**: Organize tests by category (integration, core logic, advanced scenarios) for better maintainability.

## Integration with CI/CD

The testing suite is designed to integrate seamlessly with CI/CD pipelines:

- All tests generate standardized reports
- Performance benchmarks provide measurable metrics
- Coverage analysis ensures quality gates
- Security audits can be integrated
- Structured logging provides debugging information

See the `scripts/run_comprehensive_tests.sh` script for a complete example of running all tests with reporting.
