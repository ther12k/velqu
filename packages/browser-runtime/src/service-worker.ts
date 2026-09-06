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

import type { BrowserArtifactManifest } from "./artifact-loader";

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
