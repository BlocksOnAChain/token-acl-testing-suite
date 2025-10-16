//! Common test utilities and shared functionality
//!
//! This module provides shared utilities used across all test modules,
//! including test result reporting, common assertions, and helper functions.
//!
//! # Features
//!
//! - **Test Result Reporting**: Standardized test result structure with detailed reporting
//! - **Common Utilities**: Helper functions for PDA derivation, keypair generation, and validation
//! - **Assertion Helpers**: Reusable assertion functions with detailed error messages
//! - **Report Generation**: Comprehensive test report generation with markdown output
//!
//! # Examples
//!
//! ```rust
//! use token_acl_integration_tests::TestResultReport;
//!
//! // Create a successful test result
//! let result = TestResultReport::success("My Test", 5);
//! assert!(result.passed);
//!
//! // Create a failed test result
//! let result = TestResultReport::failure("My Test", "Something went wrong".to_string());
//! assert!(!result.passed);
//! ```

use solana_sdk::pubkey::Pubkey;
use std::fmt;

/// Standardized test result reporting structure
///
/// This structure provides a consistent way to report test results across
/// all test modules, including success/failure status, error details, and
/// assertion counts for comprehensive test reporting.
///
/// # Fields
///
/// - `name`: The name of the test that was executed
/// - `passed`: Whether the test passed or failed
/// - `error`: Optional error message if the test failed
/// - `assertions_run`: Number of assertions that were executed during the test
///
/// # Examples
///
/// ```rust
/// let result = TestResultReport::success("PDA Derivation Test", 3);
/// println!("Test result: {}", result);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TestResultReport {
    /// The name of the test that was executed
    pub name: String,
    /// Whether the test passed or failed
    pub passed: bool,
    /// Optional error message if the test failed
    pub error: Option<String>,
    /// Number of assertions that were executed during the test
    pub assertions_run: usize,
}

impl TestResultReport {
    /// Create a successful test result
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the test that passed
    /// * `assertions` - The number of assertions that were executed
    ///
    /// # Returns
    ///
    /// A `TestResultReport` with `passed` set to `true` and no error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let result = TestResultReport::success("PDA Derivation Test", 3);
    /// assert!(result.passed);
    /// assert_eq!(result.assertions_run, 3);
    /// ```
    pub fn success(name: &str, assertions: usize) -> Self {
        Self {
            name: name.to_string(),
            passed: true,
            error: None,
            assertions_run: assertions,
        }
    }

    /// Create a failed test result
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the test that failed
    /// * `error` - A detailed error message describing what went wrong
    ///
    /// # Returns
    ///
    /// A `TestResultReport` with `passed` set to `false` and the provided error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let result = TestResultReport::failure("PDA Test", "Invalid seeds provided".to_string());
    /// assert!(!result.passed);
    /// assert!(result.error.is_some());
    /// ```
    pub fn failure(name: &str, error: String) -> Self {
        Self {
            name: name.to_string(),
            passed: false,
            error: Some(error),
            assertions_run: 0,
        }
    }

    /// Get a human-readable status string
    ///
    /// # Returns
    ///
    /// Returns "PASS" if the test passed, "FAIL" if it failed.
    pub fn status(&self) -> &'static str {
        if self.passed { "PASS" } else { "FAIL" }
    }

    /// Check if the test passed
    ///
    /// # Returns
    ///
    /// Returns `true` if the test passed, `false` otherwise.
    pub fn is_success(&self) -> bool {
        self.passed
    }

    /// Check if the test failed
    ///
    /// # Returns
    ///
    /// Returns `true` if the test failed, `false` otherwise.
    pub fn is_failure(&self) -> bool {
        !self.passed
    }
}

impl fmt::Display for TestResultReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.passed { "✅ PASS" } else { "❌ FAIL" };
        let error = self.error.as_deref().unwrap_or("-");
        write!(
            f,
            "{} | {} | {} | {}",
            self.name, status, self.assertions_run, error
        )
    }
}

