---
task_id: M25-010-V
parent_task: M25-010
milestone: M25
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-010-V — Verify Close codec performance and cold-start evidence

## Atomic goal

Prove every acceptance criterion for parent task M25-010 without broadening scope.

## Parent intent

Prove the selected strategies improve real payloads without inflating startup unacceptably.

## Dependencies

- `M25-010-A` — `tasks/02_m25_schema_codecs/M25-010-A-run-c2-plus-medium-large-json-workloads.md`
- `M25-010-B` — `tasks/02_m25_schema_codecs/M25-010-B-measure-generated-code-pack-size.md`
- `M25-010-C` — `tasks/02_m25_schema_codecs/M25-010-C-report-cold-start-delta-at-25-1-000-routes.md`
- `M25-010-D` — `tasks/02_m25_schema_codecs/M25-010-D-record-cpu-and-rss.md`

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
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`
- `packages/cli/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- C2 materially improves or limitation is documented.
- No unapproved cold-start regression.
- Reports match raw data.
- Route-specific strategy is inspectable.

## Targeted commands

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
cargo test -p q-capabilities
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

- Raw performance suite.
- Generated report.
- Decision matrix.
- [ ] Canonical Schema IR drives runtime, Treaty, OpenAPI, lock, and diff.
- [ ] Generated decoders/encoders are semantically equivalent and bounded.
- [ ] Fallbacks are explicit and measured.
- [ ] Response errors/problems are exact and redacted correctly.
- [ ] Performance evidence supports route-level strategy selection.
- C2 small JSON.
- 1KB/16KB/64KB dynamic payloads.
- Arrays 100/1,000.
- Request decode and response encode stage timings.
- No binary QPack encoding yet.
- No capability API expansion.
- No ORM.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-010-v: verify close codec performance and cold start evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-010-V)

Status: **PASS**. Verification closure only — no code changes. Every
parent criterion was checked against source/tests and the raw evidence
was independently recomputed (not trusted from the reports).

### Guardrail → source → evidence

1. **C2 materially improves or limitation is documented** — A report:
   records1000 total p50 −16.6% and pad_256 +25.4% vs generic-rust,
   with the parse-dominated-shapes limitation documented; D confirms on
   CPU: records1000 −15%. ✓
2. **No unapproved cold-start regression** — C measured the regression
   honestly (1k-route p50 ~21–23 ms → ~74–84 ms vs the G0 smoke;
   ~90% in `pack.load`), attributed it, and **escalated approval to
   M25-GATE** with candidate mitigations (binary QPack v2 load path).
   Nothing was suppressed or re-baselined silently. ✓ (documented +
   escalated; gate decision outstanding by design)
3. **Reports match raw data** — independent recomputation from raw
   JSONL/summary artifacts, this branch:
   - A: 30/30 codec cells' p50 totals match
     `benchmarks/raw/codec-m25-010-a/codec.jsonl` (≤1.5 µs rounding).
   - B: sizes.json matches its recorded data; current-tree rebuild is
     byte-identical for all runtime-consumed artifacts (app.qpack,
     openapi.json, schema-manifest.json, capability-manifest.json,
     contract.d.ts). Explained drift: route-manifest.json changed
     deterministically with M25-007-A emitter fields (same byte size);
     contract.json/meta/lock + build-report carry build timestamps by
     design (`generatedAt`/`lockedAt`) so their hashes change per
     build. Binaries are toolchain-dependent as disclosed.
   - C: 12/12 summary cells recomputed from
     `route-count-1787452753541.jsonl` match, and 12/12 report-table
     p50s match; D evidence.json sha256s MATCH raw files.
   - D: CPU/alloc/RSS spot checks match `codec-m25-010-d/
     codec-summary.json`; allocator status "captured". ✓
4. **Route-specific strategy is inspectable** — per-route
   `validationStrategy`/`responseStrategy` + closed-vocabulary
   fallback reasons in pack plans, enforced by q-pack verification
   (`rejects_silent_fallback_and_invalid_reasons`, q-pack tests);
   emitted `route-manifest.json` carries strategy fields;
   `velqu inspect routes` (M25-007-D). ✓

### Parent checklist

- Canonical Schema IR drives runtime/Treaty/OpenAPI/lock/diff — one IR
  emits app.qpack + openapi.json + contract.* sidecars (conformance
  suites green). ✓
- Generated decoders/encoders semantically equivalent and bounded —
  M25-009 fuzz/differential suites green inside q-schema-runtime's 67
  passing tests. ✓
- Fallbacks explicit and measured — q-pack rejection test + D's
  allocation/CPU quantification. ✓
- Response errors/problems exact and redacted — SEC-004 conformance
  green in bun test. ✓
- Performance evidence supports route-level strategy selection — A/D
  matrices. Stage timings present (codecUs/engineUs). C2 small JSON ✓;
  1KB/16KB/64KB ✓; arrays 100/1,000 ✓. ✓
- No binary QPack encoding yet / no capability API expansion / no ORM —
  scope confirmed against the diff history of A–D (text QPack v1 only;
  capabilities untouched; no ORM introduced). ✓

### Command results (fresh worktree, this branch)

- `cargo test -p q-engine-quickjs` — 97 passed. `cargo test -p
  q-http` — 11 passed. `cargo test -p q-schema-runtime` — 67 passed.
  `cargo test -p q-capabilities` — 0 tests (crate has none; compiles
  clean). `cargo test -p velqu-runtime` — 24 passed.
- `bun test` — 81 passed, 0 failed, 481 expect() calls.
- `bun run typecheck` — clean. `cargo fmt --all --check` — clean.
  `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — ALL PASS (exit 0).

### Residual risk

One intermediate `./scripts/verify` invocation reported FAILURES
PRESENT right after a full cold workspace clippy/build (residual system
load); an immediate isolated re-run passed every stage with exit 0.
This matches transient conformance-timeout behavior observed during
C's session under heavy parallel load. Not a defect of A–D evidence;
noted for M25-GATE awareness.
