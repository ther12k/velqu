---
task_id: M27-005-D
parent_task: M27-005
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-005-D — Keep parser limits explicit

## Atomic goal

Keep parser limits explicit.

## Parent intent

Provide interoperable URL behavior for backend libraries and fetch.

## Dependencies

- `M27-005-C` — `tasks/04_m27_capability_linker/M27-005-C-define-host-path-encoding-behavior.md`

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

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Keep parser limits explicit.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Selected conformance threshold passes.
- No unbounded input behavior.
- URL behavior matches fetch usage.
- Binary/startup cost recorded.

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

## Required evidence for this microtask

- WPT report.
- Edge-case fixtures.
- Module cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-005-d: keep parser limits explicit
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-005-D (PASS)

Deliverable: keep parser limits explicit and bounded for URL and URLSearchParams parsing.

### Changed files

- `crates/q-capabilities/src/url_model.rs`:
  - Defined explicit constants: `MAX_URL_LEN` (8,192 B), `MAX_SEARCH_PARAMS_LEN` (16,384 B), `MAX_SEARCH_PARAMS_COUNT` (1,024 pairs), `MAX_URL_PATH_SEGMENTS` (256 segments).
  - Added fail-closed `UrlError` variants (`ParamsTooLong`, `TooManyParams`, `TooManyPathSegments`).
  - `ParsedUrl::from_url` validates path segment count against `MAX_URL_PATH_SEGMENTS`.
  - `ParsedSearchParams::try_parse` enforces query string length and parameter count limits.
  - Added unit test `url_and_search_params_parser_limits_enforced`.
- `crates/q-capabilities/src/lib.rs`: re-exported limit constants.
- `crates/q-engine-quickjs/src/prelude.rs`: `URLSearchParams` constructor enforces maximum input string length (16,384 B) and maximum entry count (1,024) fail-closed with `RangeError`.
- `crates/q-engine-quickjs/src/worker.rs`: tested QuickJS-level URL / URLSearchParams limit enforcement.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 70 passed (+1 parser limits test).
- `cargo test -p q-engine-quickjs` — 107 passed (including QuickJS limit tests).
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-http` — 11 passed.
- `bun test` — 165 passed, 0 failed.

### Commands (fresh worktree on parent HEAD f91fad3)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 107 · `-p q-http` 11 · `-p q-capabilities` 70 · `-p velqu-runtime` 31 — pass.
- `bun test` 165 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - No unbounded input behavior: explicit byte length, entry count, and segment limits prevent HashDoS / parser DoS.

