---
task_id: M26-003-V
parent_task: M26-003
milestone: M26
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/M26.md
commit_required: true
---

# M26-003-V — Verify Encode compiled router, RoutePlans, schemas, policies, and functions as sections

## Atomic goal

Prove every acceptance criterion for parent task M26-003 without broadening scope.

## Parent intent

Serialize the already verified runtime graph without changing semantics.

## Dependencies

- `M26-003-A` — `tasks/03_m26_qpack_v2/M26-003-A-define-dense-section-schemas.md`
- `M26-003-B` — `tasks/03_m26_qpack_v2/M26-003-B-store-router-nodes-edges-terminals-routeplans-schema-programs-policy-plans-funct.md`
- `M26-003-C` — `tasks/03_m26_qpack_v2/M26-003-C-use-offsets-and-bounds-checks.md`
- `M26-003-D` — `tasks/03_m26_qpack_v2/M26-003-D-bind-sections-to-execution-integrity.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-router/src/lib.rs`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
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

- No semantic reconstruction at startup.
- Bounds and index validation reject malformed packs.
- Binary and transitional representations are property-equivalent.
- Debug names are optional and non-hot.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
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

- Round-trip/property tests.
- Mutation fuzzing.
- Section-size report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-003-v: verify encode compiled router routeplans schemas policies an
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
