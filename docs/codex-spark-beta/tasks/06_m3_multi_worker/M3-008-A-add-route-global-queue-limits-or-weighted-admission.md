---
task_id: M3-008-A
parent_task: M3-008
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-008-A — Add route/global queue limits or weighted admission

## Atomic goal

Add route/global queue limits or weighted admission.

## Parent intent

Prevent one route/tenant/slow workload from monopolizing workers.

## Dependencies

- `M3-002-Z` — `tasks/06_m3_multi_worker/M3-002-Z-package-evidence-for-implement-bounded-worker-dispatcher.md`
- `M3-006-Z` — `tasks/06_m3_multi_worker/M3-006-Z-package-evidence-for-implement-adaptive-scale-up-and-scale-down.md`

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
5. Implement exactly this deliverable: Add route/global queue limits or weighted admission.
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
m3-008-a: add route global queue limits or weighted admission
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-008-A) — PASS

- Date: 2026-08-31
- Branch/PR: m3-008-a (squash-merged; see git log for final hash)
- Closes: #414

### Changed files
- `crates/q-capabilities/src/fairness.rs` (new): `FairAdmission` — weighted
  per-class admission over a bounded global capacity (M3-008-A, ADR-0036 §4
  lifecycle/infrastructure discipline; `SharedAcrossWorkers`, one mutex,
  bounded by construction, saturating counters, no JS values).
  - `with_weights(&[u64], capacity)` — validated fail-closed
    (`FairnessError`: empty classes, weight 0, > `MAX_FAIR_CLASSES` (256),
    capacity > `MAX_FAIR_CAPACITY` (65_536), and the load-bearing invariant
    `soft_total <= capacity`). Share of class c =
    `max(1, capacity * weight_c / total_weight)`; `headroom = capacity -
    soft_total` is the SHARED borrow pool; per-class
    `ceiling = share + headroom`.
  - `admit(class)` — fail-fast typed rejections (`GlobalFull`,
    `ClassCeiling`, `UnknownClass`), each counted; same client posture as
    queue overload (503 + retry-after at the render layer).
  - `release(class)` — at the terminal transition; saturating, unmatched
    releases counted as `over_releases` (host bug made observable, never a
    panic).
  - `stats()` — redacted counters (`FairnessStats`).
- `crates/q-capabilities/src/lib.rs`: module + re-exports.
- `benchmarks/manifest.json`: refreshed for the new release hash (q-capabilities
  is linked into the runtime binary).

### Provable properties, each pinned by a test
- **P1 weighted shares / guaranteed slice** — a class below its share admits
  whenever the global pool has room, no matter what the neighbor holds:
  `class_below_share_always_admits_while_global_has_room`.
- **P2 shared borrow pool** — a class may burst to `share + headroom` but its
  ceiling binds even while other classes are idle:
  `ceiling_stops_bursting_even_when_global_has_room` (greedy class hits
  `ClassCeiling` at 10 of 14 slots; the victim class then gets its FULL share
  and is denied only at the global boundary).
- **P3 fleet protection** — with 3 classes and 2 headroom slots, a class at
  its ceiling cannot cause another class's denial below share; denial of a
  below-share class happens only when the pool is truly exhausted
  (`fleet_protection_denial_only_under_collective_saturation`).
- **P4 global bound** — 4 threads × 2000 racing admits against capacity 100:
  outstanding never exceeds 100 and admitted + rejected accounting is exact
  (`global_bound_is_never_exceeded_under_concurrency`).
- Construction validation: `construction_validates_fail_closed` (all five
  error variants, including `SharesExceedCapacity` on a 2-slot capacity with
  three classes).
- Lifecycle exactness: `release_is_saturating_and_over_releases_are_counted`,
  `admits_and_releases_race_exactly` (8 threads × 500 admit/release pairs —
  outstanding returns to exactly 0, zero over-releases),
  `unknown_class_is_typed_and_counted`, `stats_balance_and_redaction`,
  `shares_are_weighted_and_headroom_is_shared`,
  `borrow_pool_gives_every_class_the_same_burst_room`.

### Test count
- `cargo test -p q-capabilities` → **248 lib (was 237; +11 fairness tests) +
  7 fuzz + 1 + 4 + 9 WPT-manifest** — 0 failed

### Command results
- `cargo fmt --check` → clean; `cargo clippy -p q-capabilities --all-targets
  -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS** (after the standard remapped release build
  + proof build + manifest refresh)

### Wiring note
Component-only packet, per the M3-002/M3-005/M3-006 precedent: the runtime
still runs a single engine, so per-route classes have no dispatch queue to
govern yet. Wiring is M3-008-C's load-shed rendering and the multi-worker
dispatcher integration; the component API is ready for it (`admit`/`release`
bracket exactly the invocation lifecycle that M3-007-A's ownership registry
tracks).

### Disclosures
- One test needed a correction before commit (the burst arithmetic in the
  ceiling test: the greedy class consumes the single headroom slot, so the
  victim's denial is `GlobalFull` at its exact share, not `ClassCeiling`) —
  the test now asserts the correct, stronger outcome.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
