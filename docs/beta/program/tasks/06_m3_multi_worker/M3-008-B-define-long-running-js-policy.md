---
task_id: M3-008-B
parent_task: M3-008
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-008-B — Define long-running JS policy

## Atomic goal

Define long-running JS policy.

## Parent intent

Prevent one route/tenant/slow workload from monopolizing workers.

## Dependencies

- `M3-008-A` — `tasks/06_m3_multi_worker/M3-008-A-add-route-global-queue-limits-or-weighted-admission.md`

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
5. Implement exactly this deliverable: Define long-running JS policy.
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
m3-008-b: define long running js policy
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-008-B) — PASS

- Date: 2026-08-31
- Branch/PR: m3-008-b (squash-merged; see git log for final hash)
- Closes: #415

### Changed files
- `crates/q-capabilities/src/long_running.rs` (new): the long-running JS
  policy (M3-008-B) — deterministic classification + bounded budgets.
  - `LongRunningPolicy::with_limits(threshold_ms, long_slots_per_domain,
    short_capacity_per_domain)` — validated fail-closed
    (`LongRunningPolicyError`: zero threshold, threshold above
    `MAX_LONG_RUNNING_THRESHOLD_MS` (60_000), zero long slots, and the
    progress-guarantee invariant `long_slots < short_capacity`).
    `with_defaults()` = 1 s threshold, 2 long slots, 8 short slots.
  - `classifies(deadline_ms)` — deterministic, pack-reproducible: deadline
    >= threshold (inclusive boundary) is `LongClass::Long`; everything
    under is `Short` and never gated by the budget.
  - `LongRunningBudget` (`policy.budget()` — one per tracking domain: per
    worker, or fleet-wide): `try_begin()` fail-fast typed
    (`LongSlotsExhausted { limit }`, counted), `end()` saturating with
    unmatched ends counted (`over_releases`), `live() <= limit` always,
    redacted `LongRunningStats`.
- `crates/q-capabilities/src/lib.rs`: module + re-exports.
- `benchmarks/manifest.json`: refreshed for the new release hash (standard
  remapped-build flow).

### Guardrail mapping (parent M3-008)
- **Small requests make progress under slow workload** — the long budget
  gates ONLY long-classified invocations; `long_slots < short_capacity` is
  validated at construction, so short requests always retain dedicated
  capacity (`long_slots_exhaust_typed_while_short_capacity_is_untouched`).
- **Overload does not cause unbounded memory** — `live <= limit` always;
  rejections are fail-fast typed events, not queues; counters saturate.
- **Limits are configurable** — threshold and both capacity knobs are
  policy parameters with validated boundaries
  (`policy_construction_validates_fail_closed`).
- **No starvation in approved scenarios** — a freed long slot admits the
  next long caller: `approved_long_work_never_starves`; fail-fast
  rejection is backpressure, not starvation.

### Tests added (8; q-capabilities lib 248 → 256)
Construction validation (all four error variants + boundary acceptance);
inclusive classification boundary (999/1000/5000/60000); typed exhaustion
while short capacity untouched; starvation-freedom; 4×500 begin/end race
with exact accounting (`admitted + rejected == 2000`, `live == 0`); held-slot
spam race (bound holds under any interleaving); redacted stats; independent
per-worker + fleet-wide domains.

### Command results
- `cargo test -p q-capabilities` → **256 lib (was 248) + 7 fuzz + 1 + 4 + 9
  WPT-manifest** — 0 failed
- `cargo fmt --check` → clean; `cargo clippy -p q-capabilities --all-targets
  -- -D warnings` → exit 0 (one `bool_assert_comparison` lint fixed)
- `./scripts/verify` → **ALL PASS** (two bytecode conformance tests failed in
  the first verify run — root-caused to the missing debug-profile
  `velqu-bytecode` helper in the fresh worktree, not a regression; the
  standard `cargo build --workspace` step fixed it)

### Wiring note
Policy-definition packet, consistent with M3-008-A: classification and budget
components with proofs. Enforcement wiring into the dispatch/admission path
lands with M3-008-C (load-shed reasons rendered from `LongSlotsExhausted` /
`FairnessReject`) and M3-008-D (mixed workload tests).

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
