#!/bin/bash

# Token ACL Testing Suite - Test Runner
# Runs all tests and generates comprehensive report

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║   sRFC 37: Token ACL Testing Suite                               ║"
echo "║   Running All Tests                                               ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Create test-results directory
mkdir -p test-results

# Run the test suite
echo -e "${BLUE}►${NC} Running real validation test suite..."
echo ""

cd tests/integration

# Silence rustc warnings during test runs
if RUSTFLAGS="-Awarnings" cargo test -- --nocapture; then
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                    ALL TESTS PASSED! ✓                            ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo "Test reports generated:"
    echo "  • test-results/REAL_TEST_RESULTS.md"
    echo "  • test-results/CORE_LOGIC_TEST_RESULTS.md"
    echo ""
    echo "═══ QUICK SUMMARY ═══"
    echo ""
    echo "✅ PDA Derivation Tests"
    echo "✅ Discriminator Validation Tests"
    echo "✅ MintConfig Structure Tests"
    echo "✅ Permission Flags Tests"
    echo "✅ Gating Program Validation Tests"
    echo ""
    echo "Total: Real validation tests with actual assertions ✓"
    echo "See detailed results: test-results/REAL_TEST_RESULTS.md"
    echo ""
    EXIT_CODE=0
else
    echo ""
    echo -e "${RED}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║                    SOME TESTS FAILED ✗                            ║${NC}"
    echo -e "${RED}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo "Note: Tests are conceptual/illustrative. Check output above for details."
    echo ""
    EXIT_CODE=1
fi

cd ../..

echo ""
echo "═══ NEXT STEPS ═══"
echo ""
echo "View test results:"
echo "  cat test-results/REAL_TEST_RESULTS.md"
echo "  cat test-results/CORE_LOGIC_TEST_RESULTS.md"
echo ""
echo "Read documentation:"
echo "  cat docs/FINAL_SUMMARY.md"
echo "  cat docs/IMPLEMENTATION_GUIDE.md"
echo ""

exit $EXIT_CODE

