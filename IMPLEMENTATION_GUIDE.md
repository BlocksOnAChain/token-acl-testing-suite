# sRFC 37 Token ACL - Implementation Guide

## For Token Issuers

### Getting Started

If you want to create a permissioned token using Token ACL:

#### Step 1: Create Your Token Mint

```bash
# Create Token22 mint with Default Account State extension
spl-token create-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb \
  --enable-freeze \
  --default-account-state frozen
```

The `--default-account-state frozen` is crucial! This makes all new token accounts frozen by default, which enables the permissionless thaw workflow.

#### Step 2: Deploy or Choose a Gating Program

You have three options:

**Option A: Use a canonical allow list program**
- Deploy the reference allow list implementation
- Manage your allow list by adding/removing users

**Option B: Use a canonical block list program**
- Deploy the reference block list implementation
- Manage your block list for sanctions/compliance

**Option C: Create a custom gating program**
- Implement the sRFC 37 interface
- Custom logic for your specific needs

#### Step 3: Create MintConfig and Delegate Freeze Authority

```typescript
// Create MintConfig PDA
const [mintConfigPDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("MINT_CFG"), mint.toBuffer()],
  TOKEN_ACL_PROGRAM_ID
);

// Call create_config instruction
await tokenACLProgram.methods
  .createConfig({
    gatingProgram: yourGatingProgramId,
    enablePermissionlessThaw: true,
    enablePermissionlessFreeze: false, // Optional
  })
  .accounts({
    mint: mint,
    mintConfig: mintConfigPDA,
    authority: authority.publicKey,
    // ... other accounts
  })
  .signers([authority])
  .rpc();
```

This will:
- Create the MintConfig account
- Transfer freeze authority from you to the MintConfig PDA
- Configure permissionless operations

#### Step 4: Test the Workflow

```typescript
// User creates token account (frozen by default due to DAS)
const userTokenAccount = await createAccount(
  connection,
  payer,
  mint,
  user.publicKey
);

// User immediately thaws it themselves (if in allow list)
await tokenACLProgram.methods
  .thawPermissionless()
  .accounts({
    caller: user.publicKey,
    tokenAccount: userTokenAccount,
    mint: mint,
    mintConfig: mintConfigPDA,
    gatingProgram: gatingProgramId,
    // ... extra accounts from gating program
  })
  .signers([user])
  .rpc();

// Now user can transfer normally!
await transfer(
  connection,
  payer,
  userTokenAccount,
  destTokenAccount,
  user,
  1000
); // This is just a normal Token22 transfer!
```

### Managing Your Token

#### Add User to Allow List

```typescript
await gatingProgram.methods
  .addToAllowList()
  .accounts({
    authority: authority.publicKey,
    mint: mint,
    user: userToAdd,
    // ...
  })
  .signers([authority])
  .rpc();
```

#### Block a User (Sanctions)

```typescript
// Add to block list
await gatingProgram.methods
  .addToBlockList(BlockReason.Sanctions)
  .accounts({
    authority: authority.publicKey,
    mint: mint,
    user: userToBlock,
    // ...
  })
  .signers([authority])
  .rpc();

// Sweep user's token accounts (anyone can do this)
for (const tokenAccount of userTokenAccounts) {
  await tokenACLProgram.methods
    .freezePermissionless()
    .accounts({
      caller: anyoneCanCall.publicKey,
      tokenAccount: tokenAccount,
      mint: mint,
      // ...
    })
    .rpc();
}
```

#### Change Gating Program

```typescript
await tokenACLProgram.methods
  .setGatingProgram(newGatingProgramId)
  .accounts({
    mintConfig: mintConfigPDA,
    authority: authority.publicKey,
  })
  .signers([authority])
  .rpc();
```

#### Take Back Full Control (Exit Token ACL)

```typescript
await tokenACLProgram.methods
  .forfeitFreezeAuthority()
  .accounts({
    mint: mint,
    mintConfig: mintConfigPDA,
    newFreezeAuthority: authority.publicKey,
    authority: authority.publicKey,
  })
  .signers([authority])
  .rpc();
```

## For Gating Program Developers

### Implementing the Interface

Your gating program must implement this interface:

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

