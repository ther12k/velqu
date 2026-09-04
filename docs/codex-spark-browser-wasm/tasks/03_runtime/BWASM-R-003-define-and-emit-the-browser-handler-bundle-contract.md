Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-003-define-and-emit-the-browser-handler-bundle-contract.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-003 — Define and emit the browser handler-bundle contract

## Atomic goal

Define the deterministic contract connecting compiled application handlers to the browser runtime.

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
- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Define stable route-ID-to-handler module mapping and bundle metadata.
2. Specify handler invocation input/output, context shape, status declarations, and error serialization.
3. Require generated handler modules to register through a narrow runtime API instead of ambient globals.
4. Version the handler ABI independently from package version where appropriate.
5. Provide deterministic bundle ordering, source maps, and source-location metadata.

## Acceptance criteria

- [ ] The same source produces byte-stable metadata under a normalized environment.
- [ ] Unknown ABI versions fail closed with an actionable diagnostic.
- [ ] Duplicate or missing handler IDs are rejected before execution.
- [ ] Source locations survive into runtime errors without exposing private host paths.
- [ ] Handlers cannot silently register undeclared routes or statuses.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Compiler fixture snapshots.
- Duplicate/missing/unknown ABI negative tests.
- Reproducible-build test.
- Source-map browser smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Handler ABI specification.
- [ ] Golden bundle fixtures.
- [ ] Reproducibility hashes.
- [ ] Diagnostic snapshots.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Choosing a hostile-code security claim.
- Embedding QuickJS in the MVP.
- Supporting arbitrary dynamic imports.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-003:`.
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
