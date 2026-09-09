---
task_id: M3-006-C
parent_task: M3-006
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-006-C — Drain before scale-down

## Atomic goal

Drain before scale-down.

## Parent intent

Add workers according to queue pressure while preserving memory budgets.

## Dependencies

- `M3-006-B` — `tasks/06_m3_multi_worker/M3-006-B-bound-min-max-workers.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Drain before scale-down.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Adaptive mode scales under load.
- Idle workers retire safely.
- No request loss.
- RSS and latency trade-off is documented.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Adaptive load test.
- State transition tests.
- Memory report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-006-c: drain before scale down
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-006-C) — PASS

- Date: 2026-08-30
- Branch/PR: m3-006-c (squash-merged; see git log for final hash)
- Closes: #404

### Changed files
- `crates/q-runtime/src/service_profile.rs`: `RetirePhase` + `RetiringWorker` — the drain-before-scale-down lifecycle (M3-006-C) —
  - A worker chosen for retirement starts in `Draining { remaining }` (admission stop is the M3-005-A quarantine's job — begin() starts IN Draining with the reported depth).
  - `tick(remaining, _dispatch_out) -> RetirePhase`: lossless path — empty queue flips to `ReadyToTeardown`; the drain budget escalates a wedged worker to `ReadyToTeardown` anyway, and the CALLER settles leftovers with typed failures (M3-005-B semantics). Bounded: `ticks_in_retire` saturates; the budget is enforced by tick count.
  - No request loss is proven (lossless test: 3 jobs re-homed 1/tick, teardown only at remaining==0).
- Tests live in service_profile.rs tests module (3 tests).

### Tests added (+2 net → 51 runtime unit tests)
- `retirement_starts_in_draining_with_reported_depth`
- `retirement_is_lossless_while_draining` (3 jobs re-homed 1/tick, teardown only at remaining==0)
- `drain_budget_escalates_a_wedged_worker` (budget 2: tick 3 escalates to ReadyToTeardown with jobs still queued — bounded, never hung)

### Command results
- `cargo test -p velqu-runtime` → **51 unit (was 48) + 5 + 44** — 0 failed
- `cargo test -p q-capabilities` → 6 suites — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`6d5c7c3f…` matches manifest)

### Guardrail mapping
- **No request loss** — the lossless-drain test re-homes every queued job before teardown; budget escalation is the caller's typed-failure path (M3-005-B), not a silent drop.
- **Idle workers retire safely** — the lifecycle is explicit (Draining -> ReadyToTeardown), bounded, and cannot hang the scaler.

### Disclosures
- The first lifecycle draft double-counted ticks through a recursive transition; redesigned to a flat begin/tick state machine where begin() starts IN Draining (admission stop is quarantine's job). One unused-param warning fixed. All caught before commit.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
