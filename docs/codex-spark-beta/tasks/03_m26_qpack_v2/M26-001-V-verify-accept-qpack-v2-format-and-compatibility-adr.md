---
task_id: M26-001-V
parent_task: M26-001
milestone: M26
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M26.md
commit_required: true
---

# M26-001-V — Verify Accept QPack v2 format and compatibility ADR

## Atomic goal

Prove every acceptance criterion for parent task M26-001 without broadening scope.

## Parent intent

Freeze the binary format goals, trust model, compatibility, and migration rules.

## Dependencies

- `M26-001-A` — `tasks/03_m26_qpack_v2/M26-001-A-define-numeric-current-mode-and-legacy-v1-adapter.md`
- `M26-001-B` — `tasks/03_m26_qpack_v2/M26-001-B-specify-section-directory-alignment-bounds-optional-sections-and-versioning.md`
- `M26-001-C` — `tasks/03_m26_qpack_v2/M26-001-C-separate-integrity-from-authenticity.md`
- `M26-001-D` — `tasks/03_m26_qpack_v2/M26-001-D-define-debug-source-sidecar-policy.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

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
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Unknown versions fail closed.
- Current mode has no legacy handler table.
- Compatibility policy is explicit.
- Untrusted arbitrary bytecode is forbidden.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
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

- ADR.
- Binary layout diagrams.
- Compatibility matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-001-v: verify accept qpack v2 format and compatibility adr
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
