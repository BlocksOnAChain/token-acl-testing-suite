#!/bin/bash

# Token ACL Testing Suite - Build Script
# Builds all programs in the workspace

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
RELEASE=false
VERBOSE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -r|--release)
            RELEASE=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -r, --release    Build in release mode"
            echo "  -v, --verbose    Show verbose output"
            echo "  -h, --help       Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                # Build in debug mode"
            echo "  $0 --release      # Build in release mode"
            echo "  $0 --verbose      # Build with verbose output"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║   sRFC 37: Token ACL Testing Suite - Build                       ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Function to print status
print_status() {
    echo -e "${BLUE}►${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Set build mode
if [ "$RELEASE" = true ]; then
    BUILD_MODE="--release"
    MODE_TEXT="release"
else
    BUILD_MODE=""
    MODE_TEXT="debug"
fi

# Set verbosity
if [ "$VERBOSE" = true ]; then
    CARGO_FLAGS=""
else
    CARGO_FLAGS="-q"
fi

print_status "Building all programs in $MODE_TEXT mode..."

# Build integration tests
echo ""
print_status "Building integration tests..."
cd tests/integration
if cargo build $BUILD_MODE $CARGO_FLAGS; then
    print_success "Integration tests built successfully"
else
    print_error "Integration tests build failed"
    exit 1
fi
cd ../..

# Build example allow list gate program
echo ""
print_status "Building example allow list gate program..."
cd examples/allow_list
if cargo build $BUILD_MODE $CARGO_FLAGS; then
    print_success "Example allow list program built successfully"
else
    print_error "Example allow list program build failed"
    exit 1
fi
cd ../..

# Build example block list gate program
echo ""
print_status "Building example block list gate program..."
cd examples/block_list
if cargo build $BUILD_MODE $CARGO_FLAGS; then
    print_success "Example block list program built successfully"
else
    print_error "Example block list program build failed"
    exit 1
fi
cd ../..

# Build production allow list gate program
echo ""
print_status "Building production allow list gate program..."
cd programs/production_allow_list
if cargo build $BUILD_MODE $CARGO_FLAGS; then
    print_success "Production allow list program built successfully"
else
    print_error "Production allow list program build failed"
    exit 1
fi
cd ../..

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║                    BUILD COMPLETE! ✓                             ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

print_success "All programs built successfully in $MODE_TEXT mode"
print_success "Ready for testing and deployment"

echo ""
echo "═══ NEXT STEPS ═══"
echo ""
echo "Run tests:"
echo "  ${GREEN}./scripts/test.sh${NC}"
echo ""
echo "Clean build artifacts:"
echo "  ${GREEN}./scripts/clean.sh${NC}"
echo ""
