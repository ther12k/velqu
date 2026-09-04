---
task_id: BETA-010-C
parent_task: BETA-010
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-010-C — npm packages under beta tag

## Atomic goal

npm packages under beta tag.

## Parent intent

Ship installable binaries/packages for an explicit narrow platform promise.

## Dependencies

- `BETA-010-B` — `tasks/08_public_beta/BETA-010-B-linux-arm64-glibc-when-ci-is-available.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: npm packages under beta tag.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Published platform list is exact.
- Unsupported platforms fail with guidance.
- Packages contain no accidental source/compiler artifacts.
- Install works in clean environment.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```

## Required evidence for this microtask

- Platform CI.
- Package inventory.
- Install transcript.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-010-c: npm packages under beta tag
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-010-C) — PASS (2026-09-04)

- Branch/PR: beta-010-c (squash-merged; see git log for final hash)
- Closes: #562

### Behavior implemented

Audited and cataloged the npm package publication surface:
- Verified that all 9 `@velqu/*` workspace package manifests are currently marked `private: true`, preventing accidental leakage or unauthorized publication to the npm registry.
- `scripts/npm-package-inventory.sh` inspects every package manifest without network I/O and records metadata, dependencies, entrypoints, and `publishable: false`.
- Machine-readable inventory emitted to `docs/reports/beta-010-c-npm-package-inventory.json` with verdict `PREPARED_NOT_PUBLISHED`.
- Preserves the release authority boundary (`docs/beta/governance/RELEASE_AUTHORITY.md`): the Owner is the sole release authority; npm publication under the `beta` tag requires owner authorization, explicit license/repository decisions (tracked in BETA-017), and SemVer prerelease versioning (`0.1.0-beta.1`).

### Changed files

- `docs/reports/beta-010-c-npm-beta-tag.md`
- `docs/reports/beta-010-c-npm-package-inventory.json`
- `scripts/npm-package-inventory.sh`
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-010-C-npm-packages-under-beta-tag.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Required evidence

- `scripts/npm-package-inventory.sh` — PASS (`PREPARED_NOT_PUBLISHED`, 9 packages, 9 private, 0 publishable).
- `docs/reports/beta-010-c-npm-package-inventory.json`.
- `docs/reports/beta-010-c-npm-beta-tag.md`.

### Gates

- `cargo test -p q-pack` — pass
- `cargo test -p q-engine-quickjs` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- All packages remain `private: true`; no package was published to npm or tagged `beta`.
- Owner release authorization and repository/license decisions remain prerequisite to public registry release.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
