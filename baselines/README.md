# Baselines — matched benchmark candidates

All candidates implement the SAME frozen observable behavior
(`benchmarks/fixtures/fixture-contract.json`), verified by the canonical
checker `benchmarks/harness/check-server.ts` (27 assertions each, all PASS):

| Candidate | Stack | Version (pinned) | Checker |
|---|---|---|---|
| raw-bun | Bun.serve, zero deps | Bun 1.3.4 | 27/27 PASS |
| elysia2 | Elysia AOT on Bun | elysia 2.0.0-beta.4 (npm `next`) | 27/27 PASS |
| lugas | LugasJS on Bun (Standard Schema/zod) | lugas 0.1.0-beta.5, zod 3.25.76 | 27/27 PASS |
| raw-rust | hyper 1 + tokio, hand-rolled routing | rustc 1.96.0, hyper 1.11, tokio 1.53 | 27/27 PASS |

## Commands

```bash
# raw-bun
PORT=3000 bun baselines/raw-bun/server.ts

# elysia2 (own lockfile at baselines/elysia2/bun.lock)
cd baselines/elysia2 && bun install
PORT=3000 bun server.ts

# lugas (own lockfile at baselines/lugas/bun.lock)
cd baselines/lugas && bun install
PORT=3000 bun baselines/lugas/server.ts

# raw-rust (own Cargo.lock — excluded from the workspace)
cd baselines/raw-rust && cargo build --release
PORT=3000 ./target/release/velqu-baseline-raw-rust

# verify any candidate
bun benchmarks/harness/check-server.ts 3000 --candidate <bun|elysia|lugas|rust|velqu>
```

All candidates honor `PORT` and `N_ROUTES` (absent/0 = canonical fixture;
25|1000 = generated `GET /res{i}/item/:id` item routes, measured route
`GET /res7/item/7` → `{"id":7,"n":N}`); `N_ROUTES` is not yet implemented
on lugas (fixture-only).

## Lugas notes (honest recording)

- Pinned `lugas@0.1.0-beta.5` (npm dist-tag `beta`), zod `3.25.76` for
  Standard Schema validation (lugas is zero-runtime-dep; validators are
  consumer-supplied — its idiomatic path per the project README).
- Timed routes (C0–C3) use lugas's declared-schema fast path
  (`params`/`query` schemas); the C3 422 body is lugas's native Problem
  Details with `issues` identifying the failing field (semantic match).
- POST /users parses the body manually: the fixture requires malformed
  JSON → 422, while lugas's built-in body pipeline answers 400
  (MALFORMED_JSON). Disclosed mapping, same as the elysia2 400→422 note.
- Error redaction via `defineApp({ onError })`: every handler throw
  returns a fixed internal problem (no message, no stack) — contract SEC-004.
- `N_ROUTES` generation is not implemented on this baseline yet (fixture
  routes only).

## Elysia 2 AOT notes (honest recording)

- Installed `elysia@2.0.0-beta.4` (npm dist-tag `next` at freeze time,
  2026-08-17). The package ships an AOT compiler (`dist/compile/aot*.js`,
  `AOT_MANIFEST_FORMAT 3`) used through the `aot: true` constructor option
  (default `true` in 2.x). No separate build step or special API is required:
  AOT compilation happens at `listen()`/`handle()` time inside Elysia.
- Elysia 2 error handling: hook order is `(path, hook, handler)`; errors
  surface via `app.error(fn)` where the handler receives `{ error, set }` and
  must read `error.status`/`error.name` (`ValidationError` 422, `NotFound`
  404, ParseError 400). `set.status` alone does not override ParseError's
  status — returning a raw `Response` does. The malformed-JSON case is mapped
  400→422 to match the frozen fixture semantics; this mapping is disclosed
  here (fixture fairness note).
- Route validation uses TypeBox schemas (`t.Object`) — Elysia's idiomatic
  path; validation errors identify the failing property/keyword natively.

## Fairness notes

- release builds for all candidates (Rust `--release`; Bun runs JS as-is —
  its production mode; no dev logging anywhere)
- same payloads, statuses, validation semantics, JSON byte orders
- no compression, no TLS, HTTP/1.1 keep-alive, bind 127.0.0.1
- raw-rust is a TRANSPORT LOWER BOUND: no framework/Treaty/validation-library
  parity by design; never used to imply feature equality
- baseline isolation: each baseline owns its lockfile; none import velqu code
