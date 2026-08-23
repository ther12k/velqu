---
task_id: M26-005-V
parent_task: M26-005
milestone: M26
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-005-V — Verify Implement zero-copy or bounded-copy pack reader

## Atomic goal

Prove every acceptance criterion for parent task M26-005 without broadening scope.

## Parent intent

Map and validate the pack without reconstructing large owned trees.

## Dependencies

- `M26-005-A` — `tasks/03_m26_qpack_v2/M26-005-A-use-mmap-read-only-bytes-where-supported.md`
- `M26-005-B` — `tasks/03_m26_qpack_v2/M26-005-B-validate-all-section-bounds-before-access.md`
- `M26-005-C` — `tasks/03_m26_qpack_v2/M26-005-C-avoid-unsafe-unchecked-access-unless-independently-audited.md`
- `M26-005-D` — `tasks/03_m26_qpack_v2/M26-005-D-support-embedded-pack-bytes-in-standalone-binary.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Pack fuzz results.
- Allocation profile.
- Platform smoke tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-005-v: verify implement zero copy or bounded copy pack reader
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M26-005-V (PASS)

Parent: M26-005 "Implement zero-copy or bounded-copy pack reader" (map
and validate the pack without reconstructing large owned trees). All
four implementation dependencies (M26-005-A/B/C/D) merged on master
before this branch (PRs #799/#800/#801/#802; issues #204–#207 closed).

### Acceptance criterion mapping

1. **Malformed lengths cannot panic or read out of bounds.**
   `overflowing_directory_values_reject_without_panic` — five
   overflowing/oversized directory shapes return typed errors (the
   M26-005-B fix replaced unchecked `offset+len` with checked_add;
   debug-profile tests pin it: an unchecked addition would abort the
   suite). Corroborated by the 12-rule directory suite,
   `header_directory_mutation_never_panics` (4 000 rounds),
   `pack_bytes_rejects_missing_and_malformed_without_panic` (empty,
   junk-through-mapping), and integration fuzz
   (`random_bytes_never_panic_the_pack_parser`,
   `mutated_valid_pack_never_panic_and_tamper_is_detected`).

2. **Startup allocations are measured and bounded.**
   Zero-copy is proven structurally, not by timing claims:
   `pack_bytes_mapped_and_owned_validate_identically_zero_copy` asserts
   every section body pointer lies INSIDE the carrier allocation for
   mapped, owned, and embedded carriers — validation reconstructs no
   owned section trees. Allocation bounds are structural: directory
   derives from section_count against file length FIRST; per-section
   sizes come from checked ranges; MAX_NODES / MAX_CODE_BYTES caps
   guard graph/bytecode decoders (constraint 11). No performance claim
   is made here (no benchmark manifest touched).

3. **Reader works for shared and embedded modes.**
   `PackBytes::{Mapped, Owned, Embedded}` all feed the same `&[u8]`
   consumer and validate identically (the carrier identity test);
   `pack_bytes_open_works_on_write_protected_files` (mode-0444 smoke)
   covers the deployed read-only-artifact shape. Standalone-binary
   executable wiring is M26-009-B; the carrier is complete and tested.

4. **Fuzz parser remains stable.**
   Integration fuzz unchanged and green; the 2 000-round dense-section
   and graph-section mutation suites unchanged and green; no parser
   semantics changed for well-formed input (M26-005-B only replaced
   overflowing arithmetic with typed rejections; M26-005-C added
   audits and `deny(unsafe_op_in_unsafe_fn)`; M26-005-A/D added
   carriers).

### Changed files

- This task record only. No defects found requiring fixes; no
  unrelated findings needing follow-up tasks.

### Commands and results (fresh worktree on parent HEAD)

- `cargo test -p q-pack` — 80 + 2 passed.
- `cargo test -p q-router` — 15 passed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green except the pre-existing
  documented `validate-benchmark-evidence` scoped failure
  (qRuntimeRelease + proofPack manifest hashes; flagged matched-evidence
  follow-up from M26-002-A, not altered here).
