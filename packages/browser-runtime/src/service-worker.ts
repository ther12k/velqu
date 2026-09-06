/**
 * BWASM-B-004 — Service Worker adapter and static-host bootstrap.
 *
 * Exposes a built Velqu browser application on static hosting WITHOUT a
 * Velqu application server:
 *
 * - **Scope guard (pure)**: only same-origin requests under the
 *   explicit scope are ever intercepted — scope escapes, cross-origin
 *   traffic, editor/auth/model-gateway endpoints are PASSTHROUGH by
 *   construction (ADR-0038 §4).
 * - **Request classification (pure)**: navigation / api / asset /
 *   passthrough, with frozen behaviors per class.
 * - **Cache plan (pure)**: precache list derived from the B-002
 *   content-addressed manifest (hash-addressed, integrity-verified at
 *   load — a cache hit never skips verification).
 * - **Fetch-event core**: takes a FetchEvent-like object and a
 *   double-injectable environment (runtime fetch + cache + manifest),
 *   so every behavior here is deterministically testable; real-browser
 *   lanes (Chromium/Firefox/WebKit) are BWASM-Q-002.
 * - **Bootstrap + fallback**: page-side registration with structured
 *   outcomes; when Service Workers are unavailable or registration
 *   fails, `injected-fetch` fallback mode is reported (never a hanging
 *   preview).
 *
 * Offline/updates (ADR-0039 §4): first load online; artifacts cached
 * content-addressed; offline reload serves from cache; update = new
 * buildId detected → activate on next reload (never mid-request);
 * rollback = redeploy the prior build.
 */

import { loadArtifacts, sha256Hex, type BrowserArtifactManifest } from "./artifact-loader";

// ---------------------------------------------------------------------------
// Scope guard + classification (pure)
// ---------------------------------------------------------------------------

/** Editor/auth/gateway paths NEVER intercepted (ADR-0038 §4). */
export const PASSTHROUGH_PATH_PREFIXES: ReadonlyArray<string> = [
  "/__velqu_editor__",
  "/auth/",
  "/oauth/",
  "/model-gateway/",
];

export type RequestClass = "navigation" | "api" | "asset" | "passthrough";

/**
 * Decide whether a request is inside the SW scope at all. Anything
 * false here is PASSTHROUGH — the SW must not respondWith for it.
 */
export function isScopedRequest(scope: string, requestUrl: string, locationOrigin: string): boolean {
  try {
    const url = new URL(requestUrl);
    if (url.origin !== locationOrigin) return false; // unrelated origin
    const scopePath = scope.replace(/\/+$/, "");
    if (scopePath !== "" && !url.pathname.startsWith(scopePath)) return false; // scope escape
    return true;
  } catch {
    return false; // unparseable URL: never intercept
  }
}

/** Classify a scoped request (frozen behaviors, support matrix §R-002). */
export function classifyRequest(
  method: string,
  pathname: string,
  acceptHeader: string | null,
  requestMode: string,
): RequestClass {
  for (const prefix of PASSTHROUGH_PATH_PREFIXES) {
    if (pathname.startsWith(prefix)) return "passthrough";
  }
  if (requestMode === "navigate") return "navigation";
  if (acceptHeader?.includes("text/html")) return "navigation";
  if (
    pathname.startsWith("/assets/") ||
    /\.(js|mjs|css|wasm|json|map|svg|png|jpg|ico|woff2?)$/.exec(pathname) !== null
  ) {
    return "asset";
  }
  return "api";
}

// ---------------------------------------------------------------------------
// Cache plan (content-addressed, from the B-002 manifest)
// ---------------------------------------------------------------------------

export interface CachePlanEntry {
  readonly url: string;
  readonly sha256: string;
}

/** Precache list: every manifest artifact by its content hash key. */
export function buildCachePlan(
  manifest: BrowserArtifactManifest,
  baseUrl: string,
): ReadonlyArray<CachePlanEntry> {
  return Object.values(manifest.artifacts).map((entry) => ({
    url: resolveAgainst(baseUrl, entry.url),
    sha256: entry.sha256,
  }));
}

function resolveAgainst(baseUrl: string, url: string): string {
  const base = baseUrl.replace(/\/+$/, "");
  return `${base}/${url.replace(/^\/+/, "")}`;
}

/** Cache Storage name: per appId+buildId (content-addressed). */
export function cacheNameFor(appId: string, buildId: string): string {
  return `velqu:${appId}:${buildId}`;
}

// ---------------------------------------------------------------------------
// B-006: verified prefetch, shell caching, retention, last-known-good
// ---------------------------------------------------------------------------

/**
 * App-shell files the generated Service Worker must cache for offline
 * navigation (BWASM-B-006). These sit outside the B-002 manifest's frozen
 * role enum, so they carry no per-file digest contract — the runtime's
 * integrity boundary stays on the manifest roles; shell files are cached
 * for availability only.
 */
export const SW_SHELL_FILES: ReadonlyArray<string> = [
  "index.html",
  "page.js",
  "worker.js",
  "kernel.js",
  "velqu-artifacts.json",
];

