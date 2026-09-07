/**
 * BWASM-R-004 — isolated Worker execution host for generated handlers.
 *
 * Design (ADR-0037 §3, ADR-0038):
 * - handlers run in a dedicated Worker OUTSIDE the parent realm; the
 *   parent exchanges only structured, schema-checked messages;
 * - the Worker executes one invoke at a time; the supervisor enforces
 *   the plan's `deadlineMs` and hard-terminates on overrun
 *   (kill-and-replace — ADR-0038 §5 recovery model);
 * - the Worker constructor is INJECTED (`WorkerFactory`) so the same
 *   supervisor runs against real browser Workers and test doubles;
 * - invocation IDs are runtime-scoped UUID-ish counters combined with
 *   a per-runtime session token: cross-project/runtime result theft is
 *   structurally impossible (a stale/foreign message is dropped).
 *
 * Isolation honesty (binding doc statement, acceptance criterion):
 * Worker isolation is NOT by itself a proven hostile-code sandbox.
 * Same-origin Workers share the origin's authority surface; trusted-
 * handler conventions and deployment posture (separate preview origin,
 * sandboxed iframe — ADR-0038 §3) provide the real boundaries.
 */

import type { HandlerContext, HandlerResult, KernelInvokePlan } from "./index";
import {
  DIAGNOSTIC_CODES,
  type DiagnosticStream,
} from "./diagnostics";

type WorkerToHostResultMessage = Extract<
  WorkerToHostMessage,
  { type: "result" }
>;

/** Wire protocol version of the Worker messages. */
export const WORKER_PROTOCOL_VERSION = 1;

/** Bounded log forwarding: lines per invocation and bytes per line. */
export const MAX_LOG_LINES_PER_INVOCATION = 64;
export const MAX_LOG_LINE_BYTES = 512;
/** Hard output cap for a single handler result message. */
export const MAX_RESULT_BYTES = 1 << 20;

// ---------------------------------------------------------------------------
// Message protocol (both directions, validated on receipt)
// ---------------------------------------------------------------------------

export type WorkerToHostMessage =
  | {
    readonly v: typeof WORKER_PROTOCOL_VERSION;
    readonly type: "ready";
    readonly handlerKeys: ReadonlyArray<string>;
  }
  | {
    readonly v: typeof WORKER_PROTOCOL_VERSION;
    readonly type: "result";
    readonly sessionId: string;
    readonly invocationId: number;
    readonly correlationId?: string;
    readonly result: HandlerResult;
  }
  | {
    readonly v: typeof WORKER_PROTOCOL_VERSION;
    readonly type: "log";
    readonly sessionId: string;
    readonly invocationId: number;
    readonly correlationId?: string;
    readonly lines: ReadonlyArray<string>;
  }
  | {
    readonly v: typeof WORKER_PROTOCOL_VERSION;
    readonly type: "fatal";
    readonly sessionId: string;
    readonly detail: string;
  };

export type HostToWorkerMessage =
  | {
    readonly v: typeof WORKER_PROTOCOL_VERSION;
    readonly type: "invoke";
    readonly sessionId: string;
    readonly invocationId: number;
    readonly correlationId?: string;
    readonly plan: KernelInvokePlan;
  }
  | {
    readonly v: typeof WORKER_PROTOCOL_VERSION;
    readonly type: "cancel";
    readonly sessionId: string;
    readonly invocationId: number;
    readonly correlationId?: string;
  };

/** The minimal Worker-like surface the supervisor needs (injectable). */
export interface WorkerLike {
  postMessage(message: unknown): void;
  terminate(): void;
  addEventListener(type: "message", listener: (event: { data: unknown }) => void): void;
}

export type WorkerFactory = () => WorkerLike;

/** Structured rejection for uncloneable/oversized payloads. */
export class WorkerProtocolError extends Error {
  readonly code: "UNCLONEABLE_PAYLOAD" | "OVERSIZED_PAYLOAD" | "STALE_MESSAGE" | "FATAL";
  constructor(code: WorkerProtocolError["code"], message: string) {
    super(`[@velqu/browser-runtime:worker:${code}] ${message}`);
    this.name = "WorkerProtocolError";
    this.code = code;
  }
}

