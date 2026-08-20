---
task_id: G0-002-A
parent_task: G0-002
milestone: G0
priority: P0
mode: VERIFY_OR_FIX
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-002-A — Require __velquFunctionManifest for the current numeric pack version and reject a missing manifest before ready

## Atomic goal

Require __velquFunctionManifest for the current numeric pack version and reject a missing manifest before ready.

## Parent intent

Make semantic function identity mandatory for current numeric execution and keep count-only loading strictly legacy.

## Dependencies

- `G0-001-Z` — `tasks/00_g0_gate_close/G0-001-Z-package-evidence-for-freeze-and-reconcile-the-4e69049-release-baseline.md`

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
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `scripts/package`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Require __velquFunctionManifest for the current numeric pack version and reject a missing manifest before ready.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Negative engine-load tests.
- Numeric dispatch counters.
- Startup failure diagnostics snapshot.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
g0-002-a: require velqufunctionmanifest for the current numeric pack v
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
