---
task_id: M4A-003-Z
parent_task: M4A-003
milestone: M4A
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-003-Z — Package evidence for Implement project scaffolding

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-003; update status only if verification passed.

## Parent intent

Create a minimal correct project without hidden demo credentials or broad dependencies.

## Dependencies

- `M4A-003-V` — `tasks/07_m4a_developer_preview/M4A-003-V-verify-implement-project-scaffolding.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

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
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-003-z: package evidence for implement project scaffolding
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-003-Z) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-003-z (squash-merged; see git log for final hash)
- Closes: #449
- Parent verification: M4A-003-V PASS (PR #1053, merged ed4b069) on the
  same verified surface; this packet packages the evidence and flips the ledger.

### Evidence package (parent M4A-003 — Implement project scaffolding)
- **Implementation commits (squash-merged):**
  - M4A-003-A starter API — #1049 → f2a7b2b
  - M4A-003-B treaty client example — #1050 → 9491614
  - M4A-003-C testing setup — #1051 → f97a09f
  - M4A-003-D optional fetch/profile choices — #1052 → 883774d
  - M4A-003-V verification closure (incl. two defect fixes) — #1053 → ed4b069
- **Source implementations:**
  - `packages/cli/src/scaffold.ts`: `generateStarterProject` — 10-file
    serverless starter (12 with fetch), module/service/contract separation,
    Treaty client example (`StarterApi` + `createClient`), `bun:test` unit +
    runtime-local contract suites, private-alpha dependency disclosure;
    `resolveServiceProfile` enforcing the runtime's fail-closed profile
    grammar (`serverless | service:N`, N = 1..64).
  - `packages/cli/src/index.ts`: `velqu init` / `velqu create` with
    `--name`, `--profile <serverless|service:N>` (fail closed on bare
    `service`/unknown names), `--with-fetch` / `--fetch`, `--force`, `--json`
    receipt; `dev`/`build` profile validation via the same resolver.
  - `packages/cli/src/profile-fetch-choices.test.ts`: 10 tests proving the
    optional choices end to end (templates → extraction → packaging → CLI).
  - `docs/beta/08_CLI_REFERENCE.md`: `velqu init` section (grammar,
    capability flag, private-alpha note).
- **Guardrail proofs (parent acceptance):**
  - Builds/tests/runs: static extraction + compile parity tests; scaffolded
    suite green from a bare directory; LIVE `velqu dev` run of a scaffolded
    `service:4` + fetch project answered `/health/live`, `POST /greetings`,
    and `GET /greetings/:name` through the real QuickJS runtime.
  - Module/service/contract best practices: per-module routes + service
    files; client consumes contract shape with zero shared runtime code.
  - No database/auth forced into core: no auth/db references;
    `scaffold.test.ts` asserts no credentials/secret/API_KEY strings.
  - Minimal dependencies: only `@velqu/core|schema|treaty` + dev tooling.
- **Required evidence:**
  - Scaffold snapshot tests: `scaffold.test.ts` (5), `testing-setup.test.ts` (2),
    `treaty-example.test.ts` (2), `profile-fetch-choices.test.ts` (10).
  - Fresh install test: init → symlinked `@velqu/*` → `bun test` 4/0 →
    `velqu build` 4-route QPack; resolution requirement disclosed in README
    and `init` output (regression-guarded).
  - Bundle-size report (4-route service:4 + fetch scaffold): `app.qpack`
    15070B, `route-manifest.json` 2131B, `schema-manifest.json` 1829B,
    `capability-manifest.json` 547B, `contract.json` 2857B, `contract.d.ts`
    1102B, `contract.meta.json` 748B, `openapi.json` 4389B,
    `contract.lock.json` 2824B, `build-report.json` 7713B,
    `app.qpack.sources.json` 38552B.
- **Gate results (M4A-003-V worktree-fresh):** `./scripts/verify` **ALL PASS**
  (velqu-runtime 7 suites, bun 277 across 38 files, typecheck, fmt,
  workspace clippy -D warnings).

### Ledger
- `docs/beta/04_TASK_LEDGER.md`: M4A-003 TODO → **PASS** (all four
  guardrails proven; see the M4A-003-V mapping).

### Disclosures (standing)
- No runtime behavior changed in this packet: evidence-only closure
  (the profile-grammar and README-disclosure defect fixes were delivered in
  the M4A-003-V packet and are recorded there).
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
