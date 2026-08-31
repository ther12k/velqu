# M3-010-C — Retained Memory and Task/Slot Counts (15-minute soak with continuous chaos)

Generated from `benchmarks/raw/worker-scaling/soak-summary.json`
(velqu-soak-v2) and its raw window JSONL (30 windows), from the final
committed `q-soak` build. This report analyzes the exact retained-memory
and task/slot accounting under 15 minutes of continuous mixed load with
periodic worker replacement, disconnect injection, and timeout injection.

## Harness and tracking methodology

`crates/q-bench-support/src/bin/soak.rs` (bin `q-soak`):
1. **Retained memory**:
   - Initial per-worker heap size is captured immediately after bundle
     load (`e.stats().heap_used` updated on `WorkerMsg::Load`).
   - Final per-worker heap size is captured at worker shutdown.
   - Process RSS is sampled at each 30-second window boundary via
     `/proc/self/status` `VmRSS`.
2. **Task and slot tracking**:
   - Every admitted invocation is tracked through
     `q_capabilities::InvocationOwnership` (M3-007-A) upon queue pop
     and settled at outcome/error delivery.
   - Per-window samples record queue lengths, queue total, and live
     ownership-pending slots.
   - At shutdown, each consumer worker sends its final `EngineStats`
     including `native_tasks_started`, `native_tasks_completed`,
     `native_tasks_aborted`, `native_tasks_alive`, `pending_ops`, and
     `cancelled_invocations`.

```bash
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$(pwd)=/velqu-src"
cargo build --release -p q-bench-support --bin q-soak
./target/release/q-soak --workers 2 --duration-secs 900 --window-secs 30 \
    --chaos-secs 60 --disconnect-permille 5 --timeout-permille 5 \
    --out-dir benchmarks/raw/worker-scaling
```

## Results (exact values from summary)

### Retained memory analysis

| metric | worker 0 | worker 1 | process level |
|---|---|---|---|
| initial heap (post-load) | 201 376 B | 201 376 B | — |
| final heap (post-soak) | 206 130 B | 202 000 B | — |
| **net heap delta** | **+4 754 B** | **+624 B** | — |
| process RSS initial → final | — | — | 5 760 → 6 460 KiB (**+700 KiB**) |
| RSS drift per completed request | — | — | **0.30 B/request** |
| conclusion | **no monotonic leak** (heap delta flat across 2.43 M requests and 14 rebuilds) |

### Task and slot counts

| dimension | metric | value |
|---|---|---|
| **Invocation ownership** | total registered | **2 431 643** |
| | total settled | **2 431 643** |
| | **pending at shutdown** | **0** (quiesced) |
| | capacity rejections | 0 |
| | duplicate / unknown rejections | 0 |
| **Live slots** | peak live queue slots | 2 048 (bounded by `queue_capacity × 2`) |
| | peak ownership-pending slots | 2 |
| | final pending slots | **0** |
| **Native task accounting** (final worker 0) | timer ops started | 24 992 |
| | timer ops completed | 24 992 |
| | native tasks started | 24 992 |
| | native tasks completed | 24 177 |
| | native tasks aborted (timeouts) | 815 |
| | **native tasks alive at shutdown** | **0** |
| | **pending ops at shutdown** | **0** |

**Accounting is exact**: 2 407 340 completed + 12 136 injected disconnects +
12 167 injected timeouts = **2 431 643 dispatched == 2 431 643 settled**.

## Guardrail status (parent M3-010)

- *No monotonic leak* — per-worker heap delta is +4.7 KB / +0.6 KB after
  2.43 M requests and 14 engine replacements; RSS drift is 0.30 B/req
  (allocator retention).
- *All slots/queues/pools quiesce* — ownership pending at shutdown is 0;
  native tasks alive at shutdown is 0; pending native ops at shutdown is 0.
- *No boundary violations* — verify's scheduler-boundary suite passes.

## Artifact hashes (SHA-256)

| artifact | sha256 |
|---|---|
| `target/release/q-soak` (remapped build) | `6d8b841ac2193c0c209edb3aa8dfc420408b271a605d813f19cb068c2b428377` |
| `benchmarks/raw/worker-scaling/soak.jsonl` (30 lines) | `198f3a7d534654d07d3b8bebb18194720add2676641f297ae6a43f05d3fa1c24` |
| `benchmarks/raw/worker-scaling/soak-summary.json` | `1bd92101577f0b6151a46c5caedc8098215e6870b4a2d371ff034993980cc1c0` |
