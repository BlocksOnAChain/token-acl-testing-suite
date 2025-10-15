/**
 * Geo-Blocking Demo
 * 
 * Real-world use case: A token issuer must comply with regional regulations,
 * blocking users from certain jurisdictions while allowing others.
 * 
 * This demo shows a hybrid allow/block list with oracle-based geo verification.
 */

// Mock types for demo purposes (actual implementation would use @solana/web3.js)
type Address = string;
const address = (str: string): Address => str;

// Mock helper functions (demos are conceptual/illustrative)
const findMintConfigPda = async (mint: Address) => [address('MintConfigPDA' + mint.substring(0, 20)), 0];
const createPermissionlessThawInstruction = async (params: any) => ({ type: 'thaw', ...params });
const createPermissionlessFreezeInstruction = async (params: any) => ({ type: 'freeze', ...params });

interface GeoBlockingDemo {
  tokenMint: Address;
  geoOracleProgram: Address;
  geoGatingProgram: Address;
  rpc: any;
}

/**
 * Jurisdiction status
 */
enum JurisdictionStatus {
  Allowed = 'ALLOWED',
  Blocked = 'BLOCKED',
  Restricted = 'RESTRICTED',
}

/**
 * Geo verification record
 */
interface GeoRecord {
  wallet: Address;
  country: string;
  region?: string;
  status: JurisdictionStatus;
  verifiedDate: Date;
  oracleSignature: string;
  ipAddress?: string; // Hashed for privacy
}

/**
 * Jurisdiction rules
 */
interface JurisdictionRule {
  country: string;
  status: JurisdictionStatus;
  reason: string;
  restrictions?: string[];
}

export class GeoBlockingManager {
  // Jurisdiction rules (maintained by issuer)
  private jurisdictionRules: JurisdictionRule[] = [
    {
      country: 'US',
      status: JurisdictionStatus.Allowed,
      reason: 'Fully compliant with SEC regulations',
    },
    {
      country: 'EU',
      status: JurisdictionStatus.Allowed,
      reason: 'MiCA compliant',
    },
    {
      country: 'CN',
      status: JurisdictionStatus.Blocked,
      reason: 'Regulatory restrictions',
    },
    {
      country: 'KP',
      status: JurisdictionStatus.Blocked,
      reason: 'International sanctions',
    },
    {
      country: 'SG',
      status: JurisdictionStatus.Restricted,
      reason: 'Restricted to accredited investors only',
      restrictions: ['Requires additional verification', 'Max $200k per year'],
    },
  ];

  constructor(private config: GeoBlockingDemo) {}

  /**
   * Step 1: User proves location via oracle
   * 
   * The oracle verifies user's location through various means
   * (IP geolocation, device location, KYC address, etc.)
   */
  async verifyUserLocation(user: Address, country: string): Promise<GeoRecord> {
    console.log('╔════════════════════════════════════════════════════════════╗');
    console.log('║  Geo-Location Verification via Oracle                     ║');
    console.log('╚════════════════════════════════════════════════════════════╝\n');
    
    console.log(`User: ${user}`);
    console.log(`Claimed Country: ${country}\n`);
    
    console.log('🌐 Oracle verification process:');
    console.log('  1. Checking IP geolocation...');
    console.log('  2. Verifying device location...');
    console.log('  3. Cross-referencing with KYC address...');
    console.log('  4. Generating cryptographic proof...\n');
    
    const rule = this.jurisdictionRules.find(r => r.country === country);
    const status = rule ? rule.status : JurisdictionStatus.Blocked;
    
    const record: GeoRecord = {
      wallet: user,
      country,
      status,
      verifiedDate: new Date(),
      oracleSignature: 'oracle-sig-' + Math.random().toString(36).substring(7),
      ipAddress: 'hashed-ip-' + Math.random().toString(36).substring(7),
    };
    
    console.log('✅ Location verified:');
    console.log(`   Country: ${record.country}`);
    console.log(`   Status: ${record.status}`);
    console.log(`   Oracle Signature: ${record.oracleSignature}\n`);
    
    if (rule) {
      console.log(`   Reason: ${rule.reason}`);
      if (rule.restrictions) {
        console.log('   Restrictions:');
        rule.restrictions.forEach(r => console.log(`     • ${r}`));
      }
    }
    console.log('\n');
    
    return record;
  }

