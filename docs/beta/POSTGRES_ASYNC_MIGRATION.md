# Postgres capability — async contract migration (BWASM-C-002)

**Status: locked.** The `runtime:postgres` v1 capability contract is
Promise-based everywhere. This is the one breaking change to the
capability's authoring surface, landed before any browser capability
adapter depends on it (BWASM-C-004/C-005, optional C-003).

## Affected public API — complete list

| API | before | after |
|---|---|---|
| `PostgresCapability["sql"]` (`@velqu/core`, handler `ctx.native.postgres.sql`) | returned `{ rows, affectedRows }` (typed synchronous) | returns `Promise<PostgresSqlResult>` |
| `postgres["sql"]` (`@velqu/capability-postgres` SDK) | returned `SqlResult` (typed synchronous) | returns `Promise<SqlResult>` |

Nothing else in the capability surface changed: the identity pair
(`runtime:postgres`, version `1`), the grant name (`postgres`), the
parameter model (positional `$n`, scalars only), the deadline bounds
(1..120_000 ms), the pack wiring, and the Rust runtime behavior are
unchanged. At run time the native bridge always settled `sql` through
its op table — the old types simply lied about it. C-002 makes the
TypeScript contract tell the truth.

## Migrating handlers

```ts
// before (accidentally worked only because the types were wrong)
export async function handler(ctx: HandlerCtx<...>) {
  const r = ctx.native.postgres.sql("SELECT * FROM users WHERE id = $1", [id]);
  return r.rows;   // now a compile error: .rows does not exist on a Promise
}

// after
export async function handler(ctx: HandlerCtx<...>) {
  const r = await ctx.native.postgres.sql("SELECT * FROM users WHERE id = $1", [id]);
  return r.rows;
}
```

Sync-looking use is a **compile-time error** (tsc: `Property 'rows' does
not exist on type 'Promise<SqlResult>'`) — `velqu check` and any IDE
surface it immediately.

### Codemod

```bash
bun scripts/migrate-postgres-async.mjs <project-dir>           # report (exit 1 = sites found)
bun scripts/migrate-postgres-async.mjs <project-dir> --write   # rewrite
```

The codemod rewrites the mechanical patterns (`const x = X.sql(…)`,
`return X.sql(…).rows`) and reports sites it will not touch (non-async
enclosing functions — mark them `async` first; the compiler already
awaits handler results, so this is always safe). Output is a
schema-versioned JSON report (`schemaVersion: 1`).

## Frozen v1 async semantics

| aspect | contract |
|---|---|
| Return | always a `Promise`; argument validation stays synchronous (`TypeError` / `RangeError` thrown from the call) |
| Deadline | `deadlineMs` (default 5_000, ceiling 120_000) cancels the round trip; the connection is released back to the pool before the rejection is observed |
| Cancellation | the per-call deadline is the only cancellation surface in v1 (no `AbortSignal`) |
| Transactions | single-statement autocommit per call; **no transaction pinning** — `BEGIN`/`COMMIT` via `sql()` statements run on independent pooled connections and MUST NOT be used for multi-statement atomicity |
| Result values | rows are plain objects over the bounded JSON-compatible set (string/number/boolean/null); values outside the set convert via the native bounded-value rules or reject with a typed query error |
| Row counts | `affectedRows` counts DML changes; 0 for SELECT-shaped results |
| Errors | `PostgresCapabilityUnavailable` (sync, binding not linked — fail closed); `PostgresDeadlineExceeded` (deadline hit; `deadlineMs` parsed from the native reason); `PostgresQueryError` (everything else; `nativeReason` preserved verbatim — the stable diagnostic text) |

Both surfaces (`ctx.native.postgres` and the SDK `postgres`) classify
rejections identically — the classification lives in the runtime prelude
and in `@velqu/capability-postgres`, mirrored and pinned by tests.

## Why now (and why this is the last break)

Browser capability adapters (IndexedDB KV, fail-closed routing, the
optional PGlite adapter) must target one database contract. A synchronous
typing cannot be implemented by a browser-local adapter at all; freezing
two APIs (sync + async) is explicitly out of scope. C-002 is the single
locked break; adapters build on `Promise<SqlResult>` from here.
