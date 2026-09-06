# BWASM-B-002 — Content-Addressed Browser Artifact Manifest and Loader

## Result

**PASS** — one build's artifacts are bound to a single content-derived
`buildId` with per-artifact SHA-256 digests, sizes, media types, and
relative URLs; the loader verifies every byte before activation and
fails closed on tampered, truncated, missing, cross-build, and
unsupported-version inputs. The module is **browser-pure** (WebCrypto
`crypto.subtle` hashing — no `node:*` imports; enforced by the R-001
purity scan). 70/70 package tests, typecheck clean.

## Contract

- **Manifest v1** (`ARTIFACT_MANIFEST_VERSION = 1`): `formatVersion`,
  `target: "browser-wasm"`, `handlerAbiVersion`, `kernelAbiVersion`
  (both ABI versions carried per B-001/K-005), `appId`, top-level
  `packSha256`, and per-role artifact entries (`url`, `sha256`,
  `bytes`, `mediaType`) for pack, kernel wasm, handler bundle,
  manifest, contract, schemas, capabilities, source-map metadata.
- **Canonical serialization**: fixed key order, sorted artifact roles,
  no whitespace; `buildId = sha256(canonical bytes)` (ADR-0023
  posture). Golden-vector pinned in tests (prefix bytes + role sort
  order + no-whitespace).
- **Loader** (`loadArtifacts(manifestJson, reader)`): parse → version
  check → target check → **buildId re-computation** → per-artifact
  missing/size/digest checks → top-level `packSha256` cross-build
  binding. Nothing is returned until every check passes — activation
  (WASM instantiation, handler import) is impossible on unverified
  bytes by construction.
- **Base paths**: `resolveArtifactUrl` supports root (`/app.qpack`) and
  non-root (`/apps/demo/app.qpack`) static hosting; URLs are
  deploy-relative POSIX paths.
- **Errors**: `ArtifactManifestError { artifact, reason }` with reasons
  `unsupported-version | missing | truncated | tampered | cross-build`;
  messages name the artifact and expected/got digests — no content is
  dumped.

## Test evidence (11 new tests; package 70/70)

| Case | Result |
|---|---|
| Golden canonical bytes (prefix, role sort, no whitespace) | pass |
| buildId = sha256(canonical); field change ⇒ different buildId | pass |
| Clean deployment → verified bytes + buildId | pass |
| Tampered (same-length substitution) → `tampered`, artifact named, no content dump | pass |
| Truncated → `truncated` with expected vs actual | pass |
| Missing → `missing` naming the URL | pass |
| **Cross-build** (forged per-entry digest + recomputed buildId, stale `packSha256`) → `cross-build` on pack | pass |
| Unsupported formatVersion 99 → `unsupported-version` | pass |
| Swapped manifest (buildId mismatch) → `tampered` | pass |
| Root/non-root base-path resolution | pass |

## Browser-purity fix (found by the R-001 purity scan)

The first implementation hashed with `node:crypto` and read via
`node:fs` — the purity scan flagged it. Resolution: all hashing moved
to WebCrypto `crypto.subtle` (loader and emission are async; Bun tests
and Node ≥18 provide the same standard global), and the fs reader moved
out of the package (tests inject their own reader; the compiler-side
file emission passes bytes in). This is why the R-001 purity test
exists — it caught a real browser-purity regression at development
time.

## Ordering rationale (honest)

The loader checks size before digest (truncation is cheaper to name),
digest before the cross-build binding, and buildId before everything:
a forged per-entry digest invalidates the buildId first, so the
`packSha256` cross-build binding targets internally-consistent
manifests carrying a foreign pack — the test constructs exactly that
case white-box (documented in-test).

## Boundaries

Cache-storage integration and offline activation flows are B-004/B-006;
the loader here defines and enforces the verification contract they
must call. Real-browser lanes remain Q-002.

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
