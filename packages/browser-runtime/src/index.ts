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

import {
  dispatchFetchRequest,
  DispatcherError,
  DEFAULT_MAX_BODY_BYTES,
} from "./dispatcher";
import {
  DIAGNOSTIC_CODES,
  extractOrGenerateCorrelationId,
  type DiagnosticStream,
} from "./diagnostics";

export {
  DIAGNOSTIC_CODES,
  createDiagnosticStream,
  createInspectorAdapter,
  extractOrGenerateCorrelationId,
  generateCorrelationId,
  mapDiagnosticError,
  redactDiagnosticMetadata,
  DiagnosticStream,
  InspectorPanelAdapter,
  type DiagnosticCode,
  type DiagnosticEvent,
  type DiagnosticLevel,
  type DiagnosticLifecycleStage,
  type DiagnosticStreamOptions,
  type DiagnosticTraceExport,
  type InspectorSummary,
  type MappedErrorDiagnostic,
} from "./diagnostics";

export {
  defineBrowserHandlers,
  emitHandlerBundleMetadata,
  sanitizeSourceLocation,
  HANDLER_ABI_VERSION,
  HandlerBundleError,
  type BrowserHandlerRegistration,
  type HandlerBundleMetadata,
  type HandlerContext,
  type HandlerResult,
  type HandlerTable,
  type PackHandlerExpectation,
} from "./handler-bundle";

export {
  WorkerHost,
  WorkerProtocolError,
  WORKER_PROTOCOL_VERSION,
  workerBootstrapSource,
  MAX_LOG_LINES_PER_INVOCATION,
  MAX_RESULT_BYTES,
  type WorkerLike,
  type WorkerFactory,
  type WorkerToHostMessage,
  type HostToWorkerMessage,
} from "./worker-host";

export {
  createMemoryKv,
  createIndexedDbKv,
  kvHandle,
  KvError,
  KvKeyInvalid,
  KvQuotaExceeded,
  KvSerializationError,
  KvMigrationRequired,
  KvUnavailable,
  KV_CAPABILITY_ID,
  KV_CAPABILITY_VERSION,
  KV_META_KEY,
  MAX_KV_KEY_BYTES,
  MAX_KV_VALUE_BYTES,
  MAX_KV_ENTRIES,
  type KvCapability,
  type KvValue,
  type KvEntry,
  type KvOptions,
  type KvMigrationStore,
  type KvIdbFactory,
  type KvIdbDatabase,
  type MemoryKv,
  type IndexedDbKv,
} from "./kv";

export {
  createBrowserCapabilityGraph,
  createTimerCapability,
  createCryptoCapability,
  createConsoleCapability,
  createFetchCapability,
  baselineHandles,
  redactSensitiveText,
  ringBufferSink,
  BrowserCapabilityError,
  TimerCancelled,
  TimerDelayTooLarge,
  CryptoSemanticsMismatch,
  ConsoleLimitExceeded,
  FetchPolicyDenied,
  FetchLimitExceeded,
  BROWSER_TIMERS_ID,
  BROWSER_CRYPTO_ID,
  BROWSER_CONSOLE_ID,
  BROWSER_FETCH_ID,
  BROWSER_ABORT_ID,
  BROWSER_TEXT_ID,
  BROWSER_URL_ID,
  BROWSER_CAPABILITY_VERSION,
  MAX_BROWSER_TIMER_DELAY_MS,
  MAX_CONSOLE_MSG_LEN,
  MAX_CONSOLE_ARGS,
  MAX_CONSOLE_RECORDS,
  MAX_FETCH_DEADLINE_MS,
  DEFAULT_FETCH_DEADLINE_MS,
  MAX_FETCH_REQUEST_BODY_BYTES,
  MAX_FETCH_RESPONSE_BODY_BYTES,
  MAX_GET_RANDOM_VALUES_BYTES,
  DIGEST_ALGORITHMS,
  type CapabilityDescriptor,
  type ConsoleLevel,
  type ConsoleRecord,
  type ConsoleSink,
  type TimerDelayInput,
  type TimerDelayResult,
  type CryptoDigestInput,
  type BrowserFetchPolicy,
  type BrowserFetchInput,
  type BrowserFetchResult,
  type BrowserCapabilityGraph,
} from "./capabilities";

export {
  isScopedRequest,
  classifyRequest,
  buildCachePlan,
  cacheNameFor,
  handleFetchEvent,
  bootstrapServiceWorker,
  updateDecision,
  precacheVerified,
  cachesToKeep,
  loadArtifactsWithFallback,
  SW_SHELL_FILES,
  PASSTHROUGH_PATH_PREFIXES,
  type FetchEventLike,
  type WorkerEnv,
  type RequestClass,
  type CachePlanEntry,
  type BootstrapOutcome,
  type FallbackLoadedArtifacts,
} from "./service-worker";

