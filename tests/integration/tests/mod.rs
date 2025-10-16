//! Test modules organization
//!
//! This module organizes all test modules into a clean hierarchy.

mod advanced_scenarios;
mod core_logic;
mod integration;
mod security_tests;
mod test_runner;

// Re-export test modules for easy access
// Note: Individual test modules are available but not re-exported to avoid unused import warnings
