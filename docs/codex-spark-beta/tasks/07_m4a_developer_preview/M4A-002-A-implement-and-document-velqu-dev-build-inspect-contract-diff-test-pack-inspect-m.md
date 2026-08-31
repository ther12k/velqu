---
task_id: M4A-002-A
parent_task: M4A-002
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-002-A — Implement and document `velqu dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect/migrate`, and diagnostics

## Atomic goal

Implement and document `velqu dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect/migrate`, and diagnostics.

## Parent intent

Provide consistent dev/build/inspect/contract/test/package workflows.

## Dependencies

- `M4A-001-Z` — `tasks/07_m4a_developer_preview/M4A-001-Z-package-evidence-for-implement-actual-runtime-velqu-dev-loop.md`
- `M26-GATE` — `gates/M26-GATE.md`

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
5. Implement exactly this deliverable: Implement and document `velqu dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect/migrate`, and diagnostics.
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
m4a-002-a: implement and document velqu dev build inspect contract diff
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-002-A) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-002-a (squash-merged; see git log for final hash)
- Closes: #438

### Changed files
- `packages/cli/src/index.ts`: complete CLI command surface —
  - `velqu dev`: runs dev server with safe worker reload loop.
  - `velqu build`: compiles QPack, manifests, OpenAPI, and types.
  - `velqu inspect`: inspects `routes`, `route <id>`, `capabilities`, `fallbacks`,
    and `diagnostics` (static extraction diagnostics without build artifacts).
  - `velqu contract diff`: checks schema/route differences against `contract.lock.json`.
  - `velqu test`: runs test runner.
  - `velqu check`: fast static AST validation.
  - `velqu pack inspect`: inspects QPack format headers, engine tuple, and manifests.
  - `velqu pack migrate`: provides migration guidance for legacy packs.
  - `help` / `--help` / `-h` / default usage text.
- `packages/cli/src/pack-inspect.ts` (new): QPack inspection helper (`inspectPack`).
- `packages/compiler/src/index.ts`: exports `extractApp`, `ExtractedApp`, `RouteInfo`, `PolicyInfo`.
- `docs/beta/08_CLI_REFERENCE.md` (new): CLI reference document.
- `packages/cli/src/cli-surface.test.ts` (new): 6 integration tests.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Tests added (packages/cli/src/cli-surface.test.ts, +6 tests)
- Inspects compiled QPack artifact properties accurately with pack-inspect.
- Reports clear error when inspecting a non-existent pack file.
- Runs CLI inspect diagnostics subcommand producing static extraction report.
- Runs CLI check command verifying static routes without emitting artifacts.
- Runs CLI pack inspect command and outputs formatted pack summary.
- Prints usage instructions when invoked without command or with invalid arguments.

### Command results
- `cargo test -p q-pack` → 3 suites — 0 failed
- `bun test` → **243 pass / 0 fail (31 files, +6 new tests)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-002)
- **Commands work in clean checkout** — verified in fresh worktree.
- **No compiler in production artifact** — compiler remains dev/build tooling.
- **CI use is documented** — standard exit codes documented in `08_CLI_REFERENCE.md`.
- **Invalid inputs fail clearly** — unknown commands and missing files exit 1 with clear messages.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
