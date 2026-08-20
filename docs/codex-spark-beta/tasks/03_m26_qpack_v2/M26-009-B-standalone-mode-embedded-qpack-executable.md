---
task_id: M26-009-B
parent_task: M26-009
milestone: M26
priority: P1
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M26.md
commit_required: true
---

# M26-009-B — Standalone mode: embedded qpack executable

## Atomic goal

Standalone mode: embedded qpack executable.

## Parent intent

Support both small app updates and one-file deployment.

## Dependencies

- `M26-009-A` — `tasks/03_m26_qpack_v2/M26-009-A-shared-mode-velqu-runtime-plus-app-qpack.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-runtime/src/serve.rs`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Standalone mode: embedded qpack executable.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Both modes pass identical conformance.
- Standalone contains no compiler toolchain.
- Shared mode rejects mismatched runtime.
- Startup/RSS differences are measured.

## Targeted commands

```bash
cargo test -p q-pack
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

- Artifact smoke tests.
- Size/cold-start report.
- Install guide.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-009-b: standalone mode embedded qpack executable
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
