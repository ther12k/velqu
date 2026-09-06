Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-004-execute-handlers-in-isolated-workers-with-cancellation-and-hard-recovery.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

---

# BWASM-R-004 — Execute handlers in isolated Workers with cancellation and hard recovery

## Atomic goal

Run generated handlers outside the editor/UI realm and provide deterministic cancellation and recovery.

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

- `BWASM-R-003` — Define and emit the browser handler-bundle contract
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Create a dedicated Worker execution host and validated message protocol.
2. Use one-shot or pooled workers according to the threat model and document the chosen lifecycle.
3. Propagate request deadlines and AbortSignal state.
4. Terminate and replace workers that exceed time, message, log, or output budgets.
5. Redact stack paths and bound console/log forwarding.
6. Ensure runtime state cannot leak across projects unless an adapter explicitly declares persistence.

## Acceptance criteria

- [ ] Infinite loops are stopped by worker termination and the runtime remains usable afterward.
- [ ] Late messages from a terminated/stale worker are ignored.
- [ ] Cross-project invocation IDs cannot collide or receive another project's result.
- [ ] Uncloneable/oversized payloads become structured errors.
- [ ] No parent DOM, editor token, or provider credential is reachable through the execution protocol.
- [ ] Documentation states that Worker isolation is not by itself a proven hostile-code sandbox.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Infinite-loop and high-log-volume adversarial tests.
- Abort race tests.
- Worker crash/restart tests.
- Cross-project leakage tests.
- Real-browser CSP/isolation smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Threat-model test matrix.
- [ ] Worker protocol schema.
- [ ] Crash/timeout raw logs.
- [ ] Isolation screenshots or browser traces.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Claiming hard heap enforcement when the browser cannot prove it.
- Executing generated code on the editor origin.
- Passing secrets into the preview worker.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-004:`.
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

## Result (2026-09-06)

- Issue: BWASM-R-004 (#1245)
- Candidate commit: see PR; report `docs/reports/bwasm-r-004-worker-execution.md`
- WorkerHost: validated message protocol v1, host-enforced deadlines w/ kill-and-replace (host usable after any crash), session-scoped results (cross-project theft blocked), bounded logs (64 lines/invocation) + 1 MiB result cap (structured errors), stack redaction in the Worker bootstrap, AbortSignal before+mid-dispatch.
- Isolation honesty in bootstrap source AND report: Worker isolation is NOT by itself a hostile-code sandbox.
- Tests: 13 new (infinite loop+recovery, stale/foreign drops, abort races, log flood, oversized, crash/restart) — package 49/49; typecheck clean; browser bundle builds.
- Real-browser CSP smoke = Q-002. R-005 wires this host into capability-bridge/Treaty execution.
- Follow-ups: R-005 (#1246), R-006 (#1247).
```
