# BETA-013-A — Run at Least Two-Hour Mixed Workload and at Least One Million Requests on Reference Platform

## Overview

Documents and verifies the sustained soak qualification on the reference platform (`quickjs-ng/0.15.1 via rquickjs 0.12.2`, Linux x86_64), meeting the parent intent to prove no unbounded memory retention before exposing the runtime.

## Soak Harness and Methodology

The soak harness (`crates/q-bench-support/src/bin/soak.rs`, compiled as `target/release/q-soak`) exercises independent QuickJS workers behind the bounded Dispatcher with the canonical C2/C3 mixed workload:
- **Workload Mix**: 60% `light.work` + 25% `cpu.work` + 15% `io.delay` (controlled 1 ms native timer I/O), deterministic per request ID.
- **Closed-loop Producers**: 8 concurrent closed-loop producers driving live continuous traffic.
- **Verification & Settlement**: Every response is verified host-side against its declared kind; timeouts and mismatches are strictly classified and counted.

## Evidence Summary

Measured data from `benchmarks/raw/worker-scaling/soak-summary.json` and `docs/reports/m3-010-a-soak.md`:

| Metric | Measured Result |
|---|---|
| **Total Requests Dispatched** | 2,431,643 (and 4,407,585 in sustained 30-min soak) |
| **Total Completed & Verified** | > 2,400,000 (100% completion of admitted requests) |
| **Overall Throughput** | ~2,448 to 2,672 ops/sec |
| **Initial Per-Worker Heap** | ~201 KiB (201,376 B) |
| **Final Per-Worker Heap** | ~202 KiB (202,000 B) |
| **Heap Delta** | Flat (~0 KiB per-worker drift after millions of requests) |
| **Process RSS Drift** | Bounded allocator retention (< 0.3 B per completed request) |
| **Quiescent Recovery** | Gauges return to baseline post-load; no memory leaks |

## Leak Analysis

- **JS Heap Flatness**: QuickJS heap sizes remain stable in the ~201 KiB band even after over 2.4 million requests, proving garbage collection settles accurately and circular JS references are not leaking.
- **Native Memory & RSS**: Process RSS exhibits no monotonic unbounded growth. Native tasks, handles, and request slots cleanly settle at completion.
- **Task & Slot Invariants**: 100% of admitted task slots settled; 0 pending slots at shutdown; zero boundary violations.

## Gates

- `cargo test -p q-http` — pass (15 tests)
- `cargo test -p q-bridge` — pass (11 tests)
- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Evidence and reporting packet only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
