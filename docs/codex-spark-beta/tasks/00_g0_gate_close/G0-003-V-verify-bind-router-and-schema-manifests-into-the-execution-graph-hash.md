---
task_id: G0-003-V
parent_task: G0-003
milestone: G0
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-003-V — Verify Bind router and schema manifests into the execution graph hash

## Atomic goal

Prove every acceptance criterion for parent task G0-003 without broadening scope.

## Parent intent

Bind and semantically verify the entire execution graph, especially serialized routing and schema identity.

## Dependencies

- `G0-003-A` — `tasks/00_g0_gate_close/G0-003-A-define-the-execution-graph-projection-to-include-function-manifest-routeplans-po.md`
- `G0-003-B` — `tasks/00_g0_gate_close/G0-003-B-recompute-and-verify-the-execution-graph-hash-inside-qpack-verify-before-ready.md`
- `G0-003-C` — `tasks/00_g0_gate_close/G0-003-C-implement-serializedrouter-semantic-verification-non-empty-root-unique-static-ed.md`
- `G0-003-D` — `tasks/00_g0_gate_close/G0-003-D-add-tamper-fixtures-that-redirect-a-terminal-to-a-different-valid-routeid-mutate.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Any execution-critical router or schema mutation changes the hash or fails semantic verification.
- A structurally in-range but semantically wrong router is rejected before socket bind.
- Execution hash changes on internal plan changes while public contract identity remains independent.
- Empty, unreachable, duplicate-edge, or mismatched-method routers are rejected.

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
cargo test -p q-schema-runtime
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

- Pack tamper tests.
- Canonicalization golden fixtures.
- Hash separation report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-003-v: verify bind router and schema manifests into the execution g
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
