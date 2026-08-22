---
task_id: M25-006-D
parent_task: M25-006
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-006-D — Include content type and instance behavior

## Atomic goal

Include content type and instance behavior.

## Parent intent

Preserve typed domain and framework errors without generic placeholder shapes.

## Dependencies

- `M25-006-C` — `tasks/02_m25_schema_codecs/M25-006-C-ensure-policy-provided-errors-flow-into-treaty-unions.md`

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
5. Implement exactly this deliverable: Include content type and instance behavior.
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
m25-006-d: include content type and instance behavior
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-006-D)

Status: **PASS**. Problem responses now carry RFC 9457's problem media
type, and `instance` semantics are pinned by tests:

- `crates/q-runtime/src/serve.rs`: new `problem_response` helper — every
  problem emission (admission 400/413/415, validation 422 params/query/
  headers/body, deadline 504, not-found/method, quarantine/readiness 503,
  capacity 503, contract violations 500, engine failures 500, and BOTH
  `Outcome::Problem` paths incl. the generated encoder) now sends
  `Content-Type: application/problem+json`. Success responses keep
  `application/json` (asserted).
- `instance` keeps its frozen semantics — the request occurrence
  identifier `req-<start_ms>-<n>` — now asserted at runtime
  (`declared_problem_response_encodes_with_custom_fields` checks the
  `req-` prefix on the wire). No semantic change; the wire stays frozen
  per pack-format-v1.
- `packages/compiler/src/emit.ts`: OpenAPI problem responses now advertise
  the `application/problem+json` content key (guardrail: OpenAPI problem
  schemas match runtime); compiler tests updated to the same key.
- `docs/specs/pack-format-v1.md`: registry table gains the missing
  `overload` row; the problem-body paragraph now documents the
  `application/problem+json` content type and the `instance` semantics.

### Tests and evidence

- `runtime_conformance::declared_problem_response_encodes_with_custom_fields`
  — extended: declared 409 and generic 422 both assert
  `content-type: application/problem+json`; the declared problem's
  `instance` asserts the `req-` occurrence-id prefix.
- `js_fallback_body_routes_raw_json_to_handler` — extended: admission 422
  (malformed JSON) also asserts the problem media type.
- `native_response_encoder_emits_declared_order` — extended: the 200
  success response still carries `application/json`.
- Compiler conformance: OpenAPI problem schemas read from the
  `application/problem+json` content key (both the 401 policy case and
  the 404 fixture).
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

Commit: `2116474`.
