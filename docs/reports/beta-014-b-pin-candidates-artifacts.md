# BETA-014-B — Pin All Candidates and Artifacts

## Overview

Verifies that all competitor candidates, runtimes, drivers, container images, and benchmark artifacts are strictly pinned to exact, immutable versions and SHA-256 hashes without ranges, carets (`^`), or floating tags.

## Candidate & Toolchain Pins

Sourced from `benchmarks/real-world/versions.json` (format: `velqu-realworld-versions-v1`):

| Component | Pinned Version / Artifact | Enforcement |
|---|---|---|
| **Velqu** | `workspace:0.1.0` (commit-pinned) | Source repository commit hash |
| **Elysia** | `2.0.0-beta.4` | `versions.json`, `candidates/package.json`, frozen `bun.lock` |
| **Hono** | `4.13.5` | `versions.json`, `candidates/package.json`, frozen `bun.lock` |
| **Fastify** | `5.12.1` | `versions.json`, `candidates/package.json`, frozen `bun.lock` |
| **PostgreSQL** | `postgres:17.5-alpine3.22` | `compose.yaml` and `versions.json` |
| **Bun** | `1.4.0` | CI workflow `.github/workflows/verify.yml` and `versions.json` |
| **Node.js** | `Node 22 LTS` (`nodeLtsMajor: 22`) | Runtime engine check in candidate driver |
| **Database Drivers** | `pg@8.23.0`, `postgres@3.4.9` | Frozen `bun.lock` exact entries |

## Benchmark Artifact Digest Pins

Canonical digest mapping in `benchmarks/manifest.json`:
- All QPack benchmark artifacts (`app-25.qpack` through `app-10000-bc.qpack`) pinned by SHA-256 and byte sizes.
- Runtime binaries pinned by reproducible build path prefix remapping (`RUSTFLAGS="--remap-path-prefix=$(pwd)=/velqu-src"`).
- Verified via `python3 scripts/validate-benchmark-evidence.py` and `./scripts/validate-okf`.

## Testing & Guardrails

- `benchmarks/real-world/versions.test.ts`: 9 passed, 0 failed.
  - Manifest format verified (`velqu-realworld-versions-v1`).
  - No ranges or wildcard prefixes in any candidate/driver version.
  - Registry pins match frozen `bun.lock` and `compose.yaml` exactly.
- `cargo test -p q-pack`: 100 unit + 2 fuzz tests passed.
- `bun run typecheck`: passed.
- `./scripts/validate-okf`: passed (manifest hashes PASS, internal links PASS).

## Gates

- `cargo test -p q-pack` — pass (100+2)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Version and artifact pinning verification only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
