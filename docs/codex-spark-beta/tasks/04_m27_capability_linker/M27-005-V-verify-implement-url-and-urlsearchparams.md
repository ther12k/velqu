---
task_id: M27-005-V
parent_task: M27-005
milestone: M27
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-005-V — Verify Implement URL and URLSearchParams

## Atomic goal

Prove every acceptance criterion for parent task M27-005 without broadening scope.

## Parent intent

Provide interoperable URL behavior for backend libraries and fetch.

## Dependencies

- `M27-005-A` — `tasks/04_m27_capability_linker/M27-005-A-adopt-or-adapt-a-proven-implementation.md`
- `M27-005-B` — `tasks/04_m27_capability_linker/M27-005-B-run-selected-wpt-wintertc-cases.md`
- `M27-005-C` — `tasks/04_m27_capability_linker/M27-005-C-define-host-path-encoding-behavior.md`
- `M27-005-D` — `tasks/04_m27_capability_linker/M27-005-D-keep-parser-limits-explicit.md`

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

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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

- WPT report.
- Edge-case fixtures.
- Module cost report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-005-v: verify implement url and urlsearchparams
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Verification record — M27-005-V (PASS)

Parent: M27-005 "Implement URL and URLSearchParams".
Implementation packets merged prior: A (PR #866, #264), B
(PR #867, #265), C (PR #868, #266), D (PR #869, #267).

### Guardrail map

1. **Selected conformance threshold passes.** WHATWG URL and WinterTC URLSearchParams vector suites pass across Rust (`url_model.rs`) and QuickJS (`url-wpt.test.ts`).
2. **No unbounded input behavior.** Bounded by explicit constants: `MAX_URL_LEN` (8,192 B), `MAX_SEARCH_PARAMS_LEN` (16,384 B), `MAX_SEARCH_PARAMS_COUNT` (1,024 pairs), `MAX_URL_PATH_SEGMENTS` (256 segments).
3. **URL behavior matches fetch usage.** Standard WHATWG URL resolution, default port omission, IDNA Punycode host handling, and percent-encoded path segments conform to fetch prerequisites.
4. **Binary/startup cost recorded.** Documented in `docs/reports/m27-005-wpt-url-report.md`.

### Manifest

Matched refresh under verify's remap env (qRuntimeRelease hash updated for URL capability integration).

### Commands and results (fresh worktree on parent HEAD c99cd0b)

- `cargo test -p q-pack` 98 · `-p q-engine-quickjs` 107 · `-p q-http` 11 · `-p q-capabilities` 70 · `-p velqu-runtime` 31 — pass.
- `bun test` 165 pass / 0 fail; `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — ALL PASS (exit 0).

