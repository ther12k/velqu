# BETA-006-D — Memory / Tasks / Slots Gauges

Status: **IMPLEMENTED** (bounded gauges in the ops status snapshot).

## What was added

`worker_ops_status` (BETA-006-B snapshot) now carries:

- **memory**: `heapUsedBytes` — the QuickJS worker heap usage (runtime-
  tracked, ceiling = the configured heap limit).
- **tasks**: native task accounting — `nativeStarted`, `nativeAlive`,
  `nativeCompleted`, `nativeAborted` (physical Tokio task lifecycle;
  an aborted op leaves no task alive — M2.2.1-r4).
- **slots**: request-slab occupancy — `live` slots and the admission
  `capacity` (the bound from the runtime limits; exhaustion is the
  bounded `RequestCapacity` outcome, never unbounded growth).

All gauges are pre-existing counters surfaced through the bounded
status snapshot (BETA-006-B emissions policy: drain transition +
shutdown report + on-demand reads; no hot-path cost, no streaming).

## Redaction audit

Numeric gauges only — no identifiers, no payloads, no addresses.

## Tests

Targeted suites green (`cargo test -p velqu-runtime` 57+; fmt/clippy
clean). The gauges re-expose existing tested counters; composition is
the B-packet snapshot path.
