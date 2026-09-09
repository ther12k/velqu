---
task_id: M25-009-Z
parent_task: M25-009
milestone: M25
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-009-Z — Package evidence for Add codec fuzzing and differential tests

## Atomic goal

Create source-backed evidence and handoff for parent task M25-009; update status only if verification passed.

## Parent intent

Prove generated codecs match reference semantics and remain memory-safe.

## Dependencies

- `M25-009-V` — `tasks/02_m25_schema_codecs/M25-009-V-verify-add-codec-fuzzing-and-differential-tests.md`

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
- `conformance/security/security.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- No panic, hang, unbounded output, or semantic mismatch.
- All fuzz findings are triaged.
- Coverage targets are recorded.
- Generated code is deterministic.

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

- Fuzz summaries.
- Regression corpus.
- Differential report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m25-009-z: package evidence for add codec fuzzing and differential test
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M25-009-V merged in PR #766 at
  commit `2520030e54a6670a21f8debdf43df5ed9209e647`; issue #172 is
  closed. The evidence package is based on clean parent HEAD `6fd5504`
  before this commit.
- Parent acceptance matrix: `M25-009-V` maps both guardrails to source
  and named tests:
  1. No panic/hang/unbounded output/semantic mismatch: the 20,000-
     iteration round-trip fuzz, the 31+11 malformed/boundary corpus, the
     standards corpus (hand-written exact bytes), pre-existing fuzz.
  2. All fuzz findings triaged: both findings (M25-009-A encoder
     fallback transparency; M25-009-C decoder union-error leak) fixed,
     minimized into permanent fixtures, registry documented.
- Source-backed implementation records:
  - `M25-009-A` (PR #762, #168 closed): round-trip fuzz; found + fixed
    the encoder fallback-with-inner divergence.
  - `M25-009-B` (PR #763, #169 closed): standards/reference JSON
    comparison (hand-written RFC 8259 + RFC 9459 envelope bytes);
    regression corpus replay.
  - `M25-009-C` (PR #764, #170 closed): malformed + boundary corpus;
    found + fixed the decoder union-error parity divergence.
  - `M25-009-D` (PR #765, #171 closed): both findings minimized into
    permanent fixtures with a findings registry.
- Fuzz summaries (required evidence): q-schema-runtime suite = 58 unit +
  4 fuzz (validator/decoder determinism, coercion split, round-trip with
  20,000 iterations and >1,000/>1,000 corpus-health bounds) + 5
  standards-corpus tests — all green.
- No performance measurement or claim; benchmark manifest preserved
  unchanged.
- Exact verification (fresh on this branch): `cargo test -p q-pack`
  (41 + 2); `cargo test -p q-schema-runtime` (58 + 4 + 5);
  `cargo test -p velqu-runtime` (24); `cargo test -p q-engine-quickjs`
  (1 + 96); `bun test` (81 passed, 0 failed, 481 expect calls);
  `bun run typecheck` clean; `cargo fmt --check` clean; `cargo clippy
  --workspace --all-targets -- -D warnings` clean; `scripts/validate-okf`
  (176 links, 0 errors).
- Full `./scripts/verify` completed all Rust, typecheck, proof-build, and
  TypeScript stages. Its final benchmark check reports only the known
  isolated-worktree hash mismatches for `qRuntimeRelease` and `proofPack`
  against `benchmarks/manifest.json`. The canonical root manifest and
  historical raw benchmarks were preserved.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M25-009 PASS;
  the beta checklist and task index mark this Z packet PASS. The
  generated Spark queues now expose M25-010-A (#174) as the next
  dependency-ready packet.
- Remaining scope: `M25-010` and `M25-GATE` remain TODO until implemented
  and evidenced.

Commit: `f1d9491`.