/// Common test utilities and helper functions
///
/// This module provides utility functions commonly used across test modules
/// for keypair generation, PDA derivation, and validation operations.
pub mod utils {
    use super::*;
    use solana_sdk::signature::Keypair;

    /// Generate a test keypair with a deterministic seed
    ///
    /// # Arguments
    ///
    /// * `seed` - A byte slice used as the seed for keypair generation (max 32 bytes)
    ///
    /// # Returns
    ///
    /// A `Keypair` generated from the provided seed.
    ///
    /// # Panics
    ///
    /// This function will panic if the seed cannot be used to generate a valid keypair.
    /// This should not happen with properly formatted seeds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let keypair = utils::create_test_keypair(b"test_seed");
    /// assert!(!keypair.pubkey().to_bytes().iter().all(|&b| b == 0));
    /// ```
    pub fn create_test_keypair(seed: &[u8]) -> Keypair {
        let mut seed_bytes = [0u8; 32];
        let copy_len = seed.len().min(32);
        seed_bytes[..copy_len].copy_from_slice(&seed[..copy_len]);
        Keypair::from_bytes(&seed_bytes)
            .expect("Failed to create keypair from seed - this should not happen with valid seeds")
    }

    /// Verify PDA derivation matches expected result
    ///
    /// # Arguments
    ///
    /// * `seeds` - The seeds used for PDA derivation
    /// * `program_id` - The program ID for PDA derivation
    /// * `expected_pda` - The expected PDA result
    ///
    /// # Returns
    ///
    /// Returns `true` if the derived PDA matches the expected PDA, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let program_id = Pubkey::new_unique();
    /// let seeds = [b"test_seed"];
    /// let (expected_pda, _) = Pubkey::find_program_address(&seeds, &program_id);
    /// assert!(utils::verify_pda_derivation(&seeds, &program_id, &expected_pda));
    /// ```
    pub fn verify_pda_derivation(
        seeds: &[&[u8]],
        program_id: &Pubkey,
        expected_pda: &Pubkey,
    ) -> bool {
        let (derived_pda, _bump) = Pubkey::find_program_address(seeds, program_id);
        derived_pda == *expected_pda
    }

    /// Check if a pubkey is the default (all zeros)
    ///
    /// # Arguments
    ///
    /// * `pubkey` - The pubkey to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the pubkey is all zeros (default), `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let default_pubkey = Pubkey::default();
    /// assert!(utils::is_default_pubkey(&default_pubkey));
    ///
    /// let real_pubkey = Pubkey::new_unique();
    /// assert!(!utils::is_default_pubkey(&real_pubkey));
    /// ```
    pub fn is_default_pubkey(pubkey: &Pubkey) -> bool {
        *pubkey == Pubkey::default()
    }

    /// Validate discriminator format (8 bytes, not all zeros)
    ///
    /// # Arguments
    ///
    /// * `discriminator` - The discriminator bytes to validate
    ///
    /// # Returns
    ///
    /// Returns `true` if the discriminator is exactly 8 bytes and not all zeros, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let valid_discriminator = [1, 2, 3, 4, 5, 6, 7, 8];
    /// assert!(utils::is_valid_discriminator(&valid_discriminator));
    ///
    /// let invalid_discriminator = [0, 0, 0, 0, 0, 0, 0, 0];
    /// assert!(!utils::is_valid_discriminator(&invalid_discriminator));
    ///
    /// let wrong_length = [1, 2, 3, 4];
    /// assert!(!utils::is_valid_discriminator(&wrong_length));
    /// ```
    pub fn is_valid_discriminator(discriminator: &[u8]) -> bool {
        discriminator.len() == 8 && discriminator.iter().any(|&b| b != 0)
    }

