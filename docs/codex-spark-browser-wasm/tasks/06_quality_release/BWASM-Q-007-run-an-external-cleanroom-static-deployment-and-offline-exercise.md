Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-007-run-an-external-cleanroom-static-deployment-and-offline-exercise.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `EVIDENCE` — Package evidence from one exact candidate; do not mix implementation and attestation.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-007 — Run an external cleanroom static deployment and offline exercise

## Atomic goal

Prove that an external consumer can build and deploy a useful Velqu Browser-WASM application using only public artifacts and documentation.

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

- `BWASM-Q-002` — Add real-browser CI lanes and supported-browser evidence
- `BWASM-Q-003` — Verify isolated preview-origin and untrusted-code security boundaries
- `BWASM-Q-005` — Set and enforce WASM size, startup, latency, and leak budgets
- `BWASM-Q-006` — Publish Browser-WASM documentation, limitations, and migration guide

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Use a fresh external repository and participant/agent not involved in implementation.
2. Install release-candidate packages/artifacts without workspace links or source-path fallbacks.
3. Build a bounded CRUD-style app with route params, schema validation, Treaty, timer/logging, local KV, and at least one deployment-required capability.
4. Deploy to a generic static HTTPS host under a non-root base path.
5. Exercise first load, forms/fetch, persistence, offline reload, update, rollback, reset/export, and deployment-required UX.
6. Record setup failures, docs gaps, iterations, ambiguity, and framework defects separately.

## Acceptance criteria

- [ ] The app works from registry/candidate artifacts only.
- [ ] No Velqu application server is running after static deployment.
- [ ] The participant can distinguish browser-local behavior from native production behavior.
- [ ] Local data persists according to the documented policy and remains project-isolated.
- [ ] Deployment-required behavior is explicit and machine-readable.
- [ ] Every blocking defect is fixed and re-proven or leaves the candidate NO-GO.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Clean clone/install/build transcript.
- Static host network/process inventory.
- Browser user-journey suite.
- Offline/update/rollback rehearsal.
- External usability notes.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] External repository commit.
- [ ] Package/artifact lock and hashes.
- [ ] Deployment recording and network trace.
- [ ] Participant report and defect disposition.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Using monorepo workspaces or unpublished package sources.
- Treating implementer familiarity as usability evidence.
- Replacing failed journeys with manual claims.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-007:`.
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
