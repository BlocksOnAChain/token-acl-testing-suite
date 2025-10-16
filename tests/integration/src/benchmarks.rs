//! Performance benchmarks and optimization utilities
//!
//! This module provides benchmarking capabilities for measuring
//! the performance of Token ACL operations and identifying optimization opportunities.

use solana_sdk::pubkey::Pubkey;
use std::time::{Duration, Instant};

/// Benchmark result with timing information
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub iterations: usize,
    pub avg_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub success: bool,
    pub error: Option<String>,
}

impl BenchmarkResult {
    /// Create a successful benchmark result
    pub fn success(
        name: &str,
        duration: Duration,
        iterations: usize,
        min_duration: Duration,
        max_duration: Duration,
    ) -> Self {
        let avg_duration = Duration::from_nanos(duration.as_nanos() as u64 / iterations as u64);

        Self {
            name: name.to_string(),
            duration,
            iterations,
            avg_duration,
            min_duration,
            max_duration,
            success: true,
            error: None,
        }
    }

    /// Create a failed benchmark result
    pub fn failure(name: &str, error: String) -> Self {
        Self {
            name: name.to_string(),
            duration: Duration::ZERO,
            iterations: 0,
            avg_duration: Duration::ZERO,
            min_duration: Duration::ZERO,
            max_duration: Duration::ZERO,
            success: false,
            error: Some(error),
        }
    }
}

/// Benchmark runner for measuring operation performance
pub struct BenchmarkRunner {
    name: String,
    iterations: usize,
    warmup_iterations: usize,
}

impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            iterations: 1000,
            warmup_iterations: 100,
        }
    }

    /// Set the number of iterations
    pub fn iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    /// Set the number of warmup iterations
    pub fn warmup_iterations(mut self, warmup_iterations: usize) -> Self {
        self.warmup_iterations = warmup_iterations;
        self
    }

    /// Run a benchmark with the given function
    pub fn run<F>(self, mut benchmark_fn: F) -> BenchmarkResult
    where
        F: FnMut() -> Result<(), String>,
    {
        // Warmup runs
        for _ in 0..self.warmup_iterations {
            if let Err(e) = benchmark_fn() {
                return BenchmarkResult::failure(&self.name, e);
            }
        }

        // Actual benchmark runs
        let start = Instant::now();
        let mut min_duration = Duration::MAX;
        let mut max_duration = Duration::ZERO;

        for _ in 0..self.iterations {
            let iter_start = Instant::now();

            if let Err(e) = benchmark_fn() {
                return BenchmarkResult::failure(&self.name, e);
            }

            let iter_duration = iter_start.elapsed();
            min_duration = min_duration.min(iter_duration);
            max_duration = max_duration.max(iter_duration);
        }

        let total_duration = start.elapsed();

        BenchmarkResult::success(
            &self.name,
            total_duration,
            self.iterations,
            min_duration,
            max_duration,
        )
    }
}

/// Performance benchmarks for Token ACL operations
pub mod performance_benchmarks {
    use super::*;
    use solana_sdk::signature::{Keypair, Signer};

    /// Benchmark PDA derivation performance
    pub fn benchmark_pda_derivation() -> BenchmarkResult {
        BenchmarkRunner::new("PDA Derivation")
            .iterations(10000)
            .warmup_iterations(1000)
            .run(|| {
                let mint = Keypair::new();
                let program_id = Pubkey::new_unique();
                let seed = b"MINT_CFG";

                let (pda, _bump) =
                    Pubkey::find_program_address(&[seed, mint.pubkey().as_ref()], &program_id);

                if pda == Pubkey::default() {
                    return Err("PDA derivation failed".to_string());
                }

                Ok(())
            })
    }

    /// Benchmark discriminator validation performance
    pub fn benchmark_discriminator_validation() -> BenchmarkResult {
        BenchmarkRunner::new("Discriminator Validation")
            .iterations(50000)
            .warmup_iterations(5000)
            .run(|| {
                let thaw_discriminator = [8, 175, 169, 129, 137, 74, 61, 241];
                let freeze_discriminator = [214, 141, 109, 75, 248, 1, 45, 29];

                if thaw_discriminator.len() != 8 || freeze_discriminator.len() != 8 {
                    return Err("Invalid discriminator length".to_string());
                }

                if thaw_discriminator == freeze_discriminator {
                    return Err("Discriminators should be different".to_string());
                }

                Ok(())
            })
    }

