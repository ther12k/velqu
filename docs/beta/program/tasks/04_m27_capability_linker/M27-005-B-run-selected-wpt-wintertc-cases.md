---
task_id: M27-005-B
parent_task: M27-005
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-005-B — Run selected WPT/WinterTC cases

## Atomic goal

Run selected WPT/WinterTC cases.

## Parent intent

Provide interoperable URL behavior for backend libraries and fetch.

## Dependencies

- `M27-005-A` — `tasks/04_m27_capability_linker/M27-005-A-adopt-or-adapt-a-proven-implementation.md`

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
5. Implement exactly this deliverable: Run selected WPT/WinterTC cases.
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
m27-005-b: run selected wpt wintertc cases
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-005-B (PASS)

Deliverable: run selected Web Platform Tests (WPT) and WinterTC URL / URLSearchParams test cases.

### Changed files

- `crates/q-capabilities/src/url_model.rs`:
  - Added WPT relative URL resolution test vectors (`wpt_relative_url_resolution_vectors`).
  - Added WPT default port and special scheme normalizations test vectors (`wpt_special_schemes_and_default_ports`).
  - Added WPT IPv6 and host serialization test vectors (`wpt_ipv6_host_parsing`).
  - Added WinterTC URLSearchParams compliance test cases (`wintertc_urlsearchparams_vectors`).
- `packages/cli/src/url-wpt.test.ts` (new):
  - 10 WPT and WinterTC test cases covering relative paths, dot-segments, port normalizations, canParse, URLSearchParams mutations, encoding, and iterators.
- `docs/reports/m27-005-wpt-url-report.md` (new):
  - Conformance report documenting WPT/WinterTC test vectors, pass rates, and module costs.
- Bookkeeping: STATUS.md, TASK_INDEX.md.

### Tests

- `cargo test -p q-capabilities` — 68 passed (+4 WPT/WinterTC vector tests).
- `cargo test -p q-engine-quickjs` — 107 passed.
- `cargo test -p velqu-runtime` — 31 passed.
- `cargo test -p q-http` — 11 passed.
- `bun test` — 162 passed (+10 new WPT/WinterTC tests in `url-wpt.test.ts`), 0 failed.

### Commands (fresh worktree on parent HEAD f5494a4)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 107 · `-p q-http` 11 · `-p q-capabilities` 68 · `-p velqu-runtime` 31 — pass.
- `bun test` 162 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.

### Notes

- Guardrail mapping:
  - Selected conformance threshold passes: all WPT / WinterTC vector tests pass.
  - URL behavior matches fetch usage: standard WHATWG URL resolution and search param encoding.
  - Binary/startup cost recorded: detailed in `docs/reports/m27-005-wpt-url-report.md`.

