Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-006-publish-browser-wasm-documentation-limitations-and-migration-guide.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-006 — Publish Browser-WASM documentation, limitations, and migration guide

## Atomic goal

Give external users an accurate end-to-end path and prevent Browser-WASM preview from being mistaken for native production equivalence.

## Parent intent

Prove cross-target semantics, browser support, security, performance, documentation, and clean external usability.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-B-005` — Add CLI build, preview, inspect, and export workflows
- `BWASM-C-005` — Fail closed for deployment-required and unavailable capabilities
- `BWASM-Q-004` — Add Browser-WASM observability and developer diagnostics

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Document architecture, build/deploy flow, static-host requirements, base paths, HTTPS/Service Worker requirements, and fallback mode.
2. Document supported APIs, bodies, streaming, headers, cookies, persistence, capabilities, browser matrix, limits, and diagnostics.
3. Document browser preview versus native deployment semantics and evidence classes.
4. Provide migration guidance for async Postgres and deployment-required capabilities.
5. Add quickstart, troubleshooting, security model, upgrade/rollback, and app-builder integration examples.
6. Add an explicit unsupported/experimental section, including QuickJS-WASM status.

## Acceptance criteria

- [ ] A new user can build, statically host, exercise, inspect, update, and reset a sample without private repository knowledge.
- [ ] No doc says 'serverless', 'zero server', 'sandbox', 'Postgres compatible', or 'production parity' without precise qualification.
- [ ] Every public diagnostic code and capability state is documented.
- [ ] Examples are executed in CI from published/generated artifacts.
- [ ] Native deployment remains the documented path for production-only capabilities.
- [ ] Support claims exactly match BWASM-D-004 and candidate evidence.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Documentation link/checker.
- Executable quickstart.
- Migration fixture.
- Terminology/claim lint.
- Clean-reader usability run.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Rendered documentation output.
- [ ] Quickstart transcript.
- [ ] Migration diff.
- [ ] Claim-audit report.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Promising unsupported browser/runtime behavior.
- Using internal design history as a prerequisite.
- Hiding limitations in a separate obscure document.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-006:`.
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
