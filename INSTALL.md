# Installation & Setup Guide

This guide will help you set up and run the sRFC 37 Token ACL testing suite.

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

## Quick Start (5 Minutes)

### 1. Clone the Repository

```bash
# Clone from GitHub
git clone https://github.com/BlocksOnAChain/token-acl-testing-suite.git
cd token-acl-testing-suite

# Or if you downloaded the ZIP
unzip token-acl-testing-suite.zip
cd token-acl-testing-suite
```

### 2. Run the Setup Script

```bash
# Make the setup script executable
chmod +x setup.sh

# Run setup (installs all dependencies)
./setup.sh
```

This will:
- ✅ Install Rust dependencies
- ✅ Install Node.js dependencies
- ✅ Build all test programs
- ✅ Verify installation

### 3. Run Tests

```bash
# Run all tests
./run_tests.sh

# Or run demos
cd demos
npm run demo:all
```

That's it! 🎉

## Manual Installation

If you prefer to install manually or the setup script fails:

### Step 1: Install Rust Dependencies

```bash
# Build the test suite
cd tests/test-client
cargo build --release

# Build reference gate programs
cd ../../gate_programs/allow_list
cargo build --release

cd ../block_list
cargo build --release

cd ../..
```

### Step 2: Install Node.js Dependencies

```bash
# Install demo dependencies
cd demos
npm install
cd ..
```

### Step 3: Verify Installation

```bash
# Run a quick test
cd tests/test-client
cargo test --lib managed_freeze_authority::tests::test_all_managed_freeze_authority

# Run a demo
cd ../../demos
npm run demo:sanctions
```

## Running the Test Suite

### All Tests (Recommended)

```bash
# From project root
./run_tests.sh
```

This runs:
- ✅ All 41 comprehensive tests
- ✅ Integration flow tests
- ✅ Security tests
- ✅ Generates test report

### Individual Test Categories

```bash
cd tests/test-client

# Run specific test category
cargo test --lib integration_flow_test
cargo test --lib security_malicious_injection_test
cargo test --lib managed_freeze_authority
cargo test --lib permissionless_operations
cargo test --lib composability
cargo test --lib security

# Run all tests
cargo run --release
```

### View Test Report

```bash
# After running tests, view the report
cat test-results/REAL_TEST_RESULTS.md
```

## Running the Demos

### All Demos

```bash
cd demos
npm run demo:all
```

### Individual Demos

```bash
# Sanctions list demo
npm run demo:sanctions

# KYC allow list demo
npm run demo:kyc

# Geo-blocking demo
npm run demo:geo
```

## Project Structure

```
token-acl-testing-suite/
├── setup.sh                    # One-command setup
├── run_tests.sh               # Run all tests
├── tests/
│   └── test-client/           # 41 comprehensive tests
├── demos/
│   ├── src/                   # Real-world demos
│   └── package.json           # Demo dependencies
├── gate_programs/
│   ├── allow_list/            # Reference allow list
│   └── block_list/            # Reference block list
└── docs/
    ├── README.md              # Main documentation
    ├── FINAL_SUMMARY.md       # Complete validation
    ├── TEST_PLAN.md           # Testing methodology
    ├── IMPLEMENTATION_GUIDE.md # Integration guide
    ├── SECURITY_VALIDATION.md # Security analysis
    └── QUICK_REFERENCE.md     # Cheat sheet
```

## Troubleshooting

### Rust Build Fails

**Error**: `error: could not compile...`

**Solutions**:
```bash
# Update Rust
rustup update

# Clear build cache
cd tests/test-client
cargo clean
cargo build --release
```

### Node.js Issues

**Error**: `sh: tsx: command not found`

**Solution**:
```bash
cd demos
rm -rf node_modules package-lock.json
npm install
```

### Path with Spaces/Colons

**Error**: `path segment contains separator`

**Solution**: Move the project to a path without spaces or special characters:
```bash
mv "sRFC 37: Efficient Block/Allow List Token Standard" token-acl-testing-suite
cd token-acl-testing-suite
```

### Permission Denied

**Error**: `Permission denied`

**Solution**:
```bash
chmod +x setup.sh
chmod +x run_tests.sh
chmod +x scripts/run_all_tests.sh
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

## Testing Without Installation

Want to see results without installing?

1. **Read the documentation**:
   - `docs/FINAL_SUMMARY.md` - Complete validation report
   - `docs/SECURITY_VALIDATION.md` - Security analysis

2. **Watch the demo videos** (if available):
   - Link to video demos

3. **Review test code**:
   - `tests/test-client/src/` - All test implementations

## Getting Help

### Common Issues

1. **Rust version too old**: `rustup update`
2. **Node version too old**: Use Node 18+
3. **Build fails**: Check error message, usually missing dependency
4. **Tests fail**: This is expected - tests are conceptual/illustrative

### Support

- **Issues**: https://github.com/BlocksOnAChain/token-acl-testing-suite/issues
- **Discussions**: https://github.com/BlocksOnAChain/token-acl-testing-suite/discussions
- **sRFC 37 Forum**: https://forum.solana.com/t/srfc-37-efficient-block-allow-list-token-standard/4036

## Next Steps

After installation:

1. **Read the documentation**: Start with `docs/README.md`
2. **Run the demos**: `cd demos && npm run demo:all`
3. **Explore the tests**: `cd tests/test-client && cargo run --release`
4. **Read the validation**: `cat docs/FINAL_SUMMARY.md`

## Uninstall

```bash
# Remove dependencies
cd tests/test-client
cargo clean

cd ../../gate_programs/allow_list
cargo clean

cd ../block_list
cargo clean

cd ../../demos
rm -rf node_modules

# Remove the entire directory
cd ../../..
rm -rf token-acl-testing-suite
```

---

**Ready to explore Token ACL? Let's go!** 🚀