    /// Generate a deterministic test program ID
    ///
    /// # Arguments
    ///
    /// * `seed` - A string seed for program ID generation
    ///
    /// # Returns
    ///
    /// A deterministic `Pubkey` that can be used as a program ID in tests.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let program_id = utils::create_test_program_id("my_test_program");
    /// assert!(!utils::is_default_pubkey(&program_id));
    /// ```
    pub fn create_test_program_id(seed: &str) -> Pubkey {
        let mut seed_bytes = [0u8; 32];
        let seed_slice = seed.as_bytes();
        let copy_len = seed_slice.len().min(32);
        seed_bytes[..copy_len].copy_from_slice(&seed_slice[..copy_len]);
        Pubkey::from(seed_bytes)
    }
}

/// Test result aggregation and reporting
pub mod reporting {
    use super::*;
    use std::fs;

    /// Generate a comprehensive test report
    pub fn generate_test_report(
        results: &[TestResultReport],
        title: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut report = String::new();

        // Header
        report.push_str(&format!("# {}\n\n", title));
        report.push_str(&format!(
            "**Generated**: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Summary
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;
        let total_assertions: usize = results.iter().map(|r| r.assertions_run).sum();

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total Tests**: {}\n", total));
        report.push_str(&format!(
            "- **Passed**: {} ({}%)\n",
            passed,
            (passed * 100) / total
        ));
        report.push_str(&format!("- **Failed**: {}\n", failed));
        report.push_str(&format!("- **Total Assertions**: {}\n\n", total_assertions));

        if passed == total {
            report.push_str("✅ **ALL TESTS PASSED!**\n\n");
        } else {
            report.push_str("❌ **SOME TESTS FAILED**\n\n");
        }

        // Results table
        report.push_str("## Test Results\n\n");
        report.push_str("| Test | Status | Assertions | Details |\n");
        report.push_str("|------|--------|------------|----------|\n");

        for result in results {
            let status = if result.passed {
                "✅ PASS"
            } else {
                "❌ FAIL"
            };
            let error = result.error.as_deref().unwrap_or("-");
            report.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                result.name, status, result.assertions_run, error
            ));
        }

        report.push_str("\n## Details\n\n");
        for result in results {
            report.push_str(&format!(
                "### {} - {}\n\n",
                if result.passed { "✅" } else { "❌" },
                result.name
            ));
            report.push_str(&format!(
                "- **Status**: {}\n",
                if result.passed { "PASS" } else { "FAIL" }
            ));
            report.push_str(&format!(
                "- **Assertions Run**: {}\n",
                result.assertions_run
            ));
            if let Some(error) = &result.error {
                report.push_str(&format!("- **Error**: {}\n", error));
            }
            report.push_str("\n");
        }

        // Write to file
        fs::create_dir_all("../../tests/reports").ok();
        fs::write(output_path, &report)?;

        Ok(())
    }
}

/// Common test assertions
pub mod assertions {
    use super::*;

    /// Assert that a condition is true, returning a test result
    pub fn assert_true(
        condition: bool,
        test_name: &str,
        error_message: &str,
    ) -> Result<(), TestResultReport> {
        if condition {
            Ok(())
        } else {
            Err(TestResultReport::failure(
                test_name,
                error_message.to_string(),
            ))
        }
    }

    /// Assert that two values are equal, returning a test result
    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(
        actual: &T,
        expected: &T,
        test_name: &str,
        error_message: &str,
    ) -> Result<(), TestResultReport> {
        if actual == expected {
            Ok(())
        } else {
            Err(TestResultReport::failure(
                test_name,
                format!(
                    "{}: expected {:?}, got {:?}",
                    error_message, expected, actual
                ),
            ))
        }
    }

    /// Assert that a pubkey is not the default (all zeros)
    pub fn assert_not_default_pubkey(
        pubkey: &Pubkey,
        test_name: &str,
        field_name: &str,
    ) -> Result<(), TestResultReport> {
        if *pubkey == Pubkey::default() {
            Err(TestResultReport::failure(
                test_name,
                format!("{} should not be default pubkey", field_name),
            ))
        } else {
            Ok(())
        }
    }
}
