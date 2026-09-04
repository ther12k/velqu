Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-005-integrate-capability-registry-and-treaty-with-the-browser-runtime.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-005 — Integrate capability registry and Treaty with the browser runtime

## Atomic goal

Make capability injection and typed Treaty calls work through the same browser runtime boundary.

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

- `BWASM-R-002` — Implement Fetch-compatible browser dispatcher
- `BWASM-R-004` — Execute handlers in isolated Workers with cancellation and hard recovery
- `BWASM-K-005` — Implement the Rust Browser Kernel and wasm-bindgen ABI

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Define a browser capability registry keyed by declared capability IDs and versions.
2. Pass only declared capability handles into each handler context.
3. Add a Treaty transport/dispatch adapter backed by BrowserRuntime.fetch or direct typed dispatch without semantic bypass.
4. Preserve declared status narrowing and canonical problem decoding.
5. Reject missing, incompatible, undeclared, or deployment-only capability use before side effects.

## Acceptance criteria

- [ ] Treaty clients call browser routes with the same route IDs and status typing as native builds.
- [ ] Capability authorization happens before handler side effects.
- [ ] A route cannot access a capability omitted from its compiled declaration.
- [ ] Version mismatch and unavailable capability failures are machine-readable.
- [ ] Direct Treaty mode and Request/Response mode share validation and routing semantics.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Treaty compile-time fixtures.
- Runtime capability allow/deny matrix.
- Side-effect-before-authorization regression test.
- Direct-vs-fetch differential tests.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Treaty consumer fixture.
- [ ] Capability registry manifest.
- [ ] Negative-test logs.
- [ ] Native/browser behavior diff.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Implementing every capability adapter.
- Using @velqu/testing as the production browser runtime.
- Silently mocking production-only integrations.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-005:`.
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
