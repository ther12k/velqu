---
task_id: M25-005-V
parent_task: M25-005
milestone: M25
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-005-V — Verify Generate status-specific response encoders

## Atomic goal

Prove every acceptance criterion for parent task M25-005 without broadening scope.

## Parent intent

Fuse output validation and serialization for stable response contracts.

## Dependencies

- `M25-005-A` — `tasks/02_m25_schema_codecs/M25-005-A-generate-per-status-encoders.md`
- `M25-005-B` — `tasks/02_m25_schema_codecs/M25-005-B-read-declared-properties-in-fixed-order.md`
- `M25-005-C` — `tasks/02_m25_schema_codecs/M25-005-C-handle-optional-null-union-fields.md`
- `M25-005-D` — `tasks/02_m25_schema_codecs/M25-005-D-keep-quickjs-stringify-or-generic-fallback-when-measured-better.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

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
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p q-http
```
```bash
cargo test -p q-bridge
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

- Golden JSON corpus.
- Response mismatch tests.
- Mapping deadline tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-005-v: verify generate status specific response encoders
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-005-V)

Status: **PASS**. Every parent M25-005 acceptance guardrail maps to source
and passing tests; all verification commands were run fresh on this branch
(no code changes — verification closure only).

### Guardrail → source → evidence

1. **Undeclared status/body remains a contract violation.**
   - Source: `crates/q-runtime/src/serve.rs` declared-status response gate
     (unchanged); encoder typed errors route to the same controlled 500
     `contract.violation.response` (logged internally, redacted from the
     wire).
   - `runtime_conformance::response_schema_violation_is_a_controlled_500`
     (now exercising the encoder path — its flat schema compiles).
   - `runtime_conformance::native_response_encoder_emits_declared_order`
     — mismatch twin returns 500 with the violation logged.
2. **Output is JSON-equivalent to reference serialization.**
   - `encoder_matches_reference_serialization_on_golden_corpus` — byte
     equality against `serde_json::to_vec` of the reference normalized
     output across the golden corpus.
   - `optional_null_combinations_match_reference` and
     `unions_encode_via_first_matching_member_with_parity` — combination
     and union parity.
   - `runtime_conformance::quickjs_stringify_fallback_stays_json_equivalent_
     to_encoder` — live HTTP JSON-equality between the retained js
     fallback and the generated encoder.
3. **One traversal for generated paths.**
   - Source: `EncoderProgram::encode` walks the handler value once and
     emits bytes directly; the serve wiring replaces the previous
     validate pass + Value clone + second serialization.
   - `encoder_reads_properties_in_declared_fixed_order` and
     `encoder_program_is_deterministic_across_compiles` — the frozen
     property order and deterministic bytes.
4. **No user JS escapes deadline ownership during conversion.**
   - Source: encoding is native host code executed after engine
     settlement — no JS runs during conversion; recursion is bounded by
     `MAX_VALIDATE_DEPTH`.
   - `encoder_depth_is_bounded` (typed `depth` parity with the reference
     bound); M25-004-D `body_read_deadline_cancels_stalled_transfer` and
     the engine deadline tests remain green.

### Command results (this branch, fresh worktree)

- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-schema-runtime` — 54 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 20 integration passed.
- `cargo test --workspace` — zero failures (two consecutive runs).
- `bun test` — 69 passed, 0 failed, 297 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — first run reported one transient
  `FAIL: cargo test` that did not reproduce: two subsequent full verify
  runs and two direct `cargo test --workspace` runs were completely
  green (zero failed tests across every crate). Final state: all stages
  pass except the documented isolated-worktree `qRuntimeRelease`/
  `proofPack` manifest hash mismatch (identical on every packet branch
  this session).

CPU/allocation evidence is not re-measured here: the encode path is
native host code; no new performance claim is made and the benchmark
manifest is preserved unchanged.

Changed files: this record, `docs/codex-spark-beta/STATUS.md`,
`docs/codex-spark-beta/indexes/TASK_INDEX.md` (verification closure only).

Commit: `c9f05f1`.