// ---------------------------------------------------------------------------
// Supervisor
// ---------------------------------------------------------------------------

export interface WorkerHostOptions {
  readonly sessionId: string;
  readonly workerFactory: WorkerFactory;
  /** Handler executor INSIDE the worker realm (the worker bootstrap calls this). */
  readonly onInvoke?: never;
}

interface PendingInvocation {
  readonly invocationId: number;
  readonly correlationId?: string;
  readonly plan: KernelInvokePlan;
  readonly resolve: (result: HandlerResult) => void;
  readonly reject: (err: Error) => void;
  timer: ReturnType<typeof setTimeout> | null;
  logLines: number;
}

/**
 * Supervisor over one Worker: bounded deadlines, session-scoped
 * results, kill-and-replace recovery, structured payload errors.
 */
export class WorkerHost {
  private worker: WorkerLike;
  private readonly sessionId: string;
  private readonly factory: WorkerFactory;
  private readonly diagnostics?: DiagnosticStream;
  private pending = new Map<number, PendingInvocation>();
  private nextInvocationId = 1;
  private terminated = false;
  /** Late/stale results dropped by the supervisor (evidence counter). */
  public staleMessagesDropped = 0;
  public hardRecoveries = 0;

  constructor(
    sessionId: string,
    factory: WorkerFactory,
    options?: { diagnostics?: DiagnosticStream },
  ) {
    this.sessionId = sessionId;
    this.factory = factory;
    this.diagnostics = options?.diagnostics;
    this.worker = this.spawn();
  }

  private spawn(): WorkerLike {
    const worker = this.factory();
    worker.addEventListener("message", (event) => this.onMessage(event.data));
    return worker;
  }

  get isTerminated(): boolean {
    return this.terminated;
  }

  /**
   * Execute one plan. Resolves with the handler result or rejects:
   * deadline overrun (hard recovery happens automatically and the host
   * REMAINS USABLE — a fresh Worker serves the next call), abort, or
   * protocol errors.
   */
  execute(
    plan: KernelInvokePlan,
    signal?: AbortSignal | null,
    correlationId?: string,
  ): Promise<HandlerResult> {
    if (this.terminated) {
      // Kill-and-replace keeps the host usable: spawn a fresh worker.
      this.terminated = false;
      this.worker = this.spawn();
    }
    const invocationId = this.nextInvocationId++;
    this.diagnostics?.record({
      stage: "invoke",
      code: DIAGNOSTIC_CODES.INVOKE_START,
      level: "debug",
      correlationId,
      routeId: plan.routeId,
      detail: `invoking handler ${plan.handlerKey}`,
    });

    return new Promise<HandlerResult>((resolve, reject) => {
      const pending: PendingInvocation = {
        invocationId,
        correlationId,
        plan,
        resolve,
        reject,
        timer: null,
        logLines: 0,
      };
      this.pending.set(invocationId, pending);

      const abort = (): void => {
        if (!this.pending.has(invocationId)) return;
        this.pending.delete(invocationId);
        this.diagnostics?.record({
          stage: "cancel",
          code: DIAGNOSTIC_CODES.INVOKE_CANCEL,
          level: "info",
          correlationId,
          routeId: plan.routeId,
          detail: "invoke aborted by signal",
        });
        this.worker.postMessage({
          v: WORKER_PROTOCOL_VERSION,
          type: "cancel",
          sessionId: this.sessionId,
          invocationId,
          correlationId,
        });
        reject(new DOMException("The operation was aborted.", "AbortError"));
      };
      if (signal?.aborted) {
        abort();
        return;
      }
      signal?.addEventListener("abort", abort, { once: true });

      // Deadline enforcement (host-side, bounded — the Worker cannot
      // extend it; overruns hard-terminate the Worker).
      pending.timer = setTimeout(() => {
        if (!this.pending.has(invocationId)) return;
        this.pending.delete(invocationId);
        this.diagnostics?.record({
          stage: "invoke",
          code: DIAGNOSTIC_CODES.INVOKE_TIMEOUT,
          level: "error",
          correlationId,
          routeId: plan.routeId,
          detail: `deadline ${plan.deadlineMs}ms exceeded`,
        });
        this.hardTerminate(`deadline ${plan.deadlineMs}ms exceeded`);
        reject(new DOMException("The operation was aborted.", "AbortError"));
      }, plan.deadlineMs);

      let message: string;
      try {
        message = JSON.stringify({
          v: WORKER_PROTOCOL_VERSION,
          type: "invoke",
          sessionId: this.sessionId,
          invocationId,
          correlationId,
          plan,
        });
      } catch (cause) {
        this.pending.delete(invocationId);
        reject(new WorkerProtocolError("UNCLONEABLE_PAYLOAD", String(cause)));
        return;
      }
      if (message.length > MAX_RESULT_BYTES) {
        this.pending.delete(invocationId);
        reject(new WorkerProtocolError("OVERSIZED_PAYLOAD", "invoke message exceeds 1 MiB"));
        return;
      }
      // postMessage with a structured-clone-safe shape (plain object).
      this.worker.postMessage(JSON.parse(message));
    });
  }

