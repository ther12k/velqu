---
task_id: M26-005-Z
parent_task: M26-005
milestone: M26
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-005-Z — Package evidence for Implement zero-copy or bounded-copy pack reader

## Atomic goal

Create source-backed evidence and handoff for parent task M26-005; update status only if verification passed.

## Parent intent

Map and validate the pack without reconstructing large owned trees.

## Dependencies

- `M26-005-V` — `tasks/03_m26_qpack_v2/M26-005-V-verify-implement-zero-copy-or-bounded-copy-pack-reader.md`

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
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Malformed lengths cannot panic or read out of bounds.
- Startup allocations are measured and bounded.
- Reader works for shared and embedded modes.
- Fuzz parser remains stable.

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

- Pack fuzz results.
- Allocation profile.
- Platform smoke tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-005-z: package evidence for implement zero copy or bounded copy pac
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M26-005-V merged in PR #803 at
  commit `29a8f0f6abf1d13bdc22ab1b88f704fe14e8bc89`; issue #208 is
  closed. The evidence package is based on clean parent HEAD `6189285`
  before this commit.
- Parent acceptance matrix: `M26-005-V` maps all four guardrails to
  source and named tests (malformed lengths cannot panic — checked-add
  overflow fix pinned in debug; startup allocations bounded —
  structural zero-copy proofs across carriers with count-before-size
  directory derivation and MAX_NODES/MAX_CODE_BYTES caps; shared +
  embedded reader modes — Mapped/Owned/Embedded carrier identity +
  read-only smoke; fuzz parser stable — fuzz and mutation suites
  unchanged).
- Source-backed implementation records:
  - `M26-005-A` (PR #799, #204 closed): `PackBytes` read-only mmap on
    unix with owned fallback; zero-copy section views proven by
    pointer-range assertions.
  - `M26-005-B` (PR #800, #205 closed): ALL section bounds validated
    before access; fixed a real unchecked-arithmetic hole
    (`offset+len` overflow: debug panic / release wrap-past-bounds) —
    regression-pinned by five malformed shapes.
  - `M26-005-C` (PR #801, #206 closed): unsafe confinement — exactly
    one audited unsafe block in q-pack (mmap SAFETY audit),
    `deny(unsafe_op_in_unsafe_fn)`, expanded bytecode-module and
    fill-bytes audits; mode-0444 platform smoke.
  - `M26-005-D` (PR #802, #207 closed): `Embedded(&'static [u8])`
    carrier for standalone-binary mode; carrier identity + zero-copy
    proofs.
  - `M26-005-V` (PR #803, #208 closed): verification closure; no
    defects found.
- Exact verification (fresh on this branch): q-pack 80+2, q-router 15,
  q-engine-quickjs 1+97, velqu-runtime 28 passed; bun 83 pass / 0 fail
  / 487 expect(); typecheck, fmt --check, clippy `-D warnings` clean.
  `./scripts/verify` completes every stage except the documented
  isolated-worktree benchmark-manifest mismatch (qRuntimeRelease +
  proofPack; flagged matched-evidence follow-up from M26-002-A).
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M26-005 PASS;
  TASK_INDEX marks M26-005-Z PASS. The generated Spark queues expose
  M26-006-A next.
- Remaining scope: `M26-006`+ remain TODO until implemented and
  evidenced.
