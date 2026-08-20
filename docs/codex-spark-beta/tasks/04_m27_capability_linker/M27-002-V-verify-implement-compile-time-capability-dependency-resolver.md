---
task_id: M27-002-V
parent_task: M27-002
milestone: M27
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M27.md
commit_required: true
---

# M27-002-V — Verify Implement compile-time capability dependency resolver

## Atomic goal

Prove every acceptance criterion for parent task M27-002 without broadening scope.

## Parent intent

Resolve exactly which capabilities enter each application artifact.

## Dependencies

- `M27-002-A` — `tasks/04_m27_capability_linker/M27-002-A-build-dependency-dag.md`
- `M27-002-B` — `tasks/04_m27_capability_linker/M27-002-B-reject-cycles-missing-conflicting-versions.md`
- `M27-002-C` — `tasks/04_m27_capability_linker/M27-002-C-emit-capability-inventory-hash-into-qpack.md`
- `M27-002-D` — `tasks/04_m27_capability_linker/M27-002-D-remove-unused-modules.md`

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
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Unrelated app pays zero linked capability cost.
- Dependency graph is deterministic.
- Missing capability fails at build or startup.
- `velqu inspect --capabilities` is accurate.

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

- Resolver tests.
- Binary-size delta report.
- Cold-start delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-002-v: verify implement compile time capability dependency resolver
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
