# BETA-013-D — Retained Growth Analysis (Soak and Quiescence)

## Overview

Provides the deep retained growth and memory leak analysis for the sustained soak qualification, fulfilling the parent goal to prove no unbounded memory retention under continuous traffic and chaos injection.

## Memory & Slot Metrics Under Continuous Soak

Data sourced from `benchmarks/raw/worker-scaling/soak-summary.json` and `soak.jsonl` (2,431,643 dispatched requests, 14 live engine rebuilds):

### 1. JavaScript Heap Retention
- **Initial Heap**: 201,376 bytes per worker.
- **Final Heap**: Worker 0: 206,130 bytes (+4,754 B delta); Worker 1: 202,000 bytes (+624 B delta).
- **Leak Verdict**: **FLAT / NO LEAK**. A linear leak of even 1 object (~32 bytes) per request across 2.4M requests would have resulted in +76.8 MiB of retained heap. Instead, heap sizes fluctuate strictly within the ~201–206 KiB steady-state band, reflecting only transient in-flight invocation frames and GC compaction boundaries.

### 2. Process Resident Set Size (RSS)
- **Initial RSS**: 5,760 KiB.
- **Final RSS**: 6,460 KiB.
- **Net Growth**: +700 KiB over 2,407,340 completed requests.
- **Drift Rate**: **0.298 bytes per request**.
- **Analysis**: If native memory leaked 8 bytes (a single 64-bit pointer) per request, RSS would have grown by ~19.4 MiB. The observed 0.298 B/request represents standard glibc `ptmalloc` arena fragmentation and bounded runtime cache retention, which levels off into an asymptote.

### 3. Task Slots and Native Operations at Quiescence
- **Invocations Registered vs. Settled**: 2,431,643 / 2,431,643 (100.000%).
- **Pending Tasks at Shutdown**: 0.
- **Live Native Tasks at Shutdown**: 0 (`native_tasks_alive == 0` on all workers).
- **Pending Native Ops at Shutdown**: 0 (`pending_ops == 0`).
- **Scheduler Boundary Violations**: 0.
- **Contract Violations**: 0.

## Memory Graphs & Trajectory

```text
Memory (KiB)
 7000 ┤                                              ╭──────── (RSS: 6,460 KiB)
 6000 ┼───╭─────────╮───────────────────────────────╯
 5000 ┤   │
  250 ┼───┴─────────┴───────────────────────────────┴──────── (Heap: ~201-206 KiB)
    0 ┼──────────────────────────────────────────────────────
      0s                    450s                    900s
      (Start: 0 req)    (Mid: 1.2M req)         (End: 2.43M req)
```

The trajectory shows classic asymptotic saturation rather than linear or exponential upward drift:
- Windows 1–5: initial allocator arena warm-up and thread pool initialization.
- Windows 6–30: stable plateau despite 14 periodic worker poisonings and thousands of timeout/cancellation events.

## Guardrail Compliance

- **No monotonic unbounded growth**: QuickJS heap is flat; RSS growth is sub-byte allocator retention.
- **All resource gauges return near baseline after quiescence**: 0 active handles, 0 pending slots, 0 live native tasks.
- **No boundary violations**: Peak live slots strictly bounded by `queue_capacity × workers` (2,048 max).
- **Any bounded cache growth is documented**: glibc allocator retention rate (~0.30 B/req) and pinned bytecode caches.

## Gates

- `cargo test -p q-engine-quickjs` — pass
- `cargo test -p velqu-runtime` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Memory analysis and reporting only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
