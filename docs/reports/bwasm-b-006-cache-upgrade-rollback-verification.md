# BWASM-B-006 — Verify cache activation, upgrades, rollback, and static deployment

## Overview

Verify-or-fix closure for the browser-WASM static deployment lifecycle. The
verification FIRST found four real defects in the B-004/B-005 composition
(reproduced in headless Chromium 151 via Playwright), all fixed in this
packet; the re-run proves the acceptance criteria end-to-end in a real
browser.

## Defects found and fixed (verify → fix, with before/after)

1. **Offline reload was broken (shell never cached).** The B-004-derived
   install prefetched only the manifest plan; the app shell
   (`index.html`, `page.js`, `worker.js`, `kernel.js`,
   `velqu-artifacts.json`) was never cached, and the navigation lane had
   no offline fallback — an offline reload hung. *Fix:* the generated SW
   caches the shell (availability lane, outside the B-002 digest
   contract) plus a scope-root navigation fallback; navigation is
   network-first with a cached-shell fallback.
2. **Redeploys never updated: the SW script was byte-identical across
   deployments.** The browser update check only fires when SW bytes
   change, so the B-005 SW (byte-stable glue + constants) never updated.
   *Fix:* the SW is now bundled AFTER the manifest and embeds the
   deployment identity (sha256 of `velqu-artifacts.json`); every redeploy
   produces different SW bytes → the update check fires.
3. **Install could half-succeed; partial updates could activate.**
   Install fetched artifacts without digest verification and without
   failing the install atomically. *Fix:* `precacheVerified` verifies
   sha256 (manifest roles) and THROWS on HTTP error, truncation, or
   digest mismatch — a failed install never activates, so the previous
   SW stays active and usable.
4. **Offline navigation re-fetch of a navigate-mode Request never
   rejects in Chromium.** `fetch(navigationRequest)` pends forever when
   the host is down, so the cached-shell fallback never ran. *Fix:* the
   SW re-fetches a CONSTRUCTED Request (same URL/headers), which rejects
   normally; the fallback then serves the cached shell.
5. ** restarted-SW state loss (found while fixing 1).** A restarted SW
   had null in-memory state and passed through every request. *Fix:*
   `ensureReady()` rehydrates on every lifecycle path; the SW is
   cryptographically self-identifying (it adopts the cache whose
   manifest bytes hash to its embedded deployment identity), so an older
   SW generation keeps serving its own coherent build.

Additional defects fixed in the same bounded area: detached-receiver
`fetch` ("Illegal invocation" in the SW), a bogus `.../undefined`
cache key (`BASE_URL.href` on a scope string), and stale
`.map`/entry cleanup across `--source-map` builds.

## Verification evidence (real browser, headless Chromium 151.0.7922.34)

`scripts/browser-wasm-lifecycle-rehearsal.py` (committed, rerunnable;
Playwright is evidence tooling only — the repo has no Python dependency)
drives N and N+1 fixtures (incompatible: N+1 adds a route) through:

| scenario | result (evidence: `evidence/browser-lifecycle/00-scenario-summary.txt`) |
|---|---|
| S1 cold install | SW activates only after ALL bound artifacts prefetch with digest verification; 13 cache entries |
| S2 offline navigation | host completely down → build N still boots (`source: cache`) |
| S3 live upgrade | on swap the N client boots N from cache (coherent); browser update-check throttle documented; forced `update()` installs N+1 (waiting); `VELQU_APPLY_UPDATE` → skipWaiting (user-consented apply-on-next-reload) → N+1 from network |
| S4 multiple tabs | tabs converge on the activated build |
| S5 corrupt update | forged manifest (mixed N/N+1 digests) can NEVER activate; existing client keeps booting N (last known-good) |
| S6 rollback | redeploying N restores a fully coherent N |
| S7 base-path `/app/` | cold install + offline under a subpath scope |

Full JSON report + raw log: `evidence/browser-lifecycle/lifecycle-report.json`,
`lifecycle-log.txt` (request/console traces per scenario). Rehearsal run
twice; both REHEARSAL-PASS.

Unit tests for the new runtime helpers (deterministic, Bun):
`precacheVerified` (digest match/mismatch/HTTP error/shell),
`cachesToKeep` (active + ≤1 previous; never touches foreign caches),
`loadArtifactsWithFallback` (verified cache boot; refuses corrupt cache).
Browser-runtime + CLI suites: 132/132.

## Acceptance criteria

- ✅ No request executes against a mixed N/N+1 artifact set — every boot
  path runs the B-002 loader end-to-end (S3/S5; install refuses mixed sets).
- ✅ Interrupted/corrupt updates leave the last known-good usable (S5, S2).
- ✅ Rollback restores a fully coherent build (S6).
- ✅ Multiple tabs converge per the documented activation policy (S4;
  policy: apply-on-next-reload, explicit user-consented apply).
- ✅ Cache cleanup keeps artifacts required by an active client (retention
  keeps the active cache + at most one previous; per-build isolation means
  an in-memory client is unaffected; see residual risks).
- ✅ All discovered defects fixed with reproductions (5 + 3 minor, above).

## Documented update policy (ADR-0039 §4 concretized)

- First load online; install prefetches everything with verification.
- Browsers throttle navigation-time SW update checks (≤ once / 24 h);
  deterministic checks use `registration.update()` (hosted apps can call
  it on a schedule or on a version endpoint change).
- A verified install becomes `waiting`; it applies on the next reload, or
  immediately after the page posts `VELQU_APPLY_UPDATE` (user consent) —
  never a mid-session swap of a running build.
- Retention keeps the active build's cache + at most one previous
  (deterministic: lexicographically greatest buildId; content hashes carry
  no chronology — documented, bounded policy).

## Residual risks (register)

- **Retention vs. very old clients:** a client still running build N−k
  (k ≥ 2) may find its cache evicted on reload and, if the host is
  unavailable, cannot boot. Mitigation: static hosts serve all builds;
  retention only bounds CacheStorage growth.
- **Update-check throttle:** without a forced `update()` or a version
  endpoint, update discovery follows the browser's ≤24 h schedule.
- **Bun-vs-browser harness gap closed for these lanes only:** Playwright
  scenarios run one Chromium build; the supported-browser matrix (Firefox/
  WebKit, mobile) remains BWASM-Q-002.
- **Shell files are availability-cached without per-file digests** (they
  sit outside the frozen B-002 role enum); the runtime integrity boundary
  (kernel/pack/handler bundle/manifests) is fully digest-bound.

## Boundaries

- Native Velqu behavior untouched (browser pipeline consumes native build
  output read-only — B-001/B-005 regressions re-verified).
- No hostile-code sandboxing / PostgreSQL-parity / native-performance
  claims; handlers remain trusted code in an isolated Worker (ADR-0037/38).
- The rehearsal script is committed evidence tooling; Q-002 owns CI lanes.

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
