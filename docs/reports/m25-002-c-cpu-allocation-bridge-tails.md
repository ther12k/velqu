---
type: Evidence Report
title: Codec CPU, Allocation, Bridge Time, and Tails (M25-002-C)
status: complete
milestone: M25
---

# Codec CPU, allocation, bridge time, and tails — M25-002-C

M25-002-C adds per-sample CPU, allocation-event, native-bridge, and stage
evidence to the frozen M25-002-B payload matrix (same ten shapes, same three
candidates, 2,000 samples per cell, correctness asserted per sample). It
changes no production codec, strategy, or fallback behavior.

Run ID: `m25-002-c-1787293686`. 30/30 cells OK, 2,000/2,000 correct each,
60,000 raw rows, zero null allocator fields.

## Reproducible command

```text
RUSTFLAGS="--remap-path-prefix=$(pwd)=/velqu-src" \
CFLAGS="-ffile-prefix-map=$(pwd)=/velqu-src -fdebug-prefix-map=$(pwd)=/velqu-src" \
CARGO_TARGET_DIR=target/m25-002-c-bench \
  cargo build --release -p q-bench-support \
  --features bench-instrumentation --bin q-codec-bench

cc -shared -fPIC -O2 -ldl -o target/alloc-tracer.so scripts/alloc-tracer.c

LD_PRELOAD=target/alloc-tracer.so \
VELQU_ALLOC_PROFILE=benchmarks/raw/codec-c/codec.alloc.json \
/usr/bin/time -v -o benchmarks/raw/codec-c/codec.process.time.txt \
target/m25-002-c-bench/release/q-codec-bench \
  --out-dir benchmarks/raw/codec-c --iters 2000
```

The `bench-instrumentation` feature (q-bench-support → q-engine-quickjs →
q-bridge) compiles the bridge timing into an isolated target directory; the
default production workspace build stays feature-free and carries no timing
code.

Environment: 13th Gen Intel Core i5-13420H, Linux 7.0.0-28-generic, rustc
1.96.0, quickjs-ng 0.15.1 via rquickjs 0.12.2, one QuickJS worker, in-process,
no network, Bun not in the measured path.

## Instrumentation semantics and limits (read before citing numbers)

- **CPU** is `getrusage(RUSAGE_SELF)` deltas per sample — user+system across
  all threads (QuickJS worker + tokio). This host sets
  `perf_event_paranoid=4`, so hardware perf counters are unavailable; no
  cycle/cache claim is made.
- **Allocation** rows are LD_PRELOAD allocator events and *requested* bytes
  (malloc/calloc/realloc/free), not live heap or RSS. The tracer's
  process-wide exit totals are committed separately in `codec.alloc.json`.
- **bridgeAccessUs** times only `RequestStore::access`/`cached_query` inside
  q-bridge (feature-gated). It is *not* the full JS/native round trip:
  `engineUs` is the complete invoke→outcome span (queue, JS, conversion).
- **codecUs** covers host-side parse+validate/project. For `quickjs-json` it
  is ~0 by construction: that candidate's parse happens inside the engine, so
  its codec cost appears in `engineUs` instead. Cross-candidate codecUs
  comparisons are only valid between the two host candidates.
- Wall numbers from this instrumented run are **not comparable** with the
  M25-002-B run: instrumentation adds two rusage reads, two allocator
  snapshots, and two bridge snapshots per sample, and run-to-run variance
  dominates. Compare candidates within this run only.

## Wall and CPU tails (μs, lower is better)

