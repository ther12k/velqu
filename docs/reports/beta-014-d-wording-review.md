# BETA-014-D — Benchmark Wording Review

## Review Result

**PASS** after correcting the canonical report against the current committed evidence.

Reviewed:
- `docs/reports/beta-014-a-canonical-benchmark-report.md`
- `benchmarks/raw/cold-start/summary.json`
- `benchmarks/raw/warm/summary.json`
- `benchmarks/raw/ramp/summary.json`
- `benchmarks/raw/worker-scaling/soak-summary.json`
- `benchmarks/raw/ramp/losses.json`
- `docs/reports/beta-003-d-honest-losses.md`
- `docs/beta/PERFORMANCE-METHODOLOGY.md`

## Corrections Applied

1. Replaced the unsupported warm table with the exact median-across-five-repetitions values from `benchmarks/raw/warm/summary.json`, including candidate, route class, latency percentiles, throughput, and same-cell RSS. The report now states that the current warm fixture contains C0–C3 only.
2. Replaced stale/historical C2 wording (`59 µs vs 37 µs`, 1.59×) with the current ramp evidence: Velqu C0 steady p50 55 µs versus the 24 µs class best (2.29×), and no Velqu overtake of raw-rust within the recorded 100-request C0/C2 horizon.
3. Removed unsupported idle/peak RSS and Node/Fastify rows. Added source-backed median RSS snapshots for C0–C3 and an explicit statement that these are not memory ceilings or cost predictions; Fastify is pinned but absent from this warm fixture.
4. Preserved required caveats: local process measurements are not cloud cold-start claims; comparisons are fixture-specific; the public beta is non-SLA; losses are reported alongside wins.

## Evidence and Checks

- Raw archive paths remain listed and indexed by `benchmarks/manifest.json`.
- `benchmarks/real-world/retain.test.ts` — 5 passed, 0 failed.
- `benchmarks/real-world/versions.test.ts` — 9 passed, 0 failed.
- `bun test` — 434 passed, 0 failed (67 files).
- `bun run typecheck` — pass.
- `./scripts/validate-okf` — pass.

## Disclosure

This was a documentation and evidence correction only; no runtime behavior changed. The report does not make universal performance, cloud cold-start, production-readiness, SLA, or cost claims.
