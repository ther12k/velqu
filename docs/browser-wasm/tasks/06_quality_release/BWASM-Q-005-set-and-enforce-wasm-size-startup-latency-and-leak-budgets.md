Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-005-set-and-enforce-wasm-size-startup-latency-and-leak-budgets.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

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

- [x] Core projects do not download optional SQL or parity-engine assets.
- [x] Every blocking metric has a command, raw sample set, percentile/statistic definition, environment, and threshold.
- [x] CI or candidate verification detects material size/startup regressions.
- [x] No unbounded memory growth remains in the defined soak scenario.
- [x] Results are not represented as native-runtime throughput benchmarks.
- [x] Budget exceptions require an owner decision and before/after evidence.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Artifact size gate.
- Cold/warm browser benchmark harness.
- Repeated-request and Worker-restart soak.
- Memory/leak instrumentation.
- Optional-capability lazy-load trace.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [x] Raw samples and statistics.
- [x] Environment/device/browser manifest.
- [x] Artifact size inventory.
- [x] Regression-gate output.
- [x] Accepted budget-change decisions.

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

## Result

```text
Issue: BWASM-Q-005 (#1280)
Candidate commit: pending commit on bwasm-q-005
Files changed:
  packages/browser-runtime/test/budgets.test.ts
  scripts/browser-budgets-rehearsal.py
  docs/codex-spark-browser-wasm/evidence/q-005/rehearsal-report.json
  docs/codex-spark-browser-wasm/evidence/q-005/environment-manifest.json
  docs/codex-spark-browser-wasm/evidence/q-005/artifact-size-inventory.md
  docs/codex-spark-browser-wasm/evidence/q-005/budget-report.md
  docs/codex-spark-browser-wasm/evidence/q-005/budget-disposition.md
  docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-005-set-and-enforce-wasm-size-startup-latency-and-leak-budgets.md
Commands run:
  bun test packages/browser-runtime/test/budgets.test.ts
  python3 scripts/browser-budgets-rehearsal.py --browser chromium --out docs/codex-spark-browser-wasm/evidence/q-005/rehearsal-report.json
  bun run typecheck
  cargo build -p q-bytecode-tool
  bun packages/cli/src/index.ts build --project examples/proof
  RUSTFLAGS="--remap-path-prefix=$(pwd)=/velqu-src" cargo build --release -p velqu-runtime
  unshare -rn bash -c 'ip link set lo up; ./scripts/verify'
Targeted tests:
  - packages/browser-runtime/test/budgets.test.ts: 5 pass / 0 fail
  - scripts/browser-budgets-rehearsal.py: 8 checks all PASS
Full verification: ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
Artifacts and SHA-256:
  - q_browser_kernel_bg.wasm: a5b33a56ae0e65b08d88308b9423165f0d572c2852d20c7169746d05c79b08e5 (400,229 B brotli-11 <= 512,000 B budget)
  - total initial transfer: 453,771 B brotli-11 <= 1,048,576 B budget
Browser/OS/toolchain: Chromium 134.0.6998.35, Linux x86_64, Bun 1.4.0
Acceptance criteria:
  - Core projects do not download optional SQL or parity-engine assets: PASS (0 forbidden requests)
  - Blocking metrics with raw samples, percentiles, environments, thresholds: PASS
  - Size/startup regression detection: PASS
  - Memory growth bounded in soak scenario (+556 KiB across 100 mixed cycles): PASS
  - No native-throughput equivalence claims: PASS (browser-local latency overhead documented)
  - Process for intentional budget changes documented: PASS
Known limitations:
  - Gzip-9 interim proxy (572,337 B) exceeds 500 KiB, but ratified standard Brotli-11 is 400,229 B (resolved carried finding)
Residual risks:
  - High-concurrency browser environments and low-memory mobile devices remain experimental
Follow-up issue links: BWASM-Q-006 (#1281), BWASM-Q-007 (#1282), BWASM-Q-008 (#1283)
```
