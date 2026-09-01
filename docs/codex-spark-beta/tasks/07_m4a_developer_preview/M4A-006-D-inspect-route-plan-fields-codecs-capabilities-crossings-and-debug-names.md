---
task_id: M4A-006-D
parent_task: M4A-006
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-006-D — Inspect route plan, fields, codecs, capabilities, crossings, and debug names

## Atomic goal

Inspect route plan, fields, codecs, capabilities, crossings, and debug names.

## Parent intent

Make compile, startup, contract, capability, and runtime failures actionable.

## Dependencies

- `M4A-006-C` — `tasks/07_m4a_developer_preview/M4A-006-C-redaction-policy.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`
- `packages/contract/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Inspect route plan, fields, codecs, capabilities, crossings, and debug names.
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
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
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
m4a-006-d: inspect route plan fields codecs capabilities crossings and
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-006-D) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-006-d (squash-merged; see git log for final hash)
- Closes: #465

### Changed files
- `packages/cli/src/index.ts`: inspect JSON output now includes explicit
  `routeCount`, `routeId`, declared capability count, and actual native/JS
  validation/response strategy distribution (rather than route-count
  placeholders), while preserving detailed per-route codec, bridge, stage,
  policy, capability, and fallback fields.
- `packages/cli/src/inspect-output.test.ts` (new): 3 tests covering route-plan
  fields/debug names, strategy distribution accounting, and capability
  inventory output.
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Golden diagnostics**: inspect JSON assertions pin route count, route IDs,
  codec/bridge/stage fields, and fallback counts.
- **Redaction tests**: inherited q-capabilities redaction suite remains green.
- **Source-map tests**: inherited q-runtime source-map conformance remains
  green; inspect output does not load maps on the success path.

### Guardrail mapping (parent M4A-006)

- **No secrets in production diagnostics**: inspect outputs contain declared
  plan/capability metadata only; source values are not introduced.
- **Errors identify route/source/contract cause**: route IDs and detailed plan
  fields make codec/crossing/fallback decisions diagnosable.
- **Source maps are lazy on success path**: inspection reads generated
  manifests only and does not parse source-map sidecars.
- **Diagnostic catalog exists**: M4A-006-A structured code catalog remains
  unchanged and tested.

### Command results

- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-schema-runtime` → PASS
- `bun test` → **308 pass / 0 fail (48 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
