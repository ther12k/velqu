---
task_id: M25-009-B
parent_task: M25-009
milestone: M25
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-009-B — Compare generated output with standards/reference JSON behavior

## Atomic goal

Compare generated output with standards/reference JSON behavior.

## Parent intent

Prove generated codecs match reference semantics and remain memory-safe.

## Dependencies

- `M25-009-A` — `tasks/02_m25_schema_codecs/M25-009-A-fuzz-encoded-decoded-values.md`

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
5. Implement exactly this deliverable: Compare generated output with standards/reference JSON behavior.
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
m25-009-b: compare generated output with standards reference json behav
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-009-B)

Status: **PASS**. Generated codec output is compared against STANDARD JSON
behavior through hand-written expected bytes (independent of serde_json
construction), plus a minimized regression corpus:

- New suite `crates/q-schema-runtime/tests/codec_standards_corpus.rs`
  (3 tests):
  1. `encoder_output_matches_standard_json_bytes` — encoder bytes equal
     hand-written RFC 8259 expectations across string escaping (quote,
     backslash, the two-character control escapes `
`/`	`,
     ``/`` for the rest of the control range, non-ASCII
     UTF-8 passthrough ☺ƒ, forward slash NOT escaped), number formatting
     (2.5, integral-float `3.0` staying float form, `0.0`, i64
     MIN/MAX), member ordering, empty containers, and nested arrays —
     each case also re-parsed by a standards-conformant parser and
     compared semantically.
  2. `problem_encoder_matches_standard_envelope` — the RFC 9457 envelope
     in canonical member order against a hand-written envelope string
     (type, title, status, instance, detail, extension last).
  3. `codec_regression_corpus_replays` — the minimized M25-009-A finding
     (fallback-with-inner transparency: encodes `{"fb":21}`, rejects 99
     with the inner `maximum` code) plus exact-boundary replays for
     minLength/maxLength (2..4 accepted at bounds; 1 and 5 rejected).
- Fuzz summaries (required evidence): the M25-009-A round-trip fuzz
  (20,000 iterations, >1,000 accepted / >1,000 rejected corpus-health
  bounds) remains green alongside the new corpus.

### Tests and evidence

- `codec_standards_corpus` — 3 passed.
- `cargo test -p q-schema-runtime` — 58 unit + 4 fuzz + 3 standards —
  all passed.
- `cargo test -p q-engine-quickjs` — 1 + 96; `cargo test -p q-pack` —
  41 + 2; `cargo test -p velqu-runtime` — 24 — all passed.
- `bun test` — 81 passed, 0 failed, 481 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (known, pre-existing on every packet branch).

Malformed-input corpus expansion lands in M25-009-C.

Commit: `ab2a8c3`.