  /**
   * Step 2: User creates on-chain geo record
   */
  async createGeoRecord(record: GeoRecord): Promise<string> {
    console.log('╔════════════════════════════════════════════════════════════╗');
    console.log('║  Creating On-Chain Geo Record                             ║');
    console.log('╚════════════════════════════════════════════════════════════╝\n');
    
    const geoPDA = await this.getGeoPDA(record.wallet);
    
    console.log(`User: ${record.wallet}`);
    console.log(`Geo PDA: ${geoPDA}\n`);
    
    console.log('Writing on-chain:');
    console.log(`  ✓ Country: ${record.country}`);
    console.log(`  ✓ Status: ${record.status}`);
    console.log(`  ✓ Oracle Signature: ${record.oracleSignature}`);
    console.log(`  ✓ Verified: ${record.verifiedDate.toISOString()}\n`);
    
    console.log('✅ Geo record created on-chain!\n');
    
    return 'mock-transaction-signature';
  }

  /**
   * Step 3a: Allowed jurisdiction - User can thaw
   */
  async allowedJurisdictionFlow(user: Address): Promise<void> {
    console.log('╔════════════════════════════════════════════════════════════╗');
    console.log('║  ALLOWED Jurisdiction Flow                                ║');
    console.log('╚════════════════════════════════════════════════════════════╝\n');
    
    console.log(`User: ${user}`);
    console.log('Country: US (ALLOWED)\n');
    
    console.log('📋 Creating token account...');
    const tokenAccount = address('UserTokenAccount' + user.substring(0, 20));
    console.log(`   Token account: ${tokenAccount}`);
    console.log('   Status: FROZEN (Default Account State)\n');
    
    console.log('🔓 Attempting permissionless thaw...');
    console.log('   Calling gating program...');
    console.log('   Checking geo record...');
    console.log('   Country: US ✓');
    console.log('   Status: ALLOWED ✓');
    console.log('   Oracle signature valid ✓\n');
    
    console.log('✅ PERMISSIONLESS THAW SUCCESS!');
    console.log('   Token account is now unfrozen and ready to use!\n');
    
    console.log('✨ User can now:');
    console.log('   • Trade on any DEX');
    console.log('   • Transfer to other allowed users');
    console.log('   • Participate in DeFi protocols\n');
  }

  /**
   * Step 3b: Blocked jurisdiction - User cannot thaw
   */
  async blockedJurisdictionFlow(user: Address): Promise<void> {
    console.log('╔════════════════════════════════════════════════════════════╗');
    console.log('║  BLOCKED Jurisdiction Flow                                ║');
    console.log('╚════════════════════════════════════════════════════════════╝\n');
    
    console.log(`User: ${user}`);
    console.log('Country: CN (BLOCKED)\n');
    
    console.log('📋 Creating token account...');
    const tokenAccount = address('UserTokenAccount' + user.substring(0, 20));
    console.log(`   Token account: ${tokenAccount}`);
    console.log('   Status: FROZEN (Default Account State)\n');
    
    console.log('❌ Attempting permissionless thaw...');
    console.log('   Calling gating program...');
    console.log('   Checking geo record...');
    console.log('   Country: CN ✗');
    console.log('   Status: BLOCKED ✗');
    console.log('   Reason: Regulatory restrictions\n');
    
    console.log('🚫 PERMISSIONLESS THAW DENIED!');
    console.log('   Token account remains frozen.\n');
    
    console.log('User receives clear error message:');
    console.log('   "Your jurisdiction is blocked from holding this token');
    console.log('    due to regulatory restrictions."\n');
  }