  /** Hard recovery: terminate now; next execute() spawns a fresh Worker. */
  private hardTerminate(reason: string): void {
    this.hardRecoveries += 1;
    this.worker.terminate();
    this.terminated = true;
    // Every still-pending invocation rejects deterministically.
    for (const pending of this.pending.values()) {
      if (pending.timer) clearTimeout(pending.timer);
      pending.reject(new WorkerProtocolError("FATAL", `worker terminated: ${reason}`));
    }
    this.pending.clear();
  }

  dispose(): void {
    this.hardTerminate("disposed");
  }

  private onMessage(data: unknown): void {
    if (typeof data !== "object" || data === null) return;
    // Manual narrowing: Partial<T> loses the discriminant contract, and
    // the wire is untrusted — validate fields explicitly per type.
    const msg = data as Record<string, unknown> & { type?: string };
    if (msg.v !== WORKER_PROTOCOL_VERSION) return; // unknown protocol: drop
    if (
      msg.type !== "result" && msg.type !== "log" && msg.type !== "fatal"
    ) {
      return; // unknown type: drop
    }
    if (
      typeof msg.sessionId !== "string" ||
      (msg.type !== "fatal" && typeof msg.invocationId !== "number") ||
      (msg.type === "result" &&
        (typeof msg.result !== "object" || msg.result === null)) ||
      (msg.type === "log" && !Array.isArray(msg.lines))
    ) {
      this.staleMessagesDropped += 1; // malformed: drop
      return;
    }
    switch (msg.type) {
      case "result": {
        if (msg.sessionId !== this.sessionId) {
          this.staleMessagesDropped += 1; // foreign session: drop
          return;
        }
        const invocationId = msg.invocationId as number;
        const pending = this.pending.get(invocationId);
        if (!pending) {
          this.staleMessagesDropped += 1; // late result from a terminated run
          return;
        }
        this.pending.delete(invocationId);
        if (pending.timer) clearTimeout(pending.timer);
        const result = msg.result as WorkerToHostResultMessage["result"];
        if (this.oversized(result)) {
          this.diagnostics?.record({
            stage: "invoke",
            code: DIAGNOSTIC_CODES.INVOKE_FAILED,
            level: "error",
            correlationId: pending.correlationId,
            routeId: pending.plan.routeId,
            detail: "result exceeds 1 MiB",
          });
          pending.reject(
            new WorkerProtocolError("OVERSIZED_PAYLOAD", "result exceeds 1 MiB"),
          );
          return;
        }
        if (result.kind === "response") {
          this.diagnostics?.record({
            stage: "invoke",
            code: DIAGNOSTIC_CODES.INVOKE_SUCCESS,
            level: "debug",
            correlationId: pending.correlationId,
            routeId: pending.plan.routeId,
            detail: `status ${result.status}`,
          });
        } else {
          this.diagnostics?.record({
            stage: "invoke",
            code: DIAGNOSTIC_CODES.INVOKE_FAILED,
            level: "warn",
            correlationId: pending.correlationId,
            routeId: pending.plan.routeId,
            detail: `problem ${result.problemId}`,
          });
        }
        pending.resolve(result);
        return;
      }
      case "log": {
        if (msg.sessionId !== this.sessionId) {
          this.staleMessagesDropped += 1;
          return;
        }
        const invocationId = msg.invocationId as number;
        const pending = this.pending.get(invocationId);
        if (!pending) {
          this.staleMessagesDropped += 1;
          return;
        }
        pending.logLines += (msg.lines as unknown[]).length;
        if (pending.logLines > MAX_LOG_LINES_PER_INVOCATION) {
          this.hardTerminate(`log volume exceeded ${MAX_LOG_LINES_PER_INVOCATION} lines`);
        }
        return;
      }
      case "fatal": {
        if (msg.sessionId !== this.sessionId) return;
        this.hardTerminate(typeof msg.detail === "string" ? msg.detail : "worker reported fatal");
        return;
      }
      default:
        return; // unknown type: drop
    }
  }

