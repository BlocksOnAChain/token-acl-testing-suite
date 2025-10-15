# Real-World Use Case Demos

This directory contains practical demonstrations of sRFC 37 Token ACL for common real-world scenarios.

## Use Cases

### 1. 🚫 Sanctions List (Block List)
**File**: `sanctions-list-demo.ts`

A financial institution issuing stablecoins needs to comply with OFAC sanctions. When a wallet is sanctioned, all token accounts must be frozen immediately.

**Features**:
- Permissionless freeze for sanctioned wallets
- Anyone can sweep sanctioned accounts
- No issuer intervention needed after sanctions update
- Compliance automation

**Workflow**:
1. Compliance officer adds wallet to sanctions list
2. Automated system or anyone can freeze all token accounts
3. User cannot transfer tokens
4. If removed from sanctions list, user can permissionlessly thaw

### 2. ✅ KYC Allow List
**File**: `kyc-allowlist-demo.ts`

A security token issuer requires KYC verification. Only KYC-verified users can hold and transfer tokens.

**Features**:
- Permissionless thaw after KYC completion
- Instant access (no waiting for issuer)
- KYC status on-chain
- Tiered access levels

**Workflow**:
1. User completes KYC off-chain
2. KYC provider adds user to on-chain allow list
3. User creates token account (frozen by default)
4. User immediately thaws via permissionless operation
5. User can now trade normally

### 3. 🌍 Geo-Blocking (Hybrid)
**File**: `geo-blocking-demo.ts`

A token issuer needs to comply with regional regulations, blocking users from certain jurisdictions while allowing others.

**Features**:
- Oracle-based geo verification
- Hybrid allow/block list
- Regional compliance
- Automatic enforcement

**Workflow**:
1. User proves location via oracle
2. If allowed region: can permissionless thaw
3. If blocked region: cannot thaw
4. If region changes: automatic freeze via permissionless operation

## Running Demos

### Prerequisites

```bash
# Install dependencies
npm install

# Start local validator
solana-test-validator
```

### Run Individual Demos

```bash
# Sanctions list demo
npm run demo:sanctions

# KYC allowlist demo
npm run demo:kyc

# Geo-blocking demo
npm run demo:geo
```

### Run All Demos

```bash
npm run demo:all
```

## Integration with Production

Each demo includes:
- Full TypeScript implementation
- web3.js v2 helpers
- SPL Token integration
- Error handling
- Testing utilities
- Production deployment guide

