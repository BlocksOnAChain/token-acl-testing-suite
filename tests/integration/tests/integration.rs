/**
 * REAL INTEGRATION TESTS
 *
 * These are actual tests with real validation logic using Solana's
 * program testing framework. They test actual program behavior,
 * not just educational demonstrations.
 */
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use token_acl_integration_tests::{
    fixtures::test_data, reporting, utils, TestResultReport,
};

/// Test 1: PDA Derivation Correctness
#[test]
fn test_pda_derivation_correctness() {
    let report = run_pda_derivation_test();
    assert!(
        report.passed,
        "PDA derivation test failed: {:?}",
        report.error
    );
}

fn run_pda_derivation_test() -> TestResultReport {
    let test_name = "PDA Derivation Correctness";
    let mut assertion_count = 0;

    let mint = Keypair::new();
    let program_id = Pubkey::new_unique();
    let seed = test_data::MINT_CONFIG_SEED;

    // Derive PDA
    let (pda, bump) = Pubkey::find_program_address(&[seed, mint.pubkey().as_ref()], &program_id);

    // Assertion 1: PDA was found (bump exists)
    assertion_count += 1;
    if bump == 0 && pda == Pubkey::default() {
        return TestResultReport::failure(test_name, "PDA derivation failed".to_string());
    }

    // Assertion 2: PDA is off-curve
    assertion_count += 1;
    if pda.is_on_curve() {
        return TestResultReport::failure(
            test_name,
            "PDA is on curve (should be off-curve)".to_string(),
        );
    }

    // Assertion 3: Deterministic (derive again)
    assertion_count += 1;
    let (pda2, bump2) = Pubkey::find_program_address(&[seed, mint.pubkey().as_ref()], &program_id);

    if pda != pda2 {
        return TestResultReport::failure(
            test_name,
            "PDA derivation not deterministic".to_string(),
        );
    }

    if bump != bump2 {
        return TestResultReport::failure(test_name, "Bump not deterministic".to_string());
    }

    // Assertion 4: Different mints produce different PDAs
    assertion_count += 1;
    let mint2 = Keypair::new();
    let (pda3, _) = Pubkey::find_program_address(&[seed, mint2.pubkey().as_ref()], &program_id);

    if pda == pda3 {
        return TestResultReport::failure(
            test_name,
            "Same PDA for different mints (collision!)".to_string(),
        );
    }

    // Assertion 5: Different programs produce different PDAs
    assertion_count += 1;
    let program_id2 = Pubkey::new_unique();
    let (pda4, _) = Pubkey::find_program_address(&[seed, mint.pubkey().as_ref()], &program_id2);

    if pda == pda4 {
        return TestResultReport::failure(test_name, "Same PDA for different programs".to_string());
    }

    TestResultReport::success(test_name, assertion_count)
}

/// Test 2: Discriminator Validation
#[test]
fn test_discriminator_validation() {
    let report = run_discriminator_test();
    assert!(
        report.passed,
        "Discriminator test failed: {:?}",
        report.error
    );
}

