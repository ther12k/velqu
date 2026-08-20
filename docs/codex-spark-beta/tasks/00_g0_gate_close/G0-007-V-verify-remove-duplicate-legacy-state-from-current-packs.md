---
task_id: G0-007-V
parent_task: G0-007
milestone: G0
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-007-V — Verify Remove duplicate legacy state from current packs

## Atomic goal

Prove every acceptance criterion for parent task G0-007 without broadening scope.

## Parent intent

Make current numeric mode explicit and structurally independent of legacy handler-table execution.

## Dependencies

- `G0-007-A` — `tasks/00_g0_gate_close/G0-007-A-introduce-an-explicit-current-pack-format-execution-mode-for-numeric-artifacts.md`
- `G0-007-B` — `tasks/00_g0_gate_close/G0-007-B-remove-handlertable-from-the-current-numeric-pack-schema-and-compiler-output.md`
- `G0-007-C` — `tasks/00_g0_gate_close/G0-007-C-require-function-policy-schema-routeplan-and-serialized-router-manifests-in-nume.md`
- `G0-007-D` — `tasks/00_g0_gate_close/G0-007-D-isolate-v1-legacy-loading-in-a-versioned-compatibility-adapter-and-reject-mixed.md`
- `G0-007-E` — `tasks/00_g0_gate_close/G0-007-E-verify-current-numeric-startup-allocates-no-legacy-handler-cache-or-registration.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`

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

- Current pack has zero handlerTable entries.
- Worker allocates no legacy handler cache.
- Legacy pack compatibility is explicit, not inferred.
- Compiler and runtime reject mixed-mode artifacts.

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

- Pack-format fixtures.
- Memory/startup comparison.
- Legacy migration test.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-007-v: verify remove duplicate legacy state from current packs
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Evidence checkpoint: `03cc48955c2f8b05c29cf6ca196572c67ed5dd2d`; the final release packet binds the exact clean HEAD after documentation updates.
- Source/evidence files:
  - `crates/q-pack/src/lib.rs`
  - `packages/compiler/src/emit.ts`
  - `crates/q-engine-quickjs/src/worker.rs`
- Verification:
  - `cargo test -p q-pack`
  - `numeric_pack_with_handler_table_is_rejected`
  - `numeric_pack_without_compiled_router_is_rejected`
- Evidence artifacts:
  - `REVIEW_INDEX.json`
  - `EVIDENCE_INDEX.json`
  - `benchmarks/manifest.json`
  - `crates/q-engine-quickjs/tests/engine.rs`
- Remaining risk: none for this packet; G0 remains subject to the gate packet and final clean release binding.
- Next dependency-ready task: the next packet in `indexes/EXECUTION_QUEUE.md`.
