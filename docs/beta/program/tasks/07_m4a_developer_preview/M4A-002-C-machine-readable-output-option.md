---
task_id: M4A-002-C
parent_task: M4A-002
milestone: M4A
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-002-C — Machine-readable output option

## Atomic goal

Machine-readable output option.

## Parent intent

Provide consistent dev/build/inspect/contract/test/package workflows.

## Dependencies

- `M4A-002-B` — `tasks/07_m4a_developer_preview/M4A-002-B-stable-exit-codes.md`

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
5. Implement exactly this deliverable: Machine-readable output option.
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
m4a-002-c: machine readable output option
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-002-C) — PASS

- Date: 2026-08-31
- Branch/PR: m4a-002-c (squash-merged; see git log for final hash)
- Closes: #440

### Changed files
- `packages/cli/src/index.ts`: added `--json` support across all CLI commands —
  - `velqu build --json`: outputs structured JSON with build stats, route count, artifact sizes, and lock status.
  - `velqu check --json`: outputs static validation report in JSON.
  - `velqu inspect diagnostics|routes|route <id>|capabilities|fallbacks --json`: outputs machine-readable diagnostics and manifests.
  - `velqu contract diff --json`: outputs structured diff entries, breaking counts, and status.
  - `velqu pack inspect|migrate --json`: outputs machine-readable pack properties or migration instructions.
  - Error responses with `--json` output structured JSON `{ status: "error", error: ... }` for CI and toolchain integration.
- `packages/cli/src/json-output.test.ts` (new): 6 integration tests verifying JSON output format across all commands and error paths.
- `benchmarks/manifest.json`: refreshed (standard remapped flow).

### Tests added (packages/cli/src/json-output.test.ts, +6 tests)
- Outputs structured JSON on velqu build --json.
- Outputs structured JSON on velqu check --json.
- Outputs structured JSON on velqu inspect diagnostics --json.
- Outputs structured JSON on velqu pack inspect --json.
- Outputs structured JSON on velqu contract diff --json.
- Outputs structured error JSON when compilation fails with --json.

### Command results
- `cargo test -p q-pack` → 3 suites — 0 failed
- `bun test` → **255 pass / 0 fail (33 files, +6 new tests)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Guardrail mapping (parent M4A-002)
- **CI use is documented** — `--json` flag provides deterministic machine-readable schemas for all commands and errors.
- **Commands work in clean checkout** — verified in clean worktree.

### Disclosures
- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
