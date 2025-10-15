#!/bin/bash

# Cleanup Script - Remove build artifacts and temporary files
# Run this before committing to GitHub

echo "🧹 Cleaning up build artifacts and temporary files..."

# Clean Rust build artifacts
echo "  Cleaning Rust targets..."
find . -name "target" -type d -exec rm -rf {} + 2>/dev/null

# Clean Cargo.lock files (workspace will regenerate)
echo "  Cleaning Cargo.lock files..."
find . -name "Cargo.lock" -exec rm -f {} + 2>/dev/null

# Clean Node modules
echo "  Cleaning node_modules..."
find . -name "node_modules" -type d -exec rm -rf {} + 2>/dev/null

# Clean package-lock.json
echo "  Cleaning package-lock.json..."
find . -name "package-lock.json" -exec rm -f {} + 2>/dev/null

# Clean results directory
echo "  Cleaning results..."
rm -rf results/

# Clean temporary files
echo "  Cleaning temporary files..."
find . -name "*.tmp" -exec rm -f {} + 2>/dev/null
find . -name "*.temp" -exec rm -f {} + 2>/dev/null
find . -name "*.log" -exec rm -f {} + 2>/dev/null
find . -name ".DS_Store" -exec rm -f {} + 2>/dev/null

# Clean cache
echo "  Cleaning cache..."
find . -name ".cache" -type d -exec rm -rf {} + 2>/dev/null

echo ""
echo "✅ Cleanup complete!"
echo ""
echo "Removed:"
echo "  ✓ Rust build artifacts (target/)"
echo "  ✓ Node.js modules (node_modules/)"
echo "  ✓ Lock files (Cargo.lock, package-lock.json)"
echo "  ✓ Test results (results/)"
echo "  ✓ Temporary files (*.tmp, *.log, .DS_Store)"
echo ""
echo "Your repository is clean and ready for GitHub! 🚀"
echo ""

