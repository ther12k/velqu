---
task_id: M27-004-B
parent_task: M27-004
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-004-B — Define console levels and redaction

## Atomic goal

Define console levels and redaction.

## Parent intent

Move existing timer behavior under the capability ABI and add bounded console semantics.

## Dependencies

- `M27-004-A` — `tasks/04_m27_capability_linker/M27-004-A-port-timer-cancellation-accounting.md`

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
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `docs/reports/`
- `docs/beta/workstreams/OBSERVABILITY_OPERATIONS.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define console levels and redaction.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Regression suite.
- Lifecycle tests.
- Overhead measurement.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-004-b: define console levels and redaction
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-004-B (PASS)

Deliverable: define console levels, message bounds, and sensitive data redaction.

### Changed files

- `crates/q-capabilities/src/console.rs` (new):
  - `ConsoleLevel`: closed vocabulary (`Debug`, `Info`, `Warn`, `Error`), default `Info`, parsing & display.
  - `MAX_CONSOLE_MSG_LEN` (16,384 bytes) and `MAX_CONSOLE_ARGS` (32).
  - `redact_sensitive_text`: automatic redaction of `Bearer ...`, `Basic ...`, `sk-live-...`, API keys, passwords, auth tokens, secrets.
  - `ConsoleRecord`: structured log record rendering `{ level, event: "console.log", message, routeId, invocationId }` with bounded message length.
- `crates/q-capabilities/src/lib.rs` & `Cargo.toml`: exposed `console` module and re-exports; added `serde` and `serde_json` dependencies.
- `crates/q-engine-quickjs/src/prelude.rs`: defined `console` global (`debug`, `log`, `info`, `warn`, `error`) and `native.console` capability handle formatting up to 32 arguments.
- `crates/q-engine-quickjs/src/worker.rs`: registered native `__velquConsoleLog` implementing redaction, bounding, and structured output; added unit test `console_capability_methods_and_redaction`.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 57 passed (+6 console level/redaction/bounds tests).
- `cargo test -p q-engine-quickjs` — 104 passed (+1 console JS environment test).
- `cargo test -p velqu-runtime` — 31 passed.
- `bun test` — 152 passed, 0 failed.

### Commands (fresh worktree on parent HEAD aeecfe1)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 104 · `-p q-capabilities` 57 · `-p velqu-runtime` 31 — pass.
- `bun test` 152 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - Existing scheduler invariants remain: all scheduler/quarantine tests pass unchanged.
  - No unbounded logging queue: message lengths are bounded by `MAX_CONSOLE_MSG_LEN` and args by `MAX_CONSOLE_ARGS`; sink buffering is in M27-004-C.
  - Timers physically cancel: verified in M27-004-A and passing.

