# BETA-014-V — Verify Publish Canonical Beta Benchmark Report

## Overview

Verification closure for parent task **BETA-014** ("Publish canonical beta benchmark report"). Every parent acceptance criterion was mapped to its implementation packet, source, and re-confirmed evidence.

## Acceptance Criteria Matrix

| Parent guardrail | Implementation | Verification evidence | Result |
|---|---|---|---|
| **Every number links to raw evidence** | BETA-014-A report + BETA-014-D wording review | All report tables re-derived from `benchmarks/raw/{cold-start,warm,ramp,worker-scaling}/*.json`; stale numbers removed in BETA-014-D (#1200); `python3 scripts/validate-benchmark-evidence.py` — PASS (all manifest hashes match) | PASS |
| **Fixture-specific wording** | BETA-014-A/D | Report states fixture scope (C0–C3 warm cells, single host, Bun 1.4.0, 5 repetitions, randomized candidate order); `docs/beta/PERFORMANCE-METHODOLOGY.md` invariant #1 | PASS |
| **Velqu losses are included** | BETA-014-D | Honest-loss section corrected to current `ramp-1788451334621` artifact: C0 steady p50 2.29× class best; no overtake of raw-rust within 100-request C0/C2 horizons | PASS |
| **No cloud cold-start claim from local process data** | BETA-014-A | Guardrail note in §1 explicitly forbids extrapolating local `fork/exec` measurements into cloud cold-start promises | PASS |
| **Candidates/artifacts pinned** | BETA-014-B | `benchmarks/real-world/versions.test.ts` — 9 pass / 0 fail; exact pins in `versions.json` + frozen `bun.lock` + `compose.yaml` | PASS |
| **Raw data retained** | BETA-014-C | `benchmarks/real-world/retain.test.ts` — 5 pass / 0 fail (deterministic gzip, lossless round-trip, hash verification, byte-identical rebuild) | PASS |
| **Wording reviewed** | BETA-014-D | Methodology review report `docs/reports/beta-014-d-wording-review.md`; corrections applied and disclosed in PR #1200 | PASS |

## Methodology review confirmation

- `docs/beta/PERFORMANCE-METHODOLOGY.md` documents bytecode-vs-JIT accuracy, distribution reporting requirements ($n$, mean, p50, p95, p99), and the no-universal-claims invariant.
- The canonical report (`docs/reports/beta-014-a-canonical-benchmark-report.md`) as corrected by BETA-014-D conforms: every table cites its raw source file, and unsupported claims (idle/peak RSS, Node/Fastify warm rows, cost normalization) were removed rather than left unreferenced.

## Gates

- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `python3 scripts/validate-benchmark-evidence.py` — PASS (no manifest errors)
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Verification closure only; no runtime behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