    /// Benchmark serialization/deserialization performance
    pub fn benchmark_serialization() -> BenchmarkResult {
        use borsh::{BorshDeserialize, BorshSerialize};

        #[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
        struct TestStruct {
            pub field1: u64,
            pub field2: Pubkey,
            pub field3: bool,
            pub field4: Vec<u8>,
        }

        BenchmarkRunner::new("Serialization/Deserialization")
            .iterations(10000)
            .warmup_iterations(1000)
            .run(|| {
                let test_data = TestStruct {
                    field1: 12345,
                    field2: Pubkey::new_unique(),
                    field3: true,
                    field4: vec![1, 2, 3, 4, 5],
                };

                let serialized = test_data
                    .try_to_vec()
                    .map_err(|e| format!("Serialization failed: {}", e))?;

                let deserialized: TestStruct = BorshDeserialize::try_from_slice(&serialized)
                    .map_err(|e| format!("Deserialization failed: {}", e))?;

                if deserialized.field1 != test_data.field1 {
                    return Err("Round-trip serialization failed".to_string());
                }

                Ok(())
            })
    }

    /// Benchmark account validation performance
    pub fn benchmark_account_validation() -> BenchmarkResult {
        BenchmarkRunner::new("Account Validation")
            .iterations(20000)
            .warmup_iterations(2000)
            .run(|| {
                let account = Keypair::new();
                let authority = Keypair::new();

                // Simulate account validation logic
                if account.pubkey() == Pubkey::default() {
                    return Err("Invalid account".to_string());
                }

                if authority.pubkey() == Pubkey::default() {
                    return Err("Invalid authority".to_string());
                }

                // Simulate permission check
                let is_authorized = account.pubkey() == authority.pubkey();
                if !is_authorized {
                    // This is expected in most cases, so we'll just continue
                }

                Ok(())
            })
    }

    /// Run all performance benchmarks
    pub fn run_all_benchmarks() -> Vec<BenchmarkResult> {
        vec![
            benchmark_pda_derivation(),
            benchmark_discriminator_validation(),
            benchmark_serialization(),
            benchmark_account_validation(),
        ]
    }
}

/// Performance analysis and reporting
pub mod performance_analysis {
    use super::*;
    use std::fs;

    /// Generate performance report
    pub fn generate_performance_report(
        results: &[BenchmarkResult],
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut report = String::new();

        // Header
        report.push_str("# Token ACL Performance Benchmarks\n\n");
        report.push_str(&format!(
            "**Generated**: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Summary
        let total_benchmarks = results.len();
        let successful_benchmarks = results.iter().filter(|r| r.success).count();
        let failed_benchmarks = total_benchmarks - successful_benchmarks;

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total Benchmarks**: {}\n", total_benchmarks));
        report.push_str(&format!("- **Successful**: {}\n", successful_benchmarks));
        report.push_str(&format!("- **Failed**: {}\n", failed_benchmarks));

        if failed_benchmarks == 0 {
            report.push_str("✅ **ALL BENCHMARKS PASSED!**\n\n");
        } else {
            report.push_str("❌ **SOME BENCHMARKS FAILED**\n\n");
        }

        // Results table
        report.push_str("## Benchmark Results\n\n");
        report.push_str(
            "| Benchmark | Status | Iterations | Avg Time | Min Time | Max Time | Total Time |\n",
        );
        report.push_str(
            "|-----------|--------|------------|----------|----------|----------|------------|\n",
        );

        for result in results {
            let status = if result.success {
                "✅ PASS"
            } else {
                "❌ FAIL"
            };
            let avg_time = format!("{:.2}μs", result.avg_duration.as_micros());
            let min_time = format!("{:.2}μs", result.min_duration.as_micros());
            let max_time = format!("{:.2}μs", result.max_duration.as_micros());
            let total_time = format!("{:.2}ms", result.duration.as_millis());

            report.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                result.name, status, result.iterations, avg_time, min_time, max_time, total_time
            ));
        }

        // Performance analysis
        report.push_str("\n## Performance Analysis\n\n");

        if let Some(fastest) = results
            .iter()
            .filter(|r| r.success)
            .min_by_key(|r| r.avg_duration)
        {
            report.push_str(&format!(
                "**Fastest Operation**: {} (avg: {:.2}μs)\n\n",
                fastest.name,
                fastest.avg_duration.as_micros()
            ));
        }

        if let Some(slowest) = results
            .iter()
            .filter(|r| r.success)
            .max_by_key(|r| r.avg_duration)
        {
            report.push_str(&format!(
                "**Slowest Operation**: {} (avg: {:.2}μs)\n\n",
                slowest.name,
                slowest.avg_duration.as_micros()
            ));
        }

        // Optimization recommendations
        report.push_str("## Optimization Recommendations\n\n");

        for result in results.iter().filter(|r| r.success) {
            if result.avg_duration.as_micros() > 1000 {
                report.push_str(&format!(
                    "- **{}**: Consider optimization (avg: {:.2}μs)\n",
                    result.name,
                    result.avg_duration.as_micros()
                ));
            }
        }

        // Write to file
        fs::create_dir_all("../../tests/reports").ok();
        fs::write(output_path, &report)?;

        Ok(())
    }
}
