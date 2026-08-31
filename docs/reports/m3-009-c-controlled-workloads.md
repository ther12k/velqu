# M3-009-C — Controlled Workloads C1/C2/C3 (CPU, mixed, controlled I/O)

Generated from `benchmarks/raw/worker-scaling/worker-scaling-summary.json`
(velqu-worker-scaling-v4) and its raw JSONL (71 100 samples). The harness
is M3-009-A/B's worker-scaling bench generalized with a WORKLOAD
dimension: 9 configs (3 workloads × 1/2/4 workers), 3 repetitions each,
interleaved round-robin so host drift hits every config equally. Every
response is verified host-side per kind; errors are classified
(timeout/mismatch), never dropped.

## Frozen workload definitions

- **C1 — CPU-bound**: 100 % `cpu.work` (20 000-iteration deterministic
  arithmetic+string JS, result verified exactly).
- **C2 — mixed**: 80 % `light.work` (tiny object return) + 20 %
  `cpu.work`, chosen by the deterministic rule `id.is_multiple_of(5) →
  cpu` so the consumer verifies every response against the known kind.
- **C3 — I/O-bound**: 100 % `io.delay` — one 1 ms native timer op per
  invocation. **Controlled I/O**: deterministic, fully local, no
  external network (1 200 requests per repetition).

Command and environment: identical to M3-009-A/B (remapped release
build of `q-worker-scaling`; quickjs-ng 0.15.1 via rquickjs 0.12.2;
12 logical cores visible; shared host).

## Results (exact values from the summary)

### C1 — CPU-bound

| workers | tput median | per repetition | service p50/p99 (µs) | scaling | correct |
|--------:|------------:|----------------|---------------------|--------:|--------:|
| 1 | 802 | — (see summary) | 1 064 / 2 397 | 1.00× | 9000/9000 |
| 2 | 1 583 | — | 1 117 / 2 342 | 1.97× | 9000/9000 |
| 4 | 2 835 | — | 1 252 / 2 444 | 3.53× | 9000/9000 |

### C2 — mixed (80 % light / 20 % CPU)

| workers | tput median | service p50/p99 (µs) | scaling | correct |
|--------:|------------:|---------------------|--------:|--------:|
| 1 | 3 545 | 24 / 2 119 | 1.00× | 9000/9000 |
| 2 | 7 508 | 23 / 2 075 | 2.12× | 9000/9000 |
| 4 | 13 911 | 23 / 1 995 | 3.92× | 9000/9000 |

Light-request service p50 is ~23 µs at every worker count while the
CPU tail runs — mixing light and heavy work does not starve the light
class (the M3-008 fairness posture, measured in situ). C2 scales to
13.9k ops/s at 4 workers.

### C3 — I/O-bound (controlled: 1 ms native timers)

| workers | tput median | service p50/p99 (µs) | scaling | correct |
|--------:|------------:|---------------------|--------:|--------:|
| 1 | 438 | 2 289 / 2 613 | 1.00× | 3600/3600 |
| 2 | 871 | 2 318 / 2 774 | 1.99× | 3600/3600 |
| 4 | 1 688 | 2 405 / 3 714 | 3.85× | 3600/3600 |

The I/O-bound workload scales near-linearly with the tightest
per-repetition spread of the three — waiting work overlaps across
parallel runtimes instead of serializing on one. Process CPU-per-op
collapses from ~2.8 ms (W=1) to ~0.4 ms (W=4): timer waits do not burn
CPU, exactly as an I/O-bound profile should. Memory: C3 per-worker heap
is 204 182 B vs 201 339 B for CPU/mixed — the timer op table accounts
for the small delta; no growth across repetitions.

## Cross-workload readings (honest)

1. **Every workload scales** across 1→2→4 workers: C1 1.97×/3.53×,
   C2 2.12×/3.92×, C3 1.99×/3.85× (medians). No workload inverts; C2's
   mix benefits most (light handlers dominate the throughput).
2. **Service p99** stays in-band across worker counts within each
   workload (C1 ~2.3–2.4 ms; C2 ~2.0–2.1 ms; C3 2.6–3.7 ms) — no p99
   collapse attributable to the multi-worker architecture.
3. Queue-wait p99 dominates total p99 everywhere (closed-loop burst
   against 1 024-slot queues) and is reported separately in the
   summary, never hidden.
4. Errors: **0 across all 21 600 measured requests** (timeout and
   mismatch classification armed and empty).

## Artifact hashes (SHA-256)

| artifact | sha256 |
|---|---|
| `target/release/q-worker-scaling` (remapped build) | `4386157183f0b0283c10ac2eb93185394ac2745ae6f2f09d75ab5a1ad761bcb7` |
| `benchmarks/raw/worker-scaling/worker-scaling.jsonl` (71 100 lines) | `c056e123e4ec6bfba2c8b49e7ca91781324385b065163c76a67f0cccd3c81891` |
| `benchmarks/raw/worker-scaling/worker-scaling-summary.json` | `e5385f1ed2748ca4a718343cf090d920a065733cff5845f4222f5dd126546c4b` |
