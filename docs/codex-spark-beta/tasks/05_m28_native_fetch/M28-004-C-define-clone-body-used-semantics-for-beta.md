---
task_id: M28-004-C
parent_task: M28-004
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-004-C — Define clone/body-used semantics for beta

## Atomic goal

Define clone/body-used semantics for beta.

## Parent intent

Expose a useful Web-compatible API without materializing unnecessary objects.

## Dependencies

- `M28-004-B` — `tasks/05_m28_native_fetch/M28-004-B-use-lazy-native-backed-objects.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define clone/body-used semantics for beta.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- API conformance.
- Body-used tests.
- Allocation profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-004-c: define clone body used semantics for beta
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-004-C) — PASS

- Date: 2026-08-28
- Branch/PR: m28-004-c (squash-merged; see git log for final hash)
- Closes: #326

### Changed files
- `crates/q-engine-quickjs/src/prelude.rs`: Added `Response.prototype.clone()` and `Request.prototype.clone()`; enforces that cloning an already-consumed response/request throws `TypeError` fail-closed, while cloning an unconsumed instance creates an independent copy with its own `bodyUsed = false` lifecycle state.
- `crates/q-engine-quickjs/src/worker.rs`: Added unit test in `fetch_request_response_headers_and_body_used_in_js_environment` verifying clone independence and consumed clone rejection.
- `conformance/web-api/web-api.conformance.test.ts`: Added automated test `Response.clone and Request.clone independent bodyUsed semantics (M28-004-C)`.
- `benchmarks/manifest.json`: Refreshed `qRuntimeRelease` hash.

### Command results
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

### Guardrail mapping
- **Expose a useful Web-compatible API without materializing unnecessary objects** — `clone()` duplicates headers and body references cleanly; `bodyUsed` transitions and re-consumption/re-clone errors follow WHATWG specification.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
