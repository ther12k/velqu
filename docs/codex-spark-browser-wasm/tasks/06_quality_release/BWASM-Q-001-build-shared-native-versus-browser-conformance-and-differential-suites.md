Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-001-build-shared-native-versus-browser-conformance-and-differential-suites.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-001 — Build shared native-versus-browser conformance and differential suites

## Atomic goal

Prove that compatibility-critical behavior is shared or explicitly classified across native Velqu and Browser-WASM.

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

- `BWASM-K-006` — Verify and package portable-kernel evidence
- `BWASM-R-006` — Verify and package browser-runtime evidence
- `BWASM-B-006` — Verify cache activation, upgrades, rollback, and static deployment
- `BWASM-C-005` — Fail closed for deployment-required and unavailable capabilities

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Create a single fixture corpus for routes, methods, params, query, bodies, headers, schema validation, status declarations, problem responses, Treaty calls, and capability authorization.
2. Run each applicable fixture through native runtime and browser runtime.
3. Canonicalize only approved nondeterministic fields before comparison.
4. Classify each fixture as exact parity, equivalent-by-contract, browser-only, native-only, or unsupported.
5. Fail CI on unreviewed drift.

## Acceptance criteria

- [ ] Every public Browser-WASM behavior has at least one conformance fixture.
- [ ] Route and schema compatibility-critical paths use the Rust/WASM kernel.
- [ ] Differences are linked to a frozen support-matrix entry and owner decision.
- [ ] The suite detects intentional mutation of routing, validation, status, or problem semantics.
- [ ] Results include exact source commit, native binary hash, WASM hash, and browser versions.
- [ ] No broad snapshot update can approve unrelated drift silently.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Native/browser differential runner.
- Mutation/sensitivity tests.
- Contract and Treaty type fixtures.
- Full repository verification.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Machine-readable conformance matrix.
- [ ] Raw native/browser outputs.
- [ ] Mutation-test report.
- [ ] Artifact/toolchain hashes.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Requiring parity for transport features browsers cannot expose.
- Normalizing away substantive semantic differences.
- Using only happy-path routes.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-001:`.
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
