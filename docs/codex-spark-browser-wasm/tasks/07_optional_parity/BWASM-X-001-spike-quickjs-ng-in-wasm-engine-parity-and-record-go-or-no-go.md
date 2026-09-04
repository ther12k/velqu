Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/07_optional_parity/BWASM-X-001-spike-quickjs-ng-in-wasm-engine-parity-and-record-go-or-no-go.md`  
Program: `BWASM`  
Phase: `07_optional_parity` — Optional QuickJS-NG WASM parity  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P1`  
Optional: `YES — excluded from the MVP release gate unless an owner decision promotes it before candidate freeze.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-X-001 — Spike QuickJS-NG-in-WASM engine parity and record GO or NO-GO

## Atomic goal

Determine whether executing handlers in QuickJS-NG compiled to browser WASM materially improves Velqu parity enough to justify its cost.

## Parent intent

Investigate closer QuickJS-NG engine parity without silently making it a prerequisite for the practical MVP.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-K-005` — Implement the Rust Browser Kernel and wasm-bindgen ABI
- `BWASM-R-004` — Execute handlers in isolated Workers with cancellation and hard recovery
- `BWASM-D-004` — Ratify support matrix, compatibility claims, and release budgets

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `crates/q-engine/src/lib.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/Cargo.toml`
- `packages/browser-runtime/`

## Steps

1. Identify a maintainable QuickJS-NG browser-WASM build path and exact engine version strategy.
2. Implement a bounded prototype that loads one compiled handler bundle through the frozen handler ABI.
3. Compare JavaScript semantics, startup, payload, memory, cancellation, debugging, CSP requirements, and maintenance risk against native browser Worker execution.
4. Test interoperability with the Rust/WASM kernel and browser capability bridge.
5. Record blockers in rquickjs/upstream/toolchain integration without hiding them behind a custom fork.
6. Produce a scored GO/NO-GO decision and an adoption plan only if thresholds are met.

## Acceptance criteria

- [ ] The spike uses reproducible source/toolchain references and does not masquerade as production support.
- [ ] Engine-version mismatch with native Velqu is measured and explicitly classified.
- [ ] Payload/startup/memory costs are compared using raw evidence.
- [ ] Infinite loop/cancellation/recovery behavior is demonstrated.
- [ ] A GO decision identifies ownership, update cadence, security review, release budget, and fallback behavior.
- [ ] A NO-GO decision leaves the default Worker-based Browser-WASM target unaffected.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Engine semantic fixture corpus.
- Cold/warm benchmark.
- Loop/cancellation/recovery tests.
- CSP and browser matrix smoke.
- Prototype reproducibility build.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Prototype source and artifact hashes.
- [ ] Version/toolchain inventory.
- [ ] Comparative benchmark and semantic report.
- [ ] GO/NO-GO decision record.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Making QuickJS-WASM a hidden dependency of the MVP.
- Claiming same-engine parity with a different QuickJS-NG version.
- Maintaining an unreviewed permanent fork by default.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-x-001:`.
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