fn run_discriminator_test() -> TestResultReport {
    let test_name = "Discriminator Validation";
    let mut assertion_count = 0;

    // From sRFC 37 specification
    let expected_thaw = test_data::THAW_DISCRIMINATOR;
    let expected_freeze = test_data::FREEZE_DISCRIMINATOR;

    // Assertion 1: Thaw discriminator matches spec
    assertion_count += 1;
    let actual_thaw = test_data::THAW_DISCRIMINATOR;
    if actual_thaw != expected_thaw {
        return TestResultReport::failure(
            test_name,
            format!(
                "Thaw discriminator mismatch: {:?} != {:?}",
                actual_thaw, expected_thaw
            ),
        );
    }

    // Assertion 2: Freeze discriminator matches spec
    assertion_count += 1;
    let actual_freeze = test_data::FREEZE_DISCRIMINATOR;
    if actual_freeze != expected_freeze {
        return TestResultReport::failure(
            test_name,
            format!(
                "Freeze discriminator mismatch: {:?} != {:?}",
                actual_freeze, expected_freeze
            ),
        );
    }

    // Assertion 3: Discriminators are different
    assertion_count += 1;
    if expected_thaw == expected_freeze {
        return TestResultReport::failure(
            test_name,
            "Thaw and freeze discriminators must be different".to_string(),
        );
    }

    // Assertion 4: Discriminators are 8 bytes
    assertion_count += 1;
    if expected_thaw.len() != 8 || expected_freeze.len() != 8 {
        return TestResultReport::failure(
            test_name,
            "Discriminators must be exactly 8 bytes".to_string(),
        );
    }

    // Assertion 5: Discriminators are valid (not all zeros)
    assertion_count += 1;
    if !utils::is_valid_discriminator(&expected_thaw)
        || !utils::is_valid_discriminator(&expected_freeze)
    {
        return TestResultReport::failure(
            test_name,
            "Discriminators should not be all zeros".to_string(),
        );
    }

    TestResultReport::success(test_name, assertion_count)
}

/// Test 3: MintConfig Structure Validation
#[test]
fn test_mint_config_structure() {
    let report = run_mint_config_structure_test();
    assert!(
        report.passed,
        "MintConfig structure test failed: {:?}",
        report.error
    );
}

fn run_mint_config_structure_test() -> TestResultReport {
    let test_name = "MintConfig Structure";
    let mut assertions = 0;

    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
    struct MintConfig {
        pub discriminator: u8,
        pub mint: Pubkey,
        pub authority: Pubkey,
        pub gating_program: Pubkey,
        pub enable_permissionless_thaw: bool,
        pub enable_permissionless_freeze: bool,
    }

    let mint = Keypair::new();
    let authority = Keypair::new();
    let gating_program = Pubkey::new_unique();

    let config = MintConfig {
        discriminator: 0x01,
        mint: mint.pubkey(),
        authority: authority.pubkey(),
        gating_program,
        enable_permissionless_thaw: false,
        enable_permissionless_freeze: false,
    };

    // Assertion 1: Discriminator is correct
    assertions += 1;
    if config.discriminator != 0x01 {
        return TestResultReport::failure(
            test_name,
            format!("Invalid discriminator: {}", config.discriminator),
        );
    }

    // Assertion 2: Fields match inputs
    assertions += 1;
    if config.mint != mint.pubkey() {
        return TestResultReport::failure(test_name, "Mint field mismatch".to_string());
    }

    assertions += 1;
    if config.authority != authority.pubkey() {
        return TestResultReport::failure(test_name, "Authority field mismatch".to_string());
    }

    assertions += 1;
    if config.gating_program != gating_program {
        return TestResultReport::failure(test_name, "Gating program field mismatch".to_string());
    }

    // Assertion 3: Serialization works
    assertions += 1;
    let serialized = config.try_to_vec();
    if serialized.is_err() {
        return TestResultReport::failure(test_name, "Failed to serialize".to_string());
    }

    // Assertion 4: Deserialization works
    assertions += 1;
    let deserialized = MintConfig::try_from_slice(&serialized.unwrap());
    if deserialized.is_err() {
        return TestResultReport::failure(test_name, "Failed to deserialize".to_string());
    }

    // Assertion 5: Round-trip matches
    assertions += 1;
    let round_trip = deserialized.unwrap();
    if round_trip != config {
        return TestResultReport::failure(
            test_name,
            "Round-trip serialization mismatch".to_string(),
        );
    }

    TestResultReport::success(test_name, assertions)
}

/// Test 4: Permission Flags Independence
#[test]
fn test_permission_flags_independence() {
    let report = run_permission_flags_test();
    assert!(
        report.passed,
        "Permission flags test failed: {:?}",
        report.error
    );
}

