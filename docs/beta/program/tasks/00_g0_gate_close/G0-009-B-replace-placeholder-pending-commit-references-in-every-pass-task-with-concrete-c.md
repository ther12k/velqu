---
task_id: G0-009-B
parent_task: G0-009
milestone: G0
priority: P1
mode: VERIFY_OR_FIX
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-009-B — Replace placeholder/PENDING commit references in every PASS task with concrete commit, source, tests, raw evidence, report, and artifact hashes

## Atomic goal

Replace placeholder/PENDING commit references in every PASS task with concrete commit, source, tests, raw evidence, report, and artifact hashes.

## Parent intent

Make release, review, evidence, and task status self-verifying and commit-accurate.

## Dependencies

- `G0-009-A` — `tasks/00_g0_gate_close/G0-009-A-generate-review-index-and-evidence-index-only-after-the-source-commit-is-fixed-a.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`
- `package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Replace placeholder/PENDING commit references in every PASS task with concrete commit, source, tests, raw evidence, report, and artifact hashes.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
g0-009-b: replace placeholder pending commit references in every pass
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
- Next dependency-ready task: the next packet in `indexes/EXECUTION_QUEUE.md`.
