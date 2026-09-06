/**
 * @velqu/browser-runtime — Velqu's in-browser Request→Response runtime
 * boundary (BWASM-R-001, ADR-0037 §2).
 *
 * The canonical public surface is exactly one function shape:
 *
 *     fetch(request: Request): Promise<Response>
 *
 * A `BrowserRuntime` owns artifact (pack) bytes, initializes the
 * Rust/WASM kernel behind them, plans requests, and normalizes
 * completions into standard `Response` objects. Handler execution in
 * an isolated Worker is wired by the dispatcher packet (BWASM-R-002);
 * this package defines and enforces the boundary, lifecycle, and
 * error contract.
 *
 * Browser-only: this package must never import Bun.*, node:*, native
 * addons, @velqu/testing, or any native runtime surface (enforced by
 * the forbidden-import test). The kernel itself is injected as a
 * `KernelModule` so the package stays bundler-friendly and does not
 * own .wasm asset loading (that is the artifact/loader contract of
 * BWASM-B-002).
 */

// ---------------------------------------------------------------------------
// Kernel ABI mirror (K-005 wasm-bindgen surface)
// ---------------------------------------------------------------------------

/**
 * The wasm-bindgen surface of `q-browser-kernel` (K-005). The compiled
 * glue module satisfies this interface structurally; the runtime never
 * assumes anything beyond it.
 */
export interface KernelModule {
  readonly kernel_abi_version: () => number;
  new (packBytes: Uint8Array): KernelInstance;
}

/** A live kernel instance (K-005 `WasmKernel`). */
export interface KernelInstance {
  plan_request(requestJson: string): string;
  complete_invocation(completionJson: string): string;
  authorize_capability(name: string): string;
  dispose(): void;
}

/** Wire shapes of the kernel ABI messages (camelCase, ADR §ABI). */
export interface KernelPlanRequest {
  readonly abiVersion: number;
  readonly method: string;
  readonly path: string;
  readonly query?: ReadonlyArray<readonly [string, string]>;
  readonly headers?: ReadonlyArray<readonly [string, string]>;
  readonly body?: string;
}

export interface KernelInvokePlan {
  readonly kind: "invoke";
  readonly abiVersion: number;
  readonly routeId: number;
  readonly handlerKey: string;
  readonly policyKey?: string;
  readonly params?: unknown;
  readonly query?: unknown;
  readonly headers?: unknown;
  readonly body?: unknown;
  readonly bodyText?: string;
  readonly allowedStatuses: ReadonlyArray<number>;
  readonly defaultStatus: number;
  readonly deadlineMs: number;
}

export interface KernelProblemShape {
  readonly kind: "problem";
  readonly problem: {
    readonly problemId: string;
    readonly type: string;
    readonly title: string;
    readonly status: number;
    readonly detail?: string;
    readonly errors?: ReadonlyArray<{ path: string; code: string; message: string }>;
    readonly allow?: ReadonlyArray<string>;
  };
}

export type KernelPlanResult = KernelInvokePlan | KernelProblemShape;

// ---------------------------------------------------------------------------
// Lifecycle and errors
// ---------------------------------------------------------------------------

/** Runtime lifecycle states; transitions are forward-only to `disposed`. */
export type BrowserRuntimeState = "idle" | "ready" | "disposed";

/**
 * Structured runtime error. Every failure surfaces as one of these —
 * never a bare console log and never a swallowed success.
 */
export class BrowserRuntimeError extends Error {
  readonly code:
    | "KERNEL_ABI_MISMATCH"
    | "ARTIFACT_REJECTED"
    | "RUNTIME_DISPOSED"
    | "RUNTIME_NOT_READY"
    | "REQUEST_INVALID"
    | "KERNEL_PROBLEM"
    | "KERNEL_PROTOCOL";
  readonly status: number;
  /** Present for KERNEL_PROBLEM: the kernel's typed problem object. */
  readonly problem?: KernelProblemShape["problem"];

