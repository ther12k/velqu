---
task_id: M26-008-A
parent_task: M26-008
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-008-A — Implement separate v1 reader/adapter

## Atomic goal

Implement separate v1 reader/adapter.

## Parent intent

Keep old packs supportable without contaminating current hot paths.

## Dependencies

- `M26-001-Z` — `tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md`
- `M26-005-Z` — `tasks/03_m26_qpack_v2/M26-005-Z-package-evidence-for-implement-zero-copy-or-bounded-copy-pack-reader.md`

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
5. Implement exactly this deliverable: Implement separate v1 reader/adapter.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Current runtime path allocates no legacy structures.
- Supported v1 pack either migrates or loads through adapter.
- Unsupported pack fails with actionable message.
- Migration does not change public contract.

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

- Compatibility fixtures.
- Migration tests.
- Deprecation documentation.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-008-a: implement separate v1 reader adapter
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-008-A)

Status: **PASS**.

### Deliverables

- **Separate v1 reader/adapter** (`crates/q-pack/src/legacy_v1.rs`,
  wired as `pub mod legacy_v1`): the single sanctioned entry point for
  legacy packs — `read_and_verify` (disk) / `read_and_verify_bytes`
  (bytes), each re-gating `detect_pack_format_mode` so legacy
  structures are constructed only behind the adapter. Module docs
  state the isolation invariant (qpack2 zero-copy hot paths share no
  types or code path; they borrow, the adapter builds owned trees) and
  the deprecation policy (supported through M2.6; removal needs an
  explicit owner decision).
- **Compatibility fixture** (required evidence): golden v1 pack
  committed at `crates/q-pack/tests/fixtures/v1/minimal.json`,
  regenerable via `cargo run -p q-pack --example gen-fixture`; test
  `loads_committed_v1_fixture` pins byte-for-byte loadability.
- **Actionable failure** (guardrail): unsupported-version rejection now
  names the way out — "rebuild the pack with the current compiler or
  migrate it (see docs/specs/pack-format-v1.md deprecation notes)";
  pinned by `unsupported_version_message_is_actionable`.
- **Migration tests + deprecation documentation**: new "Deprecation and
  migration" section in `docs/specs/pack-format-v1.md` (status
  deprecated-but-supported, loader entry points, both migration paths,
  pointers to M26-008-B/C/D).
- Public contract unchanged: all existing `q_pack::*` paths still work;
  no behavior change for valid v1 packs.

### Command results (fresh on branch m26-008-a)

- `cargo test -p q-pack` — 90 passed (+3: two adapter tests + fixture);
  `cargo test -p velqu-runtime` — 28 passed. `bun test` — 0 fail / 518
  expect(). typecheck/fmt/clippy `-D warnings` clean.
- `./scripts/verify` — ALL PASS (exit 0; one manifest-hash refresh after
  the remapped release rebuild — known pattern).
