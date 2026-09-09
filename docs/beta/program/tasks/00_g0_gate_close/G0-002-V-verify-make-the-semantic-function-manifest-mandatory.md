---
task_id: G0-002-V
parent_task: G0-002
milestone: G0
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-002-V — Verify Make the semantic function manifest mandatory

## Atomic goal

Prove every acceptance criterion for parent task G0-002 without broadening scope.

## Parent intent

Make semantic function identity mandatory for current numeric execution and keep count-only loading strictly legacy.

## Dependencies

- `G0-002-A` — `tasks/00_g0_gate_close/G0-002-A-require-velqufunctionmanifest-for-the-current-numeric-pack-version-and-reject-a.md`
- `G0-002-B` — `tasks/00_g0_gate_close/G0-002-B-remove-the-current-numeric-count-only-velqufunctions-fallback-from-workerinner-l.md`
- `G0-002-C` — `tasks/00_g0_gate_close/G0-002-C-validate-every-numeric-vector-entry-by-exact-index-key-kind-and-callability.md`
- `G0-002-D` — `tasks/00_g0_gate_close/G0-002-D-move-any-count-only-behavior-behind-an-explicit-legacy-pack-version-adapter-with.md`

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

- Missing semantic manifest rejects before socket bind.
- Swapped callable entries reject.
- Route/policy kind mismatch rejects.
- No numeric request can execute through the legacy map.

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

- Negative engine-load tests.
- Numeric dispatch counters.
- Startup failure diagnostics snapshot.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
g0-002-v: verify make the semantic function manifest mandatory
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Evidence checkpoint: `03cc48955c2f8b05c29cf6ca196572c67ed5dd2d`; the final release packet binds the exact clean HEAD after documentation updates.
- Source/evidence files:
  - `crates/q-engine-quickjs/src/worker.rs`
  - `crates/q-engine-quickjs/tests/engine.rs`
- Verification:
  - `cargo test -p q-engine-quickjs --test engine`
  - `numeric_pack_without_semantic_manifest_is_rejected`
  - `swapped_function_vector_entries_are_rejected`
- Evidence artifacts:
  - `REVIEW_INDEX.json`
  - `EVIDENCE_INDEX.json`
  - `benchmarks/manifest.json`
  - `crates/q-engine-quickjs/tests/engine.rs`
- Remaining risk: none for this packet; G0 remains subject to the gate packet and final clean release binding.
- Next dependency-ready task: the next packet in `indexes/EXECUTION_QUEUE.md`.