  private oversized(result: unknown): boolean {
    try {
      return JSON.stringify(result).length > MAX_RESULT_BYTES;
    } catch {
      return true; // uncloneable (cyclic) — structured error path
    }
  }
}

/**
 * The Worker-realm bootstrap (stringified into the real Worker by the
 * embedding layer, or imported by a Worker double). Executes one
 * invoke at a time against the registered handler table and answers
 * with `result` messages; never touches parent-realm globals.
 */
export function workerBootstrapSource(): string {
  return `
// Velqu browser handler Worker (BWASM-R-004) — protocol v${WORKER_PROTOCOL_VERSION}
// Isolation honesty: a same-origin Worker is NOT a hostile-code sandbox.
let handlers = null; // HandlerTable from defineBrowserHandlers
// BWASM-Q-007 (D5): the page registers its C-001 capability graph before
// the first invoke; handlers reach declared capabilities through
// ctx.native.<grant> exactly as the native runtime's prelude exposes
// them (grant names: timer, console, crypto, fetch, abort, text, url).
let nativeCapabilities = null;
self.velquRegisterNativeCapabilities = (caps) => { nativeCapabilities = caps; };
self.onmessage = (event) => {
  const msg = event.data;
  if (!msg || msg.v !== ${WORKER_PROTOCOL_VERSION}) return;
  if (msg.type === "invoke") {
    if (msg.sessionId !== self.VELQU_SESSION_ID) return;
    Promise.resolve()
      .then(() => handlers.invoke(msg.plan.handlerKey, toContext(msg.plan)))
      .then(
        (result) => post({ type: "result", sessionId: msg.sessionId, invocationId: msg.invocationId, correlationId: msg.correlationId, result }),
        (err) => post({
          type: "result", sessionId: msg.sessionId, invocationId: msg.invocationId, correlationId: msg.correlationId,
          result: { kind: "problem", problemId: "internal", detail: redact(String(err && err.stack || err)) },
        }),
      );
  }
};
function post(message) { self.postMessage({ v: ${WORKER_PROTOCOL_VERSION}, ...message }); }
function toContext(plan) {
  return {
    routeId: plan.routeId, handlerKey: plan.handlerKey,
    params: plan.params ?? null, query: plan.query ?? null,
    headers: plan.headers ?? null, body: plan.body ?? null,
    bodyText: plan.bodyText ?? null, deadlineMs: plan.deadlineMs,
    // Only grants the route actually declares are exposed — no ambient
    // authority (ADR-0038 §5). The capability objects themselves are the
    // page-registered, policy-bounded implementations.
    native: planCapabilities(plan.capabilities ?? []),
  };
}
function planCapabilities(declared) {
  if (!nativeCapabilities || !declared || declared.length === 0) return {};
  const view = {};
  for (const grant of declared) {
    if (grant === "timer" && nativeCapabilities.timers) {
      view.timer = { delay: (ms, signal) => nativeCapabilities.timers.delay(ms, signal) };
    } else if (nativeCapabilities[grant] !== undefined) {
      view[grant] = nativeCapabilities[grant];
    }
  }
  return Object.freeze(view);
}
function redact(stack) {
  // Strip host-absolute path fragments (ADR-0038 §5 redaction).
  return String(stack).replace(/(?:\\/|\\\\)[^\\s()]*[/\\\\]/g, "");
}
self.velquRegisterHandlers = (table, sessionId) => {
  handlers = table; self.VELQU_SESSION_ID = sessionId;
  post({ type: "ready", sessionId, handlerKeys: table.keys });
};
`;
}
