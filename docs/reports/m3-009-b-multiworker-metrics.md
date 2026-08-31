# M3-009-B — Multi-Worker Metrics Report (throughput, latency percentiles, queue time, CPU, RSS, errors)

Generated from `benchmarks/raw/worker-scaling/worker-scaling-summary.json`
(velqu-worker-scaling-v3) and its raw JSONL (48 500 samples). The summary
extends M3-009-A's measurement (identical harness: interleaved
repetitions, N real parallel QuickJS runtimes behind the M3-002 bounded
Dispatcher, 3 000 requests per repetition × 5 repetitions per worker
count) with the parent's full metric set: **process CPU seconds, wall
seconds per run, and classified error counters**. M3-009-A's report
covers the method; this report consolidates every metric the parent
requires and adds the CPU/error dimensions.

## Command

```bash
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$(pwd)=/velqu-src"
export CFLAGS="${CFLAGS:-} -ffile-prefix-map=$(pwd)=/velqu-src -fdebug-prefix-map=$(pwd)=/velqu-src"
cargo build --release -p q-bench-support --bin q-worker-scaling
./target/release/q-worker-scaling            # writes benchmarks/raw/worker-scaling/
```

Environment: quickjs-ng 0.15.1 via rquickjs 0.12.2 (pinned), Linux, 12
logical cores visible to the process (SHARED host — this final run
executed while the host carried external load; absolute numbers are
lower than quieter-host runs, which is exactly why the repetition loop
is interleaved and why the claims below are ratio- and flatness-based).
CPU via `getrusage(RUSAGE_SELF)` (user+system, process-level — engines
plus producers and Tokio threads; attribution disclosed, never
per-worker). RSS via `/proc/self/status` `VmRSS` (process-level).

## Consolidated metrics (exact values from the summary)

### Throughput and scaling

| workers | tput median (ops/s) | per repetition | scaling (median) |
|--------:|--------------------:|----------------|-----------------:|
| 1 | 469 | 574 / 469 / 444 / 376 / 504 | 1.00× |
| 2 | 936 | 971 / 849 / 823 / 936 / 1 128 | 2.00× |
| 4 | 1 660 | 1 660 / 1 731 / 1 277 / 1 567 / 2 194 | 3.54× |

The RATIOS are the claim: even with absolute throughput depressed by
host load, the interleaved design keeps the multi-worker scaling ratios
honest (2.00× at W=2, 3.54× at W=4 — vs 2.25×/4.03× on the quieter
run). Scaling degrades gracefully under load; it does not invert.

### Latency percentiles (enqueue→outcome, µs — includes queue time)

| workers | total p50 | total p95 | total p99 | service p50 | service p95 | service p99 | queue-wait p99 |
|--------:|----------:|----------:|----------:|------------:|------------:|------------:|---------------:|
| 1 | 2 138 761 | 2 741 302 | 2 801 629 | 1 996 | 3 378 | 5 458 | 2 798 797 |
| 2 | 1 708 986 | 2 547 655 | 2 650 282 | 1 983 | 3 343 | 5 097 | 2 648 623 |
| 4 | 879 317 | 1 798 091 | 2 101 671 | 2 283 | 3 382 | 5 104 | 2 098 955 |

Service p99 moves with the host (5.1–5.5 ms here vs 2.7–3.5 ms on the
quieter v2 run) but stays FLAT ACROSS worker counts at every point in
time — no p99 collapse attributable to the multi-worker architecture.
Total-latency p99 remains queue-wait-dominated (closed-loop burst
fills the 1 024-slot queues; queue wait is reported alongside, never
hidden).

### CPU

| workers | CPU secs per repetition (user+sys) | wall secs per rep | CPU secs per op |
|--------:|------------------------------------|-------------------|----------------:|
| 1 | 27.4 / 28.0 / 31.5 / 29.0 / 31.0 | 5.2–8.0 | 0.0098 |
| 2 | 11.1 / 11.8 / 11.2 / 10.7 / 9.1 | 2.7–3.6 | 0.0036 |
| 4 | 6.8 / 6.7 / 8.1 / 7.3 / 5.2 | 1.4–2.3 | 0.0023 |

CPU-per-op falls ~4× from W=1 to W=4: at W=1 the single consumer is a
smaller share of the process, and the 8 producers burn CPU in spin
loops while the burst saturates the queue (the closed-loop generator's
own cost — disclosed, not a runtime cost). Absolute CPU bounds, not
pins, the runtime's cost.

### RSS (KiB, process-level per repetition before→after)

| workers | first rep | last rep |
|--------:|-----------|----------|
| 1 | 3 232→5 708 | 11 868→12 316 |
| 2 | 5 708→6 592 | 12 264→12 104 (net negative) |
| 4 | 6 400→7 860 | 12 000→12 760 |

RSS is sequential-process cumulative (configs share the process), so
cross-config deltas conflate allocator retention with new engines; the
per-worker metric is `heap_used`: **200 336 B per worker, identical
across every worker, config, and repetition** (W=4 total 801 344 B).
RSS stabilizes across repetitions (the W=2 last rep even ends lower
than it starts) — no unbounded growth over the 15-measured-run
sequence.

### Errors

| workers | errors (by class) | correct | samples |
|--------:|------------------:|--------:|--------:|
| 1 | {} (0) | 15 000 | 15 000 |
| 2 | {} (0) | 15 000 | 15 000 |
| 4 | {} (0) | 15 000 | 15 000 |

Every dispatched request is either a verified sample or a classified
error (`timeout`: no outcome within budget; `mismatch`: wrong
status/body/value) — none dropped. Zero errors across all 45 000
measured requests.

## Guardrail status (parent M3-009)

- *Raw scaling data* — 48 500 samples (`worker-scaling.jsonl`).
- *Generated report* — this file, generated from the summary JSON.
- *Artifact/environment hashes* — below.
- 2-worker scaling: 2.00× median measured under host load (2.25× on the
  quieter v2 run); numeric target remains an owner decision (tracked
  with REVIEW_INDEX open items).
- No p99 collapse: service p99 flat ACROSS worker counts at every
  point in time.

## Scope boundary (explicit)

Invocation-boundary measurement (engine + dispatcher core). C1/C2/C3
controlled workloads (CPU / short-poll / I/O-bound mixes) are M3-009-C;
physical core topology recording is M3-009-D.

## Artifact hashes (SHA-256)

| artifact | sha256 |
|---|---|
| `target/release/q-worker-scaling` (remapped build) | `778b750621988003c8bf3f86c3483bb4a04fdd33557ec96416659c7e27ecf840` |
| `benchmarks/raw/worker-scaling/worker-scaling.jsonl` (48 500 lines) | `24eb745a90d040910ff807e39f4a26e163123c5360d51412134b1520560be8f7` |
| `benchmarks/raw/worker-scaling/worker-scaling-summary.json` | `30e7af4d8008d323afc28b4b35f859d33075a07672e9690b7254fed55ffefa38` |
