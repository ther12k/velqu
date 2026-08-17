---
type: Evidence Report
title: Memory Report (idle RSS and scaling)
status: complete
milestone: M1
---

# Memory report

Source: `benchmarks/raw/cold-start/summary.json` (`rssKbAfterReady`, VmRSS via
/proc at first-valid-response time) and `benchmarks/raw/route-count/summary.json`.
Same environment as the cold-start report.

## Idle RSS after ready (p50, fixture app)

| Candidate | RSS p50 | vs budget |
|---|---:|---|
| **velqu** | **6.2 MiB** | ≤12 MiB — **PASS** |
| raw-rust | 3.4 MiB | lower bound |
| raw-bun | 36.2 MiB | — |
| elysia2 | 82.6 MiB | — |

Framework/engine incremental RSS over raw Rust: **~2.8 MiB** (budget ≤8 MiB —
PASS). velqu uses ~17% of raw-bun's RSS and ~7.5% of Elysia's.

## Route-count scaling (RSS p50)

| Candidate | 25 routes | 1,000 routes |
|---|---:|---:|
| velqu | 6.2 MiB | 11.1 MiB |
| raw-bun | 36.7 MiB | 36.7 MiB |
| elysia2 | 91.6 MiB | 114.7 MiB |

velqu stays under the 12 MiB idle budget even at 1,000 routes (11.1 MiB) —
the pack+bundle+route table cost ~4.9 MiB at 1,000 routes.

## JS heap observation

Engine stats expose `heap_used` (QuickJS memory_usage) on every loop turn;
fixture app after load: ~119KB heap used (logged at shutdown). Heap cap is
set at 32 MiB, stack 512 KiB — resource limits, not a sandbox boundary
(SEC-002).

## Retained-state checks

- Request handle slots after completion: 0 live (`store.live_slots()==0`
  asserted in engine + conformance tests; slots are recycled through a free
  list with generation bump).
- Cancelled operation registry after settle: ops removed on completion or
  rejection; late completions counted and dropped
  (`late_completions_dropped`).
- 10k-request retained-state soak is an M2 item (warm report); not yet
  measured — recorded as UNEXECUTED, not assumed.

## Platform allocator note

Default system allocator (glibc malloc), no jemalloc/mimallop tuning; RSS
numbers include allocator overhead. OS variance disclosed per methodology.
