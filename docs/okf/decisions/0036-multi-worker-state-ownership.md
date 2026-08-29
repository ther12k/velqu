---
type: Architecture Decision Record
title: ADR-0036 Multi-Worker State Ownership and Concurrency Model
status: accepted
date: 2026-08-30
implements: ADR-0018 (M3 multi-worker track), ADR-0014 (version-pinned bytecode), ADR-0030 (native operation ownership), ADR-0031 (bounded shutdown), ADR-0035 (trusted-code assumption)
---

# ADR-0036: Multi-Worker State Ownership and Concurrency Model

## Context

M1/M2 froze a single QuickJS worker per process. M3 introduces multiple
workers so throughput can scale with cores and a serverless profile can
start with one worker and grow adaptively. The moment a second QuickJS
runtime exists in the process, three questions become load-bearing:

1. What JS state does each worker own exclusively?
2. What native state may be shared, and under which discipline?
3. What is forbidden outright?

These answers must be frozen **before** the dispatcher lands
(M3-002), or every later packet will invent its own ad-hoc sharing
rules. This ADR is that freeze. It generalizes the single-worker
invariants that already hold (ADR-0030 operation ownership, ADR-0035
trusted-code assumption) to N workers, and it deliberately forbids the
shortcuts that make multi-runtime engines unsound.

## Decision

### 1. One runtime, one owner thread

Every QuickJS runtime is owned by exactly one worker thread for its
entire lifetime. All JS execution, all handles into that runtime, and
all runtime teardown happen on the owner thread. The runtime is never
moved between threads, never locked from outside, never polled from a
dispatcher. A worker that cannot keep up gets its own admission
pressure (M3-002) or is quarantined and replaced (M3-005) — it is
never "helped" from another thread.

### 2. Per-worker JS state (exclusive)

Each worker exclusively owns:

- the QuickJS runtime, context, and heap (including `WorkerShared`'s
  per-runtime portions: invocation counters, pending-op tables,
  poison flag — these are worker-local by construction);
- all module-level JS state (top-level `let`/`const`, module caches,
  closures) created by evaluating the pack;
- all JS timers, promises, microtasks, and `FinalizationRegistry`
  activity;
- the per-worker native operation registry and its request/handle
  stores for work admitted to that worker.

Consequences applications can rely on (and developer docs describe):

- module-level mutable state is worker-local: with N workers there are
  N independent copies of every module-level variable;
- counters, caches, and singletons in JS do not globally accumulate —
  each worker counts its own;
- `globalThis` is per worker.

### 3. Shared state (immutable, read-only)

Immutable after startup and safe to share by `Arc` without locks:

- the QPack artifact bytes (memory-mapped or boxed, identical for every
  worker — every worker evaluates the same pack bytes in the same
  deterministic order);
- compiled route plans, route ID tables, schema IR vectors, decoder
  and encoder tables, response schema ID maps;
- capability descriptors/manifests resolved from the pack.

Nothing in this category is ever mutated after the startup phase that
produces it. Startup constructs these once, freezes them, and only
then admits traffic.

### 4. Shared state (mutable, explicit discipline)

The only mutable cross-worker state is host-owned infrastructure with
an explicit concurrency discipline, named here so any new shared state
must justify itself against the same bar:

- **dispatch queues**: MPMC channels (M3-002); producers are any host
  thread, the consumer is exactly one worker;
- **metrics**: the M28-009 collector pattern — a fixed-size shard
  behind a mutex, or per-worker shards merged at sample time; saturating
  adds only; never a growable structure;
- **outbound pool handles**: the M28-003 pool (Arc + atomic state);
  connections themselves are `Send` hyper internals, never JS values;
- **lifecycle flags**: atomics (`watch`, `AtomicBool`/`AtomicU64`) for
  shutdown, quarantine, readiness aggregation (M3-005/M3-007).

Anything mutable that cannot be phrased in one of these four shapes
requires a new ADR.

### 5. Forbidden outright

- **No `JSValue` crosses workers.** No JS value, runtime pointer,
  context pointer, or heap pointer is sent, shared, or cached across
  worker boundaries. Native op results destined for another worker are
  serialized data (bytes), never engine objects.
- **No shared JS heaps, shared contexts, or runtime stealing.**
- **No locks held across JS execution.** A host lock guards host data
  only; while JS runs, the owner thread holds no host lock that another
  thread needs to make progress (deadlock-freedom by construction).
- **No ambient thread pool touching runtimes.** Blocking work
  (M28-005 native ops) runs on Tokio's pool and reports back through
  the owning worker's queue — it never reaches into a runtime.

### 6. Deterministic initialization

Worker K is initialized by evaluating the identical pack artifact
bytes, in pack order, with the same construction sequence as worker 0
(profile, natives install, prelude evaluation — M3-004 freezes the
exact procedure and bounds its parallelism). Two workers of the same
pack at the same commit are behaviorally indistinguishable at their
ready lines: same routes, same schema IDs, same capability set. Only
accumulated per-worker state (counters, module mutations by traffic)
may differ after traffic flows.

## Concurrency model tests plan

The following obligations bind later packets (each proves its slice
when the machinery lands; none requires new infrastructure in this
packet):

| Obligation | Proving packet |
| :--- | :--- |
| Worker K ready line matches worker 0 (same routes/schemas/capabilities) | M3-004-A/B |
| Module-level mutation on worker A is invisible on worker B | M3-004-B (state isolation example) |
| No JS execution off the owner thread (scheduler boundary assertions) | existing M2.2.1 boundary tests + M3-002-D |
| Dispatcher admits to bounded per-worker queues; overflow is typed | M3-002-A/C |
| Quarantined worker: pending work settled, replacement deterministic | M3-005-A/B/C |
| Drain: admission stops, in-flight bounded, abort after deadline | M3-007-A..D |
| Shutdown reaches quiescence across N workers (budgeted) | M3-007-D |

## State examples

- `let hits = 0;` at module top level with 4 workers and 100 requests:
  each worker's `hits` sums to 100/4 in the balanced case — four
  separate variables, never a shared counter. Applications needing
  global counters use host metrics (`FetchMetricsCollector`), which are
  explicitly shared per §4.
- A `Map` cache written on worker A is not readable on worker B; each
  worker builds its own cache lazily. (Documented cold-start cost of
  N caches is part of the M3-009 evidence.)
- A native `fetch` permit is acquired by whichever worker admits the
  request; the permit is returned on that worker's completion path.
  The pool itself is shared (§4); permits never migrate between workers
  with a request.

## Alternatives considered

- **Shared runtime with a global lock** (one JS heap, N host threads
  serialized by a mutex): defeats the purpose of worker scaling
  (lock contention is the bottleneck) and preserves the single point
  of quarantine blast radius. Rejected.
- **Workers as Worker-like isolated processes**: stronger isolation
  but pays IPC serialization for every request and duplicates the pack
  mapping; incompatible with the bounded single-process memory model.
  Revisit only if a future isolation requirement demands it (would be a
  new ADR).
- **Software transactional memory / STM for shared JS state**:
  research-grade, unsound with QuickJS internals. Rejected.

## Consequences

- The dispatcher (M3-002) is a pure host-side router: it owns no JS
  state and holds no locks across execution.
- Scaling is honest: throughput gains come from real parallel runtimes,
  measured in M3-009 with physical topology recorded.
- Failure isolation is real: a quarantined worker poisons exactly its
  own runtime; replacement re-runs the deterministic initialization.
- Developer docs must state plainly that module-level state is
  per-worker (M3-001-B owns that documentation).
