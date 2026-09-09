---
task_id: M26-001-D
parent_task: M26-001
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-001-D — Define debug/source sidecar policy

## Atomic goal

Define debug/source sidecar policy.

## Parent intent

Freeze the binary format goals, trust model, compatibility, and migration rules.

## Dependencies

- `M26-001-C` — `tasks/03_m26_qpack_v2/M26-001-C-separate-integrity-from-authenticity.md`

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
5. Implement exactly this deliverable: Define debug/source sidecar policy.
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
m26-001-d: define debug source sidecar policy
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-001-D)

Status: **PASS**.

### Deliverables

- **ADR-0027** (`docs/okf/decisions/0027-debug-source-sidecar-
  policy.md`): production packs carry no debug/source content — mode 2
  has no source/source-map section (bytecode section OPTIONAL so the
  source rebuild path survives); legacy v1 keeps its frozen optional
  `sourceMap` but producers should omit it for production, with the
  default flip landing atomically with the M26-003 encoder. Debug
  material moves to an external `<pack>.sources.json` sidecar
  (`packSha256`-bound, tool-verified): the runtime NEVER reads it —
  no load path, no fallback, no env knob. Sidecars inherit ADR-0026:
  integrity-referenced, authenticity-free, behavior-neutral. Includes
  sidecar layout diagram and compatibility matrix (pack type x sidecar
  presence).
- **Test pinning runtime independence**
  (`crates/q-pack/src/lib.rs`):
  `verification_is_independent_of_debug_sidecars` — a pack verifies
  identically with and without embedded source-map text; verification
  takes no sidecar input.
- **Spec cross-references**: `docs/specs/pack-format-v2.md` header now
  states there is no source/map section by design (ADR-0027);
  `docs/specs/pack-format-v1.md` marks `sourceMap` as debug material.

### Command results (fresh worktree)

- `cargo test -p q-pack` — 50 passed (49 + 1 new); `cargo test -p
  velqu-runtime` — 24 passed; `bun test` — 81 passed / 481 expect();
  typecheck/fmt clean; clippy `-D warnings` clean (one unused-mut in
  the new test fixed before commit); `./scripts/verify` — ALL PASS
  (exit 0).
