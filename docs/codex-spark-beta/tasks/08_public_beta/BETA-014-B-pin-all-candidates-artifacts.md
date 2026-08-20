---
task_id: BETA-014-B
parent_task: BETA-014
milestone: BETA
priority: P1
mode: IMPLEMENT
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-014-B — Pin all candidates/artifacts

## Atomic goal

Pin all candidates/artifacts.

## Parent intent

Create an honest comparison for beta users.

## Dependencies

- `BETA-014-A` — `tasks/08_public_beta/BETA-014-A-include-cold-start-categories-warm-microbenchmarks-real-db-auth-i-o-cpu-jit-cros.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Pin all candidates/artifacts.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Every number links to raw evidence.
- Fixture-specific wording.
- Velqu losses are included.
- No cloud cold-start claim from local process data.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Benchmark report.
- Raw archive.
- Methodology review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-014-b: pin all candidates artifacts
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
