---
task_id: BETA-012-F
parent_task: BETA-012
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-012-F — Deployment

## Atomic goal

Deployment.

## Parent intent

Make scope, support, and trade-offs impossible to misunderstand.

## Dependencies

- `BETA-012-E` — `tasks/08_public_beta/BETA-012-E-fetch-postgres-auth.md`

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
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/src/main.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Deployment.
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
beta-012-f: deployment
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-012-F) — PASS (2026-09-04)

- Branch/PR: beta-012-f (squash-merged; see git log for final hash)
- Closes: #579

### Behavior implemented

Audited and strengthened `docs/beta/DEPLOYMENT-REVERSE-PROXY.md`:
- The guide was already beta-accurate (reverse-proxy-first, forwarded-header policy, health/readiness/drain, rollout sequence); the gap was the untested Nginx sample.
- Rehearsed the sample's proxy semantics end-to-end with a non-TLS derivation (`listen 8080`, backend on private port 3100): runtime in a minimal glibc container + nginx:alpine edge with the doc's config shape. Through the edge: `/health/live` = `{"status":"ok"}`, `/health/ready` = `{"ready":true}`, `/hello/nginx` = `{"message":"Hello nginx"}`.
- Added an honest note to the doc: proxy semantics rehearsed via the non-TLS derivation; TLS directives require a real certificate environment.

Link check OK. No runtime behavior modified.

### Changed files

- `docs/beta/DEPLOYMENT-REVERSE-PROXY.md` (rehearsal note)
- `docs/reports/beta-012-f-deployment.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-012-F-deployment.md`
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

- Documentation change only; rehearsal used ephemeral local containers, no external systems.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
