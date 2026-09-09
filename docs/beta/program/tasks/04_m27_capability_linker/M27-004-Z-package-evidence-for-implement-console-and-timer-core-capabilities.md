---
task_id: M27-004-Z
parent_task: M27-004
milestone: M27
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-004-Z — Package evidence for Implement console and timer core capabilities

## Atomic goal

Create source-backed evidence and handoff for parent task M27-004; update status only if verification passed.

## Parent intent

Move existing timer behavior under the capability ABI and add bounded console semantics.

## Dependencies

- `M27-004-V` — `tasks/04_m27_capability_linker/M27-004-V-verify-implement-console-and-timer-core-capabilities.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Existing scheduler invariants remain.
- No unbounded logging queue.
- Timers physically cancel.
- Capabilities absent when unused.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
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

- Regression suite.
- Lifecycle tests.
- Overhead measurement.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-004-z: package evidence for implement console and timer core capabi
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M27-004-V merged in PR #864
  at commit `f0cd4bfe7d224ee9b44842cf3276892b25888607`; issue #262
  is closed. Based on clean parent HEAD `2fda05d` (queue-regen).
- Parent acceptance matrix: `M27-004-V` maps all four guardrails
  (scheduler invariants preserved; bounded log sink preventing unbounded
  memory inflation; physical timer task cancellation + `NativeOp` state
  tracking; capability resolver & pruning verified).
- Source-backed implementation records:
  - `M27-004-A` (PR #860, #258 closed): timer cancellation & accounting
    ported under `q-capabilities` ABI (`NativeOp`, `CapabilityLifecycle`).
  - `M27-004-B` (PR #861, #259 closed): `ConsoleLevel` closed vocabulary,
    message bounds `MAX_CONSOLE_MSG_LEN`, token redaction
    (`Bearer`, `Basic`, `sk-live-...`, API keys, passwords).
  - `M27-004-C` (PR #862, #260 closed): `BoundedLogSink` asynchronous
    non-blocking buffering with atomic `LogSinkStats`.
  - `M27-004-D` (PR #863, #261 closed): shutdown & quarantine lifecycle
    transitions and log flushing.
  - `M27-004-V` (PR #864, #262 closed): verification closure + matched
    manifest refresh.
- Canonical evidence artifacts:
  - Tests: `q-engine-quickjs` 106 passed (+4 new timer/console/sink/shutdown tests),
    `q-capabilities` 58 passed, `velqu-runtime` 31 passed, `bun test` 152 passed.
  - Manifest: `benchmarks/manifest.json` matched refresh under verify remap environment.
- Exact verification (fresh on this branch): `cargo test` across all crates passes;
  `bun test` 152/0; typecheck, fmt --check, clippy `-D warnings` clean;
  `./scripts/verify` — ALL PASS (exit 0).
- Status bookkeeping: ledger marks M27-004 PASS; TASK_INDEX marks M27-004-Z PASS.
  Queues expose M27-005-A next.
- Remaining scope: M27-005+ (URL & URLSearchParams), M27-GATE.
