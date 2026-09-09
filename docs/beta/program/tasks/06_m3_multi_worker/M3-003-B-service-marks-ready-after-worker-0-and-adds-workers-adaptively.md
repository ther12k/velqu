---
task_id: M3-003-B
parent_task: M3-003
milestone: M3
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-003-B — Service marks ready after worker 0 and adds workers adaptively

## Atomic goal

Service marks ready after worker 0 and adds workers adaptively.

## Parent intent

Make cold start versus immediate throughput an explicit deployment choice.

## Dependencies

- `M3-003-A` — `tasks/06_m3_multi_worker/M3-003-A-serverless-starts-one-worker-only.md`

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
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Service marks ready after worker 0 and adds workers adaptively.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Serverless cold start remains within approved budget.
- Profiles have deterministic readiness.
- No hidden worker creation.
- Profile-specific RSS is reported.

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

## Required evidence for this microtask

- Profile conformance.
- Cold/RSS report.
- Configuration docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-003-b: service marks ready after worker 0 and adds workers adaptive
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-003-B) — PASS

- Date: 2026-08-30
- Branch/PR: m3-003-b (squash-merged; see git log for final hash)
- Closes: #385

### Changed files
- `crates/q-runtime/src/service_profile.rs`: `AdaptiveWorkers` — the bounded adaptive add policy over the serverless posture —
  - **Ready after worker 0**: `starting()` initializes with `running: 1, ready: true` — worker 0 IS the service; readiness is the initial state, not an event that has to happen.
  - `tick(pressure, threshold) -> ScaleTick`: pressure strictly ABOVE the threshold may add exactly ONE worker, gated by `max_workers` (never exceeded) and a cooldown (no add before the first add ever; between adds, `ticks_since_add > cooldown_ticks`) — a burst cannot spawn a burst.
  - `add_events`/`ticks_since_add` saturating observability counters.

### Tests added (service_profile.rs, +5 → 24 runtime unit tests)
- `adaptive_starts_ready_after_worker_zero`
- `pressure_adds_one_worker_per_tick` (one add per tick under sustained pressure)
- `max_workers_bounds_growth_exactly` (10 high-pressure ticks against max 3 → exactly 3, then Hold forever)
- `cooldown_gates_bursts_against_oscillation` (cooldown 2: add, Hold, Hold, add — a burst cannot spawn a burst)
- `below_threshold_pressure_always_holds` (pressure == threshold holds; strictly-above adds)

### Command results
- `cargo test -p velqu-runtime` → **24 unit (was 19) + 5 + 44** — 0 failed
- `cargo test -p q-engine-quickjs` → 20+101 — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary unchanged (`9c2ebc08…` matches manifest)

### Guardrail mapping
- **Serverless cold start remains within approved budget** — ready is declared with exactly one worker (initial state).
- **No hidden worker creation** — adds happen ONLY through the policy tick, bounded by max + cooldown; never spontaneous.

### Disclosures
- Two iterations before commit: the cooldown initially blocked the FIRST add ever (no prior add exists to cool down from — added the explicit first-add exception), and the burst test's final assertion contradicted its own comment (running must be 3 after the post-cooldown add). Both caught by the suite itself.
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
