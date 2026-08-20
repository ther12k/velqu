---
task_id: M3-006-Z
parent_task: M3-006
milestone: M3
priority: P1
mode: EVIDENCE
status: TODO
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
