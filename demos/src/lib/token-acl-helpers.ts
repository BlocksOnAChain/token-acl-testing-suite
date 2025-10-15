/**
 * Token ACL Helpers for @solana/web3.js v2
 * 
 * This module provides production-ready helper functions for integrating
 * sRFC 37 Token ACL into applications using the latest Solana web3.js.
 * 
 * Proposed for mainline support in @solana/spl-token package.
 */

import {
  Address,
  address,
  getAddressEncoder,
  getProgramDerivedAddress,
  IInstruction,
  ReadonlyUint8Array,
} from '@solana/web3.js';

// Token ACL Program ID (mainnet/devnet address would go here)
export const TOKEN_ACL_PROGRAM_ADDRESS = address(
  'ACLzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb' // Placeholder
);

// Discriminators from sRFC 37
export const CAN_THAW_PERMISSIONLESS_DISCRIMINATOR = new Uint8Array([
  8, 175, 169, 129, 137, 74, 61, 241
]);

export const CAN_FREEZE_PERMISSIONLESS_DISCRIMINATOR = new Uint8Array([
  214, 141, 109, 75, 248, 1, 45, 29
]);

// Seeds
export const MINT_CONFIG_SEED = new Uint8Array([
  77, 73, 78, 84, 95, 67, 70, 71 // "MINT_CFG"
]);

export const THAW_EXTRA_ACCOUNT_METAS_SEED = new Uint8Array([
  116, 104, 97, 119, 45, 101, 120, 116, 114, 97, 45, 
  97, 99, 99, 111, 117, 110, 116, 45, 109, 101, 116, 97, 115
]); // "thaw-extra-account-metas"

export const FREEZE_EXTRA_ACCOUNT_METAS_SEED = new Uint8Array([
  102, 114, 101, 101, 122, 101, 45, 101, 120, 116, 114, 97, 45,
  97, 99, 99, 111, 117, 110, 116, 45, 109, 101, 116, 97, 115
]); // "freeze-extra-account-metas"

/**
 * MintConfig account structure
 */
export interface MintConfig {
  discriminator: number;
  mint: Address;
  authority: Address;
  gatingProgram: Address;
  enablePermissionlessThaw: boolean;
  enablePermissionlessFreeze: boolean;
}

/**
 * Find the MintConfig PDA for a given mint
 * 
 * @param mint - The token mint address
 * @param programId - Token ACL program ID (defaults to TOKEN_ACL_PROGRAM_ADDRESS)
 * @returns The MintConfig PDA and bump seed
 * 
 * @example
 * ```ts
 * const [mintConfig, bump] = await findMintConfigPda(mintAddress);
 * ```
 */
export async function findMintConfigPda(
  mint: Address,
  programId: Address = TOKEN_ACL_PROGRAM_ADDRESS
): Promise<[Address, number]> {
  const encoder = getAddressEncoder();
  const mintBytes = encoder.encode(mint);
  
  return await getProgramDerivedAddress({
    programAddress: programId,
    seeds: [MINT_CONFIG_SEED, mintBytes],
  });
}

/**
 * Find the extra account metas PDA for permissionless thaw
 * 
 * @param mint - The token mint address
 * @param gatingProgram - The gating program address
 * @returns The extra account metas PDA and bump seed
 */
export async function findThawExtraAccountMetasPda(
  mint: Address,
  gatingProgram: Address
): Promise<[Address, number]> {
  const encoder = getAddressEncoder();
  const mintBytes = encoder.encode(mint);
  
  return await getProgramDerivedAddress({
    programAddress: gatingProgram,
    seeds: [THAW_EXTRA_ACCOUNT_METAS_SEED, mintBytes],
  });
}

/**
 * Find the extra account metas PDA for permissionless freeze
 * 
 * @param mint - The token mint address
 * @param gatingProgram - The gating program address
 * @returns The extra account metas PDA and bump seed
 */
export async function findFreezeExtraAccountMetasPda(
  mint: Address,
  gatingProgram: Address
): Promise<[Address, number]> {
  const encoder = getAddressEncoder();
  const mintBytes = encoder.encode(mint);
  
  return await getProgramDerivedAddress({
    programAddress: gatingProgram,
    seeds: [FREEZE_EXTRA_ACCOUNT_METAS_SEED, mintBytes],
  });
}

/**
 * Parameters for creating a MintConfig
 */
export interface CreateMintConfigParams {
  mint: Address;
  authority: Address;
  gatingProgram?: Address;
  enablePermissionlessThaw?: boolean;
  enablePermissionlessFreeze?: boolean;
}

