---
task_id: M26-003-A
parent_task: M26-003
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-003-A — Define dense section schemas

## Atomic goal

Define dense section schemas.

## Parent intent

Serialize the already verified runtime graph without changing semantics.

## Dependencies

- `M26-001-Z` — `tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md`
- `G0-GATE` — `gates/G0-GATE.md`
- `M25-GATE` — `gates/M25-GATE.md`

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
5. Implement exactly this deliverable: Define dense section schemas.
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
m26-003-a: define dense section schemas
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-003-A)

Status: **PASS**. The dense section schemas for QPack v2 are defined in
the `qpack2` module with round-trip, mutation-fuzz, and size-report
evidence:

- **Section id catalog** (`qpack2::section`): the spec §6 reserved ids
  (STRINGS/ROUTES/ROUTE_PLANS/SCHEMA_MANIFEST/POLICIES/CAPABILITIES/
  BUNDLE_BYTECODE/CONTRACT_SUMMARY) plus `REQUIRED` — a pack missing any
  required id rejects.
- **Dense schemas** (each with encode + bounds-checked decode):
  - `strings_table` (0x0001): count + length-prefixed UTF-8; strict
    trailing-byte and UTF-8 checks.
  - `functions_table`: count + per record `id u32, key ref u32,
    kind u8`; key refs resolve against the strings table (out-of-bounds
    rejects; unknown kind bytes reject).
  - `policies_table` (0x0005, `PolicyRow` type): id/handler/provides
    refs + u16 status lists; `NONE_REF` sentinel for absent references.
  - `capabilities_table` (0x0006): count + string refs (the M26-002-A
    capability hash computes over the same sorted names).
  - `contract_summary` (0x0008): exactly 12 bytes — hash ref, route
    count, format revision.
  - `NONE_REF` sentinel: dense encodings never use Option.
- **Evidence**:
  - Round-trip: `dense_sections_round_trip` (full corpus decode-back)
    and `dense_sections_empty_round_trip` (empty tables are legal).
  - Mutation fuzz: `dense_sections_never_panic_under_mutation` — 2,000
    single-byte corruptions across all tables: no panic, bounded work,
    overwhelmingly-rejecting counts asserted (section content hashes at
    read time are the spec-level backstop, noted in the test).
  - Size report: `dense_section_size_report` — HONEST findings on a
    25-record corpus: record tables (functions/policies) are strictly
    smaller dense (fixed-width + string refs vs repeated JSON keys);
    the strings table is size-NEUTRAL for plain ASCII (JSON ~3
    bytes/string overhead vs dense's 4-byte length prefix) — its value
    is the shared-reference model; escape-heavy content flips strings
    decisively. The report prints measured numbers; assertions encode
    the honest claims only.

### Tests and evidence

- `cargo test -p q-pack` — 57 + 2 passed (5 new dense tests).
- `cargo test -p q-engine-quickjs` — 1 + 97; `cargo test -p velqu-runtime`
  — 26 — all passed.
- `bun test` — 82 passed, 0 failed, 487 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — clean.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree benchmark-manifest mismatch (pack bytes changed in
  M26-002-A; canonical proofPack refresh flagged there).

The full encoder (router/RoutePlans/schema sections) lands in
M26-003-B; offsets/bounds runtime readers in M26-003-C; integrity
binding in M26-003-D.

Commit: `2a31aff`.
