---
task_id: M25-005-Z
parent_task: M25-005
milestone: M25
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-005-Z — Package evidence for Generate status-specific response encoders

## Atomic goal

Create source-backed evidence and handoff for parent task M25-005; update status only if verification passed.

## Parent intent

Fuse output validation and serialization for stable response contracts.

## Dependencies

- `M25-005-V` — `tasks/02_m25_schema_codecs/M25-005-V-verify-generate-status-specific-response-encoders.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Golden JSON corpus.
- Response mismatch tests.
- Mapping deadline tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-005-z: package evidence for generate status specific response encod
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M25-005-V merged in PR #742 at commit
  `2a9783cf6223f4165520b810a8d41c673aa8d47a`; issue #148 is closed. The
  evidence package is based on clean parent HEAD `bc4e08d` before this
  commit.
- Parent acceptance matrix: `M25-005-V` maps all four guardrails to source
  and named tests:
  1. Undeclared status/body stays a contract violation:
     `crates/q-runtime/src/serve.rs` declared-status gate;
     `response_schema_violation_is_a_controlled_500` (encoder path);
     mismatch twin in `native_response_encoder_emits_declared_order`.
  2. Output JSON-equivalent to reference serialization:
     `encoder_matches_reference_serialization_on_golden_corpus` (byte
     equality), `optional_null_combinations_match_reference`,
     `unions_encode_via_first_matching_member_with_parity`, live-HTTP
     JSON-equality in `quickjs_stringify_fallback_stays_json_equivalent_
     to_encoder`.
  3. One traversal for generated paths: `EncoderProgram::encode` single
     walk emitting bytes directly; `encoder_reads_properties_in_declared_
     fixed_order`, `encoder_program_is_deterministic_across_compiles`.
  4. No user JS escapes deadline ownership during conversion: encoding is
     native host code after engine settlement, depth-bounded
     (`encoder_depth_is_bounded`); M25-004-D deadline evidence remains
     green.
- Source-backed implementation records:
  - `M25-005-A` (PR #738, #144 closed): generated per-status encoders
    (`EncoderProgram`/`EncoderTable`, one-traversal validate+emit, byte
    parity via serde_json leaf delegation, compile-to-None for
    unrepresentable schemas, runtime wiring with startup-resolved
    status→SchemaId maps).
  - `M25-005-B` (PR #739, #145 closed): fixed declared property order —
    frozen `Vec<PropertyEncoder>` with hoisted defaults and binary-search
    unknown-key detection; determinism evidence.
  - `M25-005-C` (PR #740, #146 closed): optional/null/union handling —
    scratch-buffer union retry (first-match-wins, typed parity),
    combination matrix evidence.
  - `M25-005-D` (PR #741, #147 closed): QuickJS stringify and generic
    fallback retained and JSON-equal (twin-route proof); `measured`
    selection mechanism from M25-002-D intact; no default flipped without
    new measurement.
- CPU/allocation evidence: no new measurement is made in M25-005; the
  encode path is native host code and no performance claim is asserted.
  The benchmark manifest is preserved unchanged.
- Exact verification (fresh on this branch): `cargo test -p q-schema-runtime`
  (54 unit + 3 fuzz pass); `cargo test -p velqu-runtime` (20 pass);
  `cargo test -p q-engine-quickjs` (1 + 96 pass); `bun test` (69 passed,
  0 failed, 297 expect calls); `bun run typecheck` clean; `cargo fmt
  --check` clean; `cargo clippy --workspace --all-targets -- -D warnings`
  clean; `scripts/validate-okf` (176 links, 0 errors).
- Full `./scripts/verify` completed all Rust, typecheck, proof-build, and
  TypeScript stages. Its final benchmark check reports only the known
  isolated-worktree hash mismatches for `qRuntimeRelease` and `proofPack`
  against `benchmarks/manifest.json`. The canonical root manifest and
  historical raw benchmarks were preserved.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M25-005 PASS;
  the beta checklist and task index mark this Z packet PASS. The generated
  Spark queues now expose M25-006-A (#150) as the next dependency-ready
  packet.
- Remaining scope: `M25-006`–`M25-010` and `M25-GATE` remain TODO until
  implemented and evidenced.

Commit: `d53c5c8`.
