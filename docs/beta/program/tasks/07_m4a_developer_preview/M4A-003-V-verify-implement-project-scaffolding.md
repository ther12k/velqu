---
task_id: M4A-003-V
parent_task: M4A-003
milestone: M4A
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-003-V — Verify Implement project scaffolding

## Atomic goal

Prove every acceptance criterion for parent task M4A-003 without broadening scope.

## Parent intent

Create a minimal correct project without hidden demo credentials or broad dependencies.

## Dependencies

- `M4A-003-A` — `tasks/07_m4a_developer_preview/M4A-003-A-starter-api.md`
- `M4A-003-B` — `tasks/07_m4a_developer_preview/M4A-003-B-treaty-client-example.md`
- `M4A-003-C` — `tasks/07_m4a_developer_preview/M4A-003-C-testing-setup.md`
- `M4A-003-D` — `tasks/07_m4a_developer_preview/M4A-003-D-optional-fetch-profile-choices.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Scaffold snapshot tests.
- Fresh install test.
- Bundle-size report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-003-v: verify implement project scaffolding
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-003-V) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-003-v (squash-merged; see git log for final hash)
- Closes: #448

### Acceptance-criterion mapping (parent M4A-003 guardrails)

1. **Generated project builds/tests/runs** — verified:
   - Builds: scaffolded projects statically extract and compile
     (`scaffold.test.ts` "statically compiles the generated starter project",
     `profile-fetch-choices.test.ts` "generates upstream fetch module … ").
   - Tests: scaffolded suite runs green from a bare directory
     (`testing-setup.test.ts` "scaffolded project test suite runs green via bun test").
   - Runs: **LIVE** — a freshly scaffolded `service:4` + fetch project was
     started through `velqu dev` and answered `GET /health/live` →
     `{"status":"ok"}`, `POST /greetings` → `{"name":"FreshUser",
     "greeting":"Hello fresh install!"}`, `GET /greetings/FreshUser` →
     `{"message":"Hello fresh install!"}` through the real QuickJS runtime.
2. **Starter follows module/service/contract best practices** — verified:
   health/greetings(/upstream) modules each own routes + service; the Treaty
   client consumes the contract shape (`treaty-example.test.ts`).
3. **No database/auth forced into core** — verified: starter files contain no
   auth/db references; `scaffold.test.ts` asserts no credentials/secret/
   API_KEY strings anywhere in the generated tree.
4. **Dependencies are minimal** — verified: only `@velqu/core`, `@velqu/schema`,
   `@velqu/treaty` + `@types/bun`/`typescript` dev tooling.

### Required evidence

- **Scaffold snapshot tests**: `scaffold.test.ts` (5), `testing-setup.test.ts` (2),
  `treaty-example.test.ts` (2), `profile-fetch-choices.test.ts` (10).
- **Fresh install test**: `velqu init` into an empty temp dir → symlinked
  `@velqu/*` packages → `bun test` (4 pass / 0 fail) → `velqu build` → 4-route
  QPack. Private-alpha resolution requirement is now DISCLOSED in the generated
  README (`Dependencies (private alpha)` section) and in the `init` output note;
  regression-guarded by `scaffold.test.ts` "README discloses the private-alpha
  workspace resolution requirement".
- **Bundle-size report** (fresh scaffold, service:4 + fetch, 4 routes):
  `app.qpack` 15070B, `route-manifest.json` 2131B, `schema-manifest.json` 1829B,
  `capability-manifest.json` 547B, `contract.json` 2857B, `contract.d.ts` 1102B,
  `contract.meta.json` 748B, `openapi.json` 4389B, `contract.lock.json` 2824B,
  `build-report.json` 7713B, `app.qpack.sources.json` 38552B.

### Defects found by this verification and fixed here

1. **Scaffold emitted profiles the runtime fails closed on** (from M4A-003-D):
   the runtime grammar is strictly `serverless | service:N` (N = 1..64,
   `crates/q-runtime/src/service_profile.rs`); the D templates generated
   `velqu dev --profile service` / `--profile throughput`, which the runtime
   rejects ("service profile requires an explicit worker count" — reproduced
   live). Fix: `resolveServiceProfile` helper in `scaffold.ts` validating the
   exact runtime grammar; `velqu init|dev|build --profile` now fail closed in
   the CLI with actionable guidance; templates emit `service:N` scripts only.
2. **Fresh-install guidance was silently wrong**: scaffold README told users
   to run `bun install` although `@velqu/*` are workspace-protocol packages
   (not on npm in private alpha). Fix: README "Dependencies (private alpha)"
   section + `init` next-steps note stating the monorepo/symlink resolution
   requirement; documented `velqu init` in `docs/beta/08_CLI_REFERENCE.md`
   (was missing since M4A-003-A).

### Changed files

- `packages/cli/src/scaffold.ts` — `resolveServiceProfile`, grammar-aligned
  `ServiceProfileChoice`, private-alpha README disclosure.
- `packages/cli/src/index.ts` — `init`/`dev`/`build` use the resolver (fail
  closed), help text updated, init next-steps note.
- `packages/cli/src/profile-fetch-choices.test.ts` — rewritten to the runtime
  grammar (10 tests, incl. bare-`service`/`throughput` fail-closed cases).
- `packages/cli/src/scaffold.test.ts` — README disclosure test (+1).
- `docs/beta/08_CLI_REFERENCE.md` — `velqu init` section + command table row.
- `benchmarks/manifest.json` — refreshed.

### Verification runs (this branch, worktree-fresh)

- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → **277 pass / 0 fail (38 files, +2 net new tests)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures (standing)

- CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR. Local evidence above is complete.
