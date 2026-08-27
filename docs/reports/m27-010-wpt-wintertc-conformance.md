# M27-010 Web API Conformance — Pinned WPT & WinterTC Subsets

Programmatic conformance baseline pinning Web Platform Tests (WPT) and WinterTC Minimum Common Web Platform API test subsets for Velqu M27 capabilities.

## Standards and Pinned Manifest

Pinned test vectors and explicit skips are formally declared in [`conformance/web-api/wpt-manifest.json`](../../conformance/web-api/wpt-manifest.json).

| Capability | Upstream Standard | WinterTC Profile | Pinned Subset ID | Status |
| :--- | :--- | :--- | :--- | :--- |
| `url` | WHATWG URL Standard | Minimum Common Web Platform API | `wpt-url-resolution`, `wpt-url-normalization`, `wintertc-urlsearchparams` | PASS (15/15) |
| `text_encoding` | WHATWG Encoding Standard | Minimal Web Runtime (UTF-8 subset) | `wpt-textencoder-utf8`, `wpt-textdecoder-utf8` | PASS (9/9) |
| `abort` | WHATWG DOM Standard | Minimal Web Runtime | `wpt-abortcontroller-basic`, `wpt-abortsignal-static` | PASS (4/4) |
| `crypto` | W3C Web Cryptography API | Minimal Web Crypto Subset | `wpt-crypto-getrandomvalues`, `wpt-crypto-randomuuid` | PASS (6/6) |

## Explicit Skips & Rationale (M27-010-B)

To prevent advertising unsupported APIs while maintaining honesty regarding web standards coverage, out-of-scope features are explicitly enumerated with machine-readable reason codes:

| Capability | Skip Identifier | Reason Code | Rationale | Deferred Target |
| :--- | :--- | :--- | :--- | :--- |
| `url` | `wpt-url-blob-scheme` | `BROWSER_ONLY_FEATURE` | Blob URLs require client-side Blob store; server runtime does not expose Blob storage. | OUT_OF_SCOPE |
| `url` | `wpt-url-file-windows-drive` | `POSIX_RUNTIME_TARGET` | Velqu targets Linux/POSIX server runtimes; Windows drive letter normalization excluded. | OUT_OF_SCOPE |
| `text_encoding` | `wpt-encoding-legacy-labels` | `WINTERTC_UTF8_ONLY` | WinterTC minimal profile specifies UTF-8 only; legacy multi-byte labels fail closed with TypeError. | OUT_OF_SCOPE |
| `text_encoding` | `wpt-encoding-streaming` | `STREAMING_DEFERRED` | Request bodies in M27 are bounded memory buffers; chunked streaming decoders deferred. | POST_M27 |
| `abort` | `wpt-abort-signal-any` | `ASYNC_COMBINATOR_DEFERRED` | Multi-signal combinator composition is deferred to M28 native fetch / async combinator track. | M28 |
| `abort` | `wpt-abort-event-bubbling` | `MINIMAL_EVENT_TARGET` | Single-target event listener dispatch; hierarchical event tree propagation excluded. | OUT_OF_SCOPE |
| `crypto` | `wpt-crypto-subtle` | `UNSUPPORTED_CRYPTO_SUBTLE` | ADR-0018 / M27-008-D constraint: no custom/complex cryptography; OS CSPRNG entropy only. | GA_TRACK |
| `crypto` | `wpt-crypto-float-typedarray` | `SPEC_MANDATED_TYPE_ERROR` | Web Crypto specification mandates TypeError for floating-point and non-integer views. | OUT_OF_SCOPE |

## Test Suites & Executable Proofs

1. **TypeScript Conformance Suite**: `conformance/web-api/web-api.conformance.test.ts` (executes against the pinned JSON manifest).
2. **Rust Integration Conformance**: `crates/q-capabilities/tests/wpt_wintertc_conformance.rs` (executes against native model APIs).
3. **QuickJS Worker Integration**: `crates/q-engine-quickjs/src/worker.rs` (executes inside QuickJS context).

## Acceptance Guardrails (M27-010)

- **No unsupported API advertised**: Subsets strictly cover implemented primitives; no placeholder or unbacked APIs exist.
- **Pass/fail/skip counts are reproducible**: 34 pinned test vectors (100% PASS) + 8 explicit skips documented with rationale.
- **Behavioral regressions block relevant gate**: Conformance suite runs under standard `./scripts/verify` and `bun test`.
- **Reports link to exact runtime build**: Bound to `velqu-runtime` and `q-capabilities` at commit hash recorded in task ledger.
