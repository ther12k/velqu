Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-004-add-namespaced-indexeddb-kv-persistence-capability.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-004 — Add namespaced IndexedDB KV persistence capability

## Atomic goal

Provide the small mandatory local-persistence primitive for browser previews without requiring a SQL engine.

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

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Define a versioned async KV capability with get, set, delete, list/prefix, clear, and optional transaction/batch semantics.
2. Implement a memory adapter and a namespaced IndexedDB adapter.
3. Define serialization, quotas, schema/version migration, cancellation, and error behavior.
4. Namespace all data by application/build/project identity according to the product contract.
5. Provide explicit export, reset, and garbage-collection controls.

## Acceptance criteria

- [ ] Memory and IndexedDB adapters pass one shared contract suite.
- [ ] One project cannot enumerate or read another project's keys.
- [ ] Quota, serialization, migration, and blocked-database failures are structured.
- [ ] Upgrading an application does not silently erase data outside declared migration policy.
- [ ] Private/incognito or unavailable IndexedDB conditions have a documented fallback/error.
- [ ] No preview data is represented as production-durable or multi-user.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Shared adapter contract tests.
- Real-browser IndexedDB tests.
- Cross-project namespace tests.
- Quota/migration/blocked-upgrade tests.
- Export/reset/GC tests.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] KV API specification.
- [ ] Browser traces.
- [ ] Migration fixtures.
- [ ] Isolation and quota results.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Distributed consistency.
- Cross-device synchronization.
- Using localStorage for unbounded application state.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-004:`.
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
