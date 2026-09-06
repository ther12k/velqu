// packages/browser-runtime/src/dispatcher.ts
var DEFAULT_MAX_BODY_BYTES = 1 << 20;
var MAX_MULTIPART_HEADER_BYTES = 8 << 10;
// packages/browser-runtime/src/worker-host.ts
var MAX_RESULT_BYTES = 1 << 20;
// packages/browser-runtime/src/kv.ts
var MAX_KV_VALUE_BYTES = 1024 * 1024;
// packages/browser-runtime/src/capabilities.ts
var MAX_FETCH_REQUEST_BODY_BYTES = 16 * 1024 * 1024;
var MAX_FETCH_RESPONSE_BODY_BYTES = 16 * 1024 * 1024;
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
    if (scopePath !== "" && !url.pathname.startsWith(scopePath))
      return false;
    return true;
  } catch {
    return false;
  }
}
function classifyRequest(method, pathname, acceptHeader, requestMode) {
  for (const prefix of PASSTHROUGH_PATH_PREFIXES) {
    if (pathname.startsWith(prefix))
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
  const url = new URL(request.url);
  const cls = classifyRequest(request.method, url.pathname, request.headers.get("accept"), request.mode);
  event.respondWith((async () => {
    if (cls === "asset") {
      const cached = await env.cache.match(url.href);
      if (cached)
        return cached;
      return problemResponse({
        status: 504,
        title: "Asset unavailable offline",
        detail: `${url.pathname} is not in the verified build cache`
      });
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
var DEPLOYMENT_SHA256 = "6c305b1e57a754e4b90c00004e4ae7fa065696ef64d2d628b853225b97ea2ad0";
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
