# BWASM-B-001 — Compiler Target `browser-wasm`

## Result

**PASS** — the compiler now emits a deterministic, self-contained
browser artifact directory via
`buildBrowserWasmArtifacts` (`packages/compiler/src/browser.ts`), with
the native QPack path untouched, build-time diagnostics for
native-only constructs, and two-build byte-reproducibility.

## Documented command + artifact tree

```bash
bun packages/cli/src/index.ts build --project examples/browser-demo
bun -e '…buildBrowserWasmArtifacts…'   # or via the B-005 CLI workflow
```

Sample project committed: `examples/browser-demo` (hello.get +
echo.post — context-using handlers; the proof app's `health.live` is a
native-liveness route and therefore CORRECTLY rejected by this target —
see diagnostics). Emitted tree (`examples/browser-demo/dist/browser/`):

| File | Bytes | sha256 (build 1 = build 2) |
|---|---|---|
| app.browser.js | 1,218 | `430ac67cb4ff18e092a1e82a4104bb59b29609185232e3e54cf332c2c584cae1` |
| app.qpack | 9,333 | `11adb3cea4706e2c4a9c04169c73e93109188eb89310708566ffecdaee7650bd` |
| browser-manifest.json | 471 | `86fa53f2797af22524febde153a4c29bca0cc2cce927470fb267fa0da1bf9b75` |
| capability-manifest.json | 449 | `1ec7da6bea1e467e90569d8caefb7a40ed70cdb3072f71e4a2cbd27e5a4a264a` |
| contract.json | 1,603 | `be8f2fd060e9947826733256166b66691da5283ededa4c65c911fb1e27e2124e` |
| schema-manifest.json | 1,082 | `a0bc84783d077115c40b6f0a9ef6336a41ad49ff6d1eff0cf0a95326b5dc2911` |

**REPRODUCIBILITY-OK**: two full builds → byte-identical hashes across
all six files (`evidence/compiler-browser/01-artifact-tree-hashes.txt`).

## Emission design

- `app.browser.js` — handler-bundle ES module: imports the app's route
  modules (paths relative to the emitted file), registers through the
  **narrow R-003 API** (`defineBrowserHandlers`) — the golden fixture
  asserts `globalThis.__velquFunctionManifest` (ambient-global style)
  is absent; every registration carries its declared statuses; the
  pack expectation table is embedded (requiredHandlerKeys +
  declaredStatuses + ABI).
- `browser-manifest.json` — target identity (`browser-wasm`), handler
  ABI v1, kernel ABI v1, pack sha256, sorted handler table. No
  wall-clock fields, no absolute paths (COMP-003 + acceptance 5).
- `app.qpack` carried **byte-identical** from the native build
  (test-pinned); contract/schema/capability manifests carried for a
  self-contained directory.
- Compiler host stays on native/Bun tooling; only emitted output runs
  in browsers (invariant kept).

## Diagnostics (fail at build time, never as blank browser failures)

- **Native-liveness routes rejected** with a source-located
  `CompileError` — snapshot:
  `browser-wasm target: route "health.live" uses native liveness
  (RUN-009), which the browser kernel does not provide — remove it or
  split it into a native-only service (src/modules/health/routes.ts)`.
  The diagnostic path creates no artifact directory (tested).
- No workspace path leaks: emitted sources/metadata carry
  project-relative, sanitized locations (tested with the diagnostic
  message containing `src/app.ts` for a `.tmp` fixture).

## Native-target regression

Native output unchanged: the browser emitter consumes native build
products read-only (byte-equality pinned in tests); compiler suite
19/19 including the published-artifact determinism tests.

## Acceptance disposition

- ✅ Documented command builds the sample project into a self-contained
  browser artifact directory (committed `examples/browser-demo`).
- ✅ Output deterministic modulo normalized metadata (two-build hash
  equality; no wall-clock/absolute-path fields).
- ✅ Native target output unchanged (read-only consumption; compiler
  suite green).
- ✅ Unsupported constructs (native-liveness) fail at build time with
  source-located diagnostics; diagnostic path emits nothing.
- ✅ No development workspace path leaks into emitted artifacts.

## Boundary

Kernel WASM binary + JS glue distribution is BWASM-B-002
(content-addressed artifact manifest/loader); the emitted manifest
references required ABI versions (K-005/R-003 contracts).

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
