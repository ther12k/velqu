# BWASM-Q-001 — Native-versus-browser conformance and differential suites

## Overview

VERIFY_OR_FIX closure: a single fixture corpus now runs through BOTH
execution lanes against the SAME compiled pack — the native Rust runtime
over HTTP (production posture) and the Browser-WASM lane (loader-verified
artifacts → Rust/WASM kernel → isolated handler Worker →
`fetch(Request) => Promise<Response>`) — with canonical comparison,
explicit classification, drift detection, and a machine-readable matrix.

**The verification found and fixed a real kernel defect**: the wasm
kernel's problem registry had drifted from the frozen native registry —
validation failures returned **400 "Validation Failed"** instead of
**422 "Validation failed"** (and the completion path used stale titles;
body 400 vs 415). Fixed at the source in `q-browser-kernel`, the vendored
kernel artifacts were rebuilt with the pinned toolchain (wasm-bindgen
0.2.108) and the `kernel.json` pins updated deliberately (wasm sha256
`a5b33a56…`, glue unchanged `8434c857…`). Kernel tests updated and green.

## Structure

- **Fixture app**: `conformance/browser/fixture-app` — 7 routes pinning
  routing, query/body schema validation (valid + invalid), declared
  statuses, problem responses, method coverage, params/query/capability
  consumption (the classified native-only set).
- **Runner**: `conformance/browser/differential.test.ts` — 16 tests:
  11 corpus fixtures (per-fixture native+browser execution with
  canonical comparison), matrix writer (schema-versioned JSON with
  source commit, native binary sha256, kernel wasm sha256, pack sha256,
  browser buildId, Bun version), and 4 mutation-sensitivity tests
  (mutated status/body/content-type detected; mutated request path
  detected against recorded expectations).
- **Canonicalization (approved fields only)**: deep key-order sorting;
  problem-envelope reduction to contract fields
  (status/type/title/errors). Envelope deltas — native `instance` +
  `detail`, native body-`allow` omission (the HTTP `Allow` header
  carries it), kernel `problemId` — classify
  **equivalent-by-contract** and stay visible in the matrix via
  `rawEqual=false`.
- **Classification**: exact-parity / equivalent-by-contract /
  native-only / drift-detected. Frozen counts pinned in-test:
  **4 exact-parity, 3 equivalent-by-contract, 4 native-only,
  0 drift**. Any count change is reviewed drift — never auto-approved.

## Acceptance criteria

- ✅ Every public browser behavior has at least one fixture (routing,
  validation, bodies, declared statuses, problems, method coverage,
  capability authorization; SW/upgrade lanes are B-006's corpus).
- ✅ Compatibility-critical routing/schema paths run the Rust/WASM
  kernel (both lanes use the same Rust logic — native binary and
  wasm32 kernel from the same crate sources).
- ✅ Differences linked to a frozen support-matrix entry:
  `docs/specs/browser-support-matrix.md` (native-only: params/query
  worker-context consumption, `ctx.native` worker wiring, `status()`
  returns; envelope deltas documented). Owner ratification of the
  matrix is requested in the PR (the classifications are data, pinned
  by tests).
- ✅ The suite detects intentional mutation (status/body/content-type/
  request-path mutations all flip results to drift).
- ✅ Results include source commit, native binary hash, WASM hash,
  browser/toolchain versions (`evidence/conformance/differential-matrix.json`).
- ✅ No broad snapshot update: classification counts are pinned; drift
  fails the suite; the matrix writer asserts zero drift-detected.

## Gates

Full `bun test` in the prescribed netns: **676/676** (84 files; 16 new
differential tests). Kernel: 15/15 (updated pins). `tsc -b` clean; fmt,
clippy, validate-okf green; `./scripts/verify` ALL PASS (manifest
refreshed with matched evidence). Artifact hashes for this packet:
kernel wasm `a5b33a56…`, native runtime `194fbbd4…`, fixture pack
`38869b6e…` (full hashes in the matrix JSON).

## Honest notes

- The differential RUNS ONLY under Bun + the release native binary — the
  "browser" lane is the real wasm kernel + real Worker in Bun, not a
  browser engine; browser-engine lanes are Q-002.
- The native-only classifications (params/query worker-context
  consumption, `ctx.native` in the worker, `status()` returns) are real
  MVP gaps surfaced by this suite and recorded — NOT silently normalized.
  Each carries a follow-up direction in the support matrix.
- The kernel registry alignment changes browser-lane validation status
  400→422 and body 400→415 to match the frozen native registry — a
  behavior change to the browser lane justified by registry parity
  (frozen URNs spec), with kernel tests updated.

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
