---
task_id: M4A-003-D
parent_task: M4A-003
milestone: M4A
priority: P1
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-003-D — Optional fetch/profile choices

## Atomic goal

Optional fetch/profile choices.

## Parent intent

Create a minimal correct project without hidden demo credentials or broad dependencies.

## Dependencies

- `M4A-003-C` — `tasks/07_m4a_developer_preview/M4A-003-C-testing-setup.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`
- `crates/q-runtime/src/main.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Optional fetch/profile choices.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Generated project builds/tests/runs.
- Starter follows module/service/contract best practices.
- No database/auth forced into core.
- Dependencies are minimal.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
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

- Scaffold snapshot tests.
- Fresh install test.
- Bundle-size report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-003-d: optional fetch profile choices
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
