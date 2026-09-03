# @velqu/capability-postgres

First-party Postgres capability (`runtime:postgres` v1). **SQL in, rows
out — no ORM.**

## The surface (frozen by test)

One method:

```ts
const rows = await native.postgres.sql(
  "SELECT id, qty FROM items WHERE id = $1",
  [id],          // positional params, scalars only
  2_000,         // optional deadline ms (1..=30_000)
); // -> [{ id: "itm_1", qty: 7 }, ...]
```

There is deliberately **no query builder, no model/entity mapping, no
repository or migration DSL, no relation graph**. Statements are
parameterized by construction — the only API takes SQL text plus
positional scalar parameters, so string interpolation has no
convenience path and the executed statement is always visible in
application code. A builder method added later breaks the surface-
freeze test (`index.test.ts`), which fails the suite.

Scalars only: `null`, `boolean`, `number`, `string`. Nested objects and
arrays are rejected typed — they are model-shaped, and models are out
of scope by design. Applications compose SQL in their own modules;
the capability runs it.

## Identity & linking

- Grant: `postgres` (handler `native.postgres`)
- Requirement: exact `runtime:postgres` v1 in the pack inventory
- Runtime without a configured pool → startup fails closed (typed
  readiness error); pack without the grant → pool is never constructed
  (zero dependency/init cost).

## Limits & observability

`VELQU_PG_POOL_MAX` (1..=100), `VELQU_PG_POOL_CONNECT_TIMEOUT_MS`
(1..=30000), `VELQU_PG_POOL_IDLE_TIMEOUT_MS` — invalid values reject
startup, never clamp. Pool counters and `postgres_ops_*` engine stats
surface acquire/reuse/timeout/capacity behavior (BETA-004-E).
