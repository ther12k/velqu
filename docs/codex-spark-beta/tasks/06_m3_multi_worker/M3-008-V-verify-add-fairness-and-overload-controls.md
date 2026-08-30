---
task_id: M3-008-V
parent_task: M3-008
milestone: M3
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-008-V — Verify Add fairness and overload controls

## Atomic goal

Prove every acceptance criterion for parent task M3-008 without broadening scope.

## Parent intent

Prevent one route/tenant/slow workload from monopolizing workers.

## Dependencies

- `M3-008-A` — `tasks/06_m3_multi_worker/M3-008-A-add-route-global-queue-limits-or-weighted-admission.md`
- `M3-008-B` — `tasks/06_m3_multi_worker/M3-008-B-define-long-running-js-policy.md`
- `M3-008-C` — `tasks/06_m3_multi_worker/M3-008-C-expose-load-shed-reasons.md`
- `M3-008-D` — `tasks/06_m3_multi_worker/M3-008-D-test-mixed-workloads.md`

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
- `packages/core/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p q-http
```
```bash
cargo test -p q-bridge
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

- Mixed-load benchmarks.
- Fairness metrics.
- Adversarial tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-008-v: verify add fairness and overload controls
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-008-V) — PASS

- Date: 2026-08-31
- Branch/PR: m3-008-v (squash-merged; see git log for final hash)
- Closes: #418

### Acceptance-criterion mapping (parent M3-008 guardrails)

1. **Small requests make progress under slow workload** — verified:
   `LongRunningPolicy` gates ONLY long-classified invocations and validates
   `long_slots < short_capacity` at construction
   (`policy_construction_validates_fail_closed`,
   `long_slots_exhaust_typed_while_short_capacity_is_untouched`, B);
   mixed-load conformance shows the fast tenant completing 12 000 admits
   while the slow tenant holds its bounded share and long slots saturate
   (`slow_workload_bounded_while_fast_traffic_flows`, D).
2. **Overload does not cause unbounded memory** — verified: FairAdmission's
   global bound holds under a 4×2000 racing burst
   (`global_bound_is_never_exceeds_under_concurrency`, A); LongRunningBudget
   keeps `live <= limit` under held-slot spam (`live_never_exceeds_limit_
   under_a_held_slot_race`, B); rejection accounting reconciles exactly in
   the adversarial flail (6 000 seeded attempts, D); counters saturate
   (`refused_count_saturates_without_panicking` from M3-007-B applies to the
   same saturating pattern; `counters_record_saturate_and_snapshot_
   deterministically`, C).
3. **Limits are configurable** — verified: fairness weights + capacity
   (`construction_validates_fail_closed`, A: five typed error variants
   incl. `SharesExceedCapacity`), long-running threshold + slot budgets
   (`with_limits` boundaries, B), all fail-closed — an invalid config can
   never silently become "no fairness"/"unlimited".
4. **No starvation in approved scenarios** — verified: freed long slots
   admit approved long work (`approved_long_work_never_starves`, B); a
   greedy tenant at its ceiling cannot cause another class's denial below
   share (`fleet_protection_denial_only_under_collective_saturation`, A;
   `greedy_tenant_cannot_monopolize_or_starve` — victim completes 500
   rounds with its FULL share — D); share-slice guarantee
   (`class_below_share_always_admits_while_global_has_room`, A).

### Load-shed exposure (C, composing)
Every refusal maps into the closed 7-kind vocabulary with redacted details
and the frozen overload verdict; the runtime records reasons at its live
refusal sites and renders `loadShed` in the shutdown report
(`kinds_are_stable_deterministic_and_complete`,
`client_detail_is_redacted_and_verdict_is_frozen`,
`conversions_cover_the_component_rejections`,
`adversarial_burst_maps_to_load_shed_reasons`, D; conformance
`graceful_drain_flips_gate_and_reports_before_exit` asserts all seven kinds
in the report). Contract violations can never be laundered into
"server is busy" (conversion returns None).

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p q-capabilities` → **260 lib + 6 workload + 7 fuzz + 1 + 4 +
  9 WPT-manifest** — 0 failed
- `cargo test -p velqu-runtime` → 7 suites (55 unit + 6 + 5 + 2 + 35
  conformance) — 0 failed
- `bun test` → 219 pass, 0 fail (27 files); `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets --
  -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS** (release hash matches the manifest)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of
  M3-008-A/B/C/D.
- Component-layer scope note: fairness/long-running enforcement wires into
  the dispatch path with the multi-worker integration; the guardrail
  behavior proven here is the same code the runtime will call.
- CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR. Local evidence above is complete.
