# Token ACL Architecture Guide

This guide explains the Token ACL system architecture, how it works, and how to integrate it into your applications.

## Overview

Token ACL (Access Control List) is a novel mechanism for permissioned tokens on Solana that uses Token22's Default Account State extension and delegated freeze authority to eliminate UX friction while maintaining protocol composability.

## System Architecture

### Core Components

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Token Mint    │    │   Token ACL      │    │  Gating Program │
│   (Token22)     │◄──►│   (FAMP)         │◄──►│  (Custom Logic) │
│                 │    │                  │    │                 │
│ • DAS: Frozen   │    │ • MintConfig     │    │ • Allow List    │
│ • Freeze Auth   │    │ • Freeze Auth    │    │ • Block List    │
│ • Standard Ops  │    │ • Permissionless │    │ • Hybrid        │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Key Innovation: Permission De-escalation

Token ACL acts as a **controlled proxy** that:
1. Receives user requests with full permissions
2. De-escalates permissions before calling gating programs
3. Executes actions based on gating program decisions
4. Prevents malicious gating programs from harming users

## Token ACL Interface (sRFC 37)

### Core Instructions

#### 1. Create Config
```rust
pub struct CreateConfig {
    pub gating_program: Pubkey,
    pub enable_permissionless_thaw: bool,
    pub enable_permissionless_freeze: bool,
}
```

#### 2. Thaw Permissionless
```rust
// Discriminator: [8, 175, 169, 129, 137, 74, 61, 241]
// Accounts: caller, token_account, mint, mint_config, gating_program, extra_accounts...
```

#### 3. Freeze Permissionless
```rust
// Discriminator: [214, 141, 109, 75, 248, 1, 45, 29]
// Accounts: caller, token_account, mint, mint_config, gating_program, extra_accounts...
```

### MintConfig Structure

```rust
pub struct MintConfig {
    pub discriminator: u8,                    // 0x01
    pub mint: Pubkey,                        // Token mint address
    pub authority: Pubkey,                   // Issuer authority
    pub gating_program: Pubkey,              // Gating program ID
    pub enable_permissionless_thaw: bool,    // Allow permissionless thaw
    pub enable_permissionless_freeze: bool,  // Allow permissionless freeze
}
```

## Gating Program Interface

### Required Interface

Gating programs must implement the sRFC 37 interface:

#### Thaw Permissionless
```rust
// Discriminator: [8, 175, 169, 129, 137, 74, 61, 241]
// Returns: Ok(()) if allowed, Err(...) if denied
fn can_thaw_permissionless(accounts: &[AccountInfo]) -> ProgramResult
```

#### Freeze Permissionless (Optional)
```rust
// Discriminator: [214, 141, 109, 75, 248, 1, 45, 29]
// Returns: Ok(()) if allowed, Err(...) if denied
fn can_freeze_permissionless(accounts: &[AccountInfo]) -> ProgramResult
```

### Account Permissions

**Critical Security Feature**: Gating programs receive **de-escalated permissions**:

```rust
// What Token ACL receives from user:
AccountMeta::new(user_pubkey, true)      // Signer
AccountMeta::new(token_account, false)   // Writable

// What Token ACL passes to gating program:
AccountMeta::new_readonly(user_pubkey, false)    // NOT signer!
AccountMeta::new_readonly(token_account, false)  // NOT writable!
```

This prevents gating programs from:
- Modifying user balances
- Making unauthorized transfers
- Injecting malicious instructions

## Integration Patterns

### For Token Issuers

#### 1. Create Permissioned Token

```typescript
// Step 1: Create Token22 mint with DAS
const mint = await createMint(
  connection,
  payer,
  authority,
  null,
  6,
  TOKEN_2022_PROGRAM_ID,
  {
    defaultAccountState: { frozen: true }  // Key: frozen by default
  }
);

// Step 2: Create MintConfig
const [mintConfigPDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("MINT_CFG"), mint.toBuffer()],
  TOKEN_ACL_PROGRAM_ID
);

await tokenACLProgram.methods
  .createConfig({
    gatingProgram: allowListProgramId,
    enablePermissionlessThaw: true,
    enablePermissionlessFreeze: false,
  })
  .accounts({
    mint,
    mintConfig: mintConfigPDA,
    authority: authority.publicKey,
  })
  .signers([authority])
  .rpc();
```

#### 2. User Onboarding Flow

```typescript
// User creates token account (automatically frozen)
const userTokenAccount = await createAccount(
  connection,
  payer,
  mint,
  user.publicKey
);

// User immediately thaws (if in allow list)
await tokenACLProgram.methods
  .thawPermissionless()
  .accounts({
    caller: user.publicKey,
    tokenAccount: userTokenAccount,
    mint,
    mintConfig: mintConfigPDA,
    gatingProgram: allowListProgramId,
    // Extra accounts from gating program
    tokenAccountOwner: user.publicKey,
    allowListPDA: allowListPDA,
  })
  .signers([user])
  .rpc();

// Now user can transfer normally!
```

### For Developers

#### 1. Detect Token ACL Tokens

```typescript
async function isTokenAclMint(connection: Connection, mint: PublicKey): Promise<boolean> {
  const [mintConfigPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("MINT_CFG"), mint.toBuffer()],
    TOKEN_ACL_PROGRAM_ID
  );
  
  try {
    const accountInfo = await connection.getAccountInfo(mintConfigPDA);
    return accountInfo !== null;
  } catch {
    return false;
  }
}
```

#### 2. Smart Thaw Helper

