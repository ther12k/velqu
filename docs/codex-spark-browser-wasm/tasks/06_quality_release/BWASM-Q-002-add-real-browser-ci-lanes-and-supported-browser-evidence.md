Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-002-add-real-browser-ci-lanes-and-supported-browser-evidence.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-002 — Add real-browser CI lanes and supported-browser evidence

## Atomic goal

Run Browser-WASM tests in real browsers and bind support claims to maintained CI evidence.

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

- `BWASM-Q-001` — Build shared native-versus-browser conformance and differential suites
- `BWASM-B-006` — Verify cache activation, upgrades, rollback, and static deployment
- `BWASM-D-004` — Ratify support matrix, compatibility claims, and release budgets

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Add wasm-bindgen/browser and end-to-end lanes for the browsers/platforms selected in BWASM-D-004.
2. Run kernel, dispatcher, Worker, Service Worker, IndexedDB, cache/update, and static-host smoke suites.
3. Separate required lanes from allowed-failure experimental lanes.
4. Capture browser engine/version, OS/architecture, feature flags, and artifacts.
5. Add scheduled or release-candidate coverage where the normal PR matrix is intentionally smaller.

## Acceptance criteria

- [ ] Every claimed supported browser has a blocking evidence lane.
- [ ] A browser absent from evidence is marked unverified/unsupported rather than implicitly supported.
- [ ] CI tests actual emitted release-like artifacts, not development source imports only.
- [ ] Failures upload enough logs/traces/artifacts for diagnosis.
- [ ] Experimental lanes cannot satisfy a release gate.
- [ ] Matrix ownership and update cadence are documented.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Real-browser kernel tests.
- Playwright/WebDriver end-to-end suite.
- Static-host and Service Worker suite.
- Scheduled compatibility run.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] CI workflow definitions.
- [ ] Representative green run links/logs.
- [ ] Browser/OS matrix manifest.
- [ ] Failure artifact example.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Browser support claims based solely on standards documentation.
- Headless-only claims where headed behavior materially differs.
- Making every browser/version blocking without an owner decision.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-002:`.
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
