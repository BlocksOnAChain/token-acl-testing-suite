/// Example Allow List Gate Program
/// 
/// This demonstrates how to implement a gate program following the sRFC 37 interface.
/// This allow list program:
/// - Implements can-thaw-permissionless: Returns success if user is in allow list
/// - Optionally implements can-freeze-permissionless: Not supported (returns error)
/// - Creates and manages extra-account-metas PDAs

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

// Discriminators from sRFC 37
const CAN_THAW_PERMISSIONLESS_DISCRIMINATOR: [u8; 8] = [8, 175, 169, 129, 137, 74, 61, 241];
const CAN_FREEZE_PERMISSIONLESS_DISCRIMINATOR: [u8; 8] = [214, 141, 109, 75, 248, 1, 45, 29];

// Seeds
const ALLOW_LIST_SEED: &[u8] = b"allow-list";
const THAW_EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"thaw-extra-account-metas";

entrypoint!(process_instruction);

/// Allow List record for a user
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AllowListRecord {
    pub mint: Pubkey,
    pub user: Pubkey,
    pub allowed: bool,
    pub added_timestamp: i64,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let discriminator = &instruction_data[0..8];
    
    match discriminator {
        d if d == CAN_THAW_PERMISSIONLESS_DISCRIMINATOR => {
            process_can_thaw_permissionless(program_id, accounts)
        }
        d if d == CAN_FREEZE_PERMISSIONLESS_DISCRIMINATOR => {
            // Allow list doesn't support permissionless freeze
            msg!("Permissionless freeze not supported by allow list");
            Err(ProgramError::InvalidInstructionData)
        }
        _ => {
            msg!("Unknown instruction");
            Err(ProgramError::InvalidInstructionData)
        }
    }
}

fn process_can_thaw_permissionless(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Accounts as per sRFC 37 interface:
    // 0. caller
    // 1. token account
    // 2. mint
    // 3. extra-account-metas
    // Extra accounts (from extra-account-metas):
    // 4. token account owner
    // 5. allow list PDA
    
    let _caller = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let _extra_account_metas = next_account_info(account_info_iter)?;
    let token_account_owner = next_account_info(account_info_iter)?;
    let allow_list_pda = next_account_info(account_info_iter)?;
    
    // Verify allow list PDA derivation
    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[
            ALLOW_LIST_SEED,
            mint.key.as_ref(),
            token_account_owner.key.as_ref(),
        ],
        program_id,
    );
    
    if *allow_list_pda.key != expected_pda {
        msg!("Invalid allow list PDA");
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check if allow list record exists and user is allowed
    if allow_list_pda.data_is_empty() {
        msg!("User not in allow list");
        return Err(ProgramError::InvalidAccountData);
    }
    
    let record = AllowListRecord::try_from_slice(&allow_list_pda.data.borrow())?;
    
    if !record.allowed {
        msg!("User {} is not allowed", token_account_owner.key);
        return Err(ProgramError::InvalidAccountData);
    }
    
    msg!("✅ User {} is in allow list - permissionless thaw authorized", token_account_owner.key);
    Ok(())
}

// Helper function to create allow list record (would be called by issuer/admin)
pub fn create_allow_list_record(
    mint: &Pubkey,
    user: &Pubkey,
    timestamp: i64,
) -> AllowListRecord {
    AllowListRecord {
        mint: *mint,
        user: *user,
        allowed: true,
        added_timestamp: timestamp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_allow_list_record_serialization() {
        let mint = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        let record = create_allow_list_record(&mint, &user, 1234567890);
        
        let serialized = record.try_to_vec().unwrap();
        let deserialized = AllowListRecord::try_from_slice(&serialized).unwrap();
        
        assert_eq!(deserialized.mint, mint);
        assert_eq!(deserialized.user, user);
        assert!(deserialized.allowed);
    }
    
    #[test]
    fn test_discriminators() {
        // Verify discriminators match sRFC 37 spec
        assert_eq!(CAN_THAW_PERMISSIONLESS_DISCRIMINATOR, [8, 175, 169, 129, 137, 74, 61, 241]);
        assert_eq!(CAN_FREEZE_PERMISSIONLESS_DISCRIMINATOR, [214, 141, 109, 75, 248, 1, 45, 29]);
    }
}

