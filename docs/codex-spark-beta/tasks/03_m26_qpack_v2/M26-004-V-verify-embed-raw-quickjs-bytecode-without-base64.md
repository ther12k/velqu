---
task_id: M26-004-V
parent_task: M26-004
milestone: M26
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M26.md
commit_required: true
---

# M26-004-V — Verify Embed raw QuickJS bytecode without base64

## Atomic goal

Prove every acceptance criterion for parent task M26-004 without broadening scope.

## Parent intent

Remove base64 storage/decoding and duplicate production source by default.

## Dependencies

- `M26-004-A` — `tasks/03_m26_qpack_v2/M26-004-A-store-raw-module-bytecode-section.md`
- `M26-004-B` — `tasks/03_m26_qpack_v2/M26-004-B-load-exactly-once.md`
- `M26-004-C` — `tasks/03_m26_qpack_v2/M26-004-C-make-source-optional-sidecar-development-section.md`
- `M26-004-D` — `tasks/03_m26_qpack_v2/M26-004-D-include-prelude-and-handler-manifest-in-the-compiled-module.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No base64 decode at startup.
- No source parse in bytecode production mode.
- Tamper/incompatibility rejects.
- Small-app source mode remains explicit if measured faster.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
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

- Bytecode integration tests.
- Tamper tests.
- Pack size/startup evidence.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-004-v: verify embed raw quickjs bytecode without base64
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
