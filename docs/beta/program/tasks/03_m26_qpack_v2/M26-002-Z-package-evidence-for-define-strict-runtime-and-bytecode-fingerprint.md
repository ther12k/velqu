---
task_id: M26-002-Z
parent_task: M26-002
milestone: M26
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-002-Z — Package evidence for Define strict runtime and bytecode fingerprint

## Atomic goal

Create source-backed evidence and handoff for parent task M26-002; update status only if verification passed.

## Parent intent

Prevent loading bytecode or plans under an incompatible engine/runtime build.

## Dependencies

- `M26-002-V` — `tasks/03_m26_qpack_v2/M26-002-V-verify-define-strict-runtime-and-bytecode-fingerprint.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Any fingerprint mismatch rejects before ready.
- Error identifies incompatible dimension.
- Engine upgrades require pack rebuild.
- Cross-target packs are rejected.

## Targeted commands

```bash
cargo test -p q-pack
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

- Fingerprint tests.
- Cross-build fixtures.
- Upgrade lane documentation.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-002-z: package evidence for define strict runtime and bytecode fing
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M26-002-V merged in PR #785 at
  commit `0c5509c2bff5e1e607c1863eda0908845fac56ec`; issue #190 is
  closed. The evidence package is based on clean parent HEAD `e151534`
  before this commit.
- Parent acceptance matrix: `M26-002-V` maps all four guardrails to
  source and named tests (mismatches reject before ready; dimension-
  named errors; engine upgrades require rebuild via the build hash;
  cross-target packs rejected with the explicit `--no-bytecode` source
  path and the no-silent-fallback proof).
- Source-backed implementation records:
  - `M26-002-A` (PR #781, #186 closed): the full runtime fingerprint
    (rquickjs version, runtime build hash, capability hash; bytecode
    target fields pre-existing) with dimension-named rejections and
    compiler parity (Bun-mirrored hashes, proven by loading).
  - `M26-002-B` (PR #782, #187 closed): fail closed on cross-target
    mismatch (arch/os/pointer-width/endianness; missing target rejects).
  - `M26-002-C` (PR #783, #188 closed): the explicit `--no-bytecode`
    source-rebuild path end to end.
  - `M26-002-D` (PR #784, #189 closed): never silently fall back —
    pinned at engine and process boundaries (hash-valid garbage
    bytecode rejects before ready).
- Note: the fingerprint fields change pack bytes; the canonical
  benchmark manifest's `proofPack` entry requires a matched refresh
  (flagged in M26-002-A; not silently altered here).
- Exact verification (fresh on this branch): all targeted suites green
  (q-pack 53+2, q-engine-quickjs 1+97, q-http 4+6+1, q-schema-runtime
  58+4+5, velqu-runtime 26; bun 81→82 with 487 expects; typecheck, fmt,
  clippy -D warnings, validate-okf clean). `./scripts/verify` completes
  every stage except the documented isolated-worktree benchmark-manifest
  mismatch.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M26-002 PASS.
  The generated Spark queues expose M26-003-A next.
- Remaining scope: `M26-003`+ remain TODO until implemented and
  evidenced.

Commit: `8bc32e9`.
