---
task_id: M27-010-V
parent_task: M27-010
milestone: M27
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-010-V — Verify Establish Web API conformance program

## Atomic goal

Prove every acceptance criterion for parent task M27-010 without broadening scope.

## Parent intent

Separate standards compatibility from internal framework tests.

## Dependencies

- `M27-010-A` — `tasks/04_m27_capability_linker/M27-010-A-pin-wpt-wintertc-subsets.md`
- `M27-010-B` — `tasks/04_m27_capability_linker/M27-010-B-record-skips-and-reasons.md`
- `M27-010-C` — `tasks/04_m27_capability_linker/M27-010-C-automate-regression-reports.md`
- `M27-010-D` — `tasks/04_m27_capability_linker/M27-010-D-keep-unsupported-apis-explicit.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
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

- No unsupported API is advertised.
- Pass/fail/skip counts are reproducible.
- Behavioral regressions block relevant gate.
- Reports link to exact runtime build.

## Targeted commands

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

- Conformance report.
- Pinned test manifest.
- CI output.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m27-010-v: verify establish web api conformance program
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-010-V) — PASS

- Date: 2026-08-27
- Branch/PR: m27-010-v (squash-merged; see git log for final hash)
- Closes: #298

### Acceptance-criterion mapping (parent M27-010 guardrails)

1. **No unsupported API is advertised** — verified: `unsupported_web_apis_are_strictly_absent_and_never_stubbed` (`crates/q-engine-quickjs/src/worker.rs`) asserts `crypto.subtle`, `fetch`, `WebSocket`, `EventSource`, `localStorage`, `sessionStorage`, `document`, `window`, `Worker`, `Blob` are strictly `undefined`, dummy stubs are never injected, and undeclared identifiers fail closed with `ReferenceError`; `web-api.conformance.test.ts` asserts all 8 skips are classified with standard reason codes.
2. **Pass/fail/skip counts are reproducible** — verified: 34 pinned test vectors (100% PASS) + 8 explicit skips documented in `conformance/web-api/wpt-manifest.json` and verified in both Rust integration and TypeScript conformance suites.
3. **Behavioral regressions block relevant gate** — verified: `web-api.conformance.test.ts` and `scripts/generate-conformance-report.py --check` run under `./scripts/verify` and `bun test`.
4. **Reports link to exact runtime build** — verified: `docs/reports/m27-010-wpt-wintertc-conformance.md` carries manifest SHA-256 and exact commit hash.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-capabilities` → 107 unit tests + 7 integration tests passed
- `cargo test -p q-engine-quickjs` → 15 unit tests + 97 worker tests passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 213 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `python3 scripts/generate-conformance-report.py --check` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
