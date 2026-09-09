---
task_id: M25-005-B
parent_task: M25-005
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-005-B — Read declared properties in fixed order

## Atomic goal

Read declared properties in fixed order.

## Parent intent

Fuse output validation and serialization for stable response contracts.

## Dependencies

- `M25-005-A` — `tasks/02_m25_schema_codecs/M25-005-A-generate-per-status-encoders.md`

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
5. Implement exactly this deliverable: Read declared properties in fixed order.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Undeclared status/body remains a contract violation.
- Output is JSON-equivalent to reference serialization.
- One traversal for generated paths.
- No user JS escapes deadline ownership during conversion.

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

- Golden JSON corpus.
- Response mismatch tests.
- Mapping deadline tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-005-b: read declared properties in fixed order
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-005-B)

Status: **PASS**. The encoder program now reads declared properties in a
compile-time-FIXED order, with determinism evidence:

- `crates/q-schema-runtime/src/encoder.rs`: `EncoderProgram` stores the
  property table as a frozen `Vec<PropertyEncoder>` (byte-sorted at
  compile) — every response reads exactly these properties in exactly
  this order regardless of the handler value's key insertion order. Each
  `PropertyEncoder` carries its `Optional` default hoisted at compile
  time (no per-response match on the spec shape) and unknown-key
  detection binary-searches the sorted vector (no runtime map
  iteration). The missing-required pass keeps iterating the schema's
  declaration-order required list so typed-error ordering matches the
  reference validator exactly.
- Behavior is preserved bit-for-bit: all M25-005-A tests (golden corpus,
  mismatch parity, depth bound, compile-to-None, dense table) and the
  runtime HTTP test (`native_response_encoder_emits_declared_order`)
  pass unchanged.

### Tests and evidence

- `encoder_reads_properties_in_declared_fixed_order` — a value whose
  keys arrive reversed with every property failing produces the typed
  error sequence in DECLARED order (alpha, beta, gamma), not value
  order; a valid reversed-key value encodes to declared-order bytes.
- `encoder_program_is_deterministic_across_compiles` — the same schema
  compiles to an equal program and identical bytes on every encode
  (including hoisted-default emission).
- Golden corpus / response mismatch / depth-bound evidence: unchanged
  from M25-005-A and still green (52 unit + 3 fuzz in q-schema-runtime).
- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p velqu-runtime` — 19 integration passed.
- `bun test` — 69 passed, 0 failed, 297 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

No performance claim is made in this packet; benchmark manifest
preserved unchanged.

Commit: `540396f`.
