---
task_id: M27-009-V
parent_task: M27-009
milestone: M27
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-009-V — Verify Publish capability SDK and inspection surface

## Atomic goal

Prove every acceptance criterion for parent task M27-009 without broadening scope.

## Parent intent

Make first-party and external capabilities implementable without internal runtime mutation.

## Dependencies

- `M27-009-A` — `tasks/04_m27_capability_linker/M27-009-A-define-rust-side-sdk-traits-and-metadata.md`
- `M27-009-B` — `tasks/04_m27_capability_linker/M27-009-B-provide-test-harness-and-example-capability.md`
- `M27-009-C` — `tasks/04_m27_capability_linker/M27-009-C-expose-build-inspect-diagnostics.md`
- `M27-009-D` — `tasks/04_m27_capability_linker/M27-009-D-define-semver-abi-compatibility.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `crates/q-pack/src/lib.rs`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Capability does not receive arbitrary mutable app state.
- SDK tests lifecycle/cancel/shutdown.
- Versioning is explicit.
- Example capability remains outside core.

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

- SDK docs.
- Example package.
- Compatibility tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-009-v: verify publish capability sdk and inspection surface
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
