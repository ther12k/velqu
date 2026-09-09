---
task_id: M3-005-D
parent_task: M3-005
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-005-D — Aggregate readiness from usable capacity

## Atomic goal

Aggregate readiness from usable capacity.

## Parent intent

Replace poisoned workers without keeping the whole service permanently unhealthy.

## Dependencies

- `M3-005-C` — `tasks/06_m3_multi_worker/M3-005-C-initialize-replacement-under-bounded-policy.md`

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
5. Implement exactly this deliverable: Aggregate readiness from usable capacity.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Poisoned worker receives no new requests.
- Replacement restores capacity.
- Repeated poison cannot create restart storm.
- Liveness/readiness semantics are correct.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Poison/replacement chaos tests.
- Readiness tests.
- Restart-rate metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-005-d: aggregate readiness from usable capacity
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-005-D) — PASS

- Date: 2026-08-30
- Branch/PR: m3-005-d (squash-merged; see git log for final hash)
- Closes: #399

### Changed files
- `crates/q-runtime/src/service_profile.rs`: `aggregate_readiness(usable, total) -> FleetReadiness` (M3-005-D) —
  - Readiness derives from what CAN serve, not from what exists: quarantined/replacing workers contribute nothing.
  - Fleet stays ready while at least one usable worker remains; readiness drops only at usable == 0.
  - Pure and deterministic: usable clamped to total (over-reporting clamps), degenerate totals clamp to 1; `FleetReadiness { usable, total, ready }` makes degradation observable.
- `crates/q-runtime/src/lib.rs`: re-exports `aggregate_readiness`, `FleetReadiness`.

### Tests added (service_profile.rs, +4 → 39 runtime unit tests)
- `readiness_is_true_while_any_worker_is_usable` (1-of-2 and 1-of-64 still serve)
- `readiness_is_false_only_when_nothing_is_usable` (usable 0; degenerate total clamps)
- `usable_is_capped_at_total_and_counts_degrade_monotonically` (over-report clamps; 4→0 monotonic ladder)
- `quarantine_lifecycle_reaches_degraded_then_ready_again` (the full M3-005 story: 4 healthy → 2 usable (ready, degraded) → 1 usable (still ready) → replacement restores 4)

### Command results
- `cargo test -p velqu-runtime` → **39 unit (was 35) + 5 + 44** — 0 failed
- `cargo test -p q-capabilities` → 6 suites — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; manifest refreshed (`7c8b3f5b…`) — re-exports land in the runtime artifact

### Guardrail mapping
- **Liveness/readiness semantics are correct** — readiness = at least one usable worker; degradation is observable via `usable`, and the lifecycle test proves degraded-but-ready through quarantine and restored-ready after replacement.

### Disclosures
- Test arithmetic slip (4-worker lifecycle story) caught by the suite before commit. Two verify iterations on legitimate artifact refreshes.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
