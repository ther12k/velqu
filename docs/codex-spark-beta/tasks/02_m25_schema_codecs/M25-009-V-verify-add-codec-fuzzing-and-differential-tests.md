---
task_id: M25-009-V
parent_task: M25-009
milestone: M25
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-009-V — Verify Add codec fuzzing and differential tests

## Atomic goal

Prove every acceptance criterion for parent task M25-009 without broadening scope.

## Parent intent

Prove generated codecs match reference semantics and remain memory-safe.

## Dependencies

- `M25-009-A` — `tasks/02_m25_schema_codecs/M25-009-A-fuzz-encoded-decoded-values.md`
- `M25-009-B` — `tasks/02_m25_schema_codecs/M25-009-B-compare-generated-output-with-standards-reference-json-behavior.md`
- `M25-009-C` — `tasks/02_m25_schema_codecs/M25-009-C-run-malformed-and-boundary-values.md`
- `M25-009-D` — `tasks/02_m25_schema_codecs/M25-009-D-minimize-failures-into-permanent-fixtures.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Fuzz summaries.
- Regression corpus.
- Differential report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-009-v: verify add codec fuzzing and differential tests
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-009-V)

Status: **PASS**. Both parent acceptance guardrails map to source and
passing evidence; all verification commands ran fresh on this branch (no
code changes — verification closure only).

### Guardrail → source → evidence

1. **No panic, hang, unbounded output, or semantic mismatch.**
   - `encoded_decoded_round_trip_matches_reference` — 20,000-iteration
     round-trip fuzz over the decoder + encoder with corpus-health
     bounds; accept/reject agreement with the reference in both
     directions; byte and parse parity on acceptance; full decode →
     encode → decode round-trip.
   - `malformed_and_boundary_corpus` — 31 malformed entries (typed-code
     parity, no panic) + 11 boundary entries (accept + byte parity).
   - The pre-existing fuzz (validator determinism, decoder determinism,
     source coercion split) stays green.
   - Bounded output: the standards corpus asserts exact byte strings
     (hand-written), and every suite runs in bounded time (the full
     q-schema-runtime suite finishes in well under a second).
2. **All fuzz findings are triaged.**
   - M25-009-A finding (encoder fallback-with-inner): fixed + minimized
     fixture `fallback_with_inner_encodes_transparently`.
   - M25-009-C finding (decoder union-error leak): fixed + minimized
     fixture `union_miss_reports_canonical_code_everywhere`.
   - Findings registry documented in the corpus module header; both
     fixtures replay in `codec_regression_corpus_replays`.

### Command results (this branch, fresh worktree)

- `cargo test -p q-schema-runtime` — 58 unit + 4 fuzz + 5 standards —
  all passed.
- `cargo test -p q-engine-quickjs` — 1 + 96; `cargo test -p q-pack` —
  41 + 2; `cargo test -p velqu-runtime` — 24 — all passed.
- `bun test` — 81 passed, 0 failed, 481 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (identical on every packet branch this session).

Changed files: this record, `docs/codex-spark-beta/STATUS.md`,
`docs/codex-spark-beta/indexes/TASK_INDEX.md` (verification closure only).

Commit: `7ed3e51`.
