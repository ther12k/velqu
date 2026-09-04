---
task_id: BETA-012-B
parent_task: BETA-012
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-012-B — Quickstart

## Atomic goal

Quickstart.

## Parent intent

Make scope, support, and trade-offs impossible to misunderstand.

## Dependencies

- `BETA-012-A` — `tasks/08_public_beta/BETA-012-A-installation.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Quickstart.
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
beta-012-b: quickstart
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-012-B) — PASS (2026-09-04)

- Branch/PR: beta-012-b (squash-merged; see git log for final hash)
- Closes: #575

### Behavior implemented

Rewrote `docs/beta/QUICKSTART.md` from private-alpha framing to the public-beta reality:
- Replaced the broken `/tmp` scaffold flow (fails fail-closed on the pinned-toolchain check outside the monorepo) with the tested in-checkout flow: `create` + explicit `node_modules/@velqu/` workspace links + build + run + curl.
- Corrected the platform statement to Linux x86_64 glibc (macOS development-only).
- Documented the `service:N` profile grammar with a tested `--profile service:4` scaffold.
- Added a tested dev-loop sample (`dev --project <dir> --port <p>`) and the `inspect` command.
- Added guardrail language: bytecode-not-JIT accuracy note, no-SLA/no-production-readiness, no performance implications, trusted-code-only statement, bounded `defer`.

Every command/sample executed in this worktree with actual responses recorded (LIVE `{"status":"ok"}`, GREET `{"message":"Hello, world!"}`, dev-loop GREET `{"message":"Hello, dev!"}`). Link check OK; doc indexed by docs/beta/INDEX.md and README.md.

### Changed files

- `docs/beta/QUICKSTART.md` (rewritten)
- `docs/reports/beta-012-b-quickstart.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-012-B-quickstart.md`
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

- Documentation change only; no runtime behavior modified. Scaffold test directories removed before commit.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
