# Command Cheat Sheet

## Repository

```bash
git status --short
git diff --check
git rev-parse HEAD
```

## Rust

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p q-pack
cargo test -p q-router
cargo test -p q-engine-quickjs
cargo test -p q-http
cargo test -p q-bridge
cargo test -p q-schema-runtime
cargo test -p q-capabilities
cargo test -p velqu-runtime
```

## TypeScript

```bash
bun test
bun run typecheck
```

## Project gates

```bash
./scripts/verify
./scripts/validate-okf
./scripts/validate-production-plan
bun run benchmark:all
```

Run only the targeted command during implementation. Run full verification in verification/evidence/gate packets.