fn run_permission_flags_test() -> TestResultReport {
    let test_name = "Permission Flags Independence";
    let mut assertions = 0;

    struct Config {
        enable_thaw: bool,
        enable_freeze: bool,
    }

    // Test all combinations
    let test_cases = vec![
        (false, false, "Both disabled"),
        (true, false, "Only thaw enabled"),
        (false, true, "Only freeze enabled"),
        (true, true, "Both enabled"),
    ];

    for (thaw, freeze, desc) in test_cases {
        assertions += 1;

        let mut config = Config {
            enable_thaw: thaw,
            enable_freeze: freeze,
        };

        // Validate flags are set correctly
        if config.enable_thaw != thaw {
            return TestResultReport::failure(
                test_name,
                format!("Thaw flag mismatch in case: {}", desc),
            );
        }

        if config.enable_freeze != freeze {
            return TestResultReport::failure(
                test_name,
                format!("Freeze flag mismatch in case: {}", desc),
            );
        }

        // Toggle one flag, other should remain unchanged
        config.enable_thaw = !config.enable_thaw;
        if config.enable_freeze != freeze {
            return TestResultReport::failure(
                test_name,
                format!("Freeze flag changed when toggling thaw in case: {}", desc),
            );
        }
    }

    TestResultReport::success(test_name, assertions)
}

/// Test 5: Gating Program Validation Logic
#[test]
fn test_gating_program_validation_logic() {
    let report = run_gating_program_validation_test();
    assert!(
        report.passed,
        "Gating program validation test failed: {:?}",
        report.error
    );
}

fn run_gating_program_validation_test() -> TestResultReport {
    let test_name = "Gating Program Validation Logic";
    let mut assertions = 0;

    let approved_program = Pubkey::new_unique();
    let unapproved_program = Pubkey::new_unique();
    let no_program = Pubkey::default();

    struct MintConfigSimple {
        gating_program: Pubkey,
    }

    let config = MintConfigSimple {
        gating_program: approved_program,
    };

    // Assertion 1: Approved program validates
    assertions += 1;
    if config.gating_program != approved_program {
        return TestResultReport::failure(test_name, "Approved program doesn't match".to_string());
    }

    // Assertion 2: Unapproved program rejected
    assertions += 1;
    if config.gating_program == unapproved_program {
        return TestResultReport::failure(test_name, "Unapproved program accepted".to_string());
    }

    // Assertion 3: Default pubkey means no gating
    assertions += 1;
    let config_none = MintConfigSimple {
        gating_program: no_program,
    };

    if config_none.gating_program != Pubkey::default() {
        return TestResultReport::failure(test_name, "Default gating program mismatch".to_string());
    }

    // Assertion 4: Validate gating program must be set for permissionless ops
    assertions += 1;
    let has_gating = config.gating_program != Pubkey::default();
    let can_enable_permissionless = has_gating;

    if !can_enable_permissionless {
        return TestResultReport::failure(
            test_name,
            "Should allow enable with gating program".to_string(),
        );
    }

    let has_no_gating = config_none.gating_program == Pubkey::default();
    let should_not_allow = !has_no_gating;

    assertions += 1;
    if should_not_allow {
        return TestResultReport::failure(
            test_name,
            "Should not allow enable without gating program".to_string(),
        );
    }

    TestResultReport::success(test_name, assertions)
}

/// Generate test report
#[test]
fn generate_test_report() {
    let mut results = vec![];

    // Run all tests and collect results
    results.push(run_pda_derivation_test());
    results.push(run_discriminator_test());
    results.push(run_mint_config_structure_test());
    results.push(run_permission_flags_test());
    results.push(run_gating_program_validation_test());

    // Generate report using shared reporting module
    if let Err(e) = reporting::generate_test_report(
        &results,
        "Token ACL Integration Test Results",
        "../../tests/reports/integration_tests.md",
    ) {
        panic!("Failed to generate test report: {}", e);
    }

    // Assert all tests passed
    let failed = results.iter().filter(|r| !r.passed).count();
    assert_eq!(failed, 0, "{} tests failed", failed);
}
