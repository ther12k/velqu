# Re-verification report — Velqu Browser-WASM fix round (D4, D5, D7)

Environment: fresh app `/tmp/velqu-cleanroom/reverify-app/`, registry tarballs extracted
into `node_modules/@velqu/{core,schema,browser-runtime,compiler,contract,cli}` (browser-runtime
tarball DOES ship `kernel/` — confirmed: `kernel/q_browser_kernel_bg.wasm` present).
TypeScript pinned to /tmp/ts-pin 5.9.3 via symlink. Deployment built with:

```
bun node_modules/@velqu/cli/src/index.ts build --target browser-wasm --project . --kv --base-path /app/
# buildId 211b593c117eb3e8873355611e3284daeb15ecccb926bf6ef2651537749cbe8d, 3 routes
```

Static site: `/tmp/velqu-cleanroom/reverify-site/app/` (dist/browser + hand-added
`custom.html` + bundled `custom.js`, custom page NOT part of the build manifest).
Served under `/app/` at http://127.0.0.1:8223/ by `/tmp/velqu-cleanroom/reverify-static-server.ts`.
Driven by Playwright + Chromium (`chromium-1234`, `--no-sandbox`), all calls in-page via
`window.velqu.fetch(new Request("http://velqu.local" + path))`. Full transcript: `transcript.txt`.

---

## D4 — non-default statuses: PASS

Route `orders.create`: `response: { 201: s.object({ id: s.string(), item: s.string() }) }`,
handler `return status(201).value({ id: crypto.randomUUID().slice(0,8), item: body.item })`.

```
POST /orders  {"item":"widget"}
→ status 201
→ body {"id":"215b2973","item":"widget"}
```

Route `orders.get`: `response: { 200: ..., 404: s.object({ detail: s.string() }) }`,
handler `return status(404).problem("not-found", { detail: "no order nope" })`.

```
GET /orders/nope
→ status 404
→ body {"problemId":"not-found","type":"https://velqu.dev/problems/not-found",
        "title":"Not Found","status":404,"detail":"no order nope"}
```

Both declared statuses respond with their declared status code and schema-valid bodies.
No 500 "response schema violation". **PASS.**

## D5 — timer capability: FAIL

Route `sys.tick`: `response: { 200: s.object({ ms: s.number() }) }`,
handler `async ({ native }) => { const ms = await native.timer.delay(5); return { ms }; }`.
Deployment rebuilt with `--kv --base-path /app/`; the artifact declares the capability —
`capability-manifest.json`: `"declared": ["timer"]`, `perRoute["sys.tick"] = ["timer"]`,
`portability.timer.state = "browser-and-native"`. The original 501
("artifact inventory does not carry timer") is gone, but the request still fails:

```
GET /sys/tick
→ status 500
→ body {"problemId":"internal","type":"https://velqu.dev/problems/internal","title":"Internal",
        "status":500,
        "detail":"TypeError: Cannot read properties of undefined (reading 'timer')\n
                  at Object.handle (worker.js:1924:29)\n
                  at invoke (worker.js:2008:33)\n
                  at Object.invoke (worker.js:688:14)\n
                  at worker.js:2059:44"}
```

Root cause (from the emitted `dist/browser/worker.js`, which matches
`@velqu/browser-runtime/src/worker-host.ts` `workerBootstrapSource()` / `toContext(plan)`,
line ~429): the worker builds the handler context as
`{ routeId, handlerKey, params, query, headers, body, bodyText, deadlineMs }` — it never
attaches `native`. The timer implementation exists host-side (`createTimerCapability` in
`src/capabilities.ts`, wired into the capability graph dispatch) but nothing passes it into
the Worker handler context, so destructured `native` is `undefined` and the handler throws.
Same 500 reproduced after the D7 stop/restart cycle. Result: 200 `{ms:<number>}` expected,
500 TypeError observed. **FAIL (partially fixed: artifact declaration fixed, context injection missing).**

## D7 — custom asset after offline: PASS

`/app/custom.html` (hand-served, not in build manifest) booted the full runtime via
`custom.js` (artifact load → kernel init → Worker host → runtime → SW registration,
scope `/app/`).

1. Online load: HTTP **200**, boot status
   `ready — custom page · build 211b593c117e (network) · sw: service-worker`.
2. Server killed (probe: connection refused) → reload `/app/custom.html`: HTTP **200**,
   booted from SW cache: `ready — build 211b593c117e (cache) · worker ready · sw: service-worker`.
3. Server restarted → reload `/app/custom.html`: HTTP **200** (no hang, no 504),
   `ready — custom page · build 211b593c117e (network) · sw: service-worker`.
   Runtime still functional after the cycle (`POST /orders` → 201).

