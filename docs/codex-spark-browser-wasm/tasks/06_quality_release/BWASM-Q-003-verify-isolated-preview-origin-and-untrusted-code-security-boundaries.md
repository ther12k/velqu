Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-003-verify-isolated-preview-origin-and-untrusted-code-security-boundaries.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-003 — Verify isolated preview-origin and untrusted-code security boundaries

## Atomic goal

Demonstrate that generated preview code cannot cross the documented editor, credential, network, storage, or project boundaries.

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

- `BWASM-R-004` — Execute handlers in isolated Workers with cancellation and hard recovery
- `BWASM-B-004` — Add Service Worker adapter and static-host bootstrap
- `BWASM-C-001` — Implement browser-safe timer, crypto, logging, and restricted fetch capabilities
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Build a production-shaped two-origin fixture: editor/control plane and isolated preview origin.
2. Apply sandboxed iframe policy, strict CSP, Permissions Policy, COOP/COEP only where required, and validated postMessage schemas.
3. Attempt DOM escape, parent access, credential/cookie access, provider-key theft, network exfiltration, import bypass, storage crossover, Service Worker scope escape, and message confusion.
4. Test malicious logs, stack traces, redirects, URLs, headers, HTML, and oversized messages.
5. Commission an independent review of the implemented threat model and claims.

## Acceptance criteria

- [ ] Preview code cannot read editor-origin DOM, storage, authentication material, or provider secrets.
- [ ] Default network policy blocks unapproved exfiltration paths.
- [ ] Service Worker scope cannot control the editor/control-plane origin.
- [ ] All cross-origin messages are origin-, schema-, project-, and invocation-validated.
- [ ] Known browser limitations and residual risks are explicit.
- [ ] No document claims a hostile-code sandbox unless the independent review supports that exact claim.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Adversarial browser suite.
- CSP/Permissions Policy reporting tests.
- Cross-origin storage/cookie tests.
- Service Worker scope attacks.
- Dependency/security scan.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Threat-model verification report.
- [ ] Independent reviewer findings and disposition.
- [ ] CSP/network traces.
- [ ] Residual-risk register.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Assuming WebAssembly or Workers are automatically secure sandboxes.
- Testing only same-origin development mode.
- Suppressing exploit evidence after a fix.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-003:`.
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
