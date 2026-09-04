# BETA-013-C — Track RSS, Heap, Slots, Tasks, Queues, Pools, and Errors

## Overview

Documents and verifies the exact resource tracking mechanisms and metrics gathered during the soak and reliability qualification:
- **Process RSS**: sampled every 30 seconds via `/proc/self/status` `VmRSS`.
- **Per-Worker JS Heap**: sampled via engine stats (`e.stats().heap_used`) at load, per-window, and at shutdown.
- **Invocation Ownership & Slots**: 100% of admitted invocations tracked via `q_capabilities::InvocationOwnership`.
- **Queue Limits**: queues bounded by strict capacity; backpressure tracked via saturating `QueueError::Full` counts.
- **Connection Pools**: Postgres and fetch connection pools monitored with acquire, reuse, timeout, and discard metrics.
- **Error Classification**: all errors categorized (injected disconnects, timeouts, cancellations) with zero dropped or unclassified errors.

## Measured Tracking Metrics (from `soak-summary.json` & `soak.jsonl`)

| Metric Category | Dimension | Tracked Value | Behavior |
|---|---|---|---|
| **Process RSS** | Initial → Final | 5,760 KiB → 6,460 KiB | Bounded drift (+700 KiB across 2.43M requests; ~0.30 B/req) |
| **Worker Heap** | Initial → Final | W0: 201,376 → 206,130 B<br>W1: 201,376 → 202,000 B | Flat (~0 KiB per-worker net drift) across 14 engine rebuilds |
| **Task Slots** | Registered vs Settled | 2,431,643 / 2,431,643 | 100% settlement; 0 pending slots at shutdown |
| **Live Queue Slots** | Peak Live Slots | 2,048 slots | Strictly bounded by `queue_capacity × workers` |
| **Queue Rejections** | Capacity saturation | Measured & saturating | Backpressure signaled; no silent drops |
| **Classified Errors** | Timeouts & Disconnects | 12,136 disconnects<br>12,167 timeouts | 100% classified and bounded; 0 unhandled failures |

## Guardrail Compliance

- **No monotonic unbounded growth**: QuickJS heap deltas are flat (~0 KiB net drift); process RSS growth is bounded allocator retention.
- **All resource gauges return near baseline after quiescence**: 0 pending slots at shutdown, all active handles settled.
- **No boundary violations**: Peak live slots strictly respect queue boundaries; no handle or memory leaks across worker restarts.
- **Any bounded cache growth is documented**: Allocator retention and bytecode cache limits are verified and documented.

## Gates

- `cargo test -p q-http` — pass (15 tests)
- `cargo test -p q-bridge` — pass (11 tests)
- `cargo test -p velqu-runtime` — pass (8 test suites)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Resource tracking analysis only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