**PASS.**

---

## Verdict

| Defect | Verdict | Evidence |
|---|---|---|
| D4 non-default statuses | **PASS** | 201 + value body; 404 + RFC-9457 problem body, both via runtime.fetch |
| D5 timer capability | **FAIL** | artifact declares `runtime:timers` (501 gone) but worker `toContext()` omits `native` → 500 `TypeError: Cannot read properties of undefined (reading 'timer')` |
| D7 custom asset after offline | **PASS** | custom.html: 200 online → 200 from SW cache offline → 200 after server restart, boot ready each time |

---

# Round 3 (final kernel)

Re-extracted refreshed registry tarballs into `reverify-app/node_modules/@velqu/*`
(browser-runtime ships `kernel/`; new kernel.wasm hash `b0485be76284…`→`b0485be753c9…`,
1,732,433B). Rebuilt: `bun node_modules/@velqu/cli/src/index.ts build --target browser-wasm
--project . --kv --base-path /app/ --probe-path /health` (buildId `b81551d2781a…`, 3 routes).
Confirmed the new plumbing exists end-to-end: kernel plan carries route grants
(`plan.capabilities`), worker bootstrap (`worker-host.ts:416-459`) holds
`nativeCapabilities` (registered via `self.velquRegisterNativeCapabilities`, called by the
generated `page.js` as `worker.velquRegisterNativeCapabilities?.(capabilities.graph)` in the
Worker factory) and `toContext` now exposes `native: planCapabilities(plan.capabilities ?? [])`.

My custom page was updated to mirror the new generated page (added
`w.velquRegisterNativeCapabilities?.(capabilities.graph);` to the Worker factory).

## D5 — timer capability: FAIL (progress: error moved from `reading 'timer'` to `reading 'delay'`)

```
GET /sys/tick   (handler: async ({ native }) => { const ms = await native.timer.delay(5); return { ms }; })
→ status 500
→ body {"problemId":"internal","type":"https://velqu.dev/problems/internal","title":"Internal",
        "status":500,
        "detail":"TypeError: Cannot read properties of undefined (reading 'delay')\n
                  at Object.handle (worker.js:1924:35)\n
                  at invoke (worker.js:2008:33)\n
                  at Object.invoke (worker.js:688:14)\n
                  at worker.js:2063:44"}
```

Analysis: `ctx.native` is now an injected object (the 500 no longer says "reading 'timer'"),
but `ctx.native.timer` is undefined. `planCapabilities` (`worker-host.ts:448-456`) only maps
the grant when `grant === "timer"` (then `view.timer = { delay }` from
`nativeCapabilities.timers`); the page-side graph is keyed `timers`
(`capabilities.ts:666-683`: `graph: { timers: timer.timers, … }`, and the qpack carries both
`runtime:timers` and `timer`). For `native` to be defined while `native.timer` is missing, the
plan's grant name must not match `"timer"` (consistently: it lands in the generic branch,
e.g. as `"timers"`/`"runtime:timers"`, producing `view.timers` instead of `view.timer`).
Net effect unchanged for the developer: 500 instead of 200 `{ms:<number>}`. Same 500
reproduced after the offline/restart cycle.

## Regression sweep (D4 / D7 flows, same run)

- **D4a** `POST /orders` → **201** `{"id":"8dd4be85","item":"widget"}` — still PASS.
- **D4b** `GET /orders/nope` → **404** RFC-9457 `{"problemId":"not-found",…,"detail":"no order nope"}` — still PASS.
- **D7 REGRESSION on the offline leg**: after stopping the static server, reloading
  `/app/custom.html` fails at navigation: `Page.goto: net::ERR_CONNECTION_REFUSED`
  (previously the service worker served the navigation from cache: HTTP 200,
  boot `… (cache) · worker ready`). The SW registers (`sw: service-worker`) and the final
  leg still works: after restarting the server, reload → **HTTP 200**, boot
  `ready — custom page · build b81551d2781a (network) · sw: service-worker`, and
  `POST /orders` → 201 post-cycle. So: online load OK, post-restart load OK, but the
  "reload while offline boots from SW cache" behavior observed in Round 2 is broken in
  this build — the SW did not fall back to cache for the navigation.

## Round 3 verdict

| Check | Verdict | Evidence |
|---|---|---|
| D5 timer | **FAIL** | 500 `TypeError: Cannot read properties of undefined (reading 'delay')` — `ctx.native` injected but `native.timer` missing (grant-name mismatch vs the `"timer"` special case in `planCapabilities`) |
| D4 regression | **PASS** | 201 + 404 problem unchanged |
| D7 regression | **PARTIAL FAIL (new)** | offline reload of custom.html → `ERR_CONNECTION_REFUSED` (SW no longer serves the navigation from cache); restart+reload leg still 200 |