  constructor(
    code: BrowserRuntimeError["code"],
    message: string,
    options?: { status?: number; problem?: KernelProblemShape["problem"]; cause?: unknown },
  ) {
    super(`[@velqu/browser-runtime:${code}] ${message}`, { cause: options?.cause });
    this.name = "BrowserRuntimeError";
    this.code = code;
    this.status = options?.status ?? 500;
    this.problem = options?.problem;
  }
}

export interface BrowserRuntimeOptions {
  /**
   * Verified pack (artifact) bytes. The runtime initializes the kernel
   * against exactly these bytes; integrity failures reject at create
   * time with ARTIFACT_REJECTED.
   */
  readonly packBytes: Uint8Array;
  /** The K-005 kernel module (wasm-bindgen glue class + version fn). */
  readonly kernel: KernelModule;
  /** Expected kernel ABI version; defaults to the current contract (1). */
  readonly expectedAbiVersion?: number;
}

// ---------------------------------------------------------------------------
// BrowserRuntime
// ---------------------------------------------------------------------------

interface KernelResponseShape {
  readonly kind: "response";
  readonly status: number;
  readonly headers: ReadonlyArray<readonly [string, string]>;
  readonly body?: unknown;
}

/**
 * The browser runtime. Create via {@link createBrowserRuntime}; call
 * `fetch(request)` for any planned route. Lifecycle is explicit:
 * `state` moves `idle → ready` at creation and `→ disposed` via
 * `dispose()`; post-dispose calls fail with RUNTIME_DISPOSED.
 */
export interface BrowserRuntime {
  readonly state: BrowserRuntimeState;
  /** Kernel ABI version the runtime was verified against. */
  readonly abiVersion: number;
  /**
   * The canonical boundary (ADR-0037 §2): accept a standard `Request`,
   * resolve a standard `Response`. Kernel problems map to Response
   * bodies carrying the RFC-9457-shaped problem JSON with the kernel's
   * status; transport-level failures (bad input, disposed runtime)
   * reject with {@link BrowserRuntimeError}.
   */
  fetch(request: Request): Promise<Response>;
  /** Bridge query: authorize a declared capability call (fail-closed). */
  authorizeCapability(name: string): { authorized: boolean } | KernelProblemShape["problem"];
  /** Explicit disposal; idempotent. */
  dispose(): void;
}

function parseKernelMessage<T>(raw: string, what: string): T {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch (cause) {
    throw new BrowserRuntimeError("KERNEL_PROTOCOL", `kernel ${what} was not valid JSON`, {
      cause,
    });
  }
  return parsed as T;
}

function isProblem(x: unknown): x is KernelProblemShape {
  return (
    typeof x === "object" && x !== null && (x as { kind?: unknown }).kind === "problem" &&
    typeof (x as KernelProblemShape).problem === "object"
  );
}

