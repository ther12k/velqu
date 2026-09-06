# BWASM-R-001 — @velqu/browser-runtime Package and Public Runtime Contract

## Result

**PASS** — `@velqu/browser-runtime` exists as a browser-only package
whose public surface is exactly the frozen ADR-0037 §2 boundary:
`fetch(request: Request): Promise<Response>`, plus the lifecycle,
structured-error, and kernel-ABI contracts around it. 12/12 package
tests, typecheck clean, clean-consumer fixture **bundled for the
browser target (5.4 KB) and executed** with standard Request/Response.

## Package surface

`packages/browser-runtime/src/index.ts`:

- **`createBrowserRuntime(options): BrowserRuntime`** — verifies the
  kernel ABI (mismatch ⇒ structured `KERNEL_ABI_MISMATCH` at create
  time), initializes the kernel with the artifact bytes (rejection ⇒
  `ARTIFACT_REJECTED` carrying the kernel's artifact problem), and
  yields a runtime in `ready`.
- **`BrowserRuntime`** — `state` (`idle→ready→disposed`, forward-only),
  `abiVersion`, `fetch(Request)`, `authorizeCapability(name)`
  (fail-closed bridge query), `dispose()` (idempotent).
- **`fetch` path**: standard `Request` → query/headers extraction →
  bounded kernel `plan_request` message → problem mapping to
  `application/problem+json` Responses (kernel status, `Allow` on 405)
  or invoke plan → kernel `complete_invocation` → standard `Response`.
  Handler execution is the documented R-002 seam
  (`executeHandler` resolves the declared-default completion so the
  complete path — including declared-status enforcement — is real
  today, not stubbed).
- **`BrowserRuntimeError`** — structured (`code`, `status`, optional
  kernel `problem`); codes: KERNEL_ABI_MISMATCH, ARTIFACT_REJECTED,
  RUNTIME_DISPOSED, RUNTIME_NOT_READY, REQUEST_INVALID, KERNEL_PROBLEM,
  KERNEL_PROTOCOL. Never console-only.
- **Kernel ABI types** mirror K-005 exactly (`KernelModule`,
  `KernelInstance`, plan/completion message shapes). The kernel module
  is injected — the package owns no .wasm loading (that is the
  BWASM-B-002 artifact/loader contract), keeping the graph bundler-clean.

## Acceptance evidence

| Criterion | Evidence |
|---|---|
| Clean consumer imports from a browser bundle | `test/fixtures/consumer.ts` → `bun build --target browser` → 5,396 B bundle; **executed**: `CONSUMER-SMOKE-OK` (ready/200/body through the full seam); bundle scan: **0** `node:`/`Bun.`/`require(` occurrences |
| Public entry accepts Request, resolves Response | contract tests: `fetch(Request)` → `Response` (status/headers/body); problems → `application/problem+json` with kernel status + `allow` |
| Dependency graph purity | purity test: mechanical source scan (no `bun*`, `node:*`, `@velqu/testing`, q-http/q-runtime imports, `process.`, `__dirname/__filename`); `package.json` declares a single browser entry, zero scripts |
| Explicit exports + tested declarations | single explicit export map (`.`); typecheck green (`tsc -b`) |
| Structured lifecycle errors | tests: ABI mismatch, artifact rejection (problem surfaced structurally), disposed-runtime fail-closed, kernel-protocol violations never become success |

Commands: `bun test packages/browser-runtime` (12/12),
`./node_modules/typescript/bin/tsc -b tsconfig.json` (exit 0), the
bundler smoke above. Full battery run for handoff (see below).

## Honest boundaries

- Real-browser execution lanes are BWASM-Q-002 scope; the smoke runs
  the browser-target bundle under Node 22's standard web APIs
  (Request/Response/fetch globals — the same standards surface).
- `executeHandler` is the R-002 seam, explicitly documented as such in
  source; R-001 deliberately does not implement Worker execution.
- The runtime injects the kernel module; binding the compiled kernel
  glue into a distributable artifact set belongs to B-002/R-002.

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
