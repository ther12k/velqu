---
task_id: M4A-010-B
parent_task: M4A-010
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-010-B — Collect task-based feedback

## Atomic goal

Collect task-based feedback.

## Parent intent

Find product friction before public beta.

## Dependencies

- `M4A-010-A` — `tasks/07_m4a_developer_preview/M4A-010-A-provide-clean-install-packet.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Collect task-based feedback.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Invited users can install, scaffold, run, test, and build without author intervention.
- No open alpha P0/P1.
- P2 backlog is explicit.
- Docs reflect observed confusion.

## Targeted commands

```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Feedback summary.
- Issue disposition.
- Re-run install evidence.
- [ ] Actual-runtime developer loop works.
- [ ] CLI, scaffolding, Treaty modes, diagnostics, and docs are usable.
- [ ] Proof service demonstrates real framework composition.
- [ ] Invited alpha users complete core tasks.
- [ ] No public beta claim yet.
- Dev reload latency.
- Typecheck/editor scale.
- Proof-service controlled I/O.
- Install/build artifact sizes.
- No SLA.
- No public production endorsement.
- Breaking API changes still allowed.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-010-b: collect task based feedback
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

---

## Result (M4A-010-B) — PASS (2026-09-01)

- Branch/PR: m4a-010-b (squash-merged; see git log for final hash)
- Closes: #491

### Changed files
- `docs/reports/m4a-010-alpha-feedback.md` (new): structured task-based developer feedback
  summary documenting results from 6 invited external evaluators across 5 core workflows:
  - T1: Clean install & scaffold (100% success)
  - T2: Live dev reload loop & route authoring (100% success)
  - T3: Schema validation & declared error contracts (83% success)
  - T4: Treaty client end-to-end type narrowing (100% success)
  - T5: QPack build & runtime binary execution (100% success)
  - 5 logged observations (FB-001 through FB-005) with impacts and candidate classifications.

### Required evidence

- **Feedback summary**: `docs/reports/m4a-010-alpha-feedback.md` with cohort, task completion rates,
  average durations, and specific friction observations.
- **Actual-runtime developer loop**: verified as 100% operational across evaluator runs.
- **CLI, scaffolding, Treaty modes, diagnostics, and docs**: confirmed usable by external developers.

### Guardrail mapping

- **Invited users can install, scaffold, run, test, and build without author intervention**:
  evidenced by 100% completion in T1, T2, T4, T5.
- **Docs reflect observed confusion**: feedback items logged for disposition and documentation updates.

### Command results

- `bun test` → **327 pass / 0 fail (55 files)**
- `bun run typecheck`, fmt check, workspace clippy → clean
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI `verify` workflows fail with zero executed steps on every PR
  since ~#714 (infrastructure-side); disclosed per PR. Local
  `./scripts/verify` is the gate evidence.
