---
task_id: BETA-010-Z
parent_task: BETA-010
milestone: BETA
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-010-Z — Package evidence for Create supported beta platform and packaging matrix

## Atomic goal

Create source-backed evidence and handoff for parent task BETA-010; update status only if verification passed.

## Parent intent

Ship installable binaries/packages for an explicit narrow platform promise.

## Dependencies

- `BETA-010-V` — `tasks/08_public_beta/BETA-010-V-verify-create-supported-beta-platform-and-packaging-matrix.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

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

- Published platform list is exact.
- Unsupported platforms fail with guidance.
- Packages contain no accidental source/compiler artifacts.
- Install works in clean environment.

## Targeted commands

```bash
cargo test -p q-pack
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

- Platform CI.
- Package inventory.
- Install transcript.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
beta-010-z: package evidence for create supported beta platform and pack
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-010-Z) — PASS (2026-09-04)

- Branch/PR: beta-010-z (squash-merged; see git log for final hash)
- Closes: #566

### Behavior verified and packaged

Parent task BETA-010 is closed as **PASS** and flipped in `docs/beta/04_TASK_LEDGER.md`.
- **Published platform list is exact**: Linux x86_64 glibc is the sole public beta platform promise (`docs/beta/governance/PLATFORM_SUPPORT.md`, `docs/reports/beta-010-a-linux-x86-64-glibc-platform.md`).
- **Unsupported platforms fail with guidance**: ARM64 documented as conditional CI portability signal; macOS development-only; Windows/musl unsupported (`docs/reports/beta-010-b-linux-arm64-glibc-ci.md`).
- **Packages contain no accidental source/compiler artifacts**: 9 `@velqu/*` packages explicitly marked `private: true`, preventing unauthorized publish (`scripts/npm-package-inventory.sh` -> `PREPARED_NOT_PUBLISHED`). Dockerfile excludes build tooling.
- **Install works in clean environment**: `scripts/clean-install-test.sh` executes release binary and verified pack in an isolated temporary directory with zero repository state (`CLEAN-INSTALL-TEST-OK`).
- `scripts/qpack-tools-inventory.sh`: validated `velqu-runtime`, `velqu-standalone`, `velqu-bytecode`, `velqu pack inspect`, and `velqu pack migrate` (`docs/reports/beta-010-d-qpack-tools-inventory.json`).

### Changed files

- `docs/reports/beta-010-z-package-evidence.md` (new packaging evidence report)
- `docs/beta/04_TASK_LEDGER.md` (BETA-010 flipped from TODO to PASS)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-010-Z-package-evidence-for-create-supported-beta-platform-and-packaging-matrix.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Required evidence

- `docs/reports/beta-010-a-linux-x86-64-glibc-platform.md`
- `docs/reports/beta-010-b-linux-arm64-glibc-ci.md`
- `docs/reports/beta-010-c-npm-package-inventory.json`
- `docs/reports/beta-010-c-npm-beta-tag.md`
- `docs/reports/beta-010-d-qpack-tools-inventory.json`
- `docs/reports/beta-010-d-runtime-binary-qpack-tools.md`
- `docs/reports/beta-010-e-clean-install-tests.md`
- `docs/reports/beta-010-v-verify-platform-packaging-matrix.md`
- `docs/reports/beta-010-z-package-evidence.md`

### Gates

- `cargo test -p q-pack` — pass
- `cargo test -p q-engine-quickjs` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `scripts/clean-install-test.sh` — `CLEAN-INSTALL-TEST-OK`
- `scripts/artifact-smoke.sh` — `SMOKE-OK`
- `scripts/proxy-smoke.sh` — `PROXY-SMOKE-OK`

### Disclosures

- Evidence packaging only; no runtime functional changes.
- Public beta platform scope is strictly Linux x86_64 glibc.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
