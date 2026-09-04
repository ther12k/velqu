Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-006-verify-and-package-browser-runtime-evidence.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `EVIDENCE` — Package evidence from one exact candidate; do not mix implementation and attestation.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-006 — Verify and package browser-runtime evidence

## Atomic goal

Independently verify the JavaScript/Worker browser runtime at one exact commit.

## Parent intent

Provide a real browser runtime over Request/Response, isolated handler execution, and Treaty without a listening application server.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-001` — Create @velqu/browser-runtime package and public runtime contract
- `BWASM-R-002` — Implement Fetch-compatible browser dispatcher
- `BWASM-R-003` — Define and emit the browser handler-bundle contract
- `BWASM-R-004` — Execute handlers in isolated Workers with cancellation and hard recovery
- `BWASM-R-005` — Integrate capability registry and Treaty with the browser runtime
- `BWASM-K-006` — Verify and package portable-kernel evidence

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Freeze a clean candidate and install packages from produced tarballs in a clean fixture.
2. Run dispatcher, Worker, Treaty, capability, abort, and negative-path browser suites.
3. Audit bundles for forbidden imports and accidental secret/global access.
4. Inspect public exports, ABI versions, source maps, and package metadata.
5. Package raw logs, browser/toolchain versions, hashes, and reviewer conclusions.

## Acceptance criteria

- [ ] All R-phase criteria are independently demonstrated.
- [ ] Evidence is generated from package bytes intended for release.
- [ ] Browser execution uses the WASM kernel rather than an unverified JavaScript matcher/validator.
- [ ] No unresolved runtime P0 remains.
- [ ] Every known semantic difference is documented and linked to owner acceptance or a blocking issue.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Clean tarball consumer.
- Real-browser suite.
- Bundle/import audit.
- Adversarial Worker suite.
- Full repository verify.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Runtime verification report.
- [ ] Raw browser logs/traces.
- [ ] Tarball and bundle hashes.
- [ ] Browser/OS/toolchain manifest.
- [ ] Reviewer sign-off.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Implementing missing features during evidence work except evidence-harness defects.
- Mixing commits or locally modified package bytes.
- Waiving failures without a linked decision.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-006:`.
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
