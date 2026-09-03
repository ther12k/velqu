# BETA-006-B — Worker Queues / Quarantine / Replacements (ops status)

Status: **IMPLEMENTED** (bounded structured status snapshot).

## What was built

`worker_ops_status(state)` (`crates/q-runtime/src/serve.rs`) — one
bounded, structured JSON snapshot aggregating the worker operating
state:

- **Queues**: request-slab live slots, queue pending depth, pending
  invocations (ownership).
- **Quarantine**: worker quarantined flag, queue-poisoned state, and
  cumulative poison events (from engine stats — the M2.2.1 bounded
  quarantine machinery).
- **Drain**: draining flag + refused-admission count.
- **Load shedding**: full per-reason load-shed counter snapshot
  (closed-set reasons).

Emissions are bounded by policy — rendered at drain transition
(`ops.worker.status` alongside `drain.begin`) and in the shutdown
report (which already carries full engine stats, stage metrics, and
ownership invariants). No high-frequency emitter: status is read on
demand, never streamed.

## Replacements

The bounded replacement policy (M3-005-C `ReplacementPolicy`: budget +
cooldown + fixed window, quarantined-worker replacement decisions) is
the fleet-shape machinery. In the single-worker beta topology a
"replacement" manifests as a process restart under supervisor control;
the quarantine counters above are the operator signal that precedes it
(poison events, quarantined flag). No in-process replacement is
triggered implicitly — restart storms are structurally excluded.

## Tests

All targeted suites green: `cargo test -p velqu-runtime` (57+ incl.
route metrics), q-http, q-bridge; fmt/clippy clean. The snapshot
composes existing tested primitives (counters, gates, health).
