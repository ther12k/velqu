# BETA-012-H — Performance Methodology

## Overview

Added `docs/beta/PERFORMANCE-METHODOLOGY.md` (indexed in `docs/beta/INDEX.md`), defining the normative benchmarking standards, measurement invariants, and QuickJS bytecode vs JIT compilation trade-offs for the public beta release (`0.1.0-beta.1`).

## Core Invariants & Guardrails

1. **No Universal Performance Claims**: Explicitly states that Velqu makes no claim of universal superiority over Node, Bun, Elysia, or other runtimes. Performance is strictly workload-, fixture-, and hardware-dependent.
2. **Bytecode vs. JIT Compilation**: Clarifies that ahead-of-time bytecode compilation into `app.qpack` eliminates cold-start parse/transpile delays and ensures deterministic verification, but is not native JIT machine-code compilation. Compute-heavy CPU-bound loops in JIT engines (V8, JavaScriptCore) will outpace QuickJS in raw throughput.
3. **Reproducibility & Distribution Reporting**: All measured claims require matched candidates, identical datasets/seeds, and full statistical distributions ($n$, mean, p50, p95, p99) under `benchmarks/raw/` with manifest checksums in `benchmarks/manifest.json`.
4. **No Production-Ready / SLA Wording**: Clarifies beta status (`0.1.0-beta.1`) with non-SLA terms and trusted-application-only execution.

## Testing & Verification

- Tested validation commands: `python3 scripts/validate-benchmark-evidence.py` (PASS), `./scripts/validate-okf` (PASS).
- Link check: verified all internal markdown links resolve cleanly (`01_BETA_DEFINITION.md`, `REAL_WORLD_BENCHMARKS.md`, `INDEX.md`, `PERFORMANCE-METHODOLOGY.md`).
- Gates:
  - `cargo test -p velqu-runtime` — pass (8 suites ok)
  - `bun test` — 434 pass / 0 fail (67 files)
  - `bun run typecheck` — pass
  - `cargo fmt --all --check` — pass
  - `cargo clippy --workspace --all-targets -- -D warnings` — pass
  - `./scripts/validate-okf` — pass
  - `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Documentation and methodology only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
