---
type: Evidence Report
title: Type-System Report (authoring + Treaty spike)
status: complete
milestone: M0
---

# Type-system report

Sources: `packages/{schema,core,contract,treaty}`, `benchmarks/type-scale/`,
`bun run typecheck` (clean), `bun test packages` (7 pass).

## What was proven

1. **One schema, many artifacts (SCHEMA-001)**: `s.object({...})` produces
   BOTH the inferred TS type and the exact Schema IR v1 JSON node the Rust
   runtime validates with — verified by unit tests asserting the emitted IR
   (`kind`, `required` list, optional/default semantics).
2. **Route-local typing**: handler ctx (`params`, `query`, `body`,
   `session`) derives from the route's schema arguments; undeclared statuses
   are rejected at the type level (`HandlerResult` union keyed by declared
   response codes) and at runtime (contract violation outcome).
3. **Policy context flows into types (SCHEMA-004)**: `definePolicy` with
   `provides: "session"` types `ctx.session` inside the handler
   (`expectTypeOf(session).toEqualTypeOf<{userId:string}>()`).
4. **Status narrowing (TRT-003)**: Treaty errors narrow by status —
   `if (r.error.status === 401)` types `r.error.problem` as the 401 shape;
   test proves narrowing on a real 401.
5. **Non-throwing results (TRT-002)**: declared HTTP failures are values;
   network failures carry `status: 0, kind: "network"`; aborts
   `kind: "abort"` — three-way distinction tested.
6. **Published-contract mode**: `@velqu/contract` `RouteContract` shape drives
   the same Treaty client as source mode would; the compiler (M2) generates
   the `Api` type from the app — hand-written proof exists in the test.
7. **Negative typing caught**: `@ts-expect-error` fixtures and a
   deliberate wrong-typed call are rejected by tsc at 100/500/1,000 routes
   (`negativeCaught: CAUGHT`).

## Scale results (fresh `tsc --noEmit`)

| Routes | best / worst run | budget | status |
|---:|---|---:|---|
| 100 | 2.1s / 4.8s | ≤1.5s | FAIL (fixed tsc floor ≈2s dominates) |
| 500 | 2.2s / 2.9s | ≤3.0s | PASS |
| 1,000 | 2.7s / 5.8s | ≤5.0s | PASS (borderline worst-case) |

Raw: `benchmarks/type-scale/results.json` (wall time, peak RSS ~425–467MB is
tsc itself, declaration sizes 14–144KB). No O(n²) blowup: 10× routes ≈ 1.3×
time after the fixed floor.

## Deviation recorded (ID-011)

Eden-exact single-segment navigation (`api.hello({name}).get()`) is
ambiguous when one first segment hosts multiple routes (`/users` POST +
`/users/:id` GET). M0 uses route-id navigation `api.hello.get({name}).get()`
— unambiguous, typed, and compiler-friendly. Owner decision open for the
final ergonomics.

## Treaty client size budget

TRT-004/≤8KiB: `packages/treaty/src/index.ts` is a single dependency-free
module (~5.5KB source, no server imports). Minified-size measurement is an
M2 packaging step (UNEXECUTED yet; source size recorded).
