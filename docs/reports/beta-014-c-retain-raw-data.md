# BETA-014-C — Retain Raw Data

## Overview

Documents and verifies the raw data retention policies and deterministic archive mechanisms used across Velqu benchmark suites:
- Every measurement claim must be traceable to unaggregated, raw individual request samples (`.jsonl`).
- Raw archives are compressed deterministically with pinned modification times (`deterministicGzip`), ensuring byte-identical reproduction across independent runs.
- Lossless verification: exact row counts and raw SHA-256 digests are maintained in retention manifests (`RETENTION.md`, `benchmarks/manifest.json`).

## Retention Architecture & Invariants

1. **Deterministic Compression**: `deterministicGzip` zeroes out volatile OS timestamps and metadata headers, guaranteeing that identical input rows generate identical compressed byte streams.
2. **Lossless Verification**: `verifyArchive` decompress and verifies the raw payload hash against the recorded manifest digest, rejecting any bit-level drift or truncation.
3. **No Cherry-Picking or Truncation**: All failed requests, connection errors, and timeout events are captured in the raw samples and reported in distributions ($n$, mean, p50, p95, p99), never discarded.
4. **Committed Raw Samples Inventory**:
   - `benchmarks/raw/cold-start/g0-cold-1787214119.jsonl`: per-process cold-start request rows.
   - `benchmarks/raw/warm/g0-warm-1787214167.jsonl`: steady-state warm request rows.
   - `benchmarks/raw/worker-scaling/soak.jsonl`: 30-second window samples over millions of requests.
   - `benchmarks/raw/route-count/`: route scaling latency samples (1 to 10,000 routes).
   - `benchmarks/raw/profiles/startup-10000.alloc.json`: exact allocator instrumentation counts.

## Testing & Verification

- `benchmarks/real-world/retain.test.ts`: 5 passed, 0 failed.
  - Verified `deterministicGzip` byte identity across runs.
  - Verified lossless round-trip decompression and exact row count verification.
  - Verified `verifyArchive` accepts matching hash and rejects tampered data.
  - Verified manifest creation and archive hash agreement.
  - Verified identical input rows reproduce identical `.jsonl.gz` bytes.
- `python3 scripts/validate-benchmark-evidence.py` — PASS.
- `./scripts/validate-okf` — PASS.

## Gates

- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Raw data retention verification only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
