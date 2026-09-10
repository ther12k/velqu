# Velqu Browser-WASM — External Cleanroom Evaluation Report

Evaluator: external cleanroom user (no access to implementation repo).
Date: 2026-09-07. Environment: Bun 1.4.0, Python 3.12 + Playwright, Chromium, Linux x64.
Inputs: 4 markdown docs (`BROWSER_WASM.md`, `QUICKSTART.md`, `INSTALL.md`, `KNOWN-LIMITATIONS.md`),
10 `.tgz` archives in `registry/`, 3 kernel artifacts in `kernel/`.

## Verdict up front

**A determined external user can complete the build → deploy → offline-reload → persistence
journey, but only by reading package source (not docs) and performing two undocumented
workarounds (manual tarball extraction, vendoring the kernel).** The kernel/WASM/SW/IndexedDB
machinery itself worked impressively well — first-load boot, integrity verification, offline
reload from cache, and KV persistence all succeeded in real Chromium. However, three handler-facing
defects (`status().value()`, `status().problem()`, and the timer capability) break documented
framework features, and the generated preview page offers no way to exercise any route other
than the build-time probe.

## Step-by-step narrative

### 1. Reading the docs (15 min)

- `BROWSER_WASM.md` is the only browser-relevant doc; it documents architecture, CLI workflow
  (`build --target browser-wasm`, `inspect browser`, `preview`, `export`), static-host
  requirements (HTTPS/localhost, MIME types, `--base-path /app/`), IndexedDB KV, and the
  Postgres deployment-required refusal. Good conceptual coverage.
- **Gap:** it never shows how to *author* an app. No route-declaration syntax, no project
  layout, no handler example. All commands are written as `bun packages/cli/src/index.ts …`
  (monorepo checkout paths) even though `INSTALL.md` says npm packages exist as tarballs.
- `QUICKSTART.md` and `INSTALL.md` describe only the **native Rust** target (cargo build,
  velqu-runtime binary). A browser-target user can ignore them, but their presence without
  any browser quickstart is confusing.
- No doc mentions the `--probe-path`/`--probe-method` flags or that the generated page
  exposes no runtime handle. I found them only by reading `@velqu/cli` source.

### 2. Installing the packages — BLOCKED, worked around (defect D1)

`bun add /tmp/velqu-cleanroom/registry/velqu-*.tgz` **failed for every tarball** with
`"package.json" failed to open: ENOENT` / `Invalid dependency name`. Cause: the archives have
`package.json` and `src/` at the tarball root instead of under the npm-standard `package/`
prefix, so they are not installable by npm/Bun. Additionally every package.json is
`"private": true` with `"workspace:*"` inter-dependencies, which would fail outside a
workspace even with a correct layout.

**Workaround (user workaround, ~2 commands):** manually
`tar -xzf velqu-<name>.tgz -C node_modules/@velqu/<name>`.

Also required: `typescript` must be **exactly 5.9.3** (the compiler enforces a toolchain pin;
`bun init` had pulled TS 7.0.2 → `check` failed with a clear toolchain-mismatch error). Bun
1.4.0 matched the pin. The error message was actionable and correct.

### 3. Authoring the app — no docs, source-reading required (defect D2)

I reverse-engineered the authoring API from `@velqu/core`, `@velqu/schema`, and the CLI's
embedded scaffold template:

- `src/app.ts` (entry candidates `src/app.ts`, `app.ts`, `src/index.ts`) with
  `defineApp({ id, modules: [defineModule({ id, routes })] })`.
- Routes via `route({ id, method, path, params?, query?, body?, response, handle })`,
  schemas via `s.object/s.string/s.integer/s.optional(...)`.
- Postgres capability: any `ctx.native.postgres` property access in a handler marks the route
  `runtime:postgres` (deployment-required). Timer: `ctx.native.timer.delay(ms)`.
- KV for handlers: `createIndexedDbKv({ namespace })` imported from `@velqu/browser-runtime`
  **in handler module scope** — the docs describe the adapter but never say where to
  construct it; the `--kv` CLI flag only creates an unused KV handle inside the generated
  page shell (defect D6).

App built at `/tmp/velqu-cleanroom/consumer-app/`: POST /items (body schema), GET /items/:id
(params schema + declared 404), GET /items (query schema with min/max), GET /sys/tick (timer),
GET /sys/report (postgres, to demonstrate refusal). All handlers deliberately reference `ctx`
— handlers that ignore ctx and return literals get statically reclassified as "native
liveness" routes, which the browser target rejects (undocumented trap).

### 4. Build

- `velqu check --project .` → `5 routes … — clean` after pinning TS 5.9.3.
- First `build --target browser-wasm` with the Postgres route wired in → **build-time refusal,
  exactly as documented**: `browser-wasm target: route "sys.report" declares capability
  "postgres", which is deployment-required … (src/modules/sys/report.ts)`. Excellent
  diagnostic (source-located, actionable).
- Kernel resolution: the CLI expects the kernel pinned at
  `node_modules/@velqu/browser-runtime/kernel/{kernel.json, q_browser_kernel_bg.wasm,
  q_browser_kernel.nodejs-glue.js}` — but the `@velqu/browser-runtime` tarball ships **no
  kernel directory at all** (defect D3). I copied the cleanroom `kernel/` files there
  manually; then the build succeeded without needing `--kernel`.