  /**
   * Step 3c: Restricted jurisdiction - Conditional access
   */
  async restrictedJurisdictionFlow(user: Address): Promise<void> {
    console.log('╔════════════════════════════════════════════════════════════╗');
    console.log('║  RESTRICTED Jurisdiction Flow                             ║');
    console.log('╚════════════════════════════════════════════════════════════╝\n');
    
    console.log(`User: ${user}`);
    console.log('Country: SG (RESTRICTED)\n');
    
    console.log('📋 Checking additional requirements...');
    console.log('   • Accredited investor status: Required ✓');
    console.log('   • Enhanced KYC: Required ✓');
    console.log('   • Investment limit: $200k per year\n');
    
    console.log('🔓 Attempting permissionless thaw...');
    console.log('   Calling gating program...');
    console.log('   Checking geo record...');
    console.log('   Country: SG ✓');
    console.log('   Status: RESTRICTED');
    console.log('   Checking additional requirements...');
    console.log('   Accredited investor verified ✓');
    console.log('   Enhanced KYC valid ✓\n');
    
    console.log('✅ PERMISSIONLESS THAW SUCCESS (with restrictions)!\n');
    
    console.log('User can trade with restrictions:');
    console.log('   • Max investment: $200,000 per year');
    console.log('   • Must maintain accredited status');
    console.log('   • Enhanced reporting required\n');
  }

  /**
   * Step 4: User relocates - Automatic freeze
   */
  async userRelocationFlow(
    user: Address,
    fromCountry: string,
    toCountry: string
  ): Promise<void> {
    console.log('╔════════════════════════════════════════════════════════════╗');
    console.log('║  User Relocation Flow                                     ║');
    console.log('╚════════════════════════════════════════════════════════════╝\n');
    
    console.log(`User: ${user}`);
    console.log(`Relocating: ${fromCountry} → ${toCountry}\n`);
    
    const newRecord = await this.verifyUserLocation(user, toCountry);
    
    if (newRecord.status === JurisdictionStatus.Blocked) {
      console.log('⚠️  New jurisdiction is BLOCKED!\n');
      
      console.log('Automatic actions:');
      console.log('  1. Oracle detects location change');
      console.log('  2. Updates on-chain geo record');
      console.log('  3. Automated system freezes all token accounts');
      console.log('  4. User receives notification\n');
      
      console.log('❄️  All token accounts frozen via permissionless freeze\n');
      
      console.log('User cannot trade until:');
      console.log('  • Relocates to allowed jurisdiction, OR');
      console.log('  • Provides evidence of temporary stay\n');
    } else {
      console.log('✅ New jurisdiction is allowed!\n');
      console.log('No action needed - user can continue trading.\n');
    }
  }

  /**
   * Compliance monitoring dashboard
   */
  async generateGeoComplianceReport(): Promise<void> {
    console.log('╔════════════════════════════════════════════════════════════╗');
    console.log('║         GEO-BLOCKING COMPLIANCE REPORT                    ║');
    console.log('╚════════════════════════════════════════════════════════════╝\n');
    
    console.log('Jurisdiction Rules Summary:\n');
    
    const allowed = this.jurisdictionRules.filter(r => r.status === JurisdictionStatus.Allowed);
    const blocked = this.jurisdictionRules.filter(r => r.status === JurisdictionStatus.Blocked);
    const restricted = this.jurisdictionRules.filter(r => r.status === JurisdictionStatus.Restricted);
    
    console.log(`✅ Allowed Jurisdictions (${allowed.length}):`);
    allowed.forEach(r => {
      console.log(`   • ${r.country}: ${r.reason}`);
    });
    console.log('');
    
    console.log(`🚫 Blocked Jurisdictions (${blocked.length}):`);
    blocked.forEach(r => {
      console.log(`   • ${r.country}: ${r.reason}`);
    });
    console.log('');
    
    console.log(`⚠️  Restricted Jurisdictions (${restricted.length}):`);
    restricted.forEach(r => {
      console.log(`   • ${r.country}: ${r.reason}`);
      if (r.restrictions) {
        r.restrictions.forEach(rest => console.log(`     - ${rest}`));
      }
    });
    console.log('');
    
    console.log('Enforcement Metrics:');
    console.log('  ✓ Oracle-verified locations: On-chain');
    console.log('  ✓ Real-time geo-blocking: Automated');
    console.log('  ✓ Relocation detection: Automated');
    console.log('  ✓ Compliance audit trail: Immutable\n');
  }

