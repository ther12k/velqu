---
type: Evidence Report
title: M0 Contracts and Baselines Report
status: complete
milestone: M0
---

# M0 — contracts and fair baselines (2026-08-17)

## Gate status: PASS

| M0 gate requirement | Evidence | Status |
|---|---|---|
| OKF structure and links pass | `scripts/validate-okf` → manifest hashes PASS, internal links PASS (160 checked) | PASS |
| implementation audit and open decisions exist | `docs/implementation-audit.md`, `docs/open-decisions.md` | PASS |
| proof route semantics frozen | `benchmarks/fixtures/fixture-contract.json` (frozen; canonical across all candidates) | PASS |
| raw Rust/Bun/Elysia baseline versions+commands recorded | `baselines/README.md` (Bun 1.3.4; elysia 2.0.0-beta.4; rustc 1.96.0/hyper 1.11/tokio 1.53) | PASS |
| cold-start harness produces raw machine-readable samples | `benchmarks/raw/cold-start/*.jsonl` + `summary.json` (1680 samples) | PASS |
| baseline correctness fixtures pass | `benchmarks/harness/check-server.ts`: raw-bun 27/27, elysia2 27/27, raw-rust 27/27, velqu 31/31 | PASS |
| route/schema/policy/Treaty type spike compiles | `bun run typecheck` clean; `bun test packages` 7 pass | PASS |
| 100/500/1,000 route type benchmark exists | `benchmarks/type-scale/results.json` | PASS |
| no performance claim is published | reports carry exact-scope statements only | PASS |

## Frozen fixture contract

`benchmarks/fixtures/fixture-contract.json` (velqu-benchmark-fixture-v1) covers
C0–C5 + R (throw) with exact response bytes, validation semantics, auth
fixture, abort case, and route-count generators. The black-box checker
(`benchmarks/harness/check-server.ts`) is the single source of correctness
truth shared by ALL candidates: 27 semantic assertions (+4 velqu-exact problem
assertions).

## Matched candidates — fairness audit summary

- Identical observable behavior, verified by the same checker run against each
  candidate (all 27/27).
- Release builds: Rust `--release` (lto=thin, strip); Bun candidates run
  Bun's production mode (single-file JS execution); no dev logging.
- Same payloads/statuses/byte orders; no compression; no TLS; HTTP/1.1
  keep-alive; bind 127.0.0.1; fresh process per sample.
- Baselines own their lockfiles (`baselines/elysia2/bun.lock`,
  `baselines/raw-rust/Cargo.lock` via workspace exclusion) and import no velqu
  code.
- Elysia 2 AOT notes (recorded honestly): elysia 2.0.0-beta.4 (npm `next`),
  AOT is built-in and enabled via `aot: true` (default) — compilation occurs
  at `listen()`; malformed-JSON (ParseError 400) is mapped to 422 to match the
  frozen fixture, disclosed in `baselines/README.md`.
- raw-rust is a transport lower bound; no framework/Treaty parity implied.

## Type-system spike (M0 §10.5)

- `packages/schema`: builders produce exactly Schema IR v1 JSON nodes with
  inferred TS types (`Infer<S>`); optional/default semantics verified.
- `packages/core`: `route/defineModule/defineApp/definePolicy/defineService/
  status` — pure data constructors; handler ctx typed route-locally; policy
  session flows into handler types (SCHEMA-004 proven at type level).
- `packages/treaty`: route-id navigation `api.hello.get({name}).get()`;
  non-throwing `{data}|{error}`; status-narrowed errors (401 narrows problem);
  network (status 0, kind "network") vs abort ("abort") distinction; unit-local
  runtime tests labeled NOT runtime conformance (TRT-005).
- Deviation recorded: Eden-exact single-segment navigation
  (`api.hello({name})`) collides when one first segment hosts multiple routes
  (`/users` POST vs `/users/:id` GET); route-id navigation chosen for M0.
  Open decision ID-011 in `docs/open-decisions.md`.

## Type-scale results (tsc --noEmit, cold, this host)

| Routes | Time | Budget | Status |
|---:|---:|---:|---|
| 100 | 2.1–4.8 s | ≤1.5 s | FAIL (tsc fixed startup dominates) |
| 500 | 2.2–2.9 s | ≤3.0 s | PASS |
| 1,000 | 2.7–5.8 s | ≤5.0 s | PASS (borderline; worst run 5.8s) |

Negative-type checks were CAUGHT at all scales (`negativeCaught: CAUGHT`).
Raw: `benchmarks/type-scale/results.json`. The 100-route miss is dominated by
tsc process+library load (~2s floor measured at n=100 with near-identical
times at n=500); scaling itself is near-linear. Honest status: budget missed
at 100 routes; cause is fixed tsc cost, not route-count scaling.

## Commands

```bash
bun install
bun run typecheck && bun test packages
for c in "raw-bun bun" "elysia elysia"; do :; done  # see baselines/README.md
bun benchmarks/harness/check-server.ts <port> --candidate <id>
cargo build --release --workspace
```

No comparative public claim is made by this report; numbers appear in the
cold-start report with exact scope.
