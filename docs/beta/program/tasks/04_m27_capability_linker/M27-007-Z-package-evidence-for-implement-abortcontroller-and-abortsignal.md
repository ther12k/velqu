---
task_id: M27-007-Z
parent_task: M27-007
milestone: M27
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-007-Z — Package evidence for Implement AbortController and AbortSignal

## Atomic goal

Create source-backed evidence and handoff for parent task M27-007; update status only if verification passed.

## Parent intent

Create one cancellation primitive shared by fetch and native capabilities.

## Dependencies

- `M27-007-V` — `tasks/04_m27_capability_linker/M27-007-V-verify-implement-abortcontroller-and-abortsignal.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Abort propagates exactly once.
- Late listeners follow defined semantics.
- No cross-invocation ownership.
- Shutdown cancellation is bounded.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
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

- Conformance tests.
- Leak tests.
- Race tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m27-007-z: package evidence for implement abortcontroller and abortsign
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Evidence package

- Status: **PASS**. Parent verification M27-007-V merged in PR #882
  at commit `dced7be11723f027bcc9b44d97da787f5552437d`; issue #280
  is closed. Based on clean parent HEAD `b9df75c` (queue-regen).
- Parent acceptance matrix: `M27-007-V` maps all four guardrails
  (atomic exactly-once abort propagation, late listener immediate execution,
  no cross-invocation ownership via scoped signals, and bounded shutdown cancellation).
- Source-backed implementation records:
  - `M27-007-A` (PR #878, #276 closed): `AbortSignalModel`, `AbortControllerModel`,
    and standard WHATWG JS bindings in prelude.
  - `M27-007-B` (PR #879, #277 closed): `ctx.signal`, `req.signal`, and
    `native.timer.delay(ms, { signal })` abort integration.
  - `M27-007-C` (PR #880, #278 closed): listener leak prevention (`MAX_ABORT_LISTENERS = 1024`,
    dispatch auto-clearing, and `AbortSignal.any` source signal cleanup).
  - `M27-007-D` (PR #881, #279 closed): cancellation idempotency and multi-threaded race resilience.
  - `M27-007-V` (PR #882, #280 closed): verification closure + matched manifest refresh.
- Canonical evidence artifacts:
  - Tests: `q-capabilities` 85 passed (+6 AbortSignal/Controller tests),
    `q-engine-quickjs` 110 passed (+2 JS integration tests), `bun test` 194 passed (+13 Abort tests).
  - Manifest: `benchmarks/manifest.json` matched refresh under verify remap environment.
- Exact verification (fresh on this branch): `cargo test` across all crates passes;
  `bun test` 194/0; typecheck, fmt --check, clippy `-D warnings` clean;
  `./scripts/verify` — ALL PASS (exit 0).
- Status bookkeeping: ledger marks M27-007 PASS; TASK_INDEX marks M27-007-Z PASS.
  Queues expose M27-008-A next.
- Remaining scope: M27-008+ (Crypto random subset), M27-GATE.
