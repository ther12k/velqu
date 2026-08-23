# M25-010-D — CPU and RSS evidence

Raw data: `benchmarks/raw/codec-m25-010-d/` (`codec.jsonl` — 2,000
samples per candidate per case after 200 warmup; `codec-summary.json`
with per-case RSS and allocator metrics; `codec.alloc.json` tracer
profile; `evidence.json` with artifact hashes). Run id packet field
`M25-010-D`, engine quickjs-ng/0.15.1, iters 2000 / warmup 200 — matched
to M25-010-A for comparability.

## What this packet adds

M25-010-A recorded wall-clock totals with allocator tracing disabled.
This packet completes the instrumentation:

1. **RSS recording added to the bench** (`crates/q-bench-support/src/bin/codec_bench/main.rs`):
   per-case `rssKbAfter`/`hwmKb` snapshot from `/proc/self/status`
   (VmRSS/VmHWM) plus a process-level `maxRssKb`. CPU per sample
   (getrusage RUSAGE_SELF) was already recorded and stays.
2. **Allocator tracing enabled**: run under
   `LD_PRELOAD=target/alloc-tracer.so` (sha256 `f53c5f7c…`, identical to
   the M25-002-C record) — A's `allocator: unavailable` gap is closed.
3. Evidence-file paths in `evidence.json` now point at the actual
   `--out-dir` instead of a hardcoded historical path (A disclosed that
   wart; fixed here).

## Host caveat (disclosed)

On this host (kernel 7.0.0-28-generic) `ru_maxrss` was observed to
return implausible values on some invocations (a first bench run and a
hello-world probe both reported 5,544,608 KiB). The committed run
reported a plausible value (12,656 KiB, matching VmHWM), but the
instrumentation note flags it: **per-case `hwmKb` from `/proc` is the
authoritative peak-RSS source**; `maxRssKb` is recorded raw.

## CPU per op (p50 µs, 2,000 samples/cell)

| case | quickjs-json | generic-rust | generated-schema | gen vs generic |
|---|---|---|---|---|
| small_user | 35 | 33 | 37 | +12% |
| nested_order | 53 | 33 | 42 | +27% |
| records100 | 323 | 176 | 172 | −2% |
| records1000 | 2661 | 2895 | **2461** | **−15%** |
| pad_256 | 24 | 21 | 23 | +10% |
| pad_1k | 26 | 21 | 20 | −5% |
| pad_16k | 106 | 29 | 29 | ±0% |
| pad_64k | 289 | 45 | 49 | +9% |
| opt_null | 24 | 37 | 34 | −8% |
| problem_shape | 37 | 24 | 23 | −4% |

Reading: C2's CPU advantage concentrates where validation/projection is
a real fraction of the work (records1000 −15%; records100 −2%); the
remaining shapes sit inside a ±10–15% band that this host shows
run-to-run (compare M25-010-A, where the same cells landed differently,
e.g. nested_order flipped sign between runs). No cell shows runaway CPU
cost from strategy selection; cpu_p99 stays bounded (worst 5.6 ms at
records1000-generic).

## Allocations per op (p50, tracer-captured)

generic-rust and generated-schema allocate nearly identically — the
shared serde_json parse dominates both (records1000: 1,894,915 B and
1.89 MB/op respectively; call counts 63,782 vs 64,713, i.e. +1.5% for
the projection). quickjs-json allocates fewer requested bytes on padded
shapes but spends multiples more CPU there. Allocator events and
requested bytes are deltas per op, not live heap.

## RSS

Bench process: ~6.6 MB resident at the first case, 12.0 MB after the
full corpus; process HWM 12,656 KiB. Same-case candidate-to-candidate
RSS deltas are ≤ ~220 KB across all 30 cells — **strategy selection
does not measurably inflate memory**. Server-side memory is covered by
M25-010-C's cold-start evidence (velqu 52 MB RSS at 1,000 routes vs
elysia2 66 MB; route-count harness records `rssP50Kb` per cell).

## Decision matrix

| axis | evidence | verdict |
|---|---|---|
| CPU cost of strategies | table above; wins at array scale, neutral elsewhere, bounded tails | supports native default + measured fallback (M25-002-D) |
| memory cost of strategies | ≤220 KB per-case deltas; no growth trend across corpus | no inflation |
| allocation profile | host candidates share the parse-dominated profile (+1.5% calls for projection) | fallbacks explicit and measured |
| cold-start cross-check | M25-010-C: regression escalated to gate; RSS below baseline competitor | open item remains startup load path, not codecs |

Scope: no binary QPack encoding, no capability API expansion, no ORM.
No performance claim beyond the recorded samples; raw JSONL retained.
