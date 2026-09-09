---
task_id: M27-006-Z
parent_task: M27-006
milestone: M27
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-006-Z — Package evidence for Implement TextEncoder and TextDecoder

## Atomic goal

Create source-backed evidence and handoff for parent task M27-006; update status only if verification passed.

## Parent intent

Provide bounded text encoding primitives used by modern packages.

## Dependencies

- `M27-006-V` — `tasks/04_m27_capability_linker/M27-006-V-verify-implement-textencoder-and-textdecoder.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `crates/q-pack/src/lib.rs`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Encoding semantics match selected standard cases.
- Large buffers are bounded.
- No duplicate full-buffer copies without evidence.
- Capability can be tree-linked.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
```
```bash
cargo test -p q-capabilities
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

- WPT/conformance.
- Memory tests.
- Benchmark.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-006-z: package evidence for implement textencoder and textdecoder
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M27-006-V merged in PR #876
  at commit `b66fa4c2c825b94890a667642616f40317d90156`; issue #274
  is closed. Based on clean parent HEAD `066f6e1` (queue-regen).
- Parent acceptance matrix: `M27-006-V` maps all four guardrails
  (full UTF-8 standard encoding/decoding semantics, buffer length bounding
  at 16 MB, zero-copy pointer slice access for `encodeInto`/`decode`, and
  `native.text` capability tree linking).
- Source-backed implementation records:
  - `M27-006-A` (PR #872, #270 closed): UTF-8 baseline for `TextEncoder`
    and `TextDecoder` under `q-capabilities` and QuickJS globals.
  - `M27-006-B` (PR #873, #271 closed): invalid sequence U+FFFD replacement
    and fatal decode error handling.
  - `M27-006-C` (PR #874, #272 closed): `TypedArray` ownership, `ArrayBufferView`
    slicing with non-zero offset, and `encodeInto` subarray support.
  - `M27-006-D` (PR #875, #273 closed): WPT multi-byte & astral plane vectors +
    `docs/reports/m27-006-wpt-text-encoding-report.md`.
  - `M27-006-V` (PR #876, #274 closed): verification closure + matched manifest refresh.
- Canonical evidence artifacts:
  - Tests: `q-capabilities` 79 passed (+9 text encoding tests), `q-engine-quickjs`
    108 passed (+2 JS TextEncoder/TextDecoder integration tests), `bun test` 181 passed (+16 text encoding tests).
  - Report: `docs/reports/m27-006-wpt-text-encoding-report.md`.
  - Manifest: `benchmarks/manifest.json` matched refresh under verify remap environment.
- Exact verification (fresh on this branch): `cargo test` across all crates passes;
  `bun test` 181/0; typecheck, fmt --check, clippy `-D warnings` clean;
  `./scripts/verify` — ALL PASS (exit 0).
- Status bookkeeping: ledger marks M27-006 PASS; TASK_INDEX marks M27-006-Z PASS.
  Queues expose M27-007-A next.
- Remaining scope: M27-007+ (AbortController & AbortSignal), M27-GATE.
