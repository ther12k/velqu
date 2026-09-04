Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/04_build_deploy/BWASM-B-004-add-service-worker-adapter-and-static-host-bootstrap.md`  
Program: `BWASM`  
Phase: `04_build_deploy` — Compiler, artifacts, and static deployment  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-B-004 — Add Service Worker adapter and static-host bootstrap

## Atomic goal

Expose a built Velqu browser application through scoped Service Worker fetch interception on static hosting.

## Parent intent

Turn the runtime into deterministic static artifacts that can be built, inspected, updated, rolled back, and hosted generically.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-002` — Implement Fetch-compatible browser dispatcher
- `BWASM-B-002` — Define content-addressed browser artifact manifest and loader
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/compiler/src/index.ts`
- `packages/compiler/src/extract.ts`
- `packages/compiler/src/emit.ts`
- `packages/cli/src/index.ts`
- `packages/browser-runtime/`
- `.github/workflows/verify.yml`

## Steps

1. Provide generated bootstrap code that registers, installs, verifies, and activates the runtime under an explicit scope.
2. Route only owned application requests to BrowserRuntime.fetch.
3. Define navigation, API, asset, form, redirect, cache, and offline behavior.
4. Avoid intercepting editor/auth/model-gateway traffic outside the preview scope.
5. Provide a Worker-only/injected-fetch fallback for unsupported or embedded environments.

## Acceptance criteria

- [ ] A static HTTPS deployment serves application routes without a Velqu application server.
- [ ] Scope escape and unrelated-origin requests are not intercepted.
- [ ] First install, reload, update, offline reload, and unregister behaviors are deterministic.
- [ ] Redirects and forms behave according to the support matrix.
- [ ] Failure to register Service Worker produces an actionable fallback/error, not a hanging preview.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Chromium/Firefox/WebKit registration and fetch tests according to the frozen matrix.
- Root/subpath/static-host fixtures.
- Offline/reload/update tests.
- Scope-escape tests.
- Worker-only fallback smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Static deployment recording.
- [ ] Service Worker lifecycle logs.
- [ ] Scope test results.
- [ ] Fallback-mode evidence.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Claiming that static hosting means zero hosting.
- Intercepting provider gateway or editor control-plane requests.
- Using Service Worker as the only test path.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-b-004:`.
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
