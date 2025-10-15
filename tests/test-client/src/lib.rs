use solana_sdk::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};

pub mod managed_freeze_authority;
pub mod permissionless_operations;
pub mod gate_program_interface;
pub mod composability;
pub mod security;

// Constants from sRFC 37 specification
pub const MINT_CONFIG_SEED: &[u8] = b"MINT_CFG";
pub const THAW_EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"thaw-extra-account-metas";
pub const FREEZE_EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"freeze-extra-account-metas";

// Discriminators from sRFC 37
pub const PERMISSIONLESS_THAW_DISCRIMINATOR: [u8; 8] = [8, 175, 169, 129, 137, 74, 61, 241];
pub const PERMISSIONLESS_FREEZE_DISCRIMINATOR: [u8; 8] = [214, 141, 109, 75, 248, 1, 45, 29];

/// MintConfig account structure as per sRFC 37
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct MintConfig {
    pub discriminator: u8,
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub gating_program: Pubkey,
    pub enable_permissionless_thaw: bool,
    pub enable_permissionless_freeze: bool,
}

impl MintConfig {
    pub const DISCRIMINATOR: u8 = 0x01;
    
    pub fn new(
        mint: Pubkey,
        authority: Pubkey,
        gating_program: Option<Pubkey>,
    ) -> Self {
        Self {
            discriminator: Self::DISCRIMINATOR,
            mint,
            authority,
            gating_program: gating_program.unwrap_or(Pubkey::default()),
            enable_permissionless_thaw: false,
            enable_permissionless_freeze: false,
        }
    }
    
    pub fn find_pda(mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[MINT_CONFIG_SEED, mint.as_ref()],
            program_id,
        )
    }
}

/// Test results tracker
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub metrics: Option<TestMetrics>,
}

#[derive(Debug, Clone)]
pub struct TestMetrics {
    pub compute_units: u64,
    pub accounts_count: usize,
    pub execution_time_ms: u128,
}

impl TestResult {
    pub fn success(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: true,
            message: message.into(),
            metrics: None,
        }
    }
    
    pub fn failure(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: false,
            message: message.into(),
            metrics: None,
        }
    }
    
    pub fn with_metrics(mut self, metrics: TestMetrics) -> Self {
        self.metrics = Some(metrics);
        self
    }
}

pub struct TestSuite {
    pub results: Vec<TestResult>,
}

impl TestSuite {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    
    pub fn add_result(&mut self, result: TestResult) {
        println!("[{}] {}: {}", 
            if result.passed { "✅" } else { "❌" },
            result.name,
            result.message
        );
        if let Some(metrics) = &result.metrics {
            println!("   Compute Units: {}", metrics.compute_units);
            println!("   Accounts: {}", metrics.accounts_count);
            println!("   Time: {}ms", metrics.execution_time_ms);
        }
        self.results.push(result);
    }
    
    pub fn print_summary(&self) {
        let total = self.results.len();
        let passed = self.results.iter().filter(|r| r.passed).count();
        let failed = total - passed;
        
        println!("\n=== Test Summary ===");
        println!("Total: {}", total);
        println!("Passed: {} ({}%)", passed, (passed * 100) / total);
        println!("Failed: {}", failed);
        
        if failed > 0 {
            println!("\nFailed tests:");
            for result in self.results.iter().filter(|r| !r.passed) {
                println!("  - {}: {}", result.name, result.message);
            }
        }
    }
    
    pub fn generate_report(&self) -> String {
        let mut report = String::from("# sRFC 37 Token ACL Test Report\n\n");
        
        report.push_str("## Summary\n\n");
        let total = self.results.len();
        let passed = self.results.iter().filter(|r| r.passed).count();
        report.push_str(&format!("- Total Tests: {}\n", total));
        report.push_str(&format!("- Passed: {} ({}%)\n", passed, (passed * 100) / total));
        report.push_str(&format!("- Failed: {}\n\n", total - passed));
        
        report.push_str("## Detailed Results\n\n");
        for result in &self.results {
            let status = if result.passed { "✅ PASS" } else { "❌ FAIL" };
            report.push_str(&format!("### {} - {}\n\n", status, result.name));
            report.push_str(&format!("{}\n\n", result.message));
            
            if let Some(metrics) = &result.metrics {
                report.push_str("**Metrics:**\n");
                report.push_str(&format!("- Compute Units: {}\n", metrics.compute_units));
                report.push_str(&format!("- Accounts Count: {}\n", metrics.accounts_count));
                report.push_str(&format!("- Execution Time: {}ms\n\n", metrics.execution_time_ms));
            }
        }
        
        report
    }
}

impl Default for TestSuite {
    fn default() -> Self {
        Self::new()
    }
}

