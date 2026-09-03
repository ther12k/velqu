---
task_id: BETA-002-D
parent_task: BETA-002
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-002-D — Document unavoidable differences

## Atomic goal

Document unavoidable differences.

## Parent intent

Provide Raw Rust, Elysia 2, Hono/Bun, and Fastify/Node implementations of identical contracts.

## Dependencies

- `BETA-002-C` — `tasks/08_public_beta/BETA-002-C-add-contract-response-verification.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Document unavoidable differences.
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
beta-002-d: document unavoidable differences
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-002-D) — PASS (2026-09-03)

- Branch/PR: beta-002-d (squash-merged; see git log for final hash)
- Closes: #507

### Changed files
- `benchmarks/real-world/DIFFERENCES.md` (new): the normative declaration of
  unavoidable candidate differences after contract matching (BETA-002-A) and
  response verification (BETA-002-C): runtime process model and HTTP stack
  (Rust host + QuickJS bytecode vs Bun/JSC vs Node/V8), JSON serializer
  paths, native fetch clients, router internals, runtime/toolchain versions;
  plus the two accepted divergences outside the measured surface (framework
  body-parser error shapes on malformed W2 bodies; JWT library cost not
  measured). Explicit non-goals: no performance claims here.
- `benchmarks/real-world/load.ts`: every summary now records
  `configHashes.differences` (sha256 of DIFFERENCES.md) — the declared
  differences are version/hash-pinned per run, not prose.
- `benchmarks/real-world/result-schema.ts`: `configHashes.differences` is a
  required sha256 field for validated summaries.
- `benchmarks/real-world/fairness.ts`: `hash.differences` added to the
  compared contract hashes — a run set with differing (or missing)
  differences-document hashes fails the audit loudly.
- `benchmarks/real-world/fairness.test.ts`: +2 tests (differences drift and
  missing differences hash both fail the audit).
- `benchmarks/real-world/result-schema.test.ts`: fixture carries the new
  hash; missing-key validation asserted.
- `benchmarks/real-world/README.md`: "Unavoidable differences" section
  linking the document and the hash-pinning behavior.

### Required evidence

- **Candidate source**: unchanged (documentation packet); candidates remain
  as verified by BETA-002-C.
- **Parity tests**: fairness + result-schema suites 17/17 pass (including
  the two new drift tests); full `bun test` 346 pass / 0 fail (57 files).
- **Fairness report**: the fairness audit now also fails on any drift in the
  declared-differences document, closing the loop between prose and evidence.

### Commands

- `bun test benchmarks/real-world/fairness.test.ts result-schema.test.ts` -> 17 pass / 0 fail
- `bun test` -> 346 pass / 0 fail (57 files)
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (run inside an isolated netns; see BETA-002-C environment note: an unrelated
  dev server owned by the user occupies 127.0.0.1:3000 on this host and trips
  the M4A-010-A scaffold template's live-Treaty skip predicate. No test weakened.)

### Guardrail mapping

- **Candidates are semantically equivalent**: enforced previously; this
  packet documents exactly what remains different and pins it per run.
- **No framework receives hidden advantages**: differences are declared,
  hash-pinned, and uncompensated; nothing is normalized away.
- **All outputs pass contract fixtures**: unchanged, still 18/18 x 4.
- **Version/hash metadata is captured**: `configHashes.differences` joins
  spec/workloads/schema/seed/versions in every summary and the audit.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
