# M25-010-A — C2 plus medium/large JSON workloads (fresh evidence)

Raw data: `benchmarks/raw/codec-m25-010-a/` (codec.jsonl — 2,000 samples
per candidate per case after 200 warmup; codec-summary.json; evidence.json
with sha256 of the binary and generated module).

## Environment and command

- Engine: quickjs-ng/0.15.1; iters 2000, warmup 200.
- Actual invocation (the harness-generated `command` field inside
  evidence.json still carries the M25-002-C codec-c template string — a
  known harness artifact, disclosed here rather than hand-editing a
  generated file):

```
RUSTFLAGS="--remap-path-prefix=$PWD=/velqu-src" \
CFLAGS="-ffile-prefix-map=$PWD=/velqu-src -fdebug-prefix-map=$PWD=/velqu-src" \
cargo build --release -p q-bench-support
./target/release/q-codec-bench --out-dir benchmarks/raw/codec-m25-010-a --iters 2000
```

- Allocator tracing was NOT enabled in this run (`allocator: unavailable`
  in the summary); CPU/RSS and allocator deltas belong to M25-010-D.
- `generated-schema` is the C2 fused decode/validate projection prototype
  (M25-002-A); the PRODUCTION direct decoders/encoders (M25-003..M25-006)
  are not separately timed here — this run re-establishes the strategy
  matrix on the current tree.

## Workload matrix (request decode + response encode, total µs per op)

All ten frozen corpus cases — small JSON, nested key order, arrays of
100/1,000 records, padded dynamic payloads 256B/1KB/16KB/64KB,
optional/null, problem shape:

| case | bytes | quickjs p50 | generic p50 | C2 p50 | C2 vs generic |
|---|---|---|---|---|---|
| small_user | 75 | 20.9 | 18.7 | 18.9 | -1.1% |
| nested_order | 106 | 25.8 | 23.5 | 23.3 | +0.9% |
| records100 | 4976 | 247.6 | 269.1 | 266.2 | +1.1% |
| records1000 | 52726 | 2367.7 | 2772.0 | 2313.1 | +16.6% |
| pad_256 | 244 | 23.6 | 23.3 | 17.4 | +25.4% |
| pad_1k | 1012 | 29.3 | 18.0 | 18.5 | -2.8% |
| pad_16k | 16372 | 80.7 | 24.6 | 25.2 | -2.6% |
| pad_64k | 65524 | 258.6 | 41.2 | 41.7 | -1.3% |
| opt_null | 49 | 21.8 | 19.3 | 19.0 | +2.0% |
| problem_shape | 156 | 20.4 | 20.5 | 20.1 | +2.1% |

Stage timings (C2 codec stage — decode+validate+projection, excluding the
shared serde_json parse and QuickJS boundary): p50 1.35µs
(small_user), 3.73µs (pad_16k), 12.10µs (pad_64k),
1117.7µs (records1000).

## Findings (matched, reproducible; honest reading)

1. **C2 materially improves two shapes**: records1000 (+16.6% total p50
   vs generic-rust) and pad_256 (+25.4%). On the remaining eight cases
   C2 sits within ±3% of generic-rust — both host candidates share the
   same serde_json parse and the same QuickJS boundary, so total time is
   dominated by that shared cost and the µs-scale codec stage cannot move
   it. This is the documented limitation the parent guardrail asks for:
   the fused projection wins where validation/projection is a meaningful
   fraction of the work (deep arrays, many small fields), and is neutral
   where parse/bridge dominates.
2. **Native (host) vs engine (quickjs-json)**: the host candidates are
   3.2x faster at pad_16k (25µs vs 81µs) and 6.2x at
   pad_64k (42µs vs 259µs) — engine stringify degrades
   sharply with payload size while the native traversal stays flat.
   quickjs-json is 7% faster than the host candidates at records100
   (~5KB array-of-records with per-record objects) — the one shape where
   engine handling approaches parity and wins slightly.
3. **Decision matrix** (supports the M25-002-D native default + measured
   fallback): native for representable schemas (wins or ties everywhere
   except the ~5KB records shape where the gap is 7%); explicit fallback
   (`s.fallback("measured", ...)`) for shapes where a route's own
   evidence favors the engine. Strategy choice and bridge model remain
   inspectable per route (`velqu inspect routes`, M25-007-D).

No binary QPack encoding, no capability API expansion, no ORM — per the
packet's scope line. No performance claim beyond the recorded samples;
raw JSONL retained for reproduction.
