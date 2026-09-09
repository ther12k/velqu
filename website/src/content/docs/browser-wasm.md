---
title: "Browser-WASM"
description: "Build a Velqu app to static assets that run in an ordinary browser — Rust/WASM kernel, isolated Worker handlers, no Velqu application server."
---
This document is the canonical reference for compiling, deploying, running, and migrating Velqu applications targeting the **Browser-WASM** runtime.

---

## 1. Architecture & Execution Model

Velqu Browser-WASM implements a **hybrid architecture** that brings Velqu's contract-first routing and validation directly into modern web browsers:

```text
[ Incoming Request (fetch) ]
           │
           ▼
[ Service Worker (B-004) ] ── (Asset / Cache Storage)
           │ (API Route)
           ▼
[ Wasm Kernel (K-005) ] ── Rust compiled to wasm32-unknown-unknown
  ├─ URL routing & method matching
  ├─ Schema validation (q-schema-runtime)
  ├─ QPack artifact verification & integrity checks
  ├─ Capability authorization
  └─ RFC-9457 problem envelope mapping
           │
           ▼ (Invoke Plan via postMessage)
[ Isolated Worker Host (R-004) ]
  └─ Generated TypeScript handler execution (app.browser.js)
```

### Key Architectural Invariants

1. **Rust / Wasm Authority**: Routing, path/query/header/body validation, capability authorization, and RFC-9457 problem generation execute inside the compiled WebAssembly kernel (`q_browser_kernel_bg.wasm`). No JavaScript "fast path" bypasses the kernel.
2. **Dedicated Handler Worker**: Handlers execute inside an isolated Web Worker (`worker.js`). Communication occurs strictly via structured-clone messages with bounded sizes and enforced deadlines.
3. **Public Seam**: The public boundary is `Request -> Promise<Response>`. In-page callers interact with standard Web APIs without proprietary protocol shims.
4. **No Application Server**: Browser-WASM runs entirely client-side on static hosting (CDN, S3, GitHub Pages, Cloudflare Pages). It requires no Node.js or Velqu application server to handle runtime requests.

---

## 2. Quickstart

### Prerequisites

