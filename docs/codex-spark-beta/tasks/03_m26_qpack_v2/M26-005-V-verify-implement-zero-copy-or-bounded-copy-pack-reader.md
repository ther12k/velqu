---
task_id: M26-005-V
parent_task: M26-005
milestone: M26
priority: P0
mode: VERIFY
status: TODO
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
