---
task_id: M4A-002-Z
parent_task: M4A-002
milestone: M4A
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-002-Z — Package evidence for Complete CLI command surface

## Atomic goal

Create source-backed evidence and handoff for parent task M4A-002; update status only if verification passed.

## Parent intent

Provide consistent dev/build/inspect/contract/test/package workflows.

## Dependencies

- `M4A-002-V` — `tasks/07_m4a_developer_preview/M4A-002-V-verify-complete-cli-command-surface.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`
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

- Commands work in clean checkout.
- No compiler in production artifact.
- CI use is documented.
- Invalid inputs fail clearly.

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

## Required evidence for this microtask

- CLI integration tests.
- Golden output.
- Clean-install demo.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m4a-002-z: package evidence for complete cli command surface
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-002-Z) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-002-z (squash-merged; see git log for final hash)
- Closes: #443
- Parent verification: M4A-002-V PASS (PR #1047, merged 76dd6f7) on the
  identical tree; this packet packages the evidence and flips the ledger.

### Evidence package (parent M4A-002 — complete CLI command surface)
- **Implementation commits (squash-merged):**
  - M4A-002-A complete CLI command surface — #1043 → 50ad849
  - M4A-002-B stable exit codes — #1044 → f7ee168
  - M4A-002-C machine-readable output option — #1045 → 476bb24
  - M4A-002-D helpful actionable errors — #1046 → aad613d
  - M4A-002-V verification closure — #1047 → 76dd6f7
- **Source implementations & documentation:**
  - `packages/cli/src/index.ts`: complete CLI command dispatcher (`dev`, `build`,
    `inspect`, `contract diff`, `test`, `check`, `pack inspect/migrate`, `help`)
    supporting `--json`, standard `--project` / `--out` / `--port` flags.
  - `packages/cli/src/exit-codes.ts`: deterministic exit code constants
    (`ExitCode.SUCCESS = 0`, `GENERAL_ERROR = 1`, `BREAKING_CONTRACT = 2`,
    `UNSUPPORTED_FORMAT = 3`).
  - `packages/cli/src/pack-inspect.ts`: `inspectPack` helper.
  - `packages/cli/src/errors.ts`: `formatActionableError` with `renderCodeFrame`.
  - `docs/beta/08_CLI_REFERENCE.md`: comprehensive CLI reference document.
- **Key test coverage (34 test files, 259 tests):**
  - `packages/cli/src/cli-surface.test.ts` (6 tests): inspect, check, pack inspect.
  - `packages/cli/src/exit-codes.test.ts` (6 tests): exact exit code verification.
  - `packages/cli/src/json-output.test.ts` (6 tests): `--json` across commands.
  - `packages/cli/src/actionable-errors.test.ts` (4 tests): code frames and hints.
- **Gate results (worktree-fresh):** `./scripts/verify` **ALL PASS** (incl.
  velqu-runtime 7 suites, bun 259, fmt, workspace clippy -D warnings).

### Ledger
- `docs/beta/04_TASK_LEDGER.md`: M4A-002 TODO → **PASS** (all four
  guardrails proven; see the M4A-002-V mapping).

### Disclosures (standing)
- No runtime behavior changed in this packet: evidence-only closure.
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
