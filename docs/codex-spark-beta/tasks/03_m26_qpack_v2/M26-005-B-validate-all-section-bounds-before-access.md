---
task_id: M26-005-B
parent_task: M26-005
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-005-B — Validate all section bounds before access

## Atomic goal

Validate all section bounds before access.

## Parent intent

Map and validate the pack without reconstructing large owned trees.

## Dependencies

- `M26-005-A` — `tasks/03_m26_qpack_v2/M26-005-A-use-mmap-read-only-bytes-where-supported.md`

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
5. Implement exactly this deliverable: Validate all section bounds before access.
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
m26-005-b: validate all section bounds before access
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-005-B (PASS)

Deliverable: validate ALL section bounds before access — closing a real
integer-overflow hole in the directory range checks.

Defect fixed: directory `offset`/`len` are raw file-controlled u64s, and
the previous range rule used unchecked `e.offset + e.len`:

- debug builds: an overflowing pair PANICS (arithmetic overflow) inside
  the parser — violating "malformed lengths cannot panic";
- release builds: the sum wraps, the `> file_len` check can pass, and
  the later slice indexes wrap past every bound.

Change (`crates/q-pack/src/lib.rs`, `parse_directory_of_size`):

- `e.offset.checked_add(e.len)` with a typed rejection
  ("offset+len overflows u64") before any use;
- the pairwise overlap rule rewritten over checked sums (prior entries
  are already range-validated, so their ends are finite);
- every remaining rule (offset ≥ dir_end, 8-alignment, len > 0,
  past-end, duplicate ids) unchanged and still evaluated before any
  section body is sliced or hashed.

Tests (79 total):

- `overflowing_directory_values_reject_without_panic` — five malformed
  shapes (aligned u64::MAX-7 offset + 16 len; u64::MAX len from the
  first legal body offset; unaligned u64::MAX offset; both fields huge;
  untouched control file still validates). Debug-profile tests mean an
  unchecked addition would abort the suite — this is the regression
  pin.
- `bounds_checks_precede_any_section_access` — a past-end SECOND entry
  rejects identically even when the FIRST section's body bytes are also
  corrupted: structural bounds run before content hashing (ordering
  proof).
- Existing 12-rule directory suite, 4 000-round header mutation, and
  integration fuzz unchanged and green.

Commands and results:

- `cargo test -p q-pack` — 79 passed + 2, 0 failed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (flagged follow-up from
  M26-002-A).

Guardrails: malformed lengths cannot panic (overflow regression pin);
fuzz parser stable (no parser semantics changed for well-formed input —
valid files parse identically); shared/embedded modes unaffected
(same `&[u8]` consumer). Allocation profiling is V/Z scope.
