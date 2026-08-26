---
type: Architecture Decision Record
title: ADR-0030 Native Operation Ownership, Deadlines, and Cancellation
status: accepted
date: 2026-08-26
implements: ADR-0028 (capability ABI and lifecycle §4–§5)
---

# ADR-0030: Native Operation Ownership, Deadlines, and Cancellation

## Context

ADR-0028 froze that every native operation has one owner, that
settlement is delivered to the owner's generation or dropped as
expired, and that every operation is either physically cancellable or
explicitly non-cancellable. Before capabilities multiply (M27-004
ports timers; M27-005…M27-008 add URL/text/abort/crypto), the runtime
needs the concrete operation state model: owner identity, deadline
bounds, the state vocabulary, and the delivery rules — pinned by
tests so every capability host behaves identically.

## Decision

### 1. One owner per operation: slot + generation

`OpOwner { slot, generation }` reuses the pair the request store
already validates. An operation without an owner is a bug; a
settlement delivered by a non-owner is a typed error (`NotOwner`),
never accepted. When the owning invocation's generation expires
before a settlement arrives, the host routes the late completion
through `deliver_or_drop`: the result is **dropped as an expected
outcome**, not an error — but the owner check still applies on the
drop path.

### 2. Deadlines are bounded, fail-closed

Every operation carries `deadline_ms ∈ [1, MAX_OP_DEADLINE_MS]`
(`300_000` today). Zero and over-ceiling values are typed rejections
at start — never clamped silently (AGENTS.md constraint 11: all
deadlines are bounded). Raising the ceiling is an ADR-level
decision.

### 3. Closed operation-state vocabulary

```text
Pending -> Settled | Cancelled | Expired     (terminal, final)
```

- `Settled`: completed normally.
- `Cancelled`: physically stopped (cancellable class only).
- `Expired`: the deadline fired first. Expiry applies to **both**
  cancellation classes — deadlines bound every operation.

Terminal states are final: every further state change (double
settle, cancel-after-settle, expire-after-cancel) is a typed
`IllegalOpTransition` and never mutates. Exactly-once settlement and
visible double-cancellation fall out of this rule.

### 4. Two cancellation classes, enforced at the data-model level

- `Cancellable`: `cancel()` succeeds from `Pending`. Idempotency is
  **visible**: a second cancel is a typed illegal transition, not a
  silent no-op, so double-cancellation shows up in tests and logs.
- `NonCancellable`: `cancel()` is a typed `NotCancellable` rejection
  and leaves the operation untouched. The class is chosen at
  `NativeOp::start` — there is no default.

### 5. Operation starts obey the lifecycle

`NativeOp::start` takes the capability's lifecycle and refuses
(typed `NotReady`) unless the phase is exactly `Ready` — the
operation-level enforcement of ADR-0028 guardrail 1, on top of the
lifecycle's own `start_op` guard.

## Threat review

- **Orphaned operations**: single-owner + generation-checked delivery
  means an operation can never outlive its invocation silently; late
  settlements drop.
- **Settlement forgery**: `NotOwner` makes cross-invocation
  settlement a typed bug, including on the drop path.
- **Unbounded work**: the deadline ceiling bounds every operation
  start; over-limit asks fail closed with the limit named.
- **Silent double effects**: exactly-once settlement via terminal
  finality; double-cancellation and cancel-after-settle are visible
  typed errors.
- **Class confusion**: non-cancellable operations cannot be cancelled
  through any path — the rejection is structural, not a runtime
  policy check.

## Consequences

- `q-capabilities::operations` is the shared operation model: the
  timer port (M27-004) and every later capability host use
  `NativeOp` instead of ad-hoc op tables.
- M27-001-D (bounded shutdown) drains by iterating pending
  operations and applying cancel/expire per class through this
  model.
- The author guide gains an "Operations, deadlines, and
  cancellation" section written against these rules.

## Status

Accepted (M27-001-C). Tests in
`crates/q-capabilities/src/operations.rs`:
`start_requires_ready_phase`, `deadlines_are_bounded_fail_closed`,
`settle_only_by_owner_only_when_pending`,
`cancel_only_cancellable_only_pending`, `expiry_applies_to_both_classes`,
`expired_generation_settlement_drops_as_expected_outcome`,
`terminal_states_reject_every_state_change`.
