#!/bin/bash

# sRFC 37 Token ACL - Test Runner
# This script runs all tests and generates comprehensive reports

set -e

echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║   sRFC 37: Token ACL Testing Suite                               ║"
echo "║   Running All Tests                                               ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create results directory
mkdir -p results

echo "Step 1: Building test client..."
cd tests/test-client
RUSTFLAGS="-Awarnings" cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Build successful${NC}"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

echo ""
echo "Step 2: Building gate programs..."
cd ../../gate_programs/allow_list
RUSTFLAGS="-Awarnings" cargo build --release
cd ../block_list
RUSTFLAGS="-Awarnings" cargo build --release
cd ../..

echo -e "${GREEN}✅ Gate programs built${NC}"

echo ""
echo "Step 3: Running tests..."
cd tests/test-client
RUSTFLAGS="-Awarnings" cargo run --release

TEST_RESULT=$?

echo ""
if [ $TEST_RESULT -eq 0 ]; then
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                    ALL TESTS PASSED! 🎉                           ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════════╝${NC}"
else
    echo -e "${RED}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║                    SOME TESTS FAILED ❌                            ║${NC}"
    echo -e "${RED}╚═══════════════════════════════════════════════════════════════════╝${NC}"
fi

echo ""
echo "Test reports generated at:"
echo "  • test-results/REAL_TEST_RESULTS.md"
echo "  • test-results/CORE_LOGIC_TEST_RESULTS.md"
echo ""

exit $TEST_RESULT

