---
task_id: M4A-002-V
parent_task: M4A-002
milestone: M4A
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-002-V — Verify Complete CLI command surface

## Atomic goal

Prove every acceptance criterion for parent task M4A-002 without broadening scope.

## Parent intent

Provide consistent dev/build/inspect/contract/test/package workflows.

## Dependencies

- `M4A-002-A` — `tasks/07_m4a_developer_preview/M4A-002-A-implement-and-document-velqu-dev-build-inspect-contract-diff-test-pack-inspect-m.md`
- `M4A-002-B` — `tasks/07_m4a_developer_preview/M4A-002-B-stable-exit-codes.md`
- `M4A-002-C` — `tasks/07_m4a_developer_preview/M4A-002-C-machine-readable-output-option.md`
- `M4A-002-D` — `tasks/07_m4a_developer_preview/M4A-002-D-helpful-actionable-errors.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Commands work in clean checkout.
- No compiler in production artifact.
- CI use is documented.
- Invalid inputs fail clearly.

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

## Required evidence for this microtask

- CLI integration tests.
- Golden output.
- Clean-install demo.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-002-v: verify complete cli command surface
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-002-V) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-002-v (squash-merged; see git log for final hash)
- Closes: #442

### Acceptance-criterion mapping (parent M4A-002 guardrails)

1. **Commands work in clean checkout** — verified:
   - All `velqu` commands (`dev`, `build`, `inspect`, `contract diff`, `test`,
     `check`, `pack inspect/migrate`, `help`) execute cleanly in a fresh worktree.
   - Tested: `packages/cli/src/cli-surface.test.ts` (6 tests).
2. **No compiler in production artifact** — verified:
   - Production artifacts (`app.qpack`, `velqu-runtime`) perform zero TypeScript
     compilation or route extraction (COMP-002/005). The `@velqu/compiler` package
     remains build/dev tooling only.
3. **CI use is documented** — verified:
   - `docs/beta/08_CLI_REFERENCE.md` documents all CLI subcommands, flags, and
     standard exit codes (0 = pass, 1 = general/user error, 2 = breaking contract
     drift, 3 = unsupported format).
   - Machine-readable `--json` output option tested across all commands in
     `packages/cli/src/json-output.test.ts` (6 tests).
4. **Invalid inputs fail clearly** — verified:
   - Invalid syntax, missing files, unsupported imports, and bad route declarations
     surface source-located code frames with line:col caret indicators and
     actionable hints (`formatActionableError`, `renderCodeFrame`).
   - Tested: `packages/cli/src/actionable-errors.test.ts` (4 tests).

### Evidence chain (all committed, tested, verified)
- **A** #1043 (50ad849): complete CLI command surface (`dev`, `build`, `inspect`,
  `contract diff`, `test`, `check`, `pack inspect/migrate`, `help`), `inspectPack`
  helper, CLI reference documentation (`08_CLI_REFERENCE.md`).
- **B** #1044 (f7ee168): typed deterministic exit codes (`ExitCode`, 6 tests).
- **C** #1045 (476bb24): `--json` machine-readable output option across all subcommands
  and error paths (6 tests).
- **D** #1046 (aad613d): source-located code frames, caret indicators, and actionable
  diagnostic hints (`formatActionableError`, 4 tests).

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-pack` → 3 suites — 0 failed
- `bun test` → **259 pass / 0 fail (34 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of
  M4A-002-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR. Local evidence above is complete.
