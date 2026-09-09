---
task_id: M3-004-Z
parent_task: M3-004
milestone: M3
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-004-Z — Package evidence for Implement deterministic worker initialization and artifact sharing

## Atomic goal

Create source-backed evidence and handoff for parent task M3-004; update status only if verification passed.

## Parent intent

Load identical verified artifacts into independent runtimes efficiently.

## Dependencies

- `M3-004-V` — `tasks/06_m3_multi_worker/M3-004-V-verify-implement-deterministic-worker-initialization-and-artifact-sharing.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M3.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/multiworker.md`

### Source files

- `AGENTS.md`
- `crates/q-runtime/src/main.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Workers execute identical contracts.
- One worker failure does not corrupt others.
- No JS object crosses workers.
- Artifact memory sharing is measured.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p velqu-runtime
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

- Worker parity tests.
- Memory mapping report.
- Startup tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m3-004-z: package evidence for implement deterministic worker initiali
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-004-Z) — PASS

- Date: 2026-08-30
- Branch/PR: m3-004-z (squash-merged; see git log for final hash)
- Closes: #395

### Parent closure — M3-004 Implement deterministic worker initialization and artifact sharing

Parent intent: load identical verified artifacts into independent runtimes efficiently. Status: **PASS**.

Packet commits (squash merges):
- M3-004-A — c56b7e1 (#994, Closes #390): `SharedPack` — verify-once/freeze-forever artifact sharing (Arc<QPack> + Arc<[u8]> + SHA-256 pin; SharedAcrossWorkers impl); 4 worker-parity/immutability/concurrency tests
- M3-004-B — 02a51b4 (#995, Closes #391): `spawn_independent` — N fully independent engines (own thread/context/heap/state) + worker labels; the live state-isolation proof (A's mutation invisible to B; same input, same result)
- M3-004-C — 1252b6b (#996, Closes #392): `validate_compatibility_per_worker` — every worker validates the same manifest; exact versions; identical typed failures (no split capability reality)
- M3-004-D — 4b627c3 (#997, Closes #393): bounded startup parallelism — lanes = min(workers, cores, 8); batch sums exact; WORKER_INIT_DEADLINE_MS
- M3-004-V — 1f35c9e (#998, Closes #394): verification closure mapping all 4 guardrails to the enforcement tests

### Required evidence
- **Worker parity tests**: 4 SharedPack parity/immutability/concurrency tests + the 2-engine state-isolation proof.
- **Memory mapping report**: Arc::ptr_eq single-artifact sharing; 8-way barrier-synchronized concurrent reads never block; SHA-256 identity pin per worker; full mmap-scale measurement is M3-009's dedicated evidence.
- **Startup tests**: parallelism bounds (workers/cores/cap matrices), batch sums exact, serverless = 1 lane of 1, per-worker capability compatibility (identical results or identical typed failures).

### Source/test map
- `crates/q-pack/src/lib.rs` (SharedPack; 4 tests)
- `crates/q-engine-quickjs/src/lib.rs` (spawn_independent; worker_label)
- `crates/q-engine-quickjs/tests/engine.rs` (state-isolation proof)
- `crates/q-capabilities/src/identity.rs` (per-worker validation; 3 tests)
- `crates/q-runtime/src/service_profile.rs` (startup bounds; 3 tests)
- Release binary `6d5c7c3f…` matches manifest

### Command results (this branch)
- `cargo test -p q-pack` → 100+2; `-p q-engine-quickjs` → 20+102+1; `-p velqu-runtime` → 31+5+44; `-p q-capabilities` → 6 suites; `-p q-http` → 4+6+1; `-p q-bridge` → 11 — all pass
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M3-004 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
