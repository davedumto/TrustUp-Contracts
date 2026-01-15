# AI Context - TrustUp Contracts

> **For AI Assistants**: This file provides quick context for working on TrustUp-Contracts

## Quick Overview

TrustUp is a decentralized BNPL (Buy Now, Pay Later) platform on Stellar blockchain. Users pay 20% guarantee, get 80% credit from liquidity pool. On-chain reputation adjusts based on repayment behavior.

## Essential Documentation

Read these before making changes:

1. **[docs/PROJECT_CONTEXT.md](docs/PROJECT_CONTEXT.md)** (385 lines)
   - Complete project vision, BNPL mechanics, economic model
   - Reputation system (0-100 scoring)
   - Liquidity provider economics
   - Smart contract architecture

2. **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)** (203 lines)
   - Technology stack (Stellar/Soroban/Rust)
   - Module organization pattern (7 modules per contract)
   - Storage, authorization, event, error patterns
   - Build configuration and commands

3. **[docs/ROADMAP.md](docs/ROADMAP.md)** (1133 lines)
   - 7 phases, 20 issues total
   - 8 completed (40%), 12 pending
   - Detailed specs for each issue
   - Current focus: Phase 3 (CreditLine)

4. **[docs/CONTRIBUTING.md](docs/CONTRIBUTING.md)** (213 lines)
   - Setup instructions
   - Branch naming: `feat/SC-XX-desc`, `fix/desc`, `docs/desc`
   - Commit standards: Conventional Commits
   - Code patterns and testing requirements

5. **[docs/ERROR_CODES.md](docs/ERROR_CODES.md)** (113 lines)
   - Error handling patterns
   - Reputation contract: 5 errors defined
   - Planned errors for other contracts

6. **[docs/FILE_ORGANIZATION.md](docs/FILE_ORGANIZATION.md)** (345 lines)
   - Repository structure
   - Module pattern (lib, types, errors, storage, access, events, tests)
   - Build artifacts location
   - Naming conventions

## Critical Standards

### Branch Naming
```
feat/SC-XX-description   # New features (e.g., feat/SC-08-loan-creation)
fix/description          # Bug fixes
docs/description         # Documentation
test/description         # Tests
refactor/description     # Refactoring
ci/description          # CI/CD
```

### Commit Messages (Conventional Commits)
```
feat: implement loan creation (SC-08)
fix: prevent score overflow in increase_score
docs: update roadmap with completed issues
test: add boundary tests for reputation
```

### Module Pattern
Every contract MUST have these 7 modules:
```
src/
├── lib.rs        # Contract entry, #[contract], #[contractimpl]
├── types.rs      # Constants (MAX_SCORE, etc.)
├── errors.rs     # #[contracterror] enum
├── storage.rs    # get_*(), set_*() functions
├── access.rs     # require_admin(), require_updater()
├── events.rs     # emit_*() event helpers
└── tests.rs      # Test suite
```

### Error Handling
```rust
// Define errors
#[contracterror]
#[repr(u32)]
pub enum MyError {
    ErrorName = 1,  // Sequential from 1
}

// Safe arithmetic
let result = value.checked_add(amount).ok_or(Error::Overflow)?;

// Authorization FIRST, then state changes
pub fn function(env: Env, admin: Address) {
    require_admin(&env, &admin);  // ← Check auth first
    // ... state changes
}
```

### Testing Requirements
- >90% coverage goal
- Test all public functions
- Test error cases (`#[should_panic(expected = "ErrorName")]`)
- Test authorization
- Test boundary conditions

## Quick Commands

```bash
# Check compilation
cargo check

# Format code (REQUIRED before commit)
cargo fmt

# Lint (REQUIRED before commit)
cargo clippy -- -D warnings

# Run tests
cargo test
cargo test -p reputation-contract

# Build WASM for deployment
cargo build -p <contract> --target wasm32-unknown-unknown --release
# Output: target/wasm32-unknown-unknown/release/<contract>.wasm
```

## Current State (2026-01-15)

**Completed (Phase 1 & 2)**:
- ✅ Reputation Contract fully implemented
- ✅ Admin and updater access control
- ✅ Score operations (get, increase, decrease)
- ✅ Events (SCORECHGD, UPDCHGD, ADMINCHGD)
- ✅ 10 comprehensive tests
- ✅ CI/CD pipeline

**Next Phase (Phase 3 - CreditLine)**:
- ⏳ SC-08: Loan creation
- ⏳ SC-09: Loan repayment
- ⏳ SC-10: Loan default

**Contracts**:
```
contracts/
├── reputation-contract/     ✅ Implemented (Phase 2)
├── creditline-contract/     ⏳ Next (Phase 3)
├── merchant-registry/       ⏳ Planned (Phase 5)
└── liquidity-pool/          ⏳ Planned (Phase 6)
```

## Key Principles

1. **Authorization First**: Always check `require_admin()` or `require_updater()` BEFORE state changes
2. **Safe Arithmetic**: Use `checked_add/sub/mul/div`, never plain `+/-/*//`
3. **Event Emission**: Emit events for ALL state changes
4. **Error Codes**: Sequential from 1, descriptive names
5. **Testing**: Comprehensive coverage, including error paths
6. **Module Organization**: Follow the 7-module pattern strictly

## Common Patterns

### Storage Pattern
```rust
pub enum DataKey {
    Admin,                // Singleton
    Updater(Address),     // Map: Address → bool
    Score(Address),       // Map: Address → u32
}

// Read
env.storage().instance().get(&key).unwrap_or(default)

// Write
env.storage().instance().set(&key, &value)
```

### Event Pattern
```rust
env.events().publish(
    (symbol_short!("TOPIC"), indexed_address),  // Topics (searchable)
    (data1, data2)                               // Data
);
```

### Test Pattern
```rust
#[test]
fn test_name() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MyContract, ());
    let client = MyContractClient::new(&env, &contract_id);

    let addr = Address::generate(&env);

    client.function(&addr);
    assert_eq!(client.get_value(&addr), expected);
}
```

## When in Doubt

1. Check existing code: `contracts/reputation-contract/` is the reference implementation
2. Read the docs (especially PROJECT_CONTEXT and ARCHITECTURE)
3. Follow the patterns established in reputation contract
4. Ask questions in PR comments

## Build & Test Before PR

```bash
cargo fmt              # Format
cargo clippy           # Lint
cargo test             # Test
cargo build --release  # Build
cargo build -p <contract> --target wasm32-unknown-unknown --release  # WASM
```

All must pass before submitting PR.

---

**Last Updated**: 2026-01-15
**Current Phase**: Phase 3 (CreditLine Contract)
**Progress**: 8/20 issues (40%)
