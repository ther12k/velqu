---
task_id: M25-007-A
parent_task: M25-007
milestone: M25
priority: P1
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M25.md
commit_required: true
---

# M25-007-A — Tag fallback reason in RoutePlan

## Atomic goal

Tag fallback reason in RoutePlan.

## Parent intent

Support advanced cases without hiding performance or semantic costs.

## Dependencies

- `M25-003-Z` — `tasks/02_m25_schema_codecs/M25-003-Z-package-evidence-for-generate-params-query-header-decoders.md`
- `M25-004-Z` — `tasks/02_m25_schema_codecs/M25-004-Z-package-evidence-for-generate-json-body-decoders.md`
- `M25-005-Z` — `tasks/02_m25_schema_codecs/M25-005-Z-package-evidence-for-generate-status-specific-response-encoders.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
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
5. Implement exactly this deliverable: Tag fallback reason in RoutePlan.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Fallback never activates silently.
- Raw Response bypass behavior is documented.
- No contract claim is generated when adapter lacks required projection.
- Fallback routes pass conformance.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Inspect snapshots.
- Fallback integration tests.
- Performance delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-007-a: tag fallback reason in routeplan
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
