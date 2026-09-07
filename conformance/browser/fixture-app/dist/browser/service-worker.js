// packages/browser-runtime/src/dispatcher.ts
var DEFAULT_MAX_BODY_BYTES = 1 << 20;
var MAX_MULTIPART_HEADER_BYTES = 8 << 10;

// packages/browser-runtime/src/capabilities.ts
var MAX_FETCH_REQUEST_BODY_BYTES = 16 * 1024 * 1024;
var MAX_FETCH_RESPONSE_BODY_BYTES = 16 * 1024 * 1024;

// packages/browser-runtime/src/diagnostics.ts
var DIAGNOSTIC_CODES = {
  LOAD_MANIFEST_OK: "DIAG_LOAD_MANIFEST_OK",
  LOAD_MANIFEST_FAIL: "DIAG_LOAD_MANIFEST_FAIL",
  VERIFY_INTEGRITY_OK: "DIAG_VERIFY_INTEGRITY_OK",
  VERIFY_INTEGRITY_FAIL: "DIAG_VERIFY_INTEGRITY_FAIL",
  COMPAT_KERNEL_ABI_OK: "DIAG_COMPAT_KERNEL_ABI_OK",
  COMPAT_KERNEL_ABI_MISMATCH: "DIAG_COMPAT_KERNEL_ABI_MISMATCH",
  COMPAT_HANDLER_ABI_MISMATCH: "DIAG_COMPAT_HANDLER_ABI_MISMATCH",
  LIFECYCLE_INSTANTIATING: "DIAG_LIFECYCLE_INSTANTIATING",
  LIFECYCLE_READY: "DIAG_LIFECYCLE_READY",
  LIFECYCLE_DISPOSED: "DIAG_LIFECYCLE_DISPOSED",
  ROUTE_MATCHED: "DIAG_ROUTE_MATCHED",
  ROUTE_NOT_FOUND: "DIAG_ROUTE_NOT_FOUND",
  ROUTE_METHOD_NOT_ALLOWED: "DIAG_ROUTE_METHOD_NOT_ALLOWED",
  VALIDATE_PASSED: "DIAG_VALIDATE_PASSED",
  VALIDATE_FAILED: "DIAG_VALIDATE_FAILED",
  CAPABILITY_INVOKED: "DIAG_CAPABILITY_INVOKED",
  CAPABILITY_DENIED: "DIAG_CAPABILITY_DENIED",
  CAPABILITY_DEPLOYMENT_REQUIRED: "DIAG_CAPABILITY_DEPLOYMENT_REQUIRED",
  INVOKE_START: "DIAG_INVOKE_START",
  INVOKE_SUCCESS: "DIAG_INVOKE_SUCCESS",
  INVOKE_TIMEOUT: "DIAG_INVOKE_TIMEOUT",
  INVOKE_CANCEL: "DIAG_INVOKE_CANCEL",
  INVOKE_FAILED: "DIAG_INVOKE_FAILED",
  PERSIST_ACCESS: "DIAG_PERSIST_ACCESS",
  PERSIST_ERROR: "DIAG_PERSIST_ERROR",
  PERSIST_QUOTA_EXCEEDED: "DIAG_PERSIST_QUOTA_EXCEEDED",
  PERSIST_MIGRATION_REQUIRED: "DIAG_PERSIST_MIGRATION_REQUIRED",
  CACHE_HIT: "DIAG_CACHE_HIT",
  CACHE_MISS: "DIAG_CACHE_MISS",
  CACHE_STORED: "DIAG_CACHE_STORED",
  SW_REGISTERED: "DIAG_SW_REGISTERED",
  SW_UPDATE_AVAILABLE: "DIAG_SW_UPDATE_AVAILABLE",
  SW_UPDATE_APPLIED: "DIAG_SW_UPDATE_APPLIED",
  FAIL_INTERNAL: "DIAG_FAIL_INTERNAL",
  FAIL_REDACTED: "DIAG_FAIL_REDACTED"
};
var correlationCounter = 0;
function generateCorrelationId(prefix = "cr") {
  const ts = Date.now().toString(36);
  const count = (++correlationCounter).toString(36);
  const rand = Math.random().toString(36).slice(2, 8);
  return `${prefix}_${ts}_${count}_${rand}`;
}
function extractOrGenerateCorrelationId(headers) {
  if (headers) {
    if (typeof headers.get === "function") {
      const h = headers;
      const found = h.get("x-correlation-id") || h.get("x-request-id");
      if (found)
        return found;
    } else {
      const rec = headers;
      const found = rec["x-correlation-id"] || rec["x-request-id"] || rec["X-Correlation-Id"] || rec["X-Request-Id"];
      if (found)
        return found;
    }
  }
  return generateCorrelationId();
}
// packages/browser-runtime/src/worker-host.ts
var MAX_RESULT_BYTES = 1 << 20;
// packages/browser-runtime/src/kv.ts
var MAX_KV_VALUE_BYTES = 1024 * 1024;
// packages/browser-runtime/src/artifact-loader.ts
async function sha256Hex(bytes) {
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return [...new Uint8Array(digest)].map((b) => b.toString(16).padStart(2, "0")).join("");
}

