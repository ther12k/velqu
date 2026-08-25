---
task_id: M26-008-C
parent_task: M26-008
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-008-C — Deprecate mixed-mode packs

## Atomic goal

Deprecate mixed-mode packs.

## Parent intent

Keep old packs supportable without contaminating current hot paths.

## Dependencies

- `M26-008-B` — `tasks/03_m26_qpack_v2/M26-008-B-provide-velqu-pack-migrate-or-rebuild-guidance.md`

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
5. Implement exactly this deliverable: Deprecate mixed-mode packs.
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
m26-008-c: deprecate mixed mode packs
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

  Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-008-C)

Status: **PASS**.

### Deliverables

- **Mixed-mode gate** (`crates/q-pack/src/lib.rs`):
  `reject_mixed_mode_bytes` rejects, by name, the two hybrid shapes:
  (1) bytes starting with the `VELQUQPK` magic presented where a JSON
  pack is parsed; (2) a JSON pack carrying mode-2-reserved top-level
  fields (`sections`, `sectionDirectory`, `qpack2`) — closing the hole
  where serde silently dropped unknown keys and a hybrid artifact could
  load as v1 while tooling read different semantics. Reserved keys
  exported as `MODE2_RESERVED_JSON_KEYS`. Wired into BOTH load paths:
  `QPack::load_and_verify_with` (runtime/tooling) and
  `legacy_v1::read_and_verify_bytes`.
- **Compatibility fixture** (negative): committed
  `crates/q-pack/tests/fixtures/v1/mixed-mode-sections.json` (golden v1
  + injected `sections` key).
- **Migration tests** (+2): `mixed_mode_sections_key_is_rejected`
  (fixture → named rejection of `'sections'`),
  `binary_container_presented_as_json_is_rejected` (magic → mixed-mode
  error before any parse). Public contract unchanged for valid packs.
- **Deprecation documentation**: `docs/specs/pack-format-v1.md`
  migration section now defines what counts as mixed-mode, why the
  reserved-key gate exists (serde silent-drop hole), and that modes are
  exclusive per ADR-0024.

Guardrails held: no legacy structures on current paths (gate runs on
bytes before adapters); supported v1 still loads through the adapter;
hybrid artifacts fail with actionable rebuild/migrate messages; public
contract unchanged.

### Command results

- `cargo test -p q-pack` — 92 passed (+2); `cargo test -p
  velqu-runtime` — 28 passed; `bun test` — 89 passed / 0 fail / 531
  expect(); typecheck/fmt/clippy `-D warnings` clean. `./scripts/verify`
  — ALL PASS (exit 0; one manifest-hash refresh after remapped release
  rebuild — known pattern).
