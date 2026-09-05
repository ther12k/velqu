# BWASM-K-001 — Portable Runtime Model Crate

## Result

**PASS** — `q-runtime-model` is now the host-independent model layer for
native and Browser-WASM paths. `q-engine` keeps only the native engine
surface and re-exports the model types unchanged, preserving existing
native identifiers.

## Moved-type map

| Original path | New portable path | Reason |
|---|---|---|
| `q_engine::{RouteId, HandlerId, PolicyId, SchemaId}` | `q_runtime_model::{...}` | cross-target numeric identities |
| `q_engine::{FunctionKind, FunctionDecl, FieldNeeds}` | `q_runtime_model::{...}` | manifest/route model values |
| `q_engine::{ParamSpec, RequestMeta, DispatchRoute}` | `q_runtime_model::{...}` | request and resolved-route data |
| `q_engine::{ResponseStrategy, BodyOut, ProblemOut, FieldErrorOut}` | `q_runtime_model::{...}` | response/problem values |
| `q_engine::{Outcome, SourceLocation, OriginalLocation}` | `q_runtime_model::{...}` | invocation outcomes and diagnostics |
| `q_engine::{LoadStats, EngineLoadPlan, EngineStats}` | `q_runtime_model::{...}` | engine-adapter result/plan/stat values |
| `q_engine::NO_REQUEST_SLOT` | `q_runtime_model::NO_REQUEST_SLOT` | request-handle sentinel |
| new `MODEL_ABI_VERSION = 1` | `q_runtime_model::MODEL_ABI_VERSION` | explicit portable serialization/ABI identity |
| `q_engine::InvocationSpec` | **stays in `q-engine`** | carries host `std::time::Instant`, native request/job fields |
| `q_engine::Engine` | **stays in `q-engine`** | `tokio::sync::oneshot`, native worker lifecycle |

`q-engine` re-exports all moved names, so existing consumers continue to
compile through `q_engine::RouteId`, `q_engine::Outcome`, and so on. No
router, pack, bridge, runtime, or QuickJS source path required identifier
changes.

## Before/after dependency graph

Before: `q-engine` owned model values and also depended on `tokio`.
`q-router` and `q-pack` reached the Tokio/Mio edge through q-engine.

After:

```text
q-runtime-model (portable)
  ├── serde
  ├── serde_json
  └── bytes

q-engine (native adapter)
  ├── q-runtime-model
  ├── serde / serde_json / bytes
  └── tokio  (Engine trait + native invocation only)

q-router / q-pack / q-bridge  ── existing q-engine API (re-exports)
```

The portable crate's normal dependency tree contains no Tokio, Hyper,
`rquickjs`, `memmap2`, filesystem/process/socket dependency, or native
Postgres capability. `cargo tree -p q-runtime-model -e normal` was retained
in the handoff transcript and shows only `bytes`, `serde`, and
`serde_json` (plus their serialization-only transitive dependencies).

## Serialization fixtures and hashes

`crates/q-runtime-model/tests/fixtures.rs` pins:

- `MODEL_ABI_VERSION == 1`;
- exact JSON bytes for `FunctionDecl`;
- SHA-256 over `MODEL_ABI_VERSION.to_be_bytes()` followed by those bytes:
  `9dacb30d967a41ade33554fe8d8c57ffd6a03afe9e026b1721cb50aac6871d1c`;
- exact JSON key order and round-trip for `FieldNeeds`;
- `BodyOut::Json` shape preservation.

The fixture fails if the wire shape, enum spelling, key ordering, or ABI
version changes; a future intentional change must bump the model ABI and
update the fixture/hash in the same review.

## Exact commands and results

```text
cargo check -p q-runtime-model                         PASS
cargo check -p q-engine                                 PASS
cargo check --target wasm32-unknown-unknown -p q-runtime-model PASS
cargo tree -p q-runtime-model -e normal                 PASS; no forbidden native deps
cargo test -p q-runtime-model                            4 passed / 0 failed
cargo test -p q-engine -p q-router -p q-pack -p q-bridge  affected suites pass
./scripts/validate-okf                                  PASS
./scripts/verify                                       ALL PASS
```

The affected regression rerun recorded: q-bridge 11 tests, q-pack 100,
q-router 15, q-engine 0 unit tests — all pass. The initial background
attempt that returned 101 was a concurrent build-lock/partial-run event;
it was rerun to completion with exit 0 before this report was written.

Full-battery history (all runs in `unshare -rn` netns, setup completed
first): the first `./scripts/verify` failed `bun test` with 7 failures
while cargo compiled in parallel — those tests pass standalone (434/0)
and in every later run, so that run is recorded as load-induced
flakiness, not waived. The second run failed only
`validate-benchmark-evidence` (`manifest: hash mismatch for
qRuntimeRelease`) — the documented post-rebuild class: adding a crate
changes the release binary hash pinned by `benchmarks/manifest.json`.
Resolution per procedure: `python3 scripts/refresh-benchmark-manifest.py`
(manifest delta: 3 lines) committed as its own commit, then verify
re-run: **ALL PASS**.

## Acceptance disposition

- [x] Portable model crate compiles natively and for wasm32.
- [x] Normal dependency tree excludes Tokio, Hyper, rquickjs, memmap2,
  filesystem/process/socket, and native Postgres.
- [x] Existing native behavior and identifiers do not drift; q-engine
  re-exports all moved types and affected regression suites pass.
- [x] Round-trip fixtures are deterministic and versioned.
- [x] Moved-type map, before/after graph, fixture hash, and exact results
  are recorded.

## Scope and follow-ups

No browser APIs, engine traits, or TypeScript contracts were added. The
next K-phase cuts are K-002 (byte-based QPack core), K-003 (host-independent
router core), K-004 (on-target schema qualification), K-005 (browser kernel
ABI), and K-006 (portable-kernel evidence).

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
