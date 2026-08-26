---
type: Architecture Decision Record
title: ADR-0031 Bounded Capability Shutdown and Quiescence
status: accepted
date: 2026-08-26
implements: ADR-0028 (capability lifecycle §6), ADR-0030 (operation model)
---

# ADR-0031: Bounded Capability Shutdown and Quiescence

## Context

ADR-0028 froze the lifecycle phases and stated that shutdown reaches
quiescence or fails closed. ADR-0030 provided the operation model
(owners, deadlines, cancellation classes). What remains is the drain
protocol itself: what happens to pending operations when shutdown
begins, what "done" means, what happens when the budget is missed,
and how each outcome maps to lifecycle phases — so every capability
host shuts down identically and no operation is ever silently
abandoned.

## Decision

### 1. Shutdown has a fail-closed budget

`SHUTDOWN_BUDGET_MS` (5,000 today) bounds the whole drain. Reaching
quiescence after the budget is a `DeadlineExceeded` failure, never a
late success. Moving the ceiling is an ADR-level decision. The
protocol is deterministic — the host reports whether the deadline
fired; the model never reads a clock, so every outcome is testable.

### 2. The drain protocol

```text
begin_shutdown   Ready -> Draining        (new operations refused)
drain_step       cancel pending cancellable ops;
                 count pending non-cancellable (the await set)
[host settles the await set... or the budget fires]
finish_shutdown  no pending  -> Draining -> Quiesced   (accounted)
                 budget fired -> expire stragglers;
                                 lifecycle -> Failed    (fail closed)
```

- `begin_shutdown` wraps the lifecycle's `Ready → Draining` edge;
  draining a capability that never served, or draining twice, is the
  lifecycle's typed illegal transition.
- `drain_step` cancels every pending cancellable operation
  immediately (cancellation is legal there by construction) and
  returns the await set: pending non-cancellable operations that
  were declared to complete on their own.
- `finish_shutdown`:
  - **No pending operations** → `Quiesced` with accounting
    (`cancelled`/`settled`/`expired` counts). The only success
    outcome.
  - **Budget fired with pending operations** → the stragglers are
    `Expired` (visibly, not silently) and the lifecycle goes to
    `Failed`. A late settlement for an expired operation drops
    (ADR-0030), and quiesce is terminally refused — the failure is
    observable and reportable.
  - **Pending operations and no observed deadline** → typed error,
    lifecycle unchanged. That state has no honest outcome; it
    indicates a host bug, and the protocol refuses to invent one.

### 3. Quiescence accounting is part of the contract

The success outcome names what happened to every operation. Shutdown
reports are generated from these counts, not reconstructed from
logs; drain reports never contradict operation states.

## Threat review

- **Silent abandonment**: impossible by construction — a missed
  budget expires stragglers explicitly and fails the lifecycle;
  nothing exits "quietly pending".
- **Shutdown races with new work**: `Draining` refuses operation
  starts through the ADR-0030 `NotReady` guard, tested on this exact
  path.
- **Zombie settlements**: post-expiry deliveries drop through
  `deliver_or_drop`; a `Failed` lifecycle can never be revived to
  `Quiesced`.
- **Unbounded drain**: the budget is a fail-closed ceiling; the
  failure path itself terminates deterministically (expire + fail).

## Consequences

- `q-capabilities::shutdown` completes the M27-001 define set:
  lifecycle (ADR-0028), identity (ADR-0029), operations (ADR-0030),
  and bounded shutdown (this ADR) are pinned by 30 tests.
- Capability hosts (starting with the M27-004 timer port) call this
  protocol instead of bespoke drain loops.
- The author guide gains a "Shutdown and drain" section; the M27-001
  verification packet can now map every parent guardrail to a named
  test.

## Status

Accepted (M27-001-D). Tests in
`crates/q-capabilities/src/shutdown.rs`:
`all_cancellable_operations_drain_to_quiesced`,
`non_cancellable_operations_settle_within_budget`,
`missed_budget_expires_stragglers_and_fails_closed`,
`empty_operation_set_quiesces_immediately`,
`draining_refuses_new_operations`, `shutdown_requires_ready_lifecycle`,
`finish_with_pending_and_no_deadline_has_no_honest_outcome`.
