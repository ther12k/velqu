---
task_id: M25-006-Z
parent_task: M25-006
milestone: M25
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-006-Z — Package evidence for Generate RFC 9457 problem encoders

## Atomic goal

Create source-backed evidence and handoff for parent task M25-006; update status only if verification passed.

## Parent intent

Preserve typed domain and framework errors without generic placeholder shapes.

## Dependencies

- `M25-006-V` — `tasks/02_m25_schema_codecs/M25-006-V-verify-generate-rfc-9457-problem-encoders.md`

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
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-006-z: package evidence for generate rfc 9457 problem encoders
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M25-006-V merged in PR #748 at
  commit `7770310b150cbf02a72a275e3a2f53fbe10e3caf`; issue #154 is closed.
  The evidence package is based on clean parent HEAD `d5dbe21` before this
  commit.
- Parent acceptance matrix: `M25-006-V` maps all four guardrails to source
  and named tests:
  1. Custom problem fields survive end-to-end: `problem_from_object`
     extension reading, `ProblemProgram`/`problems::body` sorted emission,
     live-HTTP `declared_problem_response_encodes_with_custom_fields`,
     `problem({ fields })` TS surface.
  2. Unexpected errors never expose secrets/stacks:
     `internal_problem_detail_and_extensions_are_redacted` (wire stripped,
     `problem.redacted` log), `Outcome::EngineFailure` redaction + bun
     security conformance.
  3. Error status narrowing is exact: frozen literals + declared status
     literal in `contract.d.ts`; Treaty 401 narrowing test against the
     live body.
  4. OpenAPI problem schemas match runtime:
     `application/problem+json` schemas with enum-constrained
     type/title/status (compiler tests, policy 401 + declared 404
     fixture); `PROBLEM_REGISTRY` id parity.
- Source-backed implementation records:
  - `M25-006-A` (PR #744, #150 closed): generated problem encoders
    (`ProblemProgram` frozen status/title/type + detail-shape validation;
    runtime wiring for declared problem responses; extension members
    carried end-to-end).
  - `M25-006-B` (PR #745, #151 closed): unexpected failures redacted —
    internal-problem detail/extensions stripped from the wire, preserved
    in the internal log.
  - `M25-006-C` (PR #746, #152 closed): policy-provided errors flow into
    Treaty unions with exact narrowing; OpenAPI problem schemas match the
    runtime envelope.
  - `M25-006-D` (PR #747, #153 closed): `Content-Type:
    application/problem+json` on every problem emission; `instance`
    occurrence-id semantics pinned by tests; spec updated.
- CPU/allocation evidence: no new measurement; the problem encoding path
  is native host code and no performance claim is asserted. The benchmark
  manifest is preserved unchanged.
- Exact verification (fresh on this branch): `cargo test -p q-schema-runtime`
  (57 unit + 3 fuzz pass); `cargo test -p velqu-runtime` (22 pass);
  `cargo test -p q-engine-quickjs` (1 + 96 pass); `bun test` (73 passed,
  0 failed, 313 expect calls); `bun run typecheck` clean; `cargo fmt
  --check` clean; `cargo clippy --workspace --all-targets -- -D warnings`
  clean; `scripts/validate-okf` (176 links, 0 errors).
- Full `./scripts/verify` completed all Rust, typecheck, proof-build, and
  TypeScript stages. Its final benchmark check reports only the known
  isolated-worktree hash mismatches for `qRuntimeRelease` and `proofPack`
  against `benchmarks/manifest.json`. The canonical root manifest and
  historical raw benchmarks were preserved.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M25-006 PASS;
  the beta checklist and task index mark this Z packet PASS. The generated
  Spark queues now expose M25-007-A (#156) as the next dependency-ready
  packet.
- Remaining scope: `M25-007`–`M25-010` and `M25-GATE` remain TODO until
  implemented and evidenced.

Commit: `645aded`.
