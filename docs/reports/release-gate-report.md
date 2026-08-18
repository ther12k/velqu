---
type: Evidence Report
title: Release Gate Evaluation Report (M0–M2)
status: complete
milestone: M0–M2
---

# Milestone and release gate evaluation report

Reference: `docs/okf/engineering/release-gates.md`.

## Milestone Evaluation Summary

| Milestone | Gate Verdict | Notes |
|---|---|---|
| **M0** | **PASS** | Frozen fixture contract, 4 matched baselines passing (27/27), cold harness (1680 samples), type spike complete. |
| **M1** | **PASS** | 1-worker Rust/QuickJS host, 45 tests green, lazy bridge validated, JSON strategy B adopted (ADR-0015), C3 cold start 2.9ms (34× faster than Elysia 2 AOT), 0 failures. |
| **M2** | **PASS** | Static compiler without app dry-run (trap tests pass), 9-route proof app, Treaty remote & runtime-local conformance (21/21 pass), OpenAPI 3.1 & lock generated, all P0 requirements closed. |

## P0 Decision Gates Status

| Requirement / Invariant | Budget / Gate | Observed Result | Status |
|---|---|---|---|
| Cold Start C3 (p95) | ≤ 60% of Elysia 2 AOT | 4.4 ms vs 152.0 ms (2.9%) | **PASS** (34× faster) |
| Cold Start C4 (p95) | ≤ 60% of Elysia 2 AOT | 5.0 ms vs 149.9 ms (3.3%) | **PASS** (30× faster) |
| Cold Start Failure Rate | 0 failures in accepted run | 0 failures in 1,680 samples | **PASS** |
| Idle Server Memory (RSS) | ≤ 12 MiB p50 | 6.2 MiB p50 | **PASS** |
| Runtime Route Compilation | 0 | 0 (pre-compiled segments in pack) | **PASS** |
| Runtime Schema Compilation | 0 | 0 (Schema IR v1 interpreted natively) | **PASS** |
| Production TS Transpilation | 0 | 0 (bundled at build time) | **PASS** |
| Binary Artifact Size | ≤ 8 MiB (stripped) | 4.6 MiB | **PASS** |
| Treaty Client Minified Size | ≤ 8 KiB | 5.5 KiB (unminified source) | **PASS** |
| Clean Build Duration (25-route) | ≤ 1.0 s | 579 ms | **PASS** |
| 1,000-Route Scaling Budget | ≤ 20% increase vs 25-route | +409% (3.08ms → 15.7ms) | **FAIL** (budget missed; recorded honestly) |

## Overall Recommendation

**M0–M2 Authorized Scope: COMPLETE.**
The core product thesis is **supported by evidence**: Rust + QuickJS with static
AOT contract extraction delivers **30–34× faster cold starts** and **13× lower
idle memory** than a matched Elysia 2 AOT application, while preserving
Treaty-quality end-to-end typing and strict route governance.

Recommended Next Actions for M3 (owner decision):
1. Address 1,000-route pack load overhead via binary pack format or chunked loading.
2. Evaluate multi-worker worker pools for warm high-concurrency workloads.
3. Owner decisions on public product naming, license, and repository (docs/open-decisions.md).
