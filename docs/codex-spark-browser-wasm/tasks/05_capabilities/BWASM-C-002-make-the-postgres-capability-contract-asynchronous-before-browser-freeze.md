Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-002-make-the-postgres-capability-contract-asynchronous-before-browser-freeze.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-002 — Make the Postgres capability contract asynchronous before browser freeze

## Atomic goal

Change the public Postgres capability to an async contract that can be implemented safely by both native Postgres and browser-local adapters.

## Parent intent

Expose only browser-safe capabilities, support explicit local persistence, and fail closed for production-only requirements.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Change sql/query operations to return Promise-based results in TypeScript authoring types.
2. Update compiler analysis, generated types/contracts, native bridge, testing helpers, examples, and documentation.
3. Define cancellation, deadline, transaction, result-value, error, and row-count semantics.
4. Provide migration diagnostics or a codemod where synchronous-looking use was previously accepted.
5. Lock the API before browser capability adapters depend on it.

## Acceptance criteria

- [ ] All official examples use await and compile against the new contract.
- [ ] Native Postgres behavior and errors remain covered by integration tests.
- [ ] Generated handler code cannot accidentally serialize an unresolved Promise.
- [ ] Migration guidance identifies every affected public API.
- [ ] A beta API snapshot records the async contract.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Typecheck all packages/examples.
- Compiler fixtures.
- Native Postgres integration tests.
- Migration fixture from old to new API.
- Public API snapshot.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] API diff.
- [ ] Migration guide/codemod transcript.
- [ ] Native integration logs.
- [ ] Updated contract fixtures.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Adding browser PGlite itself.
- Hiding the break through unsafe any types.
- Freezing two incompatible sync/async database APIs.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-002:`.
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
