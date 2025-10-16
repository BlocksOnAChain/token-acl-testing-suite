#!/bin/bash

# Token ACL Testing Suite - Setup Script
# This script installs all dependencies and sets up the environment

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║   sRFC 37: Token ACL Testing Suite - Setup                       ║"
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

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check prerequisites
print_status "Checking prerequisites..."

# Check Rust
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    print_success "Rust found: $RUST_VERSION"
else
    print_error "Rust not found!"
    echo "Please install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check Cargo
if command -v cargo &> /dev/null; then
    print_success "Cargo found"
else
    print_error "Cargo not found!"
    exit 1
fi

# Check Node.js
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    print_success "Node.js found: $NODE_VERSION"
else
    print_error "Node.js not found!"
    echo "Please install Node.js: https://nodejs.org/"
    exit 1
fi

# Check npm
if command -v npm &> /dev/null; then
    NPM_VERSION=$(npm --version)
    print_success "npm found: $NPM_VERSION"
else
    print_error "npm not found!"
    exit 1
fi

echo ""
print_status "Building all programs..."

# Build integration tests
echo ""
print_status "Building integration tests..."
cd tests/integration
if cargo build --release; then
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
if cargo build --release; then
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
if cargo build --release; then
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
if cargo build --release; then
    print_success "Production allow list program built successfully"
else
    print_error "Production allow list program build failed"
    exit 1
fi
cd ../..

# Create reports directory
mkdir -p tests/reports

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║                    SETUP COMPLETE! ✓                              ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

print_success "All dependencies verified"
print_success "All programs built"
print_success "Environment ready"

echo ""
echo "═══ NEXT STEPS ═══"
echo ""
echo "Run tests:"
echo "  ${GREEN}./scripts/test.sh${NC}"
echo ""
echo "Build only:"
echo "  ${GREEN}./scripts/build.sh${NC}"
echo ""
echo "Clean build artifacts:"
echo "  ${GREEN}./scripts/clean.sh${NC}"
echo ""
echo "Read documentation:"
echo "  ${GREEN}cat docs/README.md${NC}"
echo "  ${GREEN}cat docs/validation-report.md${NC}"
echo ""
echo "🚀 You're ready to explore Token ACL!"
echo ""
