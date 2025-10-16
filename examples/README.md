# Example Gate Programs

This directory contains example implementations of gating programs that follow the sRFC 37 Token ACL interface.

## Available Examples

### Allow List (`allow_list/`)
A reference implementation of an allow list gating program that:
- Implements `can_thaw_permissionless` interface
- Manages user allow list records
- Supports permissionless thaw operations
- Demonstrates basic allow list functionality

**Use Cases:**
- KYC verification
- Whitelist management
- User onboarding

### Block List (`block_list/`)
A reference implementation of a block list gating program that:
- Implements `can_freeze_permissionless` interface
- Manages user block list records
- Supports permissionless freeze operations
- Demonstrates sanctions/compliance functionality

**Use Cases:**
- Sanctions enforcement
- Compliance management
- Risk assessment

## Building Examples

```bash
# Build all examples
cargo build --release

# Build specific example
cd examples/allow_list
cargo build --release
```

## Integration

These examples can be used as:
1. **Learning reference** - Study the sRFC 37 interface implementation
2. **Starting point** - Customize for your specific needs
3. **Testing** - Use in test environments
4. **Documentation** - Understand the interface requirements

## Interface Requirements

All gating programs must implement the sRFC 37 interface:

### Required Discriminators
- Thaw: `[8, 175, 169, 129, 137, 74, 61, 241]`
- Freeze: `[214, 141, 109, 75, 248, 1, 45, 29]`

### Account Permissions
- Gating programs receive **read-only** accounts
- Cannot modify user balances
- Cannot execute unauthorized instructions
- Can only return success/failure decisions

### PDA Requirements
- Must derive PDAs correctly
- Must populate extra account metas
- Must validate account relationships

## Customization

To create your own gating program:

1. **Copy an example** that matches your use case
2. **Implement your logic** in the interface functions
3. **Add your data structures** for user records
4. **Test thoroughly** with the integration test suite
5. **Deploy** to your target network

## Security Considerations

- Always validate account relationships
- Never trust external inputs
- Use proper PDA derivation
- Implement proper access controls
- Test edge cases thoroughly

## Resources

- [sRFC 37 Specification](https://forum.solana.com/t/srfc-37-efficient-block-allow-list-token-standard/4036)
- [Token ACL Implementation](https://github.com/solana-foundation/token-acl)
- [Architecture Guide](../docs/architecture.md)
- [Testing Guide](../docs/testing-guide.md)
