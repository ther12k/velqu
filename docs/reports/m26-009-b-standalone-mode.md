# M26-009-B — Standalone deployment mode: measured evidence

Standalone mode (`velqu-standalone`, feature `standalone`) embeds the
verified pack at compile time (`include_bytes!` via
`VELQU_STANDALONE_PACK`). Both modes run the IDENTICAL
load-verify-serve pipeline (`velqu_runtime::run`); they differ only in
`PackSource` (`Path` vs `Embedded`). The embedded artifact is the exact
verified build output and is STILL fully verified at startup via
`QPack::verify_from_slice` — embedding grants no trust.

## Commands

```bash
cargo build --release -p velqu-runtime
VELQU_STANDALONE_PACK=examples/proof/dist/app.qpack \
  cargo build --release -p velqu-runtime --features standalone
```

Measurements: 10 fresh processes per mode (sequential, same host,
release builds), `startupMs` from each process's own `ready` line and
`VmRSS` from `/proc/<pid>/status` sampled immediately after ready.
Raw generator: this report's numbers were produced by a one-shot
Python harness (spawn → read ready → read VmRSS → SIGTERM) retained in
the packet record; samples below are the complete raw set.

## Artifact sizes

| artifact | bytes |
|---|---|
| `velqu-runtime` (shared) | 5,201,208 |
| `velqu-standalone`       | 5,224,216 |

Delta +23,008 B ≈ the embedded proof pack (24,414 B on disk; page
alignment absorbs the rest).

## Cold start (startupMs, n=10 per mode)

| mode | min | p50 | p95 | max |
|---|---|---|---|---|
| shared | 2.314 | 3.500 | 4.592 | 4.913 |
| standalone | 2.344 | 2.976 | 3.780 | 3.928 |

Raw shared: 4.913, 3.968, 2.314, 2.570, 4.200, 4.046, 3.403, 3.339,
3.225, 3.598.
Raw standalone: 3.928, 2.344, 3.004, 2.670, 3.068, 2.778, 2.949, 3.599,
2.454, 3.011.

## RSS after ready (VmRSS kB, n=10 per mode)

| mode | min | p50 | p95 | max |
|---|---|---|---|---|
| shared | 7,112 | 7,236 | 7,382 | 7,404 |
| standalone | 6,996 | 7,124 | 7,199 | 7,204 |

Raw shared: 7252, 7112, 7220, 7404, 7152, 7332, 7356, 7340, 7216, 7132.
Raw standalone: 7120, 7128, 7188, 7100, 7204, 7192, 7112, 7140, 7120,
6996.

## Reading (no extrapolation)

- Both modes serve the SAME pack with identical route counts (9) and
  identical answers (`/health/live`, `/hello/:name` verified by the
  extended `scripts/artifact-smoke.sh`).
- At n=10 on one host, standalone's p50 startup is ~0.5 ms lower and
  p50 RSS ~112 kB lower; the distributions overlap heavily. This is a
  same-host sanity delta, not a portability claim. Route-count scaling
  evidence is M26-010.
- Host: this machine, release profile, no CPU pinning; environment
  recorded in `benchmarks/manifest.json`.

## Guardrail mapping

- Both modes pass identical conformance — the 28 runtime conformance
  tests drive shared mode; the smoke script drives both modes over the
  same pack and asserts identical route answers and `mode` telemetry.
- Standalone contains no compiler toolchain — the bin links only the
  runtime pipeline (`velqu_runtime` lib); no Bun, no TypeScript, no
  route/schema/OpenAPI compilation (G-004 preserved).
- Shared mode rejects mismatched runtime — unchanged (M26-009-A smoke
  step 3; fingerprint checks run in both modes before ready).
- Startup/RSS differences are measured — tables above.
