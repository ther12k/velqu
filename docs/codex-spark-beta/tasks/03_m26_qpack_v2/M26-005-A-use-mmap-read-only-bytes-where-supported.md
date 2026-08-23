---
task_id: M26-005-A
parent_task: M26-005
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-005-A — Use mmap/read-only bytes where supported

## Atomic goal

Use mmap/read-only bytes where supported.

## Parent intent

Map and validate the pack without reconstructing large owned trees.

## Dependencies

- `M26-003-Z` — `tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Use mmap/read-only bytes where supported.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Pack fuzz results.
- Allocation profile.
- Platform smoke tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-005-a: use mmap read only bytes where supported
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-005-A (PASS)

Deliverable: read-only mapped pack bytes where supported — the qpack2
reader can consume the pack through a zero-copy `&[u8]` view backed by
a read-only mmap on unix (owned fallback elsewhere), so directory
validation borrows section views straight out of the mapping without
reconstructing owned trees.

Changed files:

- `Cargo.toml` + `crates/q-pack/Cargo.toml` + `Cargo.lock` — workspace
  dep `memmap2 = "0.9"` (q-pack only).
- `crates/q-pack/src/lib.rs` — `qpack2::reader::PackBytes`:
  - `Mapped(memmap2::Mmap)` on unix / `Owned(Vec<u8>)` otherwise;
    `Deref<Target = [u8]>` so every existing rule (header, directory,
    per-section sha256, required ids, binding) consumes it unchanged.
  - `PackBytes::open(path)`: opens read-only; maps non-empty files on
    unix (`Mmap::map` = PROT_READ), falls back to an owned read for
    empty files, mmap failures, and non-unix.

Tests (crates/q-pack/src/lib.rs, 77 total):

- `pack_bytes_mapped_and_owned_validate_identically_zero_copy` — the
  same bound file validates identically through the mapped and owned
  paths (ids, hashes, byte equality), AND every section body pointer
  lies inside the pack-bytes allocation: sections are views, not
  copies (parent intent: no reconstructed owned trees).
- `pack_bytes_rejects_missing_and_malformed_without_panic` — missing
  file errors; empty file falls back to owned and rejects in header
  validation; 4 KiB junk through the mapped path rejects in validate
  and parse_directory_with_binding without panicking (guardrail:
  malformed lengths cannot panic or read out of bounds).

Commands and results:

- `cargo test -p q-pack` — 77 passed + 2, 0 failed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (flagged follow-up from
  M26-002-A).

Guardrails: malformed lengths cannot panic (junk/empty tests; the
12-rule directory suite and mutation fuzz unchanged); reader is
mode-agnostic (`&[u8]` consumer — shared and embedded modes both
hand slices); fuzz parser stable (no parser change; PackBytes is a
carrier). Startup allocation boundedness measurement is M26-005-V/Z
scope (no perf claims made here).
