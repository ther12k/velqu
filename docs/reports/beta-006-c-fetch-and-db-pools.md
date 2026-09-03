# BETA-006-C — Fetch and DB Pools Observability

Status: **IMPLEMENTED** (bounded snapshots, deterministic tests).

## What was built

- **Fetch pool** (`FetchPool::stats()` → `FetchPoolStats`): initialized,
  shutdown, active (derived from semaphore permits), max_active, and
  cumulative rejections. Bounded fields only; no per-host/per-URL
  cardinality.
- **Postgres pool**: `PostgresQueryDialer::pool_stats_json()` (default
  `None`; implemented for the lazy pool) — idle, inUse, createdTotal,
  maxConnections, and the ten BETA-004-E counters (acquires, reused,
  discards by cause, at-capacity, timeouts, rejections, shutdown
  refusals).
- **Aggregation**: `worker_ops_status` (BETA-006-B) now carries a
  `pools` section — `fetch` always present (lazy pool, zero activity
  until first use) and `postgres` reporting `linked: false` for apps
  without the grant, or the live pool snapshot when linked. Bounded
  emissions policy unchanged: on-demand / at drain and shutdown.
- **ServeState** holds the linked dialer handle so the status snapshot
  can reach the pool without new global state.

## Redaction audit

Pool snapshots carry counts, gauges, and configuration bounds only —
no URLs, no host names, no credentials, no query text.

## Tests (2 new + suites)

- `fetch_pool_stats_track_lazy_active_and_rejections` — laziness (no
  activity before first use), active-permit tracking, release.
- `fetch_pool_stats_reflect_shutdown` — terminal state visible.
- Postgres pool stats JSON covered by the B/E counters suite (28 unit).
- Full gates: fmt/clippy clean; `./scripts/verify` ALL PASS; `bun test`
  434+ pass / 0 fail.
