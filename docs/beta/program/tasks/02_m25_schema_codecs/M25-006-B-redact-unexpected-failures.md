---
task_id: M25-006-B
parent_task: M25-006
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-006-B — Redact unexpected failures

## Atomic goal

Redact unexpected failures.

## Parent intent

Preserve typed domain and framework errors without generic placeholder shapes.

## Dependencies

- `M25-006-A` — `tasks/02_m25_schema_codecs/M25-006-A-generate-problem-type-status-title-detail-custom-field-encoders.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Redact unexpected failures.
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
m25-006-b: redact unexpected failures
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-006-B)

Status: **PASS**. Unexpected failures never expose secrets or stacks:

- `crates/q-runtime/src/serve.rs` (`Outcome::Problem` arm): problems
  settling as the framework's `internal` problem (including unknown
  `custom:` ids that the registry resolves to internal) are treated as
  UNEXPECTED failures — their `detail` and RFC 9457 extension members may
  carry exception text, stacks, or secrets, so they are stripped from the
  wire BEFORE either the generated problem encoder (M25-006-A) or the
  generic builder runs, and preserved only in the internal
  `problem.redacted` log. Declared registry problems (validation,
  not-found, ...) keep their detail by design; thrown exceptions were
  already redacted through `Outcome::EngineFailure` (RUN-007, unchanged).
- Evidence: `runtime_conformance::internal_problem_detail_and_extensions_
  are_redacted` — a handler-settled internal problem carrying a
  stack-like secret detail and an `apiKey` extension member produces a 500
  whose body has NO detail member, NO extension member, and none of the
  secret substrings; the internal log records the full redacted payload
  via `problem.redacted`; a declared `validation` problem on the same
  pack keeps its detail.

### Tests and evidence

- `runtime_conformance::internal_problem_detail_and_extensions_are_redacted`
  (problem fixture + redaction assertions, live HTTP).
- Existing redaction coverage stays green: bun security conformance
  "unexpected errors are redacted from responses" (thrown exceptions),
  `throw.redacted` fixture, `response_schema_violation_is_a_controlled_500`
  (violation detail never leaks).
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-schema-runtime` — 57 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 22 integration passed (new redaction
  test).
- `bun test` — 69 passed, 0 failed, 297 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Treaty narrowing tests belong to M25-006-C; content-type/instance
behavior to M25-006-D.

Commit: `c41f611`.
