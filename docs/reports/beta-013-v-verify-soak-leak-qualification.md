# BETA-013-V — Verify Run Beta Soak and Leak Qualification

## Overview

Verification closure for parent task **BETA-013** ("Run beta soak and leak qualification"). Every parent acceptance guardrail and subsystem requirement was mapped, tested, and verified against empirical evidence.

## Acceptance Criteria & Guardrails Matrix

| Guardrail / Requirement | Verified Evidence | Status |
|---|---|---|
| **No monotonic unbounded growth** | Multi-worker soak data (`soak-summary.json`, 2.43M requests) shows QuickJS heap delta is flat (~0 KiB net drift: +4.7 KiB on Worker 0, +624 B on Worker 1); process RSS drift is bounded allocator retention (~0.298 B/req). | PASS |
| **All resource gauges return near baseline after quiescence** | Invocations: 2,431,643 registered, 2,431,643 settled; pending slots at shutdown = 0; live native tasks = 0; pending native ops = 0. | PASS |
| **No boundary violations** | Peak live slots strictly capped at 2,048 (bounded by queue capacity); zero scheduler boundary violations across 14 live worker poisonings and thousands of timeout/disconnect events. | PASS |
| **Any bounded cache growth is documented** | Glibc `ptmalloc` arena fragmentation and verified bytecode cache boundaries documented in `docs/reports/beta-013-d-retained-growth-analysis.md`. | PASS |
| **Subsystem coverage (Fetch, DB, Auth, Timeouts, Cancellation, Worker Replacement, Reload)** | Confirmed via `q-engine-quickjs`, `q-capabilities`, `q-http`, `q-capability-postgres`, and `@velqu/capability-auth-jwt` conformance suites. | PASS |

## Targeted Commands & Gates

- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `bun run typecheck` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Verification closure only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
