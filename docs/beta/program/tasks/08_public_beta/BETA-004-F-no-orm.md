---
task_id: BETA-004-F
parent_task: BETA-004
milestone: BETA
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-004-F — No ORM

## Atomic goal

No ORM.

## Parent intent

Provide a real database story without enlarging core.

## Dependencies

- `BETA-004-E` — `tasks/08_public_beta/BETA-004-E-pool-limits-and-observability.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-pack/src/lib.rs`
- `benchmarks/real-world/postgres/`
- `benchmarks/real-world/SPEC.md`
- `packages/capability-postgres/ (create if absent)`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: No ORM.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- App without Postgres pays zero dependency/init cost.
- Queries are parameterized.
- Timeout cancels/releases connection safely.
- Pool exhaustion is bounded.
- W1/W2/W3 workloads pass.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Capability tests.
- Real-world results.
- Cold/RSS cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-004-f: no orm
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-004-F) — PASS (2026-09-04)

- Branch/PR: beta-004-f (squash-merged; see git log for final hash)
- Closes: #521

### Changed files
- `packages/capability-postgres/src/index.test.ts`: **surface-freeze
  test** — the exported function surface must be exactly `["sql"]`; a
  19-name builder/model/migration vocabulary must stay absent;
  `sql` stays positional-parameters-only. A builder added later fails
  the suite.
- `packages/capability-postgres/README.md` (new): the frozen surface +
  no-ORM statement + identity/linking/limits summary.
- `docs/beta/POSTGRES-CAPABILITY.md` (new, indexed in
  `docs/beta/INDEX.md`): normative capability guide — identity, fail-
  closed linking, lifecycle/safety, no-ORM posture.
- `docs/reports/beta-004-f-no-orm.md` (new): evidence report.

### Required evidence

- **Capability tests**: surface-freeze + parameterized-only tests 9/9
  (`bun test packages/capability-postgres`).
- **Real-world results**: no runtime changes; posture unchanged from
  BETA-004-A..E (live evidence there).
- **Cold/RSS cost report**: this packet adds no runtime code — only
  tests and documentation; costs unchanged.

### Commands

- `bun test packages/capability-postgres` -> 9 pass / 0 fail
- `bun test` -> 384 pass / 0 fail (62 files)
- `bun run typecheck` -> clean; fmt/clippy -> clean
- `./scripts/verify` -> ALL PASS (M0-M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
  (isolated netns; standing port-3000 environment note, BETA-002-C record)

### Guardrail mapping

- **App without Postgres pays zero dependency/init cost**: unchanged.
- **Queries are parameterized**: the only API is positional-params-only
  sql(); interpolation has no convenience path (freeze-tested).
- **Timeout cancels/releases safely**: unchanged from D.
- **Pool exhaustion is bounded**: unchanged from B/E.
- **W1/W2/W3 workloads pass**: parent exit; not claimed here.

### Standing CI disclosure

CI `verify` workflows stall/fail with zero executed steps on PR creation
across all branches (infrastructure-side, tracked since ~#714); the local
`./scripts/verify` run above is the real gate evidence for this packet.
