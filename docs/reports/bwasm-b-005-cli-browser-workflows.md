# BWASM-B-005 — CLI build, preview, inspect, and export workflows

## Overview

Closes the B-phase developer path: one documented command sequence takes a
Velqu project from source to a verified, static-hostable browser-WASM
deployment, with integrity inspection and a clean export gate. All four
commands emit schema-versioned JSON (`schemaVersion: 1`) for CI, pinned by
committed fixtures.

## Commands

| command | what it does | failure posture |
|---|---|---|
| `velqu build --target browser-wasm` | native build → B-001 browser set → pinned kernel artifacts → four Bun bundles (page/worker/service-worker/handler) → shell (`index.html`) → B-002 content-addressed `velqu-artifacts.json` (buildId) | nonzero on policy/liveness/pin/bundle failures; nothing emitted on diagnostic paths |
| `velqu inspect browser` | verifies every manifest artifact (digest/size/cross-build binding via the B-002 loader) + shell presence; reports sizes, digests, ABI versions, capabilities, nativeOps, deployment requirements | nonzero on any tampered/mixed/missing artifact |
| `velqu preview` | serves ONLY the generated static bytes plus one `/_velqu_preview/diagnostics` JSON endpoint (`production: false`, deployment requirements surfaced — external/native dependencies are not hidden) | dev tooling only; not required in production |
| `velqu export` | inspect-then-copy of exactly the deployment set (manifest artifacts + shell); refuses tampered sets (`INTEGRITY_FAILED`) | fail-closed copy |

Controls: `--base-path` (SW scope + preview mount; URL forms validated),
`--clean` (wipe stale deployment files), `--source-map` (linked maps +
`sourceMap` manifest role; a following no-map build removes stale maps),
`--kernel <wasm>` (explicit kernel override), `--out`, `--dist`, `--port`,
`--json`.

## Design decisions

- **Vendored, hash-pinned kernel** (`packages/browser-runtime/kernel/`):
  the K-005/K-006 kernel wasm (sha256 `db72b8e8…`, 1,731,509 B) plus the
  pinned wasm-bindgen 0.2.108 nodejs glue, with provenance in `kernel.json`.
  The build fails closed (`KERNEL_PIN_MISMATCH`) if either file changes.
  A third 1.7 MB copy in the demo's dist is deliberately NOT committed
  (`examples/browser-demo/.gitignore`) — the set is reproducible by one
  command and digest-pinned by tests.
- **Browser kernel glue = deterministic fail-closed transform**: the
  nodejs glue's fs-loading tail (unique `const wasmPath = ` marker) is
  replaced by `initKernelSync(verifiedBytes)`; the generated glue above
  the marker is kept verbatim. Layout drift fails the build
  (`KERNEL_GLUE_LAYOUT`), never emits a broken glue. Instantiation uses
  only loader-verified bytes (B-002).
- **Generated page/worker/SW entries**: the page boots the runtime from
  verified artifacts (kernel + pack + `WorkerHost` over a real module
  Worker; session id passes via the first FIFO host message — `self.name`
  and `self.location` are unavailable in Bun workers, and the R-004
  protocol is unchanged). The SW is the B-004 asset/offline lane and
  NEVER executes handlers (ADR-0037 keeps handlers in the page's isolated
  Worker; SW-served API execution is out of MVP scope — B-006/Q-002).
- **Same-dir entry bundling**: generated entries import their neighbors
  with `./` specifiers. Parent-relative specifiers proved
  context-sensitive under Bun's bundler (the test runner's tsconfig
  context changes `../` resolution — reproduced and isolated during
  development); same-dir specifiers resolve identically in every context.
- **No ambient globals** in any emitted code; workspace paths never enter
  artifacts (B-001 sanitizer preserved).

## Acceptance criteria

- ✅ Clean sample, one documented sequence: `velqu build --target
  browser-wasm --project examples/browser-demo` → `velqu preview --project
  examples/browser-demo` (transcript: `evidence/cli-browser/01-cli-transcript.txt`).
- ✅ Inspect detects tampered and mixed sets (unit + CLI subprocess tests:
  truncated pack/contract, foreign pack swap, missing shell; exit 1).
- ✅ JSON schema-versioned and fixture-tested: `schemaVersion: 1` on all
  four commands; committed fixtures `packages/cli/src/fixtures/browser-cli/{inspect-browser,export}.json`
  with digest-tolerant structural matching.
- ✅ Nonzero exits: unsupported `--target`, BWASM-POLICY violations
  (`eval` fixture → `BWASM-POLICY-DYNAMIC-CODE`, nothing emitted), failed
  integrity, kernel pin mismatch.
- ✅ Preview serves only generated static bytes + diagnostics; external
  server dependencies are surfaced (nativeOps, declared capabilities,
  static-hosting note), never hidden; `production: false` in every
  diagnostics response.

## Test evidence

34 new tests (`packages/cli/src/browser-deploy.test.ts`): kernel pin/transform
(5, including real WebAssembly instantiation of the transformed glue),
compose (6, including two-build byte-identity and `--clean`), inspect (6),
export (2), preview (5, traversal/ method guards), **static deployment
smoke** (HTTP-served verified bytes → kernel → real module Worker →
200 `{"message":"Hello world"}` + RFC 9457 404 lane), CLI integration (6),
fixtures (2). Neighbor suites green (114 pass across compiler/CLI/runtime).

Evidence: `docs/codex-spark-browser-wasm/evidence/cli-browser/`
(`01-cli-transcript.txt`, `02-clean-consumer-log.txt` with 27 artifact
hashes). Demo deployment buildId (this source state):
`ffdb03d11e78f76df3f6167d6c30fb27e9d781d890f6b5e17fe9532a1ba903fa`.

## Boundaries and honest notes

- The static deployment smoke runs under Bun (real WebAssembly, real
  module Worker, real HTTP) — it is NOT a browser-engine lane. Real-browser
  activation/upgrade/rollback behavior is B-006 (#1260); supported-browser
  evidence is Q-002.
- Deployment shell files (`index.html`, `page.js`, `worker.js`,
  `service-worker.js`, `kernel.js`) are outside the B-002 manifest's frozen
  role enum; inspect checks their presence and reports digests, but the
  content-addressed integrity contract covers the manifest roles only.
- Bundle-per-entry without shared chunks: the runtime code is embedded in
  each bundle (page/worker/SW/handler). Size duplication is accepted for
  the MVP; code-sharing is a later optimization.
- The emitted page is a status/bootstrap shell; application UIs bundle
  their own page against the same artifacts. No ambient API is exposed.
- Kernel regeneration (e.g. for a future kernel ABI) is a repo release
  step: rebuild q-browser-kernel with pinned wasm-bindgen, then update
  `kernel.json` pins deliberately — not a CLI concern.

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
