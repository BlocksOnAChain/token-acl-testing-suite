# Quick Start Guide

## Installation (2 minutes)

```bash
# Clone the repository
git clone https://github.com/BlocksOnAChain/token-acl-testing-suite.git
cd token-acl-testing-suite

# Build everything
./setup.sh
```

## Run Tests (1 minute)

```bash
# Run all tests
./run_tests.sh
```

**Output**: You'll see 15 tests running with 56 assertions.

## View Results

```bash
# View integration test results
cat test-results/REAL_TEST_RESULTS.md

# View core logic test results
cat test-results/CORE_LOGIC_TEST_RESULTS.md
```

## What Gets Tested

### Integration Tests (5 tests, 26 assertions)
- PDA derivation correctness
- Discriminator validation
- MintConfig structure
- Permission flags
- Gating program validation

### Core Logic Tests (6 tests, 30 assertions)
- ✅ FAMP baseline freeze authority
- ✅ Interface optional method support
- ✅ **Permission de-escalation (SECURITY)**
- ✅ Gating program limited power
- ✅ Decision vs execution separation
- ✅ Issuer control with 3rd party gating

## Test Results Location

All results are auto-generated in the `test-results/` directory:

- `test-results/REAL_TEST_RESULTS.md` - Integration tests
- `test-results/CORE_LOGIC_TEST_RESULTS.md` - Core logic tests
- `test-results/README.md` - Guide to results

**View online**: https://github.com/BlocksOnAChain/token-acl-testing-suite/tree/main/test-results

## Documentation

All detailed docs are in the `docs/` directory:

```bash
cat docs/FINAL_SUMMARY.md           # Complete validation
cat docs/IMPLEMENTATION_GUIDE.md    # How to integrate
cat docs/TEST_PLAN.md               # Testing approach
```

## Current Results

**Latest Test Run**:
- ✅ 15 tests passing
- ✅ 56 assertions passing
- ✅ 100% success rate
- ✅ All critical sRFC 37 logic validated

---

**That's it! Your tests are running and results are generated.** ✅

