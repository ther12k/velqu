# BWASM-K-003 — Host-Independent Router Core

## Result

**PASS** — `q-router` is now a host-independent route core: its
dependency graph is `q-runtime-model` + `q-pack` (portable core, no
native features) + `thiserror`, it compiles unchanged for
`wasm32-unknown-unknown` with **default features**, and its frozen
matching semantics are specified in-crate with a pinning test named for
every rule. Native behavior is untouched (identifiers preserved via the
K-001 re-export identity).

## Changes

- `crates/q-router/Cargo.toml`: `q-engine` dependency **removed**
  (replaced by `q-runtime-model` — the only symbols used were the ID
  types); `q-pack` consumed as a direct path dependency with
  `default-features = false` (portable byte core only; same
  workspace-inheritance workaround as K-002, documented in-place).
- `crates/q-router/src/lib.rs`: 46 `q_engine::` references →
  `q_runtime_model::` (same types through the K-001 re-export — zero
  drift); new **frozen matching-semantics specification** doc section
  naming the pinning test for every rule the task lists: base path,
  percent decoding (none — raw bytes by policy), query exclusion,
  trailing slash, malformed URL bytes (panic-free corpus), ambiguity /
  duplicate rejection at build, method semantics (HEAD→GET, 405 with
  Allow), and precedence (static > param > wildcard, no cross-method
  shadowing).

## Verification evidence

```text
cargo check -p q-router                                             PASS
cargo check --target wasm32-unknown-unknown -p q-router             PASS (default features)
cargo tree --target wasm32-unknown-unknown -p q-router -e normal
    → 0 matches for tokio/hyper/rquickjs/memmap2/ed25519/getrandom/
    postgres/q-engine
cargo test -p q-router                                              15/15 PASS
cargo check --workspace                                             0 errors
cargo fmt --all --check / cargo clippy --workspace -D warnings      PASS
./scripts/validate-okf                                              PASS
./scripts/verify                                                    ALL PASS (two-pass manifest refresh)
```

### Shared fixtures / baseline equivalence

The router's test module **is** the shared fixture set: it contains the
property-equivalence test (compiled vs reference router), the raw-bytes
encoding corpus, collision/ambiguity rejection, method-mapping and
precedence cases. The same crate compiles for wasm32, so native and
browser builds consume one matcher implementation and one fixture set —
there is no second implementation to drift. On-target *execution* of
the fixtures in a browser harness is K-005/K-006 work (no wasm-bindgen
harness exists yet); recorded as the honest boundary, same as K-002.

## Acceptance disposition

- [x] Router core compiles for wasm32 without host runtime dependencies
  (dep audit: zero host crates in the wasm32 graph).
- [x] Native and browser tests consume the same route fixtures (single
  crate, single test module; semantics spec names each pinning test).
- [x] Precedence and decoded params match the native baseline (by
  construction — one implementation; native suites pin the behavior).

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
