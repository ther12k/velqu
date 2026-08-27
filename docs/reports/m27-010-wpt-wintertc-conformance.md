# M27-010 Web API Conformance — Pinned WPT & WinterTC Subsets

Programmatic conformance baseline pinning Web Platform Tests (WPT) and WinterTC Minimum Common Web Platform API test subsets for Velqu M27 capabilities.

## Standards and Pinned Manifest

Pinned test vectors are formally declared in [`conformance/web-api/wpt-manifest.json`](../../conformance/web-api/wpt-manifest.json).

| Capability | Upstream Standard | WinterTC Profile | Pinned Subset ID | Status |
| :--- | :--- | :--- | :--- | :--- |
| `url` | WHATWG URL Standard | Minimum Common Web Platform API | `wpt-url-resolution`, `wpt-url-normalization`, `wintertc-urlsearchparams` | PASS (15/15) |
| `text_encoding` | WHATWG Encoding Standard | Minimal Web Runtime (UTF-8 subset) | `wpt-textencoder-utf8`, `wpt-textdecoder-utf8` | PASS (9/9) |
| `abort` | WHATWG DOM Standard | Minimal Web Runtime | `wpt-abortcontroller-basic`, `wpt-abortsignal-static` | PASS (4/4) |
| `crypto` | W3C Web Cryptography API | Minimal Web Crypto Subset | `wpt-crypto-getrandomvalues`, `wpt-crypto-randomuuid` | PASS (6/6) |

## Test Suites & Executable Proofs

1. **TypeScript Conformance Suite**: `conformance/web-api/web-api.conformance.test.ts` (executes against the pinned JSON manifest).
2. **Rust Integration Conformance**: `crates/q-capabilities/tests/wpt_wintertc_conformance.rs` (executes against native model APIs).
3. **QuickJS Worker Integration**: `crates/q-engine-quickjs/src/worker.rs` (executes inside QuickJS context).

## Acceptance Guardrails (M27-010)

- **No unsupported API advertised**: Subsets strictly cover implemented primitives; no placeholder or unbacked APIs exist.
- **Pass/fail/skip counts are reproducible**: All 34 pinned test vectors across 4 capabilities evaluate deterministically in CI and local test harnesses.
- **Behavioral regressions block relevant gate**: Conformance suite runs under standard `./scripts/verify` and `bun test`.
- **Reports link to exact runtime build**: Bound to `velqu-runtime` and `q-capabilities` at commit hash recorded in task ledger.
