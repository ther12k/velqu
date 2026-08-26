# Capability Author Guide (draft)

Status: **draft** — normative base is
[ADR-0028](../okf/decisions/0028-capability-abi-and-lifecycle.md).
Identity/version rules (M27-001-B), operation owner/deadline state
(M27-001-C), and bounded-shutdown mechanics (M27-001-D) will extend
this guide as they land. The capability SDK surface itself is M27-009.

## What a capability is

A capability is a named, versioned native module the runtime can link
for an application — `runtime:timers`, `runtime:url`,
`runtime:crypto`. Applications import capabilities explicitly; the
compiler resolves those imports to manifest requirements and the
runtime refuses to serve a pack whose requirements are unsatisfiable
or version-conflicted. There is no ambient Node compatibility and no
undeclared authority: if a module was not declared, it never reaches
the serving phase.

## The lifecycle your module obeys

```text
Declared -> Installed -> Ready -> Draining -> Quiesced
    \           \          \         \
     +-----------+----------+--------+--> Failed
```

Rules that apply to every capability, enforced by typed errors in
`q-capabilities`:

1. **Work starts only in `Ready`.** Operation starts in any other
   phase are rejected (`OpsOutsideReady`). There is no "warm up while
   draining" and no "serve before install".
2. **Version conflicts fail before ready.** If your module cannot
   satisfy the pack's requirement at link time, it goes to `Failed`
   and the pack never serves.
3. **Nothing initializes at build or pack load.** Initializers run on
   first demand after activation (G-004). Keep them small and bounded.
4. **`Failed` and `Quiesced` are terminal.** A failed capability is
   never silently revived; it fails closed and is reported.
5. **Shutdown drains under a deadline.** On `Draining`, new operations
   are refused; cancellable operations are cancelled; explicitly
   non-cancellable ones run to completion. Reaching `Quiesced` within
   the deadline is the only success path — missing it fails closed.

## Identity and versions

Your capability's id is a validated `namespace:name` string:

- The namespace vocabulary is closed — `runtime` is the only member
  today. `node:fs` is not a typo we repair; it is a typed rejection.
- Names use `[a-z0-9-]`, are non-empty, ≤ 48 bytes; the whole id is
  ≤ 64 bytes.
- Versions are integers compared **exactly**. A requirement for
  version 1 is not satisfied by version 2 — upgrades are explicit
  requirement bumps, never silent compatibility.

A pack states `CapabilityRequirement { id, version }`; the runtime
resolves it against the linked descriptors at install. `Missing` and
`VersionConflict` are typed failures that route the capability to
`Failed` before it can serve — your module will never observe
`Ready` with a wrong-version dependency (ADR-0029).

## Cancellation classes

Every operation you expose declares one of exactly two classes:

- **Cancellable** — the host can physically stop it mid-flight (timer
  removal, an aborted connect). Cancellation must be idempotent.
- **Explicitly non-cancellable** — the operation is short, bounded,
  and always completes on its own (a bounded CSPRNG fill). This is a
  reviewed declaration, not a default.

An operation that is neither is a design bug and will be rejected in
review. "Eventually times out" is not a cancellation class.

## Ownership

Every operation has exactly one owner: the invocation that started
it, identified by the worker slot and generation. Settlements
(resolve/reject) are delivered only to the owning generation; a
settlement arriving for an expired generation is dropped. You never
freehand callbacks into the engine — you complete operations through
the host's dispatch path.

## Operations, deadlines, and cancellation

Every operation your capability starts is a `NativeOp` with:

- **One owner** — the invocation that started it (the same
  slot/generation pair the request store checks). Settlements from
  anyone else are typed `NotOwner` errors. Late settlements for an
  expired generation are dropped as expected outcomes.
- **A bounded deadline** — 1..300,000 ms. Zero and over-ceiling
  values are typed rejections; the ceiling moves only by ADR.
- **A state from a closed vocabulary** — `Pending → Settled |
  Cancelled | Expired`, all terminal. Double settles, double
  cancels, and cancel-after-settle are typed illegal transitions —
  exactly-once is structural, and idempotency violations are visible
  in tests and logs.
- **A cancellation class chosen at start** — `Cancellable` or
  `NonCancellable`. Cancelling a non-cancellable operation is a
  typed rejection. There is no default class.

## Shutdown and drain

When the runtime shuts your capability down, the protocol is fixed
(ADR-0031):

1. **Drain begins** — new operations are refused from that instant.
2. **Cancellable pending operations are cancelled immediately**;
   non-cancellable ones form the await set and must complete on
   their own.
3. **The budget is 5 seconds, fail-closed.** Everything settled in
   time → `Quiesced`, with an accounting report (cancelled /
   settled / expired counts). Budget missed → stragglers are expired
   **visibly** and the capability fails closed (`Failed`). A late
   settlement for an expired operation is dropped; a failed
   capability is never revived.

Your module cannot opt out of the budget, hide pending work, or turn
a late drain into a success.

## Errors

Expected failures are typed values with declared statuses that cross
into JS as RFC 9457-compatible problems. Unexpected host errors are
redacted before leaving the host. Do not invent stringly-typed error
channels, and do not leak request objects or host internals in
messages.

## Review checklist (what we will ask you)

- [ ] Operations start only in `Ready` (state-machine tests cite the
      guard).
- [ ] Every operation declares cancellable or explicitly
      non-cancellable.
- [ ] Cancellation (where declared) is idempotent and tested.
- [ ] Initializer is lazy, bounded, and allocates predictably.
- [ ] Drain path reaches `Quiesced` or fails closed within the
      deadline (test both branches).
- [ ] Errors are typed; redaction holds for unexpected paths.
- [ ] No ambient authority: nothing is reachable that was not
      declared in the manifest.
