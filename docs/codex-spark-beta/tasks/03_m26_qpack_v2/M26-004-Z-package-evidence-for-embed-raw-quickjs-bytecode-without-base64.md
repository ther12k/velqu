---
task_id: M26-004-Z
parent_task: M26-004
milestone: M26
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-004-Z — Package evidence for Embed raw QuickJS bytecode without base64

## Atomic goal

Create source-backed evidence and handoff for parent task M26-004; update status only if verification passed.

## Parent intent

Remove base64 storage/decoding and duplicate production source by default.

## Dependencies

- `M26-004-V` — `tasks/03_m26_qpack_v2/M26-004-V-verify-embed-raw-quickjs-bytecode-without-base64.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
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

- No base64 decode at startup.
- No source parse in bytecode production mode.
- Tamper/incompatibility rejects.
- Small-app source mode remains explicit if measured faster.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
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

- Bytecode integration tests.
- Tamper tests.
- Pack size/startup evidence.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-004-z: package evidence for embed raw quickjs bytecode without base
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M26-004-V merged in PR #797 at
  commit `e898645debfac74a63d8ebe7333e54ee14c9a299`; issue #202 is
  closed. The evidence package is based on clean parent HEAD `edfbe6a`
  before this commit.
- Parent acceptance matrix: `M26-004-V` maps all four guardrails to
  source and named tests (no base64 decode at startup — v2 raw section
  with non-base64-alphabet round-trip proof + single-decode cache on
  the v1 transition path; no source parse in bytecode production mode —
  embedded prelude/manifest module with zero prelude source eval;
  tamper/incompatibility rejects — sha256 mismatch, fingerprint
  dimensions, marker vocabulary, execution-integrity binding catching
  repaired-hash tamper; explicit source mode — `--no-bytecode`
  sanctioned recovery with the host prelude, no silent fallback).
- Source-backed implementation records:
  - `M26-004-A` (PR #793, #198 closed): qpack2 section 0x0007 raw
    bytecode (30-byte metadata header, bytes verbatim) with
    bounds/drift rejections and the honest base64-vs-raw size report
    (768 raw → 1026 base64-in-JSON vs 798 section).
  - `M26-004-B` (PR #794, #199 closed): single base64 decode per
    startup (`verify_and_cache_bytecode`; handoff via
    `decoded_bytecode.take()`; failed verify leaves no cache).
  - `M26-004-C` (PR #795, #200 closed): ADR-0027 producer flip —
    production packs debug-free; `app.qpack.sources.json` sidecar
    bound by packSha256; runtime never reads sidecars (serving test).
  - `M26-004-D` (PR #796, #201 closed): compiled module contains
    prelude + handler manifest (`bundle_prelude: "embedded"` closed
    vocabulary; bytecode tool compiles PRELUDE+bundle; worker skips
    prelude source eval; embedded-without-bytecode fails closed).
  - `M26-004-V` (PR #797, #202 closed): verification closure with the
    criterion→evidence map; no production change required.
- Exact verification (fresh on this branch): q-pack 75+2, q-router 15,
  q-engine-quickjs 1+97, velqu-runtime 28 passed; bun 83 pass / 0 fail
  / 487 expect(); typecheck, fmt --check, clippy `-D warnings` clean.
  `./scripts/verify` completes every stage except the documented
  isolated-worktree benchmark-manifest mismatch (qRuntimeRelease +
  proofPack; flagged matched-evidence follow-up from M26-002-A).
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M26-004 PASS;
  TASK_INDEX marks M26-004-Z PASS. The generated Spark queues expose
  M26-005-A next.
- Remaining scope: `M26-005`+ remain TODO until implemented and
  evidenced.
