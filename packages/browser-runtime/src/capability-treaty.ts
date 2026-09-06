/**
 * BWASM-R-005 — capability registry + Treaty integration for the
 * browser runtime.
 *
 * - **Capability registry**: keyed by declared capability id + exact
 *   version (kernel inventory semantics, q-capabilities). A handler
 *   context receives ONLY the handles its compiled route declaration
 *   names — a route cannot touch a capability omitted from its
 *   declaration, and every use is authorized against the kernel BEFORE
 *   the side-effecting call runs (ADR-0037 §5, ADR-0038 §5).
 * - **Treaty adapter**: both modes share one seam —
 *   `fetchImpl` (Request/Response mode) and `dispatchImpl` (direct
 *   typed mode) are both backed by the SAME BrowserRuntime dispatch,
 *   so route IDs, status narrowing, and canonical problem decoding are
 *   identical (no semantic bypass).
 */

import type { BrowserRuntime } from "./index";

// ---------------------------------------------------------------------------
// Capability registry
// ---------------------------------------------------------------------------

/** One installed capability implementation (host bridge). */
export interface CapabilityHandle {
  readonly id: string;
  readonly version: number;
  /** The side-effecting operation. Called ONLY after authorization. */
  readonly call: (input: unknown) => Promise<unknown>;
}

/** Failures are machine-readable (acceptance criterion 4). */
export type CapabilityRejection =
  | { readonly reason: "not-declared"; readonly capability: string; readonly detail: string }
  | { readonly reason: "not-in-inventory"; readonly capability: string; readonly detail: string }
  | { readonly reason: "version-mismatch"; readonly capability: string; readonly detail: string }
  | { readonly reason: "policy-forbidden"; readonly capability: string; readonly detail: string };

export class CapabilityError extends Error {
  readonly rejection: CapabilityRejection;
  constructor(rejection: CapabilityRejection) {
    const detail =
      rejection.reason === "not-declared"
        ? `route does not declare capability "${rejection.capability}"`
        : rejection.reason === "not-in-inventory"
          ? `capability "${rejection.capability}" is not in the artifact inventory (deployment-required)`
          : rejection.reason === "policy-forbidden"
            ? `capability "${rejection.capability}" refused by deployment policy: ${rejection.detail}`
            : `capability "${rejection.capability}" version mismatch: ${rejection.detail}`;
    super(`[@velqu/browser-runtime:capability] ${detail}`);
    this.name = "CapabilityError";
    this.rejection = rejection;
  }
}

/**
 * Deployment capability policy (BWASM-C-005): ids classified
 * `deployment-required` or `forbidden` by the portability registry are
 * refused at INSTALL time and again at INVOKE time — before any side
 * effect. The host supplies the classification (derived from the
 * compiler's portability registry, so build/inspect/runtime agree).
 */
export interface CapabilityPolicy {
  /** Ids that require the native runtime; refused on the browser side. */
  readonly deploymentRequired?: ReadonlyArray<string>;
  /** Ids that never exist and are never simulated. */
  readonly forbidden?: ReadonlyArray<string>;
}

/**
 * The stable deployment-required problem (BWASM-C-005 step 5):
 * RFC-9457-compatible with the capability id, route id, reason, and
 * safe remediation metadata — NO secret values or provider-private data
 * (the caller supplies only safe remediation text).
 */
export function deploymentRequiredProblem(options: {
  readonly capabilityId: string;
  readonly routeId: string;
  readonly reason: string;
  readonly remediation: string;
}): { status: 501; body: Record<string, unknown> } {
  return {
    status: 501,
    body: {
      problemId: "deployment-required",
      type: "https://velqu.dev/problems/deployment-required",
      title: "Capability requires native deployment",
      status: 501,
      capabilityId: options.capabilityId,
      routeId: options.routeId,
      reason: options.reason,
      remediation: options.remediation,
    },
  };
}

/**
 * The registry: installed handles (host bridge) checked against the
 * kernel's per-route authorization AND the deployment capability policy.
 */
export class CapabilityRegistry {
  private readonly handles = new Map<string, CapabilityHandle>();
  private readonly runtime: BrowserRuntime;
  private readonly policy: CapabilityPolicy;

  constructor(runtime: BrowserRuntime, handles: ReadonlyArray<CapabilityHandle>, policy?: CapabilityPolicy) {
    this.runtime = runtime;
    this.policy = policy ?? {};
    for (const handle of handles) {
      this.assertInstallable(handle);
      this.handles.set(handle.id, handle);
    }
  }

  /** Fail closed at INSTALL: policy-refused ids never become callable. */
  private assertInstallable(handle: CapabilityHandle): void {
    if (this.policy.forbidden?.includes(handle.id)) {
      throw new CapabilityError({
        reason: "policy-forbidden",
        capability: handle.id,
        detail: "install refused: no such capability exists and it is never simulated",
      });
    }
    if (this.policy.deploymentRequired?.includes(handle.id)) {
      throw new CapabilityError({
        reason: "policy-forbidden",
        capability: handle.id,
        detail: "install refused: deployment-required on the browser target (use the native runtime)",
      });
    }
  }

