Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-003-add-optional-pglite-backed-local-sql-capability.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P1`  
Optional: `YES — excluded from the MVP release gate unless an owner decision promotes it before candidate freeze.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-003 — Add optional PGlite-backed local SQL capability

## Atomic goal

Offer an opt-in browser-local SQL adapter for prototype applications without pretending it is production PostgreSQL infrastructure.

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

- `BWASM-C-002` — Make the Postgres capability contract asynchronous before browser freeze
- `BWASM-R-005` — Integrate capability registry and Treaty with the browser runtime
- `BWASM-B-001` — Add compiler target browser-wasm

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Create an optional package/adapter backed by PGlite or an owner-approved equivalent.
2. Support memory and explicitly enabled IndexedDB persistence modes.
3. Map the frozen Velqu Postgres capability subset to the browser adapter.
4. Define unsupported SQL/extensions/concurrency/transaction behavior.
5. Expose database reset/export/import hooks for preview UX and tests.
6. Lazy-load the database WASM/assets so projects without SQL do not pay the payload cost.

## Acceptance criteria

- [ ] Supported SQL fixtures behave according to the documented capability subset.
- [ ] Unsupported operations fail with stable, actionable codes.
- [ ] Persistence is isolated by project and origin namespace.
- [ ] The adapter never claims multi-user durability, production availability, or native Postgres performance.
- [ ] Projects without the capability do not download or instantiate database assets.
- [ ] Database bytes and versions are integrity-bound to the browser build.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- SQL compatibility fixture corpus.
- Memory and IndexedDB persistence tests.
- Project-isolation tests.
- Lazy-load/network trace.
- Export/import/reset tests.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] SQL support matrix.
- [ ] Payload/network measurements.
- [ ] Persistence/isolation logs.
- [ ] Compatibility test results.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Full PostgreSQL parity.
- Shared multi-user database.
- Server-side secrets or remote database credentials.
- Making PGlite mandatory for the core Browser-WASM runtime.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-003:`.
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
