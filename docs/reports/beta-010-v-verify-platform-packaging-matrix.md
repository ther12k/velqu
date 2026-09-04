# BETA-010-V — Verify Supported Beta Platform and Packaging Matrix

## Overview

Verification closure for parent task BETA-010 ("Create supported beta platform and packaging matrix").
Maps all parent acceptance criteria to source evidence, tests, and runbooks across microtasks A through E.

## Acceptance Criteria Mapping

| Acceptance Criterion | Implementation & Evidence | Status |
|---|---|---|
| **Published platform list is exact** | `docs/beta/governance/PLATFORM_SUPPORT.md`: Linux x86_64 glibc is the only supported public beta promise. Captured host transcript (Linux 7.0 Ubuntu 24.04.1 x86_64, GLIBC 2.39, Rust 1.96.0 pinned, Bun 1.4.0) in `docs/reports/beta-010-a-linux-x86-64-glibc-platform.md`. | PASS |
| **Unsupported platforms fail with guidance** | `docs/beta/governance/PLATFORM_SUPPORT.md` and `docs/reports/beta-010-b-linux-arm64-glibc-ci.md` define boundaries: ARM64 remains conditional CI-only; macOS is development-only best-effort; Windows, musl, and other architectures are unsupported. | PASS |
| **Packages contain no accidental source/compiler artifacts** | `scripts/npm-package-inventory.sh` and `docs/reports/beta-010-c-npm-package-inventory.json`: all 9 `@velqu/*` packages are explicitly `"private": true`, preventing unauthorized publishing. Multi-stage Dockerfile copies only release binary and QPack into final image. | PASS |
| **Install works in clean environment** | `scripts/clean-install-test.sh`: installs release `velqu-runtime` and `app.qpack` in a pristine isolated temporary directory with zero repository state; passes fingerprint compatibility check, serves HTTP (`/health/live`, `/hello/clean-env`), and exits cleanly on SIGTERM. Emits `CLEAN-INSTALL-TEST-OK` (`docs/reports/beta-010-e-clean-install-tests.md`). | PASS |

## Toolchain & Runtime Suite

- `scripts/qpack-tools-inventory.sh`: automated check across all 5 runtime/tool surfaces (`velqu-runtime`, `velqu-standalone`, `velqu-bytecode`, `velqu pack inspect`, `velqu pack migrate`). Emitted `docs/reports/beta-010-d-qpack-tools-inventory.json` with verdict `PASS`.
- `scripts/artifact-smoke.sh`: verifies shared and standalone mode binary execution, engine mismatch fail-closed rejection, and cold start timing. Result: `SMOKE-OK`.
- `scripts/proxy-smoke.sh`: verifies loopback reverse-proxy bind and clean exit. Result: `PROXY-SMOKE-OK`.
- `scripts/clean-install-test.sh`: verifies clean environment execution. Result: `CLEAN-INSTALL-TEST-OK`.

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

- Verification-only task; no runtime behavior changed.
- Linux x86_64 glibc is the sole public beta platform; ARM64 remains conditional until hosted CI and owner acceptance are complete.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
