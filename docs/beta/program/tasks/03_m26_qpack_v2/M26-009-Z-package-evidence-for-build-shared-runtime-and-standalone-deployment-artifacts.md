---
task_id: M26-009-Z
parent_task: M26-009
milestone: M26
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-009-Z — Package evidence for Build shared-runtime and standalone deployment artifacts

## Atomic goal

Create source-backed evidence and handoff for parent task M26-009; update status only if verification passed.

## Parent intent

Support both small app updates and one-file deployment.

## Dependencies

- `M26-009-V` — `tasks/03_m26_qpack_v2/M26-009-V-verify-build-shared-runtime-and-standalone-deployment-artifacts.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`
- `context/components/evidence.md`

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
- `scripts/package`
- `scripts/release-packet`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m26-009-z: package evidence for build shared runtime and standalone dep
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M26-009-V merged in PR #833 at
  commit `55d806f83e2b845d8b97d92f03fb4b42561e3cd4`; issue #232 is
  closed. The evidence package is based on clean parent HEAD before
  this commit.
- Parent acceptance matrix: `M26-009-V` maps all four guardrails to
  source and named tests (identical conformance both modes; standalone
  toolchain-free; shared-mode mismatch rejection before ready +
  pre-checkable; startup/RSS measured with raw samples).
- Source-backed implementation records:
  - `M26-009-A` (PR #824, #228 closed): shared-mode artifact smoke
    test, size/cold-start report, install guide.
  - `M26-009-B` (PR #830, #229 closed): standalone mode —
    `velqu_runtime` lib extraction (`PackSource::{Path, Embedded}`),
    feature-gated `velqu-standalone` bin embedding the pack at compile
    time, `verify_from_slice` (embedded artifact still fully
    verified), cross-mode smoke section, measured startup/RSS/sizes
    report. Completed on the idle parallel session's WIP (provenance
    documented in the record).
  - `M26-009-C` (PR #831, #230 closed): exact runtime fingerprint —
    `RuntimeFingerprint` type + `--fingerprint` on both binaries
    (verify without serving; exit 0/2 with diagnostics).
  - `M26-009-D` (PR #832, #231 closed): debug sidecar convention for
    both modes (`<pack>.sources.json` / `<executable>.sources.json`),
    tooling helpers, `packSha256` binding key via `--fingerprint`,
    `docs/beta/DEBUGGING.md`.
  - `M26-009-V` (PR #833, #232 closed): verification closure; no
    defects found.
- Exact verification (fresh on this branch): q-pack 94+2, q-router 15,
  q-engine-quickjs 1+97, velqu-runtime 30 passed; bun 125 pass / 0
  fail; typecheck, fmt --check, clippy `-D warnings` clean.
  `./scripts/verify` — ALL PASS (exit 0), including
  validate-benchmark-evidence after the matched manifest refresh on
  this branch.
- Status bookkeeping: `docs/beta/04_TASK_LEDGER.md` marks M26-009 PASS;
  TASK_INDEX marks M26-009-Z PASS. The generated Spark queues expose
  M26-010-A next.
- Remaining scope: `M26-010` (route-count cold-start evidence) and the
  M26-GATE remain TODO.
