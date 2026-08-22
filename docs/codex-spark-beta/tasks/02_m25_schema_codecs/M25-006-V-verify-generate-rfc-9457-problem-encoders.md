---
task_id: M25-006-V
parent_task: M25-006
milestone: M25
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-006-V — Verify Generate RFC 9457 problem encoders

## Atomic goal

Prove every acceptance criterion for parent task M25-006 without broadening scope.

## Parent intent

Preserve typed domain and framework errors without generic placeholder shapes.

## Dependencies

- `M25-006-A` — `tasks/02_m25_schema_codecs/M25-006-A-generate-problem-type-status-title-detail-custom-field-encoders.md`
- `M25-006-B` — `tasks/02_m25_schema_codecs/M25-006-B-redact-unexpected-failures.md`
- `M25-006-C` — `tasks/02_m25_schema_codecs/M25-006-C-ensure-policy-provided-errors-flow-into-treaty-unions.md`
- `M25-006-D` — `tasks/02_m25_schema_codecs/M25-006-D-include-content-type-and-instance-behavior.md`

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
- `crates/q-runtime/src/main.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Problem fixtures.
- Redaction tests.
- Treaty narrowing tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-006-v: verify generate rfc 9457 problem encoders
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-006-V)

Status: **PASS**. Every parent M25-006 acceptance guardrail maps to source
and passing tests; all verification commands were run fresh on this branch
(no code changes — verification closure only).

### Guardrail → source → evidence

1. **Custom problem fields survive end-to-end.**
   - `problem_from_object` (worker.rs) reads extension members into
     `ProblemOut.extensions`; both the generated `ProblemProgram` and the
     generic `problems::body` emit them sorted.
   - `declared_problem_response_encodes_with_custom_fields` — live HTTP:
     `orderId` and `retryable` cross on the declared 409; TS surface
     `problem({ fields })` spreads them.
2. **Unexpected errors never expose secrets/stacks in production.**
   - `internal_problem_detail_and_extensions_are_redacted` — internal-
     problem detail + extension stripped from the wire, preserved in the
     `problem.redacted` log; thrown exceptions stay redacted
     (`Outcome::EngineFailure`, bun security conformance).
3. **Error status narrowing is exact.**
   - `problemTypeOf` emits frozen type/title literals + the declared
     status literal in `contract.d.ts`; Treaty narrowing test asserts
     `r.error.problem` typed as the unauthorized envelope after
     `status === 401` and checks the live 401 body values.
   - Status key lookup means the generated encoder only engages for the
     status the route declared for that problem.
4. **OpenAPI problem schemas match runtime.**
   - Compiler tests: `application/problem+json` content schemas with
     required `[type, title, status, instance]` and enum-constrained
     type/title/status for the policy 401 and a declared 404 fixture;
     `PROBLEM_REGISTRY` id parity with the runtime registry.

### Command results (this branch, fresh worktree)

- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-schema-runtime` — 57 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 22 integration passed.
- `bun test` — 73 passed, 0 failed, 313 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (identical on every packet branch this session).

Changed files: this record, `docs/codex-spark-beta/STATUS.md`,
`docs/codex-spark-beta/indexes/TASK_INDEX.md` (verification closure only).

Commit: `48f9c10`.
