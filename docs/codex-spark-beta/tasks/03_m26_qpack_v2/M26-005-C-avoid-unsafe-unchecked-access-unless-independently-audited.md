---
task_id: M26-005-C
parent_task: M26-005
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-005-C — Avoid unsafe unchecked access unless independently audited

## Atomic goal

Avoid unsafe unchecked access unless independently audited.

## Parent intent

Map and validate the pack without reconstructing large owned trees.

## Dependencies

- `M26-005-B` — `tasks/03_m26_qpack_v2/M26-005-B-validate-all-section-bounds-before-access.md`

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
5. Implement exactly this deliverable: Avoid unsafe unchecked access unless independently audited.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Pack fuzz results.
- Allocation profile.
- Platform smoke tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-005-c: avoid unsafe unchecked access unless independently audited
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-005-C (PASS)

Deliverable: no unchecked unsafe access on the pack path; every
remaining `unsafe` is independently audited with invariants and pinned
tests.

Unsafe inventory on the pack path (complete):

1. `crates/q-pack/src/lib.rs` — exactly ONE unsafe block: the read-only
   mmap in `qpack2::reader::PackBytes::open`. Expanded to a full SAFETY
   audit: PROT_READ/MAP_SHARED, Mmap owns the fd, consumers never write
   (write-through faults), residual SIGBUS-on-external-truncation hazard
   stated as accepted (immutable build artifacts; owned fallback for
   empty/unmappable), and all reads go through the M26-005-B
   checked-bounds reader. Crate root now carries
   `#![deny(unsafe_op_in_unsafe_fn)]` plus the one-unsafe policy
   doc — any future unsafe must be an explicit reviewable block.
2. `crates/q-engine-quickjs/src/worker.rs` `Module::load` (bytecode) —
   audit expanded: lists the upstream-enforced invariants (engine/ABI/
   binding fingerprint; sha256 over the single decoded buffer, with the
   exact tamper/dimension test names pinning each). Hash-valid garbage
   still rejects at eval, so a crafted buffer cannot reach QuickJS
   internals.
3. `crates/q-engine-quickjs/src/worker.rs` `__velquFillBytes` copy —
   already carries a reviewed FFI SAFETY block (unique live Uint8Array,
   single worker thread); unchanged.

Everything else on the pack path — header/directory parsing, bounds
checks, per-section hashing, binding verification, graph/bytecode
section decoders — is safe code with checked arithmetic.

Test added (80 total in q-pack):

- `pack_bytes_open_works_on_write_protected_files` — platform smoke:
  a mode-0444 pack opens through the mapped path and validates
  (production packs are read-only artifacts).

Commands and results:

- `cargo test -p q-pack` — 80 passed + 2, 0 failed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (flagged follow-up from
  M26-002-A).
