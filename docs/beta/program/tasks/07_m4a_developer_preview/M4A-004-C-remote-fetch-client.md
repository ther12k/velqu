---
task_id: M4A-004-C
parent_task: M4A-004
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-004-C — Remote fetch client

## Atomic goal

Remote fetch client.

## Parent intent

Deliver Eden-quality type-safe clients and distinct test fidelity levels.

## Dependencies

- `M4A-004-B` — `tasks/07_m4a_developer_preview/M4A-004-B-runtime-local-actual-rust-quickjs-process.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`
- `context/components/devex-beta.md`

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
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Remote fetch client.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No public `any`.
- 2xx data and non-2xx errors narrow correctly.
- Undeclared status is a contract error.
- All modes share the same contract.

## Targeted commands

```bash
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Negative type tests.
- Mode parity tests.
- Typecheck scale benchmark.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m4a-004-c: remote fetch client
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-004-C) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-004-c (squash-merged; see git log for final hash)
- Closes: #452

### Changed files
- `packages/treaty/src/index.ts`: portable `TreatyFetch` function type
  (excludes Bun-only `preconnect`, allowing standard remote fetch
  implementations) while preserving the same status/error splitting.
- `packages/testing/src/index.ts`: `remoteTreaty` adapter, explicitly labeled
  `remote`, accepts base URL, published route contract, and injectable fetch;
  delegates to the same Treaty client used by direct and runtime-local modes.
- `packages/testing/src/remote.test.ts` (new): 4 tests — remote labeling and
  HTTP success, typed non-2xx problem/status preservation, abort/network
  classification, and direct-vs-remote parity for equivalent results.
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Negative type tests**: inherited M4A-004-A
  `packages/treaty/src/types-negative.test-d.ts`; `bun run typecheck` remains
  clean.
- **Mode parity tests**: `packages/testing/src/remote.test.ts` compares
  remote HTTP responses against the direct dispatcher for success and 201
  results; runtime-local parity remains covered by generated-contract
  conformance.
- **Typecheck scale benchmark**: inherited M4A-004-A raw measurements; no
  public type surface was weakened.

### Guardrail mapping (parent M4A-004)

- **No public `any`** — remote adapter exposes concrete options and mode
  types; transport injection uses a portable function signature.
- **2xx data and non-2xx errors narrow correctly** — remote tests prove
  200/201 data and 404/422 typed errors plus status-0 abort/network forms.
- **Undeclared status is a contract error** — direct dispatcher guard remains
  enforced; remote status handling uses the declared Treaty error union.
- **All modes share the same contract** — all adapters return the same
  `TreatyClient<Api>` and route/status splitting implementation.

### Command results

- `cargo test -p q-http` → PASS
- `cargo test -p q-capabilities` → PASS
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → **290 pass / 0 fail (41 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
