//! Test fixtures and data
//!
//! This module provides test data, mock objects, and fixtures used across
//! the test suite for consistent testing scenarios.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

/// Test data for common scenarios
pub mod test_data {
    use super::*;

    /// Standard test discriminators from sRFC 37
    pub const THAW_DISCRIMINATOR: [u8; 8] = [8, 175, 169, 129, 137, 74, 61, 241];
    pub const FREEZE_DISCRIMINATOR: [u8; 8] = [214, 141, 109, 75, 248, 1, 45, 29];

    /// Standard seeds for PDA derivation
    pub const MINT_CONFIG_SEED: &[u8] = b"MINT_CFG";
    pub const ALLOW_LIST_SEED: &[u8] = b"allow-list";
    pub const BLOCK_LIST_SEED: &[u8] = b"block-list";
    pub const THAW_EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"thaw-extra-account-metas";
    pub const FREEZE_EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"freeze-extra-account-metas";

    /// Test mint configuration
    pub fn create_test_mint_config(
        mint: Pubkey,
        authority: Pubkey,
        gating_program: Pubkey,
    ) -> TestMintConfig {
        TestMintConfig {
            discriminator: 0x01,
            mint,
            authority,
            gating_program,
            enable_permissionless_thaw: true,
            enable_permissionless_freeze: false,
        }
    }
}

/// Mock MintConfig for testing
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct TestMintConfig {
    pub discriminator: u8,
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub gating_program: Pubkey,
    pub enable_permissionless_thaw: bool,
    pub enable_permissionless_freeze: bool,
}

/// Mock AllowListRecord for testing
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TestAllowListRecord {
    pub mint: Pubkey,
    pub user: Pubkey,
    pub allowed: bool,
    pub added_timestamp: i64,
    pub bump: u8,
}

/// Mock BlockListRecord for testing
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TestBlockListRecord {
    pub mint: Pubkey,
    pub user: Pubkey,
    pub blocked: bool,
    pub reason: BlockReason,
    pub added_timestamp: i64,
    pub bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum BlockReason {
    Sanctions,
    Compliance,
    RiskAssessment,
    Other,
}

/// Test scenarios and use cases
pub mod scenarios {
    use super::*;

    /// KYC allowlist scenario
    pub struct KYCScenario {
        pub user: Pubkey,
        pub kyc_complete: bool,
        pub accredited: bool,
        pub expiration_timestamp: Option<i64>,
    }

    impl KYCScenario {
        pub fn new_valid_user(user: Pubkey) -> Self {
            Self {
                user,
                kyc_complete: true,
                accredited: true,
                expiration_timestamp: None,
            }
        }

        pub fn new_expired_user(user: Pubkey) -> Self {
            Self {
                user,
                kyc_complete: true,
                accredited: true,
                expiration_timestamp: Some(1000), // Expired
            }
        }

        pub fn new_non_accredited_user(user: Pubkey) -> Self {
            Self {
                user,
                kyc_complete: true,
                accredited: false,
                expiration_timestamp: None,
            }
        }
    }

    /// Sanctions scenario
    pub struct SanctionsScenario {
        pub user: Pubkey,
        pub in_sanctions: bool,
        pub in_allowlist: bool,
    }

    impl SanctionsScenario {
        pub fn new_sanctioned_user(user: Pubkey) -> Self {
            Self {
                user,
                in_sanctions: true,
                in_allowlist: false,
            }
        }

        pub fn new_allowlisted_user(user: Pubkey) -> Self {
            Self {
                user,
                in_sanctions: false,
                in_allowlist: true,
            }
        }

        pub fn new_sanctioned_but_allowlisted_user(user: Pubkey) -> Self {
            Self {
                user,
                in_sanctions: true,
                in_allowlist: true,
            }
        }
    }

    /// Geo-blocking scenario
    pub struct GeoBlockingScenario {
        pub user: Pubkey,
        pub jurisdiction: Jurisdiction,
        pub allowed_jurisdictions: Vec<Jurisdiction>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Jurisdiction {
        US,
        EU,
        OFAC, // Sanctioned countries
        Other,
    }

    impl GeoBlockingScenario {
        pub fn new_us_user(user: Pubkey) -> Self {
            Self {
                user,
                jurisdiction: Jurisdiction::US,
                allowed_jurisdictions: vec![Jurisdiction::US, Jurisdiction::EU],
            }
        }

        pub fn new_ofac_user(user: Pubkey) -> Self {
            Self {
                user,
                jurisdiction: Jurisdiction::OFAC,
                allowed_jurisdictions: vec![Jurisdiction::US, Jurisdiction::EU],
            }
        }
    }
}

/// Performance test data
pub mod performance {

    /// Expected compute unit usage for different operations
    pub const TRANSFER_CU_TRANSFER_HOOK: u32 = 50_000;
    pub const TRANSFER_CU_TOKEN_ACL: u32 = 5_000;
    pub const THAW_PERMISSIONLESS_CU: u32 = 8_000;
    pub const FREEZE_PERMISSIONLESS_CU: u32 = 8_000;
    pub const PERMISSIONED_FREEZE_CU: u32 = 3_000;

    /// Expected account counts for different operations
    pub const TRANSFER_ACCOUNTS_TRANSFER_HOOK: usize = 8;
    pub const TRANSFER_ACCOUNTS_TOKEN_ACL: usize = 3;
    pub const COMPLEX_DEFI_ACCOUNTS_TRANSFER_HOOK: usize = 39;
    pub const COMPLEX_DEFI_ACCOUNTS_TOKEN_ACL: usize = 15;

    /// Time benchmarks
    pub const MANUAL_THAW_TIME_SECONDS: u64 = 3600; // 1 hour
    pub const PERMISSIONLESS_THAW_TIME_SECONDS: u64 = 5; // 5 seconds
}
