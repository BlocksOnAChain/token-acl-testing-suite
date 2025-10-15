# Token ACL - Quick Reference Card

## One-Minute Overview

**Token ACL** = Permissioned tokens WITHOUT the friction

### The Problem

**Before Token ACL**:
```
User creates token account → FROZEN
↓
User contacts support
↓
Support creates ticket
↓
Issuer manually thaws (hours/days later)
↓
User can FINALLY trade
```
⏱️ **Time**: Hours to DAYS
😫 **User Experience**: TERRIBLE

**With transfer-hooks**:
```
User transfers token
↓
Extra 8-15 accounts + 50,000 CU
↓
Many protocols just blacklist these tokens
```
💔 **Protocol Support**: Only ~15%

### The Solution

**With Token ACL**:
```
User creates token account → FROZEN
↓
User immediately thaws THEMSELVES
↓
User can trade
```
⏱️ **Time**: SECONDS
😊 **User Experience**: EXCELLENT
🔄 **Protocol Support**: 100%

## Key Concepts

### 1. Default Account State (DAS)
All new token accounts start **frozen**

### 2. Permissionless Thaw
Users can **thaw their own accounts** if they pass gating checks

### 3. Gating Program
Smart contract that decides who can thaw/freeze
- Allow List: User in list? → Can thaw
- Block List: User in list? → Can freeze
- Geo-Blocking: User in allowed region? → Can thaw

### 4. Token ACL (FAMP)
Manages freeze authority and acts as controlled proxy

## Quick Commands

### Run Demos
```bash
cd demos
npm install

# All demos
npm run demo:all

# Individual demos
npm run demo:sanctions    # Sanctions list
npm run demo:kyc          # KYC allowlist
npm run demo:geo          # Geo-blocking
```

### Run Tests
```bash
cd tests/test-client
cargo test --all
```

## Code Snippets

### Check if Mint Uses Token ACL
```typescript
import { isTokenAclMint } from './lib/token-acl-helpers';

const isACL = await isTokenAclMint(rpc, mintAddress);
// → true/false
```

### Permissionless Thaw
```typescript
import { buildPermissionlessThawTransaction } from './lib/token-acl-helpers';

const thawIx = await buildPermissionlessThawTransaction(rpc, {
  caller: userAddress,
  tokenAccount: userTokenAccount,
  mint: mintAddress,
});
// User can send this transaction themselves!
```

### Smart Thaw (Auto-detect)
```typescript
import { smartThaw } from './lib/spl-token-integration';

const thawIx = await smartThaw(rpc, tokenAccount, userAddress);
// Automatically uses permissionless if available
```

### Get Account with ACL Info
```typescript
import { getAccountWithACL } from './lib/spl-token-integration';

const account = await getAccountWithACL(rpc, tokenAccount);

if (account.isFrozen && account.canPermissionlessThaw) {
  // Show "Thaw Account" button in UI
}
```

### Batch Operations
```typescript
import { batchThaw } from './lib/spl-token-integration';

// Thaw multiple accounts at once
const instructions = await batchThaw(rpc, tokenAccounts, userAddress);
```

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    TOKEN MINT                           │
│  (Token22 with Default Account State)                  │
│  Freeze Authority: MintConfig PDA                      │
└─────────────────────────────────────────────────────────┘
                      │
                      │ freeze authority delegated
                      ↓
┌─────────────────────────────────────────────────────────┐
│               TOKEN ACL (FAMP)                          │
│  • Holds freeze authority                              │
│  • Acts as controlled proxy                            │
│  • De-escalates permissions                            │
└─────────────────────────────────────────────────────────┘
                      │
                      │ calls with de-escalated permissions
                      ↓
┌─────────────────────────────────────────────────────────┐
│              GATING PROGRAM                             │
│  • Allow List: Check if user allowed                   │
│  • Block List: Check if user blocked                   │
│  • Custom: Any logic you want                          │
│  • Returns: Success or Failure                         │
└─────────────────────────────────────────────────────────┘
```

## PDAs (Program Derived Addresses)

### MintConfig PDA
```typescript
Seeds: ["MINT_CFG", mint_address]
Program: Token ACL Program

Contains:
- mint: Pubkey
- authority: Pubkey
- gating_program: Pubkey
- enable_permissionless_thaw: bool
- enable_permissionless_freeze: bool
```

### Extra Account Metas PDAs
```typescript
// For thaw
Seeds: ["thaw-extra-account-metas", mint_address]
Program: Gating Program

