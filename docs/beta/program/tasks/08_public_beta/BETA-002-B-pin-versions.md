---
task_id: BETA-002-B
parent_task: BETA-002
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
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

## Result (BETA-002-B) — PASS (2026-09-03)

- Branch/PR: beta-002-b (squash-merged; see git log for final hash)
- Closes: #505

### Changed files
- `benchmarks/real-world/versions.json`: candidate registry `hono` pin aligned
  4.13.4 -> 4.13.5 to match `candidates/package.json` and the frozen
  `candidates/bun.lock` (the only drift found); `notes` refreshed to record that
  the BETA-002-B cross-check tests now hold the registry, package manifest, and
  lockfile in lockstep. `velqu`, `elysia`, `fastify` pins verified already exact.
- `benchmarks/real-world/versions.test.ts`: added two regression tests —
  `BETA-002-B: registry pins match candidates/package.json exactly` (compares
  `versions.candidates[name]` against `pkg.dependencies[name]` for elysia/hono/
  fastify) and `BETA-002-B: registry pins match the frozen bun.lock exactly`
  (asserts each `"name@version"` resolved entry is present in the frozen lock).
  9/9 versions tests pass.

### Required evidence

- **Candidate source**: unchanged in this packet; BETA-002-A candidates remain
  pinned via `candidates/package.json` + frozen `candidates/bun.lock`.
- **Parity tests**: `versions.test.ts` now 9/9 pass, including the two new
  cross-check tests that fail on any future registry/manifest/lock drift.
- **Fairness report**: `versions.json` is the hash-fed fairness input; pins are
  now provably identical across registry, package manifest, and lockfile, so the
  version metadata channel cannot silently diverge.

### Commands

- `bun test benchmarks/real-world/versions.test.ts` -> 9 pass / 0 fail
- `bun test` -> 336 pass / 0 fail (56 files)
- `bun run typecheck` -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Guardrail mapping

- **Candidates are semantically equivalent**: all candidates resolve to the same
  pinned versions enforced against package.json and bun.lock by the new tests.
- **No framework receives hidden advantages**: exact-pin equality means no
  candidate can pick up a different (e.g. newer) framework version at install.
- **All outputs pass contract fixtures**: unchanged contract tests still pass
  (336/0).
- **Version/hash metadata is captured**: `versions.json` is the registry consumed
  by the fairness pipeline; drift it describes is now impossible without a test
  failure.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation across
all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
