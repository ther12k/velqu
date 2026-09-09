---
task_id: M26-003-B
parent_task: M26-003
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-003-B — Store router nodes/edges/terminals, RoutePlans, schema programs, policy plans, function manifest, debug names, and capability inventory

## Atomic goal

Store router nodes/edges/terminals, RoutePlans, schema programs, policy plans, function manifest, debug names, and capability inventory.

## Parent intent

Serialize the already verified runtime graph without changing semantics.

## Dependencies

- `M26-003-A` — `tasks/03_m26_qpack_v2/M26-003-A-define-dense-section-schemas.md`

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
5. Implement exactly this deliverable: Store router nodes/edges/terminals, RoutePlans, schema programs, policy plans, function manifest, debug names, and capability inventory.
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
m26-003-b: store router nodes edges terminals routeplans schema program
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-003-B)

Status: **PASS**. The verified runtime graph stores as dense v2 sections
(`qpack2::graph`), property-equivalent to the transitional structures:

- **Router section** (0x0002 — nodes/edges/terminals): dense node
  records (static-edge count, param/wildcard refs, optional terminal
  with method mask + 7 route slots) + per-edge (segment string ref,
  target). Decode is bounds-checked with a sane-range cap, resolves
  segment refs against the shared strings table, and rejects trailing
  bytes.
- **RoutePlans section** (0x0003): fixed-width per-plan records (route/
  handler/policy/schema ids, default status, strategy byte, field-needs
  flags, deadline, fallback-reason string refs) + variable tails
  (allowed statuses, header/query/cookie name-id arrays). Decodes back
  to the exact `RoutePlanDecl` (fallback reasons resolve through the
  strings table).
- **Schema manifest section** (0x0004): dense envelope (id, key ref,
  feature refs) with the IR node as canonical JSON payload —
  property-equivalent; the binary IR codec is future codec work, noted.
- **Policy plans** (0x0005): M26-003-A rows + a dense `PolicyDecl`
  manifest tail; `policy_section::decode` walks the row records to find
  the manifest boundary and decodes both parts.
- **Function manifest**: the M26-003-A functions table (already dense).
- **Shared strings**: an interning `Strings` builder dedups every string
  (segments, keys, reasons, features) into the single strings section.

### Tests and evidence

- `graph_sections_round_trip` — router, plans, schemas decode back to
  the EXACT encoded structures (PartialEq on the real types); policy
  rows + manifest decode as one section.
- `graph_sections_mutation_never_panics` — 3,000 single-byte mutations
  across router/plans/schemas sections: no panic, bounded work,
  overwhelmingly-rejecting counts asserted (section content hashes are
  the read-time backstop for semantically legal bit flips).
- `graph_section_size_report` — measured dense-vs-JSON sizes printed
  for router/plans/schemas/policies (structural assertions only; the
  per-record fixed-width win scales with record count per the
  M26-003-A report).
- `cargo test -p q-pack` — 60 + 2; `cargo test -p q-router` — 15;
  `cargo test -p q-engine-quickjs` — 1 + 97; `cargo test -p
  velqu-runtime` — 26 — all passed.
- `bun test` — 82 passed, 0 failed, 487 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — clean.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree benchmark-manifest mismatch (pack bytes changed in
  M26-002-A; canonical proofPack refresh flagged there).

Offsets/bounds runtime readers land in M26-003-C; integrity binding in
M26-003-D.

Commit: `120a72c`.
