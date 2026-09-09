---
task_id: M4A-004-D
parent_task: M4A-004
milestone: M4A
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-004-D — Exact method/body/query/status/problem typing

## Atomic goal

Exact method/body/query/status/problem typing.

## Parent intent

Deliver Eden-quality type-safe clients and distinct test fidelity levels.

## Dependencies

- `M4A-004-C` — `tasks/07_m4a_developer_preview/M4A-004-C-remote-fetch-client.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `packages/cli/src/index.ts`
- `packages/compiler/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Exact method/body/query/status/problem typing.
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
cargo test -p q-http
```
```bash
cargo test -p q-bridge
```
```bash
cargo test -p q-schema-runtime
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
m4a-004-d: exact method body query status problem typing
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-004-D) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-004-d (squash-merged; see git log for final hash)
- Closes: #453

### Changed files
- `packages/contract/src/index.ts`: added a typed `headers` generic and
  property to `RouteContract` (defaulting to the legacy open string map),
  preserving existing six-argument contracts while enabling exact generated
  header shapes.
- `packages/treaty/src/index.ts`: exact `RequestOptions<Q, H>` with required
  query/header keys enforced and excess keys rejected; `HeadersOf` flows the
  contract header shape through every method; portable `TreatyFetch` was
  retained; no public `any` in `AnyRouteContract.responses`.
- `packages/treaty/src/exact-typing.test.ts` (new): 2 tests proving exact
  query/header forwarding, required auth header, and declared 400 problem
  response behavior.
- `packages/treaty/src/types-negative.test-d.ts`: compile-time rejects for
  unsupported methods, missing/wrong POST bodies, GET bodies, unknown query
  fields, missing/unknown headers, missing/misnamed path params, and
  impossible 200 error status; exact status/problem narrowing pins.
- `packages/testing/src/unit-direct.test.ts`: adjusted problem assertions to
  reflect the typed standard error union.
- `benchmarks/manifest.json`: refreshed.

### Required evidence

- **Negative type tests**: `types-negative.test-d.ts` — `bun run typecheck`
  passes with every `@ts-expect-error` still consuming a real error.
- **Mode parity tests**: inherited direct/loopback and remote/runtime tests;
  all four modes consume the same `TreatyResult` contract and status split.
- **Typecheck scale benchmark** (`bun scripts/typecheck-scale.ts`, 3 reps):
  25 routes 1034.8/1341.4/1009.8 ms; 100 routes 969.6/1064.7/1563.3 ms;
  200 routes 1062.8/979.0/1087.4 ms. Startup-dominated measurements only;
  no unsupported performance claim.

### Guardrail mapping (parent M4A-004)

- **No public `any`** — `AnyRouteContract.responses` now uses
  `Record<number, unknown>` and all request/contract additions are concrete.
- **2xx data and non-2xx errors narrow correctly** — exact status and problem
  narrowing tests cover 200 data, 400/404 typed problems, and status-0
  network/abort forms.
- **Undeclared status is a contract error** — direct dispatcher
  `UndeclaredStatusError` coverage remains green; response statuses are
  contract-keyed in the Treaty union.
- **All modes share the same contract** — query/header/body/response types
  are consumed by the same `TreatyClient<Api>` shape across unit-local
  direct, unit-local loopback, runtime-local, and remote adapters.

### Command results

- `cargo test -p q-engine-quickjs` → PASS
- `cargo test -p q-http` → PASS
- `cargo test -p q-bridge` → PASS
- `cargo test -p q-schema-runtime` → PASS
- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → **290 pass / 0 fail (41 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
