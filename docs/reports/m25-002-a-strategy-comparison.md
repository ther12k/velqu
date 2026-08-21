---
type: Evidence Report
title: Codec Strategy Comparison (M25-002-A)
status: complete
milestone: M25
---

# Codec strategy comparison — QuickJS parse/stringify vs generic Rust vs generated projection (M25-002-A)

Raw data: `benchmarks/raw/codec/codec.jsonl` (18,000 rows — one per timed
sample) + `benchmarks/raw/codec/codec-summary.json` (per-cell stats) +
`benchmarks/raw/codec/evidence.json` (sha256 manifest). Command:
`./target/release/q-codec-bench --out-dir benchmarks/raw/codec --iters 2000`
(warmup 200 per cell; correctness asserted per sample — 2000/2000 correct in
all nine cells). Release build produced with the repository's reproducible
path-remapping flags (`scripts/benchmark` conventions).

Environment: 13th Gen Intel Core i5-13420H, Linux 7.0.0-28-generic, rustc
1.96.0, pinned engine quickjs-ng 0.15.1 via rquickjs 0.12.2, single engine
worker, in-process (no network), Bun not involved in the measured path.

## Candidates (input direction)

| Candidate | Parse | Validate/project | JS boundary | Response |
| --- | --- | --- | --- | --- |
| `quickjs-json` | engine `JSON.parse` (lazy `ctx.json()`) | none | raw bytes in | engine `JSON.stringify` |
| `generic-rust` | `serde_json` | generic tree-walk (`q_schema_runtime::validate`) | recursive object construction | native traversal |
| `generated-schema` | `serde_json` | generated fused decode/validate projection (`codec_bench/generated.rs`) | recursive object construction | native traversal |

The generated decoder is produced by
`crates/q-bench-support/src/bin/codec_bench/generator.rs` from the frozen
Schema IR v2 corpus (`small_user`, `nested_order`, `records100`); the emitted
file and generator output are locked byte-identical by the
`generated_source_is_current` test, and
`differential_decode_matches_generic_validator` proves error-for-error parity
with the generic validator (same paths, codes, messages) across the valid
fixtures and 12 invalid mutations.

## Fairness and scope (read before citing numbers)

- `quickjs-json` performs **no schema validation**; both host candidates fully
  validate inputs. That asymmetry is the strategy question itself, not a flaw.
- `generated-schema` shares `generic-rust`'s serde_json parse and QuickJS
  boundary, so its delta isolates validation/projection only.
- **Prototype boundary**: `generated-schema` is a projection over the parsed
  JSON value. The direct byte scanner/decoder (no intermediate `serde_json`
  tree) is the M25-003/M25-004 deliverable and is NOT measured here.
- No CPU/allocation capture in this packet (M25-002-C). No strategy is
  selected here and no compiler decision rule changes (M25-002-D).
- Payload matrix is the three frozen shapes above; the 256B/1KB/16KB/64KB,
  arrays-1,000, optional/null-heavy, and problem shapes are M25-002-B.

## Results (μs per invoke→outcome round trip, lower is better)

| Case (bytes) | quickjs-json p50 | generic-rust p50 | generated-schema p50 | gen vs generic | gen vs quickjs |
| --- | ---: | ---: | ---: | ---: | ---: |
| small_user (75 B) | 54.5 | 46.5 | 33.4 | **−28%** | **−39%** |
| nested_order (106 B) | 47.5 | 24.6 | 22.7 | −8% | −52% |
| records100 (4,976 B) | **250.3** | 268.1 | 262.8 | −2% | +5% |

Tails (p95/p99, μs): small_user — quickjs 147.5/469.9, generic 88.9/135.2,
generated 67.4/100.4; nested_order — quickjs 65.3/92.4, generic 46.5/76.1,
generated 42.6/48.1; records100 — quickjs 451.5/512.2, generic 405.6/475.6,
generated 449.4/500.8.

## Observations (scope-limited to this host/engine/shapes)

- The generated projection clearly wins the validation/projection stage on
  small and nested objects (fused typed access, no per-node enum dispatch or
  error-vec plumbing on the happy path).
- On the 100-record array the three strategies are within ~7% of each other
  and `quickjs-json` is fastest: the array path is dominated by serde parse,
  output-tree construction, and JS object building, which all three share or
  (for quickjs) avoid entirely. No single strategy wins across shapes — the
  parent acceptance criterion "no single strategy is forced globally" is
  supported by this evidence.
- `quickjs-json`'s small-user p99 (469.9 μs) reflects engine-side parse
  jitter; its records100 lead comes with zero validation work included.

## Decision status

**None.** This packet records measurement evidence only. Strategy selection
per route shape, compiler decision rules, and fallback-cost visibility are
M25-002-C/D outputs; production decoders/encoders are M25-003–M25-005.

## Artifact hashes (`benchmarks/raw/codec/evidence.json`)

| Artifact | sha256 |
| --- | --- |
| `target/release/q-codec-bench` | `d606ffc6fb8512f36c114d4a7fa033aa27346a6e94311c46fb86fbff0e4fa691` |
| `crates/q-bench-support/src/bin/codec_bench/generated.rs` | `ab091cb2d03b3ceaa9c39c712d86567cd9d22ee2e4c3f121753e41fb3c829611` |
| `benchmarks/raw/codec/codec.jsonl` | `9471af450d5b0044216282ed4a06ba5c6afbd74272e843797eb100627290eb20` |
| `benchmarks/raw/codec/codec-summary.json` | `0b484d122d0ab5db05f7a8aa3a3cef233ef50c8669b02aab95ef141efbe15ab1` |

Run id: `m25-002-a-1787288528`.
