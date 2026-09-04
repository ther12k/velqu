Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-005-set-and-enforce-wasm-size-startup-latency-and-leak-budgets.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-005 — Set and enforce WASM size, startup, latency, and leak budgets

## Atomic goal

Turn browser feasibility into measurable release budgets rather than an unbounded payload/performance claim.

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

1. Measure compressed/uncompressed kernel, glue, handler, and optional-capability sizes separately.
2. Measure cold/warm load, verification, compilation/instantiation, first request, steady request, Worker restart, and update activation.
3. Measure memory growth across repeated requests, failures, aborts, worker restarts, and route/schema corpora.
4. Run on the device/browser tiers selected in BWASM-D-004.
5. Add blocking budgets and a documented process for intentional budget changes.

## Acceptance criteria

- [ ] Core projects do not download optional SQL or parity-engine assets.
- [ ] Every blocking metric has a command, raw sample set, percentile/statistic definition, environment, and threshold.
- [ ] CI or candidate verification detects material size/startup regressions.
- [ ] No unbounded memory growth remains in the defined soak scenario.
- [ ] Results are not represented as native-runtime throughput benchmarks.
- [ ] Budget exceptions require an owner decision and before/after evidence.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Artifact size gate.
- Cold/warm browser benchmark harness.
- Repeated-request and Worker-restart soak.
- Memory/leak instrumentation.
- Optional-capability lazy-load trace.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Raw samples and statistics.
- [ ] Environment/device/browser manifest.
- [ ] Artifact size inventory.
- [ ] Regression-gate output.
- [ ] Accepted budget-change decisions.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Marketing benchmarks without reproducible raw data.
- Comparing browser-local requests directly to network-server throughput.
- One high-end desktop as the only device tier.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-005:`.
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
