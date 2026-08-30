---
task_id: M3-005-V
parent_task: M3-005
milestone: M3
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-005-V — Verify Implement quarantine, replacement, and readiness aggregation

## Atomic goal

Prove every acceptance criterion for parent task M3-005 without broadening scope.

## Parent intent

Replace poisoned workers without keeping the whole service permanently unhealthy.

## Dependencies

- `M3-005-A` — `tasks/06_m3_multi_worker/M3-005-A-remove-quarantined-worker-from-dispatch.md`
- `M3-005-B` — `tasks/06_m3_multi_worker/M3-005-B-fail-settle-its-pending-work.md`
- `M3-005-C` — `tasks/06_m3_multi_worker/M3-005-C-initialize-replacement-under-bounded-policy.md`
- `M3-005-D` — `tasks/06_m3_multi_worker/M3-005-D-aggregate-readiness-from-usable-capacity.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
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
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p q-http
```
```bash
cargo test -p q-bridge
```
```bash
cargo test -p velqu-runtime
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

- Poison/replacement chaos tests.
- Readiness tests.
- Restart-rate metrics.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-005-v: verify implement quarantine replacement and readiness aggreg
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-005-V) — PASS

- Date: 2026-08-30
- Branch/PR: m3-005-v (squash-merged; see git log for final hash)
- Closes: #400

### Acceptance-criterion mapping (parent M3-005 guardrails)

1. **Poisoned worker receives no new requests** — verified: `select()` skips quarantined workers before the load scan; `settle_quarantined` recovers pending jobs (FIFO) so the poisoned runtime never executes them; a post-settle pop returns nothing. Tests: `quarantined_worker_receives_no_new_requests`, `quarantined_pending_work_never_reaches_the_poisoned_runtime`, `quarantine_closes_queue_and_is_idempotent`, `settle_requires_quarantine_state`.
2. **Replacement restores capacity** — verified: `replace()` returns the slot to Serving with a fresh bounded queue; the replacement is selectable immediately; `ReplacementPolicy::request_replacement` initializes under the bounded budget/cooldown. Tests: `replacement_restores_capacity_and_keeps_restart_history`, `replacement_initializes_under_budget`, `budget_window_resets_after_elapsing`.
3. **Repeated poison cannot create restart storm** — verified: the fixed-window budget rate-limits replacements to budget/window (100 rapid poison events → exactly 50 bounded replacements), and replace() restores the ORIGINAL fleet size only. Tests: `restart_storm_scenario_stays_bounded` (exact math), `repeated_poison_cycle_never_exceeds_initial_worker_count`, `cooldown_blocks_immediate_re_replacement`.
4. **Liveness/readiness semantics are correct** — verified: readiness derives from usable capacity; the fleet stays ready while ≥ 1 worker remains, degrades observably via the usable count, and restores after replacement. Tests: `quarantine_lifecycle_reaches_degraded_then_ready_again`, `readiness_is_true_while_any_worker_is_usable`, `readiness_is_false_only_when_nothing_is_usable`, `usable_is_capped_at_total_and_counts_degrade_monotonically`, `all_quarantined_means_no_selection`.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 6 suites (218 unit incl. 24 dispatch tests)
- `cargo test -p velqu-runtime` → 39+5+44 (incl. 16 profile tests: readiness/adaptive/replacement/startup bounds)
- `cargo test -p q-engine-quickjs` → 20+102+1; `-p q-http` → 4+6+1; `-p q-bridge` → 11 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`7c8b3f5b…` matches the M3-005-D manifest)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of M3-005-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
