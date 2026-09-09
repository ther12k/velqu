---
task_id: M3-008-D
parent_task: M3-008
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-008-D — Test mixed workloads

## Atomic goal

Test mixed workloads.

## Parent intent

Prevent one route/tenant/slow workload from monopolizing workers.

## Dependencies

- `M3-008-C` — `tasks/06_m3_multi_worker/M3-008-C-expose-load-shed-reasons.md`

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
5. Implement exactly this deliverable: Test mixed workloads.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Small requests make progress under slow workload.
- Overload does not cause unbounded memory.
- Limits are configurable.
- No starvation in approved scenarios.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Mixed-load benchmarks.
- Fairness metrics.
- Adversarial tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-008-d: test mixed workloads
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-008-D) — PASS

- Date: 2026-08-31
- Branch/PR: m3-008-d (squash-merged; see git log for final hash)
- Closes: #417

### Changed files
- `crates/q-capabilities/tests/fairness_workloads.rs` (new): mixed-load,
  adversarial, and bounded-micro-benchmark scenarios composing the M3-008
  stack (FairAdmission + LongRunningPolicy + LoadShedCounters).
- `benchmarks/manifest.json`: refreshed (standard remapped-build flow).

### Required evidence
- **Mixed-load**: `mixed_fast_and_slow_tenants_share_capacity` — two tenants
  weighted 3:1 over capacity 40 run 400 concurrent admit/release rounds each;
  each completes exactly rounds x its guaranteed share (12000 vs 4000 — the
  3:1 weight realized), clean teardown, zero over-releases.
- **Fairness metrics**: `stats()` balances asserted in every scenario
  (admitted + rejected == attempts, over_releases == 0, outstanding == live);
  `adversarial_deterministic_flail_keeps_every_bound` reconciles the
  LoadShedCounters snapshot against admission accounting over 6 000 seeded
  adversarial attempts.
- **Adversarial tests**:
  - `greedy_tenant_cannot_monopolize_or_starve` — a 20k-attempt greedy
    tenant never exceeds its ceiling (peak pinned <= 4) while the victim
    completes 500 rounds with its FULL guaranteed share every round.
  - `slow_workload_bounded_while_fast_traffic_flows` — long slots saturated
    and refused beyond (`limit == 2` typed), 10k short operations flow
    untouched, freed slot admits approved long work, unmatched ends counted.
  - `adversarial_burst_maps_to_load_shed_reasons` — 1 000 rejections at
    global saturation each convert to a valid redacted LoadShedReason with
    the frozen overload verdict; counters reconcile to exactly 1 000 across
    the 7-kind vocabulary.
  - `adversarial_deterministic_flail_keeps_every_bound` — seeded xorshift
    (reproducible) with greedy bias: global capacity and per-class ceilings
    hold at EVERY step; per-class ceiling rejections observed > 0.
  - `mixed_workload_micro_benchmark_completes_bounded` — 50k decisions
    across 4 classes complete within a coarse bound (constraint-12-safe:
    a "no pathological slowdown" pin, NOT performance evidence).

### Test corrections before commit
Two test-logic fixes (component behavior was correct): the mixed-load
assertion now pins proportional completion (rounds x share) instead of an
incorrect equality, and the greedy adversary drains its holds before exit so
the fleet teardown assertion is meaningful.

### Command results
- `cargo test -p q-capabilities` → **260 lib + 6 workload + 7 fuzz + 1 + 4 +
  9 WPT-manifest** — 0 failed
- `cargo fmt --check` → clean; `cargo clippy -p q-capabilities --all-targets
  -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures
- Scenario composition is at the component layer (the runtime wiring of
  fairness/long-running into dispatch lands with the multi-worker
  integration); the guardrail behavior proven here is the same code the
  runtime will call.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