/**
 * Fetch one URL and cache it, verifying its sha256 when the manifest
 * declares one. A failed fetch, non-OK response, or digest mismatch
 * THROWS — a B-006 SW install that throws never activates, so a partial
 * or corrupt update can never displace the last known-good build.
 */
export async function precacheVerified(
  url: string,
  expectedSha256: string | null,
  env: {
    fetch: (url: string) => Promise<Response>;
    cache: { put(url: string, response: Response): Promise<void> };
  },
): Promise<void> {
  const response = await env.fetch(url);
  if (!response.ok) {
    throw new Error(`precache failed: ${url} → HTTP ${response.status}`);
  }
  const bytes = new Uint8Array(await response.arrayBuffer());
  if (expectedSha256 !== null) {
    const digest = await sha256Hex(bytes);
    if (digest !== expectedSha256) {
      throw new Error(
        `precache digest mismatch: ${url} (expected ${expectedSha256.slice(0, 12)}…, got ${digest.slice(0, 12)}…)`,
      );
    }
  }
  await env.cache.put(
    url,
    new Response(bytes, {
      status: 200,
      headers: { "content-type": response.headers.get("content-type") ?? "application/octet-stream" },
    }),
  );
}

/**
 * Cache retention policy (B-006): keep the active build's cache plus at
 * most ONE previous build cache (deterministic choice — buildIds are
 * content hashes with no ordering, so the retained previous build is the
 * lexicographically greatest other cache). Active clients hold their
 * artifacts in memory (per-build cache isolation already prevents mixed
 * state); retention bounds growth and keeps one-step rollback servable
 * from cache.
 */
export function cachesToKeep(
  appId: string,
  activeBuildId: string,
  existingCacheNames: ReadonlyArray<string>,
): { keep: ReadonlyArray<string>; drop: ReadonlyArray<string> } {
  const prefix = `velqu:${appId}:`;
  const active = cacheNameFor(appId, activeBuildId);
  const others = existingCacheNames
    .filter((n) => n.startsWith(prefix) && n !== active)
    .sort();
  const keep = [active, ...others.slice(-1)];
  const drop = existingCacheNames.filter((n) => n.startsWith(prefix) && !keep.includes(n));
  return { keep, drop };
}

// ---------------------------------------------------------------------------
// Fetch-event core (double-injectable; deterministic)
// ---------------------------------------------------------------------------

/** Minimal FetchEvent-like surface the core needs. */
export interface FetchEventLike {
  readonly request: Request;
  respondWith(response: Promise<Response> | Response): void;
  waitUntil(promise: Promise<unknown>): void;
}

export interface WorkerEnv {
  readonly scope: string;
  readonly locationOrigin: string;
  readonly manifest: BrowserArtifactManifest;
  readonly runtimeFetch: (request: Request) => Promise<Response>;
  /** Cache Storage facade (injectable; Cache API in browsers). */
  readonly cache: {
    match(url: string): Promise<Response | undefined>;
    put(url: string, response: Response): Promise<void>;
  };
  readonly baseUrl: string;
}

function problemResponse(problem: {
  status: number;
  type?: string;
  title?: string;
  detail?: string;
}): Response {
  return new Response(
    JSON.stringify({
      problemId: "offline",
      type: "https://velqu.dev/problems/offline",
      title: problem.title ?? "Offline",
      status: problem.status,
      ...(problem.detail ? { detail: problem.detail } : {}),
    }),
    {
      status: problem.status,
      headers: [["content-type", "application/problem+json"]],
    },
  );
}

/**
 * The SW fetch handler core: scope guard → classification → behavior.
 * Never intercepts out-of-scope traffic; never serves an unverified
 * artifact (the runtime was built from loader-verified bytes).
 */
export async function handleFetchEvent(event: FetchEventLike, env: WorkerEnv): Promise<void> {
  const request = event.request;
  if (!isScopedRequest(env.scope, request.url, env.locationOrigin)) {
    return; // passthrough: SW does not respondWith
  }
  const url = new URL(request.url);
  const cls = classifyRequest(
    request.method,
    url.pathname,
    request.headers.get("accept"),
    request.mode,
  );
  event.respondWith(
    (async () => {
      if (cls === "asset") {
        // Assets: cache-first (content-addressed; cache hit implies the
        // verified hash was checked at install).
        const cached = await env.cache.match(url.href);
        if (cached) return cached;
        return problemResponse({
          status: 504,
          title: "Asset unavailable offline",
          detail: `${url.pathname} is not in the verified build cache`,
        });
      }
      // navigation + api: the runtime owns routing/validation — serve
      // through the kernel-backed fetch. Offline kernel artifacts are
      // cached, so the runtime itself works offline.
      try {
        return await env.runtimeFetch(request);
      } catch (cause) {
        if (cls === "navigation") {
          const offline = await env.cache.match(resolveAgainst(env.baseUrl, "contract.json"));
          if (offline) {
            // Deterministic offline marker for navigations.
            return problemResponse({
              status: 503,
              title: "Offline",
              detail: "runtime could not start; verified artifacts are cached — retry",
            });
          }
        }
        throw cause;
      }
    })(),
  );
}

