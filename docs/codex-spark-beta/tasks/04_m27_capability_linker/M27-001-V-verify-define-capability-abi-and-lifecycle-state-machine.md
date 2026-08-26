---
task_id: M27-001-V
parent_task: M27-001
milestone: M27
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-001-V — Verify Define capability ABI and lifecycle state machine

## Atomic goal

Prove every acceptance criterion for parent task M27-001 without broadening scope.

## Parent intent

Specify install, lazy init, invocation ownership, cancellation, drain, shutdown, versioning, and errors for native capabilities.

## Dependencies

- `M27-001-A` — `tasks/04_m27_capability_linker/M27-001-A-accept-adr.md`
- `M27-001-B` — `tasks/04_m27_capability_linker/M27-001-B-define-capabilityid-version-dependencies.md`
- `M27-001-C` — `tasks/04_m27_capability_linker/M27-001-C-define-native-operation-owner-deadline-state.md`
- `M27-001-D` — `tasks/04_m27_capability_linker/M27-001-D-define-lifecycle-phases-and-bounded-shutdown.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/src/main.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No capability can start work outside allowed phase.
- Every op is physically cancellable or explicitly non-cancellable.
- Version conflicts fail before ready.
- Shutdown reaches quiescence or fails closed.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Lifecycle state tests.
- Capability author guide draft.
- Threat review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-001-v: verify define capability abi and lifecycle state machine
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M27-001-V (PASS)

Parent: M27-001 "Define capability ABI and lifecycle state
machine". All four define packets merged before this branch:
M27-001-A (PR #842, #240), M27-001-B (PR #843, #241), M27-001-C
(PR #844, #242), M27-001-D (PR #845, #243).

### Acceptance criterion mapping (parent guardrails)

1. **No capability can start work outside allowed phase.**
   Lifecycle: `start_op()` guard + `ops_start_only_in_ready` (all
   six phases). Operations: `NativeOp::start` refuses with typed
   `NotReady` in every non-Ready phase
   (`start_requires_ready_phase`) and again while Draining
   (`draining_refuses_new_operations`). Identity: `resolve_and_install`
   only reaches `Installed` after resolution — nothing links, let
   alone serves, before that.

2. **Every op is physically cancellable or explicitly
   non-cancellable.** `CancellationClass` is chosen at start with no
   default (ADR-0030 §4); `cancel()` on a non-cancellable op is a
   typed `NotCancellable` rejection leaving state unchanged
   (`cancel_only_cancellable_only_pending`); drain applies the class
   structurally (`drain_step` cancels only cancellable; the await
   set is counted).

3. **Version conflicts fail before ready.**
   `resolve_and_install` routes `Missing`/`VersionConflict` to
   `Failed` before `Ready` — pinned by
   `resolve_and_install_conflict_fails_before_ready` and
   `resolve_and_install_missing_fails_lifecycle_before_ready`
   (activate terminally refused). Exact version matching only
   (ADR-0029 §2).

4. **Shutdown reaches quiescence or fails closed.**
   `finish_shutdown` has exactly two success/failure outcomes:
   accounted `Quiesced` when nothing is pending;
   `DeadlineExceeded` on a missed budget — stragglers visibly
   `Expired`, lifecycle `Failed`, late settlements drop, quiesce
   terminally refused
   (`missed_budget_expires_stragglers_and_fails_closed`,
   `all_cancellable_operations_drain_to_quiesced`,
   `non_cancellable_operations_settle_within_budget`). The
   pending-without-deadline state has no invented outcome
   (`finish_with_pending_and_no_deadline_has_no_honest_outcome`).

### Required evidence

- Lifecycle state tests: 30 in `q-capabilities` (7 lifecycle + 9
  identity + 7 operations + 7 shutdown), including the exhaustive
  6×6 transition matrix and 3×3 terminal matrix.
- Capability author guide draft: `docs/beta/CAPABILITY_AUTHORS.md`
  (lifecycle, identity/versions, operations/deadlines/cancellation,
  shutdown/drain, errors, review checklist).
- Threat reviews: ADR-0028, ADR-0029, ADR-0030, ADR-0031 each carry
  one; closed namespaces, exact versions, owner checks, bounded
  deadlines, visible abandonment.

### Changed files

- This task record only. No defects found; no follow-up tasks
  needed. The existing timer capability stays worker-owned until
  the M27-004 port (documented in ADR-0028 Consequences).

### Commands and results (fresh worktree on parent HEAD 7c690fd)

- `cargo test -p q-pack` 96 · `-p q-router` 15 ·
  `-p q-engine-quickjs` 98 · `-p q-capabilities` 30 ·
  `-p velqu-runtime` 30 — pass.
- `bun test` 125 pass / 0 fail; `bun run typecheck`,
  `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.
- `./scripts/verify` — ALL PASS (exit 0).