| Case | Candidate | total p50 | total p95 | total p99 | cpu p50 | cpu p99 |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| small_user | quickjs-json | 39.7 | 92.8 | 187.3 | 39.0 | 109.0 |
| small_user | generic-rust | 21.9 | 61.8 | 104.4 | 22.0 | 95.0 |
| small_user | generated-schema | 26.8 | 71.7 | 112.2 | 26.0 | 104.0 |
| nested_order | quickjs-json | 28.4 | 79.4 | 99.4 | 28.0 | 97.0 |
| nested_order | generic-rust | 35.1 | 98.0 | 164.5 | 35.0 | 153.0 |
| nested_order | generated-schema | 29.5 | 61.5 | 94.3 | 29.0 | 80.0 |
| records100 | quickjs-json | 288.6 | 535.3 | 717.9 | 288.0 | 679.0 |
| records100 | generic-rust | 340.5 | 704.4 | 1,818.1 | 174.0 | 1,345.0 |
| records100 | generated-schema | 325.1 | 648.2 | 901.3 | 156.0 | 708.0 |
| records1000 | quickjs-json | 2509.8 | 3535.8 | 5,012.4 | 2497.0 | 4,998.0 |
| records1000 | generic-rust | 3043.0 | 5081.9 | 7,481.7 | 2677.0 | 7,195.0 |
| records1000 | generated-schema | 2802.9 | 4124.7 | 6,220.0 | 2366.0 | 5,601.0 |
| pad_256 | quickjs-json | 25.7 | 55.2 | 128.5 | 26.0 | 106.0 |
| pad_256 | generic-rust | 35.2 | 62.7 | 92.0 | 35.0 | 88.0 |
| pad_256 | generated-schema | 19.0 | 44.2 | 80.8 | 19.0 | 68.0 |
| pad_1k | quickjs-json | 26.1 | 41.3 | 73.5 | 26.0 | 68.0 |
| pad_1k | generic-rust | 19.4 | 38.1 | 63.0 | 20.0 | 52.0 |
| pad_1k | generated-schema | 19.5 | 41.2 | 82.1 | 20.0 | 70.0 |
| pad_16k | quickjs-json | 90.7 | 322.4 | 478.1 | 90.0 | 475.0 |
| pad_16k | generic-rust | 44.8 | 153.1 | 386.2 | 44.0 | 303.0 |
| pad_16k | generated-schema | 32.1 | 115.9 | 188.2 | 32.0 | 177.0 |
| pad_64k | quickjs-json | 349.4 | 795.7 | 1,251.4 | 344.0 | 1,097.0 |
| pad_64k | generic-rust | 74.0 | 189.5 | 299.5 | 73.0 | 266.0 |
| pad_64k | generated-schema | 70.7 | 174.1 | 281.4 | 69.0 | 267.0 |
| opt_null | quickjs-json | 45.0 | 149.3 | 272.3 | 44.0 | 253.0 |
| opt_null | generic-rust | 39.3 | 141.8 | 566.8 | 39.0 | 241.0 |
| opt_null | generated-schema | 37.8 | 111.2 | 158.7 | 38.0 | 158.0 |
| problem_shape | quickjs-json | 33.5 | 91.3 | 160.1 | 33.0 | 142.0 |
| problem_shape | generic-rust | 23.6 | 84.9 | 152.6 | 24.0 | 144.0 |
| problem_shape | generated-schema | 25.9 | 90.9 | 138.9 | 26.0 | 131.0 |

## Stage, bridge, and allocation p50s (μs / bytes / events per sample)

| Case | Candidate | codec p50 | engine p50 | bridge p50 | bridge host calls | alloc bytes p50 | alloc calls p50 |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| small_user | quickjs-json | 0.02 | 38.1 | 0.186 | 1 | 10,780 | 379 |
| small_user | generic-rust | 1.75 | 19.2 | 0.000 | 0 | 10,138 | 356 |
| small_user | generated-schema | 1.75 | 23.0 | 0.000 | 0 | 10,129 | 352 |
| nested_order | quickjs-json | 0.02 | 27.4 | 0.117 | 1 | 13,291 | 488 |
| nested_order | generic-rust | 5.47 | 27.0 | 0.000 | 0 | 17,223 | 497 |
| nested_order | generated-schema | 4.27 | 23.9 | 0.000 | 0 | 17,223 | 497 |
| records100 | quickjs-json | 0.02 | 287.3 | 7.231 | 1 | 138,725 | 5,446 |
| records100 | generic-rust | 159.41 | 176.0 | 0.000 | 0 | 196,671 | 6,605 |
| records100 | generated-schema | 144.04 | 175.1 | 0.000 | 0 | 196,671 | 6,612 |
| records1000 | quickjs-json | 0.09 | 2504.3 | 59.761 | 1 | 1,317,476 | 51,357 |
| records1000 | generic-rust | 1548.78 | 1463.4 | 0.000 | 0 | 1,894,072 | 64,231 |
| records1000 | generated-schema | 1402.51 | 1379.0 | 0.000 | 0 | 1,894,072 | 65,853 |
| pad_256 | quickjs-json | 0.02 | 24.7 | 0.249 | 1 | 11,063 | 362 |
| pad_256 | generic-rust | 1.17 | 33.0 | 0.000 | 0 | 10,418 | 326 |
| pad_256 | generated-schema | 0.94 | 17.2 | 0.000 | 0 | 10,407 | 320 |
| pad_1k | quickjs-json | 0.02 | 25.1 | 0.661 | 1 | 14,134 | 362 |
| pad_1k | generic-rust | 1.20 | 17.3 | 0.000 | 0 | 13,489 | 326 |
| pad_1k | generated-schema | 1.11 | 17.5 | 0.000 | 0 | 13,478 | 320 |
| pad_16k | quickjs-json | 0.02 | 89.3 | 9.014 | 1 | 75,575 | 362 |
| pad_16k | generic-rust | 4.50 | 39.1 | 0.000 | 0 | 74,930 | 326 |
| pad_16k | generated-schema | 4.61 | 25.6 | 0.000 | 0 | 74,919 | 320 |
| pad_64k | quickjs-json | 0.03 | 346.6 | 27.858 | 1 | 272,183 | 362 |
| pad_64k | generic-rust | 15.46 | 53.5 | 0.000 | 0 | 271,538 | 326 |
| pad_64k | generated-schema | 13.43 | 49.3 | 0.000 | 0 | 271,527 | 320 |
| opt_null | quickjs-json | 0.02 | 43.6 | 0.259 | 1 | 10,557 | 373 |
| opt_null | generic-rust | 3.18 | 35.2 | 0.000 | 0 | 10,182 | 366 |
| opt_null | generated-schema | 2.29 | 34.7 | 0.000 | 0 | 10,189 | 372 |
| problem_shape | quickjs-json | 0.02 | 32.0 | 0.196 | 1 | 11,393 | 389 |
| problem_shape | generic-rust | 1.93 | 20.7 | 0.000 | 0 | 10,618 | 372 |
| problem_shape | generated-schema | 1.86 | 22.7 | 0.000 | 0 | 10,601 | 366 |

