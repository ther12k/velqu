---
task_id: M26-003-C
parent_task: M26-003
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-003-C — Use offsets and bounds checks

## Atomic goal

Use offsets and bounds checks.

## Parent intent

Serialize the already verified runtime graph without changing semantics.

## Dependencies

- `M26-003-B` — `tasks/03_m26_qpack_v2/M26-003-B-store-router-nodes-edges-terminals-routeplans-schema-programs-policy-plans-funct.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-router/src/lib.rs`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Use offsets and bounds checks.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No semantic reconstruction at startup.
- Bounds and index validation reject malformed packs.
- Binary and transitional representations are property-equivalent.
- Debug names are optional and non-hot.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
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

- Round-trip/property tests.
- Mutation fuzzing.
- Section-size report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-003-c: use offsets and bounds checks
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-003-C)

Status: **PASS**. File-level offsets and bounds checks enforce every
spec §2/§3 rule BEFORE any section content is interpreted:

- `qpack2::reader`: `parse_header` (magic exact, version, header_size
  == 64, total_size == actual length, both reserved fields zero) and
  `parse_directory` (unique ids, offsets past header+directory,
  8-aligned, len > 0, disjoint ranges, within the file, per-entry
  reserved zero, unknown flag bits reject). `validate` adds the catalog
  rules — unknown section ids reject even when flagged optional (spec §5
  no-skip rule), all required ids must be present (spec §6) — and
  verifies every section's content sha256 at read time (integrity only,
  ADR-0026). `build_file` is the aligned writer with computed hashes.
- Bounds and index validation reject malformed packs: `every_directory_rule_violation_rejects`
  drives each rule to its rejection (magic, header_size, total_size,
  reserved, offset-overlap, misalignment, zero length, past-end, duplicate
  id, stale content hash, unknown id, missing required id).
- `header_directory_mutation_never_panics` — 4,000 single-bit mutations
  over the header+directory: no panic, bounded work; ≥3,300 reject
  (survivors are flips inside offset/len fields landing on legal values
  or the one legal flag bit — documented; section bodies are covered by
  the M26-003-B fuzz plus the content-hash check).
- `header_and_directory_round_trip` — built file validates; offsets
  8-aligned; ids exactly the required catalog.

### Tests and evidence

- `cargo test -p q-pack` — 63 + 2 passed (3 new reader tests).
- `cargo test -p q-router` — 15; `cargo test -p q-engine-quickjs` —
  1 + 97; `cargo test -p velqu-runtime` — 26 — all passed.
- `bun test` — 82 passed, 0 failed, 487 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — clean.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree benchmark-manifest mismatch (pack bytes changed in
  M26-002-A; canonical proofPack refresh flagged there).

Commit: `0ae49b5`.
