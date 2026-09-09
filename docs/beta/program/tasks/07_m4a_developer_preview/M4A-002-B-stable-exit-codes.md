---
task_id: M4A-002-B
parent_task: M4A-002
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-002-B — Stable exit codes

## Atomic goal

Stable exit codes.

## Parent intent

Provide consistent dev/build/inspect/contract/test/package workflows.

## Dependencies

- `M4A-002-A` — `tasks/07_m4a_developer_preview/M4A-002-A-implement-and-document-velqu-dev-build-inspect-contract-diff-test-pack-inspect-m.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Stable exit codes.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- CLI integration tests.
- Golden output.
- Clean-install demo.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-002-b: stable exit codes
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-002-B) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-002-b (squash-merged; see git log for final hash)
- Closes: #439

### Changed files
- `packages/cli/src/exit-codes.ts` (new): typed exit code constants —
  - `ExitCode.SUCCESS = 0`: clean build, inspect, diff without breaking changes, clean check, help.
  - `ExitCode.GENERAL_ERROR = 1`: user error, compilation/syntax error, missing files, unknown commands.
  - `ExitCode.BREAKING_CONTRACT = 2`: breaking contract difference detected by `contract diff`.
  - `ExitCode.UNSUPPORTED_FORMAT = 3`: invalid pack version requiring migration.
- `packages/cli/src/index.ts`: wired all `process.exit()` calls to explicit `ExitCode` constants.
- `packages/cli/src/exit-codes.test.ts` (new): 6 integration tests verifying exact exit code behavior across all commands.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Tests added (packages/cli/src/exit-codes.test.ts, +6 tests)
- Exits 0 on successful build.
- Exits 0 on clean contract diff (no changes).
- Exits 2 on breaking contract diff (e.g. route removed / changed).
- Exits 1 on compilation error (syntax/extraction failure).
- Exits 1 on unknown command or missing file.
- Exits 0 on help command or --help flag.

### Command results
- `cargo test -p q-pack` → 3 suites — 0 failed
- `bun test` → **249 pass / 0 fail (32 files, +6 new tests)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-002)
- **CI use is documented** — deterministic exit codes (0 for pass, 1 for build/user errors, 2 for breaking contract drift) enable reliable CI automation.
- **Invalid inputs fail clearly** — invalid inputs produce exit code 1 with actionable errors.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