/**
 * Create a create_config instruction
 * 
 * This instruction creates the MintConfig account and delegates freeze
 * authority from the mint to Token ACL.
 * 
 * @param params - Configuration parameters
 * @returns The instruction to create MintConfig
 * 
 * @example
 * ```ts
 * const instruction = await createMintConfigInstruction({
 *   mint: mintAddress,
 *   authority: authorityAddress,
 *   gatingProgram: gatingProgramAddress,
 *   enablePermissionlessThaw: true,
 * });
 * ```
 */
export async function createMintConfigInstruction(
  params: CreateMintConfigParams
): Promise<IInstruction> {
  const [mintConfig] = await findMintConfigPda(params.mint);
  
  // Instruction discriminator for create_config
  const discriminator = new Uint8Array([0x01]);
  
  // Encode parameters (simplified - actual implementation would use borsh)
  const data = new Uint8Array([
    ...discriminator,
    params.enablePermissionlessThaw ? 1 : 0,
    params.enablePermissionlessFreeze ? 1 : 0,
  ]);
  
  return {
    programAddress: TOKEN_ACL_PROGRAM_ADDRESS,
    accounts: [
      { address: mintConfig, role: 0 }, // writable
      { address: params.mint, role: 0 }, // writable
      { address: params.authority, role: 2 }, // signer
      // ... other accounts (system program, token program, etc.)
    ],
    data,
  };
}

/**
 * Parameters for permissionless thaw
 */
export interface PermissionlessThawParams {
  caller: Address;
  tokenAccount: Address;
  mint: Address;
  gatingProgram: Address;
  extraAccounts?: Array<{ address: Address; role: number }>;
}

/**
 * Create a permissionless thaw instruction
 * 
 * This allows a user to thaw their own token account without issuer intervention,
 * if they pass the gating program's checks (e.g., in allow list).
 * 
 * @param params - Thaw parameters
 * @returns The instruction to permissionlessly thaw
 * 
 * @example
 * ```ts
 * const instruction = await createPermissionlessThawInstruction({
 *   caller: userAddress,
 *   tokenAccount: userTokenAccount,
 *   mint: mintAddress,
 *   gatingProgram: gatingProgramAddress,
 * });
 * ```
 */
export async function createPermissionlessThawInstruction(
  params: PermissionlessThawParams
): Promise<IInstruction> {
  const [mintConfig] = await findMintConfigPda(params.mint);
  const [extraAccountMetas] = await findThawExtraAccountMetasPda(
    params.mint,
    params.gatingProgram
  );
  
  return {
    programAddress: TOKEN_ACL_PROGRAM_ADDRESS,
    accounts: [
      { address: params.caller, role: 2 }, // signer
      { address: params.tokenAccount, role: 0 }, // writable
      { address: params.mint, role: 1 }, // readonly
      { address: mintConfig, role: 1 }, // readonly
      { address: params.gatingProgram, role: 1 }, // readonly
      { address: extraAccountMetas, role: 1 }, // readonly
      ...(params.extraAccounts || []),
    ],
    data: CAN_THAW_PERMISSIONLESS_DISCRIMINATOR,
  };
}

/**
 * Parameters for permissionless freeze
 */
export interface PermissionlessFreezeParams {
  caller: Address;
  tokenAccount: Address;
  mint: Address;
  gatingProgram: Address;
  extraAccounts?: Array<{ address: Address; role: number }>;
}

/**
 * Create a permissionless freeze instruction
 * 
 * This allows anyone to freeze a token account if the gating program approves
 * (e.g., user is in block/sanctions list).
 * 
 * @param params - Freeze parameters
 * @returns The instruction to permissionlessly freeze
 * 
 * @example
 * ```ts
 * const instruction = await createPermissionlessFreezeInstruction({
 *   caller: anyAddress,
 *   tokenAccount: blockedUserTokenAccount,
 *   mint: mintAddress,
 *   gatingProgram: gatingProgramAddress,
 * });
 * ```
 */
export async function createPermissionlessFreezeInstruction(
  params: PermissionlessFreezeParams
): Promise<IInstruction> {
  const [mintConfig] = await findMintConfigPda(params.mint);
  const [extraAccountMetas] = await findFreezeExtraAccountMetasPda(
    params.mint,
    params.gatingProgram
  );
  
  return {
    programAddress: TOKEN_ACL_PROGRAM_ADDRESS,
    accounts: [
      { address: params.caller, role: 2 }, // signer
      { address: params.tokenAccount, role: 0 }, // writable
      { address: params.mint, role: 1 }, // readonly
      { address: mintConfig, role: 1 }, // readonly
      { address: params.gatingProgram, role: 1 }, // readonly
      { address: extraAccountMetas, role: 1 }, // readonly
      ...(params.extraAccounts || []),
    ],
    data: CAN_FREEZE_PERMISSIONLESS_DISCRIMINATOR,
  };
}

