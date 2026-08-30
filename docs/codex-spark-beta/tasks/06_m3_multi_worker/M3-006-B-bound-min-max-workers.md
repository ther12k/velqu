---
task_id: M3-006-B
parent_task: M3-006
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-006-B — Bound min/max workers

## Atomic goal

Bound min/max workers.

## Parent intent

Add workers according to queue pressure while preserving memory budgets.

## Dependencies

- `M3-006-A` — `tasks/06_m3_multi_worker/M3-006-A-define-thresholds-hysteresis.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Bound min/max workers.
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
m3-006-b: bound min max workers
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-006-B) — PASS

- Date: 2026-08-30
- Branch/PR: m3-006-b (squash-merged; see git log for final hash)
- Closes: #403

### Changed files
- `crates/q-runtime/src/service_profile.rs`: `WorkerBounds` — validated min/max worker bounds for adaptive mode (M3-006-B) —
  - Fail-closed constructor: min == 0 rejected (an empty fleet serves nothing), min > max rejected (inverted), max above `MAX_WORKERS` rejected (memory budget ceiling).
  - `initial` defaults to `min` (serverless cold start) or is clamped into [min, max] via `with_initial`.
  - `clamp_count(running)` — the scaler's floor/ceiling in one call; `at_floor(running)` / `at_ceiling(running)` — the scale-down/scale-up blockers the hysteresis loop queries.
- `crates/q-runtime/src/lib.rs`: `WorkerBounds` + `ScaleThresholds` re-exported.

### Tests added (+3 → 48 runtime unit tests)
- `bounds_fail_closed_on_invalid_configuration` (min 0 / inverted / above-ceiling rejected; valid shapes construct)
- `initial_count_clamps_into_bounds` (below-min clamps up, above-max clamps down, in-range kept; default == min)
- `scaler_floor_and_ceiling_query_the_bounds` (at_floor/at_ceiling semantics; clamp_count keeps running counts inside)

### Command results
- `cargo test -p velqu-runtime` → **48 unit (was 45) + 5 + 44** — 0 failed
- `cargo test -p q-capabilities` → 6 suites — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`7c8b3f5b…` matches manifest)

### Guardrail mapping
- **Preserving memory budgets** — max is the memory-budget ceiling, enforced at construction and queried by the scaler's at_ceiling gate.
- **Adaptive mode scales under load** — min is the capacity floor (at_floor blocks retirement); the hysteresis loop queries exactly these predicates.

### Disclosures
- One fmt pass after the new module. No behavior beyond the new bounds type.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
