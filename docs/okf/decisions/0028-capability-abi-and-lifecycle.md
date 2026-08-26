---
type: Architecture Decision Record
title: ADR-0028 Capability ABI and Lifecycle State Machine
status: accepted
date: 2026-08-26
implements: ADR-0018 (M2.7 capability linker track), ADR-0010 (capability-based plugins and services)
---

# ADR-0028: Capability ABI and Lifecycle State Machine

## Context

M2.6 closed with a deployable runtime whose only native capability is
the worker-owned timer. M2.7 adds the capability linker and a minimal
Web runtime (`runtime:console`, `runtime:timers`, `runtime:text`,
`runtime:url`, `runtime:abort`, then `runtime:fetch`/`runtime:crypto`
per `docs/okf/architecture/capabilities-and-services.md`). Before any
new capability module lands, the runtime needs one normative answer to
eight questions: install, lazy init, invocation ownership,
cancellation, drain, shutdown, versioning, and errors. ADR-0010 named
the capability principle (no ambient authority, link only declared
modules); this ADR freezes the lifecycle those modules obey.

## Decision

### 1. A capability is a named, versioned native module

Each capability has a `CapabilityId` (closed `runtime:*` namespace for
built-ins), an integer version, and declared dependencies. The exact
identity model, compatibility rules, and dependency syntax are
normative in M27-001-B; this ADR fixes only that identity exists at
link time and that **version conflicts fail before ready** — a pack
whose capability requirements cannot be satisfied never reaches the
serving phase.

### 2. The lifecycle is a closed state machine

```text
Declared -> Installed -> Ready -> Draining -> Quiesced
    \           \          \         \
     +-----------+----------+--------+--> Failed   (from any non-terminal phase)
```

Normative rules (implemented and test-pinned in
`crates/q-capabilities/src/lib.rs`):

- The phase vocabulary is closed: `Declared`, `Installed`, `Ready`,
  `Draining`, `Quiesced`, `Failed`.
- **No capability can start work outside `Ready`.** The operation-start
  guard rejects every other phase with a typed error.
- `Failed` and `Quiesced` are terminal. Every transition out of a
  terminal phase is an error.
- Illegal transitions are typed errors that never mutate state — no
  panics, no silent no-ops, no best-effort recovery.
- Version conflicts discovered during linking route the capability to
  `Failed` before `Ready`.

### 3. Install resolves before serving; nothing initializes at build

Capabilities are declared in the pack manifest, resolved at pack load
against the runtime's linked module set, and moved `Installed` only
when resolution succeeds. **Lazy init**: a capability initializes on
first demand after activation — never at compile time, never at pack
load (G-004 zero startup compilation extends to capability
initializers). The compiler's dependency resolver (M27-002) prunes
unused capability modules at build time.

### 4. Every operation has one owner

A native operation (timer start, fetch, random fill) is owned by the
invocation that started it, carried with the worker's slot and
generation checks. Owner identity, per-operation deadline state, and
the settlement table are normative in M27-001-C. This ADR fixes the
invariant: an operation without an owner is a bug, and settlement is
always delivered to the owner's generation or dropped as expired.

### 5. Cancellable or explicitly non-cancellable — nothing in between

Every operation declares its cancellation class at definition:

- **Cancellable**: the host can physically stop it (timer removal,
  aborted connect). Cancellation is idempotent.
- **Non-cancellable**: the operation is short, bounded, and completes
  on its own (a bounded CSPRNG fill); the declaration is explicit and
  reviewed, never accidental.

A capability cannot ship an operation that is neither. Drain
(M27-001-D) relies on this classification to reach quiescence.

### 6. Shutdown drains to quiescence or fails closed

On shutdown the capability enters `Draining`: new operations are
refused, cancellable operations are cancelled, non-cancellable ones
run to completion, all under a bounded deadline. Reaching
`Quiesced` within the deadline is the only success path; missing it
fails closed (`Failed`) and is reported — the runtime never exits
with silently abandoned operations. Bounded-shutdown mechanics are
normative in M27-001-D.

### 7. Errors are typed values

Lifecycle violations (`IllegalTransition`, `OpsOutsideReady`,
`Terminal`) and capability operational errors are typed enums with a
closed vocabulary. Crossing into JavaScript they follow the existing
problem model: expected failures are declared-status typed values;
unexpected host errors are redacted before leaving the host
(AGENTS.md constraint 9). No stringly-typed capability errors.

## Threat review

- **Ambient authority**: the state machine cannot grant authority;
  capabilities only reach `Ready` through declared manifest
  resolution, so undeclared modules stay `Declared` and inert. No
  Node-compat escape hatch exists by construction (constraint 14
  applies: same-process QuickJS runs trusted app code; capabilities
  bound the host surface, not hostile sandboxing).
- **Phase confusion**: the `OpsOutsideReady` guard removes the
  "use after drain" and "use before install" classes — both are typed
  rejections, and the exhaustive transition-matrix test pins every
  illegal edge.
- **Shutdown races**: `Draining` refuses new operations by rule; the
  bounded deadline plus fail-closed `Failed` path means a stuck
  capability surfaces at shutdown instead of hanging the process.
- **Version downgrade/conflict**: conflicts fail before `Ready`
  (rule 1), so a mismatched pack never serves a request through a
  wrong-version capability.
- **Error leakage**: capability errors stay typed and bounded; host
  internals are redacted per the existing problem encoder.

## Consequences

- `q-capabilities` stops being a placeholder: it owns the normative
  phase vocabulary and transition table every capability host obeys.
- M27-001-B (identity/version), M27-001-C (op owner/deadline), and
  M27-001-D (bounded shutdown) extend this skeleton without reopening
  the phase vocabulary.
- The existing timer capability must migrate onto this lifecycle when
  M27-004 ports it; until then it remains worker-owned with the same
  invariants (bounded, cancellable, generation-checked).
- Capability authors get a single document (`docs/beta/CAPABILITY_AUTHORS.md`,
  draft in this packet) describing the contract.

## Status

Accepted (M27-001-A). Tests:
`happy_path_declared_to_quiesced`, `ops_start_only_in_ready`,
`illegal_transitions_reject_without_mutation`,
`terminal_phases_reject_everything`,
`fail_is_reachable_from_every_non_terminal_phase`,
`drain_requires_ready_no_shortcut_from_installed`,
`version_conflict_fails_before_ready`.
