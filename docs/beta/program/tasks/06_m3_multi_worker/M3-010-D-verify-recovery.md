---
task_id: M3-010-D
parent_task: M3-010
milestone: M3
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-010-D — Verify recovery

## Atomic goal

Verify recovery.

## Parent intent

Prove sustained service stability and worker replacement.

## Dependencies

- `M3-010-C` — `tasks/06_m3_multi_worker/M3-010-C-track-retained-memory-and-task-slot-counts.md`

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
5. Implement exactly this deliverable: Verify recovery.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No monotonic leak.
- Capacity recovers after replacement.
- No boundary violations.
- All errors are bounded and explained.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Soak raw data.
- Chaos timeline.
- Leak analysis.
- [ ] Independent workers scale across cores with bounded queues.
- [ ] Serverless mode preserves one-worker cold-start behavior.
- [ ] Quarantine/replacement and readiness are reliable.
- [ ] Cancellation/shutdown remain exact.
- [ ] Scaling, memory, fairness, and soak evidence pass.
- 1/2/4 worker C1/C2/C3.
- Controlled I/O at c=10/50/200.
- Mixed slow/fast fairness.
- Poison/replacement soak.
- No shared mutable JavaScript heap.
- No distributed cluster coordinator.
- No hostile tenant isolation claim.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m3-010-d: verify recovery
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-010-D) — PASS

- Date: 2026-08-31
- Branch/PR: m3-010-d (squash-merged; see git log for final hash)
- Closes: #429

### Changed files
- `crates/q-capabilities/tests/recovery.rs` (new): dedicated recovery
  integration tests (3 tests) —
  - `capacity_recovers_to_full_parallelism_after_worker_replacement`:
    proves quarantined slot stops receiving work; replacement returns
    slot to serving; least-outstanding selection equalizes queue loads.
  - `no_leaked_invocations_or_slots_across_repeated_poison_and_recovery`:
    50 rapid poison/settle/replace cycles under concurrent producer load
    with `InvocationOwnership` tracking; proves `pending == 0`,
    `registered == settled == 2000`, zero duplicate rejections, zero leaked
    slots.
  - `least_outstanding_converges_loads_after_drain_and_rebuild`:
    4-worker topology with 2 drained/rebuilt workers; proves subsequent
    dispatches route exclusively to underloaded recovered workers until
    all 4 equalize at capacity.
- `docs/reports/m3-010-d-recovery.md` (new): recovery analysis from the
  14-replacement soak timeline + unit test mapping and artifact hashes.

### Guardrail mapping (parent M3-010)
- **Capacity recovers after replacement** — proven: 14/14 soak replacements
  restored full throughput (~2.4k ops/s); unit tests prove load
  equalization to full 2-worker and 4-worker parallelism.
- **No monotonic leak** — proven: per-worker heap flat (+416 B / +640 B)
  across 14 replacements.
- **All errors bounded and explained** — proven: zero unexplained errors.
- **No boundary violations** — proven: verify's scheduler suite passes.

### Command results
- `cargo test -p q-capabilities` → **260 lib + 6 workload + 3 recovery + 7 fuzz + 1 + 4 + 9 WPT** — 0 failed
- `cargo test -p q-engine-quickjs` → 20 + 102 + 1 — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → 219 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
