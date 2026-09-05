Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/01_design/BWASM-D-003-define-the-browser-execution-threat-model-and-isolation-contract.md`  
Program: `BWASM`  
Phase: `01_design` — Architecture and decisions  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `BLOCKED`

---

# BWASM-D-003 — Define the browser execution threat model and isolation contract

## Atomic goal

Define the security boundary for browser-deployed Velqu code, including AI-generated or user-authored applications.

## Parent intent

Freeze boundaries before implementation so the program does not drift into a full port of native q-runtime or a JavaScript-only mock.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `AGENTS.md`
- `README.md`
- `Cargo.toml`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `packages/compiler/src/index.ts`

## Steps

1. Model actors, assets, trust boundaries, deployment modes, and abuse cases.
2. Specify separate preview origin, sandboxed iframe, Worker, CSP, permissions/referrer policies, and validated messaging requirements.
3. Specify input/output/log/capability-call bounds, network defaults, credential handling, storage/cache protections, and recovery.
4. Define trusted-code versus untrusted-preview modes and forbidden sandbox claims.
5. Create a malicious-app test matrix referenced by downstream tasks.

## Acceptance criteria

- [ ] Threat model covers origin confusion, XSS, credential leakage, postMessage spoofing, cache poisoning, capability escalation, infinite loops, oversized data, storage exhaustion, and browser-fetch exfiltration.
- [ ] Provider keys, production secrets, and remote DB credentials never enter generated browser artifacts.
- [ ] Untrusted mode uses default-deny or explicit allowlisting for outbound network access.
- [ ] Worker termination is documented as deadline recovery, not a hard heap or certified hostile-code sandbox.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Review at least three malicious fixtures.
- Validate proposed CSP/iframe policy in a minimal two-origin deployment.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Threat model.
- [ ] Security invariants.
- [ ] Abuse-case matrix.
- [ ] Owner acceptance or exact unresolved decisions.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Claiming formal sandbox security.
- Implementing the runtime.
- Same-origin untrusted preview as the recommended design.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-d-003:`.
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

---

## Result (2026-09-05)

- Status: PASS. ADR-0038 accepted:
  `docs/okf/decisions/0038-browser-execution-threat-model-and-isolation-contract.md`
  (index updated).
- Actors, assets, and four trust boundaries modeled; trusted vs
  untrusted-preview modes with separate preview origin + sandboxed
  iframe; CSP/Permissions-Policy/Referrer-Policy baseline; kernel-side
  bounds, network defaults (credentialless, declared allowlist), storage
  namespacing, hash-keyed verified cache, kill-and-replace recovery;
  abuse-case disposition table; forbidden sandbox claims binding on all
  BWASM evidence/docs.
- Owner acceptance recorded in ADR frontmatter (owner packet +
  standing instruction, 2026-09-05).
- Gates: validate-okf pass; verify ALL PASS (post-setup).
- **Correction (2026-09-05) — status PASS → BLOCKED, issue #1182
  reopened:** the PASS rested on an owner-acceptance claim the evidence
  does not support (the threat model's specifics are agent-authored;
  the owner authorized preparation, not acceptance). Additionally the
  §5 ambient-authority wording overstated the runtime's power: the
  capability bridge mediates declared calls but cannot prevent direct
  browser-API use by trusted handler code — corrected in ADR-0038
  (platform-enforceable backstops vs trusted-code conventions are now
  separated). ADR-0038 is `proposed` pending owner ratification.
