---
task_id: M26-003-Z
parent_task: M26-003
milestone: M26
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-003-Z — Package evidence for Encode compiled router, RoutePlans, schemas, policies, and functions as sections

## Atomic goal

Create source-backed evidence and handoff for parent task M26-003; update status only if verification passed.

## Parent intent

Serialize the already verified runtime graph without changing semantics.

## Dependencies

- `M26-003-V` — `tasks/03_m26_qpack_v2/M26-003-V-verify-encode-compiled-router-routeplans-schemas-policies-and-functions-as-secti.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-router/src/lib.rs`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No semantic reconstruction at startup.
- Bounds and index validation reject malformed packs.
- Binary and transitional representations are property-equivalent.
- Debug names are optional and non-hot.

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
cargo test -p velqu-runtime
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

- Round-trip/property tests.
- Mutation fuzzing.
- Section-size report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-003-z: package evidence for encode compiled router routeplans schem
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M26-003-V merged in PR #791 at
  commit `39f35526882108f0f41836b7695723805aec9800`; issue #196 is closed.
  The evidence package is based on clean parent HEAD `0c1d282` before this
  commit.
- Parent acceptance matrix: `M26-003-V` maps all four guardrails to source
  and named tests (no semantic reconstruction — encoders consume the
  already-verified graph and decoders return it as data; bounds/index
  validation — 12-rule directory suite, 4 000-round header mutation,
  2 000-round dense-section mutation, graph-section mutation, integration
  fuzz; binary↔transitional property equivalence —
  `binary_and_transitional_representations_agree` plus dense/graph/header/
  bound round-trips; debug names optional and non-hot — catalog §6 pins
  required ids, BUNDLE_BYTECODE optional, FLAG_OPTIONAL defined and unknown
  flag bits reject).
- Source-backed implementation records:
  - `M26-003-A` (PR #787, #192 closed): dense section schemas — strings
    interner + functions/policies/capabilities/contract-summary tables,
    layout constants pinned to spec, honest size report.
  - `M26-003-B` (PR #788, #193 closed): graph sections — router
    nodes/edges/terminals, RoutePlans (48-byte prefix + tails), schemas
    (dense envelope, canonical IR payload), policy rows + manifest.
  - `M26-003-C` (PR #789, #194 closed): offsets and bounds checks —
    reader header/directory/validate with duplicate/overlap/range rules and
    per-section sha256.
  - `M26-003-D` (PR #790, #195 closed): execution-integrity binding —
    96-byte extended header, order-independent aggregate hash over the
    canonical directory; content-repair attacks still rejected.
  - `M26-003-V` (PR #791, #196 closed): verification closure with the
    criterion→evidence map and the added property-equivalence test.
- Exact verification (fresh on this branch): q-pack 67+2 passed, q-router
  15 passed, q-engine-quickjs 1+97 passed, velqu-runtime 26 passed; bun 82
  pass / 0 fail / 487 expect(); typecheck, fmt --check, clippy
  `-D warnings` clean. `./scripts/verify` completes every stage except the
  documented isolated-worktree benchmark-manifest mismatch (qRuntimeRelease
  + proofPack; flagged matched-evidence follow-up from M26-002-A).
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M26-003 PASS;
  STATUS.md M26-003-B checkbox drift (missed in PR #788) corrected here;
  TASK_INDEX marks M26-003-Z PASS. The generated Spark queues expose
  M26-004-A next.
- Remaining scope: `M26-004`+ remain TODO until implemented and evidenced.
