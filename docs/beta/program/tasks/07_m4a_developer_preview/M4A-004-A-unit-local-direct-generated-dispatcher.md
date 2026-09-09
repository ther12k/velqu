---
task_id: M4A-004-A
parent_task: M4A-004
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-004-A — Unit-local direct generated dispatcher

## Atomic goal

Unit-local direct generated dispatcher.

## Parent intent

Deliver Eden-quality type-safe clients and distinct test fidelity levels.

## Dependencies

- `M25-GATE` — `gates/M25-GATE.md`
- `M4A-001-Z` — `tasks/07_m4a_developer_preview/M4A-001-Z-package-evidence-for-implement-actual-runtime-velqu-dev-loop.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
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
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Unit-local direct generated dispatcher.
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
cargo test -p q-engine-quickjs
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
m4a-004-a: unit local direct generated dispatcher
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-004-A) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-004-a (squash-merged; see git log for final hash)
- Closes: #450

### Changed files
- `packages/treaty/src/index.ts`: added a direct-dispatch transport beside
  HTTP fetch — `DispatchRequest` / `DispatchOutcome` / `DispatchImpl` types
  and `TreatyOptions.dispatchImpl`. When set, every invocation routes to the
  in-process dispatcher (NO HTTP); dispatcher throws propagate as contract
  errors (fail loud, never masquerade as network failures). Status
  splitting for both transports goes through the SAME contract machinery.
- `packages/testing/src/index.ts`: `unitTreatyDirect` — the unit-local
  DIRECT mode. Builds the dispatcher from declared routes (path, method,
  declared `responses` status set, handler), interprets handler results
  (`__problem` / `status().value()` / plain → 200), enforces the
  "undeclared status is a contract error" guardrail via
  `UndeclaredStatusError` (names route, offending status, declared set),
  and returns the standard `TreatyClient<Api>` labeled
  `unit-local (direct dispatcher, NOT runtime conformance)`.
- `packages/testing/src/unit-direct.test.ts` (new): 6 tests — labeling,
  dot-navigation/apply-form/POST driving, 2xx-vs-non-2xx splitting with
  typed 404 problems, undeclared-status contract error, method-mismatch
  fail-loud, and full mode parity (direct vs loopback return identical
  results).
- `packages/treaty/src/types-negative.test-d.ts` (new): compile-time
  negative tests — 9 `@ts-expect-error` assertions (method narrowing both
  directions, missing/short/wrong-typed POST bodies, GET-with-body,
  missing/misnamed path params, 200 never in the error union, error
  problem is the declared 404 shape, unknown route segment) plus
  `expectTypeOf` pins for the exact data/error unions.
- `scripts/typecheck-scale.ts` (new): typecheck scale benchmark —
  generates an N-route synthetic Api + full-navigation consumer and times
  `tsc --noEmit` (3 repetitions per size, raw samples printed).
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Negative type tests**: `packages/treaty/src/types-negative.test-d.ts`
  (typecheck-only; every directive proves a real rejection —
  `bun run typecheck` fails if any line stops erroring).
- **Mode parity tests**: `unit-direct.test.ts` "mode parity: direct
  dispatcher and loopback unit-local return identical results" (2xx data,
  201 create, and 404 problem flows identical across both unit-local
  transports); runtime-local parity is pinned by the existing
  `conformance/treaty` suite driving the actual binary.
- **Typecheck scale benchmark** (`bun scripts/typecheck-scale.ts`,
  3 reps per size, raw samples):
  - 25 routes: 1768.9 / 1852.4 / 1750.5 ms
  - 100 routes: 1411.8 / 1601.7 / 1763.2 ms
  - 200 routes: 1405.5 / 1723.8 / 3173.9 ms
  Observation (same host, same run): typechecking the generated surface
  stays in the ~1.4–3.2 s range up to 200 routes with sub-linear growth;
  samples are startup-dominated (no warming pass applied). No global
  performance claim is made.

### Guardrail mapping (parent M4A-004)

- **No public `any`** — the treaty public surface exports only concrete
  types; no new `any` was added (existing internal proxies unchanged).
- **2xx data and non-2xx errors narrow correctly** — proven by typed 404
  problem assertions in unit-direct.test.ts and the union pins in
  types-negative.test-d.ts.
- **Undeclared status is a contract error** — `UndeclaredStatusError`
  fails loud in the direct dispatcher (tested); the runtime itself fails
  closed on undeclared statuses (existing native-encoder conformance).
- **All modes share the same contract** — the direct dispatcher returns
  the same `TreatyClient<Api>` and uses the same status-splitting code
  path as HTTP transports; parity test proves identical results.

### Command results

- `cargo test -p q-engine-quickjs` → all suites — 0 failed
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → **283 pass / 0 fail (39 files, +6 new tests)**
- `bun run typecheck` → clean (includes the new negative type tests)
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
