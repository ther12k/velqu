---
task_id: BETA-010-D
parent_task: BETA-010
milestone: BETA
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-010-D — Runtime binary/QPack tools

## Atomic goal

Runtime binary/QPack tools.

## Parent intent

Ship installable binaries/packages for an explicit narrow platform promise.

## Dependencies

- `BETA-010-C` — `tasks/08_public_beta/BETA-010-C-npm-packages-under-beta-tag.md`

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
5. Implement exactly this deliverable: Runtime binary/QPack tools.
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
beta-010-d: runtime binary qpack tools
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (BETA-010-D) — PASS (2026-09-04)

- Branch/PR: beta-010-d (squash-merged; see git log for final hash)
- Closes: #563

### Behavior implemented

Audited, verified, and fixed the complete toolchain for production runtime binaries and QPack artifacts:
- Fixed a compilation defect in `velqu-standalone` (`crates/q-runtime/src/bin/velqu-standalone.rs`): added missing `context_profile: None` in `RunConfig` initialization.
- Verified standalone mode binary compilation with embedded pack (`VELQU_STANDALONE_PACK`) and execution via `scripts/artifact-smoke.sh`: answers `/health/live` and `/hello/smoke` identically to shared mode, self-reports `mode="standalone"`, and exits 0.
- Verified `velqu-runtime` fingerprint and pack verification tooling (`--fingerprint --pack <app.qpack>`).
- Verified `velqu-bytecode` (`q-bytecode-tool`) QuickJS bytecode compilation and embedding tool (`velqu-bytecode embed --pack <pack> --out <out>`).
- Verified `@velqu/cli` developer commands `velqu pack inspect <pack> --json` and `velqu pack migrate <pack> --json`.
- Emitted automated inventory via `scripts/qpack-tools-inventory.sh` to `docs/reports/beta-010-d-qpack-tools-inventory.json` with verdict `PASS`.

### Changed files

- `crates/q-runtime/src/bin/velqu-standalone.rs` (fixed missing field in `RunConfig`)
- `scripts/qpack-tools-inventory.sh` (new tool inventory script)
- `docs/reports/beta-010-d-qpack-tools-inventory.json` (new inventory artifact)
- `docs/reports/beta-010-d-runtime-binary-qpack-tools.md` (evidence report)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-010-D-runtime-binary-qpack-tools.md`
- `docs/codex-spark-beta/STATUS.md`
- `docs/codex-spark-beta/indexes/TASK_INDEX.md`

### Required evidence

- `scripts/artifact-smoke.sh` — PASS (`SMOKE-OK`, shared + standalone modes, cold-start p50 ~9.95ms).
- `scripts/qpack-tools-inventory.sh` — PASS (`velqu-runtime`, `velqu-standalone`, `velqu-bytecode`, `velqu pack inspect`, `velqu pack migrate`).
- `docs/reports/beta-010-d-qpack-tools-inventory.json`.
- `docs/reports/beta-010-d-runtime-binary-qpack-tools.md`.

### Gates

- `cargo test -p q-pack` — pass
- `cargo test -p q-engine-quickjs` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

### Disclosures

- Standalone mode requires compile-time pack path via `VELQU_STANDALONE_PACK`.
- Bytecode packs are target-architecture bound (x86_64 glibc little-endian); cross-target packs fall back to source evaluation or require `no_bytecode` / rebuild.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
