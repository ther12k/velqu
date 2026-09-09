---
task_id: M25-010-C
parent_task: M25-010
milestone: M25
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-010-C — Report cold-start delta at 25/1,000 routes

## Atomic goal

Report cold-start delta at 25/1,000 routes.

## Parent intent

Prove the selected strategies improve real payloads without inflating startup unacceptably.

## Dependencies

- `M25-010-B` — `tasks/02_m25_schema_codecs/M25-010-B-measure-generated-code-pack-size.md`

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
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Report cold-start delta at 25/1,000 routes.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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
cargo test -p q-schema-runtime
```
```bash
bun test
```
```bash
bun run typecheck
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

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m25-010-c: report cold start delta at 25 1 000 routes
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-010-C)

Status: **PASS** (with an escalated gate decision — see below).

- Raw evidence: `benchmarks/raw/route-count/route-count-1787452753541.jsonl`
  (+ regenerated `summary.json`, run `m25-010-c-1787452642`) — 4
  candidates × 3 sizes × 40 fresh-process samples, randomized order,
  zero failures.
- Report: `docs/reports/m25-010-c-cold-start-delta.md` — results table,
  delta vs the previous recorded run, stage attribution, decision
  matrix, guardrail status.
- Fixture refresh (disclosed): committed route-count packs were stale
  (`schemaIrVersion: 1`, no `responseFallbackReason`) and rejected by
  the current runtime at load. Regenerated via the checked-in fixture
  builder, which now tags js response strategies with the closed-
  vocabulary reason `"explicit"` (M25-007-A); bytecode variants
  re-embedded. Pack growth +2.8%; no protocol or assertion changes.
- Key findings (honest): within-run scaling delta 25→1,000 routes is
  +1,223% (source) / +957% (bytecode); vs the G0 smoke the 1,000-route
  cold start regressed ~3–4x (~85 µs per added route at scale,
  up from ~30 µs). Stage logs attribute ~90% of startup to `pack.load`;
  post-ready codec tables are a minority share. Per parent guardrail
  ("no unapproved cold-start regression") the regression is documented
  here and **escalated to M25-GATE** for approval/mitigation (binary
  QPack v2 load path being the natural candidate).
- `benchmarks/manifest.json` refreshed for new pack hashes and the new
  route-count run (fixture pack rebuilt in-place, gitignored output).

### Tests and evidence

- `cargo test -p q-engine-quickjs` — 1 + 96 passed; `cargo test -p
  q-schema-runtime` — 58 + 5 + 4 passed; `cargo test --workspace` — all
  green (incl. q-pack 41 + 2, velqu-runtime 24).
- `bun test` — 81 passed, 0 failed, 481 expect() calls.
- `bun run typecheck` — clean. `cargo fmt --check`, `cargo clippy
  --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors;
  `scripts/validate-benchmark-evidence.py` — no errors.
- `bun run verify` — **ALL PASS**.
- Harness invocation:
  `ROUTE_COUNT_RUN_ID=m25-010-c-$(date +%s) bun benchmarks/harness/route-count.ts --samples=40`.
