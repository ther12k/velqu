---
task_id: G0-009-Z
parent_task: G0-009
milestone: G0
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-009-Z — Package evidence for Create self-verifying milestone and evidence indexes

## Atomic goal

Create source-backed evidence and handoff for parent task G0-009; update status only if verification passed.

## Parent intent

Make release, review, evidence, and task status self-verifying and commit-accurate.

## Dependencies

- `G0-009-V` — `tasks/00_g0_gate_close/G0-009-V-verify-create-self-verifying-milestone-and-evidence-indexes.md`

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
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- `sha256sum -c` equivalent passes for the release packet.
- Every PASS task references existing evidence.
- No stale previous bundle is presented as current.
- Git bundle, source ZIP, and source commit agree.

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

- Review index.
- Evidence index.
- Release packet validation report.
- [ ] Current numeric startup requires semantic function identity and accepts no count-only fallback.
- [ ] Serialized router and schema/function plans are integrity-bound and loaded without runtime semantic reconstruction.
- [ ] RouteId, PolicyId, HandlerId, and SchemaId are operational; names are diagnostic only.
- [ ] Public contract and execution graph hashes are separate and independently verified.
- [ ] Canonical warm/cold evidence meets the frozen protocol and reports match raw data.
- [ ] Release packet is self-verifying and task/evidence state is truthful.
- Warm C0–C3: c=1/10/50, five repetitions.
- Cold: 25/1,000/10,000 routes, fresh processes.
- Allocation and startup-stage profile.
- No regression threshold is silently relaxed.
- No M2.4 request-slab integration.
- No new capability APIs.
- No database implementation.
- No multi-worker changes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
g0-009-z: package evidence for create self verifying milestone and evi
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Evidence checkpoint: `03cc48955c2f8b05c29cf6ca196572c67ed5dd2d`; the final release packet binds the exact clean HEAD after documentation updates.
- Source/evidence files:
  - `scripts/validate-production-plan`
  - `scripts/validate-benchmark-evidence.py`
  - `scripts/release-packet`
  - `REVIEW_INDEX.json`
  - `EVIDENCE_INDEX.json`
- Verification:
  - `./scripts/validate-production-plan`
  - `./scripts/release-packet`
  - `(cd release && sha256sum -c SHA256SUMS.txt)`
- Evidence artifacts:
  - `REVIEW_INDEX.json`
  - `EVIDENCE_INDEX.json`
  - `benchmarks/manifest.json`
  - `benchmarks/raw/profiles/startup-10000.json`
  - `benchmarks/raw/profiles/startup-10000.alloc.json`
  - `release/SOURCE-COMMIT.txt`
  - `release/SHA256SUMS.txt`
- Remaining risk: none for this packet; G0 remains subject to the gate packet and final clean release binding.
- Next dependency-ready task: the first unchecked M24 packet after G0-GATE closes.
