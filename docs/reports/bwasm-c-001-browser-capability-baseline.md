# BWASM-C-001 — Browser capability baseline: timer, crypto, logging, restricted fetch

## Overview

Implements the mandatory browser capability baseline as a frozen capability
graph (`createBrowserCapabilityGraph`) in `@velqu/browser-runtime`,
mirroring the native runtime's frozen prelude graph (same entry names,
same `runtime:*` ids at version 1, native-parity bounds) with an explicit
security and resource policy. The adapters are declared, with versions,
in the deployment's `velqu-artifacts.json` (optional `capabilities` field
in the B-002 canonicalization — backward compatible), and availability is
introspectable before any handler executes.

## The four baseline capabilities

| id | adapter | policy |
|---|---|---|
| `runtime:timers` v1 | browser `setTimeout` with bounded ceiling (≤ 300s, MAX_OP_DEADLINE_MS parity); `delay(ms, {signal})` | cancellation clears the timer — the work never fires (typed `TimerCancelled`); oversized delays rejected before scheduling |
| `runtime:crypto` v1 | WebCrypto subset: `getRandomValues` (≤ 65_536 B identity bound) + `digest` for SHA-256/384/512 (hex, native digest parity; "abc" pinned to the FIPS 180-4 vector shared with the native suite) | everything else (encrypt/sign/HMAC/…) rejected by name with `CryptoSemanticsMismatch` — never silently substituted |
| `runtime:console` v1 | console-shaped bounded structured logging: native-parity redactor port (auth headers, API-secret prefixes, key=value secrets), 16_384 B message truncation + `...[TRUNCATED]`, 32-arg ceiling, 64-record budget, injectable bounded host sink (default ring 128), correlation ids | floods produce typed `ConsoleLimitExceeded`; records are structured `{level, message, truncated, correlationId?}` |
| `runtime:fetch` v1 | outbound fetch with **default-deny origin allowlist** (empty = deny all), method allowlist (default GET/HEAD), scheme allowlist http/https (native parity), 16 MiB request/response ceilings (native parity), total deadline raced in-adapter (transport-independent), credentials forced to omit + credential headers stripped, redirects denied | every denial typed: `FetchPolicyDenied` / `FetchLimitExceeded` |

Plus ambient `runtime:abort`/`runtime:text`/`runtime:url` entries (standard
globals, frozen — native graph parity, no added authority).

## Integration

- **Manifest declaration**: `emitArtifactManifest` accepts optional
  `capabilities` (id+version list); present in the canonical bytes only
  when provided — legacy manifests canonicalize identically (pinned).
  The declared buildId differs from an undeclared build (canonical bytes
  differ) and verifies end-to-end through the B-002 loader.
- **Compose (B-005)**: `velqu build --target browser-wasm` declares the
  full baseline in `velqu-artifacts.json`; the generated page installs the
  graph before boot (`--fetch-allowlist` option wires the fetch policy;
  default empty = deny). `velqu inspect browser` reports the declared
  adapters.
- **Registry**: `baselineHandles(graph)` adapts the graph to R-005's
  `CapabilityHandle.call(input)` contract for deployments routing the
  baseline through kernel authorization.

## Acceptance criteria

- ✅ No adapter exposes editor credentials, ambient cookies, storage, DOM,
  or unrestricted network (fetch is default-deny; credentials forced omit;
  no storage/DOM in the graph).
- ✅ Timer and fetch stop or discard work after cancellation/deadline
  (cleared timer never fires; fetch deadline raced in-adapter, response
  read bounded with truncation marker).
- ✅ Crypto mismatch is rejected by name, never substituted (pinned subset
  = SHA-2 digests + random; the native/browser delta is documented).
- ✅ Log and response floods are bounded with structured limit errors
  (`ConsoleLimitExceeded`, `FetchLimitExceeded` — code-carrying).
- ✅ Fetch credentials default to omit and redirects cannot escape policy
  (redirect:"error" + adapter-level 3xx refusal — stricter than native
  Manual, delta documented).
- ✅ Capability availability is introspectable before handler execution
  (frozen descriptors + `describe()` at installation; declared in the
  artifact manifest; reported by `velqu inspect browser`).

## Test evidence

23 new tests (`packages/browser-runtime/test/capabilities.test.ts`):
timer resolve/bounds/cancel/pre-abort; SHA-256 FIPS vector + SHA-384/512 +
byte encoding + named rejections + random bounds; redaction property
parity with the native suite (same `!contains(secret)` +
`contains("key=[REDACTED]")` style), truncation, budget flood, arg
ceiling; fetch allow/deny/origin/method/scheme/credentials/body/response
matrix (evidence: `evidence/capabilities/c001/02-policy-traces.txt`),
deadline race, redirect denial, policy summary; graph introspection,
end-to-end through the frozen graph, manifest canonicalization round trip
(with/without capabilities, legacy compatibility), registry handles.
Full browser-runtime package 121/121; CLI browser-deploy suite 35/35
(incl. adapter-declaration assertions); `tsc -b` clean.

Evidence: `docs/codex-spark-browser-wasm/evidence/capabilities/c001/`
(`01-capability-tests.txt`, `02-policy-traces.txt`,
`03-adapter-manifest-example.json`).

## Honesty notes

- **Browser fetch redirect delta**: native Velqu returns 3xx responses to
  the caller (Manual policy); a browser fetch cannot expose cross-origin
  redirect chains for per-hop revalidation, so the browser adapter DENIES
  redirects outright (`redirect: "error"` + 3xx refusal). Stricter than
  native, documented — not claimed as equivalent.
- **Crypto parity is pinned, not total**: only SHA-2 digests + random are
  exposed because only those semantics are pinned against shared vectors;
  HMAC (native HMAC-SHA-256) is deliberately absent until shared vectors
  justify it. No cryptographic-equivalence claim beyond the pinned ops.
- **Bun fidelity gaps**: Bun's `Request` does not reflect `credentials`/
  `redirect` init fields back — the adapter additionally enforces the
  policy itself (header stripping, 3xx refusal, deadline race) so the
  contract holds on any transport; real browsers enforce the init fields.
- The colon+space secret form (`"password: x"`) leaks in the native
  redactor as well; the port preserves native parity (the `=` form is
  covered) — noted as a native-side limitation, not hidden.
- Handlers currently reach capabilities through the page-installed graph;
  worker-realm callback wiring remains future work (R-004/Q-phase), and
  C-005 owns fail-closed routing for deployment-required capabilities.

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