## Process totals

`/usr/bin/time -v` for the whole run: user 31.73 s,
system 4.77 s, 101% CPU,
max RSS 40428 KB. Tracer exit profile
(`codec.alloc.json`): 243,077,174 mallocs, 992,737
callocs, 73,370,939 reallocs, 244,069,860 frees,
16.53 GB requested allocated bytes,
19.90 GB requested realloc bytes across the
entire process lifetime (includes warmup, correctness passes, and startup).

## Observations (scoped to this host, run, and shapes)

- **Bridge cost is lazy-body-shaped.** `quickjs-json` performs exactly one
  bridge host call per request (the lazy `ctx.json()` body materialization);
  its bridge access grows with body bytes and shape (see the bridge p50
  column: small objects are sub-microsecond, the 65 KB blob is 27.9 μs at
  p50, and the 52,726 B records array is 59.8 μs — the closure copies and
  converts the body, so shape matters, not just size). Host
  candidates pass a pre-validated body and record zero bridge calls.
- **CPU tracks wall closely** at p50 for the sequential loop (the loop is
  CPU-bound); p99 tails diverge where scheduling jitter appears in wall but
  not accumulated CPU.
- **Allocation is dominated by parse/validate trees, not the bridge**: on the
  padded shapes all candidates allocate nearly identical requested bytes per
  sample (the parse tree of the same payload), while records1000 shows the
  host candidates' serde_json tree+projection above the engine-side parse
  (see the alloc bytes p50 column).
- **Stage split at records1000**: host candidates split the sample between
  codecUs (parse+validate) and engineUs, while `quickjs-json` puts everything
  in the engine stage. This is the first per-stage evidence base for
  M25-002-D strategy decisions.

## Decision status

**None.** This packet records instrumentation evidence only. Strategy
selection per route shape, compiler decision rules, and fallback-cost
visibility in inspect output are M25-002-D outputs.

## Artifact hashes

| Artifact | sha256 |
| --- | --- |
| `target/m25-002-c-bench/release/q-codec-bench` | `023115ceb39017f59b45845346fddcc7290ecd2613c622014e185b1ed1ef2e48` |
| `target/alloc-tracer.so` | `f53c5f7c02b491ab05a20ee892aa24b2326d11a79691e1e4ee96282a1a3aeb9b` |
| `crates/q-bench-support/src/bin/codec_bench/generated.rs` | `630ff40ca49225738bdc6a6c934b686a5cfedeb3e5ac5a29ea9ef0f9a41ac146` |
| `benchmarks/raw/codec-c/codec.jsonl` | `5667a466a5f6da9c0441ef411ea969bd8084a7456d37fcbf9a0e07b784cbae65` |
| `benchmarks/raw/codec-c/codec-summary.json` | `e183c16ba7bc2c1015c7cf10d1380a21dc2065ac2eaa49e8740f7e95e21ce3b1` |
| `benchmarks/raw/codec-c/codec.alloc.json` | `f738c5948495e6ecfc723e0e3160e90617f6ebd7172de8d0ecc202386c292801` |
| `benchmarks/raw/codec-c/codec.process.time.txt` | `7ddb2dc63a89455921b1bc37931d1e135f8684cb81d9327de65e361797f91e92` |

Raw paths: `benchmarks/raw/codec-c/{codec.jsonl,codec-summary.json,evidence.json,codec.alloc.json,codec.process.time.txt}`.
`scripts/validate-benchmark-evidence.py` enforces row counts, per-row
correctness, non-null allocator deltas, summary metric completeness, and
evidence hashes for this run.
