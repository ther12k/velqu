Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/04_build_deploy/BWASM-B-006-verify-cache-activation-upgrades-rollback-and-static-deployment.md`  
Program: `BWASM`  
Phase: `04_build_deploy` — Compiler, artifacts, and static deployment  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

---

# BWASM-B-006 — Verify cache activation, upgrades, rollback, and static deployment

## Atomic goal

Prove browser-WASM builds activate atomically and can upgrade or roll back on static hosts without mixed application state.

## Parent intent

Turn the runtime into deterministic static artifacts that can be built, inspected, updated, rolled back, and hosted generically.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-B-004` — Add Service Worker adapter and static-host bootstrap
- `BWASM-B-005` — Add CLI build, preview, inspect, and export workflows

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/compiler/src/index.ts`
- `packages/compiler/src/extract.ts`
- `packages/compiler/src/emit.ts`
- `packages/cli/src/index.ts`
- `packages/browser-runtime/`
- `.github/workflows/verify.yml`

## Steps

1. Create N, N+1, and rollback fixtures with incompatible route/schema/handler changes.
2. Exercise cold install, warm reload, multiple tabs, interrupted download, partial CDN propagation, and offline state.
3. Ensure a build becomes active only after all bound artifacts verify.
4. Define client notification/reload behavior when an update is ready or incompatible.
5. Test at least the frozen supported static host shapes and base paths.

## Acceptance criteria

- [x] No request executes against a mixed N/N+1 artifact set.
- [x] Interrupted or corrupt updates leave the last known-good build usable where policy permits.
- [x] Rollback restores a fully coherent build.
- [x] Multiple tabs converge according to the documented activation policy.
- [x] Cache cleanup does not remove artifacts still required by an active client.
- [x] All discovered defects are fixed or linked as blockers with reproductions.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Playwright multi-context lifecycle suite.
- CDN partial-propagation simulator.
- Offline/interrupted update tests.
- Rollback rehearsal.
- Static host matrix.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [x] Upgrade/rollback report.
- [x] Browser traces.
- [x] Artifact/cache inventories before and after each scenario.
- [x] Known-residual-risk register.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Application data migrations beyond documented adapter hooks.
- Hand-waving eventual cache consistency.
- Declaring success from a single clean first load.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-b-006:`.
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

## Result

**PASS** (2026-09-06). VERIFY_OR_FIX closure in a real browser (headless
Chromium 151.0.7922.34 via Playwright; committed rerunnable script
`scripts/browser-wasm-lifecycle-rehearsal.py`).

**Defects found by verification and fixed here (5 + 3 minor, with
reproductions in the report):** offline reload broken (shell never
cached + no navigation fallback); SW bytes byte-identical across
deploys so the browser update check never fired; install without digest
verification could half-succeed; offline re-fetch of a navigate-mode
Request never rejects in Chromium (fallback never ran); restarted-SW
state loss. Fixes: verified install (`precacheVerified`, fail-closed),
self-identifying SW (deployment sha256 embedded), shell caching +
scope-root navigation fallback, constructed-Request re-fetch,
`ensureReady()` rehydration on every lifecycle path, bounded retention
(`cachesToKeep`: active + ≤1 previous), user-consented apply
(`VELQU_APPLY_UPDATE` → skipWaiting), `loadArtifactsWithFallback`
(last-known-good boot through the B-002 loader).

**Scenarios proven (twice, REHEARSAL-PASS):** cold install; offline
navigation with the host down; live upgrade N→N+1 (coherent boot on
swap from cache; applied N+1 from network after consent; browser
update-check throttle ≤24h documented with `registration.update()`
mitigation); multi-tab convergence; corrupt/mixed update refused with
last-known-good retained; rollback to a coherent N; base-path `/app/`
cold + offline. Evidence:
`evidence/browser-lifecycle/{00-scenario-summary,lifecycle-log.txt,
lifecycle-report.json}`. Unit tests for the new runtime helpers;
browser-runtime + CLI suites 132/132.

- Report: `docs/reports/bwasm-b-006-cache-upgrade-rollback-verification.md`
  (includes the residual-risk register: retention vs. very old clients,
  update-check throttle, single-Chromium lane until Q-002, shell files
  availability-cached outside the digest contract).
- Honest boundary: one Chromium build was exercised; the supported-
  browser matrix and CI lanes remain BWASM-Q-002 per OD-BWASM-001.
