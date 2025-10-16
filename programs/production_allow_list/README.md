# Production-Ready Allow List Gate Program

This is a **production-ready** implementation of an allow list gate program following the sRFC 37 specification.

## Features

✅ **sRFC 37 Compliant** - Implements the standardized interface  
✅ **Admin Controls** - Initialize, add/remove users, update authority  
✅ **Tiered Access** - Support for multiple access levels  
✅ **Expiry Handling** - Optional time-based access expiration  
✅ **Security** - Proper PDA validation and access control  
✅ **Tested** - Comprehensive unit tests  

## Instructions

### 1. Initialize

Sets up the program configuration for a mint.

```rust
Instruction: INITIALIZE (0)
Accounts:
  0. config_account (writable, PDA)
  1. mint (readonly)
  2. authority (signer)
  3. payer (signer, writable)
  4. system_program
```

### 2. Add to Allow List

Adds a user to the allow list with specified access level.

```rust
Instruction: ADD_TO_ALLOW_LIST (1)
Accounts:
  0. config_account (readonly)
  1. allow_list_account (writable, PDA)
  2. mint (readonly)
  3. user (readonly)
  4. authority (signer)
  5. payer (signer, writable)
  6. system_program

Data:
  - access_level: u8 (0=None, 1=Basic, 2=Enhanced, 3=Institutional)
  - expiry_timestamp: Option<i64>
```

### 3. Remove from Allow List

Removes a user from the allow list.

```rust
Instruction: REMOVE_FROM_ALLOW_LIST (2)
Accounts:
  0. config_account (readonly)
  1. allow_list_account (writable)
  2. authority (signer)
```

### 4. Update Authority

Transfers admin authority to a new pubkey.

```rust
Instruction: UPDATE_AUTHORITY (3)
Accounts:
  0. config_account (writable)
  1. current_authority (signer)
  2. new_authority (readonly)
```

### 5. Can Thaw Permissionless (sRFC 37 Interface)

Interface method called by Token ACL to validate permissionless thaw.

```rust
Discriminator: [8, 175, 169, 129, 137, 74, 61, 241]
Accounts (as per sRFC 37):
  0. caller (signer)
  1. token_account (readonly)
  2. mint (readonly)
  3. extra_account_metas (readonly)
  4. token_account_owner (readonly) - from extra accounts
  5. allow_list_pda (readonly) - from extra accounts
```

## Access Levels

```rust
pub enum AccessLevel {
    None = 0,          // No access
    Basic = 1,         // Basic verified user
    Enhanced = 2,      // Enhanced due diligence
    Institutional = 3, // Institutional grade
}
```

## PDAs

### Config PDA
```
Seeds: ["config", mint_pubkey]
Program: This allow list program
```

### Allow List PDA
```
Seeds: ["allow-list", mint_pubkey, user_pubkey]
Program: This allow list program
```

## Deployment

### Build

```bash
cargo build-bpf
```

### Deploy to Devnet

```bash
solana program deploy target/deploy/gate_program_allow_list_production.so \
  --url devnet \
  --keypair ~/.config/solana/id.json
```

### Initialize for a Mint

```typescript
import { PublicKey, Transaction } from '@solana/web3.js';

const mint = new PublicKey('YourMintAddress');
const [configPDA] = await PublicKey.findProgramAddress(
  [Buffer.from('config'), mint.toBuffer()],
  ALLOW_LIST_PROGRAM_ID
);

// Create initialize instruction
const initIx = new TransactionInstruction({
  programId: ALLOW_LIST_PROGRAM_ID,
  keys: [
    { pubkey: configPDA, isSigner: false, isWritable: true },
    { pubkey: mint, isSigner: false, isWritable: false },
    { pubkey: authority, isSigner: true, isWritable: false },
    { pubkey: payer, isSigner: true, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ],
  data: Buffer.from([0]), // INITIALIZE discriminator
});
```

## Use Cases

- KYC verified users
- Accredited investors
- Tiered membership systems
- Time-limited access
- Regulatory compliance

## Security Considerations

### Before Mainnet Deployment

- [ ] **Security Audit** - Have the code professionally audited
- [ ] **Upgrade Authority** - Consider adding upgrade mechanism
- [ ] **Testing** - Extensive testing on devnet
- [ ] **Monitoring** - Set up monitoring for admin operations
- [ ] **Access Control** - Verify authority controls are robust
- [ ] **Expiry Logic** - Test expiry handling thoroughly

### Best Practices

✅ Always verify PDA derivations  
✅ Validate authority before state changes  
✅ Use Clock sysvar for timestamps  
✅ Handle expiry edge cases  
✅ Log important events  

## Integration with Token ACL

1. Deploy this gate program
2. Initialize for your mint
3. Add users to allow list
4. Create Token ACL MintConfig pointing to this program
5. Enable permissionless thaw
6. Users can now thaw their own accounts!

## License

MIT

