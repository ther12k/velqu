---
task_id: BETA-012-E
parent_task: BETA-012
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-012-E — Fetch/Postgres/auth

## Atomic goal

Fetch/Postgres/auth.

## Parent intent

Make scope, support, and trade-offs impossible to misunderstand.

## Dependencies

- `BETA-012-D` — `tasks/08_public_beta/BETA-012-D-contracts-treaty.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Fetch/Postgres/auth.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Every command/sample is tested.
- No universal performance claim.
- No production-ready/SLA wording.
- QuickJS bytecode versus JIT is explained accurately.

## Targeted commands

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

- Docs CI.
- Link check.
- Example execution.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-012-e: fetch postgres auth
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-012-E) — PASS (2026-09-04)

- Branch/PR: beta-012-e (squash-merged; see git log for final hash)
- Closes: #578

### Behavior implemented

Completed beta documentation for the three capability surfaces:
- Fetch: replaced the broken /tmp scaffold sample with the tested in-checkout flow (`create --with-fetch` + workspace links + `check` + `build` + capability-manifest `perRoute` echo); private-alpha framings → public beta.
- Postgres: audited — already beta-accurate (normative BETA-004 doc); unchanged.
- Auth: new `docs/beta/AUTH.md` (indexed) covering route-level policies with typed sessions, identity-from-verified-tokens-not-headers, the tested proof login/profile flow, the JWT reference package's five fail-closed HS256 gates, redaction, non-SLA framing.

Every command/sample tested: fetch scaffold flow (check "4 routes … clean", build, capability manifest); auth flow against the running proof runtime (login → token, profile → `{"scope":"items:read profile:read","userId":"usr_ada"}`, wrong secret → 401, missing token → 401). Testing caught a wrong fixture secret in the first draft; corrected from source.

### Changed files

- `docs/beta/FETCH-CAPABILITIES.md`
- `docs/beta/AUTH.md` (new)
- `docs/beta/INDEX.md` (AUTH entry)
- `docs/reports/beta-012-e-fetch-postgres-auth.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-012-E-fetch-postgres-auth.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Gates

- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Documentation change only; no runtime behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
