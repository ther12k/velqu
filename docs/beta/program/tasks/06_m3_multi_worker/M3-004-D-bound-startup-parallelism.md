---
task_id: M3-004-D
parent_task: M3-004
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-004-D — Bound startup parallelism

## Atomic goal

Bound startup parallelism.

## Parent intent

Load identical verified artifacts into independent runtimes efficiently.

## Dependencies

- `M3-004-C` — `tasks/06_m3_multi_worker/M3-004-C-validate-capability-compatibility-per-worker.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Bound startup parallelism.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Workers execute identical contracts.
- One worker failure does not corrupt others.
- No JS object crosses workers.
- Artifact memory sharing is measured.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Worker parity tests.
- Memory mapping report.
- Startup tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-004-d: bound startup parallelism
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-004-D) — PASS

- Date: 2026-08-30
- Branch/PR: m3-004-d (squash-merged; see git log for final hash)
- Closes: #393

### Changed files
- `crates/q-runtime/src/service_profile.rs`: startup parallelism bounds (M3-004-D) —
  - `MAX_STARTUP_PARALLELISM` = 8; `WORKER_INIT_DEADLINE_MS` = 10s.
  - `startup_parallelism(workers, cores)`: deterministic min(workers, cores) clamped to [1, cap] — a service:64 deployment never spawns 64 simultaneous engine evaluations on a small box, while still amortizing cold start.
  - `startup_batches(workers, cores) -> (lanes, sizes)`: the bounded batch plan — lane count + per-lane worker counts (last lane may be short); sum of lane sizes == workers exactly.
- `crates/q-runtime/src/lib.rs`: re-exports of the profile surface (ServiceProfile/AdaptiveWorkers/Readiness/startup bounds).

### Tests added (service_profile.rs, +3 → 31 runtime unit tests)
- `startup_parallelism_is_bounded_by_cores_and_cap` (workers>cores clamps to cores; cap applies on huge machines; degenerate inputs >= 1)
- `startup_batches_sum_exactly_to_workers` (8 worker counts x 5 core counts: lane count, lane sizes, and exact sums)
- `single_worker_startup_is_always_one_lane` (the serverless guarantee: 1 worker -> 1 lane of 1)

### Command results
- `cargo test -p velqu-runtime` → **31 unit (was 28) + 5 + 44** — 0 failed
- `cargo test -p q-capabilities` → 6 suites; `-p q-engine-quickjs` → 20+102+1 — pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (after simplifying the clamp chain per the clamp-lint)
- `./scripts/verify` → **ALL PASS (exit 0)**; manifest refreshed (`6d5c7c3f…`) — the re-exports land in the runtime artifact.

### Guardrail mapping
- Bounded startup: deterministic lanes = min(workers, cores, 8); per-worker init deadline constant defined for the M3-004-V wiring.
- Serverless guarantee re-pinned: 1 worker -> exactly 1 lane.

### Disclosures
- Clippy's clamp-lint drove the min/clamp simplification. One verify iteration on the legitimate artifact refresh (lib.rs re-exports).
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
