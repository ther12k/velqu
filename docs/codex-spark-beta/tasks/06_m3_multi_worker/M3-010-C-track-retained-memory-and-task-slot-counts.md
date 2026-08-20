---
task_id: M3-010-C
parent_task: M3-010
milestone: M3
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M3.md
commit_required: true
---

# M3-010-C — Track retained memory and task/slot counts

## Atomic goal

Track retained memory and task/slot counts.

## Parent intent

Prove sustained service stability and worker replacement.

## Dependencies

- `M3-010-B` — `tasks/06_m3_multi_worker/M3-010-B-inject-worker-poison-upstream-timeout-disconnect-and-shutdown.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Track retained memory and task/slot counts.
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
cargo test -p q-http
```
```bash
cargo test -p q-bridge
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
m3-010-c: track retained memory and task slot counts
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
