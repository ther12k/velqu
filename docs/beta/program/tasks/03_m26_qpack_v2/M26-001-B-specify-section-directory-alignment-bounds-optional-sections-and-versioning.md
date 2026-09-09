---
task_id: M26-001-B
parent_task: M26-001
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-001-B — Specify section directory, alignment, bounds, optional sections, and versioning

## Atomic goal

Specify section directory, alignment, bounds, optional sections, and versioning.

## Parent intent

Freeze the binary format goals, trust model, compatibility, and migration rules.

## Dependencies

- `M26-001-A` — `tasks/03_m26_qpack_v2/M26-001-A-define-numeric-current-mode-and-legacy-v1-adapter.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`
- `package.json`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Specify section directory, alignment, bounds, optional sections, and versioning.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Unknown versions fail closed.
- Current mode has no legacy handler table.
- Compatibility policy is explicit.
- Untrusted arbitrary bytecode is forbidden.

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

- ADR.
- Binary layout diagrams.
- Compatibility matrix.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-001-b: specify section directory alignment bounds optional sections
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-001-B)

Status: **PASS**.

### Deliverables

- **ADR-0025** (`docs/okf/decisions/0025-qpack-v2-section-directory-
  and-bounds.md`): accepts the normative mode-2 layout; integrity
  binding per section now, authenticity policy deferred to M26-001-C.
- **Normative spec** (`docs/specs/pack-format-v2.md`): 64-byte header
  (`VELQUQPK` magic, format_version=2, total_size, section_count,
  reserved-zero growth room), 64-byte directory entries (u16 id, u16
  flags bit0 OPTIONAL, u64 offset, u64 len, content SHA-256), §3
  directory rules (uniqueness, containment, disjointness, alignment,
  digest match), §4 optional-section semantics, §5 unknown-id fail-
  closed + no-minor-revisions versioning, §6 reserved section catalog
  (encodings deferred to M26-003-B), §7 bounds/DoS posture. Binary
  layout diagrams throughout; compatibility matrix lives in ADR-0024
  and is unchanged by this packet.
- **q-pack `qpack2` module** (`crates/q-pack/src/lib.rs`): code-checked
  layout constants (`MAGIC`, `FORMAT_VERSION=2`, `HEADER_SIZE=64`,
  `DIR_ENTRY_SIZE=64`, `SECTION_ALIGN=8`, `FLAG_OPTIONAL`) with tests
  pinning spec parity — drift fails tests before reviews do. No v1
  behavior change; mode dispatch still rejects version 2 until the
  native adapter lands (tested).
- Spec cross-reference added to `docs/specs/pack-format-v1.md`.

### Tests

- `qpack2::tests::layout_constants_match_spec` — magic, sizes,
  alignment invariants.
- `qpack2::tests::mode_two_still_fails_closed_before_native_adapter`
  — no producer emits v2 before M26-003; dispatch stays closed.

### Command results (fresh worktree)

- `cargo test -p q-pack` — 48 passed (46 prior + 2 new); `cargo test
  -p velqu-runtime` — 24 passed; `bun test` — 81 passed / 481 expect();
  typecheck/fmt/clippy clean. `./scripts/verify` — ALL PASS (exit 0;
  one manifest-hash refresh after the remapped release rebuild, same
  disclosed pattern as M25-010-C).
