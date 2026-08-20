---
task_id: G0-006-V
parent_task: G0-006
milestone: G0
priority: P1
mode: VERIFY
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-006-V — Verify Separate and verify public contract identity

## Atomic goal

Prove every acceptance criterion for parent task G0-006 without broadening scope.

## Parent intent

Define and verify a public contract hash containing only observable API semantics.

## Dependencies

- `G0-006-A` — `tasks/00_g0_gate_close/G0-006-A-define-a-dedicated-public-canonical-model-covering-method-path-path-query-header.md`
- `G0-006-B` — `tasks/00_g0_gate_close/G0-006-B-exclude-function-names-ids-policy-implementation-handler-serializer-strategy-rou.md`
- `G0-006-C` — `tasks/00_g0_gate_close/G0-006-C-recompute-and-require-publiccontracthash-for-current-numeric-packs-inside-qpack.md`
- `G0-006-D` — `tasks/00_g0_gate_close/G0-006-D-add-stability-change-tests-for-internal-reorder-handler-rename-serializer-change.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
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

- Internal HandlerId reorder leaves public hash unchanged.
- Wire-visible schema/security/status changes change the public hash.
- Serializer implementation change leaves public hash unchanged.
- Treaty/OpenAPI/lock all identify the same public graph.

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
cargo test -p q-http
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

- Hash stability tests.
- Contract projection parity tests.
- Semantic diff fixtures.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-006-v: verify separate and verify public contract identity
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
