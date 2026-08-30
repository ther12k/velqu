---
task_id: M3-006-Z
parent_task: M3-006
milestone: M3
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-006-Z — Package evidence for Implement adaptive scale-up and scale-down

## Atomic goal

Create source-backed evidence and handoff for parent task M3-006; update status only if verification passed.

## Parent intent

Add workers according to queue pressure while preserving memory budgets.

## Dependencies

- `M3-006-V` — `tasks/06_m3_multi_worker/M3-006-V-verify-implement-adaptive-scale-up-and-scale-down.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Adaptive load test.
- State transition tests.
- Memory report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m3-006-z: package evidence for implement adaptive scale up and scale d
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-006-Z) — PASS

- Date: 2026-08-30
- Branch/PR: m3-006-z (squash-merged; see git log for final hash)
- Closes: #407

### Parent closure — M3-006 Implement adaptive scale-up and scale-down

Parent intent: add workers according to queue pressure while preserving memory budgets. Status: **PASS**.

Packet commits (squash merges):
- M3-006-A — daa4c24 (#1006, Closes #402): `ScaleThresholds` + `HysteresisState` — hysteresis invariant (scale_down < scale_up, fail closed), dead band, stability window for retirement, cooldown gating both directions, min_workers floor
- M3-006-B — d9447d9 (#1007, Closes #403): `WorkerBounds` — validated min/max (min 0 / inverted / above-ceiling fail closed), initial clamping, floor/ceiling predicates for the scaler
- M3-006-C — 87324d3 (#1008, Closes #404): drain-before-scale-down lifecycle — `RetiringWorker` (Draining -> ReadyToTeardown), lossless re-dispatch, bounded drain budget escalation
- M3-006-D — c170c05 (#1009, Closes #405): `ScaleGovernor` — anti-oscillation outer bound (fixed-window event cap; adversarial flip-flop over 20 ticks yields exactly 5 events); found+fixed a real window-refill bug (cooldown field used instead of window length)
- M3-006-V — a1963df (#1010, Closes #406): verification closure mapping all 4 guardrails + anti-oscillation

### Required evidence
- **Adaptive load test**: `pressure_adds_one_worker_per_tick` (M3-003-B), `scale_up_fires_above_threshold_and_resets_hysteresis` (A), the adversarial flip-flop signal capped at 5 events/20 ticks (D)
- **State transition tests**: dead band (== never scales), stability window (burst resets), cooldown both directions, floor/ceiling blockers, drain lifecycle (Draining -> ReadyToTeardown lossless; budget escalation)
- **Memory report**: WorkerBounds.max is the memory-budget ceiling enforced at construction (M3-006-B); RSS trade-off documented in M3-003-V (serverless ~7.9 MB) with profile-differentiated scaling as M3-009's dedicated evidence

### Source/test map
- `crates/q-runtime/src/service_profile.rs`: ScaleThresholds/HysteresisState (A), WorkerBounds (B), RetirePhase/RetiringWorker (C), ScaleGovernor (D); 16 new tests (39 -> 55 profile tests total)
- `crates/q-runtime/src/lib.rs`: re-exports
- Release binary `7c8b3f5b…` matches manifest

### Command results (this branch)
- `cargo test -p velqu-runtime` → 55 unit + 5 + 44; `-p q-capabilities` → 6 suites; `-p q-engine-quickjs` → 20+102+1; `-p q-http` → 4+6+1; `-p q-bridge` → 11 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M3-006 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
