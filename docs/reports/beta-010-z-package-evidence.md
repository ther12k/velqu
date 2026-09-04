# BETA-010-Z — Package Evidence for Create Supported Beta Platform and Packaging Matrix

## Overview

Final evidence packaging for parent task BETA-010 ("Create supported beta platform and packaging matrix").
All microtasks A through E and verification task V are complete and verified against source, scripts, and runtime binaries.
Flips parent task BETA-010 to `PASS` in `docs/beta/04_TASK_LEDGER.md`.

## Parent Acceptance Criteria Matrix

| Criterion | Canonical Evidence / Report | Status |
|---|---|---|
| **Published platform list is exact** | `docs/beta/governance/PLATFORM_SUPPORT.md` & `docs/reports/beta-010-a-linux-x86-64-glibc-platform.md`: Linux x86_64 glibc is the sole public beta platform promise. Host toolchain transcript verified. | PASS |
| **Unsupported platforms fail with guidance** | `docs/reports/beta-010-b-linux-arm64-glibc-ci.md`: ARM64 recorded as conditional CI portability signal; macOS development-only; Windows/musl unsupported. | PASS |
| **Packages contain no accidental source/compiler artifacts** | `scripts/npm-package-inventory.sh` & `docs/reports/beta-010-c-npm-package-inventory.json`: 9 `@velqu/*` packages marked `private: true`, 0 publishable packages. Dockerfile copies only release binaries/QPack into final slim image. | PASS |
| **Install works in clean environment** | `scripts/clean-install-test.sh` & `docs/reports/beta-010-e-clean-install-tests.md`: verified clean directory execution of `velqu-runtime` and `app.qpack` without repo or dev dependencies (`CLEAN-INSTALL-TEST-OK`). | PASS |

## Tools & Runtime Packaging Evidence

- `scripts/qpack-tools-inventory.sh` & `docs/reports/beta-010-d-qpack-tools-inventory.json`: validated `velqu-runtime`, `velqu-standalone`, `velqu-bytecode`, `velqu pack inspect`, and `velqu pack migrate`.
- `scripts/artifact-smoke.sh`: validated runtime fingerprint, engine mismatch rejection, cold start p50 timing, and standalone mode execution (`SMOKE-OK`).
- `scripts/proxy-smoke.sh`: validated reverse proxy loopback posture and graceful shutdown (`PROXY-SMOKE-OK`).
- `scripts/clean-install-test.sh`: validated clean environment install and execution (`CLEAN-INSTALL-TEST-OK`).

## Gates

- `cargo test -p q-pack` — pass
- `cargo test -p q-engine-quickjs` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Evidence packaging only; no runtime code modified.
- Public beta platform scope is strictly Linux x86_64 glibc.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
