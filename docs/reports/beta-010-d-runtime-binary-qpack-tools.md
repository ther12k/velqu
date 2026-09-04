# BETA-010-D — Runtime Binary and QPack Tools

## Overview

Audited, inventoried, and verified the complete toolchain for production runtime binaries and QPack artifacts:

1. `velqu-runtime` (`target/release/velqu-runtime`):
   - Production host supporting shared mode (`--pack <app.qpack>`).
   - Supports `--fingerprint` and `--fingerprint --pack <path>` to verify pack compatibility, bytecode targets, and runtime ABI without serving.
   - Built with path remapping (`RUSTFLAGS="--remap-path-prefix=$(pwd)=/velqu-src"`).

2. `velqu-standalone` (`target/release/velqu-standalone`):
   - Standalone binary mode embedding verified QPack bytecode via `VELQU_STANDALONE_PACK`.
   - Fixed missing `context_profile` in `RunConfig` initialization (`crates/q-runtime/src/bin/velqu-standalone.rs`).
   - Verified via `scripts/artifact-smoke.sh`: answers identical `/health/live` and `/hello/smoke` routes, reports `mode=standalone`, and exits cleanly.

3. `velqu-bytecode` (`target/release/velqu-bytecode`):
   - Standalone CLI tool (`q-bytecode-tool`) to compile and embed QuickJS module bytecode into QPacks (`velqu-bytecode embed --pack <pack> --out <out>`).
   - Verified: compiles bundle + prelude into module bytecode, calculates SHA-256 integrity, embeds `bundle_bytecode` metadata, and writes valid target-tagged QPack.

4. `velqu pack inspect` / `velqu pack migrate`:
   - Developer CLI (`@velqu/cli`) tools running under Bun.
   - `velqu pack inspect <app.qpack>`: outputs `appId`, `formatVersion`, `contractHash`, engine tuple, route counts, schema counts, capabilities, and bundle SHA-256.
   - `velqu pack migrate <app.qpack>`: checks format version, reports legacy compatibility status, and guides reproducible rebuilds.

## Evidence

- `scripts/artifact-smoke.sh`: verifies release runtime execution, mismatched-engine fail-closed rejection, 10 cold-start samples (p50 ~9.95ms), and standalone mode embedded pack serving. Result: `SMOKE-OK`.
- `scripts/qpack-tools-inventory.sh`: automated check across all 5 runtime/tool surfaces. Emits `docs/reports/beta-010-d-qpack-tools-inventory.json` with verdict `PASS`.
- `docs/reports/beta-010-d-qpack-tools-inventory.json`.

## Gates

- `cargo test -p q-pack` — pass
- `cargo test -p q-engine-quickjs` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Standalone mode requires compile-time pack path via `VELQU_STANDALONE_PACK`.
- Bytecode packs are target-architecture bound (x86_64 glibc little-endian); cross-target packs fall back to source evaluation or require `no_bytecode` / rebuild.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
