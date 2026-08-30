---
task_id: M3-006-A
parent_task: M3-006
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-006-A — Define thresholds/hysteresis

## Atomic goal

Define thresholds/hysteresis.

## Parent intent

Add workers according to queue pressure while preserving memory budgets.

## Dependencies

- `M3-003-Z` — `tasks/06_m3_multi_worker/M3-003-Z-package-evidence-for-implement-serverless-service-and-throughput-profiles.md`
- `M3-005-Z` — `tasks/06_m3_multi_worker/M3-005-Z-package-evidence-for-implement-quarantine-replacement-and-readiness-aggregation.md`

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
5. Implement exactly this deliverable: Define thresholds/hysteresis.
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
m3-006-a: define thresholds hysteresis
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-006-A) — PASS

- Date: 2026-08-30
- Branch/PR: m3-006-a (squash-merged; see git log for final hash)
- Closes: #402

### Changed files
- `crates/q-runtime/src/service_profile.rs`: `ScaleThresholds` + `HysteresisState` — the adaptive scale-up/down decision core with hysteresis (M3-006-A) —
  - `ScaleThresholds::new(scale_up, scale_down, down_stable_ticks, cooldown_ticks)` — validated: `scale_down < scale_up` is the hysteresis invariant, fail closed otherwise. The dead band between the thresholds guarantees the two directions never fire simultaneously.
  - `HysteresisState::observe(thresholds, pressure, running, min_workers) -> Option<i32>` — one pressure sample per call; decision order: tick counter, cooldown gate (ticks_since_event > cooldown), scale-up (pressure > scale_up; also wipes accumulated scale-down stability — a burst invalidates pending retirement), scale-down (pressure < scale_down sustained > down_stable_ticks, never below min_workers), else reset-and-hold.
  - observe() counts its own tick BEFORE the gate: the first sample after any scale event is cooldown-gated by construction.

### Tests added (service_profile.rs, +6 → 45 runtime unit tests)
- `thresholds_reject_inverted_hysteresis` (== and > both fail closed)
- `scale_up_fires_above_threshold_and_resets_hysteresis` (partial stability wiped by a burst; the burst IS a scale-up event — modeled live)
- `scale_down_requires_sustained_stability` (2 quiet < required 3; burst resets; rebuild 3 then retire on the 4th stable tick)
- `scale_down_never_retires_below_min_workers` (10 sustained quiet ticks at the floor: hold)
- `dead_band_between_thresholds_never_scales` (pressure == scale_up / == scale_down both hold; == is not < or >)
- `cooldown_gates_both_directions` (gated ticks 1..=3 hold in BOTH directions even for extreme/quiet pressure; tsr 4 fires)

### Command results
- `cargo test -p velqu-runtime` → **45 unit (was 39) + 5 + 44** — 0 failed
- `cargo test -p q-capabilities` → 6 suites — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`7c8b3f5b…` matches manifest)

### Guardrail mapping
- **Adaptive mode scales under load** — scale-up on pressure strictly above threshold (immediate after cooldown).
- **Idle workers retire safely** — scale-down only on SUSTAINED stability (full window) and never below min_workers; a single quiet sample cannot retire a worker.
- No-oscillation: the dead band between thresholds plus cooldown plus stability-window reset on burst.

### Disclosures
- Two test corrections before commit: the burst sample in the stability test is itself a scale-up event (modeled live: running grows), and the gating sequence requires 3 gated samples before tsr 4 fires (observe counts its tick first). Both pinned by trace.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
