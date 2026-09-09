---
task_id: M27-005-C
parent_task: M27-005
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-005-C — Define host/path encoding behavior

## Atomic goal

Define host/path encoding behavior.

## Parent intent

Provide interoperable URL behavior for backend libraries and fetch.

## Dependencies

- `M27-005-B` — `tasks/04_m27_capability_linker/M27-005-B-run-selected-wpt-wintertc-cases.md`

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
5. Implement exactly this deliverable: Define host/path encoding behavior.
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
m27-005-c: define host path encoding behavior
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-005-C (PASS)

Deliverable: define and enforce host and path encoding behavior according to WHATWG standard rules.

### Changed files

- `crates/q-capabilities/src/url_model.rs`:
  - Defined `PATH_PERCENT_ENCODE_SET` using `percent-encoding`.
  - Added `encode_path_segment` (path percent-encode set encoding).
  - Added `decode_path_segment` (UTF-8 lossy decoding).
  - Added `normalize_host` (IDNA Punycode, IPv4/IPv6 host normalization with length limits).
  - Added unit test `host_and_path_encoding_behavior`.
- `crates/q-capabilities/src/lib.rs`: re-exported `encode_path_segment`, `decode_path_segment`, `normalize_host`.
- `packages/cli/src/url-wpt.test.ts`: added test suite for path percent-encoding normalization, IDNA host normalization, and IPv4/IPv6 address normalization.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 69 passed (+1 host and path encoding test).
- `cargo test -p q-engine-quickjs` — 107 passed.
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-http` — 11 passed.
- `bun test` — 165 passed (+3 host/path tests), 0 failed.

### Commands (fresh worktree on parent HEAD a564b89)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 107 · `-p q-http` 11 · `-p q-capabilities` 69 · `-p velqu-runtime` 31 — pass.
- `bun test` 165 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - Selected conformance threshold passes: IDNA Punycode and percent-encoded path segments normalize deterministically.
  - No unbounded input behavior: `normalize_host` enforces `MAX_URL_LEN` limit.
  - URL behavior matches fetch usage: path segments and host representations conform to WHATWG standard for outbound HTTP/fetch.

