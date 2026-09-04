Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-005-fail-closed-for-deployment-required-and-unavailable-capabilities.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-005 — Fail closed for deployment-required and unavailable capabilities

## Atomic goal

Turn unsupported production integrations into explicit build/runtime outcomes that can drive deployment UX and pricing.

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
- `BWASM-B-003` — Enforce browser import policy with source-located diagnostics
- `BWASM-C-001` — Implement browser-safe timer, crypto, logging, and restricted fetch capabilities
- `BWASM-C-002` — Make the Postgres capability contract asynchronous before browser freeze
- `BWASM-C-004` — Add namespaced IndexedDB KV persistence capability

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Define capability portability states: browser, browser-and-native, simulated, deployment-required, and forbidden.
2. Record states and required configuration in compiler artifacts.
3. Fail at build time when capability usage is statically known and impossible for the selected target.
4. Fail before side effects at runtime when availability depends on host configuration.
5. Return a stable deployment-required problem shape containing capability ID, route ID, reason code, and safe remediation metadata.
6. Expose a machine-readable deployment-requirements summary for CLI and app-builder UI.

## Acceptance criteria

- [ ] Secrets, real remote Postgres credentials, payments, email delivery, public webhooks, cron, and durable queues are never silently mocked unless an explicit simulation profile is selected.
- [ ] Deployment-required responses contain no secret values or provider-specific private data.
- [ ] Build, inspect, runtime, and Treaty surfaces agree on the capability classification.
- [ ] The app-builder can determine whether deployment is required without executing the route.
- [ ] Capability checks happen before handler or adapter side effects.
- [ ] Unknown classifications fail closed.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Capability classification golden fixtures.
- Build-time and runtime negative tests.
- Side-effect ordering tests.
- Treaty problem decoding tests.
- CLI JSON requirements snapshot.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Capability portability registry.
- [ ] Problem schema and examples.
- [ ] Build/runtime consistency report.
- [ ] No-side-effect proof logs.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Implementing billing or pricing logic in Velqu core.
- Returning fake successful payment/email/database operations by default.
- Treating every unavailable capability as a generic 500.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-005:`.
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
