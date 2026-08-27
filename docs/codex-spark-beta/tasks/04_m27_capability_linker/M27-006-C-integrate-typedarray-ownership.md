---
task_id: M27-006-C
parent_task: M27-006
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-006-C — Integrate TypedArray ownership

## Atomic goal

Integrate TypedArray ownership.

## Parent intent

Provide bounded text encoding primitives used by modern packages.

## Dependencies

- `M27-006-B` — `tasks/04_m27_capability_linker/M27-006-B-define-invalid-sequence-replacement-behavior.md`

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
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Integrate TypedArray ownership.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- WPT/conformance.
- Memory tests.
- Benchmark.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-006-c: integrate typedarray ownership
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-006-C (PASS)

Deliverable: integrate TypedArray ownership semantics for `TextEncoder.encodeInto` and `TextDecoder.decode` over ArrayBuffers, sub-arrays, and DataViews without duplicate full-buffer copies.

### Changed files

- `crates/q-engine-quickjs/src/worker.rs`:
  - Added unit test `text_decoder_and_encoder_typedarray_views_and_subarrays` verifying ArrayBuffer view slicing and `encodeInto` on destination sub-arrays.
- `packages/cli/src/text-encoding.test.ts`:
  - Added test suite `TypedArray ownership and view slicing` covering:
    - Decoding from `ArrayBufferView` with non-zero `byteOffset` and `byteLength`.
    - Decoding directly from raw `ArrayBuffer`.
    - Decoding from `DataView`.
    - `encodeInto` into subarray views (`dest.subarray(2, 6)`).
    - `decode` with undefined defaulting to empty string.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-engine-quickjs` — 108 passed.
- `cargo test -p q-capabilities` — 78 passed.
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-schema-runtime` — 67 passed.
- `cargo test -p q-pack` — 98 passed.
- `bun test` — 179 passed (+5 TypedArray ownership/view slicing tests), 0 failed.

### Commands (fresh worktree on parent HEAD a61654f)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 108 · `-p q-schema-runtime` 67 · `-p q-capabilities` 78 · `-p velqu-runtime` 31 — pass.
- `bun test` 179 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - No duplicate full-buffer copies: `encodeInto` and `__velquTextEncodeFill` write directly to destination pointer buffers without intermediate `Vec<u8>` allocation.
  - Large buffers are bounded: bounded by `MAX_TEXT_BUFFER_LEN` (16 MB).

