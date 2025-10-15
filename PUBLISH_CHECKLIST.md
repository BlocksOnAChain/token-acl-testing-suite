# Publishing Checklist

Use this checklist before publishing to GitHub.

## Pre-Publishing

### 1. Clean Up

- [ ] Remove unnecessary files
- [ ] Delete build artifacts
- [ ] Clear sensitive information
- [ ] Remove temporary files

```bash
# Clean Rust artifacts
find . -name "target" -type d -exec rm -rf {} +
find . -name "Cargo.lock" -exec rm -f {} +

# Clean Node artifacts
find . -name "node_modules" -type d -exec rm -rf {} +
find . -name "package-lock.json" -exec rm -f {} +

# Clean results
rm -rf results/
```

### 2. Update Documentation

- [ ] Update README.md with correct repository URL
- [ ] Update INSTALL.md with GitHub clone command
- [ ] Check all documentation links work
- [ ] Verify code examples are correct
- [ ] Update version numbers if applicable

### 3. Test Everything

- [ ] Run `./setup.sh` on clean environment
- [ ] Run `./run_tests.sh` successfully
- [ ] Run all demos: `cd demos && npm run demo:all`
- [ ] Verify on different OS (Linux, macOS)
- [ ] Check all scripts are executable

### 4. Verify Files

Required files present:
- [ ] README.md
- [ ] LICENSE
- [ ] .gitignore
- [ ] INSTALL.md
- [ ] CONTRIBUTING.md
- [ ] setup.sh
- [ ] run_tests.sh
- [ ] Cargo.toml (workspace)
- [ ] demos/package.json

### 5. Repository Settings

- [ ] Create GitHub repository
- [ ] Add appropriate topics/tags
- [ ] Enable issues
- [ ] Enable discussions
- [ ] Set up branch protection (optional)
- [ ] Add description
- [ ] Add website link (if any)

## Publishing

### 1. Initialize Git

```bash
git init
git add .
git commit -m "Initial commit: Token ACL Testing Suite"
```

### 2. Add Remote

```bash
git remote add origin https://github.com/YOUR_USERNAME/token-acl-testing-suite.git
```

### 3. Push to GitHub

```bash
git branch -M main
git push -u origin main
```

### 4. Create Release

- [ ] Tag version: `git tag v1.0.0`
- [ ] Push tags: `git push --tags`
- [ ] Create GitHub Release with changelog

## Post-Publishing

### 1. Verify

- [ ] Clone from GitHub to test
- [ ] Run setup on fresh clone
- [ ] Check all links in README work
- [ ] Verify badges display correctly

### 2. Promote

- [ ] Share on Solana Forum
- [ ] Tweet about it
- [ ] Post in relevant Discord channels
- [ ] Add to awesome-solana lists

### 3. Monitor

- [ ] Watch for issues
- [ ] Respond to discussions
- [ ] Review pull requests
- [ ] Keep documentation updated

## GitHub Repository Sections

### About

```
Comprehensive testing suite for sRFC 37: Token ACL (Efficient Block/Allow List Token Standard)

Topics: solana, blockchain, token, acl, security, testing, rust, typescript
Website: https://forum.solana.com/t/srfc-37-efficient-block-allow-list-token-standard/4036
```

### README Sections

- [x] Clear project description
- [x] Quick start guide
- [x] Installation instructions
- [x] Usage examples
- [x] Documentation links
- [x] Contributing guidelines
- [x] License information
- [x] Contact/support info

### Repository Settings

**General:**
- [x] Description added
- [x] Website added
- [x] Topics added
- [x] Issues enabled
- [x] Discussions enabled

**Code:**
- [x] .gitignore configured
- [x] License added
- [x] README present

## Quick Cleanup Script

```bash
#!/bin/bash
# Run before publishing

# Clean build artifacts
find . -name "target" -type d -exec rm -rf {} + 2>/dev/null
find . -name "node_modules" -type d -exec rm -rf {} + 2>/dev/null
find . -name "Cargo.lock" -exec rm -f {} + 2>/dev/null
find . -name "package-lock.json" -exec rm -f {} + 2>/dev/null

# Clean results
rm -rf results/

# Clean temp files
find . -name "*.tmp" -exec rm -f {} +
find . -name "*.log" -exec rm -f {} +
find . -name ".DS_Store" -exec rm -f {} +

echo "✓ Cleanup complete!"
```

## Final Check

Before pushing:
```bash
# Verify structure
tree -L 2 -I 'node_modules|target'

# Verify no sensitive data
grep -r "password\|secret\|key" . --exclude-dir={node_modules,target}

# Verify all scripts are executable
ls -l *.sh

# Verify setup works
./setup.sh && ./run_tests.sh
```

---

**Ready to publish!** 🚀

Remember:
- Keep documentation updated
- Respond to community
- Iterate based on feedback
- Have fun! 😊

