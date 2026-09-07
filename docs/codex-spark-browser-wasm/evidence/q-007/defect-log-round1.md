# Defect Log — Velqu Browser-WASM Cleanroom Evaluation

Categories: `blocked` (cannot proceed at all), `framework-defect` (shipped code misbehaves),
`setup-friction` (possible but needlessly hard), `docs-gap` (undocumented / misleading).

## D1 — blocked → worked around: registry tarballs are not npm-installable
- **Category:** blocked (as shipped) / packaging defect
- **Evidence:** `bun add /tmp/velqu-cleanroom/registry/velqu-*.tgz` →
  `"package.json" failed to open: ENOENT`, `Invalid dependency name`, `InstallFailed` for
  every archive; `tar -tzf velqu-contract.tgz` shows entries at the archive root
  (`package.json`, `src/`) instead of the npm-standard `package/` prefix.
- **Also:** every package.json is `"private": true` with `"workspace:*"` inter-deps, which
  cannot resolve outside the monorepo even with a correct layout.
- **Workaround used:** manual `tar -xzf … -C node_modules/@velqu/<name>` for all 6 packages.
- **Impact:** any npm/Bun user fails at step 1. INSTALL.md itself admits "prepared but not
  yet published", but then the simulated-registry artifacts should still be installable.

## D2 — docs-gap: no authoring documentation for the browser target
- **Category:** docs-gap
- **Evidence:** BROWSER_WASM.md §2 says "a Velqu project with routes declared via
  `@velqu/core` and `@velqu/schema`" but no doc shows `route({...})`, `defineApp`,
  project entry conventions (`src/app.ts`), schema builders, or `ctx.native.*` usage. I
  reconstructed the whole app from `@velqu/core/src/index.ts` and the CLI's scaffold.ts.
- **Impact:** ~40 minutes lost; an external user without repo-source access cannot start.

## D3 — framework-defect (packaging): `@velqu/browser-runtime` tarball ships no kernel
- **Category:** framework-defect (packaging)
- **Evidence:** CLI `browser-deploy.ts` resolves the pinned kernel from
  `node_modules/@velqu/browser-runtime/kernel/` (kernel.json + wasm + glue, hash-pinned) and
  fails closed otherwise ("kernel wasm not found … restore packages/browser-runtime/kernel/");
  the published tarball contains only `package.json` + `src/`. Even the documented
  `--kernel <file>` override cannot avoid the missing vendored `kernel.json`/glue.
- **Workaround used:** copied cleanroom `kernel/` files into
  `node_modules/@velqu/browser-runtime/kernel/`.
- **Impact:** browser-wasm build is impossible from published artifacts alone.

## D4 — framework-defect: `status(...).value()` / `status(...).problem()` broken in browser target
- **Category:** framework-defect
- **Evidence:** generated `app.browser.js` invoke wrapper checks `"kind" in result`, but
  `@velqu/core` result values are `{__ok, status, value}` / `{__problem, problem, status, …}` —
  neither has `kind`, so they are wrapped verbatim as the body of the route's *default*
  status:
  - `POST /items` returning `status(201).value(item)` → **500** `response schema violation for
    declared status 201` with errors on `__ok`/`status`/`value` (stage1-out.json:
    `B_create_valid`).
  - `GET /items/:id` returning `status(404).problem("not-found")` → **500** internal (body
    re-validated against the 200 schema; stage1-out.json: `B_get_missing`).
- **Impact:** declared non-default statuses and typed problems — the documented error model
  ("Expected HTTP failures are typed values with declared statuses") — are unreachable in
  browser deployments. Only plain-object returns of the default status work. (KV write still
  happened, so the defect also produces side effects with a 500 response.)

## D5 — framework-defect: timer capability 501 although declared
- **Category:** framework-defect
- **Evidence:** `GET /sys/tick` (handler uses documented `ctx.native.timer.delay(30)`) → **501**
  `capability`: `route declares capability "timer" which the artifact inventory does not carry
  (deployment-required; ADR-0037)`. The same deployment's `inspect browser` lists
  `runtime:timers@1` among the capability adapters, and the portability registry classifies
  `timer` as `browser-and-native`. Looks like a grant-name (`timer`) vs adapter-id
  (`runtime:timers`) inventory mismatch, plus a misleading "deployment-required" label.
- **Impact:** the one cross-target capability example a browser app can declare does not run.

## D6 — setup-friction / docs-gap: generated page cannot execute routes by design
- **Category:** setup-friction + docs-gap
- **Evidence:** generated page.js comment: "this page exposes NO ambient globals"; the only
  route execution is the build-time probe (`--probe-path/--probe-method`, undocumented in
  BROWSER_WASM.md; probe supports GET-style requests without bodies only). To exercise
  POST/validation flows in a real browser I had to hand-write a custom page mirroring the
  boot sequence (~50 lines: loadArtifactsWithFallback → initKernelSync → WorkerHost →
  createBrowserRuntime → bootstrapServiceWorker) and bundle it myself.
- **Impact:** developers cannot demo, test, or smoke-check their API in-browser without
  reimplementing the runtime bootstrap from package source.

## D7 — framework-defect: service worker fails non-manifest assets after an offline cycle
- **Category:** framework-defect
- **Evidence:** after a successful offline reload (server stopped), restarting the server and
  reloading `/app/my-page.html` (a valid, network-served file, curl 200) hangs at `booting…`;
  console shows `404`, `net::ERR_FAILED`, `504` for page resources. The same page worked
  before the offline cycle. The SW appears to intercept `/app/*` subresources that are not in
  the verified artifact manifest and fail closed (synthesized 504) instead of passing through
  to the network.
- **Impact:** any page/asset not listed in the build manifest (custom UI pages, images, CSS)
  breaks once the SW controls the scope — making D6's custom page unusable after the first
  offline visit. The default shell keeps working.

## D8 — docs-gap: `--kv` flag does not connect KV to handlers
- **Category:** docs-gap (+ dead code in generated page)
- **Evidence:** BROWSER_WASM.md §4 documents KV adapters; the `--kv` build flag emits
  `const kv = createIndexedDbKv({ namespace: "<appId>:kv" })` into page.js and never uses it
  (unused variable; handlers run in a separate worker with no KV binding). Handler KV access
  only works because I constructed the adapter myself inside a handler module
  (namespace `items-crud`, not the flag's `consumer-app:kv`).
- **Impact:** the documented flag produces no user-visible effect; namespace documented
  (`kv:${appId}:${namespace}`) does not match either actual store name (`kv:items-crud`).

## D9 — docs-gap: minor traps
- Handlers that don't reference `ctx` and return a literal are statically reclassified as
  native-liveness routes and **rejected by the browser build** (undocumented; discovered in
  compiler source).
- Toolchain pin (bun 1.4.0 / typescript 5.9.3) is enforced but only discoverable by running
  `check`; `bun init` defaults to typescript ^7 → immediate mismatch (error message itself
  was clear and accurate).
- BROWSER_WASM.md's CLI examples use monorepo paths (`bun packages/cli/src/index.ts`) that
  don't exist outside the repo; the tarball provides `bin: velqu` but the private/workspace
  packaging makes it unusable.

## Not defects (verified working)
- Build-time Postgres refusal (exact docs match), `inspect browser` integrity verification.
- First-load boot chain (kernel WASM → pack verify → worker → SW) on Chromium/Linux.
- Offline reload with full route execution from cache (`source: cache`).
- IndexedDB KV namespacing + persistence across reload and server restart.
- Kernel validation 422s (minLength/minimum), 404/405 + `Allow` header, RFC 9457 envelopes.
