---
task_id: G0-003-Z
parent_task: G0-003
milestone: G0
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-003-Z — Package evidence for Bind router and schema manifests into the execution graph hash

## Atomic goal

Create source-backed evidence and handoff for parent task G0-003; update status only if verification passed.

## Parent intent

Bind and semantically verify the entire execution graph, especially serialized routing and schema identity.

## Dependencies

- `G0-003-V` — `tasks/00_g0_gate_close/G0-003-V-verify-bind-router-and-schema-manifests-into-the-execution-graph-hash.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
```bash
./scripts/validate-production-plan
```

## Required evidence for this microtask

- Pack tamper tests.
- Canonicalization golden fixtures.
- Hash separation report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
g0-003-z: package evidence for bind router and schema manifests into t
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
  - `crates/q-router/src/lib.rs`
- Verification:
  - `cargo test -p q-pack`
  - `router_terminal_target_tamper_breaks_execution_hash`
  - `schema_manifest_tamper_breaks_execution_hash`
- Evidence artifacts:
  - `REVIEW_INDEX.json`
  - `EVIDENCE_INDEX.json`
  - `benchmarks/manifest.json`
  - `crates/q-engine-quickjs/tests/engine.rs`
- Remaining risk: none for this packet; G0 remains subject to the gate packet and final clean release binding.
- Next dependency-ready task: the first unchecked M24 packet after G0-GATE closes.
