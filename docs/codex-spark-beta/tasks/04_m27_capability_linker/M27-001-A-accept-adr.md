---
task_id: M27-001-A
parent_task: M27-001
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-001-A — Accept ADR

## Atomic goal

Accept ADR.

## Parent intent

Specify install, lazy init, invocation ownership, cancellation, drain, shutdown, versioning, and errors for native capabilities.

## Dependencies

- `M26-GATE` — `gates/M26-GATE.md`

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
5. Implement exactly this deliverable: Accept ADR.
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
m27-001-a: accept adr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-001-A (PASS)

Deliverable: ADR-0028 accepted — the capability ABI and lifecycle
state machine decision.

### Changed files

- `docs/okf/decisions/0028-capability-abi-and-lifecycle.md` — new
  ADR: closed phase vocabulary (`Declared → Installed → Ready →
  Draining → Quiesced`, `Failed` from any non-terminal phase),
  ready-only operation starts, version conflicts fail before ready,
  lazy init (nothing at build/pack load, G-004), single-owner
  operations with generation-checked settlement, exactly two
  cancellation classes (cancellable / explicitly non-cancellable),
  bounded drain reaching quiescence or failing closed, typed errors,
  and a threat review section. Details deferred to the sibling
  packets: identity/versioning (M27-001-B), op owner/deadline state
  (M27-001-C), bounded-shutdown mechanics (M27-001-D).
- `docs/okf/decisions/index.md` — ADR-0028 entry.
- `crates/q-capabilities/src/lib.rs` — replaces the M1 placeholder
  with the normative `CapabilityPhase` / `CapabilityLifecycle`
  transition table + `LifecycleError` typed enum; transitions never
  mutate on failure; `start_op()` guards the allowed phase.
- `docs/beta/CAPABILITY_AUTHORS.md` — capability author guide draft
  (lifecycle rules, cancellation classes, ownership, errors, review
  checklist; SDK surface deferred to M27-009).
- `docs/codex-spark-beta/STATUS.md`, `indexes/TASK_INDEX.md` — this
  packet PASS.

### Tests (lifecycle state evidence)

`cargo test -p q-capabilities` — 7 passed, 0 failed:

- `happy_path_declared_to_quiesced`
- `ops_start_only_in_ready` (guardrail 1: every non-Ready phase
  rejects operation starts)
- `illegal_transitions_reject_without_mutation` (exhaustive 6×6
  matrix; terminal sources yield typed `Terminal`, others
  `IllegalTransition`; state unchanged on failure)
- `terminal_phases_reject_everything`
- `fail_is_reachable_from_every_non_terminal_phase`
- `drain_requires_ready_no_shortcut_from_installed`
- `version_conflict_fails_before_ready` (guardrail 3)

### Commands (fresh worktree on M26-GATE HEAD 27de9ef)

- `cargo test -p q-pack` 96 · `-p q-engine-quickjs` 98 ·
  `-p q-capabilities` 7 · `-p velqu-runtime` 30 — pass.
- `bun test` 125 pass / 0 fail; `bun run typecheck` clean.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.

### Notes

- Guardrail mapping: no-work-outside-phase → `start_op` guard +
  `ops_start_only_in_ready`; cancellable-or-declared → ADR-0028 §5 +
  author-guide checklist (op-level enforcement lands with
  M27-001-C/D); version conflicts fail before ready →
  `version_conflict_fails_before_ready`; shutdown quiescence-or-
  fail-closed → ADR-0028 §6 (mechanics in M27-001-D).
- The existing timer capability stays worker-owned until M27-004
  ports it onto this lifecycle (recorded in the ADR's Consequences).