// ---------------------------------------------------------------------------
// Bootstrap (page side) + fallback
// ---------------------------------------------------------------------------

export type BootstrapOutcome =
  | { readonly kind: "service-worker"; readonly scope: string; readonly registration: unknown }
  | { readonly kind: "injected-fetch-fallback"; readonly reason: string };

/**
 * Page-side bootstrap: register the generated SW script under an
 * explicit scope and wait for activation. Structured outcome in every
 * path — an unsupported/failed registration is an actionable fallback,
 * never a hanging preview.
 */
export async function bootstrapServiceWorker(options: {
  scriptUrl: string;
  scope: string;
}): Promise<BootstrapOutcome> {
  const nav = globalThis as { navigator?: { serviceWorker?: { register(url: string, opts: { scope: string }): Promise<unknown> } } };
  if (!nav.navigator?.serviceWorker) {
    return {
      kind: "injected-fetch-fallback",
      reason:
        "ServiceWorker is unavailable in this environment — use the injected-fetch fallback (runtime.fetch directly in the page); see UNSUPPORTED_SEMANTICS",
    };
  }
  try {
    const registration = await nav.navigator.serviceWorker.register(options.scriptUrl, {
      scope: options.scope,
    });
    return { kind: "service-worker", scope: options.scope, registration };
  } catch (cause) {
    return {
      kind: "injected-fetch-fallback",
      reason: `ServiceWorker registration failed: ${String(cause)}`,
    };
  }
}

/**
 * Update semantics (ADR-0039 §4): compare the deployed manifest's
 * buildId against the active one; a new buildId is applied on next
 * reload (never mid-request). Pure decision function.
 */
export function updateDecision(
  activeBuildId: string,
  deployedBuildId: string,
): "apply-on-next-reload" | "no-change" {
  return deployedBuildId !== activeBuildId ? "apply-on-next-reload" : "no-change";
}

// ---------------------------------------------------------------------------
// B-006: page-boot loader with last-known-good fallback
// ---------------------------------------------------------------------------

/** What {@link loadArtifactsWithFallback} booted from. */
export interface FallbackLoadedArtifacts {
  readonly source: "network" | "cache";
  readonly buildId: string;
  readonly manifest: BrowserArtifactManifest;
  readonly bytes: Readonly<Record<import("./artifact-loader").ArtifactRole, Uint8Array>>;
}

/**
 * Page-boot artifact loading with the B-006 availability policy:
 *
 * 1. Try the deployed manifest over the network and verify it with the
 *    B-002 loader (fresh build when the origin is healthy).
 * 2. On ANY failure — offline, interrupted transfer, corrupt manifest,
 *    digest mismatch, mixed set — fall back to the cached builds and
 *    boot the first one that fully verifies (candidates tried in
 *    reverse-lexicographic cache order; buildIds are content hashes, so
 *    the order is deterministic but not chronological — this is a
 *    documented, bounded policy, not a claim of "newest").
 *
 * Never returns a partially verified or mixed set: every candidate goes
 * through {@link loadArtifacts} end-to-end. Throws the network failure
 * when no cached build verifies (first deployment, or all caches stale).
 */
export async function loadArtifactsWithFallback(
  env: {
    readonly fetch: (url: string) => Promise<Response>;
    readonly caches: {
      keys(): Promise<ReadonlyArray<string>>;
      open(name: string): Promise<{ match(url: string): Promise<Response | undefined> } | undefined>;
    };
    readonly manifestUrl: string;
    readonly baseUrl: string;
    readonly appId: string;
  },
): Promise<FallbackLoadedArtifacts> {
  let networkError: unknown;
  try {
    const response = await env.fetch(env.manifestUrl);
    if (!response.ok) {
      throw new Error(`manifest fetch → HTTP ${response.status}`);
    }
    const loaded = await loadArtifacts(await response.text(), async (url) =>
      new Uint8Array(await (await env.fetch(resolveAgainst(env.baseUrl, url))).arrayBuffer()),
    );
    return { source: "network", ...loaded };
  } catch (cause) {
    networkError = cause;
  }
  const prefix = `velqu:${env.appId}:`;
  const candidates = (await env.caches.keys())
    .filter((n) => n.startsWith(prefix))
    .sort()
    .reverse();
  for (const name of candidates) {
    const cache = await env.caches.open(name);
    if (!cache) continue;
    const manifestResponse = await cache.match(env.manifestUrl);
    if (!manifestResponse) continue;
    try {
      const loaded = await loadArtifacts(await manifestResponse.text(), async (url) => {
        const hit = await cache.match(resolveAgainst(env.baseUrl, url));
        if (!hit) throw new Error(`cache miss: ${url}`);
        return new Uint8Array(await hit.arrayBuffer());
      });
      return { source: "cache", ...loaded };
    } catch {
      // This cached build is incomplete/corrupt — try the next candidate.
    }
  }
  throw networkError instanceof Error
    ? networkError
    : new Error("no verified cached build available (last-known-good fallback exhausted)");
}
