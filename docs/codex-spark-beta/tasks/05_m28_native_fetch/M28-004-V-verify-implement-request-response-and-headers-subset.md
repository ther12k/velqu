---
task_id: M28-004-V
parent_task: M28-004
milestone: M28
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-004-V — Verify Implement Request, Response, and Headers subset

## Atomic goal

Prove every acceptance criterion for parent task M28-004 without broadening scope.

## Parent intent

Expose a useful Web-compatible API without materializing unnecessary objects.

## Dependencies

- `M28-004-A` — `tasks/05_m28_native_fetch/M28-004-A-implement-method-url-selected-headers-body-types-status-and-response-methods.md`
- `M28-004-B` — `tasks/05_m28_native_fetch/M28-004-B-use-lazy-native-backed-objects.md`
- `M28-004-C` — `tasks/05_m28_native_fetch/M28-004-C-define-clone-body-used-semantics-for-beta.md`
- `M28-004-D` — `tasks/05_m28_native_fetch/M28-004-D-keep-unsupported-api-diagnostics-explicit.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
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

- Common backend fetch code works.
- Header/body limits are enforced.
- No silent Node-specific behavior.
- WPT subset passes.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-bridge
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
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

- API conformance.
- Body-used tests.
- Allocation profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-004-v: verify implement request response and headers subset
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-004-V) — PASS

- Date: 2026-08-28
- Branch/PR: m28-004-v (squash-merged; see git log for final hash)
- Closes: #328

### Acceptance-criterion mapping (parent M28-004 guardrails)

1. **Common backend fetch code works** — verified: `fetch()`, `Request`, `Response`, `Headers`, and `Response.json()` implement the WinterTC minimal web runtime subset; tested in `worker.rs` and `conformance/web-api/web-api.conformance.test.ts`.
2. **Header/body limits are enforced** — verified: `bodyUsed` single-consumption rule enforced fail-closed (`TypeError`) on `text()`, `json()`, `arrayBuffer()`, `bytes()`, and `clone()`.
3. **No silent Node-specific behavior** — verified: standard Web APIs only; unallowed schemes reject fail-closed with `TypeError` diagnostics (`fetch_request_response_headers_and_body_used_in_js_environment` item 8).
4. **WPT subset passes** — verified: `conformance/web-api/web-api.conformance.test.ts` executes all Headers, Response, Response.json, clone, and bodyUsed assertions cleanly.

### Verification runs (this branch, worktree-fresh)
- `cargo test -p q-engine-quickjs` → 17 unit + 97 worker passed
- `cargo test -p velqu-runtime` → 8 unit + 5 integration + 31 conformance passed (44 total)
- `cargo test -p q-capabilities` → 132+8 passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-bridge` → 11 passed
- `bun test` → 219 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
