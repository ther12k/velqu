# Browser-WASM

The same Velqu application builds to **static assets that run in an
ordinary browser** — no Velqu application server:

- compatibility-critical routing, schema validation, manifest/QPack
  verification, capability authorization, and problem mapping run through
  **Rust compiled to WebAssembly**;
- generated TypeScript handlers run in an **isolated browser Worker**;
- the public runtime boundary is `fetch(Request) => Promise<Response>`;

## Quickstart

```bash
bun packages/cli/src/index.ts build --target browser-wasm --project my-app
bun packages/cli/src/index.ts inspect browser --project my-app
bun packages/cli/src/index.ts preview --project my-app --port 8080
bun packages/cli/src/index.ts export --project my-app --out dist/static-site
```

Authoring requirement: route bindings must be exported from their source
module (`export const tick = route({...})`) — the build fails closed
otherwise.

## Status

- All eight program phases (design, kernel, runtime, build/deploy,
  capabilities, quality/release) are complete; a release candidate packet
  (checksums, CycloneDX SBOM, candidate index) is assembled and awaiting
  the recorded GO/NO-GO gate review.
- Measured against ratified budgets: kernel 400,229 B brotli (≤ 512,000 B
  budget), total distributed 453,771 B (≤ 1 MiB), cold start 1938 ms
  (≤ 2000 ms), warm 128 ms (≤ 500 ms), p50 0.3 ms / p99 1.4 ms.

## Honest boundaries

- **Not a hostile-code sandbox.** Worker isolation is a process boundary
  for trusted application code, not a security sandbox; untrusted-handler
  deployments use a separate preview origin (ADR-0038).
- **No in-browser Postgres.** Browser persistence is namespaced IndexedDB
  KV; PostgreSQL stays a native-runtime capability (async contract).
- **No native-performance-parity claims** without separately required
  evidence.
- Browser evidence lanes run Chromium; other browsers are documented but
  untested.

## Canonical doc

Everything else — hosting/HTTPS/Service-Worker lifecycle, capabilities,
IndexedDB KV, observability (35 `DIAG_*` codes, correlation IDs),
support matrix, migration guide:
[docs/beta/BROWSER_WASM.md](https://github.com/ther12k/velqu/blob/master/docs/beta/BROWSER_WASM.md).
Limitations: [docs/beta/KNOWN-LIMITATIONS.md](https://github.com/ther12k/velqu/blob/master/docs/beta/KNOWN-LIMITATIONS.md).
