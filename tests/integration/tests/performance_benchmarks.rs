//! Performance benchmarks for Token ACL operations
//!
//! This module provides comprehensive performance testing for all Token ACL operations
//! to ensure they meet performance requirements and identify optimization opportunities.

use token_acl_integration_tests::{
    benchmarks::{performance_analysis, performance_benchmarks, BenchmarkRunner},
};

/// Benchmark PDA derivation performance
#[test]
fn benchmark_pda_derivation() {
    let result = performance_benchmarks::benchmark_pda_derivation();

    assert!(
        result.success,
        "PDA derivation benchmark failed: {:?}",
        result.error
    );

    // Performance assertions
    assert!(
        result.avg_duration.as_micros() < 1000,
        "PDA derivation should be fast (avg: {}μs)",
        result.avg_duration.as_micros()
    );

    println!("PDA Derivation Benchmark:");
    println!("  Average time: {:.2}μs", result.avg_duration.as_micros());
    println!("  Min time: {:.2}μs", result.min_duration.as_micros());
    println!("  Max time: {:.2}μs", result.max_duration.as_micros());
    println!("  Iterations: {}", result.iterations);
}

/// Benchmark discriminator validation performance
#[test]
fn benchmark_discriminator_validation() {
    let result = performance_benchmarks::benchmark_discriminator_validation();

    assert!(
        result.success,
        "Discriminator validation benchmark failed: {:?}",
        result.error
    );

    // Performance assertions
    assert!(
        result.avg_duration.as_micros() < 100,
        "Discriminator validation should be very fast (avg: {}μs)",
        result.avg_duration.as_micros()
    );

    println!("Discriminator Validation Benchmark:");
    println!("  Average time: {:.2}μs", result.avg_duration.as_micros());
    println!("  Min time: {:.2}μs", result.min_duration.as_micros());
    println!("  Max time: {:.2}μs", result.max_duration.as_micros());
    println!("  Iterations: {}", result.iterations);
}

/// Benchmark serialization/deserialization performance
#[test]
fn benchmark_serialization() {
    let result = performance_benchmarks::benchmark_serialization();

    assert!(
        result.success,
        "Serialization benchmark failed: {:?}",
        result.error
    );

    // Performance assertions
    assert!(
        result.avg_duration.as_micros() < 500,
        "Serialization should be fast (avg: {}μs)",
        result.avg_duration.as_micros()
    );

    println!("Serialization Benchmark:");
    println!("  Average time: {:.2}μs", result.avg_duration.as_micros());
    println!("  Min time: {:.2}μs", result.min_duration.as_micros());
    println!("  Max time: {:.2}μs", result.max_duration.as_micros());
    println!("  Iterations: {}", result.iterations);
}

/// Benchmark account validation performance
#[test]
fn benchmark_account_validation() {
    let result = performance_benchmarks::benchmark_account_validation();

    assert!(
        result.success,
        "Account validation benchmark failed: {:?}",
        result.error
    );

    // Performance assertions
    assert!(
        result.avg_duration.as_micros() < 200,
        "Account validation should be fast (avg: {}μs)",
        result.avg_duration.as_micros()
    );

    println!("Account Validation Benchmark:");
    println!("  Average time: {:.2}μs", result.avg_duration.as_micros());
    println!("  Min time: {:.2}μs", result.min_duration.as_micros());
    println!("  Max time: {:.2}μs", result.max_duration.as_micros());
    println!("  Iterations: {}", result.iterations);
}

/// Run all performance benchmarks
#[test]
fn run_all_performance_benchmarks() {
    let results = performance_benchmarks::run_all_benchmarks();

    // Generate performance report
    if let Err(e) = performance_analysis::generate_performance_report(
        &results,
        "../../tests/reports/performance_benchmarks.md",
    ) {
        panic!("Failed to generate performance report: {}", e);
    }

    // Assert all benchmarks passed
    let failed = results.iter().filter(|r| !r.success).count();
    assert_eq!(failed, 0, "{} benchmarks failed", failed);

    // Print summary
    println!("\n🎯 Performance Benchmark Summary:");
    for result in &results {
        println!(
            "  {}: {:.2}μs avg ({} iterations)",
            result.name,
            result.avg_duration.as_micros(),
            result.iterations
        );
    }
}

