# BWASM-Q-002 — Real-browser CI lanes and supported-browser evidence

## Overview

Browser-WASM tests now run in REAL browsers against EMITTED
release-like artifacts, with lanes classified required vs experimental,
browser/OS/version evidence captured per run, and the results bound to
the D-004/ADR-0039 support matrix.

## What this adds

- **`scripts/browser-e2e-rehearsal.py`** (committed, rerunnable; the
  lane runner for CI and local evidence): drives a real browser through
  six smoke suites over `velqu build --target browser-wasm` output:

  | lane | what it proves |
  |---|---|
  | E1 kernel boot | wasm kernel + verified artifacts + module Worker boot and serve a request |
  | E2 dispatcher | the runtime boundary executes a declared route in-page (self-probe), status + body canonical |
  | E3 Service Worker | install (verified prefetch) → activate → control the page |
  | E4 offline | 13-entry verified precache; host fully down → offline navigation still boots |
  | E5 IndexedDB KV | the REAL C-004 adapter: set/get/list, durable across reloads |
  | E6 update | redeploy changes SW identity bytes → forced `update()` installs → user-consented apply → new buildId on reload |

- **CI workflow** `.github/workflows/browser-lanes.yml`: Chromium =
  REQUIRED blocking lane; Firefox/WebKit = experimental allowed-failure
  lanes (they can never satisfy a release gate); weekly scheduled full
  run; failures upload the evidence JSON + diagnostics.
- **Browser/OS matrix manifest** `evidence/browser-matrix.json`: lane
  classifications (tested / experimental / unverified), ownership,
  update cadence, feature baseline with fail-closed absence semantics.
- **Support matrix updated**: `docs/specs/browser-support-matrix.md`
  now lists the Chromium lane as tested (Chromium 151.0.7922.34, Linux
  x64, local green run) with Firefox/WebKit explicitly untested.

## Acceptance criteria

- ✅ Every claimed supported browser has a blocking evidence lane — the
  only *tested* browser is Chromium, and it is the required lane.
- ✅ A browser absent from evidence is marked unverified/unsupported
  (Firefox/WebKit/mobile recorded as experimental-untested; no implicit
  claims).
- ✅ CI tests emitted release-like artifacts (the CLI-built static
  deployment, served statically), not development source imports.
- ✅ Failures upload logs/traces/artifacts (workflow artifact upload on
  failure; the runner writes structured per-check JSON with details).
- ✅ Experimental lanes cannot satisfy a release gate (continue-on-error
  + explicit gate step for required lanes only).
- ✅ Matrix ownership and update cadence documented
  (`browser-matrix.json` policy block; ADR-0039 amendment path).

## Test evidence

- Chromium 151.0.7922.34 local green run, all seven checks:
  `evidence/browser-lanes/chromium.json` (source-commit-bound).
- The same runner pattern drove the B-006 (cache/upgrade/rollback) and
  C-004 (IndexedDB) rehearsals — merged evidence.

## Honest notes

- **Single required lane**: Chromium on Linux x64 is the only tested
  browser. Firefox/WebKit lanes are defined but their browser binaries
  are not yet provisioned in this environment — recorded as
  experimental-untested, never claimed as supported.
- **Standing CI failure**: GitHub Actions runners have stalled with zero
  executed steps since ~#714 (standing disclosure on every packet PR).
  The workflow is the committed lane definition; until runners execute,
  the acceptance evidence is the local Chromium green run, explicitly
  recorded as such in the matrix (not represented as a CI run).
- Headless-only: per the task's out-of-scope note, headed behavior is
  not claimed; the lanes run headless Chromium.
- Mobile lanes remain experimental/untested per ADR-0039.

Standing CI disclosure applies; local gates are the acceptance basis.
