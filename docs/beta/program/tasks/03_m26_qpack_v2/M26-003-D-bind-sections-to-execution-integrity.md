---
task_id: M26-003-D
parent_task: M26-003
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-003-D — Bind sections to execution integrity

## Atomic goal

Bind sections to execution integrity.

## Parent intent

Serialize the already verified runtime graph without changing semantics.

## Dependencies

- `M26-003-C` — `tasks/03_m26_qpack_v2/M26-003-C-use-offsets-and-bounds-checks.md`

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
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Bind sections to execution integrity.
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
m26-003-d: bind sections to execution integrity
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-003-D (PASS)

Deliverable: bound every section of a QPack v2 file to a single
execution-integrity hash, so that any post-build change to section content
OR directory metadata is detectable even when an attacker repairs the
per-section content hash. Implements spec §2 growth path
(docs/specs/pack-format-v2.md): 96-byte extended header whose extension area
(offset 64) carries the aggregate binding.

Changed files: `crates/q-pack/src/lib.rs` only.

Implementation (qpack2::reader):

- `EXTENDED_HEADER_SIZE: u64 = 96`, `EXECUTION_HASH_OFFSET: usize = 64`.
- `compute_execution_hash(&[DirEntry]) -> [u8; 32]`: sha256 over the
  canonical directory — entries sorted by section_id, each encoded as
  (section_id u16, flags u16, offset u64, len u64, content_sha256 32B).
  Order-independence: two writers laying sections out in different orders
  produce the same binding.
- `parse_directory_with_binding(bytes)`: accepts only the 96-byte extended
  header, re-computes the binding from the parsed directory, and rejects on
  mismatch with "execution-integrity hash mismatch: the directory or any
  section content changed after build".
- `build_file_bound(payloads)`: aligned writer that stamps the binding into
  the extended header after laying out the directory.
- `parse_directory` refactored through shared `parse_directory_of_size` to
  accept both 64-byte (v2 base) and 96-byte (bound) headers; the dead
  duplicate `parse_directory_v1` body was removed.

Tests (crates/q-pack/src/lib.rs, all in qpack2::reader::tests):

- `bound_file_round_trips_and_binds_every_section` — build_file_bound output
  parses with parse_directory_with_binding and validates.
- `any_section_byte_change_breaks_the_binding` — mutates a section body byte
  AND repairs that section's content_sha256 in the directory so per-section
  integrity passes; only the binding catches the change. This is the
  attack the extended header exists for.
- `directory_field_change_breaks_the_binding` — flipping a directory offset
  field (content untouched) breaks the binding.
- Existing suite unchanged: every_directory_rule_violation_rejects (12
  rules), header_directory_mutation_never_panics (4000 rounds),
  bound/unbound round trips.

Commands and results:

- `cargo test -p q-pack` — 66 passed + 2 doc/it, 0 failed.
- `cargo test -p q-router` — 15 passed.
- `cargo test -p q-engine-quickjs` — 97 passed.
- `cargo test -p velqu-runtime` — 26 passed.
- `bun test` — 82 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt` applied; `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green except the pre-existing, documented
  `validate-benchmark-evidence` scoped failure (qRuntimeRelease + proofPack
  manifest hash mismatches inherited from M26-002-A pack-byte changes;
  canonical manifest refresh remains a flagged matched-evidence follow-up,
  not silently altered here).

Guardrail check: no startup semantics changed (reader-side only, unused by
production load path until M26-003-V); bounds rules unchanged; debug names
not touched.