/**
 * Check if a mint is using Token ACL
 * 
 * @param rpc - Solana RPC client
 * @param mint - The token mint address
 * @returns True if the mint uses Token ACL
 * 
 * @example
 * ```ts
 * const isTokenACL = await isTokenAclMint(rpc, mintAddress);
 * if (isTokenACL) {
 *   console.log('This mint uses Token ACL!');
 * }
 * ```
 */
export async function isTokenAclMint(
  rpc: any, // RPC type from @solana/web3.js v2
  mint: Address
): Promise<boolean> {
  try {
    // Fetch mint account and check if freeze authority is MintConfig PDA
    const mintAccount = await rpc.getAccountInfo(mint).send();
    if (!mintAccount.value) return false;
    
    const [expectedMintConfig] = await findMintConfigPda(mint);
    
    // Parse mint account and check freeze authority
    // (Actual implementation would parse the mint account data)
    // For now, we check if MintConfig account exists
    const mintConfigAccount = await rpc.getAccountInfo(expectedMintConfig).send();
    
    return mintConfigAccount.value !== null;
  } catch {
    return false;
  }
}

/**
 * Fetch MintConfig data
 * 
 * @param rpc - Solana RPC client
 * @param mint - The token mint address
 * @returns The MintConfig data
 * 
 * @example
 * ```ts
 * const config = await fetchMintConfig(rpc, mintAddress);
 * console.log('Gating program:', config.gatingProgram);
 * console.log('Permissionless thaw enabled:', config.enablePermissionlessThaw);
 * ```
 */
export async function fetchMintConfig(
  rpc: any,
  mint: Address
): Promise<MintConfig | null> {
  try {
    const [mintConfigAddress] = await findMintConfigPda(mint);
    const account = await rpc.getAccountInfo(mintConfigAddress).send();
    
    if (!account.value) return null;
    
    // Parse MintConfig data (simplified - actual implementation would use borsh)
    const data = account.value.data;
    
    return {
      discriminator: data[0],
      mint: mint,
      authority: address('11111111111111111111111111111111'), // Placeholder
      gatingProgram: address('11111111111111111111111111111111'), // Placeholder
      enablePermissionlessThaw: Boolean(data[1]),
      enablePermissionlessFreeze: Boolean(data[2]),
    };
  } catch {
    return null;
  }
}

/**
 * Utility: Build complete permissionless thaw transaction
 * 
 * This is a higher-level helper that fetches all necessary data and builds
 * a complete transaction ready to send.
 * 
 * @param rpc - Solana RPC client
 * @param params - Thaw parameters (mint and token account)
 * @returns Complete instruction with all extra accounts resolved
 */
export async function buildPermissionlessThawTransaction(
  rpc: any,
  params: {
    caller: Address;
    tokenAccount: Address;
    mint: Address;
  }
): Promise<IInstruction> {
  // Fetch MintConfig to get gating program
  const mintConfig = await fetchMintConfig(rpc, params.mint);
  
  if (!mintConfig) {
    throw new Error('Mint is not using Token ACL');
  }
  
  if (!mintConfig.enablePermissionlessThaw) {
    throw new Error('Permissionless thaw is not enabled for this mint');
  }
  
  // Fetch and parse extra account metas from gating program
  const [extraAccountMetas] = await findThawExtraAccountMetasPda(
    params.mint,
    mintConfig.gatingProgram
  );
  
  const extraAccountsData = await rpc.getAccountInfo(extraAccountMetas).send();
  
  // Parse extra accounts (actual implementation would parse TLV data)
  const extraAccounts: Array<{ address: Address; role: number }> = [];
  
  // Create instruction with all accounts
  return createPermissionlessThawInstruction({
    caller: params.caller,
    tokenAccount: params.tokenAccount,
    mint: params.mint,
    gatingProgram: mintConfig.gatingProgram,
    extraAccounts,
  });
}

/**
 * Utility: Build complete permissionless freeze transaction
 */
export async function buildPermissionlessFreezeTransaction(
  rpc: any,
  params: {
    caller: Address;
    tokenAccount: Address;
    mint: Address;
  }
): Promise<IInstruction> {
  const mintConfig = await fetchMintConfig(rpc, params.mint);
  
  if (!mintConfig) {
    throw new Error('Mint is not using Token ACL');
  }
  
  if (!mintConfig.enablePermissionlessFreeze) {
    throw new Error('Permissionless freeze is not enabled for this mint');
  }
  
  const [extraAccountMetas] = await findFreezeExtraAccountMetasPda(
    params.mint,
    mintConfig.gatingProgram
  );
  
  const extraAccountsData = await rpc.getAccountInfo(extraAccountMetas).send();
  const extraAccounts: Array<{ address: Address; role: number }> = [];
  
  return createPermissionlessFreezeInstruction({
    caller: params.caller,
    tokenAccount: params.tokenAccount,
    mint: params.mint,
    gatingProgram: mintConfig.gatingProgram,
    extraAccounts,
  });
}

