---
task_id: M26-009-A
parent_task: M26-009
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-009-A — Shared mode: `velqu-runtime` plus app.qpack

## Atomic goal

Shared mode: `velqu-runtime` plus app.qpack.

## Parent intent

Support both small app updates and one-file deployment.

## Dependencies

- `M26-004-Z` — `tasks/03_m26_qpack_v2/M26-004-Z-package-evidence-for-embed-raw-quickjs-bytecode-without-base64.md`
- `M26-005-Z` — `tasks/03_m26_qpack_v2/M26-005-Z-package-evidence-for-implement-zero-copy-or-bounded-copy-pack-reader.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-runtime/src/serve.rs`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Shared mode: `velqu-runtime` plus app.qpack.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Artifact smoke tests.
- Size/cold-start report.
- Install guide.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-009-a: shared mode velqu runtime plus app qpack
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

  Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-009-A)

Status: **PASS**.

### Deliverables

- **Artifact smoke tests** (`scripts/artifact-smoke.sh`): deterministic
  shared-mode check — artifact existence with byte sizes, serve +
  real-route answers (`/health/live`, `/hello/:name`),
  mismatched-runtime rejection (engine 9.9.9 pack fails closed BEFORE
  ready with the actionable "engine mismatch" diagnostic), and
  cold-start sampling from the runtime's own `startupMs` telemetry.
  Run output ends `SMOKE-OK`; any failure exits non-zero.
- **Size/cold-start report**
  (`docs/reports/m26-009-a-shared-mode-artifacts.md`): runtime
  5,194,888 B; proof pack 24,414 B; cold-start p50 ≈ 3.84 ms at 2
  routes (10 samples). Cross-mode startup/RSS delta is M26-009-B's
  measurement obligation per guardrails.
- **Install guide** (`docs/beta/INSTALL.md`): prerequisites, artifact
  producers, run command, fingerprint rule, update matrix, limits,
  explicit standalone-mode pointer.

Guardrails: identical conformance inherited via existing suites driving
the same compiled pack over HTTP; mismatched runtime rejected (smoke
step 3); no toolchain shipped in shared mode; measured numbers above.

### Command results

- Smoke test: SMOKE-OK (all four checks pass).
- `cargo test -p q-pack` — 93 passed; `cargo test -p velqu-runtime` —
  28 passed; `bun test` — 89 passed / 0 fail / 531 expect(); typecheck
  clean; `./scripts/verify` — ALL PASS (exit 0).