- Final build: `--kv --base-path /app/ --probe-path /items` → 4 routes,
  buildId `b9a00aa2…`, full artifact set (kernel.wasm, page.js, worker.js, service-worker.js,
  app.bundle.js, app.qpack, index.html, velqu-artifacts.json).
- `velqu inspect browser` → `integrity: verified (7 artifacts)`, `runtime:kv@1` adapter
  declared, deployment requirements listed. Deterministic and honest.

### 5. Deploy under /app/

- `velqu export --out /tmp/velqu-cleanroom/site/app` → 13 files, verified copy.
- Served `/tmp/velqu-cleanroom/site` with a small Bun.serve static server so the site lives at
  `http://127.0.0.1:8123/app/`. Verified MIME types: `.wasm → application/wasm`, `.js →
  text/javascript`, `.json → application/json`. (Python http.server would also work; I used
  Bun for exact MIME control.) First load initially 404'd due to my own server not mapping
  `/app/` → `index.html` — user error, fixed.

### 6. Browser journeys (Playwright, real Chromium)

| Journey | Result |
|---|---|
| A. First load `/app/` | PASS — `ready — build b9a00aa2816c (network) · worker ready · sw: service-worker`; info panel shows ABI v1/v1, capability descriptors, and the boot probe `GET /items → 200 {"items":[]}` (route executed through WASM kernel + worker) |
| B. POST /items `{"name":"alpha","qty":2}` | **FAIL (defect D4)** — 500 `response schema violation for declared status 201`; the `status(201).value(item)` result was wrapped verbatim into the response body (`__ok`,`status`,`value` flagged as unknown fields). **The KV write itself landed.** |
| B. POST /items `{"name":"x"}` (too short) | PASS — 422 `validation` problem, `path:"name", code:"minLength"` (kernel-side validation, RFC 9457 `application/problem+json`) |
| B. GET /items?limit=0 | PASS — 422 `validation`, `path:"limit", code:"minimum"` |
| B. GET /items/doesnotexist | **FAIL (defect D4)** — 500 internal (`__problem` wrapped as body of the default status); the declared 404 status is unreachable through typed problems in the browser target |
| B. GET /nope | PASS — 404 problem envelope |
| B. DELETE /items | PASS — 405 with `Allow: GET, HEAD, POST` |
| B. GET /sys/tick | **FAIL (defect D5)** — 501 `capability`: `route declares capability "timer" which the artifact inventory does not carry (deployment-required; ADR-0037)` despite `runtime:timers@1` being declared in the same deployment |
| C. Stop server (curl → 000), reload `/app/` | PASS — `ready — build … (cache) · worker ready · sw: service-worker`; boot probe executed **offline** and returned **both persisted items** through the cached kernel + worker |
| D. Restart server, reload custom page | **FAIL (defect D7)** — stuck at `booting…`; console shows 404/504/ERR_FAILED for non-manifest resources once the SW is active |
| D. Restart server, reload default page | PASS — `ready (network)`; probe returned **both persisted items** → KV persistence survived reload + full server restart |
| Raw IndexedDB inspection | PASS — database `velqu:kv`, object store `kv:items-crud`, keys `item:9b2b8c90`, `item:d81ba3a6` (+ `\u0000velqu:meta`) |

Because the generated page exposes no runtime handle ("NO ambient globals"), route execution
beyond the build-time probe required a **hand-written custom page** (custom-page.ts) that
re-implements the generated boot sequence and exposes `runtime.fetch` on `window`. This took
~50 lines plus a `bun build` — feasible only because I read the CLI source (defect D6).

### 7. Evidence files

- `stage1-out.json` — first load + all route-call responses (defect D4/D5 bodies verbatim)
- `stage23-out.json` — offline reload (source: cache, probe with persisted items), raw
  IndexedDB dump, custom-page failure, post-restart persistence
- `transcript.txt` — full shell history; `journey23.py`, `stage1.py` — Playwright scripts;
  `static-server.ts` — deployment server

## What worked well

1. Build-time Postgres refusal: exact match to docs (source-located, remediation given).
2. Artifact integrity + `inspect browser` verification.
3. First-load boot: WASM kernel instantiation, pack verification, worker handshake,
   SW registration — all on first try in Chromium.
4. **Offline reload is real**: full route execution (kernel validation → worker → handler →
   IndexedDB) from the SW cache with the server down.
5. IndexedDB KV: namespaced store appeared exactly as documented (`kv:<appId>-namespace`),
   data survived reload and server restart; raw IDB layout inspectable.
6. Kernel-side validation (422 bodies) and routing (404/405 + Allow) match the RFC 9457 and
   method-matrix claims.
7. Toolchain pin enforcement with an accurate error message.

## Could an external user realistically complete the journey?

- **With only the docs:** no. The docs never show app code, the tarballs do not install, the
  kernel artifacts are missing from the runtime package, and nothing explains that the
  generated page cannot execute routes on demand.
- **With docs + package source reading (what I did):** yes — every step eventually succeeded,
  and the parts that matter at runtime (kernel, SW offline, persistence) all work.
- Time spent: ~2.5 hours end to end, of which roughly 45 minutes were lost to the tarball
  layout, kernel vendoring, and API reverse-engineering that the docs should have covered.
