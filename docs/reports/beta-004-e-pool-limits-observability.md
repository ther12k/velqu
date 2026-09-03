# BETA-004-E — Postgres Pool Limits and Observability

Status: **MEASURED** (live startup-rejection + serving evidence).

## Pool limits (configurable, fail closed)

- `VELQU_PG_POOL_MAX` (1..=100), `VELQU_PG_POOL_CONNECT_TIMEOUT_MS`
  (1..=30000), `VELQU_PG_POOL_IDLE_TIMEOUT_MS` (positive) — all
  optional; absent values fall back to the default posture (10 / 5s /
  30s). Present-but-invalid values **reject runtime startup** with a
  typed readiness error — never clamped, never silently defaulted.
- Measured: `VELQU_PG_POOL_MAX=5000` → `ready ok:false — "pool limits
  are invalid: max_connections out of 1..=100"`; with
  `VELQU_PG_POOL_MAX=3` the fixture app served live queries normally
  under the limit.

## Observability

- **Pool counters** (`PoolCounters` / `counters()` snapshot):
  acquires_ok, reused, created, discarded_stale, discarded_dead,
  discarded_error, at_capacity, connect_timeouts, connect_rejected,
  shutdown_refusals. Monotonic atomics; reading never blocks.
- **Engine counters**: `postgres_ops_started` / `postgres_ops_completed`
  surface on `EngineStats` alongside the timer/fetch op accounting.
- Exhaustion, timeout, and rejection remain typed errors — the counters
  observe; they never absorb failures.

## Deterministic tests

- 6 new tests: counters track created/reused/at-capacity/shutdown
  refusals; connect timeout and rejection counters; error-discarded
  leases counted and never parked; env-config defaults, overrides, and
  out-of-bounds rejections. Crate total 28 unit + 2 live, all green.

## Live results (operator-run)

Fixture app served live queries under `VELQU_PG_POOL_MAX=3`
(`{"row":{"id":"usr_1","qty":42}}`; missing row → `{"row":null}`);
invalid-limit startup rejected typed; stack torn down after the run.
