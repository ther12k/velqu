---
task_id: M4A-001-V
parent_task: M4A-001
milestone: M4A
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-001-V — Verify Implement actual-runtime `velqu dev` loop

## Atomic goal

Prove every acceptance criterion for parent task M4A-001 without broadening scope.

## Parent intent

Compile and reload the real QuickJS/QPack runtime with fast feedback and parity.

## Dependencies

- `M4A-001-A` — `tasks/07_m4a_developer_preview/M4A-001-A-watch-source-and-contracts.md`
- `M4A-001-B` — `tasks/07_m4a_developer_preview/M4A-001-B-build-incremental-temporary-qpack.md`
- `M4A-001-C` — `tasks/07_m4a_developer_preview/M4A-001-C-load-new-worker-before-switching-traffic.md`
- `M4A-001-D` — `tasks/07_m4a_developer_preview/M4A-001-D-drain-old-worker-and-surface-compile-runtime-errors.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No Bun-only behavior mismatch by default.
- Failed reload keeps prior healthy app.
- Source maps point to TypeScript.
- Reload is bounded and observable.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
bun test
```
```bash
bun run typecheck
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

- Reload conformance.
- Failure recovery tests.
- Developer latency measurements.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-001-v: verify implement actual runtime velqu dev loop
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
