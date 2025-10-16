# Token ACL Advanced Scenarios Test Results

**Generated**: 2025-10-16 13:28:34 UTC

## Summary

- **Total Tests**: 5
- **Passed**: 5 (100%)
- **Failed**: 0
- **Total Assertions**: 25

✅ **ALL ADVANCED SCENARIO TESTS PASSED!**

## Real-World Scenarios Validated

### 1. KYC Allowlist with Expiration
✅ Valid KYC records recognized
✅ Expired KYC records rejected
✅ Non-accredited investors blocked

### 2. Sanctions List Precedence
✅ Sanctions ALWAYS override allowlist
✅ Sanctioned users blocked even if allowlisted
✅ Unknown users blocked by default

### 3. Geo-blocking
✅ Allowed jurisdictions granted access
✅ OFAC countries blocked
✅ Non-allowed jurisdictions blocked

### 4. Freeze/Thaw with Revocation
✅ Normal freeze/thaw cycle works
✅ Revoked accounts cannot be thawed (permanent)
✅ Revoke implies freeze

### 5. Multi-step RWA Workflow
✅ All steps required for trading
✅ Incomplete onboarding blocks trading
✅ Sanctioned investors blocked regardless
✅ Frozen accounts cannot trade

## Detailed Results

| Test | Status | Assertions | Details |
|------|--------|------------|----------|
| KYC Allowlist with Expiration | ✅ PASS | 5 | - |
| Sanctions List Precedence | ✅ PASS | 4 | - |
| Geo-blocking by Jurisdiction | ✅ PASS | 4 | - |
| Freeze/Thaw with Revocation | ✅ PASS | 7 | - |
| Multi-step RWA Workflow | ✅ PASS | 5 | - |

