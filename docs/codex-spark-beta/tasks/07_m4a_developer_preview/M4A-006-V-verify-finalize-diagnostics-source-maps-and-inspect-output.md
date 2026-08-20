---
task_id: M4A-006-V
parent_task: M4A-006
milestone: M4A
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-006-V — Verify Finalize diagnostics, source maps, and inspect output

## Atomic goal

Prove every acceptance criterion for parent task M4A-006 without broadening scope.

## Parent intent

Make compile, startup, contract, capability, and runtime failures actionable.

## Dependencies

- `M4A-006-A` — `tasks/07_m4a_developer_preview/M4A-006-A-structured-diagnostic-codes.md`
- `M4A-006-B` — `tasks/07_m4a_developer_preview/M4A-006-B-source-map-aware-stacks.md`
- `M4A-006-C` — `tasks/07_m4a_developer_preview/M4A-006-C-redaction-policy.md`
- `M4A-006-D` — `tasks/07_m4a_developer_preview/M4A-006-D-inspect-route-plan-fields-codecs-capabilities-crossings-and-debug-names.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No secrets in production diagnostics.
- Errors identify route/source/contract cause.
- Source maps are lazy on success path.
- Diagnostic catalog exists.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
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

- Golden diagnostics.
- Redaction tests.
- Source-map tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-006-v: verify finalize diagnostics source maps and inspect output
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
