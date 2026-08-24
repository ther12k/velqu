---
task_id: M26-007-C
parent_task: M26-007
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-007-C — Canonicalize section ordering and padding

## Atomic goal

Canonicalize section ordering and padding.

## Parent intent

Make identical source/locks/toolchain produce byte-identical packs.

## Dependencies

- `M26-007-B` — `tasks/03_m26_qpack_v2/M26-007-B-pin-compiler-runtime-versions.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`
- `package.json`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Canonicalize section ordering and padding.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Two clean builds produce identical SHA-256.
- Non-reproducibility is diagnosed.
- Build metadata lives outside deterministic payload or is canonical.
- CI verifies reproducibility.

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

- Independent builder report.
- Artifact hashes.
- Reproducibility test.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-007-c: canonicalize section ordering and padding
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-007-C (PASS)

Deliverable: the binary QPack v2 writers produce a CANONICAL layout —
sections in ascending section-id order regardless of caller-supplied
order, zero-only alignment padding — so identical section content
always yields byte-identical files and an identical execution binding.

### Changed files

- `crates/q-pack/src/lib.rs` only:
  - `qpack2::reader::build_file_bound` + `build_file` sort payloads by
    section id ascending before layout (payload tuples are `Copy`, so
    no other code changed); doc comments state the canonical-layout
    contract. Previously sections were laid out in caller order, so
    permuting identical content produced different bytes (and the
    execution binding over directory entries would differ too).
  - New test `section_order_and_padding_are_canonical`: for BOTH
    writers, reversed and rotated input permutations produce
    BYTE-IDENTICAL output; the directory is ascending by id; every
    offset is `SECTION_ALIGN`-aligned; every non-body byte between
    directory end and section bodies is zero; no trailing bytes after
    the last body.

### Padding semantics (canonical, now pinned by test)

Zero fill to the next 8-byte boundary before each section body; no
tail padding. The reader already rejected unaligned offsets; the
writers now guarantee the canonical complement.

### Artifact hashes

- `app.qpack` (JSON pack path) unchanged from M26-007-A/B evidence:
  `9fec4d4dfe08a9641977795756da2162c09468932cf9207e0b74a2290d39d4a7`.
- Binary v2 canonicalization is proven by permutation-invariance in
  the test above (identical bytes across input orders); independent-
  builder comparison remains M26-007-D.

### Commands and results

- `cargo test -p q-pack` — 86 passed + 2, 0 failed.
- `cargo test -p velqu-runtime` — 28 passed.
- `bun test` — 85 pass / 0 fail / 518 expect().
- `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` — clean.
- `./scripts/verify` — all gates green except the documented
  pre-existing `validate-benchmark-evidence` scoped failure
  (qRuntimeRelease + proofPack manifest hashes; flagged follow-up from
  M26-002-A). One unrelated flake (`graceful_shutdown_exits_zero`)
  failed once under full-gate parallel load and passed on isolated
  rerun AND on the next full-gate run — no code or assertion was
  changed for it.
