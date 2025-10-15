# Contributing to Token ACL Testing Suite

Thank you for your interest in contributing! This document provides guidelines for contributing to this project.

## Ways to Contribute

### 1. Testing & Validation

- Run the test suite on different platforms
- Report bugs or test failures
- Suggest additional test scenarios
- Validate security mechanisms

### 2. Documentation

- Improve existing documentation
- Fix typos and clarify explanations
- Add examples and use cases
- Translate documentation

### 3. Code Contributions

- Add new test cases
- Implement additional gate programs
- Improve helper functions
- Optimize performance

### 4. Integration Examples

- Create integration examples for wallets
- Build protocol integration guides
- Develop tooling and dashboards
- Share real-world implementations

## Getting Started

### Setup Development Environment

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/token-acl-testing-suite.git
cd token-acl-testing-suite

# Run setup
./setup.sh

# Verify installation
./run_tests.sh
```

### Project Structure

```
token-acl-testing-suite/
├── tests/
│   └── test-client/           # Main test suite
│       └── src/
│           ├── integration_flow_test.rs
│           ├── security_malicious_injection_test.rs
│           ├── managed_freeze_authority.rs
│           ├── permissionless_operations.rs
│           ├── composability.rs
│           ├── security.rs
│           └── gate_program_interface.rs
├── demos/
│   └── src/                   # Real-world demos
├── gate_programs/
│   ├── allow_list/            # Reference implementations
│   └── block_list/
└── docs/                      # Documentation
```

## Contribution Process

### 1. Create an Issue

Before starting work:
- Check existing issues
- Create a new issue describing your proposal
- Discuss approach with maintainers

### 2. Fork & Branch

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/token-acl-testing-suite.git

# Create a feature branch
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Follow existing code style
- Add tests for new functionality
- Update documentation
- Keep commits focused and clear

### 4. Test Your Changes

```bash
# Run tests
./run_tests.sh

# Run demos
cd demos && npm run demo:all

# Check formatting (Rust)
cd tests/test-client
cargo fmt --check
cargo clippy

# Check formatting (TypeScript)
cd ../../demos
npm run lint
```

### 5. Commit & Push

```bash
# Stage changes
git add .

# Commit with clear message
git commit -m "Add: Brief description of change"

# Push to your fork
git push origin feature/your-feature-name
```

### 6. Create Pull Request

- Go to GitHub and create a Pull Request
- Describe your changes clearly
- Reference related issues
- Wait for review

## Code Style

### Rust

Follow Rust standard style:
```bash
# Format code
cargo fmt

# Check for common issues
cargo clippy
```

Key conventions:
- Use descriptive variable names
- Add comments for complex logic
- Keep functions focused
- Write comprehensive tests

### TypeScript

Follow TypeScript/JavaScript standard style:
```bash
# Format code
npm run lint
```

Key conventions:
- Use TypeScript types
- Prefer const over let
- Use async/await for promises
- Add JSDoc comments

### Documentation

- Use clear, simple language
- Include code examples
- Add diagrams where helpful
- Keep formatting consistent

## Testing Guidelines

### Writing Tests

Good test characteristics:
- **Clear**: Test name describes what's being tested
- **Focused**: Tests one thing at a time
- **Independent**: Doesn't depend on other tests
- **Comprehensive**: Covers edge cases

Example:
```rust
pub fn test_specific_feature() -> TestResult {
    let test_name = "Specific Feature Test";
    
    // Arrange - set up test data
    let user = Keypair::new();
    
    // Act - perform action
    let result = some_function(user);
    
    // Assert - verify outcome
    if result.is_ok() {
        TestResult::success(test_name, "Feature works correctly")
    } else {
        TestResult::failure(test_name, "Feature failed")
    }
}
```

### Test Categories

When adding tests, place them in appropriate category:
- `integration_flow_test.rs` - End-to-end workflows
- `security_malicious_injection_test.rs` - Security validations
- `managed_freeze_authority.rs` - Authority management
- `permissionless_operations.rs` - Core UX features
- `composability.rs` - Protocol compatibility
- `security.rs` - General security
- `gate_program_interface.rs` - Interface compliance

## Documentation Standards

### File Headers

```rust
/// Module Name - Brief Description
/// 
/// Detailed description of what this module does and why it exists.
/// 
/// Key features:
/// - Feature 1
/// - Feature 2
```

### Function Documentation

```rust
/// Brief description of function
/// 
/// More detailed explanation if needed.
/// 
/// # Arguments
/// * `param1` - Description
/// * `param2` - Description
/// 
/// # Returns
/// Description of return value
/// 
/// # Example
/// ```
/// let result = function_name(arg1, arg2);
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // Implementation
}
```

## Pull Request Guidelines

### PR Title

Format: `[Type] Brief description`

Types:
- `[Feature]` - New functionality
- `[Fix]` - Bug fix
- `[Docs]` - Documentation only
- `[Test]` - Test additions/changes
- `[Refactor]` - Code refactoring
- `[Style]` - Formatting changes

Examples:
- `[Feature] Add support for tiered access control`
- `[Fix] Correct permission de-escalation test`
- `[Docs] Improve security validation guide`

### PR Description

Include:
1. **What**: What does this PR do?
2. **Why**: Why is this change needed?
3. **How**: How does it work?
4. **Testing**: How was it tested?
5. **Related**: Link to related issues

Template:
```markdown
## What
Brief description of changes

## Why
Explanation of why this is needed

## How
Technical details of implementation

## Testing
- [ ] All existing tests pass
- [ ] New tests added
- [ ] Manual testing performed

## Related
Closes #123
```

## Review Process

### For Contributors

- Be responsive to feedback
- Make requested changes promptly
- Ask questions if unclear
- Be patient during review

### For Reviewers

- Be respectful and constructive
- Explain reasoning for changes
- Approve when satisfied
- Merge or request additional changes

## Code of Conduct

### Our Pledge

We pledge to make participation in this project a harassment-free experience for everyone.

### Our Standards

**Positive behaviors:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Accepting constructive criticism
- Focusing on what's best for the community

**Unacceptable behaviors:**
- Harassment or discriminatory language
- Personal attacks
- Publishing others' private information
- Other unprofessional conduct

### Enforcement

Violations may result in:
1. Warning
2. Temporary ban
3. Permanent ban

Report issues to: [MAINTAINER_EMAIL]

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Given credit for their work

## Questions?

- **General**: Open a GitHub Discussion
- **Bugs**: Create an Issue
- **Security**: Email [SECURITY_EMAIL]
- **sRFC 37**: Post on Solana Forum

## Thank You!

Every contribution helps make Token ACL better. We appreciate your time and effort! 🙏

---

**Happy Contributing!** 🚀