- [Bun](https://bun.sh) v1.4.0+
- A Velqu project with routes declared via `@velqu/core` and `@velqu/schema`

### Authoring Requirement: Export Route Bindings

Every route declaration must be reachable through its source module's export
surface: declare routes as `export const tick = route({...})`, re-export them
in the same module (`export { tick }`), or use `export default route({...})`.
The browser build fails closed with a source-located error on non-exported
route bindings — the generated handler bundle imports each binding by name
from its module, and a non-exported const would silently resolve to
`undefined` at runtime (#1292).

### Step-by-Step Workflow

```bash
# 1. Build the Browser-WASM deployment bundle
bun packages/cli/src/index.ts build --target browser-wasm --project my-app

# 2. Inspect deployment integrity and capability status
bun packages/cli/src/index.ts inspect browser --project my-app

# 3. Preview locally on a static development server
bun packages/cli/src/index.ts preview --project my-app --port 8080

# 4. Export verified deployment assets for static hosting
bun packages/cli/src/index.ts export --project my-app --out dist/static-site
```

### Generated Artifact Structure

```text
dist/browser/
├── velqu-artifacts.json     # Content-addressed SHA-256 artifact manifest (buildId)
├── kernel.wasm              # Pinned Rust/Wasm compatibility kernel (400 KB brotli)
├── kernel.js                # Browser Wasm initialization glue
├── page.js                  # Runtime boot and Service Worker registration bootstrap
├── worker.js                # Handler Worker entry and supervisor protocol listener
├── service-worker.js        # Scope-bounded caching, offline fallback, and update SW
├── app.qpack                # Verified immutable route and schema pack
├── app.bundle.js            # Bundled application dependencies
└── index.html               # Minimal preview shell
```

---

## 3. Static Hosting, HTTPS, and Service Worker Lifecycle

### Static Host Requirements

- **HTTPS Required**: Modern browsers require a Secure Context (HTTPS or `localhost`) to register Service Workers and instantiate WebAssembly with high-resolution timers.
- **MIME Types**:
  - `.wasm` must be served with `application/wasm`.
  - `.js` must be served with `application/javascript` or `text/javascript`.
  - `.json` must be served with `application/json`.
- **Base Paths**: For deployments served under a subpath (e.g. `https://example.com/app/`), pass `--base-path /app/` during build.

### Service Worker Scope & Passthrough Boundaries

The Service Worker operates under strict path-segment boundaries:
- Scope `/app/` intercepts `/app/...` and `/app`, but passes through sibling paths such as `/app2/...`.
- Non-application endpoints (`/__velqu_editor__`, `/auth/`, `/oauth/`, `/model-gateway/`) pass through to the network untouched.

### Injected-Fetch Fallback Mode

If Service Workers are unavailable (e.g., private browsing mode, disabled by user policy, or unsupported browser), the runtime reports `sw: injected-fetch-fallback`. In this mode:
- Direct in-page calls (`runtime.fetch()`) continue to operate through the Wasm kernel and Worker.
- Navigation interception is omitted, preventing hanging preview windows.

### Updates and Rollback

- **Deploy Identity**: Each deployment embeds the SHA-256 hash of `velqu-artifacts.json` into `service-worker.js`.
- **Apply on Next Reload**: When a new deployment is detected, the waiting Service Worker activates upon user reload (`VELQU_APPLY_UPDATE`), preventing mid-session state corruption.
- **Rollback**: Previous build caches are retained (`cachesToKeep`: active + 1 prior build). Redeploying the previous manifest immediately serves the prior verified build offline.

---

## 4. Supported APIs, Persistence, and Capabilities

### Request & Response Parity

| Surface | Browser-WASM Support | Notes |
|---|---|---|
| **Methods** | `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `HEAD`, `OPTIONS` | 405 Method Not Allowed carries declared `Allow` header |
| **JSON Bodies** | Fully supported (`ctx.body`) | Validated by Wasm kernel schema engine |
| **URL-Encoded** | Fully supported | Normalized into JSON representation |
| **Multipart** | Metadata-only | Bounded part parsing; large binary streaming not supported |
| **Streaming** | Request/response buffering | Kernel ABI operates on bounded messages; streaming has no frozen MVP contract |
| **Path Params** | Kernel validates | Handlers receive validated route context |
| **Cookies** | Sanitized / Omitted | Ambient cookies are not attached to outbound capability fetch |

### Local Persistence (IndexedDB KV)

Projects enabling local storage (`--kv`) receive namespaced, client-side persistence:
- **Namespace Isolation**: Stores are scoped to `kv:${appId}:${namespace}`. One project cannot enumerate or access another project's keys.
- **Adapters**:
  - `createIndexedDbKv()`: Persistent, namespaced storage in IndexedDB.
  - `createMemoryKv()`: Ephemeral in-memory storage for test and private-mode fallbacks.
- **Limits**: Key size <= 1,024 bytes, value size <= 512 KiB, total entries <= 10,000.
- **Not Production SQL**: Preview KV is local-only and not multi-user durable.

### Outbound Fetch Capability (`runtime:fetch v1`)

Browser-safe outbound network calls are strictly mediated:
- **Default Deny**: Outbound origins must be explicitly allowlisted (`fetchPolicy.allowedOrigins`).
- **Credential Stripping**: `credentials: "omit"` is enforced at request creation; `cookie` and `authorization` headers are stripped.
- **Redirect Denial**: 3xx redirects are rejected (`redirect: "error"`) to prevent credential leakage or unintended origin escapes.
- **Deadlines**: Requests are capped at `deadlineMs` (default 30s).

---

## 5. Developer Observability & Diagnostics

The Browser-WASM runtime provides machine-readable diagnostics across **12 frozen lifecycle stages**:
`load` · `verify` · `instantiate` · `route` · `validate` · `invoke` · `capability` · `persist` · `cache` · `update` · `cancel` · `fail`

### Diagnostic Codes Catalog

The runtime emits 35 stable `DIAG_*` codes distinguishing 10 distinct failure categories:

- **Integrity**: `DIAG_VERIFY_INTEGRITY_FAIL`, `DIAG_VERIFY_INTEGRITY_OK`
- **Compatibility**: `DIAG_COMPAT_KERNEL_ABI_MISMATCH`, `DIAG_COMPAT_HANDLER_ABI_MISMATCH`
- **Routing**: `DIAG_ROUTE_NOT_FOUND` (404), `DIAG_ROUTE_METHOD_NOT_ALLOWED` (405)
- **Validation**: `DIAG_VALIDATE_FAILED` (422)
- **Capability**: `DIAG_CAPABILITY_DENIED`, `DIAG_CAPABILITY_DEPLOYMENT_REQUIRED`
- **Invocation**: `DIAG_INVOKE_START`, `DIAG_INVOKE_SUCCESS`, `DIAG_INVOKE_FAILED`
- **Timeout**: `DIAG_INVOKE_TIMEOUT` (504)
- **Persistence**: `DIAG_PERSIST_ERROR`, `DIAG_PERSIST_QUOTA_EXCEEDED`, `DIAG_PERSIST_MIGRATION_REQUIRED`
- **Cache**: `DIAG_CACHE_HIT`, `DIAG_CACHE_MISS`, `DIAG_SW_REGISTERED`
- **Failure**: `DIAG_FAIL_INTERNAL`, `DIAG_FAIL_REDACTED`

### Cross-Boundary Correlation

Incoming `x-correlation-id` or `x-request-id` headers are automatically preserved across Service Worker, Wasm kernel, Worker host, and outbound capabilities.

### Diagnostic Stream & Inspector

```ts
import { createDiagnosticStream, createInspectorAdapter } from "@velqu/browser-runtime";

const stream = createDiagnosticStream({ capacity: 256, minLevel: "info" });
const inspector = createInspectorAdapter(stream);

// Export machine-readable trace
const traceJson = inspector.exportJson();

// Render text summary or safe HTML panel
const textReport = inspector.renderTextReport();
```

---

## 6. Browser Support Matrix & Evidence Tiers

Support claims correspond to maintained CI and rehearsal evidence:

| Tier | Platform | Support Status | Blocking Evidence Lane |
|---|---|---|---|
| **Tier 1 (Tested)** | Chromium / Edge desktop (Linux x64, macOS, Windows) | **Tested** | Required CI blocking gate (`scripts/browser-e2e-rehearsal.py`) |
| **Tier 2 (Experimental)** | Firefox desktop | Experimental | Experimental lane defined; continue-on-error |
| **Tier 3 (Experimental)** | Safari / WebKit desktop | Experimental | Experimental lane defined; continue-on-error |
| **Tier 4 (Mobile)** | Chrome Android, Safari iOS | Untested | Requires mobile device qualification; desktop results do not certify mobile |
| **Out of Scope** | Browsers without Wasm / Workers / Streams | Unsupported | Runtime fails closed with actionable error |

---

## 7. Limitations & Honest Non-Goals

1. **Trusted Code Only — Not a Hostile-Code Sandbox**:
   - The isolated Worker and WebAssembly kernel execute trusted application code. They provide fault isolation, deadline enforcement, and crash recovery, but **do not guarantee protection against malicious, adversarial tenant code** sharing the browser origin.
   - For multi-tenant isolation, host previews on distinct, isolated origins (e.g. `preview-<id>.example.com`) inside sandboxed iframes (`sandbox="allow-scripts"`).
2. **No In-Browser Postgres**:
   - PostgreSQL capability (`runtime:postgres`) cannot run directly in the browser MVP without a native host.
   - Routes requiring Postgres fail closed at build time, returning a stable `501 Not Implemented` (`https://velqu.dev/problems/deployment-required`) RFC-9457 problem.
3. **QuickJS-WASM Status**:
   - The Browser-WASM MVP executes generated TypeScript handlers in a standard Web Worker. QuickJS-in-WASM is optional and remains behind an Owner-ratified decision gate.
4. **No Serverless / Zero-Server Claims Without Qualification**:
   - "Zero server" refers exclusively to the absence of a dedicated Velqu application process. A static web server or CDN is always required to host and deliver application assets.
5. **No Native Performance Parity Claims**:
   - Latency measurements (p50 ~0.3 ms, p99 ~1.4 ms) represent in-browser loopback dispatch, not high-throughput network server benchmarks.

---

## 8. Migration Guide (Preview to Native Production)

Velqu enables a seamless path from browser-local preview to native production deployment:

### Step 1: Make PostgreSQL Calls Asynchronous

As part of the C-002 contract freeze, all database queries use asynchronous settlement:

```ts
// Modern async contract (compatible across targets)
const result = await ctx.native.postgres.sql`SELECT id, name FROM users WHERE id = ${params.id}`;
```

### Step 2: Separate Preview Persistence from Production DB

```ts
// Use namespaced KV for browser preview:
const kv = createIndexedDbKv({ namespace: "app:state" });

// Reserve Postgres for deployed production routes:
// (Routes using Postgres will be flagged as deployment-required during browser export)
```

### Step 3: Inspect Deployment Requirements

Run the CLI inspection command before shipping:

```bash
velqu inspect browser --project my-app --json
```

If any route relies on production-only capabilities, `deploymentRequirements` will list the required native configurations and explain why browser execution was refused.

### Step 4: Native Production Build

Deploy production applications using the native target:

```bash
velqu build --target serverless --project my-app
# Produces app.qpack for execution on native velqu-runtime
```
