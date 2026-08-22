# M25-010-B — Generated code/pack size evidence

Raw data: `benchmarks/raw/sizes-m25-010-b/sizes.json` (byte sizes +
sha256 for every artifact, environment recorded).

## Environment

rustc 1.96.0 (ac68faa20 2026-05-25); release builds with
`--remap-path-prefix=$PWD=/velqu-src` (hash-stable across checkout
paths). Sizes are toolchain-dependent — recorded, not normative.

## Pack artifacts (proof app, 9 routes)

| artifact | bytes |
|---|---|
| app.qpack | 61582 |
| build-report.json | 13306 |
| build-report.md | 1575 |
| capability-manifest.json | 320 |
| contract.d.ts | 2018 |
| contract.json | 5511 |
| contract.lock.json | 5488 |
| contract.meta.json | 1459 |
| openapi.json | 9052 |
| route-manifest.json | 5069 |
| schema-manifest.json | 3351 |

Total dist output: **108,731 bytes**; the runtime
consumes `app.qpack` (**61,582 bytes**, ~6,842 bytes
per route including schemas, router tables, and the bundle). The
generated codec programs are compiled AT BUILD TIME into the pack's
schema IR — decoders/encoders add ZERO pack bytes (the runtime compiles
`DecoderTable`/`EncoderTable` from the same manifest at startup);
`contract.meta.json` (the M25-008-C published metadata) adds
1459 bytes.

## Generated codec module (C2 benchmark prototype)

`crates/q-bench-support/src/bin/codec_bench/generated.rs`: **60,497 bytes** (sha256 630ff40ca4922573…) —
benchmark-only; production codecs are compiled from the pack's IR, not
checked-in generated sources.

## Binaries

| binary | bytes |
|---|---|
| velqu-runtime (engine + all codecs) | 5,145,976 |
| raw-rust baseline (no engine/codecs) | 0 |

The runtime at 5.15 MB embeds quickjs-ng plus every
codec table constructor; the raw baseline isolates the HTTP/host shell.
There is no per-route code generation in the binary — route count
scales the PACK, not the executable.

## Decision-matrix impact

Size overhead of the codec work: zero per-route pack bytes (IR-derived
programs), zero binary growth per route, and one fixed compile-time
table construction at startup (bounded by the schema manifest — cold
start delta measured in M25-010-C). Reports match raw data; all sha256s
in sizes.json.
