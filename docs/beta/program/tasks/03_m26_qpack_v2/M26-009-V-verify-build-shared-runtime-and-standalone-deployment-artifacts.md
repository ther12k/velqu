---
task_id: M26-009-V
parent_task: M26-009
milestone: M26
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-009-V — Verify Build shared-runtime and standalone deployment artifacts

## Atomic goal

Prove every acceptance criterion for parent task M26-009 without broadening scope.

## Parent intent

Support both small app updates and one-file deployment.

## Dependencies

- `M26-009-A` — `tasks/03_m26_qpack_v2/M26-009-A-shared-mode-velqu-runtime-plus-app-qpack.md`
- `M26-009-B` — `tasks/03_m26_qpack_v2/M26-009-B-standalone-mode-embedded-qpack-executable.md`
- `M26-009-C` — `tasks/03_m26_qpack_v2/M26-009-C-ensure-exact-runtime-fingerprint.md`
- `M26-009-D` — `tasks/03_m26_qpack_v2/M26-009-D-define-source-map-debug-sidecars.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/evidence.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`
- `docs/reports/`
- `scripts/benchmark`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Both modes pass identical conformance.
- Standalone contains no compiler toolchain.
- Shared mode rejects mismatched runtime.
- Startup/RSS differences are measured.

## Targeted commands

```bash
cargo test -p q-pack
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

- Artifact smoke tests.
- Size/cold-start report.
- Install guide.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-009-v: verify build shared runtime and standalone deployment artifa
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M26-009-V (PASS)

Parent: M26-009 "Build shared-runtime and standalone deployment
artifacts". All four implementation dependencies merged before this
branch: M26-009-A (PR #824, #228), M26-009-B (PR #830, #229),
M26-009-C (PR #831, #230), M26-009-D (PR #832, #231).

### Acceptance criterion mapping

1. **Both modes pass identical conformance.**
   Shared mode: the 30 velqu-runtime conformance tests run the same
   compiled pipeline (`velqu_runtime::run`) extracted in M26-009-B.
   Cross-mode: `scripts/artifact-smoke.sh` section 5 (run fresh on
   this branch → SMOKE-OK) serves BOTH binaries over the same pack
   and asserts byte-identical /health/live and /hello/:name bodies
   plus `mode":"standalone"` telemetry. `--fingerprint` exercises the
   identical verification path in both modes (standalone verified
   compatible on this branch).

2. **Standalone contains no compiler toolchain.**
   `velqu-standalone` links only the runtime pipeline: no Bun, no
   TypeScript, no compiler crates — `cargo tree`-level separation
   (bin depends on `velqu_runtime` + clap only); G-004 preserved (the
   startup performs zero compilation in both modes). The binary is
   built behind `--features standalone` so default builds never see
   it.

3. **Shared mode rejects mismatched runtime.**
   Smoke step 3 (fresh run on this branch): engine-9.9.9 pack fails
   closed BEFORE ready with the actionable "engine mismatch"
   diagnostic. Pre-checkable without serving via
   `--fingerprint` (exit 2 + diagnostic; conformance test
   `fingerprint_flag_reports_exact_tuple_and_verifies_without_serving`).

4. **Startup/RSS differences are measured.**
   `docs/reports/m26-009-b-standalone-mode.md`: n=10 per mode, full
   raw samples — startupMs p50 3.500 shared vs 2.976 standalone;
   VmRSS p50 7,236 vs 7,124 kB; artifact sizes 5,201,208 vs
   5,224,216 B. Overlapping distributions, same-host delta only.

### Changed files

- This task record only. No defects found; no follow-up tasks needed.

### Commands and results (fresh worktree on parent HEAD)

- `cargo test -p q-pack` — 94 + 2; `cargo test -p q-router` — 15;
  `cargo test -p q-engine-quickjs` — 1 + 97;
  `cargo test -p velqu-runtime` — 30.
- `bun test` — 125 pass / 0 fail; `bun run typecheck`,
  `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.
- `scripts/artifact-smoke.sh` — SMOKE-OK (both modes, mismatch
  rejection, 10 cold-start samples).
- `./scripts/verify` — ALL PASS (exit 0).
