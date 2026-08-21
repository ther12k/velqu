---
type: Evidence Report
title: Codec Payload Matrix (M25-002-B)
status: complete
milestone: M25
---

# Codec payload matrix — M25-002-B

This packet extends the M25-002-A strategy comparison across the required
payload shapes: nested objects, arrays of 100 and 1,000 records, approximately
256 B / 1 KB / 16 KB / 64 KB objects, an optional/null-heavy object, and an RFC
9457-shaped problem payload. It does not change production codec selection or
runtime fallback behavior.

## Reproducible evidence

Command:

```text
RUSTFLAGS="--remap-path-prefix=$(pwd)=/velqu-src" \
CFLAGS="-ffile-prefix-map=$(pwd)=/velqu-src -fdebug-prefix-map=$(pwd)=/velqu-src" \
cargo build --release -p q-bench-support --bin q-codec-bench
./target/release/q-codec-bench --out-dir benchmarks/raw/codec --iters 2000
```

The run uses 200 warmup invokes and 2,000 timed samples per candidate/case.
The raw file contains 60,000 JSONL rows: 10 cases × 3 candidates × 2,000
samples. Every one of the 30 cells reports 2,000/2,000 correct outputs.

Environment: 13th Gen Intel Core i5-13420H, Linux 7.0.0-28-generic,
rustc 1.96.0, quickjs-ng 0.15.1 through rquickjs 0.12.2, one QuickJS engine
worker, in-process and without network I/O. Bun is not involved in the
measured path.

Artifacts:

- Raw samples: `benchmarks/raw/codec/codec.jsonl`
- Summary: `benchmarks/raw/codec/codec-summary.json`
- Hash manifest: `benchmarks/raw/codec/evidence.json`
- Generated source: `crates/q-bench-support/src/bin/codec_bench/generated.rs`

Run ID: `m25-002-b-1787289561`

## Case matrix and p50/p95/p99 (microseconds)

| Case | Bytes | Candidate | p50 | p95 | p99 |
| --- | ---: | --- | ---: | ---: | ---: |
| small_user | 75 | quickjs-json | 44.2 | 100.0 | 207.7 |
| small_user | 75 | generic-rust | 44.9 | 108.0 | 259.6 |
| small_user | 75 | generated-schema | 42.6 | 99.7 | 381.1 |
| nested_order | 106 | quickjs-json | 57.0 | 109.3 | 287.5 |
| nested_order | 106 | generic-rust | 49.6 | 111.1 | 419.3 |
| nested_order | 106 | generated-schema | 47.7 | 104.5 | 179.6 |
| records100 | 4,976 | quickjs-json | 485.1 | 656.2 | 1,231.3 |
| records100 | 4,976 | generic-rust | 464.3 | 611.4 | 817.0 |
| records100 | 4,976 | generated-schema | 512.4 | 665.0 | 1,154.4 |
| records1000 | 52,726 | quickjs-json | 2,383.9 | 4,218.0 | 4,849.7 |
| records1000 | 52,726 | generic-rust | 3,139.5 | 5,532.6 | 6,256.5 |
| records1000 | 52,726 | generated-schema | 2,840.4 | 4,019.5 | 5,074.6 |
| pad_256 | 244 | quickjs-json | 22.1 | 48.9 | 78.5 |
| pad_256 | 244 | generic-rust | 17.4 | 35.5 | 62.8 |
| pad_256 | 244 | generated-schema | 17.8 | 38.2 | 59.5 |
| pad_1k | 1,012 | quickjs-json | 22.9 | 29.0 | 53.8 |
| pad_1k | 1,012 | generic-rust | 17.6 | 38.7 | 59.3 |
| pad_1k | 1,012 | generated-schema | 17.6 | 37.2 | 61.6 |
| pad_16k | 16,372 | quickjs-json | 82.3 | 105.9 | 195.5 |
| pad_16k | 16,372 | generic-rust | 21.8 | 25.0 | 29.9 |
| pad_16k | 16,372 | generated-schema | 24.3 | 44.3 | 68.9 |
| pad_64k | 65,524 | quickjs-json | 274.3 | 369.6 | 667.3 |
| pad_64k | 65,524 | generic-rust | 48.0 | 95.8 | 119.2 |
| pad_64k | 65,524 | generated-schema | 43.1 | 77.0 | 101.3 |
| opt_null | 49 | quickjs-json | 18.3 | 19.9 | 24.3 |
| opt_null | 49 | generic-rust | 17.7 | 23.8 | 25.4 |
| opt_null | 49 | generated-schema | 17.4 | 19.3 | 22.9 |
| problem_shape | 156 | quickjs-json | 18.8 | 22.1 | 31.3 |
| problem_shape | 156 | generic-rust | 18.1 | 21.8 | 24.7 |
| problem_shape | 156 | generated-schema | 17.7 | 19.0 | 23.7 |

## Interpretation and limits

- `quickjs-json` performs no schema validation. The host candidates parse,
  validate, normalize, and then cross the same QuickJS boundary; this
  asymmetry is intentional and remains visible in the report.
- The generated candidate is a benchmark-only fused projection over a
  `serde_json` value. It is not a direct byte decoder. Direct byte scanning and
  production generated codecs remain M25-003/M25-004 work.
- Optional defaults are part of host-candidate correctness: `opt_null` has
  49-byte input and 61-byte normalized host output, while QuickJS returns the
  49-byte raw object. Correctness is compared to each candidate's declared
  semantics, not by weakening validation or mutating the fixture.
- No CPU/allocation capture or global strategy selection is claimed here;
  those belong to M25-002-C/D. The matrix shows that no one strategy wins
  every shape: QuickJS leads records1000, while native candidates lead the
  padded large-object cases and generated projection leads several smaller
  validation-heavy cases.
- Normative targets and these measured values are separate. Results are scoped
  to this host, build, engine, worker count, and fixture corpus.

## Artifact hashes

From `benchmarks/raw/codec/evidence.json`:

| Artifact | SHA-256 |
| --- | --- |
| `target/release/q-codec-bench` | `c20e23fc77eb3f952019dd92214ec143900fdf9bd4264fa2fbbdc8f1cd7e549f` |
| `crates/q-bench-support/src/bin/codec_bench/generated.rs` | `630ff40ca49225738bdc6a6c934b686a5cfedeb3e5ac5a29ea9ef0f9a41ac146` |
| `benchmarks/raw/codec/codec.jsonl` | `c62f420fd0e81aee54aaf980eab7e52dc83268f41abf45d7c614bdbe3f365786` |
| `benchmarks/raw/codec/codec-summary.json` | `0c96b3b07211ec2fb9afe8dbb049185814993e4be058c512826470621171a42c` |
