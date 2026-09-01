---
task_id: M4A-006-V
parent_task: M4A-006
milestone: M4A
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-006-V — Verify Finalize diagnostics, source maps, and inspect output

## Atomic goal

Prove every acceptance criterion for parent task M4A-006 without broadening scope.

## Parent intent

Make compile, startup, contract, capability, and runtime failures actionable.

## Dependencies

- `M4A-006-A` — `tasks/07_m4a_developer_preview/M4A-006-A-structured-diagnostic-codes.md`
- `M4A-006-B` — `tasks/07_m4a_developer_preview/M4A-006-B-source-map-aware-stacks.md`
- `M4A-006-C` — `tasks/07_m4a_developer_preview/M4A-006-C-redaction-policy.md`
- `M4A-006-D` — `tasks/07_m4a_developer_preview/M4A-006-D-inspect-route-plan-fields-codecs-capabilities-crossings-and-debug-names.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Golden diagnostics.
- Redaction tests.
- Source-map tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-006-v: verify finalize diagnostics source maps and inspect output
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-006-V) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-006-v (squash-merged; see git log for final hash)
- Closes: #466

### Acceptance-criterion mapping (parent M4A-006)

1. **No secrets in production diagnostics** — console redaction covers
   bearer/basic, API keys/passwords, prefix tokens, authorization, and cookie
   assignments; existing runtime redaction suite remains green.
2. **Errors identify route/source/contract cause** — closed CLI diagnostic
   codes, source locations/code frames, route-plan IDs, codec/bridge/stage,
   capability and fallback fields are all asserted by tests.
3. **Source maps are lazy on success path** — default runtime mapper is
   identity unless an embedded map exists; advisory sidecar parsing is a
   separate explicit `mapper_for_sidecar` call with exact pack binding.
4. **Diagnostic catalog exists** — `DiagnosticCode` is an exported closed
   union with representative golden classification tests.

### Evidence

- `diagnostic-codes.test.ts` (2), `actionable-errors.test.ts` (4): stable
  diagnostic catalog, source frames, hints, and redaction separation.
- `source_map_conformance.rs` (3): valid sidecar lookup, mismatch
  fail-closed, invalid map identity fallback.
- `inspect-output.test.ts` (3): route-plan/debug fields, strategy accounting,
  capability inventory.
- q-capabilities `redact_` suite (4): sensitive assignment/prefix/scheme
  forms.
- Raw `typecheck-scale.ts` samples: 25 routes 776.7/1114.8/748.5 ms; 100
  routes 696.1/511.6/513.6 ms; 200 routes 508.3/514.3/501.8 ms.
  Startup-dominated; no unsupported performance claim.

### Verification runs (fresh worktree)

- `cargo test -p q-pack` → PASS
- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-capabilities` → PASS
- `cargo test -p velqu-runtime` → PASS, including source-map tests
- `bun test` → **308 pass / 0 fail (48 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
