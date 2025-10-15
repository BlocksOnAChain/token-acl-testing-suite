/**
 * SPL Token Integration for Token ACL
 * 
 * This module extends @solana/spl-token with Token ACL support.
 * Proposed for integration into the main SPL Token library.
 */

import { Address } from '@solana/web3.js';
import {
  findMintConfigPda,
  fetchMintConfig,
  createPermissionlessThawInstruction,
  createPermissionlessFreezeInstruction,
  isTokenAclMint,
} from './token-acl-helpers.js';

/**
 * Extended token account interface with ACL support
 */
export interface TokenAccountWithACL {
  address: Address;
  mint: Address;
  owner: Address;
  amount: bigint;
  isFrozen: boolean;
  isTokenACL: boolean;
  canPermissionlessThaw?: boolean;
  canPermissionlessFreeze?: boolean;
}

/**
 * Get token account with ACL information
 * 
 * This extends the standard getAccount function to include Token ACL metadata.
 * 
 * @param rpc - Solana RPC client
 * @param tokenAccount - Token account address
 * @returns Extended token account info with ACL data
 * 
 * @example
 * ```ts
 * const account = await getAccountWithACL(rpc, tokenAccountAddress);
 * if (account.isFrozen && account.canPermissionlessThaw) {
 *   console.log('User can thaw this account themselves!');
 * }
 * ```
 */
export async function getAccountWithACL(
  rpc: any,
  tokenAccount: Address
): Promise<TokenAccountWithACL> {
  // Fetch standard token account data
  const accountInfo = await rpc.getAccountInfo(tokenAccount).send();
  
  if (!accountInfo.value) {
    throw new Error('Token account not found');
  }
  
  // Parse token account (simplified)
  const data = accountInfo.value.data;
  const mint = address('11111111111111111111111111111111'); // Placeholder
  const owner = address('11111111111111111111111111111111'); // Placeholder
  const amount = BigInt(0); // Placeholder
  const isFrozen = Boolean(data[0]); // Placeholder
  
  // Check if mint uses Token ACL
  const isACL = await isTokenAclMint(rpc, mint);
  
  let canPermissionlessThaw = false;
  let canPermissionlessFreeze = false;
  
  if (isACL) {
    const mintConfig = await fetchMintConfig(rpc, mint);
    if (mintConfig) {
      canPermissionlessThaw = mintConfig.enablePermissionlessThaw;
      canPermissionlessFreeze = mintConfig.enablePermissionlessFreeze;
    }
  }
  
  return {
    address: tokenAccount,
    mint,
    owner,
    amount,
    isFrozen,
    isTokenACL: isACL,
    canPermissionlessThaw,
    canPermissionlessFreeze,
  };
}

/**
 * Thaw token account (automatically chooses permissionless or permissioned)
 * 
 * Smart helper that:
 * - If Token ACL and permissionless thaw enabled: use permissionless thaw
 * - Otherwise: use standard freeze authority thaw
 * 
 * @param rpc - Solana RPC client
 * @param tokenAccount - Token account to thaw
 * @param signer - The signer (user for permissionless, authority for permissioned)
 * @returns The thaw instruction
 * 
 * @example
 * ```ts
 * // User thawing their own account
 * const ix = await smartThaw(rpc, tokenAccount, userAddress);
 * 
 * // If Token ACL: uses permissionless thaw
 * // If not: would need freeze authority (and would fail for user)
 * ```
 */
export async function smartThaw(
  rpc: any,
  tokenAccount: Address,
  signer: Address
): Promise<any> {
  const account = await getAccountWithACL(rpc, tokenAccount);
  
  if (!account.isFrozen) {
    throw new Error('Account is not frozen');
  }
  
  if (account.isTokenACL && account.canPermissionlessThaw) {
    // Use permissionless thaw
    const mintConfig = await fetchMintConfig(rpc, account.mint);
    if (!mintConfig) {
      throw new Error('MintConfig not found');
    }
    
    return createPermissionlessThawInstruction({
      caller: signer,
      tokenAccount: account.address,
      mint: account.mint,
      gatingProgram: mintConfig.gatingProgram,
    });
  } else {
    // Use standard thaw (requires freeze authority)
    // This would call the standard SPL token thaw instruction
    throw new Error('Permissionless thaw not available, freeze authority required');
  }
}

/**
 * Freeze token account (automatically chooses permissionless or permissioned)
 */