  // Helper methods
  private async getGeoPDA(wallet: Address): Promise<Address> {
    return address('GeoPDA' + wallet.substring(0, 37));
  }
}

/**
 * Run the complete geo-blocking demo
 */
export async function runGeoBlockingDemo() {
  console.log('\n');
  console.log('═══════════════════════════════════════════════════════════════════');
  console.log('   🌍 GEO-BLOCKING DEMO - Token ACL Real-World Use Case');
  console.log('═══════════════════════════════════════════════════════════════════');
  console.log('\n');
  console.log('Scenario: Global token with regional compliance requirements');
  console.log('Solution: Token ACL + Oracle-based Geo Gating Program');
  console.log('\n');

  const demo = new GeoBlockingManager({
    tokenMint: address('GlobalToken11111111111111111111111111111'),
    geoOracleProgram: address('GeoOracle111111111111111111111111111111'),
    geoGatingProgram: address('GeoGating111111111111111111111111111111'),
    rpc: null,
  });

  // Scenario 1: User from allowed jurisdiction (US)
  console.log('═══ SCENARIO 1: User from Allowed Jurisdiction (US) ═══\n');
  
  const usUser = address('USUser1111111111111111111111111111111111');
  const usGeoRecord = await demo.verifyUserLocation(usUser, 'US');
  await demo.createGeoRecord(usGeoRecord);
  await demo.allowedJurisdictionFlow(usUser);

  // Scenario 2: User from blocked jurisdiction (CN)
  console.log('\n═══ SCENARIO 2: User from Blocked Jurisdiction (CN) ═══\n');
  
  const cnUser = address('CNUser1111111111111111111111111111111111');
  const cnGeoRecord = await demo.verifyUserLocation(cnUser, 'CN');
  await demo.createGeoRecord(cnGeoRecord);
  await demo.blockedJurisdictionFlow(cnUser);

  // Scenario 3: User from restricted jurisdiction (SG)
  console.log('\n═══ SCENARIO 3: User from Restricted Jurisdiction (SG) ═══\n');
  
  const sgUser = address('SGUser1111111111111111111111111111111111');
  const sgGeoRecord = await demo.verifyUserLocation(sgUser, 'SG');
  await demo.createGeoRecord(sgGeoRecord);
  await demo.restrictedJurisdictionFlow(sgUser);

  // Scenario 4: User relocates to blocked jurisdiction
  console.log('\n═══ SCENARIO 4: User Relocates to Blocked Jurisdiction ═══\n');
  
  await demo.userRelocationFlow(usUser, 'US', 'KP');

  // Compliance report
  console.log('\n═══ COMPLIANCE REPORTING ═══\n');
  
  await demo.generateGeoComplianceReport();

  // Summary
  console.log('\n═══ GEO-BLOCKING BENEFITS ═══\n');
  
  console.log('✨ Compliance Benefits:');
  console.log('   • Automated regional restrictions');
  console.log('   • Oracle-verified locations (trustless)');
  console.log('   • Real-time relocation detection');
  console.log('   • Granular jurisdiction rules\n');
  
  console.log('✨ User Benefits:');
  console.log('   • Clear geo restrictions up-front');
  console.log('   • Instant access in allowed regions');
  console.log('   • No manual approval process\n');
  
  console.log('✨ Issuer Benefits:');
  console.log('   • Automated geo-compliance');
  console.log('   • Flexible restriction rules');
  console.log('   • Reduced legal risk');
  console.log('   • Scalable globally\n');
  
  console.log('✨ Innovation:');
  console.log('   • Hybrid allow/block list model');
  console.log('   • Oracle integration for trustless geo verification');
  console.log('   • Dynamic restrictions based on jurisdiction');
  console.log('   • Automatic enforcement without manual intervention\n');
  
  console.log('═══════════════════════════════════════════════════════════════════\n');
}

// Run the demo (this file is meant to be executed directly)
runGeoBlockingDemo().catch(console.error);

