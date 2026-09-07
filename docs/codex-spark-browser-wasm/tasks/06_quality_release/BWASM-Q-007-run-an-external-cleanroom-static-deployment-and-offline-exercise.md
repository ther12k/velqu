Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-007-run-an-external-cleanroom-static-deployment-and-offline-exercise.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `EVIDENCE` — Package evidence from one exact candidate; do not mix implementation and attestation.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

---

# BWASM-Q-007 — Run an external cleanroom static deployment and offline exercise

## Atomic goal

Prove that an external consumer can build and deploy a useful Velqu Browser-WASM application using only public artifacts and documentation.

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

- `BWASM-Q-002` — Add real-browser CI lanes and supported-browser evidence
- `BWASM-Q-003` — Verify isolated preview-origin and untrusted-code security boundaries
- `BWASM-Q-005` — Set and enforce WASM size, startup, latency, and leak budgets
- `BWASM-Q-006` — Publish Browser-WASM documentation, limitations, and migration guide

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Use a fresh external repository and participant/agent not involved in implementation.
2. Install release-candidate packages/artifacts without workspace links or source-path fallbacks.
3. Build a bounded CRUD-style app with route params, schema validation, Treaty, timer/logging, local KV, and at least one deployment-required capability.
4. Deploy to a generic static HTTPS host under a non-root base path.
5. Exercise first load, forms/fetch, persistence, offline reload, update, rollback, reset/export, and deployment-required UX.
6. Record setup failures, docs gaps, iterations, ambiguity, and framework defects separately.

## Acceptance criteria

- [ ] The app works from registry/candidate artifacts only.
- [ ] No Velqu application server is running after static deployment.
- [ ] The participant can distinguish browser-local behavior from native production behavior.
- [ ] Local data persists according to the documented policy and remains project-isolated.
- [ ] Deployment-required behavior is explicit and machine-readable.
- [ ] Every blocking defect is fixed and re-proven or leaves the candidate NO-GO.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Clean clone/install/build transcript.
- Static host network/process inventory.
- Browser user-journey suite.
- Offline/update/rollback rehearsal.
- External usability notes.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] External repository commit.
- [ ] Package/artifact lock and hashes.
- [ ] Deployment recording and network trace.
- [ ] Participant report and defect disposition.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Using monorepo workspaces or unpublished package sources.
- Treating implementer familiarity as usability evidence.
- Replacing failed journeys with manual claims.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-007:`.
- Reference this issue ID in commits, PR body, tests, and evidence.
- Avoid generated queue/index churn until implementation and targeted tests are stable.
- If scope expands materially, stop and open a new dependency issue rather than hiding extra work here.

## Stop condition

Stop and hand off when **all** acceptance criteria are demonstrated, the required evidence is attached or committed, canonical verification is green, and no unresolved in-scope P0 remains. If a prerequisite, owner decision, browser limitation, or security claim blocks truthful completion, record the exact blocker and leave this issue open.

## Handoff format

## Result

```text
Issue: BWASM-Q-007 (#1282)
Candidate commit: pending commit on bwasm-q-007 (source baseline: 2ae4800)
Files changed:
  crates/q-browser-kernel/src/lib.rs                      (D5 layer 1: grant-name authorization; plan.capabilities)
  crates/q-browser-kernel/tests/kernel_path.rs            (D5 regression test)
  packages/compiler/src/browser.ts                        (D4: __ok/__problem/__velquRaw status mapping)
  packages/browser-runtime/src/worker-host.ts             (D5 layer 2: ctx.native for declared grants)
  packages/browser-runtime/src/index.ts                   (KernelInvokePlan.capabilities)
  packages/browser-runtime/src/service-worker.ts          (D7: asset cache-miss network fallback)
  packages/cli/src/browser-deploy.ts                      (worker-side capability graph construction)
  packages/browser-runtime/kernel/*                       (vendored kernel refresh, re-pinned)
  packages/cli/src/fixtures/browser-cli/*.json            (byte-count fixtures)
  packages/cli/src/browser-deploy.test.ts                 (kernel size pin)
  conformance/browser/fixture-app/dist/browser/velqu-artifacts.json (kernel digest/buildId)
  docs/codex-spark-browser-wasm/evidence/q-007/*          (participant report, defect log/disposition,
                                                           fix verification + transcripts, artifact hashes)
Commands run:
  (cleanroom, independent agent, rounds 1-4) fresh app from artifact tarballs only;
  build --target browser-wasm --kv --base-path /app/; static host on 127.0.0.1;
  real-Chromium Playwright journeys (first load, route execution, validation failures,
  offline reload, persistence, custom page); canonical scripts/browser-e2e-rehearsal.py
  (7 checks PASS post-fix, incl. E4 offline navigation and E6 update-on-reload);
  cargo test -p q-browser-kernel --features bindgen; bun test packages/browser-runtime
  packages/cli conformance/browser; bun x tsc -b; canonical scripts/verify (netns).
Targeted tests:
  - kernel_path: 16 pass (incl. new plan_accepts_grant_name_when_linked_module_id_is_inventoried)
  - browser-runtime + cli + differential: 233+36+16 pass, 0 fail
  - canonical e2e rehearsal: E1-E6 all PASS
Full verification: ALL PASS (scripts/verify in network namespace)
Artifacts and SHA-256: docs/codex-spark-browser-wasm/evidence/q-007/artifact-hashes.txt
  (kernel wasm b0485be7…1732433B; per-tarball hashes; baseline commit 2ae4800)
Browser/OS/toolchain: Chromium 151 (chrome-for-testing), Linux x86_64, Bun 1.4.0
Acceptance criteria:
  - App works from candidate artifacts only: PASS (rounds 1-4 apps built from tarballs + docs; no repo access)
  - No Velqu application server after static deployment: PASS (python/bun static file servers only)
  - Browser-local vs native behavior distinguished: PASS (Postgres route refused at build time;
    deployment-required problem shape documented; evaluator report)
  - Local data persists per documented policy, project-isolated: PASS (IndexedDB kv:items-crud;
    survived reload + full server restart; namespaced stores verified via raw IDB dump)
  - Deployment-required behavior explicit and machine-readable: PASS (build-time refusal + 501 shape)
  - Blocking defects fixed and re-proven: PASS (D4 re-proven rounds 2-4; D5 re-proven round 4;
    D7 online/restart legs re-proven rounds 2-4; D1/D2/D3/D6/D8/D9 dispositioned in
    defect-disposition.md — none blocking; follow-up #1292 registered for the round-4 finding)
Known limitations:
  - beta distribution is source-based (npm publication Owner-gated): cleanroom install used
    manual tarball extraction, as documented in INSTALL.md
  - offline navigation fallback serves the DEFAULT cached shell; a custom page outside the
    manifest navigates from network (documented behavior; data recorded in round-4 report)
Residual risks:
  - evaluator coverage is one browser (Chromium); lanes for other engines are Q-002 experimental
  - silent-broken-bundle trap for non-exported route bindings tracked as #1292
Follow-up issue links: #1292 (export guard); D6 remainder (developer smoke page)
```
