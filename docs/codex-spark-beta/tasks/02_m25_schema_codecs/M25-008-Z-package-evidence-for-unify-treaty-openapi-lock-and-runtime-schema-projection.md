---
task_id: M25-008-Z
parent_task: M25-008
milestone: M25
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-008-Z — Package evidence for Unify Treaty, OpenAPI, lock, and runtime schema projection

## Atomic goal

Create source-backed evidence and handoff for parent task M25-008; update status only if verification passed.

## Parent intent

Eliminate projection drift across tooling and runtime.

## Dependencies

- `M25-008-V` — `tasks/02_m25_schema_codecs/M25-008-V-verify-unify-treaty-openapi-lock-and-runtime-schema-projection.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Same statuses/fields/security in all projections.
- No hand-written duplicate interface is required.
- Breaking changes are classified correctly.
- Published client does not import server implementation.

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

- Cross-projection golden tests.
- Contract diff fixtures.
- Typecheck scale results.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-008-z: package evidence for unify treaty openapi lock and runtime s
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M25-008-V merged in PR #760 at
  commit `f245f70d10cd85dc95a2babbc88c21b7beaab4ca`; issue #166 is
  closed. The evidence package is based on clean parent HEAD `f12179a`
  before this commit.
- Parent acceptance matrix: `M25-008-V` maps all four guardrails to
  source and named tests:
  1. Same statuses/fields/security in all projections: the 5-test
     projection-parity suite (134 assertions) in every verify run.
  2. No hand-written duplicate interface: the treaty suite imports the
     generated `Api` type; shape pins via `expectTypeOf` + d.ts
     snapshots.
  3. Breaking changes classified correctly: the IR v2 constraint
     classification suite (7 cases) + the structural suite.
  4. Published client imports no server implementation: TRT-004 bundle
     isolation; published surface is contract.d.ts + contract.meta.json.
- Source-backed implementation records:
  - `M25-008-A` (PR #756, #162 closed): all projections from the
    canonical IR; dead `contractFor` removed; generated-type import
    replaces the hand-written client interface.
  - `M25-008-B` (PR #757, #163 closed): parity checks inside
    verification (contract/pack/OpenAPI/d.ts cross-checks).
  - `M25-008-C` (PR #758, #164 closed): compact published
    `contract.meta.json` (1.5 KiB, hash-bound, no schema bodies) +
    sync/compactness parity test.
  - `M25-008-D` (PR #759, #165 closed): semantic diff covers IR v2
    constraint nodes with classified kinds (bounds/pattern/format/enum/
    literal/union/nullable/optional/fallback-reason).
- No performance measurement or claim; benchmark manifest preserved
  unchanged.
- Exact verification (fresh on this branch): `cargo test -p q-pack`
  (41 + 2); `cargo test -p q-schema-runtime` (57 + 3);
  `cargo test -p velqu-runtime` (24); `cargo test -p q-engine-quickjs`
  (1 + 96); `bun test` (81 passed, 0 failed, 481 expect calls);
  `bun run typecheck` clean; `cargo fmt --check` clean; `cargo clippy
  --workspace --all-targets -- -D warnings` clean; `scripts/validate-okf`
  (176 links, 0 errors).
- Full `./scripts/verify` completed all Rust, typecheck, proof-build, and
  TypeScript stages. Its final benchmark check reports only the known
  isolated-worktree hash mismatches for `qRuntimeRelease` and `proofPack`
  against `benchmarks/manifest.json`. The canonical root manifest and
  historical raw benchmarks were preserved.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M25-008 PASS;
  the beta checklist and task index mark this Z packet PASS. The
  generated Spark queues now expose M25-009-A (#168) as the next
  dependency-ready packet.
- Remaining scope: `M25-009`, `M25-010`, and `M25-GATE` remain TODO
  until implemented and evidenced.

Commit: `9a20559`.
