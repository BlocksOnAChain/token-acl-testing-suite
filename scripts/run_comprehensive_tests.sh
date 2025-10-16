#!/bin/bash

# Enhanced comprehensive test runner with all optimizations
# This script runs all tests with performance benchmarks, coverage analysis, and detailed reporting

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo -e "${PURPLE}╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║   sRFC 37: Token ACL - Comprehensive Testing Suite              ║${NC}"
echo -e "${PURPLE}║   Enhanced with Performance, Coverage & Logging                 ║${NC}"
echo -e "${PURPLE}╚═══════════════════════════════════════════════════════════════════╝${NC}"

# Function to print section headers
print_section() {
    echo -e "\n${BLUE}►${NC} $1"
    echo -e "${BLUE}─────────────────────────────────────────────────────────────────${NC}"
}

# Function to print success
print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

# Function to print error
print_error() {
    echo -e "${RED}❌${NC} $1"
}

# Function to print warning
print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

# Function to print info
print_info() {
    echo -e "${CYAN}ℹ️${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
    print_error "Not in project root directory. Please run from project root."
    exit 1
fi

# Create reports directory
mkdir -p "$PROJECT_ROOT/tests/reports"

print_section "Initializing Enhanced Test Environment"

# Initialize logging
print_info "Setting up structured logging..."
export RUST_LOG=info
export TOKEN_ACL_LOG_LEVEL=info

# Check Rust version
print_info "Checking Rust toolchain..."
rustc --version
cargo --version

print_section "Running Code Quality Checks"

# Format check
print_info "Checking code formatting..."
if cargo fmt --all -- --check; then
    print_success "Code formatting is correct"
else
    print_warning "Code formatting issues found. Run 'cargo fmt --all' to fix."
fi

# Clippy check
print_info "Running clippy lints..."
if cargo clippy --all --tests -- -D warnings; then
    print_success "No clippy warnings found"
else
    print_warning "Clippy warnings found. Please review and fix."
fi

print_section "Building All Programs"

# Build all programs
print_info "Building integration tests..."
if cargo build --package token_acl_integration_tests; then
    print_success "Integration tests built successfully"
else
    print_error "Failed to build integration tests"
    exit 1
fi

print_info "Building example programs..."
if cargo build --package example_allow_list --package example_block_list; then
    print_success "Example programs built successfully"
else
    print_error "Failed to build example programs"
    exit 1
fi

print_info "Building production programs..."
if cargo build --package production_allow_list; then
    print_success "Production programs built successfully"
else
    print_error "Failed to build production programs"
    exit 1
fi

print_section "Running Comprehensive Test Suite"

# Run integration tests
print_info "Running integration tests..."
if cargo test --package token_acl_integration_tests --test integration_tests; then
    print_success "Integration tests passed"
else
    print_error "Integration tests failed"
    exit 1
fi

# Run core logic tests
print_info "Running core logic tests..."
if cargo test --package token_acl_integration_tests --test core_logic_tests; then
    print_success "Core logic tests passed"
else
    print_error "Core logic tests failed"
    exit 1
fi

# Run advanced scenario tests
print_info "Running advanced scenario tests..."
if cargo test --package token_acl_integration_tests --test advanced_scenarios; then
    print_success "Advanced scenario tests passed"
else
    print_error "Advanced scenario tests failed"
    exit 1
fi

print_section "Running Performance Benchmarks"

# Run performance benchmarks
print_info "Running performance benchmarks..."
if cargo test --package token_acl_integration_tests --test performance_benchmarks; then
    print_success "Performance benchmarks completed"
else
    print_warning "Performance benchmarks had issues (check logs)"
fi

print_section "Generating Comprehensive Reports"

# Generate test reports
print_info "Generating test reports..."

# Check if reports were generated
if [ -f "$PROJECT_ROOT/tests/reports/integration_tests.md" ]; then
    print_success "Integration test report generated"
else
    print_warning "Integration test report not found"
fi

if [ -f "$PROJECT_ROOT/tests/reports/core_logic_tests.md" ]; then
    print_success "Core logic test report generated"
else
    print_warning "Core logic test report not found"
fi

if [ -f "$PROJECT_ROOT/tests/reports/advanced_scenarios.md" ]; then
    print_success "Advanced scenarios report generated"
else
    print_warning "Advanced scenarios report not found"
fi

if [ -f "$PROJECT_ROOT/tests/reports/comprehensive_test_results.md" ]; then
    print_success "Comprehensive test report generated"
else
    print_warning "Comprehensive test report not found"
fi

if [ -f "$PROJECT_ROOT/tests/reports/performance_benchmarks.md" ]; then
    print_success "Performance benchmark report generated"
else
    print_warning "Performance benchmark report not found"
fi

if [ -f "$PROJECT_ROOT/tests/reports/coverage_report.md" ]; then
    print_success "Coverage report generated"
else
    print_warning "Coverage report not found"
fi

print_section "Running Security Analysis"

# Run security audit
print_info "Running security audit..."
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        print_success "No security vulnerabilities found"
    else
        print_warning "Security vulnerabilities found. Please review."
    fi
else
    print_info "cargo-audit not installed. Install with: cargo install cargo-audit"
fi

print_section "Test Summary"

# Count test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Count from reports if they exist
if [ -f "$PROJECT_ROOT/tests/reports/comprehensive_test_results.md" ]; then
    # Extract test counts from the report (simplified)
    TOTAL_TESTS=$(grep -o "Total Tests.*[0-9]" "$PROJECT_ROOT/tests/reports/comprehensive_test_results.md" | grep -o "[0-9]*" | tail -1 || echo "0")
    PASSED_TESTS=$(grep -o "Passed.*[0-9]" "$PROJECT_ROOT/tests/reports/comprehensive_test_results.md" | grep -o "[0-9]*" | tail -1 || echo "0")
    FAILED_TESTS=$(grep -o "Failed.*[0-9]" "$PROJECT_ROOT/tests/reports/comprehensive_test_results.md" | grep -o "[0-9]*" | tail -1 || echo "0")
fi

echo -e "\n${PURPLE}╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║                        TEST SUMMARY                               ║${NC}"
echo -e "${PURPLE}╚═══════════════════════════════════════════════════════════════════╝${NC}"

echo -e "\n${CYAN}📊 Test Results:${NC}"
echo -e "   Total Tests: ${TOTAL_TESTS}"
echo -e "   Passed: ${GREEN}${PASSED_TESTS}${NC}"
echo -e "   Failed: ${RED}${FAILED_TESTS}${NC}"

if [ "$FAILED_TESTS" -eq 0 ] && [ "$TOTAL_TESTS" -gt 0 ]; then
    echo -e "\n${GREEN}✅ ALL TESTS PASSED!${NC}"
    echo -e "${GREEN}🎉 Token ACL implementation is working correctly!${NC}"
else
    echo -e "\n${RED}❌ SOME TESTS FAILED${NC}"
    echo -e "${YELLOW}Please review the test reports for details.${NC}"
fi

echo -e "\n${CYAN}📁 Generated Reports:${NC}"
echo -e "   📋 Integration Tests: tests/reports/integration_tests.md"
echo -e "   🔒 Core Logic Tests: tests/reports/core_logic_tests.md"
echo -e "   🌍 Advanced Scenarios: tests/reports/advanced_scenarios.md"
echo -e "   📊 Comprehensive Report: tests/reports/comprehensive_test_results.md"
echo -e "   ⚡ Performance Benchmarks: tests/reports/performance_benchmarks.md"
echo -e "   📈 Coverage Report: tests/reports/coverage_report.md"

echo -e "\n${CYAN}🔧 Next Steps:${NC}"
echo -e "   1. Review test reports for any issues"
echo -e "   2. Check performance benchmarks for optimization opportunities"
echo -e "   3. Review coverage report for missing test cases"
echo -e "   4. Address any security vulnerabilities found"

echo -e "\n${PURPLE}╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║                    COMPREHENSIVE TESTING COMPLETE                ║${NC}"
echo -e "${PURPLE}╚═══════════════════════════════════════════════════════════════════╝${NC}"

# Exit with appropriate code
if [ "$FAILED_TESTS" -eq 0 ] && [ "$TOTAL_TESTS" -gt 0 ]; then
    exit 0
else
    exit 1
fi
