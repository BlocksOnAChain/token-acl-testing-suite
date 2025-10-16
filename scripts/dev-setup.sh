#!/bin/bash

# Token ACL Testing Suite - Development Environment Setup
# This script sets up a complete development environment for contributors

set -e

echo "🚀 **Token ACL Testing Suite - Development Setup** 🚀"
echo ""
echo "════════════════════════════════════════════════════════════════════"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

print_info "Setting up development environment for Token ACL Testing Suite..."

# 1. Check system requirements
echo ""
echo "1️⃣ **Checking System Requirements**"
echo "────────────────────────────────────"

# Check Rust installation
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    print_status "Rust installed: $RUST_VERSION"
    
    # Check if version is sufficient
    if [[ "$RUST_VERSION" < "1.70.0" ]]; then
        print_warning "Rust version $RUST_VERSION detected. Recommended: 1.70.0+"
        print_info "Run 'rustup update' to update Rust"
    fi
else
    print_error "Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Cargo installation
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version | cut -d' ' -f2)
    print_status "Cargo installed: $CARGO_VERSION"
else
    print_error "Cargo not found. Please install Rust with Cargo"
    exit 1
fi

# Check Git installation
if command -v git &> /dev/null; then
    GIT_VERSION=$(git --version | cut -d' ' -f3)
    print_status "Git installed: $GIT_VERSION"
else
    print_error "Git not found. Please install Git"
    exit 1
fi

# 2. Install development tools
echo ""
echo "2️⃣ **Installing Development Tools**"
echo "────────────────────────────────────"

# Install rust-analyzer if not present
if ! command -v rust-analyzer &> /dev/null; then
    print_info "Installing rust-analyzer..."
    cargo install rust-analyzer
    print_status "rust-analyzer installed"
else
    print_status "rust-analyzer already installed"
fi

# Install cargo-audit for security auditing
if ! command -v cargo-audit &> /dev/null; then
    print_info "Installing cargo-audit..."
    cargo install cargo-audit
    print_status "cargo-audit installed"
else
    print_status "cargo-audit already installed"
fi

# Install cargo-outdated for dependency management
if ! command -v cargo-outdated &> /dev/null; then
    print_info "Installing cargo-outdated..."
    cargo install cargo-outdated
    print_status "cargo-outdated installed"
else
    print_status "cargo-outdated already installed"
fi

# 3. Setup project dependencies
echo ""
echo "3️⃣ **Setting Up Project Dependencies**"
echo "──────────────────────────────────────"

print_info "Building workspace dependencies..."
if cargo build --workspace; then
    print_status "Dependencies built successfully"
else
    print_warning "Some dependencies failed to build (this may be due to edition2024 issues)"
    print_info "Continuing with development setup..."
fi

# 4. Setup development environment
echo ""
echo "4️⃣ **Setting Up Development Environment**"
echo "─────────────────────────────────────────"

# Create necessary directories
mkdir -p tests/reports
mkdir -p .vscode
print_status "Created development directories"

# Setup Git hooks (if .git exists)
if [ -d ".git" ]; then
    print_info "Setting up Git hooks..."
    
    # Create pre-commit hook
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook for Token ACL Testing Suite

echo "🔍 Running pre-commit checks..."

# Format check
if ! cargo fmt --all -- --check; then
    echo "❌ Code formatting check failed. Run 'cargo fmt --all' to fix."
    exit 1
fi

# Clippy check
if ! cargo clippy --workspace --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy linting failed. Fix the issues above."
    exit 1
fi

echo "✅ Pre-commit checks passed!"
EOF

    chmod +x .git/hooks/pre-commit
    print_status "Git pre-commit hook installed"
else
    print_info "Not a Git repository, skipping Git hooks"
fi

# 5. Verify development environment
echo ""
echo "5️⃣ **Verifying Development Environment**"
echo "────────────────────────────────────────"

# Test basic functionality
print_info "Testing basic functionality..."

# Check if tests can be discovered
if [ -d "tests/integration" ]; then
    print_status "Test structure verified"
else
    print_error "Test structure not found"
    exit 1
fi

# Check if scripts are executable
if [ -x "scripts/setup.sh" ] && [ -x "scripts/test.sh" ] && [ -x "scripts/build.sh" ]; then
    print_status "Scripts are executable"
else
    print_warning "Some scripts may not be executable. Run 'chmod +x scripts/*.sh' to fix."
fi

# 6. Display development information
echo ""
echo "6️⃣ **Development Environment Ready!**"
echo "─────────────────────────────────────"

print_status "Development environment setup complete!"
echo ""
echo "📋 **Available Commands:**"
echo "  make help           - Show all available commands"
echo "  make setup          - Run complete setup"
echo "  make test           - Run all tests"
echo "  make lint           - Run linting"
echo "  make format         - Format code"
echo "  make docs           - Generate documentation"
echo "  make clean          - Clean build artifacts"
echo ""
echo "📁 **Project Structure:**"
echo "  tests/integration/  - Main test suite"
echo "  scripts/            - Build and test scripts"
echo "  docs/               - Documentation"
echo "  .vscode/            - VS Code configuration"
echo ""
echo "🔧 **IDE Configuration:**"
echo "  VS Code settings configured in .vscode/"
echo "  Recommended extensions listed in .vscode/extensions.json"
echo "  Debug configurations available in .vscode/launch.json"
echo ""
echo "📚 **Documentation:**"
echo "  README.md           - Project overview"
echo "  docs/getting-started.md - Quick start guide"
echo "  docs/api-reference.md - API documentation"
echo "  CHANGELOG.md        - Version history"
echo ""
echo "🎯 **Next Steps:**"
echo "  1. Run 'make test' to verify everything works"
echo "  2. Read docs/getting-started.md for development workflow"
echo "  3. Check CONTRIBUTING.md for contribution guidelines"
echo ""
echo "════════════════════════════════════════════════════════════════════"
echo ""
print_status "Happy coding! 🚀"
