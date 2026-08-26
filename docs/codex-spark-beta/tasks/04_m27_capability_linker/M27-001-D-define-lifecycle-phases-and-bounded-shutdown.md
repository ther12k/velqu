---
task_id: M27-001-D
parent_task: M27-001
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-001-D — Define lifecycle phases and bounded shutdown

## Atomic goal

Define lifecycle phases and bounded shutdown.

## Parent intent

Specify install, lazy init, invocation ownership, cancellation, drain, shutdown, versioning, and errors for native capabilities.

## Dependencies

- `M27-001-C` — `tasks/04_m27_capability_linker/M27-001-C-define-native-operation-owner-deadline-state.md`

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
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/src/main.rs`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define lifecycle phases and bounded shutdown.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Lifecycle state tests.
- Capability author guide draft.
- Threat review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-001-d: define lifecycle phases and bounded shutdown
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-001-D (PASS)

Deliverable: lifecycle phases and bounded shutdown defined
(ADR-0031) — completing the M27-001 define set (A lifecycle, B
identity, C operations, D shutdown).

### Changed files

- `docs/okf/decisions/0031-bounded-capability-shutdown-and-quiescence.md`
  — new ADR: fail-closed 5,000 ms drain budget (ceiling moves only
  by ADR), the deterministic begin/drain/finish protocol
  (cancel pending cancellable; await non-cancellable; expire
  stragglers on missed budget + `Failed`), accounted quiescence as
  the only success outcome, no-honest-outcome rule for
  pending-without-deadline, threat review.
- `docs/okf/decisions/index.md` — ADR-0031 entry.
- `crates/q-capabilities/src/shutdown.rs` — new module:
  `SHUTDOWN_BUDGET_MS`, typed `ShutdownError`, `DrainOutcome`
  (Quiesced{cancelled,settled,expired} | DeadlineExceeded{pending}),
  `begin_shutdown`, `drain_step`, `finish_shutdown`.
- `crates/q-capabilities/src/lib.rs` — `pub mod shutdown` +
  re-exports.
- `docs/beta/CAPABILITY_AUTHORS.md` — "Shutdown and drain" section.
- `docs/codex-spark-beta/STATUS.md`, `indexes/TASK_INDEX.md` — this
  packet PASS.

### Tests

`cargo test -p q-capabilities` — 30 passed (23 prior + 7 shutdown):
`all_cancellable_operations_drain_to_quiesced`,
`non_cancellable_operations_settle_within_budget` (await set = 2,
accounting cancelled 1 / settled 2),
`missed_budget_expires_stragglers_and_fails_closed` (Failed
lifecycle, visible expiry, late settlement drops, quiesce
terminally refused), `empty_operation_set_quiesces_immediately`,
`draining_refuses_new_operations` (guardrail 1 on this path),
`shutdown_requires_ready_lifecycle` (never-served and
double-drain), `finish_with_pending_and_no_deadline_has_no_honest_outcome`
(lifecycle unchanged).

### Commands (fresh worktree on M27-001-C HEAD 6202356)

- `cargo test -p q-pack` 96 · `-p q-engine-quickjs` 98 ·
  `-p q-capabilities` 30 · `-p velqu-runtime` 30 — pass.
- `bun test` 125 pass / 0 fail; `bun run typecheck` clean.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.

### Notes

- Guardrail mapping: shutdown quiescence-or-fail-closed →
  `finish_shutdown` both branches pinned by tests; no-work-outside-
  phase reinforced via `draining_refuses_new_operations`.
- The parent M27-001 acceptance is now fully covered by the four
  define packets' combined test set (lifecycle 7 + identity 9 +
  operations 7 + shutdown 7 = 30); M27-001-V maps the guardrails
  across them.
