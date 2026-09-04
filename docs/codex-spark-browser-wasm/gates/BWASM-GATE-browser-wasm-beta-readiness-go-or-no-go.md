Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/gates/BWASM-GATE-browser-wasm-beta-readiness-go-or-no-go.md`  
Program: `BWASM`  
Phase: `08_gate` — Release gate  
Mode: `GATE_REVIEW` — Review an exact candidate and issue a binary GO/NO-GO verdict.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-GATE — Browser-WASM beta readiness GO or NO-GO

## Atomic goal

Review one exact Browser-WASM candidate and record an unambiguous GO or NO-GO against the frozen product contract.

## Parent intent

Review one exact candidate and make an unambiguous GO or NO-GO decision.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-D-004` — Ratify support matrix, compatibility claims, and release budgets
- `BWASM-K-006` — Verify and package portable-kernel evidence
- `BWASM-R-006` — Verify and package browser-runtime evidence
- `BWASM-B-006` — Verify cache activation, upgrades, rollback, and static deployment
- `BWASM-C-005` — Fail closed for deployment-required and unavailable capabilities
- `BWASM-Q-001` — Build shared native-versus-browser conformance and differential suites
- `BWASM-Q-002` — Add real-browser CI lanes and supported-browser evidence
- `BWASM-Q-003` — Verify isolated preview-origin and untrusted-code security boundaries
- `BWASM-Q-005` — Set and enforce WASM size, startup, latency, and leak budgets
- `BWASM-Q-006` — Publish Browser-WASM documentation, limitations, and migration guide
- `BWASM-Q-007` — Run an external cleanroom static deployment and offline exercise
- `BWASM-Q-008` — Assemble release evidence, SBOM, checksums, provenance, and candidate packet

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `AGENTS.md`
- `README.md`
- `scripts/verify`
- `.github/workflows/verify.yml`
- `docs/codex-spark-browser-wasm/`

## Steps

1. Verify exact candidate commit, artifact inventory, checksums, SBOM, provenance, package bytes, and evidence index.
2. Review all mandatory child issues and dependency closures.
3. Confirm supported-browser lanes, conformance classes, security review, performance budgets, external cleanroom, static deployment, update/rollback, docs, and residual risks.
4. Confirm native Velqu remains green and its public/runtime behavior has not regressed outside approved migration.
5. Classify every open P0/P1 and obtain explicit owner acceptance where policy allows.
6. Record the final decision, release channel, rollback path, monitoring plan, and claims allowed after the decision.

## Acceptance criteria

- [ ] All mandatory dependencies are closed with evidence from the exact candidate.
- [ ] Zero unresolved P0 blockers exist.
- [ ] Every unresolved P1 is either a documented NO-GO blocker or explicitly accepted with owner, rationale, scope, and expiry.
- [ ] Static deployment works without a Velqu application server on every claimed browser/host shape.
- [ ] Security language does not overclaim hostile-code sandboxing.
- [ ] Optional BWASM-C-003 and BWASM-X-001 do not block MVP unless an owner decision made them mandatory before candidate freeze.
- [ ] The issue ends with exactly one prominent verdict: GO or NO-GO.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Re-run candidate packet validator.
- Verify all hashes and evidence links.
- Spot-check clean install/static deploy.
- Review required CI and security findings.
- Run final full-repository verification.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Signed/recorded gate review.
- [ ] Candidate SHA and artifact hashes.
- [ ] Dependency closure table.
- [ ] P0/P1 and residual-risk disposition.
- [ ] GO release instructions or NO-GO blocker list.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Fixing implementation inside the gate review.
- Using percentage-complete language instead of a verdict.
- Changing support claims without rerunning affected evidence.
- Closing the epic before GO is recorded.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-gate:`.
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
