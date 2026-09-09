---
task_id: M4A-004-V
parent_task: M4A-004
milestone: M4A
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M4A.md
commit_required: true
---

# M4A-004-V — Verify Complete Treaty unit-local, runtime-local, and remote modes

## Atomic goal

Prove every acceptance criterion for parent task M4A-004 without broadening scope.

## Parent intent

Deliver Eden-quality type-safe clients and distinct test fidelity levels.

## Dependencies

- `M4A-004-A` — `tasks/07_m4a_developer_preview/M4A-004-A-unit-local-direct-generated-dispatcher.md`
- `M4A-004-B` — `tasks/07_m4a_developer_preview/M4A-004-B-runtime-local-actual-rust-quickjs-process.md`
- `M4A-004-C` — `tasks/07_m4a_developer_preview/M4A-004-C-remote-fetch-client.md`
- `M4A-004-D` — `tasks/07_m4a_developer_preview/M4A-004-D-exact-method-body-query-status-problem-typing.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M4A.md`
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
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No public `any`.
- 2xx data and non-2xx errors narrow correctly.
- Undeclared status is a contract error.
- All modes share the same contract.

## Targeted commands

```bash
cargo test -p velqu-runtime
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

- Negative type tests.
- Mode parity tests.
- Typecheck scale benchmark.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m4a-004-v: verify complete treaty unit local runtime local and remote m
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M4A-004-V) — PASS

- Date: 2026-09-01
- Branch/PR: m4a-004-v (squash-merged; see git log for final hash)
- Closes: #454

### Acceptance-criterion mapping (parent M4A-004)

1. **No public `any`** — `AnyRouteContract` uses
   `Record<number, unknown>`; all new mode adapters expose concrete
   options, handles, and outcome types. Existing internal proxy casts are
   implementation-only.
2. **2xx data and non-2xx errors narrow correctly** — negative type tests
   pin method/body/query/header/status/problem unions; exact-typing tests
   prove required headers and typed 400; direct/remote/runtime tests cover
   200/201 success, 400/404/422/401 errors, and status-0 network/abort.
3. **Undeclared status is a contract error** — direct dispatcher
   `UndeclaredStatusError` fails loud with route, status, and declared set;
   coverage remains green.
4. **All modes share one contract** — direct, loopback, remote, and
   runtime-local adapters return the same `TreatyClient<Api>`; runtime-local
   and Treaty conformance now load the emitted `contract.json` route table
   rather than a duplicate hand-written table.

### Evidence package

- `packages/testing/src/unit-direct.test.ts`: direct-vs-loopback parity,
  typed problems, undeclared status, and method mismatch (6 tests).
- `packages/testing/src/runtime-local.test.ts`: generated contract loading,
  actual Rust + QuickJS process, ready identity, typed routes, bounded drain,
  `service:2` profile (3 tests).
- `packages/testing/src/remote.test.ts`: remote HTTP success/error,
  abort/network classification, direct-vs-remote parity (4 tests).
- `packages/treaty/src/exact-typing.test.ts`: exact query/header forwarding
  and declared 400 problem (2 tests).
- `packages/treaty/src/types-negative.test-d.ts`: typecheck-only negative
  assertions for methods, body, query, headers, params, status, and problem.
- `scripts/typecheck-scale.ts`: raw 3-repetition measurements:
  25 routes 817.3/836.7/853.4 ms; 100 routes 1439.1/1012.3/1008.5 ms;
  200 routes 822.1/1259.2/734.7 ms. Startup-dominated; no unsupported
  performance claim.

### Verification runs (fresh worktree)

- `cargo test -p velqu-runtime` → 7 suites — 0 failed
- `bun test` → **292 pass / 0 fail (42 files)**
- `bun run typecheck` → clean
- `cargo fmt --check` clean; workspace clippy -D warnings → exit 0
- `./scripts/verify` → **ALL PASS**

### Disclosures

- Standing: CI fails with zero executed steps on every PR since ~#714
  (infrastructure-side); disclosed per PR.
