---
task_id: M25-006-A
parent_task: M25-006
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-006-A — Generate problem type/status/title/detail/custom-field encoders

## Atomic goal

Generate problem type/status/title/detail/custom-field encoders.

## Parent intent

Preserve typed domain and framework errors without generic placeholder shapes.

## Dependencies

- `M25-001-Z` — `tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md`
- `M25-005-Z` — `tasks/02_m25_schema_codecs/M25-005-Z-package-evidence-for-generate-status-specific-response-encoders.md`

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
5. Implement exactly this deliverable: Generate problem type/status/title/detail/custom-field encoders.
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
m25-006-a: generate problem type status title detail custom field encod
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-006-A)

Status: **PASS**. Problem responses encode through generated per-declaration
programs, and RFC 9457 custom fields survive end-to-end:

- **Generated problem encoders** (`crates/q-schema-runtime/src/encoder.rs`):
  `ProblemProgram` compiles a `SchemaIr::Problem` node — frozen declared
  status, optional declared type-URI override, required declared title,
  and the declared detail shape. `encode` emits the canonical envelope in
  one pass (type, title, status, instance, optional detail, errors, then
  sorted extension members), validating the detail string against the
  declared shape (typed field error on mismatch). `EncoderTable` carries
  problem programs alongside object programs, keyed by the same dense
  SchemaId; Problem IR nodes never compile as object encoders and vice
  versa. Unrepresentable detail shapes compile to `None` (generic builder
  covers the route).
- **Runtime wiring** (`crates/q-runtime/src/serve.rs` + `problems.rs`):
  when a route declares an explicit `s.problem(...)` schema for the
  settled status, the `Outcome::Problem` arm encodes through the
  generated program (declared title/type override the registry); a
  detail-shape violation settles as the same controlled
  `contract.violation.response` 500 as response-schema violations.
  Framework problems keep the generic registry builder (unchanged
  semantics). `problems::body` gained the extensions parameter at every
  call site.
- **Custom fields survive end-to-end**: the engine
  (`problem_from_object`) reads every own property beyond the standard
  envelope into `ProblemOut.extensions` (name-sorted; non-JSON values are
  skipped, never failing the problem), and both the generated program and
  the generic builder emit them after the standard members.
- **TS surface** (`packages/core`): `problem()` gained a `fields` option
  that spreads RFC 9457 extension members onto the returned problem
  value.

### Tests and evidence

- `problem_encoder_emits_canonical_envelope_with_extensions` — golden
  envelope: declared title/type override, status override honoring,
  detail, errors member, sorted extensions, canonical member order, and
  registry fallback for the type URI.
- `problem_encoder_validates_declared_detail_shape` — detail violating
  the declared bound rejects typed (`maxLength` at path `detail`);
  unrepresentable detail shapes compile to `None`.
- `encoder_table_separates_problem_and_object_programs` — dense-table
  routing of Problem vs Object IR nodes.
- `runtime_conformance::declared_problem_response_encodes_with_custom_fields`
  — live HTTP: a declared 409 problem carries the declared
  type/title/status/detail plus BOTH custom fields (`orderId`,
  `retryable`) in canonical member order; the undeclared twin keeps the
  generic registry envelope byte-identical.
- `cargo test -p q-schema-runtime` — 57 unit + 3 fuzz passed.
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p velqu-runtime` — 21 integration passed.
- `bun test` — 69 passed, 0 failed, 297 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Redaction hardening for UNEXPECTED failures is M25-006-B; Treaty union
narrowing is M25-006-C; content-type/instance behavior is M25-006-D.

Commit: `1ec6789`.
