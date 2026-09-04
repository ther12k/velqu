# BETA-012-C — Architecture

## Overview

Added `docs/beta/ARCHITECTURE.md`, the structural reference for the public beta, and indexed it in `docs/beta/INDEX.md`. The doc makes the runtime's shape and boundaries explicit: three artifacts (schema contract, QPack, runtime binary), the six-step request path (with the engine boundary marked), the engine lockdown and bounds, the failure model, capabilities, configuration/deployment posture, and what the pack is and is not.

## Accuracy verification (claims checked against source)

- `NO_DYNAMIC_CODE_LOCKDOWN` exists as the pre-eval lockdown — `crates/q-engine-quickjs/src/prelude.rs:896`.
- `UNTRUSTED_INGRESS_HEADERS` (7 names, TCP-peer-only identity) — `crates/q-http/src/lib.rs:59`.
- RFC 9457-compatible problems — `crates/q-runtime/src/problems.rs`.
- Bounded defaults (heap 32 MiB, stack 512 KiB, deadline 5 s) — as documented and tested in `docs/beta/INSTALL.md` and prior packets.
- Exactly one QuickJS worker; production startup performs no compilation/transpilation — enforced by gate tests (`scripts/verify` conformance).
- Exact runtime-fingerprint match (pack ↔ runtime build) — tested via SEC-001 engine-mismatch behavior.

## Guardrail compliance

- **Example execution** — proof service built and run in this worktree: `/health/live` = `{"status":"ok"}` on port 8085.
- **Link check** — `INSTALL.md`, `01_BETA_DEFINITION.md`, `LIMITS-AND-NON-GOALS.md` all exist; doc indexed from `docs/beta/INDEX.md`.
- **Docs CI / gates** — battery below.
- **No universal performance claim** — explicit statement that the doc describes structure, not performance; measured claims require matched raw evidence.
- **No production-ready/SLA wording** — non-SLA beta statement with links to the beta definition and limits docs.
- **Bytecode vs JIT accurate** — "bytecode improves startup and enables strict verification; it is not native-machine-code JIT compilation."

## Gates

- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Documentation change only; no runtime behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
