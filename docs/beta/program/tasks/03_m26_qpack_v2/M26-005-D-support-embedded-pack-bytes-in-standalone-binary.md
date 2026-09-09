---
task_id: M26-005-D
parent_task: M26-005
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-005-D — Support embedded pack bytes in standalone binary

## Atomic goal

Support embedded pack bytes in standalone binary.

## Parent intent

Map and validate the pack without reconstructing large owned trees.

## Dependencies

- `M26-005-C` — `tasks/03_m26_qpack_v2/M26-005-C-avoid-unsafe-unchecked-access-unless-independently-audited.md`

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
5. Implement exactly this deliverable: Support embedded pack bytes in standalone binary.
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
m26-005-d: support embedded pack bytes in standalone binary
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-005-D (PASS)

Deliverable: the reader supports embedded pack bytes — pack content
compiled into a standalone binary — completing the reader's
shared/embedded mode coverage (parent guardrail).

Change (`crates/q-pack/src/lib.rs` only):

- `qpack2::reader::PackBytes` gains `Embedded(&'static [u8])`:
  `include_bytes!`-style carrier for the standalone-binary mode
  (executable wiring itself is M26-009-B). Zero-copy by construction —
  validation borrows section views straight out of the executable
  image; nothing is copied or reconstructed.
- `pack_bytes_mapped_and_owned_validate_identically_zero_copy`
  extended: the same bound file validates identically through mapped,
  owned, AND embedded carriers (ids, hashes, byte equality), and every
  embedded section body pointer lies inside the static bytes (views,
  not copies).

Commands and results:

- `cargo test -p q-pack` — 80 passed + 2, 0 failed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (flagged follow-up from
  M26-002-A).

Guardrails: reader works for shared (Mapped/Owned) and embedded modes
(one `&[u8]` consumer, all carriers tested); malformed lengths cannot
panic (M26-005-B checked bounds apply to every carrier — the
missing/malformed and overflow tests share the same validate path);
fuzz parser stable (no parser change).
