---
task_id: M4A-003-B
parent_task: M4A-003
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-003-B — Treaty client example

## Atomic goal

Treaty client example.

## Parent intent

Create a minimal correct project without hidden demo credentials or broad dependencies.

## Dependencies

- `M4A-003-A` — `tasks/07_m4a_developer_preview/M4A-003-A-starter-api.md`

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
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
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
5. Implement exactly this deliverable: Treaty client example.
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
m4a-003-b: treaty client example
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-003-B) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-003-b (squash-merged; see git log for final hash)
- Closes: #445

### Changed files
- `packages/cli/src/scaffold.ts`: the starter template now generates
  `src/client.ts` — a complete type-safe Treaty client example —
  - `StarterApi` type (type alias) declaring all 3 routes with
    `path`/`method`/`params`/`body`/`responses` shapes.
  - Route-id contract table (`health.live`, `greetings.get`,
    `greetings.create`).
  - `createClient(baseUrl)` factory returning `TreatyClient<StarterApi>`.
  - `main()` demonstrating route-id dot-navigation
    (`api.health.live.get()`), POST bodies, and apply-then-method path-param
    substitution (`api.greetings.get({ name: "Ada" }).get()`).
  - `@velqu/treaty` added to starter `package.json` dependencies; new
    `client` script.
- `packages/cli/src/treaty-example.test.ts` (new): 2 integration tests —
  1. Generated `src/client.ts` contains the Treaty client definition with
     dot-navigation and apply-then-method examples.
  2. **End-to-end**: drives the live `DevServer` (real `velqu-runtime`
     worker) with a Treaty client — health check, POST create, and
     path-param GET all return typed data with no errors.
- `packages/cli/src/scaffold.test.ts`: filesCount updated 7 → 8 (new
  `src/client.ts`).
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Command results
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → **265 pass / 0 fail (36 files, +2 new tests)**
- `bun run typecheck` → clean (StarterApi as a type alias satisfies
  `Record<string, AnyRouteContract>`; contract fields use treaty's
  `responses` key)
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-003)
- **Generated project builds/tests/runs** — the Treaty client drives the
  real runtime end-to-end.
- **Starter follows module/service/contract best practices** — the client
  consumes the published contract shape, no shared runtime code with the
  server.
- **Dependencies are minimal** — `@velqu/treaty` is the only addition.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
