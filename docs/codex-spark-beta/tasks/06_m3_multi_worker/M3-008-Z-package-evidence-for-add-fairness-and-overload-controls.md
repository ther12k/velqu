---
task_id: M3-008-Z
parent_task: M3-008
milestone: M3
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-008-Z — Package evidence for Add fairness and overload controls

## Atomic goal

Create source-backed evidence and handoff for parent task M3-008; update status only if verification passed.

## Parent intent

Prevent one route/tenant/slow workload from monopolizing workers.

## Dependencies

- `M3-008-V` — `tasks/06_m3_multi_worker/M3-008-V-verify-add-fairness-and-overload-controls.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`
- `crates/q-engine/src/lib.rs`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m3-008-z: package evidence for add fairness and overload controls
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-008-Z) — PASS

- Date: 2026-08-31
- Branch/PR: m3-008-z (squash-merged; see git log for final hash)
- Closes: #419
- Parent verification: M3-008-V PASS (PR #1022, merged 5415b34) on the
  identical tree; this packet packages the evidence and flips the ledger.

### Evidence package (parent M3-008 — fairness and overload controls)
- **Implementation commits (squash-merged):**
  - M3-008-A weighted admission — #1018 → 6bbc83d
  - M3-008-B long-running policy — #1019 → 1204d1f
  - M3-008-C load-shed reasons — #1020 → 72ee158
  - M3-008-D mixed workloads — #1021 → 600351d
  - M3-008-V verification closure — #1022 → 5415b34
- **Source paths:** `crates/q-capabilities/src/fairness.rs`
  (FairAdmission), `crates/q-capabilities/src/long_running.rs`
  (LongRunningPolicy/LongRunningBudget),
  `crates/q-capabilities/src/load_shed.rs` (LoadShedReason/
  LoadShedCounters), `crates/q-runtime/src/serve.rs` + `lib.rs`
  (counter wiring, `loadShed` in the shutdown report),
  `crates/q-capabilities/tests/fairness_workloads.rs` (mixed/adversarial
  conformance).
- **Key tests:** `global_bound_is_never_exceeds_under_concurrency`,
  `class_below_share_always_admits_while_global_has_room`,
  `fleet_protection_denial_only_under_collective_saturation` (A);
  `policy_construction_validates_fail_closed`,
  `approved_long_work_never_starves`,
  `live_never_exceeds_limit_under_a_held_slot_race` (B);
  `client_detail_is_redacted_and_verdict_is_frozen`,
  `conversions_cover_the_component_rejections` (C);
  `mixed_fast_and_slow_tenants_share_capacity`,
  `greedy_tenant_cannot_monopolize_or_starve`,
  `adversarial_deterministic_flail_keeps_every_bound` (D).
- **Resource invariant report:** every scenario reconciles
  admitted + shed == attempts with `over_releases == 0`; the runtime
  shutdown report carries `loadShed` (all 7 kinds) alongside the
  invocations/drain blocks from M3-007.
- **Full gate results (this branch, worktree-fresh):** `./scripts/verify`
  **ALL PASS** (incl. q-capabilities 260+6+7+1+4+9, bun 183 scoped tests,
  fmt, workspace clippy -D warnings).

### Ledger
- `docs/beta/04_TASK_LEDGER.md`: M3-008 TODO → **PASS** (all four
  guardrails proven; see the M3-008-V mapping).

### Disclosures (standing)
- No runtime behavior changed in this packet: evidence-only closure.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
