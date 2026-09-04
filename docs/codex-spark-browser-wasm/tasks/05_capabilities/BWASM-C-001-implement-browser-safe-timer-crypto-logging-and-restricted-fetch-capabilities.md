Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-001-implement-browser-safe-timer-crypto-logging-and-restricted-fetch-capabilities.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-001 — Implement browser-safe timer, crypto, logging, and restricted fetch capabilities

## Atomic goal

Provide the mandatory browser capability baseline with explicit security and resource policy.

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

- `BWASM-R-005` — Integrate capability registry and Treaty with the browser runtime
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Implement timer using browser scheduling with deadline and cancellation propagation.
2. Implement Web Crypto-backed random/digest primitives only where semantics match the native contract.
3. Implement bounded structured logging with redaction, levels, correlation IDs, and host forwarding.
4. Implement outbound fetch with default-deny policy, origin/method/header/body/response limits, timeout, redirect, and credential controls.
5. Version and declare each adapter in the browser artifact manifest.

## Acceptance criteria

- [ ] No adapter exposes editor credentials, ambient cookies, storage, DOM, or unrestricted network access.
- [ ] Timer and fetch stop or discard work after cancellation according to the contract.
- [ ] Crypto mismatch with native algorithms is rejected or documented; it is not silently substituted.
- [ ] Log and response floods are bounded and produce structured limit errors.
- [ ] Fetch credentials default to omit and redirects cannot escape policy.
- [ ] Capability availability is introspectable before handler execution.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Capability unit tests.
- Cancellation and timeout races.
- Network allow/deny/redirect/credential matrix.
- Log-flood and oversized-response tests.
- Native/browser contract fixtures where semantics overlap.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Capability conformance matrix.
- [ ] Network-policy traces.
- [ ] Limit/cancellation logs.
- [ ] Adapter manifest examples.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Giving previews unrestricted internet access.
- Implementing server secrets in browser.
- Claiming cryptographic equivalence without shared vectors.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-001:`.
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
