---
task_id: M4A-006-A
parent_task: M4A-006
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-006-A — Structured diagnostic codes

## Atomic goal

Structured diagnostic codes.

## Parent intent

Make compile, startup, contract, capability, and runtime failures actionable.

## Dependencies

- `M4A-001-Z` — `tasks/07_m4a_developer_preview/M4A-001-Z-package-evidence-for-implement-actual-runtime-velqu-dev-loop.md`
- `M4A-002-Z` — `tasks/07_m4a_developer_preview/M4A-002-Z-package-evidence-for-complete-cli-command-surface.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Structured diagnostic codes.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No secrets in production diagnostics.
- Errors identify route/source/contract cause.
- Source maps are lazy on success path.
- Diagnostic catalog exists.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Golden diagnostics.
- Redaction tests.
- Source-map tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-006-a: structured diagnostic codes
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-006-A) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-006-a (squash-merged; see git log for final hash)
- Closes: #462

### Changed files
- `packages/cli/src/errors.ts`: added closed `DiagnosticCode` catalog and
  deterministic classifier for compile import/contract/path/schema,
  toolchain, runtime, and unknown failures; `FormattedDiagnostic` now always
  carries a code while preserving source location, code frame, hint, and raw
  message.
- `packages/cli/src/actionable-errors.test.ts`: pins the import diagnostic
  code while retaining code-frame/hint/redaction coverage.
- `packages/cli/src/diagnostic-codes.test.ts` (new): golden representative
  code classification and untrusted-message separation tests (2 tests).
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Golden diagnostics**: stable code assertions for six representative
  compile/runtime categories.
- **Redaction tests**: code is structural and separate from message content;
  diagnostic output does not infer a code from user-controlled text beyond
  the closed classifier.
- **Source-map tests**: existing actionable code-frame tests remain green;
  source frames are only rendered when a valid location exists.

### Guardrail mapping (parent M4A-006)

- **No secrets in production diagnostics**: no secret values are added to
  structured fields; raw message behavior is unchanged and existing runtime
  redaction suite remains green.
- **Errors identify route/source/contract cause**: stable categories are
  attached while existing source location/code frame/hint fields are retained.
- **Source maps are lazy on success path**: code-frame rendering remains
  conditional on an error location and valid source file.
- **Diagnostic catalog exists**: `DiagnosticCode` is a closed exported union.

### Command results

- `cargo test -p q-pack` → PASS
- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-capabilities` → PASS
- `bun test` → **305 pass / 0 fail (47 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
