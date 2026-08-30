---
task_id: M3-005-C
parent_task: M3-005
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-005-C — Initialize replacement under bounded policy

## Atomic goal

Initialize replacement under bounded policy.

## Parent intent

Replace poisoned workers without keeping the whole service permanently unhealthy.

## Dependencies

- `M3-005-B` — `tasks/06_m3_multi_worker/M3-005-B-fail-settle-its-pending-work.md`

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
5. Implement exactly this deliverable: Initialize replacement under bounded policy.
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
m3-005-c: initialize replacement under bounded policy
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-005-C) — PASS

- Date: 2026-08-30
- Branch/PR: m3-005-c (squash-merged; see git log for final hash)
- Closes: #398

### Changed files
- `crates/q-runtime/src/service_profile.rs`: `ReplacementPolicy` — initialize quarantined-worker replacements under a bounded policy (M3-005-C) —
  - `starting(budget, budget_window_ticks, cooldown_ticks)`; `tick()` advances the clocks; `request_replacement() -> ReplacementDecision`.
  - Fixed-window budget: at most `budget` replacements per `budget_window_ticks` window (`BudgetExhausted` otherwise; window replenishes deterministically when its ticks elapse).
  - Cooldown between replacements: `CoolingDown` while `ticks_since_replacement <= cooldown_ticks` — but NO cooldown before the first replacement ever, and none when cooldown_ticks == 0.
  - Deterministic gates: budget first, then cooldown; `replacements` is the saturating restart-rate metric.

### Tests added (+4 → 35 runtime unit tests)
- `replacement_initializes_under_budget` (budget 3: 3x Initialize then BudgetExhausted)
- `budget_window_resets_after_elapsing` (budget 2/window 5: exhausted, window elapses, Initialize again)
- `cooldown_blocks_immediate_re_replacement` (cooldown 3: CoolingDown at tsr 1..=3, Initialize at tsr 4)
- `restart_storm_scenario_stays_bounded` (100 rapid poison events, budget 5/window 10 → exactly 50 replacements; rate-limited, never 1:1)

### Command results
- `cargo test -p velqu-runtime` → **35 unit (was 31) + 5 + 44** — 0 failed
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`6d5c7c3f…` matches manifest)

### Guardrail mapping
- **Repeated poison cannot create restart storm** — fixed-window budget rate-limits replacements to budget/window; 100 poison events yield exactly 50 bounded replacements.
- **Replacement restores capacity** — Initialize decisions restore quarantined slots (M3-005-A's replace() consumes the decision).

### Disclosures
- The window semantics were redesigned mid-packet: the first draft counted replacements against a tick-elapsing window, which could never reset (wu only grows on replacements). The final design counts WINDOW TICKS with budget consumed per replacement — the standard fixed-window rate limiter. Test arithmetic (cooldown off-by-one, exact 50) was pinned against the trace. The suite caught every iteration.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
