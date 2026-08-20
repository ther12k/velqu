---
task_id: G0-005-V
parent_task: G0-005
milestone: G0
priority: P0
mode: VERIFY
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-005-V — Verify Complete operational RouteId, PolicyId, and SchemaId usage

## Atomic goal

Prove every acceptance criterion for parent task G0-005 without broadening scope.

## Parent intent

Make RouteId, PolicyId, HandlerId, and SchemaId complete operational identities on the normal current-pack path.

## Dependencies

- `G0-005-A` — `tasks/00_g0_gate_close/G0-005-A-make-router-match-results-carry-routeid-and-use-routeid-to-index-a-dense-verifie.md`
- `G0-005-B` — `tasks/00_g0_gate_close/G0-005-B-introduce-a-dense-policyplan-manifest-so-policyid-resolves-to-the-exact-pre-veri.md`
- `G0-005-C` — `tasks/00_g0_gate_close/G0-005-C-require-a-dense-complete-schemaid-manifest-and-use-schemaid-for-all-request-vali.md`
- `G0-005-D` — `tasks/00_g0_gate_close/G0-005-D-move-route-policy-handler-and-schema-names-into-debug-inspection-tables-rather-t.md`
- `G0-005-E` — `tasks/00_g0_gate_close/G0-005-E-add-counters-or-assertions-proving-the-current-numeric-path-performs-zero-string.md`

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

- No route, policy, handler, or schema string lookup on the normal numeric path.
- Numeric manifests are dense and complete.
- Invalid IDs reject before ready.
- Diagnostics still report readable names.

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

- Hot-path counter proving zero legacy lookups.
- Malformed numeric-graph tests.
- `velqu inspect` snapshot.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-005-v: verify complete operational routeid policyid and schemaid us
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
