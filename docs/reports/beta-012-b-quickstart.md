# BETA-012-B — Quickstart

## Overview

Rewrote `docs/beta/QUICKSTART.md` from its private-alpha framing to the public-beta reality, replacing a scaffold flow that **no longer works** (the old doc instructs `init /tmp/velqu-hello` then building from there — this fails fail-closed with "toolchain mismatch — byte-identical packs require the pinned toolchain (typescript 7.0.2 != pinned 5.9.3)" because a scaffold outside the monorepo cannot resolve the pinned toolchain) with the tested in-checkout flow.

## Changes

- **Beta framing**: title/status updated from "private alpha" to public beta; platform statement corrected from "Linux or macOS" to Linux x86_64 glibc beta target with macOS development-only.
- **Working scaffold flow**: `create` inside the checkout + explicit `node_modules/@velqu/` workspace links (core/schema/treaty) — this is the flow that actually builds; the create command's own note recommends exactly this.
- **Every command/sample executed** on 2026-09-04 in this worktree (actual responses shown in the doc):
  - `bun install --frozen-lockfile`, `cargo build --release -p velqu-runtime` — OK.
  - `create hello-velqu --name hello-velqu` — scaffold generated (health, greetings module + service + test, Treaty client); workspace links created; `build --project hello-velqu` → full dist including `app.qpack`, OpenAPI, lock.
  - `create hello-svc --profile service:4` + build — OK (service:N grammar documented, bare `service` invalid).
  - Runtime run: `/health/live` = `{"status":"ok"}`, `/greetings/world` = `{"message":"Hello, world!"}` — OK.
  - Dev loop: `dev --project hello-svc --port 8084` → `/health/live` = `{"status":"ok"}`, `/greetings/dev` = `{"message":"Hello, dev!"}` — OK.
  - `inspect --project hello-velqu --json` reference (CLI's own command surface).
- **Guardrail language added**: bytecode-not-JIT statement ("improves startup and enables strict verification, but is not native-machine-code JIT compilation"), no-SLA/no-production-readiness, no performance implications (claims require matched raw samples), trusted-code-only sandbox statement retained, bounded `defer` note retained.
- **Navigation**: links to `01_BETA_DEFINITION.md`, `02_SCOPE_MATRIX.md`, `INSTALL.md` (link check OK; doc already indexed by `docs/beta/INDEX.md` and `docs/beta/README.md`).

## Link check

All referenced paths exist: `docs/beta/01_BETA_DEFINITION.md`, `docs/beta/02_SCOPE_MATRIX.md`, `docs/beta/INSTALL.md`.

## Gates

- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Documentation change only; no runtime behavior modified.
- Scaffold test directories were removed before commit; no generated artifacts shipped.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
