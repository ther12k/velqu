---
task_id: G0-004-Z
parent_task: G0-004
milestone: G0
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-004-Z — Package evidence for Load the serialized router directly

## Atomic goal

Create source-backed evidence and handoff for parent task G0-004; update status only if verification passed.

## Parent intent

Trust and load the compiler-emitted serialized automaton directly without runtime semantic reconstruction.

## Dependencies

- `G0-004-V` — `tasks/00_g0_gate_close/G0-004-V-verify-load-the-serialized-router-directly.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Current numeric startup performs zero route parsing/collision reconstruction.
- 404/405 and Allow semantics match the reference matcher.
- Route-specific parameter names are preserved.
- Every compiled route is reachable exactly as intended.

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
```bash
./scripts/validate-production-plan
```
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Startup instrumentation.
- Router property-test corpus.
- 10,000-route load test.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
g0-004-z: package evidence for load the serialized router directly
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Evidence checkpoint: `03cc48955c2f8b05c29cf6ca196572c67ed5dd2d`; the final release packet binds the exact clean HEAD after documentation updates.
- Source/evidence files:
  - `crates/q-router/src/lib.rs`
  - `crates/q-runtime/src/main.rs`
  - `crates/q-router/src/lib.rs`
- Verification:
  - `cargo test -p q-router`
  - `compiled_and_reference_routers_are_property_equivalent`
- Evidence artifacts:
  - `REVIEW_INDEX.json`
  - `EVIDENCE_INDEX.json`
  - `benchmarks/manifest.json`
  - `crates/q-engine-quickjs/tests/engine.rs`
- Remaining risk: none for this packet; G0 remains subject to the gate packet and final clean release binding.
- Next dependency-ready task: the first unchecked M24 packet after G0-GATE closes.
