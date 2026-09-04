---
task_id: BETA-010-V
parent_task: BETA-010
milestone: BETA
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-010-V — Verify Create supported beta platform and packaging matrix

## Atomic goal

Prove every acceptance criterion for parent task BETA-010 without broadening scope.

## Parent intent

Ship installable binaries/packages for an explicit narrow platform promise.

## Dependencies

- `BETA-010-A` — `tasks/08_public_beta/BETA-010-A-linux-x86-64-glibc-mandatory-working-assumption.md`
- `BETA-010-B` — `tasks/08_public_beta/BETA-010-B-linux-arm64-glibc-when-ci-is-available.md`
- `BETA-010-C` — `tasks/08_public_beta/BETA-010-C-npm-packages-under-beta-tag.md`
- `BETA-010-D` — `tasks/08_public_beta/BETA-010-D-runtime-binary-qpack-tools.md`
- `BETA-010-E` — `tasks/08_public_beta/BETA-010-E-clean-install-tests.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Platform CI.
- Package inventory.
- Install transcript.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
beta-010-v: verify create supported beta platform and packaging matrix
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-010-V) — PASS (2026-09-04)

- Branch/PR: beta-010-v (squash-merged; see git log for final hash)
- Closes: #565

### Behavior verified

Validated all parent acceptance criteria for task BETA-010 ("Create supported beta platform and packaging matrix"):
- **Published platform list is exact**: Linux x86_64 glibc is the only supported public platform promise (`docs/beta/governance/PLATFORM_SUPPORT.md`). Pinned toolchain, glibc version, and kernel transcript documented in `docs/reports/beta-010-a-linux-x86-64-glibc-platform.md`.
- **Unsupported platforms fail with guidance**: `PLATFORM_SUPPORT.md` and `docs/reports/beta-010-b-linux-arm64-glibc-ci.md` define boundaries; ARM64 remains conditional CI-only; macOS is development-only best effort; Windows and musl are unsupported.
- **Packages contain no accidental source/compiler artifacts**: All 9 `@velqu/*` packages in `packages/*/package.json` are explicitly marked `private: true` (`scripts/npm-package-inventory.sh` -> `PREPARED_NOT_PUBLISHED`). Multi-stage Dockerfile excludes build tooling from the runtime image.
- **Install works in clean environment**: `scripts/clean-install-test.sh` executes release binary and verified pack in an isolated temporary directory with zero repository state, validates fingerprint, serves routes, and exits cleanly (`CLEAN-INSTALL-TEST-OK`).
- Fixed `scripts/artifact-smoke.sh` standalone pack build to use an absolute path (`realpath "$PACK"`).

### Changed files

- `scripts/artifact-smoke.sh` (use absolute path for `VELQU_STANDALONE_PACK`)
- `docs/reports/beta-010-v-verify-platform-packaging-matrix.md` (verification report)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-010-V-verify-create-supported-beta-platform-and-packaging-matrix.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

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

- Verification-only task; no runtime functional changes.
- Linux x86_64 glibc is the sole public beta platform; ARM64 remains conditional until hosted CI and owner acceptance are complete.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
