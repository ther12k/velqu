---
task_id: M25-006-C
parent_task: M25-006
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-006-C — Ensure policy-provided errors flow into Treaty unions

## Atomic goal

Ensure policy-provided errors flow into Treaty unions.

## Parent intent

Preserve typed domain and framework errors without generic placeholder shapes.

## Dependencies

- `M25-006-B` — `tasks/02_m25_schema_codecs/M25-006-B-redact-unexpected-failures.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Ensure policy-provided errors flow into Treaty unions.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Custom problem fields survive end-to-end.
- Unexpected errors never expose secrets/stacks in production.
- Error status narrowing is exact.
- OpenAPI problem schemas match runtime.

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

- Problem fixtures.
- Redaction tests.
- Treaty narrowing tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-006-c: ensure policy provided errors flow into treaty unions
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-006-C)

Status: **PASS**. Policy-provided errors flow into Treaty unions with exact
narrowing, and OpenAPI problem schemas match the runtime envelope:

- `packages/compiler/src/emit.ts`: a new exported `PROBLEM_REGISTRY`
  mirrors the runtime's frozen URNs (ids asserted by conformance).
  `problemTypeOf(problemId, status)` now emits the EXACT runtime envelope
  per registry id — frozen type-URI and title literals, the DECLARED
  response status as a literal, optional `detail`, and the typed optional
  `errors` member for validation — replacing the `{ title: string }`
  placeholder in `contract.d.ts`. Explicit `s.problem(...)` response IRs
  type as the full envelope too (declared literals via `tsTypeOfIr`).
- `openapiFor`: problem responses now carry a JSON Schema of the same
  envelope — required `[type, title, status, instance]`, enum-constrained
  type/title/status — instead of `content: undefined`.
- Evidence (guardrail "error status narrowing is exact"): the Treaty
  conformance type now narrows `if (r.error.status === 401)` to
  `problem: { type: "https://velqu.dev/problems/unauthorized"; ...;
  status: 401; ... }`, asserted against the LIVE runtime 401 body
  (type/title/status/instance values checked at runtime-local HTTP).
- Evidence (guardrail "OpenAPI problem schemas match runtime"): compiler
  conformance asserts the d.ts literals and the OpenAPI schema enums/
  required members for the policy 401 (proof app) and a declared 404
  (`problem-app` fixture).

### Tests and evidence

- `problem contracts (M25-006-C)` compiler suite (4 tests): exact d.ts
  envelopes (policy 401 + declared 404 fixture), OpenAPI schema parity,
  `PROBLEM_REGISTRY` id parity with the runtime registry.
- `treaty.conformance.test.ts`: `ProofPublishedApi` corrected to the real
  published contract (users.get 200+401; the previously aspirational
  404/422 entries removed) with the exact unauthorized envelope; test 5
  now narrows on `status === 401` and asserts the typed problem members
  against the live 401 response.
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-schema-runtime` — 57 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 22 integration passed.
- `bun test` — 73 passed, 0 failed, 313 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Redaction tests were delivered in M25-006-B; content-type/instance
behavior is M25-006-D.

Commit: `0e6b5e7`.
