---
task_id: M26-008-B
parent_task: M26-008
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-008-B — Provide `velqu pack migrate` or rebuild guidance

## Atomic goal

Provide `velqu pack migrate` or rebuild guidance.

## Parent intent

Keep old packs supportable without contaminating current hot paths.

## Dependencies

- `M26-008-A` — `tasks/03_m26_qpack_v2/M26-008-A-implement-separate-v1-reader-adapter.md`

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
5. Implement exactly this deliverable: Provide `velqu pack migrate` or rebuild guidance.
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
m26-008-b: provide velqu pack migrate or rebuild guidance
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

  Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-008-B)

Status: **PASS**.

### Deliverables

- **`velqu pack migrate <app.qpack>`** (`packages/cli/src/index.ts` new
  `pack` command): assesses a pack and prints the recommended path —
  legacy v1 → "loads via adapter today; rebuild from source with the
  current compiler (deterministic per M26-007), byte-stable and
  behavior-neutral; binary mode-2 guidance appears here when producers
  emit it". Unknown versions fail closed with the actionable message;
  unreadable/wrong-kind inputs report precisely. Usage text updated.
- **Pure assessment core** (`packages/cli/src/pack-migrate.ts`):
  `assessPackMigrate(reader-thunk)` returns a typed report — unit-
  testable without filesystem or process exits; CLI maps reports to
  output/exit codes.
- **Migration tests** (`packages/cli/src/pack-migrate.test.ts`, 4):
  golden v1 fixture → legacy-supported with rebuild/deterministic
  wording (uses the committed compatibility fixture from M26-008-A);
  version 7 → unsupported + fail-closed + doc pointer; non-JSON →
  unreadable; wrong kind → not-a-pack.
- **Deprecation documentation**: `docs/specs/pack-format-v1.md`
  migration-paths section now names the assess command, the deterministic
  rebuild path, and the future mode-2 reporting hook.

Guardrails held: no legacy structures on current paths (assessment is
read-only over bytes); supported v1 loads through the adapter or is
rebuilt; unsupported packs get an actionable message; public contract
unchanged.

### Command results

- `bun test` — 89 passed / 0 fail / 531 expect() (+4 tests); typecheck
  clean. `cargo test -p q-pack` — 90 passed; `cargo test -p
  velqu-runtime` — 28 passed. `./scripts/verify` — ALL PASS (exit 0).
