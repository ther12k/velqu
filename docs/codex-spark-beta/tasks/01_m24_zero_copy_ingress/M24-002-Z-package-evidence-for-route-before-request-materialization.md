---
task_id: M24-002-Z
parent_task: M24-002
milestone: M24
priority: P0
mode: EVIDENCE
status: TODO
context_card: context/milestones/M24.md
commit_required: true
---

# M24-002-Z — Package evidence for Route before request materialization

## Atomic goal

Create source-backed evidence and handoff for parent task M24-002; update status only if verification passed.

## Parent intent

Avoid query/header/body work for routes that do not declare it.

## Dependencies

- `M24-002-V` — `tasks/01_m24_zero_copy_ingress/M24-002-V-verify-route-before-request-materialization.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- C0/C1 perform no query parse, header clone, cookie parse, or body collect.
- 404/405 does not materialize request bodies.
- Malformed oversized inputs fail within coarse ingress budgets.
- Routing behavior remains contract-equivalent.

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

- Admission counters.
- Negative body/header budget tests.
- Perf stage timings.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m24-002-z: package evidence for route before request materialization
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
