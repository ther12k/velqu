# BETA-014-Z — Package Evidence: Publish Canonical Beta Benchmark Report

## Overview

Evidence packaging and parent task closure for **BETA-014** ("Publish canonical beta benchmark report"). All child packets (A–D) plus the V verification packet are PASS with source-backed, re-runnable evidence. Parent row `BETA-014` is flipped to **PASS** in `docs/beta/04_TASK_LEDGER.md`.

## Packet inventory (parent BETA-014)

| Packet | Deliverable | Canonical evidence | PR |
|---|---|---|---|
| BETA-014-A | Canonical benchmark report (cold-start C0–C5, warm, real-world subsystems, crossover, losses, limitations) | `docs/reports/beta-014-a-canonical-benchmark-report.md` | #1197 |
| BETA-014-B | Pin all candidates/artifacts | `docs/reports/beta-014-b-pin-candidates-artifacts.md`; `benchmarks/real-world/versions.json` + 9 passing pin tests | #1198 |
| BETA-014-C | Retain raw data | `docs/reports/beta-014-c-retain-raw-data.md`; `benchmarks/real-world/retain.ts` + 5 passing retention tests | #1199 |
| BETA-014-D | Wording review + corrections | `docs/reports/beta-014-d-wording-review.md` (report corrected to current raw evidence) | #1200 |
| BETA-014-V | Verification closure | `docs/reports/beta-014-v-verify-canonical-benchmark-report.md` | #1201 |

## Acceptance guardrails → evidence

1. **Every number links to raw evidence** — all tables in the canonical report derive from `benchmarks/raw/{cold-start,warm,ramp,worker-scaling}` with `validate-benchmark-evidence.py` reporting zero manifest errors.
2. **Fixture-specific wording** — report and methodology doc state fixture scope, host, pins, repetitions, and randomized candidate order.
3. **Velqu losses are included** — honest-loss section (corrected in BETA-014-D): C0 steady p50 2.29× the class best; no Velqu overtake of raw-rust within the recorded 100-request C0/C2 horizons.
4. **No cloud cold-start claim from local process data** — explicit guardrail note in the report's cold-start section.

## Gates (this packet)

- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Evidence packaging and status tracking only; no runtime behavior modified.
- The report makes no universal performance, cloud cold-start, production-readiness, SLA, or cost claim.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
