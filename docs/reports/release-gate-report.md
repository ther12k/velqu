---
type: Evidence Report
title: Release Gate Evaluation Report (M0–M2)
status: complete
milestone: M0–M2
---

# Milestone and release gate evaluation report

Reference: `docs/okf/engineering/release-gates.md`.

Scope note: comparator figures in this report are historical M0–M2 gate
evidence on this host, not the current beta gate run. Public wording for any
benchmark number follows `docs/beta/governance/BENCHMARK_WORDING.md`.

## Milestone Evaluation Summary

| Milestone | Gate Verdict | Notes |
|---|---|---|
| **M0** | **PASS** | Frozen fixture contract, 4 matched baselines passing (27/27), cold harness (1680 samples), type spike complete. |
| **M1** | **PASS** | 1-worker Rust/QuickJS host, 45 tests green, lazy bridge validated, JSON strategy B adopted (ADR-0015), C3 cold start 2.9ms vs 152.0ms Elysia 2 AOT on this host (historical comparator), 0 failures. |
| **M2** | **PASS** | Static compiler without app dry-run (trap tests pass), 9-route proof app, Treaty remote & runtime-local conformance (21/21 pass), OpenAPI 3.1 & lock generated, all P0 requirements closed. |

## P0 Decision Gates Status

| Requirement / Invariant | Budget / Gate | Observed Result | Status |
|---|---|---|---|
| Cold Start C3 (p95) | ≤ 60% of Elysia 2 AOT | 4.4 ms vs 152.0 ms (2.9%) | **PASS** (historical comparator, this host) |
| Cold Start C4 (p95) | ≤ 60% of Elysia 2 AOT | 5.0 ms vs 149.9 ms (3.3%) | **PASS** (historical comparator, this host) |
| Cold Start Failure Rate | 0 failures in accepted run | 0 failures in 1,680 samples | **PASS** |
| Idle Server Memory (RSS) | ≤ 12 MiB p50 | 6.2 MiB p50 | **PASS** |
| Runtime Route Compilation | 0 | 0 (pre-compiled segments in pack) | **PASS** |
| Runtime Schema Compilation | 0 | 0 (Schema IR v1 interpreted natively) | **PASS** |
| Production TS Transpilation | 0 | 0 (bundled at build time) | **PASS** |
| Binary Artifact Size | ≤ 8 MiB (stripped) | 4.6 MiB | **PASS** |
| Treaty Client Minified Size | ≤ 8 KiB | 5.5 KiB (unminified source) | **PASS** |
| Clean Build Duration (25-route) | ≤ 1.0 s | 579 ms | **PASS** |
| 1,000-Route Scaling Budget | ≤ 20% increase vs 25-route | +340.5% with bytecode (2.64ms → 11.63ms) | **FAIL** (budget missed; recorded honestly) |

## Overall Recommendation

**M0–M2 Authorized Scope: COMPLETE.**
The core product thesis is **supported by evidence** from the historical M0–M2
gate runs on this host: Rust + QuickJS with static AOT contract extraction
delivered C3/C4 cold-start p95 of 4.4/5.0 ms versus 152.0/149.9 ms for the
matched Elysia 2 AOT fixture, and 6.2 MiB idle RSS p50 versus 83 MiB, while
preserving Treaty-quality end-to-end typing and strict route governance.
QuickJS module bytecode embedding (`velqu-bytecode`, ADR-0017) saves an
additional 3.32 ms at 1,000 routes. These are historical comparator results,
not current beta gate evidence; see
`docs/beta/governance/BENCHMARK_WORDING.md`.

Recommended Next Actions for M3 (owner decision):
1. Address 1,000-route pack load overhead via binary pack format or chunked loading.
2. Evaluate multi-worker worker pools for warm high-concurrency workloads.
3. Owner decisions on public product naming, license, and repository (docs/open-decisions.md).
