---
task_id: BETA-012-Z
parent_task: BETA-012
milestone: BETA
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-012-Z — Package evidence for Complete beta documentation and limitations

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-012; update status only if verification passed.

## Parent intent

Make scope, support, and trade-offs impossible to misunderstand.

## Dependencies

- `BETA-012-V` — `tasks/08_public_beta/BETA-012-V-verify-complete-beta-documentation-and-limitations.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

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

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```
```bash
./scripts/validate-okf
```

## Required evidence for this microtask

- Docs CI.
- Link check.
- Example execution.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-012-z: package evidence for complete beta documentation and limitat
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-012-Z) — PASS (2026-09-04)

- Branch/PR: beta-012-z (squash-merged; see git log for final hash)
- Closes: #584

### Behavior implemented

Evidence packaging and parent task closure for **BETA-012** ("Complete beta documentation and limitations"):
- Flipped parent row `BETA-012` to **PASS** in `docs/beta/04_TASK_LEDGER.md`.
- Consolidated evidence inventory across all child tasks (BETA-012-A through BETA-012-I, and verification BETA-012-V) in `docs/reports/beta-012-z-package-evidence.md`.
- Verified all parent acceptance guardrails: every command/sample tested; no universal performance claims; no production-ready/SLA wording; QuickJS bytecode vs JIT compilation accurately explained; docs link check passing with 0 errors.

### Changed files

- `docs/beta/04_TASK_LEDGER.md`
- `docs/reports/beta-012-z-package-evidence.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-012-Z-package-evidence-for-complete-beta-documentation-and-limitations.md`
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

- Evidence packaging and status tracking only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