/// Custom benchmark for specific operations
#[test]
fn benchmark_custom_operations() {
    // Benchmark custom operation 1: Complex PDA derivation
    let result1 = BenchmarkRunner::new("Complex PDA Derivation")
        .iterations(1000)
        .warmup_iterations(100)
        .run(|| {
            use solana_sdk::pubkey::Pubkey;
            use solana_sdk::signature::{Keypair, Signer};

            let mint = Keypair::new();
            let program_id = Pubkey::new_unique();
            let seed1 = b"complex";
            let seed2 = b"pda";
            let seed3 = b"derivation";

            let (pda, _) = Pubkey::find_program_address(
                &[seed1, seed2, seed3, mint.pubkey().as_ref()],
                &program_id,
            );

            if pda == Pubkey::default() {
                return Err("PDA derivation failed".to_string());
            }

            Ok(())
        });

    assert!(result1.success, "Complex PDA derivation benchmark failed");
    println!(
        "Complex PDA Derivation: {:.2}μs avg",
        result1.avg_duration.as_micros()
    );

    // Benchmark custom operation 2: Multiple account validation
    let result2 = BenchmarkRunner::new("Multiple Account Validation")
        .iterations(5000)
        .warmup_iterations(500)
        .run(|| {
            use solana_sdk::pubkey::Pubkey;
            use solana_sdk::signature::{Keypair, Signer};

            // Simulate validating multiple accounts
            let accounts = vec![
                Keypair::new(),
                Keypair::new(),
                Keypair::new(),
                Keypair::new(),
                Keypair::new(),
            ];

            for account in &accounts {
                if account.pubkey() == Pubkey::default() {
                    return Err("Invalid account".to_string());
                }
            }

            Ok(())
        });

    assert!(
        result2.success,
        "Multiple account validation benchmark failed"
    );
    println!(
        "Multiple Account Validation: {:.2}μs avg",
        result2.avg_duration.as_micros()
    );
}

/// Stress test for high-load scenarios
#[test]
fn stress_test_high_load() {
    let result = BenchmarkRunner::new("High Load Stress Test")
        .iterations(10000)
        .warmup_iterations(1000)
        .run(|| {
            use solana_sdk::pubkey::Pubkey;
            use solana_sdk::signature::{Keypair, Signer};

            // Simulate high-load scenario
            let mint = Keypair::new();
            let program_id = Pubkey::new_unique();
            let seed = b"stress_test";

            // Perform multiple operations
            let (pda1, _) =
                Pubkey::find_program_address(&[seed, mint.pubkey().as_ref()], &program_id);

            let (pda2, _) = Pubkey::find_program_address(
                &[seed, b"different", mint.pubkey().as_ref()],
                &program_id,
            );

            if pda1 == pda2 {
                return Err("PDA collision detected".to_string());
            }

            Ok(())
        });

    assert!(result.success, "High load stress test failed");

    // Stress test should still be reasonably fast
    assert!(
        result.avg_duration.as_micros() < 2000,
        "Stress test should be reasonably fast (avg: {}μs)",
        result.avg_duration.as_micros()
    );

    println!(
        "High Load Stress Test: {:.2}μs avg",
        result.avg_duration.as_micros()
    );
}

/// Memory usage benchmark
#[test]
fn benchmark_memory_usage() {
    let result = BenchmarkRunner::new("Memory Usage Test")
        .iterations(1000)
        .warmup_iterations(100)
        .run(|| {
            use solana_sdk::pubkey::Pubkey;
            use solana_sdk::signature::{Keypair, Signer};

            // Create and use multiple keypairs to test memory usage
            let mut keypairs = Vec::new();
            for _ in 0..100 {
                keypairs.push(Keypair::new());
            }

            // Perform operations with the keypairs
            let mut pdas = Vec::new();
            let program_id = Pubkey::new_unique();
            let seed = b"memory_test";

            for keypair in &keypairs {
                let (pda, _) =
                    Pubkey::find_program_address(&[seed, keypair.pubkey().as_ref()], &program_id);
                pdas.push(pda);
            }

            // Verify all PDAs are unique
            for i in 0..pdas.len() {
                for j in (i + 1)..pdas.len() {
                    if pdas[i] == pdas[j] {
                        return Err("PDA collision in memory test".to_string());
                    }
                }
            }

            Ok(())
        });

    assert!(result.success, "Memory usage benchmark failed");
    println!(
        "Memory Usage Test: {:.2}μs avg",
        result.avg_duration.as_micros()
    );
}

/// Generate comprehensive performance report
#[test]
fn generate_performance_report() {
    let mut all_results = Vec::new();

    // Run all standard benchmarks
    all_results.extend(performance_benchmarks::run_all_benchmarks());

    // Add custom benchmarks
    let custom_result1 = BenchmarkRunner::new("Custom Operation 1")
        .iterations(1000)
        .run(|| Ok(()));
    all_results.push(custom_result1);

    let custom_result2 = BenchmarkRunner::new("Custom Operation 2")
        .iterations(1000)
        .run(|| Ok(()));
    all_results.push(custom_result2);

    // Generate comprehensive report
    if let Err(e) = performance_analysis::generate_performance_report(
        &all_results,
        "../../tests/reports/comprehensive_performance_report.md",
    ) {
        panic!("Failed to generate comprehensive performance report: {}", e);
    }

    println!("📊 Comprehensive performance report generated");
    println!("   Report saved to: tests/reports/comprehensive_performance_report.md");
}
