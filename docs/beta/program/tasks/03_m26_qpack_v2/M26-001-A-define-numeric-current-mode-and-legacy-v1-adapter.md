---
task_id: M26-001-A
parent_task: M26-001
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-001-A — Define numeric current mode and legacy v1 adapter

## Atomic goal

Define numeric current mode and legacy v1 adapter.

## Parent intent

Freeze the binary format goals, trust model, compatibility, and migration rules.

## Dependencies

- `M25-GATE` — `gates/M25-GATE.md`

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
5. Implement exactly this deliverable: Define numeric current mode and legacy v1 adapter.
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
m26-001-a: define numeric current mode and legacy v1 adapter
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-001-A)

Status: **PASS**.

### Deliverables

- **ADR-0024** (`docs/okf/decisions/0024-qpack-v2-numeric-mode-and-
  legacy-v1-adapter.md`): numeric mode policy (closed set, one CURRENT,
  fail closed on unknown), legacy v1 adapter as named compatibility
  boundary, no cross-mode handler table, trust model (untrusted
  arbitrary bytecode forbidden — compiler-owned rebuild path only),
  compatibility matrix, v1/v2 layout sketches (exact bytes deferred to
  M26-001-B/M26-003-B), migration rules.
- **q-pack mode dispatch** (`crates/q-pack/src/lib.rs`):
  `PACK_FORMAT_LEGACY_V1` / `PACK_FORMAT_CURRENT` /
  `PACK_FORMAT_VERSION` constants; `PackFormatMode::{LegacyV1}` enum;
  `detect_pack_format_mode()` resolving version→adapter and rejecting
  everything else with a fail-closed message naming the supported
  modes; `verify()` dispatches through it before all other checks.
  Behavior for existing packs is unchanged (v1 still loads).
- **Spec cross-reference**: `docs/specs/pack-format-v1.md` header now
  names the legacy-v1 adapter status and the fail-closed policy.

### Tests

- `legacy_v1_resolves_to_named_adapter` — v1 → `LegacyV1`, CURRENT pin,
  end-to-end verify of a v1 pack.
- `unknown_versions_fail_closed` — versions 0/2/3/u32::MAX rejected with
  "not supported … fail closed … legacy-v1"; a formatVersion=2 pack
  fails full `verify()`.
- `current_mode_is_pinned_until_native_v2_lands` — forces a conscious
  constant flip when M26-003 lands.

### Command results (fresh worktree)

- `cargo test -p q-pack` — 46 passed (43 prior + 3 new); `cargo test
  -p velqu-runtime` — 24 passed. `bun test` — 81 passed / 481 expect().
  typecheck/fmt/clippy clean. `./scripts/verify` — ALL PASS (exit 0;
  one manifest-hash refresh after the remapped release rebuild, same
  disclosed pattern as M25-010-C).
