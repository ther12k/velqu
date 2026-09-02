# Bounded `defer` — Worker-Owned Deferred Callbacks (M4A-007-A)

- **Status:** Implemented behavior specification for M4A-007-A (private alpha)
- **Parent task:** [M4A-007 — Implement bounded `defer` and lifecycle hooks](../codex-spark-beta/tasks/07_m4a_developer_preview/M4A-007-A-define-deferred-owner-queue-deadline-cancellation-shutdown.md)
- **Scope:** deferred-callback ownership, queue bounds, drain timing, deadline, cancellation, and shutdown behavior

## 1. What defer is (and is not)

`globalThis.__velquDefer(fn)` schedules a callback to run **after the response
for the current invocation has been handed off** to the host. Deferred work is
**best-effort**: it runs when the invocation owner drains it, and any failure
inside a deferred callback is isolated and swallowed.

> **Deferred work is not a durable job queue.** Deferred callbacks live only
> in the owning worker's memory. They are never persisted, never retried, and
> are discarded on shutdown, deadline, cancellation, or worker replacement.
> Do not use `defer` for anything that must survive process exit — use an
> external durable system for that.

## 2. Ownership and lifecycle

- The deferred queue is **owned by the single QuickJS worker** that admitted
  the callbacks. Only handler code running in the Invocation phase may enqueue:
  `__velquDefer` consults the host execution phase and throws `defer queue
  unavailable outside the invocation owner` otherwise (M4A-007-B). Settlement
  cleanup reactions — e.g. rejection continuations of aborted floating ops —
  and the deferred drain itself cannot re-enqueue.
- The queue is drained in a **dedicated `DeferredDrain` execution phase**,
  strictly after the response is fixed and handed off (M4A-007-B separates
  this phase from settlement `Cleanup`). A deferred callback can never delay
  or mutate the response of the invocation that scheduled it.
- The drain phase is **op-free**: deferred callbacks cannot start new native
  capability operations (timers, fetch, storage) — `__velquTimerStart` and
  friends reject with `native operations are unavailable while deferred work
  drains`. (Because timer start throws inside the promise executor, the
  rejection surfaces as a rejected promise, not a synchronous throw.)
- Handler-scheduled microtasks (promise reactions that exist before the
  handler returns) still run in the Invocation phase microtask checkpoint and
  MAY defer; only work that runs after settlement (cleanup reactions, the
  drain itself) is gated.

## 3. Bounds and forbidden recursion (M4A-007-D)

| Bound | Default | Enforcement |
| --- | --- | --- |
| Queue capacity | 64 (`QuickJsConfig::defer_queue_capacity`) | Admission consults the host-configured capacity (`__velquDeferCap`); `__velquDefer` throws `defer queue capacity reached` when full; the host additionally truncates to the cap before draining so drift can never grow the queue |
| Drain deadline | 100 ms (`QuickJsConfig::defer_deadline_ms`) | The drain arms the worker's defer-deadline interrupt; a spinning or long-running callback is interrupted and the drain ends |
| Re-enqueue | not permitted | Phase-gated: only the Invocation phase may admit; the drain and cleanup reactions are rejected |
| Native ops during drain | not permitted | Dedicated `DeferredDrain` phase guard rejects op starts |
| Direct queue access | not permitted | The queue is closure-private (M4A-007-D): `__velquDefer` is the only entry point, so recursive spawning is forbidden structurally, not by convention |

Unbounded recursive spawning is forbidden on every vector: a handler that
self-recurses through `__velquDefer` fills the bounded queue and then fails
closed; a drained callback's re-defer attempt is rejected by the owner rule;
and no JS-reachable alias of the queue exists to push through.

## 4. Cancellation and shutdown

- **Timeout and cancel paths perform no drain:** only resolved handoffs
  (Immediate, Failed, resolved watches) drain. Callbacks queued by an
  invocation that times out or is cancelled stay in the worker-owned queue and
  are drained by the **next** handoff — or discarded (and counted) at shutdown.
- **Per-invocation cancel:** deferred callbacks belong to the worker, not the
  invocation — a cancel does not remove them. Settlement cleanup (rejection
  reactions, aborted floating ops) remains on its own `Cleanup` phase and
  budget (`SETTLEMENT_GRACE`, `MAX_CLEANUP_JOBS`); the best-effort drain does
  not share it.
- **Shutdown:** worker shutdown aborts the runtime. Queued-but-not-drained
  deferred callbacks are discarded and never run. Shutdown does not wait for
  deferred work beyond the armed drain deadline.

## 5. Metrics (M4A-007-C)

Bounded-defer lifecycle counters are exposed on `EngineStats` (thus in the
runtime's `shutdown.complete` report):

| Field | Meaning |
| --- | --- |
| `defers_admitted` | callbacks admitted to the bounded queue |
| `defers_rejected` | `__velquDefer` calls that threw (non-function, non-owner phase, capacity) |
| `defer_drains` | non-empty drains (every handoff performs a drain attempt; attempts with an empty queue are not counted) |
| `defers_drained` | callbacks executed during drains |
| `defer_drains_interrupted` | drains ended by the defer-deadline interrupt |
| `defers_dropped_at_shutdown` | queued-but-never-drained callbacks discarded at shutdown |

Counters update on the worker thread after the response leaves the worker on
all handoff paths (including Failed); observers reading `EngineStats`
concurrently with a handoff may see the pre-drain values for that handoff.

## 6. Operational notes

- `__velquDefer` and its observers are engine globals of the private alpha
  runtime; they are not part of any published contract and may change. The
  queue itself is deliberately NOT exposed as a global.
- Deferred callback failures are deliberately silent (isolated, best-effort).
  Code that must observe failures should handle them inside the callback.
- The drain happens once per invocation handoff (success, failure, immediate,
  and cancel-settled paths), after the response leaves the worker. Packs with
  the prelude embedded in the compiled module drain the same way — the queue
  is a prelude global, not a host-side handle.
