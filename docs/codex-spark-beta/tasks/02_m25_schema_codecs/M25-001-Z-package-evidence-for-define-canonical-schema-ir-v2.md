---
task_id: M25-001-Z
parent_task: M25-001
milestone: M25
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-001-Z — Package evidence for Define canonical Schema IR v2

## Atomic goal

Create source-backed evidence and handoff for parent task M25-001; update status only if verification passed.

## Parent intent

Create a versioned normalized schema model suitable for validation, decoding, encoding, OpenAPI, Treaty, and semantic diff.

## Dependencies

- `M25-001-V` — `tasks/02_m25_schema_codecs/M25-001-V-verify-define-canonical-schema-ir-v2.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- One schema identity produces equivalent runtime and public projections.
- Canonical form is deterministic.
- Unsupported constructs fail or use explicit fallback.
- Schema diff can classify nested changes.

## Targeted commands

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

- Schema golden corpus.
- Canonicalization tests.
- Compatibility matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-001-z: package evidence for define canonical schema ir v2
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M25-001-V merged in PR #718 at commit
  `8efb6e27a45f171c20c956b299b7ebe8e7233bf1`; issue #124 is closed. The
  evidence package is based on clean parent HEAD `e7df2dd` before this commit.
- Parent acceptance matrix: `M25-001-V` maps all four guardrails to source and
  named tests: equivalent runtime/public projections, deterministic canonical
  form, explicit unsupported/fallback behavior, and nested semantic diff.
- Source-backed implementation records: M25-001-A, B, C, and D are PASS and
  identify the Rust runtime, TypeScript schema contract, compiler extraction and
  emission, q-pack version/manifest checks, golden corpus, canonical corpus,
  compatibility matrix, ADR-0022, ADR-0023, and
  `docs/specs/unsupported-transformations.md`.
- Golden and compatibility evidence: `conformance/schema/golden/`,
  `conformance/schema/golden/canonical/`, and
  `conformance/schema/golden/COMPATIBILITY.md`.
- Exact verification: `cargo test -p q-engine-quickjs` (1 unit + 96
  integration passed); `cargo test -p q-schema-runtime` (28 library + 2 fuzz
  passed); `bun test` (66 passed, 0 failed, 233 expect calls); `bun run
  typecheck` clean; `cargo fmt --check` clean; `cargo clippy --workspace
  --all-targets -- -D warnings` clean; `scripts/validate-okf` (176 links, 0
  errors).
- Full `./scripts/verify` completed all Rust, typecheck, proof-build, and
  TypeScript stages. Its final benchmark-evidence check reported only the
  known isolated-worktree `qRuntimeRelease` and `proofPack` hash mismatches
  against `benchmarks/manifest.json`; the manifest was not changed, raw
  benchmark evidence was not rewritten, and no performance claim is made.
- Index and checksum boundary: root `REVIEW_INDEX.json` and
  `EVIDENCE_INDEX.json` remain M24 release-bound templates with
  `BOUND_BY_RELEASE_PACKET_TO_CLEAN_HEAD`; `scripts/release-packet` owns their
  clean-HEAD binding. No release index or benchmark manifest was modified in
  this packet.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M25-001 PASS; the
  beta checklist and task index mark this Z packet PASS. The generated Spark
  queues now expose M25-002-A (#126) as the next dependency-ready packet.
- Remaining scope: `M25-GATE` remains TODO and no later M25 packet is claimed
  complete.
- Working tree was clean before this evidence-only change.
