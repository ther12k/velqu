# M27-010 Web API Conformance — Pinned WPT & WinterTC Subsets

Programmatic conformance baseline pinning Web Platform Tests (WPT) and WinterTC Minimum Common Web Platform API test subsets for Velqu M27 capabilities.

## Standards and Pinned Manifest

Pinned test vectors and explicit skips are formally declared in [`conformance/web-api/wpt-manifest.json`](../../conformance/web-api/wpt-manifest.json).
Manifest SHA-256: `aa4f59dabab4b91fc3ce20dbc260567e95e0555e2bff2c746bc934022e5ebf19` (Commit: `c02338b`).

| Capability | Upstream Standard | WinterTC Profile | Pinned Subset ID | Status |
| :--- | :--- | :--- | :--- | :--- |
| `url` | WHATWG URL Standard | WinterTC Minimum Common Web Platform API | `wpt-url-resolution`, `wpt-url-normalization`, `wintertc-urlsearchparams` | PASS (15/15) |
| `text_encoding` | WHATWG Encoding Standard | WinterTC Minimum Common Web Platform API (UTF-8 subset) | `wpt-textencoder-utf8`, `wpt-textdecoder-utf8` | PASS (9/9) |
| `abort` | WHATWG DOM Standard — AbortController & AbortSignal | WinterTC Minimum Common Web Platform API | `wpt-abortcontroller-basic`, `wpt-abortsignal-static` | PASS (4/4) |
| `crypto` | W3C Web Cryptography API — Random Subset | WinterTC Minimal Web Crypto Subset | `wpt-crypto-getrandomvalues`, `wpt-crypto-randomuuid` | PASS (6/6) |
| `fetch` | WHATWG Fetch Standard | WinterTC Minimal Web Runtime — outbound fetch (M28) | `fetch-policy-security` | PASS (24/24) |

## Explicit Skips & Rationale (M27-010-B)

To prevent advertising unsupported APIs while maintaining honesty regarding web standards coverage, out-of-scope features are explicitly enumerated with machine-readable reason codes:

