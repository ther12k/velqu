---
task_id: BETA-002-B
parent_task: BETA-002
milestone: BETA
priority: P1
mode: IMPLEMENT
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-002-B — Pin versions

## Atomic goal

Pin versions.

## Parent intent

Provide Raw Rust, Elysia 2, Hono/Bun, and Fastify/Node implementations of identical contracts.

## Dependencies

- `BETA-002-A` — `tasks/08_public_beta/BETA-002-A-match-sql-pool-jwt-timeouts-logging-responses-compression-and-deployment-limits.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
- `scripts/package`
- `scripts/release-packet`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Pin versions.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Candidates are semantically equivalent.
- No framework receives hidden advantages.
- All outputs pass contract fixtures.
- Version/hash metadata is captured.

## Targeted commands

```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Candidate source.
- Parity tests.
- Fairness report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-002-b: pin versions
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
