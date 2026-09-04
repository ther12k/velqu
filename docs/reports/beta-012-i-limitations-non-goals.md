# BETA-012-I — Limitations and Non-Goals

## Overview

Updated `docs/beta/LIMITS-AND-NON-GOALS.md` to reflect the public beta (`0.1.0-beta.1`) scope and invariants:
- Replaced outdated "private alpha" terminology with "public beta (`0.1.0-beta.1`)".
- Added an explicit explanation of QuickJS bytecode vs JIT compilation in the runtime limits section and terminology table (“QuickJS bytecode” vs “native JIT compilation”).
- Verified the bounded execution model (queues, bodies, heap, stack, deadlines, fetch, defer).
- Verified non-goals: no full Node/Bun compatibility, no ORM in core, no WebSockets/SSE, no server-side rendering, no native direct public TLS termination (reverse-proxy first), non-SLA.
- Checked links to `docs/beta/02_SCOPE_MATRIX.md` and `docs/beta/governance/POST_BETA_BACKLOG.md`.

## Testing & Gates

- Links verified.
- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Documentation only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