// Discriminators from sRFC 37
const CAN_THAW_PERMISSIONLESS: [u8; 8] = [8, 175, 169, 129, 137, 74, 61, 241];
const CAN_FREEZE_PERMISSIONLESS: [u8; 8] = [214, 141, 109, 75, 248, 1, 45, 29];

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let discriminator = &instruction_data[0..8];
    
    match discriminator {
        CAN_THAW_PERMISSIONLESS => {
            // Return Ok(()) if thaw should be allowed
            // Return Err(...) if thaw should be denied
            process_can_thaw_permissionless(program_id, accounts)
        }
        CAN_FREEZE_PERMISSIONLESS => {
            // Return Ok(()) if freeze should be allowed
            // Return Err(...) if freeze should be denied
            process_can_freeze_permissionless(program_id, accounts)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
```

### Account Structure

Your instructions will receive these accounts:

```rust
// 0. caller (signer) - who is calling permissionless operation
// 1. token_account - the token account to thaw/freeze
// 2. mint - the token mint
// 3. extra_account_metas - PDA containing your extra accounts
// 4+ extra accounts - as defined in your extra_account_metas PDA
```

### Extra Account Metas

You must create PDAs to tell Token ACL which extra accounts you need:

```rust
// For permissionless thaw
const THAW_EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"thaw-extra-account-metas";

// For permissionless freeze
const FREEZE_EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"freeze-extra-account-metas";

// Derive the PDA
let (thaw_extra_account_metas, _) = Pubkey::find_program_address(
    &[THAW_EXTRA_ACCOUNT_METAS_SEED, mint.as_ref()],
    program_id,
);
```

Populate these PDAs using the TLV account resolution library (similar to transfer-hook).

### Example: Simple Allow List

```rust
fn process_can_thaw_permissionless(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let token_account_owner = &accounts[4]; // From extra accounts
    let allow_list_pda = &accounts[5]; // From extra accounts
    
    // Verify allow list PDA
    let (expected_pda, _) = Pubkey::find_program_address(
        &[b"allow-list", mint.key.as_ref(), token_account_owner.key.as_ref()],
        program_id,
    );
    
    if *allow_list_pda.key != expected_pda {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check if user is allowed
    if allow_list_pda.data_is_empty() {
        msg!("User not in allow list");
        return Err(ProgramError::InvalidAccountData);
    }
    
    let record = AllowListRecord::try_from_slice(&allow_list_pda.data.borrow())?;
    
    if !record.allowed {
        return Err(ProgramError::InvalidAccountData);
    }
    
    msg!("User allowed - thaw authorized");
    Ok(())
}
```

### Example: Sanctions Block List

```rust
fn process_can_freeze_permissionless(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let token_account_owner = &accounts[4];
    let block_list_pda = &accounts[5];
    
    // Verify block list PDA
    let (expected_pda, _) = Pubkey::find_program_address(
        &[b"block-list", mint.key.as_ref(), token_account_owner.key.as_ref()],
        program_id,
    );
    
    if *block_list_pda.key != expected_pda {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check if user is blocked
    if block_list_pda.data_is_empty() {
        msg!("User not in block list - freeze denied");
        return Err(ProgramError::InvalidAccountData);
    }
    
    let record = BlockListRecord::try_from_slice(&block_list_pda.data.borrow())?;
    
    if !record.blocked {
        return Err(ProgramError::InvalidAccountData);
    }
    
    msg!("User blocked - freeze authorized");
    Ok(())
}
```

## For Protocol Developers

### Good News: You Don't Need to Do Anything!

Token ACL tokens work with your protocol **without any modifications**.

### Why?

Permissioning logic is moved OUT of the transfer path:

```
Old way (transfer-hooks):
User Transfer → Transfer Hook → Permissioning Logic → Complete Transfer
                 └─ Extra accounts, CU overhead, complexity

New way (Token ACL):
User Thaw (once) → Permissionless Thaw → Gating Logic
Then all transfers: User Transfer → Complete Transfer
                                    └─ Just normal Token22!
```

### Integration Checklist

- [ ] No changes needed to your smart contracts
- [ ] No changes needed to your UI
- [ ] Transfers are standard Token22 transfers
- [ ] No extra accounts in your instructions
- [ ] No CU overhead from permissioning

### User Workflow in Your Protocol

1. User creates token account (frozen by default)
2. User calls permissionless thaw (if in allow list)
3. User interacts with your protocol normally
4. All transfers are standard Token22 transfers

### Edge Cases to Handle

**Frozen Token Accounts**: If a user tries to use your protocol with a frozen token account:
- Transfer will fail (standard Token22 behavior)
- User needs to thaw their account first
- Your UI can detect frozen accounts and prompt user to thaw

```typescript
// Check if account is frozen
const accountInfo = await getAccount(connection, tokenAccount);
if (accountInfo.isFrozen) {
  // Prompt user to thaw
  showThawPrompt();
}
```

## For Wallet Developers

### Detection

Detect Token ACL tokens by checking if freeze authority is a MintConfig PDA:

```typescript
const mintInfo = await getMint(connection, mint);

// Derive expected MintConfig PDA
const [mintConfigPDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("MINT_CFG"), mint.toBuffer()],
  TOKEN_ACL_PROGRAM_ID
);

const isTokenACL = mintInfo.freezeAuthority?.equals(mintConfigPDA);
```

### User Experience

When user creates a new token account:

```typescript
// Account created frozen (due to Default Account State)
const tokenAccount = await createAccount(connection, payer, mint, user);

// Detect it's frozen
const accountInfo = await getAccount(connection, tokenAccount);
if (accountInfo.isFrozen) {
  // Prompt user
  showDialog({
    title: "Thaw Token Account?",
    message: "This token requires thawing before use. Thaw now?",
    action: () => sendPermissionlessThawTransaction()
  });
}
```

### Building Thaw Transaction

```typescript
async function buildPermissionlessThawTransaction(
  user: PublicKey,
  tokenAccount: PublicKey,
  mint: PublicKey
): Promise<Transaction> {
  // Get MintConfig
  const [mintConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from("MINT_CFG"), mint.toBuffer()],
    TOKEN_ACL_PROGRAM_ID
  );
  
  // Fetch MintConfig data to get gating program
  const mintConfigData = await connection.getAccountInfo(mintConfig);
  const gatingProgram = // ... parse from mintConfigData
  
  // Fetch extra account metas from gating program
  const [extraAccountMetas] = PublicKey.findProgramAddressSync(
    [Buffer.from("thaw-extra-account-metas"), mint.toBuffer()],
    gatingProgram
  );
  
  const extraAccounts = await fetchAndParseExtraAccounts(extraAccountMetas);
  
  // Build instruction
  return new Transaction().add({
    programId: TOKEN_ACL_PROGRAM_ID,
    keys: [
      { pubkey: user, isSigner: true, isWritable: false },
      { pubkey: tokenAccount, isSigner: false, isWritable: true },
      { pubkey: mint, isSigner: false, isWritable: false },
      { pubkey: mintConfig, isSigner: false, isWritable: false },
      { pubkey: gatingProgram, isSigner: false, isWritable: false },
      ...extraAccounts,
    ],
    data: Buffer.from([8, 175, 169, 129, 137, 74, 61, 241]), // Thaw discriminator
  });
}
```

### Best Practices

1. **Cache MintConfig data** to avoid repeated fetches
2. **Show clear error messages** if thaw fails (user not in allow list)
3. **One-time operation**: After thaw, all transfers are normal
4. **Handle frozen state**: Check account state before transfer
5. **Batch operations**: If user has multiple frozen accounts, batch thaw transactions

## Security Considerations

### For All Developers

1. **Trust the Gating Program**: The gating program controls who can thaw/freeze
2. **Issuer Control**: Issuer can always override via permissioned freeze/thaw
3. **Verify PDAs**: Always verify PDA derivations match expected seeds
4. **De-escalated Permissions**: Gating programs receive read-only accounts
5. **No Malicious Injection**: Token ACL prevents unauthorized instruction injection

### Red Flags

⚠️ **Never**:
- Trust a gating program you haven't audited
- Assume permissionless operations are always available (issuer can disable)
- Bypass frozen account checks
- Try to modify gating program state without proper authority

✅ **Always**:
- Verify freeze authority is the expected MintConfig PDA
- Check account frozen state before transfers
- Handle thaw failures gracefully
- Respect issuer's authority settings

## Migration Guide

### From Transfer-Hooks to Token ACL

If you have an existing permissioned token using transfer-hooks:

1. **Create new Token22 mint** with Default Account State
2. **Deploy or choose gating program** matching your permission logic
3. **Set up Token ACL** with MintConfig
4. **Migrate user balances** from old token to new token
5. **Sunset old token**

Benefits of migration:
- ✅ 90% reduction in transfer CU
- ✅ Universal protocol compatibility
- ✅ Better user experience
- ✅ No account dependency hell

### From Manual Thaw to Token ACL

If you're manually thawing user accounts:

1. **Enable permissionless thaw** in your MintConfig
2. **Deploy allow list gating program**
3. **Populate allow list** with your approved users
4. **Announce to users** they can now self-thaw
5. **Stop manual thawing process**

Benefits:
- ✅ Zero issuer overhead
- ✅ Instant user experience
- ✅ Maintains full control

## References

- [sRFC 37 Forum Post](https://forum.solana.com/t/srfc-37-efficient-block-allow-list-token-standard/4036)
- [Token ACL GitHub](https://github.com/solana-foundation/token-acl)
- [Token22 Documentation](https://spl.solana.com/token-2022)
- [TLV Account Resolution](https://github.com/solana-program/libraries/tree/main/tlv-account-resolution)

