# Postgres Capability (`runtime:postgres`)

Status: normative for the first-party database capability (BETA-004).

The Postgres capability provides a real database story **without
enlarging core**: the pool and protocol live in
`crates/q-capability-postgres`, the SDK in
`packages/capability-postgres`, and the engine carries only the same
native-op machinery the timer and fetch capabilities use.

## Identity and linking

- Id: `runtime:postgres`, exact version `1` (ADR-0029 exact-match).
- Grant: `postgres` on handler `native.postgres`.
- A pack that grants it carries the requirement in its inventory; a
  runtime that cannot provide it **fails closed before serving**.
  Missing `VELQU_DATABASE_URL` is a typed startup rejection.
- A pack that does not grant it links nothing: no pool is constructed,
  no dependency is loaded, cold start and RSS are unchanged (measured:
  BETA-004-A report).

## API — no ORM

The capability exposes exactly one operation: `sql(text, params,
deadlineMs)` with positional scalar parameters. There is no query
builder, no model/entity mapping, no repository/migration DSL, no
relation graph — by design and enforced by a surface-freeze test. SQL
is written by the application and parameterized by construction; the
executed statement is always visible in application code.

## Lifecycle and safety

- Lazy: the pool constructs without I/O; connections are created on
  first acquire and reused only when idle, live, and cleanly settled.
- Bounded: connection ceiling 1..=100; acquire/connect deadlines are
  typed failures; every query op is cancellable.
- Safe release: a lease that fails or times out mid-flight is closed,
  never returned to the pool (the backend may still hold mid-query
  state).
- Shutdown: new ops are refused once draining; released connections
  close rather than park.
- Configurable limits (`VELQU_PG_POOL_MAX`, `..._CONNECT_TIMEOUT_MS`,
  `..._IDLE_TIMEOUT_MS`) reject startup when out of bounds — never
  clamp.

## Observability

Pool counters (acquire/reuse/create/discard/at-capacity/timeout paths)
and engine `postgres_ops_*` stats. Typed errors remain the failure
story — counters observe, they never absorb.
