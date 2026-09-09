---
task_id: M4A-002-D
parent_task: M4A-002
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-002-D — Helpful actionable errors

## Atomic goal

Helpful actionable errors.

## Parent intent

Provide consistent dev/build/inspect/contract/test/package workflows.

## Dependencies

- `M4A-002-C` — `tasks/07_m4a_developer_preview/M4A-002-C-machine-readable-output-option.md`

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
5. Implement exactly this deliverable: Helpful actionable errors.
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
m4a-002-d: helpful actionable errors
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-002-D) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-002-d (squash-merged; see git log for final hash)
- Closes: #441

### Changed files
- `packages/cli/src/errors.ts` (new): actionable diagnostic error formatting
  (`formatActionableError`, `renderCodeFrame`, `FormattedDiagnostic`) —
  - Source-located code frames with line numbers, context lines, and caret
    pointing to column.
  - Formats `CompileError`, TypeScript syntax errors, and toolchain errors
    with actionable hints (e.g. ADR-0003 QuickJS import guidance).
  - Integrates with `--json` mode to emit structured error objects
    `{ status: "error", error, location, hint }`.
- `packages/cli/src/index.ts`: wired `formatActionableError` across all CLI
  command error handlers (`build`, `check`, `inspect`).
- `packages/cli/src/actionable-errors.test.ts` (new): 4 integration tests
  verifying code frame generation, diagnostic hints, and CLI error output.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Tests added (packages/cli/src/actionable-errors.test.ts, +4 tests)
- Renders clean code frame with line number, context lines, and caret pointing to column.
- Formats compile errors with actionable diagnostics, code frames, and hints.
- CLI build surfaces actionable error frame on unsupported import.
- CLI check surfaces actionable error frame on invalid route structure.

### Command results
- `cargo test -p q-pack` → 3 suites — 0 failed
- `bun test` → **259 pass / 0 fail (34 files, +4 new tests)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-002 — complete)
- **Invalid inputs fail clearly** — source-located code frames and actionable hints guide developers to fix syntax and declaration errors.
- **Commands work in clean checkout** — verified in clean worktree.
- **CI use is documented** — errors produce structured JSON in `--json` mode.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
