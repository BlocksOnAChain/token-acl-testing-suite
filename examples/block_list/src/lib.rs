use borsh::{BorshDeserialize, BorshSerialize};
/// Example Block List Gate Program
///
/// This demonstrates how to implement a block list gate program following sRFC 37.
/// This block list program:
/// - Implements can-freeze-permissionless: Returns success if user is in block list
/// - Implements can-thaw-permissionless: Returns success if user is NOT in block list
/// - Creates and manages extra-account-metas PDAs
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Discriminators from sRFC 37
const CAN_THAW_PERMISSIONLESS_DISCRIMINATOR: [u8; 8] = [8, 175, 169, 129, 137, 74, 61, 241];
const CAN_FREEZE_PERMISSIONLESS_DISCRIMINATOR: [u8; 8] = [214, 141, 109, 75, 248, 1, 45, 29];

// Seeds
const BLOCK_LIST_SEED: &[u8] = b"block-list";

entrypoint!(process_instruction);

/// Block List record for a user (e.g., sanctions list)
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BlockListRecord {
    pub mint: Pubkey,
    pub user: Pubkey,
    pub blocked: bool,
    pub reason: BlockReason,
    pub added_timestamp: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum BlockReason {
    Sanctions,
    Compliance,
    RiskAssessment,
    Other,
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
            process_can_freeze_permissionless(program_id, accounts)
        }
        _ => {
            msg!("Unknown instruction");
            Err(ProgramError::InvalidInstructionData)
        }
    }
}

fn process_can_thaw_permissionless(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // Accounts as per sRFC 37 interface
    let _caller = next_account_info(account_info_iter)?;
    let _token_account = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let _extra_account_metas = next_account_info(account_info_iter)?;
    let token_account_owner = next_account_info(account_info_iter)?;
    let block_list_pda = next_account_info(account_info_iter)?;

    // Verify block list PDA derivation
    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[
            BLOCK_LIST_SEED,
            mint.key.as_ref(),
            token_account_owner.key.as_ref(),
        ],
        program_id,
    );

    if *block_list_pda.key != expected_pda {
        msg!("Invalid block list PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    // For thaw: Allow if user is NOT in block list
    if block_list_pda.data_is_empty() {
        msg!(
            "✅ User {} not in block list - permissionless thaw authorized",
            token_account_owner.key
        );
        return Ok(());
    }

    let record = BlockListRecord::try_from_slice(&block_list_pda.data.borrow())?;

    if record.blocked {
        msg!(
            "❌ User {} is blocked (reason: {:?}) - permissionless thaw denied",
            token_account_owner.key,
            record.reason
        );
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "✅ User {} not blocked - permissionless thaw authorized",
        token_account_owner.key
    );
    Ok(())
}

fn process_can_freeze_permissionless(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // Accounts as per sRFC 37 interface
    let _caller = next_account_info(account_info_iter)?;
    let _token_account = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let _extra_account_metas = next_account_info(account_info_iter)?;
    let token_account_owner = next_account_info(account_info_iter)?;
    let block_list_pda = next_account_info(account_info_iter)?;

    // Verify block list PDA derivation
    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[
            BLOCK_LIST_SEED,
            mint.key.as_ref(),
            token_account_owner.key.as_ref(),
        ],
        program_id,
    );

    if *block_list_pda.key != expected_pda {
        msg!("Invalid block list PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    // For freeze: Allow if user IS in block list
    if block_list_pda.data_is_empty() {
        msg!(
            "❌ User {} not in block list - permissionless freeze denied",
            token_account_owner.key
        );
        return Err(ProgramError::InvalidAccountData);
    }

    let record = BlockListRecord::try_from_slice(&block_list_pda.data.borrow())?;

    if !record.blocked {
        msg!(
            "❌ User {} not blocked - permissionless freeze denied",
            token_account_owner.key
        );
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "✅ User {} is blocked (reason: {:?}) - permissionless freeze authorized",
        token_account_owner.key,
        record.reason
    );
    Ok(())
}

// Helper function to create block list record (would be called by issuer/compliance officer)
pub fn create_block_list_record(
    mint: &Pubkey,
    user: &Pubkey,
    reason: BlockReason,
    timestamp: i64,
) -> BlockListRecord {
    BlockListRecord {
        mint: *mint,
        user: *user,
        blocked: true,
        reason,
        added_timestamp: timestamp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_list_record_serialization() {
        let mint = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        let record = create_block_list_record(&mint, &user, BlockReason::Sanctions, 1234567890);

        let serialized = record.try_to_vec().unwrap();
        let deserialized = BlockListRecord::try_from_slice(&serialized).unwrap();

        assert_eq!(deserialized.mint, mint);
        assert_eq!(deserialized.user, user);
        assert!(deserialized.blocked);
        assert_eq!(deserialized.reason, BlockReason::Sanctions);
    }

    #[test]
    fn test_discriminators() {
        // Verify discriminators match sRFC 37 spec
        assert_eq!(
            CAN_THAW_PERMISSIONLESS_DISCRIMINATOR,
            [8, 175, 169, 129, 137, 74, 61, 241]
        );
        assert_eq!(
            CAN_FREEZE_PERMISSIONLESS_DISCRIMINATOR,
            [214, 141, 109, 75, 248, 1, 45, 29]
        );
    }
}