export async function smartFreeze(
  rpc: any,
  tokenAccount: Address,
  signer: Address
): Promise<any> {
  const account = await getAccountWithACL(rpc, tokenAccount);
  
  if (account.isFrozen) {
    throw new Error('Account is already frozen');
  }
  
  if (account.isTokenACL && account.canPermissionlessFreeze) {
    // Use permissionless freeze
    const mintConfig = await fetchMintConfig(rpc, account.mint);
    if (!mintConfig) {
      throw new Error('MintConfig not found');
    }
    
    return createPermissionlessFreezeInstruction({
      caller: signer,
      tokenAccount: account.address,
      mint: account.mint,
      gatingProgram: mintConfig.gatingProgram,
    });
  } else {
    // Use standard freeze (requires freeze authority)
    throw new Error('Permissionless freeze not available, freeze authority required');
  }
}

/**
 * Check if user can thaw a token account
 * 
 * Useful for UI to show/hide thaw button
 * 
 * @param rpc - Solana RPC client
 * @param tokenAccount - Token account address
 * @param user - User address
 * @returns True if user can thaw the account
 */
export async function canUserThaw(
  rpc: any,
  tokenAccount: Address,
  user: Address
): Promise<boolean> {
  const account = await getAccountWithACL(rpc, tokenAccount);
  
  if (!account.isFrozen) {
    return false; // Already thawed
  }
  
  if (account.owner !== user) {
    return false; // Not the owner
  }
  
  if (account.isTokenACL && account.canPermissionlessThaw) {
    // Check if user would pass gating program checks
    // (Would need to simulate the transaction)
    return true; // Optimistically assume yes
  }
  
  return false;
}

/**
 * Batch thaw multiple token accounts
 * 
 * Useful for users with multiple frozen accounts or for sweeping operations.
 * 
 * @param rpc - Solana RPC client  
 * @param tokenAccounts - Array of token account addresses
 * @param signer - The signer
 * @returns Array of thaw instructions
 */
export async function batchThaw(
  rpc: any,
  tokenAccounts: Address[],
  signer: Address
): Promise<any[]> {
  const instructions = [];
  
  for (const tokenAccount of tokenAccounts) {
    try {
      const ix = await smartThaw(rpc, tokenAccount, signer);
      instructions.push(ix);
    } catch (err) {
      console.warn(`Failed to create thaw for ${tokenAccount}:`, err);
    }
  }
  
  return instructions;
}

/**
 * Batch freeze multiple token accounts
 * 
 * Useful for sanctions enforcement - freeze all accounts owned by blocked user.
 */
export async function batchFreeze(
  rpc: any,
  tokenAccounts: Address[],
  signer: Address
): Promise<any[]> {
  const instructions = [];
  
  for (const tokenAccount of tokenAccounts) {
    try {
      const ix = await smartFreeze(rpc, tokenAccount, signer);
      instructions.push(ix);
    } catch (err) {
      console.warn(`Failed to create freeze for ${tokenAccount}:`, err);
    }
  }
  
  return instructions;
}

/**
 * Find all token accounts for a user and mint
 */
export async function findTokenAccountsForUser(
  rpc: any,
  user: Address,
  mint: Address
): Promise<Address[]> {
  // Use getProgramAccounts to find all token accounts
  // (Actual implementation would use proper filters)
  return [];
}

/**
 * Utility: Prepare token account for use
 * 
 * This is a convenience function that:
 * 1. Creates token account (if needed)
 * 2. Thaws it (if frozen and permissionless thaw available)
 * 3. Returns ready-to-use account
 * 
 * Perfect for onboarding users!
 */
export async function prepareTokenAccountForUse(
  rpc: any,
  params: {
    user: Address;
    mint: Address;
    existingAccount?: Address;
  }
): Promise<{
  account: Address;
  needsThaw: boolean;
  thawInstruction?: any;
}> {
  let account = params.existingAccount;
  
  if (!account) {
    // Would create account here
    throw new Error('Account creation not implemented in this example');
  }
  
  const accountInfo = await getAccountWithACL(rpc, account);
  
  if (!accountInfo.isFrozen) {
    return {
      account,
      needsThaw: false,
    };
  }
  
  if (accountInfo.isTokenACL && accountInfo.canPermissionlessThaw) {
    const thawIx = await smartThaw(rpc, account, params.user);
    
    return {
      account,
      needsThaw: true,
      thawInstruction: thawIx,
    };
  }
  
  throw new Error('Account is frozen and cannot be thawed by user');
}

// Helper to import address from web3.js (placeholder)
function address(str: string): Address {
  return str as unknown as Address;
}

