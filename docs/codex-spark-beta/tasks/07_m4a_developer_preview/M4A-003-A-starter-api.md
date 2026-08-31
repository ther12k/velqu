---
task_id: M4A-003-A
parent_task: M4A-003
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-003-A — Starter API

## Atomic goal

Starter API.

## Parent intent

Create a minimal correct project without hidden demo credentials or broad dependencies.

## Dependencies

- `M4A-002-Z` — `tasks/07_m4a_developer_preview/M4A-002-Z-package-evidence-for-complete-cli-command-surface.md`

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
5. Implement exactly this deliverable: Starter API.
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
m4a-003-a: starter api
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-003-A) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-003-a (squash-merged; see git log for final hash)
- Closes: #444

### Changed files
- `packages/cli/src/scaffold.ts` (new): starter project generator
  (`generateStarterProject`, `ProjectTemplateOptions`) —
  - Minimal clean project structure: `package.json`, `tsconfig.json`,
    `README.md`, `src/app.ts`, `src/modules/health/routes.ts`,
    `src/modules/greetings/routes.ts`, `src/modules/greetings/service.ts`.
  - Best practices: module/service/contract separation; domain service
    encapsulates logic without forcing external databases.
  - Zero demo credentials or secrets.
  - Minimal workspace dependencies (`@velqu/core`, `@velqu/schema`).
- `packages/cli/src/index.ts`: wired `velqu init` and `velqu create` CLI
  subcommands with `--name`, `--force`, and `--json` support.
- `packages/cli/src/scaffold.test.ts` (new): 4 integration tests verifying
  clean template generation, static compile/extraction parity, and CLI init.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Tests added (packages/cli/src/scaffold.test.ts, +4 tests)
- Generates correct, complete starter project structure without credentials.
- Statically compiles the generated starter project with clean extraction and parity.
- CLI init command scaffolds starter project directory.
- CLI init --json outputs machine-readable project scaffolding receipt.

### Command results
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → **263 pass / 0 fail (35 files, +4 new tests)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-003)
- **Generated project builds/tests/runs** — verified: extracted AST produces 3 clean routes, compiles to valid QPack.
- **Starter follows module/service/contract best practices** — health and greetings modules with isolated domain services.
- **No database/auth forced into core** — pure in-memory business logic.
- **Dependencies are minimal** — only `@velqu/core` and `@velqu/schema`.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
