---
task_id: BETA-011-Z
parent_task: BETA-011
milestone: BETA
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-011-Z — Package evidence for Automate beta publishing and versioning

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-011; update status only if verification passed.

## Parent intent

Produce repeatable pre-release packages without implying API stability.

## Dependencies

- `BETA-011-V` — `tasks/08_public_beta/BETA-011-V-verify-automate-beta-publishing-and-versioning.md`

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
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`
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

- Version is consistent across packages/binary/QPack.
- Re-running release does not mutate existing version.
- Rollback procedure is tested.
- Breaking beta changes require notes.

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

- Dry-run publish.
- Release workflow logs.
- Rollback rehearsal.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-011-z: package evidence for automate beta publishing and versioning
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-011-Z) — PASS (2026-09-04)

- Branch/PR: beta-011-z (squash-merged; see git log for final hash)
- Closes: #573

### Evidence packaged

Full evidence closure for parent BETA-011; ledger row flipped to PASS in `docs/beta/04_TASK_LEDGER.md`:
- Packet inventory A–E + V with canonical evidence paths and PR numbers (#1167–#1172) in `docs/reports/beta-011-z-package-evidence.md`.
- All four acceptance guardrails mapped to evidence: version uniformity (0.1.0 across Cargo.toml / 9 packages / q-pack compiler string), non-mutating release re-run (370bb8b rehearsal), tested rollback (`yank-rollback-rehearsal.sh` PASS), breaking-change notes (`docs/beta/CHANGELOG.md` shipped in packet).
- All three rehearsal scripts re-run in this worktree: PASS with deterministic output, zero tree changes.

### Changed files

- `docs/reports/beta-011-z-package-evidence.md`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-011-Z-package-evidence-for-automate-beta-publishing-and-versioning.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- `docs/beta/04_TASK_LEDGER.md` (BETA-011 → PASS)

### Gates

- `cargo test -p velqu-runtime` — pass (101+6+5+2+37+3)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- npm/GitHub publication remains Owner-gated; all publishing evidence is dry-run/simulation by design.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
