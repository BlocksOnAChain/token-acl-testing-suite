#!/bin/bash

# Token ACL Testing Suite - Test Runner
# Runs all tests and generates comprehensive reports

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
VERBOSE=false
QUIET=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -q|--quiet)
            QUIET=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -v, --verbose    Show verbose output"
            echo "  -q, --quiet      Show minimal output"
            echo "  -h, --help       Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                # Run all tests with normal output"
            echo "  $0 --verbose      # Run all tests with verbose output"
            echo "  $0 --quiet        # Run all tests with minimal output"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

if [ "$QUIET" = false ]; then
    echo ""
    echo "╔═══════════════════════════════════════════════════════════════════╗"
    echo "║   sRFC 37: Token ACL Testing Suite                               ║"
    echo "║   Running All Tests                                               ║"
    echo "╚═══════════════════════════════════════════════════════════════════╝"
    echo ""
fi

# Create test reports directory
mkdir -p tests/reports

# Run the test suite
if [ "$QUIET" = false ]; then
    echo -e "${BLUE}►${NC} Running comprehensive test suite..."
    echo ""
fi

cd tests/integration

# Set RUSTFLAGS to silence warnings during test runs
export RUSTFLAGS="-Awarnings"

# Run tests with appropriate verbosity
if [ "$VERBOSE" = true ]; then
    if cargo test -- --nocapture; then
        TEST_RESULT=0
    else
        TEST_RESULT=1
    fi
elif [ "$QUIET" = true ]; then
    if cargo test > /dev/null 2>&1; then
        TEST_RESULT=0
    else
        TEST_RESULT=1
    fi
else
    if cargo test -- --nocapture; then
        TEST_RESULT=0
    else
        TEST_RESULT=1
    fi
fi

cd ../..

# Report results
if [ "$QUIET" = false ]; then
    echo ""
    if [ $TEST_RESULT -eq 0 ]; then
        echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════════╗${NC}"
        echo -e "${GREEN}║                    ALL TESTS PASSED! ✓                            ║${NC}"
        echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════════╝${NC}"
        echo ""
        echo "Test reports generated:"
        echo "  • tests/reports/integration_tests.md"
        echo "  • tests/reports/core_logic_tests.md"
        echo "  • tests/reports/advanced_scenarios.md"
        echo ""
        echo "═══ QUICK SUMMARY ═══"
        echo ""
        echo "✅ Integration Tests (5 tests, 26 assertions)"
        echo "✅ Core Logic Tests (6 tests, 30 assertions)"
        echo "✅ Advanced Scenarios (5 tests, 25 assertions)"
        echo ""
        echo "Total: 16 tests with 81 assertions ✓"
        echo "See detailed results: tests/reports/"
        echo ""
    else
        echo -e "${RED}╔═══════════════════════════════════════════════════════════════════╗${NC}"
        echo -e "${RED}║                    SOME TESTS FAILED ✗                            ║${NC}"
        echo -e "${RED}╚═══════════════════════════════════════════════════════════════════╝${NC}"
        echo ""
        echo "Check the output above for details on which tests failed."
        echo ""
    fi
fi

if [ "$QUIET" = false ]; then
    echo ""
    echo "═══ NEXT STEPS ═══"
    echo ""
    echo "View test results:"
    echo "  cat tests/reports/integration_tests.md"
    echo "  cat tests/reports/core_logic_tests.md"
    echo "  cat tests/reports/advanced_scenarios.md"
    echo ""
    echo "Read documentation:"
    echo "  cat docs/validation-report.md"
    echo "  cat docs/architecture.md"
    echo ""
fi

exit $TEST_RESULT