## Round 4 (final)

Fresh app at `/tmp/velqu-cleanroom/r4-app/` from the refreshed registry tarballs
(same kernel.wasm sha as Round 3's refresh: `b0485be753c9…`, 1,732,433B).
3 routes (`sys.tick`, `orders.create`, `orders.get`). Build:
`bun node_modules/@velqu/cli/src/index.ts build --target browser-wasm --project . --kv
--base-path /app/ --probe-path /sys/tick --probe-method GET` — buildId
`0a34654043e1…` (after route-export fix; first attempt `d154d2658b9e…`, see note below).

Fix confirmed present in artifacts BEFORE running: generated `worker.js` now imports
`createBrowserCapabilityGraph` + `ringBufferSink` from `@velqu/browser-runtime`,
constructs the graph INSIDE the Worker (`worker.js:2118-2122`,
`self.velquRegisterNativeCapabilities?.(capabilityGraph.graph)`), build-time fetch
allowlist `[]` (default deny); generated `page.js` Worker factory only postMessages
`{type:"velqu-session", sessionId}` (no capability injection). `capability-manifest.json`:
`declared: ["timer"]`, `perRoute: {"sys.tick": ["timer"]}` — the plan grant name matches
the `"timer"` special case in `planCapabilities`.

Custom page `my-page.ts` mirrors the new `page.js` boot exactly (NO
`velquRegisterNativeCapabilities` from the page side), bundled with
`bun build --target browser --format esm`, served at `/app/` (python http.server 8907,
site root symlink `app -> r4-app/dist/browser`). Boot:
`ready — r4 custom page · build 0a34654043e1 (network) · sw: service-worker`.

### D5 — timer capability: PASS

- `GET /sys/tick` (handler `await native.timer.delay(5)` → `{ ms: r.elapsedMs ?? 5 }`)
  → **200** `{"ms":5}` (online leg) and **200** `{"ms":6}` (pre-offline leg).
  No 500 TypeError, no 501. The Round-3 failure (`Cannot read properties of undefined
  (reading 'delay')`) is gone: the in-Worker graph + `"timer"` grant in the plan line up.

### D4 regression — non-default statuses: PASS

- `POST /orders` `{"item":"widget"}` → **201** `{"id":"78fe7140","item":"widget"}`
- `GET /orders/nope` → **404** RFC 9457 problem:
  `{"problemId":"not-found","type":"https://velqu.dev/problems/not-found","title":"Not Found","status":404,"detail":"no order nope"}`

### Offline leg (bonus, data not verdict)

- Browser offline (`context.set_offline`) with server actually stopped: reload of
  `/app/my-page.html` fails at navigation — `net::ERR_INTERNET_DISCONNECTED`
  (Round 3 saw the equivalent `ERR_CONNECTION_REFUSED`). The service worker still does
  NOT serve the navigation from cache; no 200-from-cache observed.
- Restart leg: server back up, same tab reload → **200** boot
  `ready — build 0a34654043e1 (network) · sw: service-worker`, and
  `POST /orders {"item":"post-cycle"}` → **201** `{"id":"d10b98ea","item":"post-cycle"}`.
  Online/restart legs of D7 remain healthy.

### Evaluator finding (new, latent — NOT the D5 fix, which works)

First build silently produced a broken Worker: the emitted
`app.browser.js` references handlers via namespace imports
(`import * as m0 from "./src/app.ts"` → `(m0.sysTick).handle(ctx)`). With route consts
declared but **not exported** from their source file (my first `app.ts` had local
consts inside `defineModule`'s array), the build SUCCEEDS but the bundler folds
`m0.sysTick` → `undefined`, so every route 500s at runtime:
`TypeError: Cannot read properties of undefined (reading 'handle') at invoke (worker.js)`.
Exporting the route consts (`export const sysTick = route({...})`) fixes it. Worth a
build-time guard (extractor should require the route binding to be exported for the
browser target, else fail the build).

### Round 4 verdict

| Check | Verdict | Evidence |
|---|---|---|
| D5 timer | **PASS** | `GET /sys/tick` → 200 `{"ms":5}` (and `{"ms":6}`) — was 500 in R3 |
| D4 regression | **PASS** | POST /orders → 201 with id; GET /orders/nope → 404 problem body |
| Offline leg (bonus) | data | offline reload → `ERR_INTERNET_DISCONNECTED` (no SW-cache navigation); restart reload → 200 boot, POST /orders → 201 |

Commands appended to `transcript.txt`. Scripts: `r4-app/r4-verify.py`, `r4-app/r4-offline.py`.
