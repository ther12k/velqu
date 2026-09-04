Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-002-implement-fetch-compatible-browser-dispatcher.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-002 — Implement Fetch-compatible browser dispatcher

## Atomic goal

Dispatch a browser Request through the WASM kernel and return a standards-compliant Response.

## Parent intent

Provide a real browser runtime over Request/Response, isolated handler execution, and Treaty without a listening application server.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-001` — Create @velqu/browser-runtime package and public runtime contract
- `BWASM-K-005` — Implement the Rust Browser Kernel and wasm-bindgen ABI

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Normalize URL, method, headers, query, body, and abort signal at the JS/WASM boundary.
2. Use the Rust/WASM kernel for route selection, parameter extraction, request validation, capability checks, and response validation.
3. Map route misses, method mismatches, malformed input, and internal failures to Velqu problem responses.
4. Support bounded text, JSON, URL-encoded, multipart metadata, and binary body handling according to the frozen support matrix.
5. Preserve deterministic header/status behavior and define unsupported streaming behavior explicitly.

## Acceptance criteria

- [ ] Static and parameterized routes dispatch with production-equivalent precedence.
- [ ] 405/Allow, OPTIONS behavior, HEAD behavior, trailing-slash policy, and duplicate-header policy are fixture-locked.
- [ ] Request and response schema failures use the canonical problem shape.
- [ ] Abort before and during dispatch returns the documented cancellation result.
- [ ] No request path can bypass kernel validation through a JavaScript-only fast path.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Browser unit tests for every supported body/status/header form.
- Shared route corpus against native and browser targets.
- Malformed and oversized request corpus.
- Abort/cancellation tests.
- Real-browser fetch smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Dispatcher conformance report.
- [ ] Native/browser fixture diff.
- [ ] Unsupported-semantics inventory.
- [ ] Raw browser logs.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Opening a TCP listener.
- Pretending Service Worker transport is real network conformance.
- Native Hyper backpressure parity where browsers provide no equivalent.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-002:`.
- Reference this issue ID in commits, PR body, tests, and evidence.
- Avoid generated queue/index churn until implementation and targeted tests are stable.
- If scope expands materially, stop and open a new dependency issue rather than hiding extra work here.

## Stop condition

Stop and hand off when **all** acceptance criteria are demonstrated, the required evidence is attached or committed, canonical verification is green, and no unresolved in-scope P0 remains. If a prerequisite, owner decision, browser limitation, or security claim blocks truthful completion, record the exact blocker and leave this issue open.

## Handoff format

```text
Issue:
Candidate commit:
Files changed:
Commands run:
Targeted tests:
Full verification:
Artifacts and SHA-256:
Browser/OS/toolchain:
Acceptance criteria:
Known limitations:
Residual risks:
Follow-up issue links:
```
