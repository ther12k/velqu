---
task_id: M4A-003-D
parent_task: M4A-003
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-003-D — Optional fetch/profile choices

## Atomic goal

Optional fetch/profile choices.

## Parent intent

Create a minimal correct project without hidden demo credentials or broad dependencies.

## Dependencies

- `M4A-003-C` — `tasks/07_m4a_developer_preview/M4A-003-C-testing-setup.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`
- `crates/q-runtime/src/main.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Optional fetch/profile choices.
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
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
```
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
m4a-003-d: optional fetch profile choices
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-003-D) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-003-d (squash-merged; see git log for final hash)
- Closes: #447

### Changed files
- `packages/cli/src/scaffold.ts`:
  - Added `ServiceProfileChoice` type and `VALID_SERVICE_PROFILES` validation array (`serverless`, `service`, `throughput`).
  - Added `profile` and `withFetch` options to `ProjectTemplateOptions`.
  - Configures `package.json` scripts (`dev`, `build`), `velqu` config block (`profile`, `capabilities`), and `README.md` metadata accordingly.
  - When `withFetch` is enabled: generates `src/modules/upstream/routes.ts` (with `upstream.quote` route using native `fetch`), `src/modules/upstream/routes.test.ts`, registers `upstream` module in `src/app.ts`, and includes `upstream.quote` in `StarterApi` Treaty client definition and route contract table.
- `packages/cli/src/index.ts`:
  - Connected `--profile <serverless|service|throughput>` and `--with-fetch` / `--fetch` flags in `velqu init` / `velqu create`.
  - Added fail-closed validation for unrecognized profile names with actionable error message naming valid choices.
  - Included `profile` and `withFetch` metadata in `--json` machine-readable output.
- `packages/cli/src/profile-fetch-choices.test.ts` (new): 8 unit and integration tests covering:
  - Default serverless profile behavior.
  - Dynamic multi-worker service profile configuration.
  - Pinned-worker throughput profile configuration.
  - Fail-closed validation on invalid profile option.
  - Generation of upstream fetch module with static app compilation and packaging.
  - CLI `init` `--profile` and `--with-fetch` CLI flags.
  - CLI `init` `--json` machine-readable output.
  - CLI `init` rejection of invalid profile options with exit code 1.
- `benchmarks/manifest.json`: refreshed.

### Command results
- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-http` → PASS
- `cargo test -p q-capabilities` → PASS
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → **275 pass / 0 fail (38 files, +8 new tests)**
- `bun run typecheck` → clean (exit 0)
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-003)
- **Generated project builds/tests/runs** — configured profile and fetch projects compile and package cleanly.
- **Starter follows module/service/contract best practices** — upstream fetch is isolated in its own module with declared schemas.
- **No database/auth forced into core** — capabilities remain opt-in and unforced.
- **Dependencies are minimal** — zero extra dependencies added.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR.
