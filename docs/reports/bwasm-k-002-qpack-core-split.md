# BWASM-K-002 — Byte-based QPack Core Split from Native Loading and Tooling

## Result

**PASS** — `q-pack` now builds in two configurations from one source of
truth: the full native crate (default features, unchanged behavior) and
a browser/portable configuration (`--no-default-features`) that carries
the byte-based parsing/verification core with **no filesystem, mmap,
signing, OS-entropy, tokio, or native-engine dependency**. The split is
a feature boundary inside `q-pack` — the sanctioned "native crate **or
feature**" option in the task text — chosen because it leaves the
7,000-line verification core physically untouched (zero moved code, zero
drift risk) while excluding every native concern from the wasm32 graph.

## What is native-only (`feature = "native"`, default on)

| Surface | Reason |
|---|---|
| `PackBytes::open` + `Mapped` variant | filesystem/mmap loading (D-002: `memmap2` native coupling) |
| `QPack::load_and_verify` / `load_and_verify_with` | fs loaders; portable callers use `verify_from_slice` |
| `legacy_v1::read_and_verify` (path variant) | fs; `read_and_verify_bytes` stays portable |
| `signatures` module (Ed25519 detached signatures, `TrustConfig`, `TrustSource` file/env) | out-of-band authenticity tooling (ADR-0026); browser kernel is **integrity-only by contract** (ADR-0037 §4) |
| sidecar `sidecar_path_for` / `load_and_verify` | fs tooling; `pack_sha256_of`/`verify_against` stay portable |
| `SharedAcrossWorkers` impl for `SharedPack` | native multi-worker sharing (ADR-0036); no browser equivalent |
| q-capabilities `getrandom` entropy | browser randomness comes from the host bridge (ADR-0037 §5); `CryptoRandom` fails closed with an actionable error when built non-native |

## Dependency changes

- `q-pack` **no longer depends on `q-engine`** (tokio): its only use,
  `FunctionDecl`/`FunctionKind`, now comes from `q-runtime-model`
  (K-001). The re-export `pub use q_runtime_model::{…}` preserves the
  public identifier.
- `memmap2` and `ed25519-dalek` are optional, enabled only by `native`.
- `q-capabilities` gains a `native` feature (`dep:getrandom`); q-pack
  consumes it via a **direct path dependency with
  `default-features = false`** — a workspace-inherited
  `default-features = false` was ignored by cargo in this graph
  (observed: `cargo metadata` reported `uses_default_features: true`
  despite the member manifest), and the direct declaration is honored.
  The capability *inventory* remains a normal dependency on every
  target because it is part of pack verification (capability
  authorization is compatibility-critical per ADR-0037 §1).

## Verification evidence

Commands and exact outcomes (this worktree):

```text
cargo check -p q-pack                                            PASS
cargo check --workspace                                          PASS (0 errors)
cargo check --target wasm32-unknown-unknown -p q-pack --no-default-features  PASS
cargo check -p q-capabilities --no-default-features              PASS
cargo tree --target wasm32-unknown-unknown -p q-pack --no-default-features -e normal
    → contains NO tokio / hyper / rquickjs / memmap2 / ed25519-dalek /
      getrandom / postgres (0 matches)
cargo test -p q-pack -p q-capabilities                            ALL PASS
    q-pack 100 + 2 fuzz; q-capabilities 268+6+7+1+3+4+9
cargo fmt --all --check                                           PASS
cargo clippy --workspace --all-targets -- -D warnings             PASS
./scripts/validate-okf                                            PASS
./scripts/verify                                                  ALL PASS (post-setup; the documented
                                                                   two-pass manifest refresh — the first
                                                                   pass mismatched because verify's own
                                                                   rebuild re-hashes qRuntimeRelease)
```

### Format compatibility

Native behavior is pinned by the existing suites — all 102 q-pack tests
(golden v1 fixture, QPack v2 directory/bounds/digest tests, cross-target
bytecode, tamper corpus, trust/signature tests) run against the same
source with `native` on; nothing moved between files. Native and WASM
produce equivalent results for valid/invalid fixtures **by construction**
— one parser, one verification core, two build configurations of the
same code; the wasm32 build compiles that exact code today. On-target
*execution* tests (wasm-bindgen-test in a browser/node harness) are
K-005/K-006 territory — the ABI and harness do not exist yet; this
packet records that boundary honestly instead of claiming executed wasm
evidence.

### Parser bounds / fuzz

The existing fail-closed corpus continues to gate the core:
`fuzz_pack.rs` (random bytes never panic; mutated valid packs always
detected), `qpack2` mutation/bounds tests (truncated, swapped,
oversized, overflowing directory values rejected without panic), and
the legacy mixed-mode rejection tests. All pass in this worktree.

### Browser-facing API audit

Mechanical audit (committed in the transcript): every `pub fn` touching
`std::path`/`std::fs`/`PathBuf` in q-pack is behind
`#[cfg(feature = "native")]` — `sidecar_path_for`, `PackBytes::open`,
`QPack::load_and_verify`(+`_with`), `legacy_v1::read_and_verify` — so
the `--no-default-features` build exposes **no filesystem path
anywhere** in its public API, and the dependency tree proves the same
at the crate level.

## Acceptance disposition

- [x] Portable pack core compiles for wasm32 without native loader
  imports (`cargo check --target wasm32-unknown-unknown
  -p q-pack --no-default-features`).
- [x] Native and WASM equivalence for valid/invalid fixtures — single
  shared parser (no duplicated core to diverge); native suites green;
  on-target execution deferred to K-005/K-006 with the bindgen harness
  (recorded, not silently claimed).
- [x] Malformed/truncated/swapped/oversized inputs never panic or
  allocate unbounded — existing fuzz/property/bounds suites pass.
- [x] No filesystem path reachable from the browser-facing API — cfg
  construction + mechanical audit + dependency tree.

## Known limitations

- The browser kernel does not yet exist; this packet makes the core
  *buildable* for wasm32 and auditable, K-005 composes it.
- Ed25519 signature verification is absent from the browser build by
  contract (integrity-only, ADR-0037 §4); artifacts needing
  authenticity verification must be verified out-of-band before
  deployment, exactly as in native mode.

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