// For freeze
Seeds: ["freeze-extra-account-metas", mint_address]
Program: Gating Program
```

### Allow/Block List PDAs
```typescript
// Allow list
Seeds: ["allow-list", mint_address, user_address]
Program: Gating Program

// Block list
Seeds: ["block-list", mint_address, user_address]
Program: Gating Program
```

## Discriminators

```rust
// From sRFC 37 specification
CAN_THAW_PERMISSIONLESS:   [8, 175, 169, 129, 137, 74, 61, 241]
CAN_FREEZE_PERMISSIONLESS: [214, 141, 109, 75, 248, 1, 45, 29]

// Hash inputs
"efficient-allow-block-list-standard:can-thaw-permissionless"
"efficient-allow-block-list-standard:can-freeze-permissionless"
```

## Instructions

| Instruction | Who Can Call | Purpose |
|-------------|--------------|---------|
| `create_config` | Freeze Authority | Create MintConfig, delegate freeze authority |
| `set_authority` | Current Authority | Change MintConfig authority |
| `set_gating_program` | Authority | Change gating program |
| `forfeit_freeze_authority` | Authority | Return freeze authority to issuer |
| `freeze` (permissioned) | Authority | Manually freeze account |
| `thaw` (permissioned) | Authority | Manually thaw account |
| `freeze_permissionless` | **Anyone** | Freeze if gating program approves |
| `thaw_permissionless` | **Anyone** | Thaw if gating program approves |

## Key Metrics

| Metric | Transfer-Hook | Token ACL |
|--------|--------------|-----------|
| Transfer CU | 50,000 | **5,000** (90% ↓) |
| Transfer accounts | 12+ | **3** (75% ↓) |
| Protocol support | ~15% | **~100%** |
| User wait time | Seconds | **Seconds** |
| Manual thaw wait | Hours/Days | **None** |
| Issuer overhead | Medium | **Zero** |

## Use Cases

### 🚫 Sanctions List (Block List)
```typescript
// Compliance officer adds to sanctions
await addToBlockList(user, 'OFAC');

// Anyone freezes all user's accounts
await batchFreeze(rpc, userAccounts, anyExecutor);
```

### ✅ KYC Allow List
```typescript
// KYC provider adds to allow list
await addToAllowList(user, kycLevel);

// User immediately thaws themselves
await smartThaw(rpc, userAccount, user);
```

### 🌍 Geo-Blocking
```typescript
// Oracle verifies location
await verifyLocation(user, country);

// If allowed: user can thaw
// If blocked: thaw denied
await smartThaw(rpc, userAccount, user);
```

## Common Patterns

### User Onboarding Flow
```typescript
// 1. User completes KYC off-chain
// 2. Add to allow list
await addToAllowList(user);

// 3. User creates account (frozen)
const account = await createAccount(user, mint);

// 4. User immediately thaws
const thawIx = await smartThaw(rpc, account, user);
await sendTransaction([thawIx]);

// 5. User can now trade!
```

### Sanctions Enforcement
```typescript
// 1. Add to sanctions list
await addToBlockList(sanctionedUser, 'OFAC');

// 2. Automated system freezes all accounts
const accounts = await findAllAccounts(sanctionedUser, mint);
const freezeIxs = await batchFreeze(rpc, accounts, bot);
await sendTransactions(freezeIxs);

// Done! No manual intervention needed.
```

### Wallet Integration
```typescript
// Detect if account is frozen and can be thawed
const account = await getAccountWithACL(rpc, tokenAccount);

if (account.isFrozen) {
  if (account.canPermissionlessThaw) {
    // Show "Thaw Account" button
    showThawButton(() => {
      const thawIx = await smartThaw(rpc, tokenAccount, user);
      await sendTransaction([thawIx]);
    });
  } else {
    // Show "Account frozen, contact issuer"
    showFrozenMessage();
  }
}
```

## Troubleshooting

### "Permissionless thaw failed"
→ User not in allow list, or gating program check failed

### "Account not frozen"
→ Already thawed, no action needed

### "Mint is not using Token ACL"
→ Freeze authority is not MintConfig PDA

### "Too many accounts"
→ You might be thinking of transfer-hooks. Token ACL transfers only need 3 accounts!

## Links

- **Spec**: https://forum.solana.com/t/srfc-37-efficient-block-allow-list-token-standard/4036
- **Impl**: https://github.com/solana-foundation/token-acl
- **Docs**: See INDEX.md, FINAL_SUMMARY.md, IMPLEMENTATION_GUIDE.md

## TL;DR

**Token ACL** = Better UX + Better DX + Better Performance

Use it for ANY permissioned token! 🚀

