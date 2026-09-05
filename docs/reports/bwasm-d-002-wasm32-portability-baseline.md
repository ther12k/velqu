# BWASM-D-002 — wasm32 Portability Baseline and Dependency Split Map

## Overview

Measures the current `wasm32-unknown-unknown` compatibility of every
runtime-relevant crate with retained compiler output, classifies each as
`portable` / `split-required` / `native-only` / `browser-only`, and
freezes the smallest dependency cuts the K-phase must perform.

## Method

- Toolchain: rustc 1.96.0 (repository lockfile), `rust-std` for
  `wasm32-unknown-unknown` added for this measurement.
- Command: `cargo check --target wasm32-unknown-unknown -p <crate>` —
  one run per crate, exit code captured directly, **exact compiler
  output retained** (no inference): raw logs in
  `docs/codex-spark-browser-wasm/evidence/wasm32/check-<crate>.log`,
  machine-readable inventory in
  `docs/codex-spark-browser-wasm/evidence/wasm32-baseline.json`.
- Transitive blocker attribution via `cargo tree -i <blocked-crate>
  --target wasm32-unknown-unknown -e normal` traces.

## Results

| Crate | wasm32 check | Classification | Blocker (exact) |
|---|---|---|---|
| `q-schema-runtime` | **PASS** (exit 0) | portable | — |
| `q-router` | FAIL (101) | split-required | `mio` via `q-engine → tokio` |
| `q-engine` | FAIL (101) | split-required | direct `tokio → mio` |
| `q-bridge` | FAIL (101) | split-required | `mio` via `q-engine` |
| `q-pack` | FAIL (101) | split-required | `mio` via q-engine **and** `getrandom` via q-capabilities |
| `q-capabilities` | FAIL (101) | split-required | direct `getrandom` (needs `js` feature on this target) |
| `q-http` | FAIL (101) | native-only | tokio/hyper ingress stack |
| `q-engine-quickjs` | FAIL (101) | native-only | rquickjs native C engine (+ tokio edge) |
| `velqu-runtime` | FAIL (101) | native-only | tokio/hyper-rustls host stack |

Exact retained error text (representative):

```text
error: This wasm target is unsupported by mio. If using Tokio, disable the net feature.
error[E0432]: unresolved import `crate::sys::IoSourceState`
error: the wasm*-unknown-unknown targets are not supported by default, you may need to enable the "js" feature.
```

## Key findings

1. **`q-schema-runtime` already compiles clean on wasm32** — the
   validation core is portable as-is (K-004's remaining work is
   qualifying its test suite on the target, not fixing compilation).
2. **One blocker class dominates**: `mio` (via tokio) — reached only
   through `q-engine`'s direct tokio dependency. Removing that edge from
   the portable surfaces (model types + router core) unblocks
   `q-router`, `q-bridge`, and `q-pack`'s mio leg simultaneously.
3. **`q-pack`'s native mmap (`memmap2`) is not the observed blocker**
   (it produced no error in these runs), but the split (K-002) is still
   frozen as architecture: the byte-based verification core must not
   depend on native mmap loading or on `q-engine`, so the browser kernel
   verifies packs from bytes.
4. **`getrandom` in `q-capabilities`** is the second blocker class; the
   smallest cut is feature-gating it as native-only — browser randomness
   comes from the host bridge (browser crypto), never ambient entropy.
5. `q-http`, `q-engine-quickjs`, `velqu-runtime` are **native-only** by
   architecture (ADR-0037 §6): the browser gets a new composition
   (`q-browser-kernel` + `@velqu/browser-runtime`), not a port.

## Dependency split map (smallest cuts, K-phase order)

1. **K-001** — extract host-independent runtime model types (route
   plan, decisions, bounds) from `q-engine` into a new portable crate;
   `q-engine` keeps tokio and its scheduler-facing surface.
2. **K-002** — split the byte-based QPack verification core from native
   loading (`memmap2` confined to the native loader); feature-gate
   `getrandom` in `q-capabilities` as native-only.
3. **K-003** — cut the `q-router → q-engine` edge; the router core
   consumes the portable model crate only.
4. **K-004** — qualify `q-schema-runtime` on-target (test suite), no
   source cut needed.
5. **Re-measure** `q-bridge` and `q-pack` on wasm32 after K-001..K-003
   (expected green without their native edges) — recorded as K-002/K-006
   acceptance evidence.

## Commands run

```bash
rustup target add wasm32-unknown-unknown
cargo check --target wasm32-unknown-unknown -p {q-schema-runtime,q-router,q-engine,q-pack,q-http,q-engine-quickjs,q-bridge,q-capabilities,velqu-runtime}
cargo tree -i mio --target wasm32-unknown-unknown -e normal
cargo tree -p {q-router,q-schema-runtime,q-pack,q-engine,q-bridge,q-capabilities} -e normal --depth 1
```

## Disclosures

- Measurement/analysis only; no source or build behavior changed.
- Check runs are `cargo check` (compilation compatibility), not on-target
  test execution; on-target test qualification is K-004's deliverable.
- Standing CI disclosure applies (zero-step verify workflows since
  ~#714); local gates are the acceptance basis.