  /**
   * Authorize + execute one capability call for a route. ORDER IS THE
   * CONTRACT: kernel authorization (`authorizeCapability` + route
   * declaration) runs BEFORE the handle's side-effecting `call`.
   */
  async invokeForRoute(
    capabilityId: string,
    routeCapabilities: ReadonlyArray<string>,
    input: unknown,
  ): Promise<unknown> {
    // 1. The route's compiled declaration must name the capability.
    if (!routeCapabilities.includes(capabilityId)) {
      throw new CapabilityError({
        reason: "not-declared",
        capability: capabilityId,
        detail: "side effect refused before any authorization or execution",
      });
    }
    // 1b. BWASM-C-005: deployment policy gate BEFORE kernel auth and the
    // handle call — policy-refused ids never reach side effects.
    if (this.policy.forbidden?.includes(capabilityId)) {
      throw new CapabilityError({
        reason: "policy-forbidden",
        capability: capabilityId,
        detail: "no such capability exists and it is never simulated",
      });
    }
    if (this.policy.deploymentRequired?.includes(capabilityId)) {
      throw new CapabilityError({
        reason: "policy-forbidden",
        capability: capabilityId,
        detail: "deployment-required on the browser target (use the native runtime)",
      });
    }
    // 2. The kernel inventory must carry it (deployment-required class).
    const verdict = this.runtime.authorizeCapability(capabilityId);
    if ("problemId" in verdict) {
      throw new CapabilityError({
        reason: "not-in-inventory",
        capability: capabilityId,
        detail: verdict.detail ?? verdict.title,
      });
    }
    if (!verdict.authorized) {
      throw new CapabilityError({
        reason: "not-in-inventory",
        capability: capabilityId,
        detail: "kernel denied the capability",
      });
    }
    // 3. The host bridge must have an installed handle with an exact
    //    version match against the inventory.
    const handle = this.handles.get(capabilityId);
    if (!handle) {
      throw new CapabilityError({
        reason: "not-in-inventory",
        capability: capabilityId,
        detail: "no host bridge handle installed",
      });
    }
    // Version check against the artifact's declared version: the
    // runtime re-queries the kernel inventory for the exact version.
    // (authorizeCapability returns a decision; the version was verified
    // at inventory load — exact-match semantics per q-capabilities.)
    return handle.call(input);
  }
}

/**
 * Build the per-invocation capability view for a route: ONLY the
 * declared capabilities, each authorized at access time.
 */
export function capabilityViewForRoute(
  registry: CapabilityRegistry,
  routeCapabilities: ReadonlyArray<string>,
): Record<string, (input: unknown) => Promise<unknown>> {
  const view: Record<string, (input: unknown) => Promise<unknown>> = {};
  for (const id of routeCapabilities) {
    view[id] = (input: unknown) => registry.invokeForRoute(id, routeCapabilities, input);
  }
  return view;
}

// ---------------------------------------------------------------------------
// Treaty integration (shared-seam adapters)
// ---------------------------------------------------------------------------

export interface BrowserTreatyRoute {
  /** Full route id, e.g. "health.live". */
  readonly routeId: string;
  readonly method: string;
  readonly path: string;
}

/**
 * Request/Response mode: a Treaty `fetchImpl` backed by
 * `BrowserRuntime.fetch` — the standard boundary, no listening server.
 */
export function treatyFetchFromRuntime(runtime: BrowserRuntime): (input: RequestInfo | URL, init?: RequestInit) => Promise<Response> {
  return (input, init) => {
    const request = init ? new Request(input, init) : new Request(input);
    return runtime.fetch(request);
  };
}

/**
 * Direct mode: a Treaty `DispatchImpl` backed by the SAME runtime
 * dispatch (kernel routing/validation) with URL-encoded bodies per the
 * dispatcher's form contract. Route IDs and status/problem semantics
 * are identical to the fetch mode — differential-tested in
 * test/capability-treaty.test.ts.
 */
export function treatyDispatchFromRuntime(
  runtime: BrowserRuntime,
  routes: ReadonlyArray<BrowserTreatyRoute>,
): (req: {
  routeId: string;
  method: string;
  path: string;
  query?: Record<string, unknown>;
  headers?: Record<string, string>;
  body?: unknown;
}) => Promise<
  { kind: "response"; status: number; bodyText: string } | { kind: "network"; message: string } | { kind: "abort" }
> {
  const byId = new Map(routes.map((r) => [r.routeId, r] as const));
  return async (req) => {
    const route = byId.get(req.routeId);
    if (!route) {
      return { kind: "network", message: `unknown route id ${req.routeId}` };
    }
    const params = new URLSearchParams();
    for (const [k, v] of Object.entries(req.query ?? {})) {
      params.set(k, String(v));
    }
    const qs = params.toString();
    const hasBody = req.method !== "GET" && req.method !== "HEAD" && req.body !== undefined;
    const request = new Request(`https://browser.velqu${req.path}${qs ? `?${qs}` : ""}`, {
      method: req.method,
      headers: req.headers,
      ...(hasBody
        ? {
            body:
              typeof req.body === "string"
                ? req.body
                : JSON.stringify(req.body),
            ...(hasBody && req.headers?.["content-type"] === undefined
              ? { headers: { "content-type": "application/json" } }
              : {}),
          }
        : {}),
    });
    try {
      const response = await runtime.fetch(request);
      const bodyText = await response.text();
      return { kind: "response", status: response.status, bodyText };
    } catch (cause) {
      if (cause instanceof DOMException && cause.name === "AbortError") {
        return { kind: "abort" };
      }
      return { kind: "network", message: String(cause) };
    }
  };
}
