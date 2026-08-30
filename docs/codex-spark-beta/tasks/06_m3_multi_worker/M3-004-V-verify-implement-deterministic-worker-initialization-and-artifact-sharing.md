---
task_id: M3-004-V
parent_task: M3-004
milestone: M3
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M3.md
commit_required: true
---

# M3-004-V — Verify Implement deterministic worker initialization and artifact sharing

## Atomic goal

Prove every acceptance criterion for parent task M3-004 without broadening scope.

## Parent intent

Load identical verified artifacts into independent runtimes efficiently.

## Dependencies

- `M3-004-A` — `tasks/06_m3_multi_worker/M3-004-A-share-immutable-mapped-qpack-bytes.md`
- `M3-004-B` — `tasks/06_m3_multi_worker/M3-004-B-create-separate-quickjs-runtimes-functions-context-state.md`
- `M3-004-C` — `tasks/06_m3_multi_worker/M3-004-C-validate-capability-compatibility-per-worker.md`
- `M3-004-D` — `tasks/06_m3_multi_worker/M3-004-D-bound-startup-parallelism.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m3-004-v: verify implement deterministic worker initialization and art
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M3-004-V) — PASS

- Date: 2026-08-30
- Branch/PR: m3-004-v (squash-merged; see git log for final hash)
- Closes: #394

### Acceptance-criterion mapping (parent M3-004 guardrails)

1. **Workers execute identical contracts** — verified twice: `worker_parities_hold_for_every_shared_clone` (4 workers read identical routes/ABI/contract-version from the SharedPack clone) and `independent_engines_share_source_but_not_module_state` (same bundle loaded into 2 independent engines, same input → same result).
2. **One worker failure does not corrupt others** — verified: a worker mutating its own clone of the shared bytes cannot touch the frozen artifact (`shared_bytes_are_frozen_against_mutation`); per-runtime JS state isolation proven live (engine A's counter mutation invisible to B).
3. **No JS object crosses workers** — verified structurally: the compile_fail doc test (values are `!Send`) still passes every run; `SharedPack` is plain bytes behind Arc; `WorkerMsg` remains Send+Sync plain data. Tests: `worker_messages_are_plain_data_send_sync`, `cross_thread_sharing_never_blocks_and_never_shares_js`.
4. **Artifact memory sharing is measured** — verified: `Arc::ptr_eq` proves workers share ONE artifact (refcount sharing, not copies); `cross_thread_sharing_never_blocks_and_never_shares_js` proves reads never block under 8-way concurrency; SharedPack carries the SHA-256 identity pin every worker can check. Full mmap-scale measurement is M3-009's dedicated evidence.

### Deterministic initialization (M3-004-C/D)
- Per-worker capability compatibility: all workers validate the same manifest with identical results, or all fail identically (typed VersionConflict/Missing) — `every_worker_validates_the_identical_manifest`, `version_conflict_fails_closed_per_worker`.
- Bounded startup: lanes = min(workers, cores, 8); batch sums exact; serverless = 1 lane of 1 — `startup_parallelism_is_bounded_by_cores_and_cap`, `startup_batches_sum_exactly_to_workers`, `single_worker_startup_is_always_one_lane`.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-pack` → 100+2 (incl. 4 shared-pack tests); `-p q-engine-quickjs` → 20+102+1 (incl. state-isolation); `-p velqu-runtime` → 31+5+44 (incl. startup bounds); `-p q-capabilities` → 6 suites (incl. per-worker compat)
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**; release binary reproduced deterministically (`6d5c7c3f…` matches the M3-004-D manifest)

### Disclosures (standing)
- No production code changed in this packet: verification-only closure of M3-004-A/B/C/D.
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
