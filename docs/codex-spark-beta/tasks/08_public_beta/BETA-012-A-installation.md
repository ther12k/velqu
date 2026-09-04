---
task_id: BETA-012-A
parent_task: BETA-012
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-012-A — Installation

## Atomic goal

Installation.

## Parent intent

Make scope, support, and trade-offs impossible to misunderstand.

## Dependencies

- `M4A-GATE` — `gates/M4A-GATE.md`
- `BETA-004-Z` — `tasks/08_public_beta/BETA-004-Z-package-evidence-for-implement-optional-first-party-postgres-capability.md`
- `BETA-005-Z` — `tasks/08_public_beta/BETA-005-Z-package-evidence-for-implement-jwt-auth-reference-package.md`
- `BETA-008-Z` — `tasks/08_public_beta/BETA-008-Z-package-evidence-for-implement-reverse-proxy-drain-and-deployment-semantics.md`

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
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Installation.
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
beta-012-a: installation
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-012-A) — PASS (2026-09-04)

- Branch/PR: beta-012-a (squash-merged; see git log for final hash)
- Closes: #574

### Behavior implemented

Rewrote `docs/beta/INSTALL.md` as the accurate, test-backed installation guide:
- Mode table (shared vs standalone) with trade-offs; beta scope reminder (Linux x86_64 glibc, non-SLA, no production-readiness claim).
- Source-build prerequisites; `@velqu/*` packages noted as prepared-but-unpublished.
- Shared mode with engine-mismatch/SEC-001 behavior, loopback reverse-proxy default, `configVersion: 1`.
- Standalone section (replaces stale "not part of the beta" claim): exact `VELQU_STANDALONE_PACK` feature build and run.
- Container section with the tested `docker run` sample (in-container `VELQU_HOST=0.0.0.0` + `VELQU_PROXY_MODE=direct`, host publish `127.0.0.1:8080:3000`) and loopback-boundary explanation; compose + container-smoke references.
- Limits, and accurate bytecode-vs-JIT and no-universal-performance-claim notes.

Every command/sample executed in this worktree: builds OK; shared `/health/live` = `{"status":"ok"}`, `/hello/beta` = `{"message":"Hello beta"}`; standalone OK; container build+run OK with identical bodies. Link check: all referenced paths exist.

### Changed files

- `docs/beta/INSTALL.md` (rewritten)
- `docs/reports/beta-012-a-installation.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-012-A-installation.md`
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
- Container sample tested against the local docker daemon; image publication remains Owner-gated.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
