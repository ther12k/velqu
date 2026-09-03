# BETA-004-B — Lazy Pool (runtime:postgres)

Status: **MEASURED** (live-Postgres evidence) — the bounded, lazy
connection pool behind the `runtime:postgres` capability. Query surface
and parameterized wire behavior are BETA-004-C; engine deadline/cancel
semantics are D; pool-limit policy/observability are E.

## What was built

`crates/q-capability-postgres` (new crate — core stays unenlarged):

- **Lazy by construction**: `LazyPool::postgres(url, config)` parses
  nothing and connects nothing; connections are created only inside
  `acquire()`. Unit-tested: zero connector invocations at construction,
  at `stats()`, and over elapsed time.
- **Bounded**: capacity is a semaphore over
  `max_connections` (validated 1..=100, fail closed — never clamped);
  an acquire beyond the ceiling fails typed
  `PoolError::AtCapacity { max, waited_ms }` when its wait deadline
  expires.
- **Typed timeouts**: connect (and capacity wait) bounded by the
  acquire's `wait_ms`; `PoolError::ConnectTimeout` — a slow or
  unroutable backend can never hang a worker.
- **Idle reuse and hygiene**: released connections park idle and are
  reused (created_total stays 1 across sequential acquires); idle
  connections older than `idle_timeout_ms` and connections the backend
  closed (`is_closed`) are discarded on the next acquire, never handed
  out.
- **Shutdown gate**: `begin_shutdown()` refuses new acquires
  (`ShuttingDown`); a connection released while shutting down is closed
  rather than parked.
- **Generic core, real driver at the edge**: pool logic is generic over
  a `Connector` trait so all 12 unit tests run deterministically against
  a mock (zero network); production uses `TokioConnector`
  (tokio-postgres, no TLS — loopback-only posture until an owner TLS
  decision).

## Real-world results (live Postgres, operator-run on this host)

`tests/live.rs` with the benchmark stack
(`docker compose up -d --wait` + `./reset.sh`, postgres:17.5,
127.0.0.1:5433, SCRAM auth, seeded 1,000 users):

- Acquired within deadline, ran `SELECT count(*) FROM users` → **1,000**
  (seeded dataset), released to idle.
- Sequential acquire **reused** the idle connection (created_total = 1).
- With 4 leases held (`max_connections = 4`), the 5th acquire failed
  typed `AtCapacity { max: 4 }`.
- Connect against an unroutable port failed typed
  (`ConnectTimeout`/`ConnectRejected`), never hung.
- After `begin_shutdown()`, acquire fails typed `ShuttingDown`.
- Stack torn down after the run (`docker compose down`).

## Cold / RSS cost report

- **Construction is free**: the pool holds a config, a semaphore, and a
  mutex-guarded queue — no URL parse, no DNS, no socket, no backend
  memory until the first acquire (unit-tested with a counting
  connector: 0 invocations).
- **Per-connection cost** appears only under demand: one TCP connection
  + backend session per lease slot, at most `max_connections` of them.
- **An app that never grants `runtime:postgres` never constructs a pool
  at all** — the capability never leaves `Declared` (BETA-004-A wiring);
  its cold/RSS profile is unchanged (BETA-004-A report: cold p50
  11.697ms, RSS p50 9,676 kB).

## Deterministic tests (no network)

12 unit tests in-crate: config bounds, laziness, connect+reuse,
ceiling+typed wait, connect timeout, connect rejection, zero-wait
rejection, stale-idle discard, shutdown gate, release-under-shutdown
close, dead-idle discard (kill-switch mock), typed missing-URL. The
live test is env-gated (`VELQU_PG_LIVE_TEST=1`) so CI stays
deterministic; it ran green here.