export {
  emitArtifactManifest,
  loadArtifacts,
  sha256Hex,
  canonicalManifestJson,
  buildIdOf,
  resolveArtifactUrl,
  ArtifactManifestError,
  ARTIFACT_MANIFEST_VERSION,
  MEDIA_TYPES,
  type ArtifactEntry,
  type ArtifactRole,
  type BrowserArtifactManifest,
  type LoadedArtifacts,
  type ArtifactReader,
} from "./artifact-loader";

export {
  CapabilityRegistry,
  CapabilityError,
  deploymentRequiredProblem,
  type CapabilityPolicy,
  capabilityViewForRoute,
  treatyFetchFromRuntime,
  treatyDispatchFromRuntime,
  type CapabilityHandle,
  type CapabilityRejection,
  type BrowserTreatyRoute,
} from "./capability-treaty";

export {
  dispatchFetchRequest,
  DispatcherError,
  UNSUPPORTED_SEMANTICS,
  DEFAULT_MAX_BODY_BYTES,
  MAX_MULTIPART_PARTS,
  MAX_QUERY_PAIRS,
  MAX_HEADER_PAIRS,
  type DispatcherOptions,
  type DispatcherRejection,
} from "./dispatcher";

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
  /** Bounded request-body cap (default 1 MiB; ADR-0037 §5). */
  readonly maxBodyBytes?: number;
  /** Abort signal applied to every fetch (R-002 abort contract). */
  readonly signal?: AbortSignal | null;
  /**
   * R-004 seam: executes an invoke plan and returns the completion
   * result. Defaults to the declared-default executor (the kernel's
   * declared-status enforcement still runs on the completion).
   */
  readonly executeHandler?: (
    plan: KernelInvokePlan,
  ) => Promise<{
    kind: "response";
    status: number;
    headers: Array<[string, string]>;
    body: unknown;
  }>;
  /** Bounded developer diagnostic stream (BWASM-Q-004). */
  readonly diagnostics?: DiagnosticStream;
}

