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
  the callbacks. Only handler code running in the Invocation phase may enqueue
  (`__velquDefer` throws `defer queue unavailable outside the invocation
  owner` otherwise).
- The queue is drained in the **Cleanup phase**, i.e. strictly after the
  response is fixed and handed off. A deferred callback can never delay or
  mutate the response of the invocation that scheduled it.
- Because the drain runs in the Cleanup phase, deferred callbacks cannot spawn
  new native capability operations (timers, fetch, storage); reactions that
  create floating promises are bounded by the cleanup job budget.

## 3. Bounds

| Bound | Default | Enforcement |
| --- | --- | --- |
| Queue capacity | 64 (`QuickJsConfig::defer_queue_capacity`) | `__velquDefer` throws `defer queue capacity reached` when full; the host additionally truncates to the cap before draining so drift can never grow the queue |
| Drain deadline | 100 ms (`QuickJsConfig::defer_deadline_ms`) | The drain arms the worker's defer-deadline interrupt; a spinning or long-running callback is interrupted and the drain ends |

Both bounds are host-enforced configuration on the engine profile; the
JS-visible check is the first line of defense, not the only one.

## 4. Cancellation and shutdown

- **Per-invocation cancel:** cancelled invocations settle like any other
  settlement; deferred callbacks that were already queued for a *completed*
  invocation still belong to the worker, not the invocation — the next drain
  after the cancel path completes runs them.
- **Shutdown:** worker shutdown aborts the runtime. Queued-but-not-drained
  deferred callbacks are discarded and never run. Shutdown does not wait for
  deferred work beyond the armed drain deadline.

## 5. Operational notes

- `__velquDefer` and the queue are engine globals of the private alpha
  runtime; they are not part of any published contract and may change.
- Deferred callback failures are deliberately silent (isolated, best-effort).
  Code that must observe failures should handle them inside the callback.
- The drain happens once per invocation handoff (success, failure, immediate,
  and cancel-settled paths), after the response leaves the worker.
