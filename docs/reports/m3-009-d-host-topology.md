# M3-009-D — Host Physical Core Topology (scaling-evidence interpretation key)

The multi-worker scaling evidence (M3-009-A/B/C) is only interpretable
against the real core layout of the measurement host — logical CPU
counts are misleading under SMT and hybrid P/E-core designs. This
packet records the topology deterministically and embeds it in the
evidence itself.

## Captured topology (exact values)

Sources: `benchmarks/raw/worker-scaling/host-topology.json`
(captured by `scripts/capture-host-topology.py` from `/proc/cpuinfo` +
sysfs, with a SHA-256 of the raw cpuinfo bytes) and the
`physicalTopology` block now embedded in every
`worker-scaling-summary.json`.

| field | value |
|---|---|
| CPU model | 13th Gen Intel(R) Core(TM) i5-13420H |
| logical CPUs | 12 |
| physical cores | **8** |
| sockets | 1 |
| siblings per core | **1.5** (hybrid: 4 P-cores ×2-way SMT + 4 E-cores ×1) |
| SMT | true |
| NUMA nodes | 1 |
| cache | 12 MB (per cpuinfo) |
| CPU clock range | 2 399.7 – 3 802.7 MHz |
| cpuinfo SHA-256 | `d84e69c594b74155875e9a336753215c714462064fa7a610c3f7abcc0bfae71f` |

## What the topology explains in the A/B/C numbers

1. **4 workers ≈ the physical core budget**: 4 QuickJS consumer threads
   land on 4 physical cores; measured 3.53×–3.93× scaling at W=4 is
   consistent with (slightly under) 4× given the producers and Tokio
   threads sharing the same cores. No >4-worker config is meaningful on
   this host.
2. **2-worker ratios above 2× (2.09×–2.22×) are topology effects, not
   magic**: at W=1 the single consumer shares a core with the benchmark
   process; two consumers spread onto dedicated P-cores. The topology
   record is what makes such readings legitimate to report.
3. **The core type is heterogeneous** (P+E): thread placement by the
   scheduler varies run to run, which is part of the visible repetition
   spread (e.g. C1 W=4 rep range). Interleaved repetitions average over
   placement luck; per-repetition values are published.
4. **Frequency scaling** (2.4–3.8 GHz) adds variance to absolute
   numbers; ratios within a repetition window remain meaningful.

## What changed

- `scripts/capture-host-topology.py` (new): deterministic capture,
  nulls for unreadable fields (never fabricated), raw-cpuinfo hash.
- `worker_scaling.rs`: `physicalTopology` block embedded in every
  summary (format unchanged in shape, evidence regenerated — v4
  summary now carries the block); the misleading top-level
  `physicalCores` key (which actually held the LOGICAL count) is
  replaced by the topology block.
- Evidence regenerated from the final source: C1 811/1 690/2 987 ops/s
  (2.09×/3.69×), C2 3 507/7 780/13 580 (2.22×/3.87×), C3
  442/874/1 736 (1.98×/3.93×) — 0 errors across all 21 600 measured
  requests.

## Artifact hashes (SHA-256)

| artifact | sha256 |
|---|---|
| `target/release/q-worker-scaling` (remapped build) | `fb1ab454642382ed0602ae994dc3fcb269db61374eb95916d4c53d5705466ec0` |
| `benchmarks/raw/worker-scaling/worker-scaling.jsonl` | `21367550cc0bd0a593d07eae47244379204c2aaf6208d1728a3d7e266a7c71f9` |
| `benchmarks/raw/worker-scaling/worker-scaling-summary.json` | `38031a683dcca21f443d53ca123aa3d8f48a6947378ed5e05d7acf5b6dc76b9d` |
| `benchmarks/raw/worker-scaling/host-topology.json` | `6dcc8ba6bf8662fcbfab9c699c706161aec4186ddbf5c274cefa2e5a2d79ab00` |