```typescript
async function smartThaw(
  connection: Connection,
  tokenAccount: PublicKey,
  user: Keypair
): Promise<TransactionInstruction[]> {
  const accountInfo = await connection.getAccountInfo(tokenAccount);
  const mint = new PublicKey(accountInfo.data.slice(0, 32));
  
  if (!(await isTokenAclMint(connection, mint))) {
    return []; // Not a Token ACL token, no thaw needed
  }
  
  // Check if account is frozen
  const tokenAccountInfo = await connection.getTokenAccountBalance(tokenAccount);
  if (tokenAccountInfo.value.state !== "frozen") {
    return []; // Already thawed
  }
  
  // Create permissionless thaw instruction
  return [await createThawPermissionlessInstruction({
    caller: user.publicKey,
    tokenAccount,
    mint,
    // ... other accounts
  })];
}
```

#### 3. Wallet Integration

```typescript
// In wallet: detect Token ACL tokens and prompt for thaw
async function handleTokenAccountCreation(
  connection: Connection,
  mint: PublicKey,
  user: PublicKey
) {
  if (await isTokenAclMint(connection, mint)) {
    // Show one-time thaw prompt
    const shouldThaw = await showThawPrompt(mint);
    if (shouldThaw) {
      await executeThaw(connection, mint, user);
    }
  }
}
```

## Gating Program Examples

### Allow List Implementation

```rust
pub fn process_can_thaw_permissionless(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    let _caller = next_account_info(account_info_iter)?;
    let _token_account = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let _extra_account_metas = next_account_info(account_info_iter)?;
    let token_account_owner = next_account_info(account_info_iter)?;
    let allow_list_pda = next_account_info(account_info_iter)?;
    
    // Verify allow list PDA
    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[ALLOW_LIST_SEED, mint.key.as_ref(), token_account_owner.key.as_ref()],
        program_id,
    );
    
    if *allow_list_pda.key != expected_pda {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check if user is in allow list
    if allow_list_pda.data_is_empty() {
        return Err(ProgramError::InvalidAccountData);
    }
    
    let record = AllowListRecord::try_from_slice(&allow_list_pda.data.borrow())?;
    
    if !record.allowed {
        return Err(ProgramError::InvalidAccountData);
    }
    
    Ok(())
}
```

### Block List Implementation

```rust
pub fn process_can_freeze_permissionless(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Similar structure but checks if user is in block list
    // Returns Ok(()) if user should be frozen
    // Returns Err(...) if user should not be frozen
}
```

## Best Practices

### For Token Issuers

1. **Choose the Right Gating Program**
   - Allow list: For KYC/whitelist scenarios
   - Block list: For sanctions/compliance
   - Hybrid: For complex requirements

2. **Plan Your Authority Management**
   - Use multisig for production
   - Plan key rotation strategy
   - Consider forfeit authority option

3. **Test Thoroughly**
   - Test with real users
   - Verify gating program logic
   - Test edge cases

### For Developers

1. **Handle Token ACL Detection**
   - Always check if token uses Token ACL
   - Provide appropriate UI/UX
   - Handle thaw failures gracefully

2. **Optimize for UX**
   - Batch thaw operations when possible
   - Cache gating program results
   - Provide clear error messages

3. **Security Considerations**
   - Never trust gating program results blindly
   - Validate all account derivations
   - Handle permission de-escalation correctly

### For Gating Program Developers

1. **Follow the Interface**
   - Use correct discriminators
   - Implement proper PDA derivation
   - Handle all required accounts

2. **Security First**
   - Remember: you receive read-only accounts
   - Validate all inputs
   - Don't assume account states

3. **Performance**
   - Minimize account reads
   - Use efficient data structures
   - Consider gas costs

## Migration Guide

### From Manual DAS

```typescript
// Old way: Manual thaw for every user
await freezeAccount(connection, payer, tokenAccount, mint, authority);
// User waits for issuer to thaw...

// New way: Permissionless thaw
await tokenACLProgram.methods.thawPermissionless()
  .accounts({ /* ... */ })
  .signers([user])
  .rpc();
// User thaws immediately!
```

### From Transfer Hooks

```typescript
// Old way: Every transfer requires gating program
const transferInstruction = createTransferInstruction(
  source,
  destination,
  amount,
  // 5-10 extra accounts for gating program
  gatingProgramAccounts...
);

// New way: One-time thaw, then normal transfers
await thawPermissionless(); // One time
const transferInstruction = createTransferInstruction(
  source,
  destination,
  amount
  // Just 3 accounts!
);
```

## Troubleshooting

### Common Issues

1. **"Account not found" errors**
   - Check MintConfig PDA derivation
   - Verify gating program is deployed
   - Ensure all required accounts are provided

2. **"Invalid discriminator" errors**
   - Use correct discriminators from sRFC 37
   - Check instruction data format
   - Verify gating program implementation

3. **"Permission denied" errors**
   - Check if user is in allow/block list
   - Verify gating program logic
   - Ensure proper account permissions

### Debug Tips

1. **Enable logging**
   ```rust
   msg!("Debug: User {} attempting thaw", user.key);
   ```

2. **Validate PDAs**
   ```rust
   let (expected_pda, bump) = Pubkey::find_program_address(seeds, program_id);
   assert_eq!(*pda.key, expected_pda);
   ```

3. **Check account states**
   ```rust
   if account.data_is_empty() {
       return Err(ProgramError::InvalidAccountData);
   }
   ```

## Resources

- **sRFC 37 Specification**: https://forum.solana.com/t/srfc-37-efficient-block-allow-list-token-standard/4036
- **Token ACL Implementation**: https://github.com/solana-foundation/token-acl
- **Token22 Documentation**: https://spl.solana.com/token-2022
- **Example Gating Programs**: `examples/` directory
- **Production Gating Programs**: `programs/` directory

---

**Token ACL enables permissioned tokens without sacrificing composability or security.** 🚀