// packages/browser-runtime/src/service-worker.ts
var PASSTHROUGH_PATH_PREFIXES = [
  "/__velqu_editor__",
  "/auth/",
  "/oauth/",
  "/model-gateway/"
];
function isScopedRequest(scope, requestUrl, locationOrigin) {
  try {
    const url = new URL(requestUrl);
    if (url.origin !== locationOrigin)
      return false;
    const scopePath = scope.replace(/\/+$/, "");
    if (scopePath !== "" && url.pathname !== scopePath && !url.pathname.startsWith(`${scopePath}/`)) {
      return false;
    }
    return true;
  } catch {
    return false;
  }
}
function classifyRequest(method, pathname, acceptHeader, requestMode, scope = "") {
  const scopePath = scope.replace(/\/+$/, "");
  const relativePath = scopePath !== "" && pathname.startsWith(`${scopePath}/`) ? pathname.slice(scopePath.length) : pathname;
  for (const prefix of PASSTHROUGH_PATH_PREFIXES) {
    if (relativePath.startsWith(prefix))
      return "passthrough";
  }
  if (requestMode === "navigate")
    return "navigation";
  if (acceptHeader?.includes("text/html"))
    return "navigation";
  if (pathname.startsWith("/assets/") || /\.(js|mjs|css|wasm|json|map|svg|png|jpg|ico|woff2?)$/.exec(pathname) !== null) {
    return "asset";
  }
  return "api";
}
function buildCachePlan(manifest, baseUrl) {
  return Object.values(manifest.artifacts).map((entry) => ({
    url: resolveAgainst(baseUrl, entry.url),
    sha256: entry.sha256
  }));
}
function resolveAgainst(baseUrl, url) {
  const base = baseUrl.replace(/\/+$/, "");
  return `${base}/${url.replace(/^\/+/, "")}`;
}
function cacheNameFor(appId, buildId) {
  return `velqu:${appId}:${buildId}`;
}
var SW_SHELL_FILES = [
  "index.html",
  "page.js",
  "worker.js",
  "kernel.js",
  "velqu-artifacts.json"
];
async function precacheVerified(url, expectedSha256, env) {
  const response = await env.fetch(url);
  if (!response.ok) {
    throw new Error(`precache failed: ${url} → HTTP ${response.status}`);
  }
  const bytes = new Uint8Array(await response.arrayBuffer());
  if (expectedSha256 !== null) {
    const digest = await sha256Hex(bytes);
    if (digest !== expectedSha256) {
      throw new Error(`precache digest mismatch: ${url} (expected ${expectedSha256.slice(0, 12)}…, got ${digest.slice(0, 12)}…)`);
    }
  }
  await env.cache.put(url, new Response(bytes, {
    status: 200,
    headers: { "content-type": response.headers.get("content-type") ?? "application/octet-stream" }
  }));
}
function cachesToKeep(appId, activeBuildId, existingCacheNames) {
  const prefix = `velqu:${appId}:`;
  const active = cacheNameFor(appId, activeBuildId);
  const others = existingCacheNames.filter((n) => n.startsWith(prefix) && n !== active).sort();
  const keep = [active, ...others.slice(-1)];
  const drop = existingCacheNames.filter((n) => n.startsWith(prefix) && !keep.includes(n));
  return { keep, drop };
}
function problemResponse(problem) {
  return new Response(JSON.stringify({
    problemId: "offline",
    type: "https://velqu.dev/problems/offline",
    title: problem.title ?? "Offline",
    status: problem.status,
    ...problem.detail ? { detail: problem.detail } : {}
  }), {
    status: problem.status,
    headers: [["content-type", "application/problem+json"]]
  });
}
async function handleFetchEvent(event, env) {
  const request = event.request;
  if (!isScopedRequest(env.scope, request.url, env.locationOrigin)) {
    return;
  }
  const correlationId = extractOrGenerateCorrelationId(request.headers);
  const url = new URL(request.url);
  const cls = classifyRequest(request.method, url.pathname, request.headers.get("accept"), request.mode, env.scope);
  event.respondWith((async () => {
    if (cls === "asset") {
      const cached = await env.cache.match(url.href);
      if (cached) {
        env.diagnostics?.record({
          stage: "cache",
          code: DIAGNOSTIC_CODES.CACHE_HIT,
          level: "debug",
          correlationId,
          detail: `cache hit: ${url.pathname}`
        });
        return cached;
      }
      env.diagnostics?.record({
        stage: "cache",
        code: DIAGNOSTIC_CODES.CACHE_MISS,
        level: "debug",
        correlationId,
        detail: `cache miss (network fallback): ${url.pathname}`
      });
      try {
        return await fetch(request);
      } catch {
        return problemResponse({
          status: 504,
          title: "Asset unavailable offline",
          detail: `${url.pathname} is not in the verified build cache and the network is unreachable`
        });
      }
    }
    try {
      return await env.runtimeFetch(request);
    } catch (cause) {
      if (cls === "navigation") {
        const offline = await env.cache.match(resolveAgainst(env.baseUrl, "contract.json"));
        if (offline) {
          return problemResponse({
            status: 503,
            title: "Offline",
            detail: "runtime could not start; verified artifacts are cached — retry"
          });
        }
      }
      throw cause;
    }
  })());
}
// conformance/browser/fixture-app/dist/browser/service-worker.js
var DEPLOYMENT_SHA256 = "e84274f68e2b14f24889de706f266302a2fc4ed0999c5f8b36847ecf77872ee9";
var APP_ID = "app";
var SCOPE = "/";
var BASE_URL = self.registration.scope;
var MANIFEST_URL = new URL("velqu-artifacts.json", BASE_URL);
var manifest = null;
var cache = null;
async function ensureReady() {
  if (manifest && cache)
    return;
  for (const name of await caches.keys()) {
    if (!name.startsWith("velqu:" + APP_ID + ":"))
      continue;
    const candidate = await caches.open(name);
    const cached = await candidate.match(MANIFEST_URL.href);
    if (!cached)
      continue;
    const bytes = new Uint8Array(await cached.clone().arrayBuffer());
    if (await sha256Hex(bytes) === DEPLOYMENT_SHA256) {
      manifest = JSON.parse(new TextDecoder().decode(bytes));
      cache = candidate;
      return;
    }
  }
  const response = await fetch(MANIFEST_URL);
  if (!response.ok)
    throw new Error("velqu sw: manifest HTTP " + response.status);
  manifest = await response.json();
  cache = await caches.open(cacheNameFor(manifest.appId, manifest.buildId));
}
async function prefetch(url, sha256) {
  await precacheVerified(url, sha256, { fetch: (u) => fetch(u), cache });
}
self.addEventListener("install", (event) => {
  event.waitUntil((async () => {
    await ensureReady();
    for (const entry of buildCachePlan(manifest, BASE_URL)) {
      await prefetch(entry.url, entry.sha256);
    }
    for (const shell of SW_SHELL_FILES) {
      await prefetch(new URL(shell, BASE_URL).href, null);
    }
    await cache.put(BASE_URL, (await fetch(new URL("index.html", BASE_URL).href)).clone());
  })());
});
self.addEventListener("message", (event) => {
  if (event.data && event.data.type === "VELQU_APPLY_UPDATE") {
    self.skipWaiting();
  }
});
self.addEventListener("activate", (event) => {
  event.waitUntil((async () => {
    await ensureReady();
    const { drop } = cachesToKeep(APP_ID, manifest.buildId, await caches.keys());
    for (const name of drop)
      await caches.delete(name);
  })());
});
var lazyCache = {
  match: async (url) => {
    await ensureReady();
    return cache.match(url);
  },
  put: async (url, response) => {
    await ensureReady();
    return cache.put(url, response);
  }
};
self.addEventListener("fetch", (event) => {
  handleFetchEvent(event, {
    scope: SCOPE,
    locationOrigin: self.location.origin,
    manifest,
    runtimeFetch: async (request) => {
      await ensureReady();
      if (request.mode === "navigate") {
        try {
          return await fetch(new Request(request.url, { headers: request.headers }));
        } catch {
          const shell = await cache.match(new URL("index.html", BASE_URL).href) ?? await cache.match(BASE_URL);
          if (shell)
            return shell;
          throw new Error("offline and no cached shell for " + request.url);
        }
      }
      return fetch(request);
    },
    cache: lazyCache,
    baseUrl: BASE_URL
  });
});
