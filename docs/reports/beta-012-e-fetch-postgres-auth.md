# BETA-012-E — Fetch/Postgres/auth

## Overview

Completed the beta documentation for the three capability surfaces:

- **Fetch** (`docs/beta/FETCH-CAPABILITIES.md`): replaced the broken `/tmp` scaffold sample (fails fail-closed on the pinned-toolchain check outside the monorepo) with the tested in-checkout flow (`create --with-fetch` + workspace links + `check` + `build`), documented the capability manifest `perRoute` echo, and updated both private-alpha framings to public beta.
- **Postgres** (`docs/beta/POSTGRES-CAPABILITY.md`): audited — already beta-accurate (normative BETA-004 deliverable: exact-match linking, fail-closed startup, no-ORM `sql()` surface, bounded pool, safe release); no changes required.
- **Auth** (new `docs/beta/AUTH.md`, indexed in `docs/beta/INDEX.md`): route-level policy model (typed sessions, declared failure statuses visible in contract/Treaty), identity-from-verified-tokens-not-headers rule, the tested proof login/profile fixture flow, the JWT reference package's five fail-closed HS256 gates summary, redaction note, and non-SLA framing.

## Every command/sample tested (2026-09-04, this worktree)

- Fetch scaffold: `create fetch-demo --with-fetch` → workspace links → `check --project fetch-demo` ("4 routes … clean") → `build` → capability manifest `perRoute` includes `upstream.quote`. Scaffold dir removed before commit.
- Auth flow against the running proof runtime (actual responses now shown in the doc):
  - `POST /auth/login` with the fixture secret → token issued (176 chars).
  - `GET /auth/profile` with `Bearer <token>` → `{"scope":"items:read profile:read","userId":"usr_ada"}`.
  - Wrong secret → 401; missing token → 401.
  - (Testing caught a wrong fixture secret in the first draft — corrected to `jwt-reference-demo-secret` from source, and the response key order matched to actual output.)
- Proof pack build + typecheck.

## Link check

`AUTH.md` references `examples/proof`, `packages/capability-auth-jwt/README.md`, and `docs/beta/CAPABILITY_AUTHORS.md` — all exist; indexed from `docs/beta/INDEX.md`.

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
- The doc's fetch fixture note is explicit: the upstream route is educational; egress policy is a deployment concern.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
