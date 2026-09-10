# BWASM-R-002 — Fetch-Compatible Browser Dispatcher

## Result

**PASS** — the dispatcher owns the JS/WASM boundary end to end:
normalization (URL/method/headers/query/body/abort), bounded body-form
handling, HEAD/OPTIONS policy, deterministic header behavior, and the
kernel round-trip. **Every routing/validation/capability decision stays
in the kernel** — verified structurally by the recording-kernel tests
(no outcome is produced without a kernel call) and by the native/browser
fixture diff against the REAL kernel over the proof pack.

## Deliverable

`packages/browser-runtime/src/dispatcher.ts` + `BrowserRuntime.fetch`
rewiring (R-001 seam replaced; options extended with `maxBodyBytes`,
`signal`, `executeHandler` — the R-004 Worker seam):

- **Normalization**: URL→path+query (≤256 pairs), headers (≤128 pairs,
  platform-joined duplicates — deterministic, fixture-locked), method,
  bounded body before anything crosses the ABI.
- **Body forms (per the frozen matrix)**: text and JSON cross verbatim
  (kernel validates); URL-encoded parses to a last-wins record crossing
  as JSON; multipart crosses as **bounded metadata only** (≤64 parts,
  ≤8 KiB part headers — names + content types); bodies capped at 1 MiB
  default.
- **Unsupported semantics inventory** (committed, tested count):
  request streaming, binary bodies, full multipart parsing, response
  streaming — each fail-closed with typed problems, never silently
  degraded.
- **Abort contract**: `AbortError` before dispatch (kernel untouched)
  and during dispatch (after normalization, before the kernel) — both
  fixture-tested; platform-standard `DOMException`.
- **HEAD**: dispatches through the kernel (HEAD→GET routing is
  K-003/kernel-side), response materialized bodyless.
- **OPTIONS**: no dispatcher special case — the kernel's 405+Allow
  problem carries it (fixture-locked).
- **No JS fast path**: the dispatcher never decides routing or
  validity; problem mapping reproduces kernel problems verbatim
  (`application/problem+json`, kernel status, `Allow` passthrough).

## Native/browser fixture diff (real kernel, same pack)

`packages/browser-runtime/test/fixtures/dispatcher-native-diff.sh` +
`dispatcher-corpus.mjs` → evidence
`docs/browser-wasm/evidence/dispatcher/native-browser-diff.txt`:

- Native: `velqu-runtime` serving `examples/proof/dist/app.qpack`
  (runtime + curls inside one netns).
- Browser: the R-002 dispatcher normalization driving the **real
  q-browser-kernel wasm** (nodejs glue, hashes recorded).
- **9-entry corpus** (routes, HEAD, POST/OPTIONS wrong-method,
  unknown paths, trailing slash): plan-level outcomes
  (ROUTE vs PROBLEM(status, problemId)) **identical** —
  `NATIVE-BROWSER-DIFF-OK`.
- **Expected difference documented** (excluded from the corpus, in the
  evidence file): `/health/ready` is a native HOST-LEVEL socket probe
  (`ops/routes.ts` comment; artifact route is `/ops/readiness`) — the
  browser kernel correctly 404s it. Native-only surface per ADR-0037 §1.
- Handler **outputs** are out of scope (browser handler execution is
  R-003/R-004); during this run the kernel's response-schema
  enforcement was observed live — stub bodies on schema'd routes
  correctly produced contract-violation problems.

## Acceptance evidence

| Criterion | Evidence |
|---|---|
| Static + parameterized routes, production-equivalent precedence | corpus diff (ROUTE parity) + recording-kernel tests (dispatch goes through the kernel for every outcome) |
| 405/Allow, OPTIONS, HEAD, trailing slash, duplicate headers fixture-locked | dispatcher tests: OPTIONS 405+Allow, HEAD bodyless + kernel-side method, `/items/` trailing-slash corpus row, duplicate-header joined form |
| Schema failures use the canonical problem shape | kernel problems verbatim (`problem+json`, native URIs); undeclared-status contract violation test |
| Abort before/during dispatch → documented cancellation | both phases tested (AbortError; kernel untouched — zero recorded plans) |
| No JS-only fast path | recording-kernel authority test; dispatcher source contains no routing logic |

Tests: **27/27** (`bun test packages/browser-runtime`); typecheck
clean; real-browser lanes remain Q-002 (browser-target bundle +
standards-surface execution documented in R-001, unchanged).

## Gates

fmt/clippy/okf clean; `./scripts/verify` ALL PASS (netns; two-pass
manifest refresh if needed). Standing CI disclosure applies (zero-step
verify workflows since ~#714); local gates are the acceptance basis.
