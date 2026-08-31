# M3-009-A — Worker Scaling Measurement (1/2/4 real parallel QuickJS runtimes)

Generated from `benchmarks/raw/worker-scaling/worker-scaling-summary.json`
(velqu-worker-scaling-v2). Every number below is quoted from that file or
its raw JSONL; nothing is hand-estimated.

## Command and environment

```bash
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$(pwd)=/velqu-src"
export CFLAGS="${CFLAGS:-} -ffile-prefix-map=$(pwd)=/velqu-src -fdebug-prefix-map=$(pwd)=/velqu-src"
cargo build --release -p q-bench-support --bin q-worker-scaling
./target/release/q-worker-scaling            # writes benchmarks/raw/worker-scaling/
```

- Engine: quickjs-ng 0.15.1 via rquickjs 0.12.2 (pinned).
- Host: 12 logical cores visible to the process
  (`available_parallelism`), Linux, shared machine.
- Workload: `cpu.work` — a 20 000-iteration deterministic
  arithmetic+string JS handler, no I/O; the result is verified host-side
  on EVERY invocation (45 000/45 000 correct).
- Method: for each repetition (5, INTERLEAVED round-robin over the
  worker counts so host drift spreads across configs instead of
  correlating with config order) and each W ∈ {1, 2, 4}:
  `spawn_independent` creates W real parallel QuickJS runtimes (one
  thread + one runtime each, ADR-0036 §1/§2) loaded with identical
  bundle bytes in the same construction order (§6). 8 producer threads
  dispatch through the M3-002 bounded `Dispatcher` (per-worker queue
  capacity 1 024, least-outstanding selection); one consumer thread per
  worker owns its engine. After a 100-per-worker warmup (excluded),
  3 000 requests are measured. Latency is measured enqueue→outcome —
  **queue wait is inside every latency number and also reported
  separately**, plus service time (total − queue wait).
- Methodology note: the first (sequential-phases) version of this bench
  produced impossible >linear ratios on this host because the W=1 phase
  happened to run under heavier load; the committed version interleaves
  repetitions to remove that bias, which is why the format is v2.

## Results (exact values from the summary)

| workers | tput median (ops/s) | tput per repetition | service p50/p95/p99 (µs) | queue-wait p99 (µs) | scaling vs 1 (median) | correct |
|--------:|--------------------:|---------------------|--------------------------|--------------------:|----------------------:|--------:|
| 1 | 705 | 825 / 811 / 677 / 705 / 672 | 1 190 / 2 212 / 2 973 | 1 990 138 | 1.00× | 15000/15000 |
| 2 | 1 589 | 1 553 / 1 665 / 1 673 / 1 539 / 1 589 | 1 089 / 2 002 / 2 234 | 1 387 841 | 2.25× | 15000/15000 |
| 4 | 2 752 | 3 127 / 2 752 / 2 946 / 2 651 / 2 679 | 1 233 / 2 203 / 2 379 | 1 077 587 | 3.90× | 15000/15000 |

- Per-worker JS heap: **200 336 bytes per worker on average,
  identical across every worker, config, and repetition** (ADR-0036 §6:
  workers of the same pack are indistinguishable). Total heap = W ×
  200 336 B (W=4: 801 344 B).
- Process RSS, first repetition (KiB, process-level only — allocator
  reuse makes per-engine RSS deltas coarse; heap is the per-worker
  metric): W=1 3 036→5 676, W=2 5 676→6 576, W=4 6 388→7 836.
  Subsequent repetitions grow slowly (engine churn + allocator
  retention; full per-repetition series in the summary).
- 48 500 raw samples retained in `worker-scaling.jsonl` (3 500 warmup +
  45 000 measured; one line each: workers, rep, idx, totalUs,
  queueWaitUs, correct).

## Reading the numbers honestly

1. **Real scaling exists**: median throughput 705 → 1 589 → 2 752 ops/s
   for 1 → 2 → 4 workers (2.25× / 3.90×), with per-request service time
   flat (~1.1–1.2 ms p50). Throughput gains come from parallel runtimes,
   not faster individual requests. This is the invocation-boundary
   measurement ADR-0036 scoped for M3-009.
2. **Ratios slightly above linear at W=2 (2.25×) are a host-scheduling
   artifact, not magic**: at W=1 the single consumer thread competes
   with 8 producer threads and 2 Tokio threads on 12 cores, and its
   service p50 is measurably higher (1 190 µs vs 1 089 µs at W=2);
   spreading consumers over more cores relieves that contention. The
   honest claim is "scaling is at least near-linear in this range",
   not a precise efficiency figure.
3. **Repetition spread**: W=1 varies 672–825 ops/s (±10 %) on this
   shared host even interleaved; per-repetition values are published so
   downstream readers can judge stability (constraint 12).
4. **Queue latency is visible, not hidden**: the closed-loop burst
   fills the 1 024-slot queues (queue-wait p99 1.99 s at W=1 down to
   1.08 s at W=4 — the burst is the load generator's shape, reported
   next to service time). **Service p99 stays flat (~2.2–3.0 ms) across
   1/2/4 workers under full saturation: no p99 collapse** (parent
   guardrail).
5. **Memory scales linearly and is bounded per worker**: identical
   200 336 B heaps per worker; the worker-count ceiling (`MAX_WORKERS`)
   bounds total heap by construction.

## Guardrail status (parent M3-009)

- *2 workers achieve approved scaling target or limitation is
  documented* — **no numeric approved target exists in the docs
  today**; measured 2.25× (2 workers) and 3.90× (4 workers) medians
  are recorded. Setting a numeric target is an owner decision (tracked
  with the open-items owner-decision list in REVIEW_INDEX).
- *4-worker memory is budgeted* — per-worker heap identical and linear;
  W=4 total heap 801 344 B.
- *Serverless profile remains unchanged* — this bench exercises no
  runtime profile path; the serverless HTTP runtime is untouched by
  this packet.
- *No p99 collapse under saturation* — service p99 flat (~2.2–3.0 ms)
  across 1/2/4 workers under full saturation.

## Scope boundary (explicit)

The HTTP layer still drives a single engine; multi-engine HTTP wiring
is the M3 integration. This packet measures the exact core that wiring
will call — N real parallel runtimes behind the M3-002 bounded
Dispatcher — per ADR-0036's M3-009 obligation. Percentile report
formatting, C1/C2/C3 controlled workloads, and physical topology
recording are M3-009-B/C/D.

## Artifact hashes (SHA-256)

| artifact | sha256 |
|---|---|
| `target/release/q-worker-scaling` (remapped build) | `69c50c4a59152c33eccfdb10d56ebfd168c3331bfc63b58fc090b8a98d48895f` |
| `benchmarks/raw/worker-scaling/worker-scaling.jsonl` (48 500 lines) | `559df85058658a369c4f22ca6c01189b3251789ca4839019c301cddc646a8270` |
| `benchmarks/raw/worker-scaling/worker-scaling-summary.json` | `516ca3ac66cab3ef22000a5c6719bba997757d20b714acc171639feb5ac1f3f8` |
