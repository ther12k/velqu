# Debugging deployed applications (source-map sidecars)

ADR-0027 moves sources and maps OUT of production artifacts into an
external, untrusted, non-executable sidecar. This document defines the
sidecar convention for BOTH deployment modes (M26-009-D).

## The sidecar file

```json
{
  "formatVersion": 1,
  "packSha256": "<sha256 hex of the exact pack bytes>",
  "bundleSource": "<the compiled JS bundle text>",
  "sourceMap": "<source map JSON or null>",
  "modules": [{ "id": "app.ts", "file": "app.ts" }]
}
```

- Written by the compiler next to the artifact: `velqu build` emits
  `app.qpack.sources.json` beside `app.qpack`.
- Binds to exactly one pack via `packSha256` — advisory for tooling,
  verified by the TOOL, never by the runtime (ADR-0026 trust model:
  integrity-referenced, confers no authenticity; a wrong sidecar
  corrupts only developer ergonomics).

## Where it lives per mode

| mode | artifact | sidecar path | binding key |
|---|---|---|---|
| shared | `app.qpack` | `app.qpack.sources.json` | sha256(app.qpack) |
| standalone | `velqu-standalone` executable | `velqu-standalone.sources.json` | sha256(EMBEDDED pack bytes) |

For standalone mode there is no pack file on disk: the sidecar binds to
the bytes embedded in the executable (the compiler emits it from the
same pack used at the `VELQU_STANDALONE_PACK` build).

## Finding the binding key

Both binaries print it without serving:

```bash
velqu-runtime --fingerprint --pack app.qpack
velqu-standalone --fingerprint
```

The ready output includes `"packSha256"` and the mode's sidecar name.
Tooling (or a cautious operator) compares it against the sidecar's
`packSha256`; `q_pack::sources_sidecar::SourcesSidecar::load_and_verify`
does this check programmatically (unknown format versions fail closed
for tooling; hash mismatch = wrong artifact).

## Symbolizing a stack trace

1. Get the raw QuickJS frame (file `app.js`, line/col in the bundle).
2. Load the sidecar's `sourceMap` with any source-map consumer.
3. Map the bundle position to the original TypeScript location.

The runtime's own diagnostics use the pack-embedded map when a legacy
v1 pack still carries one (`mapper_for`); production v2 packs are
debug-free by design and rely on this sidecar instead.