// ---------------------------------------------------------------------------
// BrowserRuntime
// ---------------------------------------------------------------------------

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
  /** Diagnostic stream if configured (BWASM-Q-004). */
  readonly diagnostics?: DiagnosticStream;
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
  const diag = options.diagnostics;
  const expectedAbi = options.expectedAbiVersion ?? 1;
  const kernelAbi = options.kernel.kernel_abi_version();
  if (kernelAbi !== expectedAbi) {
    diag?.record({
      stage: "instantiate",
      code: DIAGNOSTIC_CODES.COMPAT_KERNEL_ABI_MISMATCH,
      level: "error",
      detail: `kernel reports ABI ${kernelAbi}, runtime contract expects ${expectedAbi}`,
    });
    throw new BrowserRuntimeError(
      "KERNEL_ABI_MISMATCH",
      `kernel reports ABI ${kernelAbi}, runtime contract expects ${expectedAbi}`,
    );
  }

  diag?.record({
    stage: "instantiate",
    code: DIAGNOSTIC_CODES.LIFECYCLE_INSTANTIATING,
    level: "info",
    detail: "initializing runtime kernel",
  });

  let instance: KernelInstance;
  try {
    instance = new options.kernel(options.packBytes);
  } catch (cause) {
    diag?.record({
      stage: "verify",
      code: DIAGNOSTIC_CODES.VERIFY_INTEGRITY_FAIL,
      level: "error",
      detail: "kernel rejected the artifact",
    });
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

  diag?.record({
    stage: "instantiate",
    code: DIAGNOSTIC_CODES.LIFECYCLE_READY,
    level: "info",
    detail: `kernel ready (ABI ${kernelAbi})`,
  });

  let state: BrowserRuntimeState = "ready";
  const requireReady = (): void => {
    if (state === "disposed") {
      diag?.record({
        stage: "fail",
        code: DIAGNOSTIC_CODES.FAIL_INTERNAL,
        level: "error",
        detail: "fetch on a disposed runtime",
      });
      throw new BrowserRuntimeError("RUNTIME_DISPOSED", "fetch on a disposed runtime");
    }
  };

  return {
    get state(): BrowserRuntimeState {
      return state;
    },
    abiVersion: kernelAbi,
    diagnostics: diag,

    async fetch(request: Request): Promise<Response> {
      requireReady();
      const correlationId = extractOrGenerateCorrelationId(request.headers);
      diag?.record({
        stage: "route",
        code: DIAGNOSTIC_CODES.ROUTE_MATCHED,
        level: "debug",
        correlationId,
        detail: `${request.method} ${request.url}`,
      });

      // R-002: the dispatcher owns boundary normalization, body forms,
      // abort, HEAD policy, and the kernel round-trip. The kernel stays
      // the only routing/validation authority (no JS fast path).
      try {
        const res = await dispatchFetchRequest(instance, kernelAbi, request, {
          maxBodyBytes: options.maxBodyBytes ?? DEFAULT_MAX_BODY_BYTES,
          signal: options.signal ?? null,
          ...(options.executeHandler ? { executeHandler: options.executeHandler } : {}),
        });
        if (res.status === 404) {
          diag?.record({
            stage: "route",
            code: DIAGNOSTIC_CODES.ROUTE_NOT_FOUND,
            level: "warn",
            correlationId,
            detail: `route not found: ${request.url}`,
          });
        } else if (res.status === 405) {
          diag?.record({
            stage: "route",
            code: DIAGNOSTIC_CODES.ROUTE_METHOD_NOT_ALLOWED,
            level: "warn",
            correlationId,
            detail: `method ${request.method} not allowed on ${request.url}`,
          });
        } else if (res.status === 422) {
          diag?.record({
            stage: "validate",
            code: DIAGNOSTIC_CODES.VALIDATE_FAILED,
            level: "warn",
            correlationId,
            detail: "schema validation failed",
          });
        }
        return res;
      } catch (cause: unknown) {
        if (cause instanceof DOMException && cause.name === "AbortError") {
          diag?.record({
            stage: "cancel",
            code: DIAGNOSTIC_CODES.INVOKE_CANCEL,
            level: "info",
            correlationId,
            detail: "operation aborted",
          });
          throw cause;
        }
        diag?.record({
          stage: "fail",
          code: DIAGNOSTIC_CODES.FAIL_INTERNAL,
          level: "error",
          correlationId,
          detail: cause instanceof Error ? cause.message : String(cause),
        });
        if (cause instanceof BrowserRuntimeError) throw cause;
        if (cause instanceof DispatcherError) {
          const code =
            cause.rejection.kind === "protocol"
              ? "KERNEL_PROTOCOL"
              : cause.rejection.kind === "unsupported"
                ? "REQUEST_INVALID"
                : "REQUEST_INVALID";
          throw new BrowserRuntimeError(code, cause.message, { cause });
        }
        throw new BrowserRuntimeError("KERNEL_PROTOCOL", String(cause), { cause });
      }
    },

    authorizeCapability(name: string): { authorized: boolean } | KernelProblemShape["problem"] {
      requireReady();
      diag?.record({
        stage: "capability",
        code: DIAGNOSTIC_CODES.CAPABILITY_INVOKED,
        level: "debug",
        detail: `authorizing capability: ${name}`,
      });
      const raw = instance.authorize_capability(name);
      const parsed = parseKernelMessage<{ authorized?: boolean } | KernelProblemShape>(
        raw,
        "capability authorization",
      );
      if (isProblem(parsed)) {
        diag?.record({
          stage: "capability",
          code: parsed.problem.problemId === "deployment_required"
            ? DIAGNOSTIC_CODES.CAPABILITY_DEPLOYMENT_REQUIRED
            : DIAGNOSTIC_CODES.CAPABILITY_DENIED,
          level: "warn",
          detail: `capability ${name} rejected: ${parsed.problem.title}`,
        });
        return parsed.problem;
      }
      if (typeof parsed.authorized !== "boolean") {
        diag?.record({
          stage: "fail",
          code: DIAGNOSTIC_CODES.FAIL_INTERNAL,
          level: "error",
          detail: `capability authorization returned neither a decision nor a problem for ${name}`,
        });
        throw new BrowserRuntimeError(
          "KERNEL_PROTOCOL",
          "capability authorization returned neither a decision nor a problem",
        );
      }
      if (!parsed.authorized) {
        diag?.record({
          stage: "capability",
          code: DIAGNOSTIC_CODES.CAPABILITY_DENIED,
          level: "warn",
          detail: `capability ${name} not authorized`,
        });
      }
      return { authorized: parsed.authorized };
    },

    dispose(): void {
      if (state === "disposed") return;
      state = "disposed";
      instance.dispose();
      diag?.record({
        stage: "instantiate",
        code: DIAGNOSTIC_CODES.LIFECYCLE_DISPOSED,
        level: "info",
        detail: "runtime disposed",
      });
    },
  };
}
