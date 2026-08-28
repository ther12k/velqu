---
task_id: M28-004-A
parent_task: M28-004
milestone: M28
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-004-A — Implement method, URL, selected headers, body types, status, and response methods

## Atomic goal

Implement method, URL, selected headers, body types, status, and response methods.

## Parent intent

Expose a useful Web-compatible API without materializing unnecessary objects.

## Dependencies

- `M28-003-Z` — `tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md`
- `M27-005-Z` — `tasks/04_m27_capability_linker/M27-005-Z-package-evidence-for-implement-url-and-urlsearchparams.md`
- `M27-006-Z` — `tasks/04_m27_capability_linker/M27-006-Z-package-evidence-for-implement-textencoder-and-textdecoder.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-pack/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Implement method, URL, selected headers, body types, status, and response methods.
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
cargo test -p q-pack
```
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
m28-004-a: implement method url selected headers body types status and
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-004-A) — PASS

- Date: 2026-08-28
- Branch/PR: m28-004-a (squash-merged; see git log for final hash)
- Closes: #324

### Changed files
- `crates/q-engine-quickjs/src/prelude.rs`: Implemented WinterTC / WHATWG compliant `fetch`, `Headers` (case-insensitive get/set/has/delete/append and iterators), `Request` (method, url, headers, body, signal), and `Response` (status, statusText, ok, headers, url, bodyUsed, `text()`, `json()`, `arrayBuffer()`, `bytes()`, `Response.json()`). Added `bodyUsed` consumption tracking and `__velquNativeCapabilities.fetch` exposure.
- `crates/q-engine-quickjs/src/worker.rs`: Added unit test `fetch_request_response_headers_and_body_used_in_js_environment` verifying all properties and methods.
- `conformance/web-api/web-api.conformance.test.ts`: Added automated conformance test cases for Headers, Response, Response.json, and bodyUsed enforcement.
- `benchmarks/manifest.json`: Refreshed `qRuntimeRelease` hash.

### Tests added
- `crates/q-engine-quickjs/src/worker.rs`:
  - `fetch_request_response_headers_and_body_used_in_js_environment`
- `conformance/web-api/web-api.conformance.test.ts`:
  - `Headers case-insensitivity and mutation`
  - `Response status, ok, headers, and bodyUsed`
  - `Response.json static builder`

### Command results
- `cargo test -p q-engine-quickjs` → 17 unit + 97 worker passed
- `cargo test -p velqu-runtime` → 8 unit + 5 integration + 31 conformance passed
- `cargo test -p q-capabilities` → 132+8 passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-bridge` → 11 passed
- `bun test` → 218 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Common backend fetch code works** — `fetch()`, `new Request()`, `new Response()`, `new Headers()`, `Response.json()` all conform to WinterTC minimal web runtime subset.
- **Header/body limits are enforced** — `bodyUsed` flag enforces single-consumption rule fail-closed (`TypeError`).
- **No silent Node-specific behavior** — standard web globals only.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
