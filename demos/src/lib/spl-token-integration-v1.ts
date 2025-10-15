/**
 * SPL Token Integration for Token ACL (PRODUCTION READY)
 * 
 * This module extends @solana/spl-token with Token ACL support
 * using web3.js v1.x (current stable version).
 * 
 * Compatible with:
 * - @solana/web3.js ^1.95.5
 * - @solana/spl-token ^0.4.14
 */

import {
  PublicKey,
  Connection,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js';
import {
  getAccount,
  Account as TokenAccount,
} from '@solana/spl-token';
import {
  findMintConfigPda,
  fetchMintConfig,
  createPermissionlessThawInstruction,
  createPermissionlessFreezeInstruction,
  buildPermissionlessThawInstruction,
  buildPermissionlessFreezeInstruction,
  isTokenAclMint,
} from './token-acl-helpers-v1.js';

/**
 * Extended token account interface with ACL support
 */
export interface TokenAccountWithACL extends TokenAccount {
  isTokenACL: boolean;
  canPermissionlessThaw?: boolean;
  canPermissionlessFreeze?: boolean;
}

/**
 * Get token account with ACL information
 * 
 * This extends the standard getAccount function to include Token ACL metadata.
 * 
 * @param connection - Solana connection
 * @param tokenAccount - Token account public key
 * @returns Extended token account info with ACL data
 * 
 * @example
 * ```ts
 * const account = await getAccountWithACL(connection, tokenAccountPubkey);
 * if (account.isFrozen && account.canPermissionlessThaw) {
 *   console.log('User can thaw this account themselves!');
 * }
 * ```
 */
export async function getAccountWithACL(
  connection: Connection,
  tokenAccount: PublicKey
): Promise<TokenAccountWithACL> {
  // Fetch standard token account data
  const account = await getAccount(connection, tokenAccount);
  
  // Check if mint uses Token ACL
  const isACL = await isTokenAclMint(connection, account.mint);
  
  let canPermissionlessThaw = false;
  let canPermissionlessFreeze = false;
  
  if (isACL) {
    const mintConfig = await fetchMintConfig(connection, account.mint);
    if (mintConfig) {
      canPermissionlessThaw = mintConfig.enablePermissionlessThaw;
      canPermissionlessFreeze = mintConfig.enablePermissionlessFreeze;
    }
  }
  
  return {
    ...account,
    isTokenACL: isACL,
    canPermissionlessThaw,
    canPermissionlessFreeze,
  };
}

/**
 * Smart thaw - automatically chooses permissionless or permissioned
 * 
 * @param connection - Solana connection
 * @param tokenAccount - Token account to thaw
 * @param signer - The signer (user for permissionless, authority for permissioned)
 * @returns The thaw instruction
 * 
 * @example
 * ```ts
 * // User thawing their own account
 * const thawIx = await smartThaw(connection, tokenAccount, userPublicKey);
 * 
 * // If Token ACL: uses permissionless thaw
 * // Otherwise: would need freeze authority
 * ```
 */
export async function smartThaw(
  connection: Connection,
  tokenAccount: PublicKey,
  signer: PublicKey
): Promise<TransactionInstruction> {
  const account = await getAccountWithACL(connection, tokenAccount);
  
  if (!account.isFrozen) {
    throw new Error('Account is not frozen');
  }
  
  if (account.isTokenACL && account.canPermissionlessThaw) {
    // Use permissionless thaw
    return await buildPermissionlessThawInstruction(
      connection,
      signer,
      tokenAccount,
      account.mint
    );
  } else {
    // Would need to use standard freeze authority thaw
    throw new Error(
      'Permissionless thaw not available. Freeze authority required.'
    );
  }
}

/**
 * Smart freeze - automatically chooses permissionless or permissioned
 * 
 * @param connection - Solana connection
 * @param tokenAccount - Token account to freeze
 * @param signer - The signer
 * @returns The freeze instruction
 */
export async function smartFreeze(
  connection: Connection,
  tokenAccount: PublicKey,
  signer: PublicKey
): Promise<TransactionInstruction> {
  const account = await getAccountWithACL(connection, tokenAccount);
  
  if (account.isFrozen) {
    throw new Error('Account is already frozen');
  }
  
  if (account.isTokenACL && account.canPermissionlessFreeze) {
    // Use permissionless freeze
    return await buildPermissionlessFreezeInstruction(
      connection,
      signer,
      tokenAccount,
      account.mint
    );
  } else {
    throw new Error(
      'Permissionless freeze not available. Freeze authority required.'
    );
  }
}

/**
 * Check if user can thaw a token account
 * 
 * Useful for UI to show/hide thaw button
 * 
 * @param connection - Solana connection
 * @param tokenAccount - Token account public key
 * @param user - User public key
 * @returns True if user can thaw the account
 * 
 * @example
 * ```ts
 * if (await canUserThaw(connection, tokenAccount, userPublicKey)) {
 *   showThawButton();
 * }
 * ```
 */
export async function canUserThaw(
  connection: Connection,
  tokenAccount: PublicKey,
  user: PublicKey
): Promise<boolean> {
  try {
    const account = await getAccountWithACL(connection, tokenAccount);
    
    if (!account.isFrozen) {
      return false; // Already thawed
    }
    
    if (!account.owner.equals(user)) {
      return false; // Not the owner
    }
    
    return account.isTokenACL && account.canPermissionlessThaw;
  } catch {
    return false;
  }
}

/**
 * Batch thaw multiple token accounts
 * 
 * Useful for users with multiple frozen accounts.
 * 
 * @param connection - Solana connection
 * @param tokenAccounts - Array of token account public keys
 * @param signer - The signer
 * @returns Array of thaw instructions
 * 
 * @example
 * ```ts
 * const instructions = await batchThaw(connection, tokenAccounts, userPublicKey);
 * const transaction = new Transaction().add(...instructions);
 * ```
 */
export async function batchThaw(
  connection: Connection,
  tokenAccounts: PublicKey[],
  signer: PublicKey
): Promise<TransactionInstruction[]> {
  const instructions: TransactionInstruction[] = [];
  
  for (const tokenAccount of tokenAccounts) {
    try {
      const ix = await smartThaw(connection, tokenAccount, signer);
      instructions.push(ix);
    } catch (error) {
      console.warn(`Failed to create thaw for ${tokenAccount.toBase58()}:`, error);
    }
  }
  
  return instructions;
}

/**
 * Batch freeze multiple token accounts
 * 
 * Useful for sanctions enforcement - freeze all accounts owned by blocked user.
 * 
 * @param connection - Solana connection
 * @param tokenAccounts - Array of token account public keys
 * @param signer - The signer (can be anyone for permissionless freeze)
 * @returns Array of freeze instructions
 */
export async function batchFreeze(
  connection: Connection,
  tokenAccounts: PublicKey[],
  signer: PublicKey
): Promise<TransactionInstruction[]> {
  const instructions: TransactionInstruction[] = [];
  
  for (const tokenAccount of tokenAccounts) {
    try {
      const ix = await smartFreeze(connection, tokenAccount, signer);
      instructions.push(ix);
    } catch (error) {
      console.warn(`Failed to create freeze for ${tokenAccount.toBase58()}:`, error);
    }
  }
  
  return instructions;
}

