---
task_id: M27-005-A
parent_task: M27-005
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-005-A — Adopt or adapt a proven implementation

## Atomic goal

Adopt or adapt a proven implementation.

## Parent intent

Provide interoperable URL behavior for backend libraries and fetch.

## Dependencies

- `M27-001-Z` — `tasks/04_m27_capability_linker/M27-001-Z-package-evidence-for-define-capability-abi-and-lifecycle-state-machine.md`
- `M27-003-Z` — `tasks/04_m27_capability_linker/M27-003-Z-package-evidence-for-introduce-custom-quickjs-context-profiles.md`

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
5. Implement exactly this deliverable: Adopt or adapt a proven implementation.
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
m27-005-a: adopt or adapt a proven implementation
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-005-A (PASS)

Deliverable: adopt a proven WHATWG URL standard implementation (`url = "2.5"` and `percent-encoding = "2.3"`) wrapped in `q-capabilities` and exposed in QuickJS via `URL` and `URLSearchParams`.

### Changed files

- `Cargo.toml` & `crates/q-capabilities/Cargo.toml` — added `url = "2.5"` and `percent-encoding = "2.3"`.
- `crates/q-capabilities/src/url_model.rs` (new):
  - `ParsedUrl`: href, origin, protocol, username, password, host, hostname, port, pathname, search, hash, search_params. Bounded input length `MAX_URL_LEN` (8,192 B).
  - `ParsedSearchParams`: append, get, getAll, has, set, delete, sort, toString, entries.
  - Fail-closed typed `UrlError` (`EmptyInput`, `InputTooLong`, `InvalidUrl`, `InvalidBase`).
- `crates/q-capabilities/src/lib.rs`: exposed `pub mod url_model;` and re-exported types.
- `crates/q-engine-quickjs/src/prelude.rs`:
  - Defined WHATWG-compliant `URL` and `URLSearchParams` globals and `native.url` capability handle without regular expression literals (regex-free string operations so prelude functions across Full/Web/Minimal profiles).
- `crates/q-engine-quickjs/src/worker.rs`:
  - Registered `__velquUrlParse` native bridge calling `q_capabilities::ParsedUrl::parse`.
  - Added unit test `url_and_urlsearchparams_in_js_environment`.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 64 passed (+6 URL and URLSearchParams parsing, bounds, base URL, encoding/sorting tests).
- `cargo test -p q-engine-quickjs` — 107 passed (+1 JS URL/URLSearchParams integration test).
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-http` — 11 passed.
- `bun test` — 152 passed, 0 failed.

### Commands (fresh worktree on parent HEAD 68c61cd)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 107 · `-p q-http` 11 · `-p q-capabilities` 64 · `-p velqu-runtime` 31 — pass.
- `bun test` 152 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - No unbounded input behavior: bounded by `MAX_URL_LEN` (8,192 B).
  - URL behavior matches fetch usage: standard WHATWG URL parsing with base URL resolution and standard percent-encoding.