/** Create a BrowserRuntime: verify ABI, initialize the kernel, fail closed. */
export function createBrowserRuntime(options: BrowserRuntimeOptions): BrowserRuntime {
  const expectedAbi = options.expectedAbiVersion ?? 1;
  const kernelAbi = options.kernel.kernel_abi_version();
  if (kernelAbi !== expectedAbi) {
    throw new BrowserRuntimeError(
      "KERNEL_ABI_MISMATCH",
      `kernel reports ABI ${kernelAbi}, runtime contract expects ${expectedAbi}`,
    );
  }

  let instance: KernelInstance;
  try {
    instance = new options.kernel(options.packBytes);
  } catch (cause) {
    // The bindgen constructor carries the artifact problem as its error
    // message JSON (K-005); surface it structurally.
    let problem: KernelProblemShape["problem"] | undefined;
    if (cause instanceof Error) {
      try {
        const parsed = JSON.parse(cause.message) as KernelProblemShape["problem"];
        if (typeof parsed?.problemId === "string") problem = parsed;
      } catch {
        problem = undefined;
      }
    }
    throw new BrowserRuntimeError("ARTIFACT_REJECTED", "kernel rejected the artifact", {
      problem,
      cause,
    });
  }

  let state: BrowserRuntimeState = "ready";
  const requireReady = (): void => {
    if (state === "disposed") {
      throw new BrowserRuntimeError("RUNTIME_DISPOSED", "fetch on a disposed runtime");
    }
  };

  return {
    get state(): BrowserRuntimeState {
      return state;
    },
    abiVersion: kernelAbi,

    async fetch(request: Request): Promise<Response> {
      requireReady();

      const url = new URL(request.url);
      const query: Array<[string, string]> = [];
      url.searchParams.forEach((value, key) => query.push([key, value]));
      const headers: Array<[string, string]> = [];
      request.headers.forEach((value, key) => headers.push([key, value]));
      let body: string | undefined;
      if (request.method !== "GET" && request.method !== "HEAD") {
        body = await request.text();
      }

      const message: KernelPlanRequest = {
        abiVersion: kernelAbi,
        method: request.method,
        path: url.pathname,
        query,
        headers,
        ...(body !== undefined ? { body } : {}),
      };

      const plan = parseKernelMessage<KernelPlanResult>(
        instance.plan_request(JSON.stringify(message)),
        "plan",
      );
      if (isProblem(plan)) {
        return problemResponse(plan.problem);
      }

      // Handler execution is the dispatcher's concern (BWASM-R-002):
      // this boundary contract resolves the plan's default response via
      // the kernel completion path so the Request→Response seam is real
      // end-to-end today. The dispatcher replaces `executeHandler`.
      const completion = {
        abiVersion: kernelAbi,
        routeId: plan.routeId,
        result: await executeHandler(plan),
      };
      const completed = parseKernelMessage<KernelResponseShape | KernelProblemShape>(
        instance.complete_invocation(JSON.stringify(completion)),
        "completion",
      );
      if (isProblem(completed)) {
        return problemResponse(completed.problem);
      }
      return new Response(
        completed.body === undefined ? null : JSON.stringify(completed.body),
        {
          status: completed.status,
          headers: new Headers(completed.headers.map(([k, v]) => [k, v] as [string, string])),
        },
      );
    },

    authorizeCapability(name: string): { authorized: boolean } | KernelProblemShape["problem"] {
      requireReady();
      const raw = instance.authorize_capability(name);
      const parsed = parseKernelMessage<{ authorized?: boolean } | KernelProblemShape>(
        raw,
        "capability authorization",
      );
      if (isProblem(parsed)) return parsed.problem;
      if (typeof parsed.authorized !== "boolean") {
        throw new BrowserRuntimeError(
          "KERNEL_PROTOCOL",
          "capability authorization returned neither a decision nor a problem",
        );
      }
      return { authorized: parsed.authorized };
    },

    dispose(): void {
      if (state === "disposed") return;
      state = "disposed";
      instance.dispose();
    },
  };
}

/**
 * Placeholder handler execution seam (BWASM-R-002 owns the real Worker
 * dispatcher). Today the boundary resolves a plan with the route's
 * declared default status and an empty object body so the complete
 * path (declared-status enforcement!) is exercised for real: an
 * undeclared default would produce the kernel's contract-violation
 * problem, proving the seam end-to-end.
 */
async function executeHandler(plan: KernelInvokePlan): Promise<{
  kind: "response";
  status: number;
  headers: Array<[string, string]>;
  body: unknown;
}> {
  return {
    kind: "response",
    status: plan.defaultStatus,
    headers: [["content-type", "application/json"]],
    body: {},
  };
}

function problemResponse(problem: KernelProblemShape["problem"]): Response {
  const headers = new Headers();
  headers.set("content-type", "application/problem+json");
  if (problem.allow && problem.allow.length > 0) {
    headers.set("allow", problem.allow.join(", "));
  }
  return new Response(JSON.stringify({ ...problem }), {
    status: problem.status,
    headers,
  });
}
