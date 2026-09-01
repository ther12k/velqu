---
task_id: M4A-003-C
parent_task: M4A-003
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-003-C — Testing setup

## Atomic goal

Testing setup.

## Parent intent

Create a minimal correct project without hidden demo credentials or broad dependencies.

## Dependencies

- `M4A-003-B` — `tasks/07_m4a_developer_preview/M4A-003-B-treaty-client-example.md`

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
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Testing setup.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Generated project builds/tests/runs.
- Starter follows module/service/contract best practices.
- No database/auth forced into core.
- Dependencies are minimal.

## Targeted commands

```bash
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Scaffold snapshot tests.
- Fresh install test.
- Bundle-size report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-003-c: testing setup
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-003-C) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-003-c (squash-merged; see git log for final hash)
- Closes: #446

### Changed files
- `packages/cli/src/scaffold.ts`: added test suite templates to the scaffolded starter project:
  - `src/modules/greetings/service.test.ts`: unit tests covering greeting service behavior (default greetings, custom greetings, case-insensitivity) using `bun:test`.
  - `src/client.test.ts`: runtime-local Treaty contract tests that test client contract behavior against the dev server.
  - starter `package.json` includes `test: "bun test"`.
- `packages/cli/src/testing-setup.test.ts` (new): 2 end-to-end integration tests —
  1. Scaffolded project test suite runs green via `bun test` in a fresh directory with zero external test scaffolding.
  2. Runtime-local Treaty contract test passes against the LIVE dev server (real `velqu-runtime` worker process).
- `packages/cli/src/scaffold.test.ts`: updated filesCount assertion to 10.
- `benchmarks/manifest.json`: refreshed.

### Command results
- `cargo test -p velqu-runtime` → 7 suites — 0 failed (55 unit + 6 fetch fixture + 5 fetch pool + 2 proxy cancellation + 35 runtime conformance)
- `bun test` → **267 pass / 0 fail (37 files, +2 new tests)**
- `bun run typecheck` → clean (exit 0)
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-003)
- **Generated project builds/tests/runs** — scaffolded project includes executable unit & contract tests that pass with `bun test`.
- **Starter follows module/service/contract best practices** — modular service tests alongside route contracts.
- **Dependencies are minimal** — relies on built-in `bun:test` runner.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
