# Getting Started with Token ACL Testing Suite

This guide will help you set up and run the sRFC 37 Token ACL testing suite in minutes.

## Prerequisites

### Required Software

1. **Rust & Cargo** (1.70.0 or later)
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Node.js & npm** (v18 or later)
   ```bash
   # On macOS with Homebrew
   brew install node
   
   # On Ubuntu/Debian
   sudo apt install nodejs npm
   
   # Verify installation
   node --version
   npm --version
   ```

3. **Git**
   ```bash
   # On macOS
   brew install git
   
   # On Ubuntu/Debian
   sudo apt install git
   
   # Verify installation
   git --version
   ```

## Quick Start (2 minutes)

### 1. Clone and Setup

```bash
# Clone the repository
git clone https://github.com/BlocksOnAChain/token-acl-testing-suite.git
cd token-acl-testing-suite

# Run setup (installs all dependencies and builds everything)
./scripts/setup.sh
```

This will:
- ✅ Check prerequisites
- ✅ Install Rust dependencies
- ✅ Build all test programs
- ✅ Verify installation

### 2. Run Tests

```bash
# Run all tests
./scripts/test.sh
```

**Output**: You'll see 16 tests running with 81 assertions.

### 3. View Results

```bash
# View test results
cat tests/reports/integration_tests.md
cat tests/reports/core_logic_tests.md
cat tests/reports/advanced_scenarios.md
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

### Advanced Scenarios (5 tests, 25 assertions)
- ✅ KYC allowlist with expiration
- ✅ Sanctions list precedence
- ✅ Geo-blocking by jurisdiction
- ✅ Freeze/thaw with revocation
- ✅ Multi-step RWA workflow

## Project Structure

```
token-acl-testing-suite/
├── scripts/                      # Build and utility scripts
│   ├── setup.sh                 # One-command setup
│   ├── test.sh                  # Run all tests
│   ├── build.sh                 # Build all programs
│   └── clean.sh                 # Clean build artifacts
├── tests/                        # All test code
│   ├── integration/             # Integration test suite
│   ├── fixtures/                # Test data/helpers
│   └── reports/                 # Generated test reports
├── examples/                     # Example gate programs
│   ├── allow_list/              # Reference allow list
│   └── block_list/              # Reference block list
├── programs/                     # Production gate programs
│   └── production_allow_list/   # Production-ready allow list
└── docs/                         # Documentation
    ├── getting-started.md       # This file
    ├── testing-guide.md         # How tests work
    ├── architecture.md          # System architecture
    └── validation-report.md     # Comprehensive validation
```

## Manual Installation

If you prefer to install manually or the setup script fails:

### Step 1: Build Test Suite

```bash
# Build the integration tests
cd tests/integration
cargo build --release

# Build example gate programs
cd ../../examples/allow_list
cargo build --release

cd ../block_list
cargo build --release

# Build production gate program
cd ../../programs/production_allow_list
cargo build --release

cd ../..
```

### Step 2: Verify Installation

```bash
# Run a quick test
cd tests/integration
cargo test --lib integration_tests::test_pda_derivation_correctness
```

## Running Tests

### All Tests (Recommended)

```bash
# From project root
./scripts/test.sh
```

This runs:
- ✅ All 16 comprehensive tests
- ✅ Integration tests
- ✅ Core logic tests
- ✅ Advanced scenarios
- ✅ Generates test reports

### Individual Test Categories

```bash
cd tests/integration

# Run specific test category
cargo test --lib integration_tests
cargo test --lib core_logic_tests
cargo test --lib advanced_scenarios

# Run all tests
cargo test
```

## Troubleshooting

### Rust Build Fails

**Error**: `error: could not compile...`

**Solutions**:
```bash
# Update Rust
rustup update

# Clear build cache
cd tests/integration
cargo clean
cargo build --release
```

### Permission Denied

**Error**: `Permission denied`

**Solution**:
```bash
chmod +x scripts/setup.sh
chmod +x scripts/test.sh
chmod +x scripts/build.sh
chmod +x scripts/clean.sh
```

### Path with Spaces/Colons

**Error**: `path segment contains separator`

**Solution**: Move the project to a path without spaces or special characters:
```bash
mv "sRFC 37: Efficient Block/Allow List Token Standard" token-acl-testing-suite
cd token-acl-testing-suite
```

## System Requirements

### Minimum
- **OS**: Linux, macOS, or Windows (WSL2)
- **RAM**: 4GB
- **Disk**: 2GB free space
- **CPU**: Any modern CPU

### Recommended
- **OS**: macOS or Linux
- **RAM**: 8GB+
- **Disk**: 5GB free space
- **CPU**: Multi-core processor

## Getting Help

### Common Issues

1. **Rust version too old**: `rustup update`
2. **Node version too old**: Use Node 18+
3. **Build fails**: Check error message, usually missing dependency
4. **Tests fail**: Check the error output for specific issues

### Support

- **Issues**: https://github.com/BlocksOnAChain/token-acl-testing-suite/issues
- **Discussions**: https://github.com/BlocksOnAChain/token-acl-testing-suite/discussions
- **sRFC 37 Forum**: https://forum.solana.com/t/srfc-37-efficient-block-allow-list-token-standard/4036

## Next Steps

After installation:

1. **Read the documentation**: Start with `docs/validation-report.md`
2. **Explore the tests**: `cd tests/integration && cargo test`
3. **Read the validation**: `cat docs/validation-report.md`
4. **Check out examples**: `ls examples/` and `ls programs/`

## Uninstall

```bash
# Clean build artifacts
./scripts/clean.sh

# Remove the entire directory
cd ..
rm -rf token-acl-testing-suite
```

---

**Ready to explore Token ACL? Let's go!** 🚀
