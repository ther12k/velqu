# BETA-006-A — Request/Route/Status/Duration Metrics

Status: **IMPLEMENTED + MEASURED** (bounded aggregation, deterministic
tests).

## Metrics schema (BETA-006-A)

Per-route, cardinality-bounded by construction — one entry per pack
route (a static table fixed at startup) plus a single `<unknown>`
fallback bucket. **Never** per-path, per-status-code, or per-label.

Per route (all monotonic atomics; O(1) increments, no locks, no
allocation on the request path):

| field | meaning |
|---|---|
| total | requests completed on the route |
| ok_2xx / redirect_3xx / client_error_4xx / server_error_5xx | status-class buckets (not raw codes — bounded cardinality) |
| duration_us_total | sum of request durations (µs) |
| duration_us_max | max request duration (µs) |

Snapshot: `RouteStatusMetrics::snapshot()` →
`Vec<RouteStatusEntrySnapshot>` (serde-serializable; `<unknown>` last).

## Redaction audit

The aggregation carries **route ids, status classes, and durations
only** — no paths, no header values, no query strings, no bodies, no
client identifiers. This matches SEC-004: the existing
`request.complete` log (LogMode-gated) also never logs header values;
the metrics layer adds no new surface where secrets/PII could land.

## Overhead (disabled vs enabled)

- **Disabled** (per LogMode=Off): the previous behavior kept
  `started = None` and skipped all serialization. Now duration capture
  is a single `Instant` copy (always on, so metrics stay meaningful
  when logging is off) — measured below; log serialization remains
  fully gated (Off/Errors/Full unchanged, sampling restored).
- **Enabled**: one `record()` = two hash-free index lookups (route id →
  fixed index from a startup map) + ~7 atomic relaxed increments.
  Measured in-test (`record_overhead_is_budgeted`): 10,000 calls with a
  generous per-call bound (<50 µs/call asserted; observed well below).

## Tests (2 new, in-crate)

`statuses_bucket_and_durations_aggregate` — status bucketing, duration
total/max math, unknown-route fallback, fixed entry count;
`record_overhead_is_budgeted` — hot-path cost bound. Targeted suites:
`cargo test -p q-http` (2 suites ok), `-p q-bridge` (ok), `-p
velqu-runtime` (57+ pass including the new tests).
