# BETA-013-Z — Package Evidence: Run Beta Soak and Leak Qualification

## Overview

Evidence packaging and parent task closure for **BETA-013** ("Run beta soak and leak qualification").
With this packet, all child microtasks (BETA-013-A through BETA-013-D) and verification (BETA-013-V) are complete and passing. Parent row `BETA-013` is flipped to **PASS** in `docs/beta/04_TASK_LEDGER.md`.

## Child Packet Evidence Inventory

| Task ID | Component | Deliverable | Evidence Report | PR |
|---|---|---|---|---|
| BETA-013-A | Soak Execution | Multi-worker continuous soak (>2.4M requests, flat heap, negative/sub-byte RSS drift) | `docs/reports/beta-013-a-soak-qualification.md` | #1191 |
| BETA-013-B | Subsystems | Coverage of outbound fetch, Postgres DB, JWT auth, timeouts, cancellation, worker replacement, and reload | `docs/reports/beta-013-b-soak-coverage.md` | #1192 |
| BETA-013-C | Resource Tracking | Rigorous metrics tracking for RSS, JS heap, task slots, queues, pools, and error classes | `docs/reports/beta-013-c-resource-tracking.md` | #1193 |
| BETA-013-D | Retained Growth | Asymptotic saturation analysis, memory graphs, and quiescence verification | `docs/reports/beta-013-d-retained-growth-analysis.md` | #1194 |
| BETA-013-V | Verification | Verification of all guardrails, recovery, and subsystem invariants | `docs/reports/beta-013-v-verify-soak-leak-qualification.md` | #1195 |

## Parent Acceptance Guardrails Verified

- **No monotonic unbounded growth**: QuickJS per-worker heap delta is flat (~0 KiB net drift across 2.43M requests and 14 worker rebuilds). Process RSS growth is sub-byte allocator retention (~0.298 B/req), saturating into an asymptote.
- **All resource gauges return near baseline after quiescence**: 0 pending slots at shutdown, 0 live native tasks, 0 pending native ops, all active handles cleanly settled.
- **No boundary violations**: Peak live slots strictly capped at 2,048 (bounded by queue capacity). 0 scheduler boundary violations.
- **Any bounded cache growth is documented**: Glibc allocator retention and bytecode caches are documented with exact empirical figures.

## Gates

- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `bun run typecheck` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Evidence packaging and status tracking only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
