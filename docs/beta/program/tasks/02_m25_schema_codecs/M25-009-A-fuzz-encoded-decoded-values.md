---
task_id: M25-009-A
parent_task: M25-009
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-009-A — Fuzz encoded/decoded values

## Atomic goal

Fuzz encoded/decoded values.

## Parent intent

Prove generated codecs match reference semantics and remain memory-safe.

## Dependencies

- `M25-003-Z` — `tasks/02_m25_schema_codecs/M25-003-Z-package-evidence-for-generate-params-query-header-decoders.md`
- `M25-004-Z` — `tasks/02_m25_schema_codecs/M25-004-Z-package-evidence-for-generate-json-body-decoders.md`
- `M25-005-Z` — `tasks/02_m25_schema_codecs/M25-005-Z-package-evidence-for-generate-status-specific-response-encoders.md`
- `M25-006-Z` — `tasks/02_m25_schema_codecs/M25-006-Z-package-evidence-for-generate-rfc-9457-problem-encoders.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `conformance/security/security.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-http/tests/fuzz_parsers.rs`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Fuzz encoded/decoded values.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No panic, hang, unbounded output, or semantic mismatch.
- All fuzz findings are triaged.
- Coverage targets are recorded.
- Generated code is deterministic.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
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

- Fuzz summaries.
- Regression corpus.
- Differential report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-009-a: fuzz encoded decoded values
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-009-A)

Status: **PASS**. Round-trip fuzz over the generated codecs — and it found
and fixed a real divergence:

- **Fuzz finding (triaged + fixed)**: iteration 2 of the new round-trip
  fuzz exposed that `FieldSpec::Fallback { inner: Some(_) }` compiled
  into the encoder but `encode_spec` had no branch for it — the encoder
  rejected with `unsupported` where the reference validator transparently
  applies the inner shape. Fixed: fallback-with-inner is now transparent
  in the encoder (recurses into the inner spec), mirroring the reference
  exactly. Minimized into a permanent fixture:
  `fallback_with_inner_encodes_transparently` (byte parity + inner-bound
  rejection `maximum`).
- **New fuzz** (`encoded_decoded_round_trip_matches_reference`, 20,000
  iterations over three representable object schemas — mixed bounded
  scalars/optional defaults, bounded arrays, nullable/union/enum/literal/
  fallback-inner): for every (schema, value) pair with a half-biased
  valid-value generator and half arbitrary JSON:
  - the direct encoder accepts EXACTLY when the reference validator
    accepts (no silent divergence in either direction);
  - on acceptance, encoder bytes equal serde_json serialization of the
    reference-normalized output AND parse back to that output;
  - the direct body decoder re-accepts the encoded bytes (full
    decode → encode → decode round-trip parity);
  - corpus health assertions: >1,000 accepted and >1,000 rejected so the
    corpus keeps exercising both sides.

### Tests and evidence

- `encoded_decoded_round_trip_matches_reference` — 4 total fuzz tests in
  the suite, all passing.
- `fallback_with_inner_encodes_transparently` — the minimized fixture.
- `cargo test -p q-schema-runtime` — 58 unit + 4 fuzz passed.
- `cargo test -p q-engine-quickjs` — 1 + 96; `cargo test -p velqu-runtime`
  — 24; `cargo test -p q-pack` — 41 + 2 — all passed.
- `bun test` — 81 passed, 0 failed, 481 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Reference-JSON comparison and malformed/boundary corpus expansion land in
M25-009-B/C.

Commit: `de1ff89`.
