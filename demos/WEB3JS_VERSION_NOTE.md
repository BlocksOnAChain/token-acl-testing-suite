# Web3.js Version Note

## Current Status

The helper functions in `src/lib/token-acl-helpers.ts` and `src/lib/spl-token-integration.ts` are written using **@solana/web3.js v2 API** (which is still in development).

### Why?

These helpers are **proposals for mainline integration** into future versions of:
- `@solana/web3.js` (v2)
- `@solana/spl-token`

They demonstrate the **ideal API design** for Token ACL integration.

### For Current Use (web3.js v1.x)

The demos in this directory use **web3.js v1.95.5** (current stable) for compatibility with `@solana/spl-token@0.4.14`.

The helper functions would need to be adapted to use v1.x APIs:

#### Key Differences

**v2 API (in helpers):**
```typescript
import { Address, address, IInstruction } from '@solana/web3.js';

const mintAddress: Address = address('...');
const instruction: IInstruction = { ... };
```

**v1.x API (current):**
```typescript
import { PublicKey, TransactionInstruction } from '@solana/web3.js';

const mintAddress = new PublicKey('...');
const instruction = new TransactionInstruction({ ... });
```

### Adaptation Guide

To use these helpers with web3.js v1.x:

1. **Replace `Address` with `PublicKey`**
   ```typescript
   // v2
   mint: Address
   
   // v1.x
   mint: PublicKey
   ```

2. **Replace `address()` with `new PublicKey()`**
   ```typescript
   // v2
   const addr = address('11111111111111111111111111111111');
   
   // v1.x
   const addr = new PublicKey('11111111111111111111111111111111');
   ```

3. **Replace `IInstruction` with `TransactionInstruction`**
   ```typescript
   // v2
   const ix: IInstruction = {
     programAddress: programId,
     accounts: [{ address: account, role: 0 }],
     data: new Uint8Array([...]),
   };
   
   // v1.x
   const ix = new TransactionInstruction({
     programId: programId,
     keys: [{ pubkey: account, isSigner: false, isWritable: true }],
     data: Buffer.from([...]),
   });
   ```

4. **Replace `getProgramDerivedAddress()` with `PublicKey.findProgramAddress()`**
   ```typescript
   // v2
   import { getProgramDerivedAddress } from '@solana/web3.js';
   const [pda, bump] = await getProgramDerivedAddress({
     programAddress: programId,
     seeds: [seed1, seed2],
   });
   
   // v1.x
   const [pda, bump] = await PublicKey.findProgramAddress(
     [seed1, seed2],
     programId
   );
   ```

### Example Adapted Helper (v1.x)

```typescript
import { PublicKey, TransactionInstruction } from '@solana/web3.js';

export async function findMintConfigPda(
  mint: PublicKey,
  programId: PublicKey = TOKEN_ACL_PROGRAM_ID
): Promise<[PublicKey, number]> {
  return await PublicKey.findProgramAddress(
    [Buffer.from('MINT_CFG'), mint.toBuffer()],
    programId
  );
}

export async function createPermissionlessThawInstruction(params: {
  caller: PublicKey;
  tokenAccount: PublicKey;
  mint: PublicKey;
  gatingProgram: PublicKey;
  extraAccounts?: Array<{ pubkey: PublicKey; isSigner: boolean; isWritable: boolean }>;
}): Promise<TransactionInstruction> {
  const [mintConfig] = await findMintConfigPda(params.mint);
  const [extraAccountMetas] = await findThawExtraAccountMetasPda(
    params.mint,
    params.gatingProgram
  );
  
  return new TransactionInstruction({
    programId: TOKEN_ACL_PROGRAM_ID,
    keys: [
      { pubkey: params.caller, isSigner: true, isWritable: false },
      { pubkey: params.tokenAccount, isSigner: false, isWritable: true },
      { pubkey: params.mint, isSigner: false, isWritable: false },
      { pubkey: mintConfig, isSigner: false, isWritable: false },
      { pubkey: params.gatingProgram, isSigner: false, isWritable: false },
      { pubkey: extraAccountMetas, isSigner: false, isWritable: false },
      ...(params.extraAccounts || []),
    ],
    data: Buffer.from(CAN_THAW_PERMISSIONLESS_DISCRIMINATOR),
  });
}
```

## Demos

The demo files (`sanctions-list-demo.ts`, `kyc-allowlist-demo.ts`, `geo-blocking-demo.ts`) are **conceptual demonstrations** showing the workflows and benefits. They use mock data and don't execute actual blockchain transactions.

To run the demos:
```bash
npm install
npm run demo:all
```

These demos will work with the current package.json (web3.js v1.95.5) since they're primarily illustrative and use mock implementations.

## Future

When web3.js v2 is officially released, the helpers in `src/lib/` can be used as-is or with minimal modifications.

## Recommendation

For **production use today**:
1. Adapt the helpers to web3.js v1.x (see examples above)
2. Test thoroughly with actual Token ACL deployment
3. Refer to the actual token-acl implementation: https://github.com/solana-foundation/token-acl

For **future-proofing**:
1. Keep both v1.x and v2 versions
2. Switch to v2 helpers when web3.js v2 is stable
3. Propose these helpers for mainline integration

## Questions?

The helper APIs are designed to be:
- **Intuitive**: Clear function names and parameters
- **Type-safe**: Full TypeScript support
- **Flexible**: Both low-level and high-level helpers
- **Production-ready**: Error handling and validation

They represent the **ideal developer experience** for Token ACL integration, regardless of web3.js version.

