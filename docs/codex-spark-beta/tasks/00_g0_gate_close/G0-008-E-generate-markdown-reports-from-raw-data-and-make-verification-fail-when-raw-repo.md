---
task_id: G0-008-E
parent_task: G0-008
milestone: G0
priority: P1
mode: VERIFY_OR_FIX
status: TODO
context_card: context/milestones/G0.md
commit_required: true
---

# G0-008-E — Generate Markdown reports from raw data and make verification fail when raw/report values diverge

## Atomic goal

Generate Markdown reports from raw data and make verification fail when raw/report values diverge.

## Parent intent

Close the frozen G0 benchmark evidence requirements instead of relying on a single clean pass.

## Dependencies

- `G0-008-D` — `tasks/00_g0_gate_close/G0-008-D-capture-allocation-startup-profiles-including-the-10-000-route-json-pack-parsing.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Generate Markdown reports from raw data and make verification fail when raw/report values diverge.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Markdown reports are generated from current raw data.
- Verifier fails on stale reports.
- No public claim uses a single spot check.
- Any regression is documented rather than hidden.

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
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Raw benchmark directory.
- Generated report.
- Environment and artifact manifest.
- Ablation results for relevant changes.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
g0-008-e: generate markdown reports from raw data and make verificatio
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
