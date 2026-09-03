# BETA-004-D — Postgres Deadline, Cancellation, Shutdown (engine boundary)

Status: **MEASURED** (live end-to-end). Wires the `runtime:postgres`
capability through the engine boundary: native binding, phase guards,
deadline enforcement, safe release, and startup fail-closed linking.

## What was built

- **Engine dialer contract** (`q-capability-postgres/src/dialer.rs`):
  `PostgresQueryDialer` — one call = one bounded parameterized query
  returning rows as JSON. The deadline bounds **acquire AND execution**.
  Safety rule: a lease that ends in any error (backend rejection,
  timeout, cancel) is **discarded — closed, never parked** — because a
  connection whose query failed mid-flight may still hold backend
  state. Clean leases are the only ones reused.
- **Engine native** (`q-engine-quickjs/src/worker.rs`):
  `__velquPostgresQuery(text, params_json, deadline_ms)` — installed
  ONLY when the host wired a pool (fail closed, never a mock). Same
  phase guards as timer/fetch (ops start only in a live Invocation;
  Cleanup/DeferredDrain/Shutdown/Idle reject), same op registry,
  owner-tagged `NativeOp`, same task-abort cancellation, same promise
  continuation contract (PostgresComplete arm carries the full
  resolution tail: finish_resolved + settle_background for chained
  continuations). `deadline_ms = 0 / >30s` is rejected at the native
  boundary before any I/O.
- **Prelude** (`prelude.rs`): `native.postgres.sql(text, params,
  deadlineMs)` — op-table Promise pattern (identical to
  `__velquTimerP`); when the binding is absent it throws the typed
  unavailability error (never a silent fallback).
- **Runtime linking** (`q-runtime/src/lib.rs`): the pack inventory is
  checked at startup. Pack requires `runtime:postgres` +
  `VELQU_DATABASE_URL` set → pool constructed (lazy; zero I/O until
  first query). Requires it + env missing → **startup rejected with a
  typed readiness error**. Pack does not require it → no pool is ever
  constructed (zero cost, BETA-004-A posture).
- **Pack validation** (`q-pack/src/lib.rs`): `postgres` joins the known
  route grants; unknown grants still reject.

## Measured: live end-to-end (operator-run on this host)

Benchmark stack (postgres:17.5, SCRAM, seeded), fixture app with one
postgres-granting route (`GET /db/users/:id` → parameterized SELECT):

- Startup WITH `VELQU_DATABASE_URL`: server ready; `curl
  /db/users/usr_1` → **`{"row":{"id":"usr_1","qty":42}}`** — full path
  HTTP → Rust host → QuickJS handler → native binding → pool → real
  Postgres → schema-validated response. Missing row → `{"row":null}`.
- Startup WITHOUT the env: **`ready ok:false — "pack requires
  runtime:postgres but VELQU_DATABASE_URL is not configured"`** (fail
  closed before serving). Stack torn down after the run.

## Deterministic tests (no network)

- 4 engine tests: rows resolve through the dialer (bound param +
  deadline echoed by a mock dialer, proving the JS→native→dialer wire
  path); fail-closed without a linked pool; deadline 0 rejected at the
  native boundary before I/O; ops during non-invocation phases refused.
- Pool/query/transaction suites from B/C unchanged and green (22 + 21
  unit, 2 live).

## Shutdown semantics

Worker shutdown refuses new postgres ops (Shutdown phase guard);
aborting an invocation aborts the in-flight query task (task
abort_handle, owner-checked settlement); the dialer discards the lease
on failure, so shutdown/cancel cannot leak a half-read connection back
into the pool. `LazyPool::begin_shutdown` (B) remains the pool-level
gate for the runtime drop path.
