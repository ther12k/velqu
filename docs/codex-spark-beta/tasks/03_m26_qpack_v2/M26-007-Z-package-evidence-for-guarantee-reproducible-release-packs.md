---
task_id: M26-007-Z
parent_task: M26-007
milestone: M26
priority: P1
mode: EVIDENCE
status: TODO
context_card: context/milestones/M26.md
commit_required: true
---

# M26-007-Z — Package evidence for Guarantee reproducible release packs

## Atomic goal

Create source-backed evidence and handoff for parent task M26-007; update status only if verification passed.

## Parent intent

Make identical source/locks/toolchain produce byte-identical packs.

## Dependencies

- `M26-007-V` — `tasks/03_m26_qpack_v2/M26-007-V-verify-guarantee-reproducible-release-packs.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Independent builder report.
- Artifact hashes.
- Reproducibility test.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-007-z: package evidence for guarantee reproducible release packs
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
