Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/01_design/BWASM-D-004-ratify-support-matrix-compatibility-claims-and-release-budgets.md`  
Program: `BWASM`  
Phase: `01_design` — Architecture and decisions  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-D-004 — Ratify support matrix, compatibility claims, and release budgets

## Atomic goal

Set evidence-bound browser support, compatibility, size, startup, latency, memory, offline, and update targets.

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
- `BWASM-D-002` — Produce the wasm32 portability baseline and dependency split map
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `AGENTS.md`
- `README.md`
- `Cargo.toml`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/lib.rs`

## Steps

1. Classify browser/OS/device lanes as tested, supported, experimental, or out of scope.
2. Define budgets for base WASM, JS glue, handler bundle, optional capability bundles, and total initial transfer.
3. Define cold/warm startup, request latency, memory-growth, and repeated-lifecycle measurement procedures.
4. Define offline, cache activation, update/rollback, and artifact-version compatibility expectations.
5. List allowed intentional native/browser differences.

## Acceptance criteria

- [ ] Every budget names a representative environment, sample procedure, and noise/waiver policy.
- [ ] No universal browser/mobile support is claimed without lanes.
- [ ] No native/competitor performance comparison is allowed without matched methodology.
- [ ] Budget/support changes require evidence and owner approval.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Run a small measurability prototype only; do not treat it as final performance evidence.
- Review all targets against D-002 and D-003.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Support matrix.
- [ ] Machine-readable budgets.
- [ ] Known-limitations baseline.
- [ ] Owner decision record.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Optimization work.
- Arbitrary marketing numbers.
- Using one laptop/browser result as a support claim.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-d-004:`.
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
