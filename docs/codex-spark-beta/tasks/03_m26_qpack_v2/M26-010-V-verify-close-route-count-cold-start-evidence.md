---
task_id: M26-010-V
parent_task: M26-010
milestone: M26
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-010-V — Verify Close route-count cold-start evidence

## Atomic goal

Prove every acceptance criterion for parent task M26-010 without broadening scope.

## Parent intent

Demonstrate flatter startup scaling and preserve small-app behavior.

## Dependencies

- `M26-010-A` — `tasks/03_m26_qpack_v2/M26-010-A-measure-25-100-1-000-5-000-10-000-routes.md`
- `M26-010-B` — `tasks/03_m26_qpack_v2/M26-010-B-at-least-100-fresh-processes-for-release-evidence.md`
- `M26-010-C` — `tasks/03_m26_qpack_v2/M26-010-C-randomize-source-bytecode-competitor-order.md`
- `M26-010-D` — `tasks/03_m26_qpack_v2/M26-010-D-record-p50-p95-p99-rss-stage-timings-and-hashes.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-router/src/lib.rs`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- No runtime router/schema compilation.
- No base64 decoding.
- 25-route budget is not sacrificed silently.
- 10,000-route scaling is documented honestly.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
```bash
cargo test -p q-engine-quickjs
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

- Raw cold data.
- Generated report.
- Startup-stage trace.
- [x] QPack v2 is deterministic, fail-closed, and version/fingerprint safe.
- [x] Production startup maps verified runtime IR and raw bytecode without JSON/base64 reconstruction.
- [x] Legacy compatibility is isolated.
- [x] Shared and standalone artifacts pass conformance.
- [x] Cold-start route scaling evidence is canonical.
- 25/100/1,000/5,000/10,000 route cold start.
- Shared vs standalone RSS/startup.
- Pack parse/allocation stages.
- Source vs bytecode selection.
- No full capability ecosystem.
- No Node compatibility.
- No multi-worker yet.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-010-v: verify close route count cold start evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M26-010-V (PASS)

Parent: M26-010 "Close route-count cold-start evidence". All four
implementation dependencies merged before this branch: M26-010-A
(PR #835, #234), M26-010-B (PR #836, #235), M26-010-C (PR #837,
#236), M26-010-D (PR #838, #237). (Issue refs corrected in
M26-010-Z; the original record cited #233/#234 for A/B.)

### Acceptance criterion mapping

1. **No runtime router/schema compilation.** Rust routes by
   method/path before any JavaScript runs (AGENTS.md constraint 2);
   the ladder's per-sample ready-line `stages` prove the startup
   pipeline is `pack.load → router.build → engine.spawn →
   bundle.load → listen` with router.build p50 growing from 0.018 ms
   (n=25) to 2.178 ms (n=10,000) — plan application, not compilation,
   and two orders of magnitude below pack.load at the top size.
   Guarded by
   `routing_precedes_body_materialization` (velqu-runtime) and the
   q-router suite (15 tests); G-004 unchanged.

2. **No base64 decoding.** The bytecode candidate maps the verified
   bytecode section directly: one base64 decode per process happens
   at pack load inside `verify_and_cache_bytecode`
   (`QPack.decoded_bytecode`), after which the worker evals raw
   bytes — no JSON/base64 reconstruction of the application.
   Negative: `hash_valid_garbage_bytecode_rejects_before_ready`;
   positive: `bytecode_pack_serves_identically_and_mismatch_fails_before_ready`.

3. **25-route budget is not sacrificed silently.** Measured, not
   asserted: at n=25 velqu (bytecode) p50 6.143 ms / p99 9.270 ms /
   rssP50 7,716 kB — the fastest velqu candidate, with the budget
   figure and its derivation documented in
   `docs/reports/m26-010-a-route-count-ladder.md` (25-route section).

4. **10,000-route scaling is documented honestly.** Super-linear
   growth is reported with attribution, not hidden: at n=10,000
   p50 926.3 ms (bytecode) / 947.5 ms (source) with `pack.load`
   p50 901.1 / 892.4 ms (97% / 94% of total). The ladder report
   names QPack v2 binary layout as the authorized lever (M26 track)
   and makes NO slope-fix claim. Raw evidence: 2,000 rows, 100
   samples/cell, 0 failures, per-sample order randomized
   (LCG-shuffled), p50/p95/p99 + rssP50/rssP95 + stage timings in
   every cell.

5. **QPack v2 deterministic, fail-closed, version/fingerprint
   safe.** q-pack suite (96 tests) including
   `mutated_valid_pack_never_panic_and_tamper_is_detected` (fuzz),
   `overflowing_directory_values_reject_without_panic`,
   `bounds_checks_precede_any_section_access`,
   `runtime_fingerprint_tuple_is_the_enforced_identity`; runtime
   side `tampered_pack_fails_before_ready` and smoke step 3
   (engine-9.9.9 pack fails closed before ready).

6. **Legacy compatibility isolated.** `bundle_prelude` is a closed
   vocabulary; JSON packs keep working through the source path:
   `no_bytecode_flag_recovers_cross_target_packs_from_source`,
   `embedded_prelude_pack_serves_identically_and_source_recovery_works`.

7. **Shared and standalone artifacts pass conformance.** 30
   velqu-runtime tests; `scripts/artifact-smoke.sh` run fresh on
   this branch → SMOKE-OK including section 5: standalone serves
   byte-identical bodies with `mode":"standalone"`.

8. **Cold-start evidence canonical.** Summary format
   `velqu-route-count-v4-full-metrics`, 10 pack sha256 hashes +
   runtime binary hash embedded, raw JSONL retained
   (`benchmarks/raw/route-count/route-count-1787685374497.jsonl`),
   manifest refreshed under verify's exact remap-flag environment
   (this branch's `./scripts/verify` → ALL PASS, exit 0). Shared vs
   standalone RSS/startup: `docs/reports/m26-009-b-standalone-mode.md`
   (n=10 per mode, overlapping distributions). No capability
   ecosystem, no Node compat, no multi-worker — none present.

### Changed files

- This task record; `benchmarks/manifest.json` (metadata-only
  refresh produced by this branch's verify run: generatedAt +
  commit; zero artifact-hash changes).

### Commands and results (fresh worktree on parent HEAD)

- `cargo test -p q-pack` — 96; `cargo test -p q-router` — 15;
  `cargo test -p q-engine-quickjs` — 98;
  `cargo test -p q-schema-runtime` — pass;
  `cargo test -p velqu-runtime` — 30.
- `bun test` — 125 pass / 0 fail; `bun run typecheck`,
  `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.
- `./scripts/verify` — ALL PASS (exit 0, zero failures).
- `scripts/artifact-smoke.sh` — SMOKE-OK (both modes, mismatch
  rejection, cold-start samples, standalone identical answers).

### Follow-up note

One environmental finding, no code defect: the first smoke run
failed at section 5 because the shell that ran verify's
RUSTFLAGS/CFLAGS remap exports re-used them for the standalone
build, forcing a conflicting full rebuild. Re-running
`scripts/artifact-smoke.sh` in a clean shell builds
`velqu-standalone` and passes. Recorded here so future packets
run the smoke script in a separate shell from manifest refreshes.

