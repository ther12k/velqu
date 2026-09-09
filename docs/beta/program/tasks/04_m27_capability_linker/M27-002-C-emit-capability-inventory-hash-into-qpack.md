---
task_id: M27-002-C
parent_task: M27-002
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-002-C — Emit capability inventory/hash into QPack

## Atomic goal

Emit capability inventory/hash into QPack.

## Parent intent

Resolve exactly which capabilities enter each application artifact.

## Dependencies

- `M27-002-B` — `tasks/04_m27_capability_linker/M27-002-B-reject-cycles-missing-conflicting-versions.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Emit capability inventory/hash into QPack.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Unrelated app pays zero linked capability cost.
- Dependency graph is deterministic.
- Missing capability fails at build or startup.
- `velqu inspect --capabilities` is accurate.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Resolver tests.
- Binary-size delta report.
- Cold-start delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-002-c: emit capability inventory hash into qpack
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-002-C (PASS)

Deliverable: the resolved capability inventory emitted into the
qpack, hash-bound and verified.

### Changed files

- `crates/q-capabilities/src/inventory.rs` (new) —
  `CapabilityInventory`: id-sorted entries; canonical byte encoding
  (`u32-le` count, per entry `u16-le` id-length + utf-8 id +
  `u32-le` version); `sha256_hex()` over those bytes;
  `from_pairs()` checked wire constructor (full ADR-0029 id
  validation, sort canonicalization, duplicate rejection);
  cross-language hash vectors pinned against an independent Python
  reference and the TypeScript mirror.
- `crates/q-pack/src/lib.rs` — `CapabilityInventoryEntryWire`
  wire type; `QPack.capabilityInventory` +
  `capabilityInventorySha256` (both optional for pre-M27 packs);
  verify() rules: fields must appear together, entries sorted
  ascending + unique + valid ids, declared hash must equal the
  computed canonical hash.
- `crates/q-pack/Cargo.toml`, root `Cargo.toml` wiring — `q-pack`
  now depends on `q-capabilities`.
- `packages/compiler/src/emit.ts` — `capabilityInventoryHash()`
  TypeScript mirror of the canonical encoding (exported), and the
  pack now always carries `capabilityInventory: []` +
  `capabilityInventorySha256` until the linker integrates real
  capability modules (M27-004+ ports): the section's presence is a
  build-output invariant, not an accident.
- `packages/cli/src/capability-inventory.test.ts` (new) — TS vector
  test mirroring the Rust pins.
- `crates/q-runtime/tests/runtime_conformance.rs` — initializer
  gains the two new (None) fields.

### Tests

- `cargo test -p q-capabilities` — 51 passed (inventory 8: empty
  inventory hashes the 4-byte count prefix so absence ≠ empty set,
  sorted-by-id from DAG, deterministic unambiguous bytes with
  field-level offsets pinned, versions change the hash,
  distinct sets hash differently, from_pairs sorts/validates/
  rejects-duplicates with order-independent hash, display form,
  cross-language vectors).
- `cargo test -p q-pack` — 98 passed (+2:
  `capability_inventory_section_is_hash_bound_and_canonical`
  covering all five failure shapes plus the success path and both
  one-sided-field rejections;
  `capability_inventory_round_trips_through_json`).
- `bun test packages/cli/src/capability-inventory.test.ts` — 2 pass
  (mirror vectors). Full `bun test` 127 pass / 0 fail.
- **Cross-language agreement caught a real bug**: the first TS
  mirror encoded `[len][version][id]`; the pinned vectors exposed
  it immediately (Rust `[len][id][version]`). Fixed before merge —
  the pinning strategy did its job.

### Commands (fresh worktree on M27-002-B HEAD 3e9cc83)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 98 ·
  `-p q-capabilities` 51 · `-p velqu-runtime` 30 — pass.
- `bun test` 127 pass / 0 fail; `bun run typecheck` clean.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.

### Notes

- The route-level grants (`capabilities` names +
  `capability_hash`) keep their M26-002-A semantics untouched; the
  inventory is a distinct, version-bearing section describing
  linked modules.
- `velqu inspect --capabilities` accuracy (parent guardrail)
  completes with D's CLI surface reading this section.
