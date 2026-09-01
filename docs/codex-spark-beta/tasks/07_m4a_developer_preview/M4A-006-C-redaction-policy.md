---
task_id: M4A-006-C
parent_task: M4A-006
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-006-C — Redaction policy

## Atomic goal

Redaction policy.

## Parent intent

Make compile, startup, contract, capability, and runtime failures actionable.

## Dependencies

- `M4A-006-B` — `tasks/07_m4a_developer_preview/M4A-006-B-source-map-aware-stacks.md`

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
5. Implement exactly this deliverable: Redaction policy.
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
m4a-006-c: redaction policy
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-006-C) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-006-c (squash-merged; see git log for final hash)
- Closes: #464

### Changed files
- `crates/q-capabilities/src/console.rs`: expanded the closed sensitive-value
  redaction vocabulary to include `authorization` and `cookie` assignment
  forms, including Bearer/Basic scheme-plus-credential values; no raw
  credentials cross the console sink.
- Existing console redaction tests plus a new header/cookie regression test
  prove the boundary; `benchmarks/manifest.json` refreshed.

### Required evidence

- **Golden diagnostics**: CLI structured diagnostic tests remain green.
- **Redaction tests**: `redact_` suite passes all four tests, including
  Bearer/Basic, API key/password, prefix-token, and header/cookie assignment
  forms.
- **Source-map tests**: inherited M4A-006-B source-map conformance remains
  green; this packet does not alter source-map loading.

### Guardrail mapping (parent M4A-006)

- **No secrets in production diagnostics**: authorization and cookie values
  are replaced by `[REDACTED]` before records enter the sink.
- **Errors identify route/source/contract cause**: no diagnostic routing or
  source location behavior is changed.
- **Source maps are lazy on success path**: no source-map code is touched.
- **Diagnostic catalog exists**: M4A-006-A closed `DiagnosticCode` catalog
  remains unchanged and tested.

### Command results

- `cargo test -p q-capabilities` → PASS (redaction suite: 4/4)
- `cargo test -p q-pack` → PASS
- `cargo test -p q-engine-quickjs` → PASS
- `bun test` → **305 pass / 0 fail (47 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