| Capability | Skip Identifier | Reason Code | Rationale | Deferred Target |
| :--- | :--- | :--- | :--- | :--- |
| `url` | `wpt-url-blob-scheme` | `BROWSER_ONLY_FEATURE` | Blob URLs require client-side Blob store; server-side POSIX runtime does not expose Blob storage. | OUT_OF_SCOPE |
| `url` | `wpt-url-file-windows-drive` | `POSIX_RUNTIME_TARGET` | Velqu targets POSIX/Linux execution; Windows file drive letters are excluded from URL normalization. | OUT_OF_SCOPE |
| `text_encoding` | `wpt-encoding-legacy-labels` | `WINTERTC_UTF8_ONLY` | WinterTC minimal web runtime profile specifies UTF-8 only; legacy multi-byte encodings fail closed with TypeError. | OUT_OF_SCOPE |
| `text_encoding` | `wpt-encoding-streaming` | `STREAMING_DEFERRED` | Request bodies in M27 are bounded memory buffers; chunked streaming reassembly is deferred to streaming milestones. | POST_M27 |
| `abort` | `wpt-abort-signal-any` | `ASYNC_COMBINATOR_DEFERRED` | Multiple signal composition is deferred to M28 native fetch and async combinator track. | M28 |
| `abort` | `wpt-abort-event-bubbling` | `MINIMAL_EVENT_TARGET` | Lightweight single-target event listener dispatch; hierarchical event propagation is not supported in minimal runtime. | OUT_OF_SCOPE |
| `crypto` | `wpt-crypto-subtle` | `UNSUPPORTED_CRYPTO_SUBTLE` | ADR-0018 / M27-008-D: Do not implement custom or complex cryptography; M27 provides only OS CSPRNG entropy. | GA_TRACK |
| `crypto` | `wpt-crypto-float-typedarray` | `SPEC_MANDATED_TYPE_ERROR` | Web Crypto specification mandates TypeError for floating-point and non-integer views; rejected fail-closed. | OUT_OF_SCOPE |
| `fetch` | `wpt-fetch-websockets` | `NO_WEBSOCKET_BETA` | WebSocket is out of the beta scope entirely (AGENTS.md constraint 15, ADR-0018); fetch stays request/response only. | POST_BETA |
| `fetch` | `wpt-fetch-sse` | `NO_SSE_BETA` | SSE requires a long-lived downstream connection per request; excluded from beta alongside WebSocket by AGENTS.md constraint 15. | POST_BETA |
| `fetch` | `wpt-fetch-request-streaming` | `NO_STREAMING_REQUEST_BODIES` | Request bodies are bounded in-memory buffers in beta; response streaming lands in M28-006 but request bodies stay bounded. | POST_M28 |
| `fetch` | `wpt-fetch-formdata` | `NO_FORMDATA_MULTIPART` | No FormData constructor or multipart encoder ships in beta; bodies are strings, bytes, or JSON. | POST_M28 |
| `fetch` | `wpt-fetch-blob-file` | `NO_BLOB_FILE_TYPES` | Server runtime has no Blob store (consistent with the url capability skip); Blob/File bodies fail closed at the policy layer. | OUT_OF_SCOPE |
| `fetch` | `wpt-fetch-serviceworkers` | `NO_SERVICE_WORKERS` | Service Workers are a browser lifecycle concept with no meaning in a compiled server runtime. | OUT_OF_SCOPE |
| `fetch` | `wpt-fetch-xhr` | `NO_XHR` | fetch supersedes XHR; WinterTC minimal profile excludes XHR and no compatibility layer ships. | OUT_OF_SCOPE |
| `fetch` | `wpt-fetch-http2` | `NO_HTTP2_UPSTREAM` | Outbound requests speak HTTP/1.1 in beta (ADR-0033 non-goals); HTTP/2 upstream is a GA-track evaluation. | GA_TRACK |
| `fetch` | `wpt-fetch-client-certs` | `NO_CLIENT_CERTS` | Client certificates are an ADR-0033 §6 non-goal for beta; only server-verified TLS with bundled roots ships. | GA_TRACK |
| `fetch` | `wpt-fetch-socks-proxy` | `NO_SOCKS_PROXY` | No ambient or explicit proxy support in beta (ADR-0033 §5); a proxy would move the trust boundary off the address classifier. | OUT_OF_SCOPE |
| `fetch` | `wpt-fetch-cookies` | `NO_COOKIE_JAR` | No persistent cookie store ships; cross-origin redirect credential stripping IS enforced, but cookie persistence is deferred. | POST_M28 |
| `fetch` | `wpt-fetch-br-zstd` | `NO_BR_ZSTD` | Only opt-in gzip is negotiated and decompressed, bounded by the response body limit (ADR-0033 §8). | POST_M28 |

## Test Suites & Executable Proofs

1. **TypeScript Conformance Suite**: `conformance/web-api/web-api.conformance.test.ts` (executes against the pinned JSON manifest).
2. **Rust Integration Conformance**: `crates/q-capabilities/tests/wpt_wintertc_conformance.rs` (executes against native model APIs).
3. **QuickJS Worker Integration**: `crates/q-engine-quickjs/src/worker.rs` (executes inside QuickJS context).

## Acceptance Guardrails (M27-010)

- **No unsupported API advertised**: Subsets strictly cover implemented primitives; no placeholder or unbacked APIs exist.
- **Pass/fail/skip counts are reproducible**: 58 pinned test vectors (100% PASS) + 20 explicit skips documented with rationale.
- **Behavioral regressions block relevant gate**: Conformance suite runs under standard `./scripts/verify` and `bun test`.
- **Reports link to exact runtime build**: Bound to `velqu-runtime` and `q-capabilities` at commit hash recorded in task ledger.
