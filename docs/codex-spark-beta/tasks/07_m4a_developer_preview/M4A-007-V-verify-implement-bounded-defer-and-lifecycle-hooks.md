---
task_id: M4A-007-V
parent_task: M4A-007
milestone: M4A
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-007-V — Verify Implement bounded `defer` and lifecycle hooks

## Atomic goal

Prove every acceptance criterion for parent task M4A-007 without broadening scope.

## Parent intent

Provide after-response cleanup/best-effort work without pretending it is durable jobs.

## Dependencies

- `M4A-007-A` — `tasks/07_m4a_developer_preview/M4A-007-A-define-deferred-owner-queue-deadline-cancellation-shutdown.md`
- `M4A-007-B` — `tasks/07_m4a_developer_preview/M4A-007-B-separate-cleanup-from-best-effort-work.md`
- `M4A-007-C` — `tasks/07_m4a_developer_preview/M4A-007-C-expose-metrics.md`
- `M4A-007-D` — `tasks/07_m4a_developer_preview/M4A-007-D-forbid-unbounded-recursive-spawning.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/ingress-bridge.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Response is not delayed beyond defined handoff.
- Deferred work is bounded.
- Shutdown handles or aborts it deterministically.
- Docs warn against durable-job use.

## Targeted commands

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

- Lifecycle tests.
- Load/cleanup tests.
- Operational docs.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-007-v: verify implement bounded defer and lifecycle hooks
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
