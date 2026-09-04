# BETA-012-A — Installation

## Overview

Rewrote `docs/beta/INSTALL.md` as the accurate, test-backed installation guide for the public beta. The previous version documented only shared mode and ended with a stale claim that standalone deployment "is not part of" the beta — standalone has shipped since BETA-010-D/M26-009.

## Document structure

1. **Mode table**: shared (`velqu-runtime` + `app.qpack`) vs standalone (`velqu-standalone`, pack embedded), with the swap-app-without-rebuild trade-off stated.
2. **Beta scope reminder**: Linux x86_64 glibc target, non-SLA, no production-readiness claim, API may change between betas (links `docs/beta/01_BETA_DEFINITION.md`).
3. **Prerequisites**: source-based distribution; `@velqu/*` npm packages prepared but unpublished (`private`); Rust stable + Bun 1.4 as build tooling only (production execution is Rust + quickjs-ng 0.15.1 via rquickjs 0.12.2).
4. **Shared mode**: `bun install --frozen-lockfile` → `cargo build --release -p velqu-runtime` → CLI pack build → run + curl samples; engine-mismatch/SEC-001 exact-match failure behavior; loopback `proxyMode: "reverse-proxy"` default and `"configVersion": 1` requirement.
5. **Standalone**: exact `VELQU_STANDALONE_PACK="$(realpath …)" cargo build --release -p velqu-runtime --features standalone` command; startup verification identical; `"mode":"standalone"` ready line.
6. **Container**: `docker build` + corrected `docker run` sample (in-container `VELQU_HOST=0.0.0.0` + `VELQU_PROXY_MODE=direct`, host publish `127.0.0.1:8080:3000`) with the loopback-boundary explanation; `docker-compose.beta.yml` and `scripts/container-smoke.sh` references.
7. **Updating table** (shared mode), **Limits** (bounded defaults), **accuracy notes**: QPack bytecode improves startup and verification but is not native-machine-code JIT; no universal performance claim; measured numbers under `benchmarks/`.

## Guardrail compliance

- **Every command/sample is tested** — all samples below were executed in this worktree on 2026-09-04:
  - `bun install --frozen-lockfile`, `cargo build --release -p velqu-runtime`, `bun packages/cli/src/index.ts build --project examples/proof` → `dist/app.qpack` (72,116 bytes) — OK.
  - Shared mode: `velqu-runtime --pack examples/proof/dist/app.qpack --port 8080` → `/health/live` = `{"status":"ok"}`, `/hello/beta` = `{"message":"Hello beta"}` — OK.
  - Standalone: feature build → `velqu-standalone --port 8081` → `/health/live` = `{"status":"ok"}` — OK.
  - Container: `docker build -t velqu-runtime .` + the documented `docker run` (env overrides, loopback publish) → `/health/live` = `{"status":"ok"}`, `/hello/beta` = `{"message":"Hello beta"}` — OK.
  - (Test-harness note: an initial docker sample in the draft omitted the env overrides and could not work; the doc now shows the working invocation, which is what was tested.)
- **Link check** — every path referenced in the doc exists (`docs/beta/01_BETA_DEFINITION.md`, `docker-compose.beta.yml`, `scripts/container-smoke.sh`); no markdown-relative links are broken.
- **No universal performance claim** — accuracy notes direct measured claims to `benchmarks/` raw evidence.
- **No production-ready/SLA wording** — beta scope reminder states non-SLA and no production-readiness claim.
- **QuickJS bytecode vs JIT explained accurately** — explicit statement that bytecode is not native-machine-code JIT compilation.

## Docs CI / gates

- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Documentation change only; no runtime behavior modified.
- Container sample tested against the local docker daemon; image publication remains Owner-gated.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
