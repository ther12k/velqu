# Beta API snapshot — capability contracts (BWASM-C-002)

Recorded 2026-09-06 at the C-002 contract lock, before any browser
capability adapter (BWASM-C-004/C-005, optional C-003) depends on the
database surface. This snapshot records the async contract as the frozen
beta authoring surface; the native wire/ABI identity is unchanged.

## `runtime:postgres` v1 (grant `postgres`)

- Identity: id `runtime:postgres`, version `1` — exact match, unchanged
  from BETA-004-A.
- Handler surface (`ctx.native.postgres`):

  ```ts
  interface PostgresCapability {
    sql(text: string, params?: readonly PostgresSqlParam[], deadlineMs?: number):
      Promise<PostgresSqlResult>;
  }
  type PostgresSqlParam = string | number | boolean | null;
  type PostgresSqlRow = Record<string, PostgresSqlParam>;
  interface PostgresSqlResult { rows: PostgresSqlRow[]; affectedRows: number }
  ```

- SDK surface (`@velqu/capability-postgres`): `postgres.sql(text, params?,
  deadlineMs?): Promise<SqlResult>` with the same shape; typed error
  classes `PostgresCapabilityUnavailable` (sync), `PostgresDeadlineExceeded`,
  `PostgresQueryError`.
- Semantics: deadline-bounded (default 5_000 ms, ceiling 120_000 ms),
  single-statement autocommit, bounded JSON-compatible values,
  `affectedRows` for DML — full table in
  [POSTGRES_ASYNC_MIGRATION.md](POSTGRES_ASYNC_MIGRATION.md).

## `runtime:timer` (grant `timer`)

- `TimerCapability.delay(ms: number): Promise<number>` — already
  Promise-based; unchanged by C-002.

## Host capability graph (native prelude, frozen graph)

`timer`, `console`, `url`, `text` (TextEncoder/TextDecoder), `abort`
(AbortController/AbortSignal), `crypto` (WebCrypto), `fetch`
(fetch/Headers/Request/Response), `postgres` — each fail closed when the
host does not provide them; capability authorization remains native per
call. Browser-lane capability bridging is defined by BWASM-R-005 and
extended by BWASM-C-001/C-004/C-005 on top of this snapshot.

## Snapshot verification

Pinned by `packages/capability-postgres/src/async-contract.test.ts`
(Promise surface, typed rejections, sync fail-closed, compile-time
negative/positive fixtures) and the engine tests
(`crates/q-engine-quickjs/tests/engine.rs`, BETA-004-D block: resolve,
fail-closed, deadline boundary, cleanup-phase refusal).
