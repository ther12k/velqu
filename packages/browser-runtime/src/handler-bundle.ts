/**
 * BWASM-R-003 — the browser handler-bundle contract.
 *
 * The deterministic contract connecting compiled application handlers
 * to the browser runtime:
 *
 * - handlers register through this NARROW runtime API
 *   (`defineBrowserHandlers`) — no ambient globals (an undeclared
 *   route or status cannot be registered silently);
 * - a bundle is validated against the pack's function manifest BEFORE
 *   execution (duplicates, missing IDs, unknown ABI versions all fail
 *   closed with actionable diagnostics);
 * - bundle metadata is emitted deterministically (byte-stable under a
 *   normalized environment) with sanitized source locations;
 * - the handler ABI is versioned independently of package versions.
 *
 * The compiler-side emission from application source (the
 * `browser-wasm` build target) is BWASM-B-001; this module is the
 * contract and its reference emitter.
 */

// ---------------------------------------------------------------------------
// ABI identity
// ---------------------------------------------------------------------------

/** Handler-bundle ABI version — independent of package versions. */
export const HANDLER_ABI_VERSION = 1;

export class HandlerBundleError extends Error {
  readonly code:
    | "UNKNOWN_ABI_VERSION"
    | "DUPLICATE_HANDLER_KEY"
    | "MISSING_HANDLER"
    | "UNDECLARED_STATUS"
    | "INVALID_REGISTRATION";
  constructor(code: HandlerBundleError["code"], message: string) {
    super(`[@velqu/browser-runtime:handler-bundle:${code}] ${message}`);
    this.name = "HandlerBundleError";
    this.code = code;
  }
}

// ---------------------------------------------------------------------------
// Invocation contract (kernel plan → handler → kernel completion)
// ---------------------------------------------------------------------------

/**
 * Handler invocation context — the validated, kernel-produced plan data
 * (ADR-0037 §3): no raw request handle, no ambient authority.
 * Capability calls cross the runtime bridge, never ambient APIs
 * (trusted-mode convention, ADR-0038 §5).
 */
export interface HandlerContext {
  readonly routeId: number;
  readonly handlerKey: string;
  readonly params: Readonly<Record<string, unknown>> | null;
  readonly query: Readonly<Record<string, unknown>> | null;
  readonly headers: Readonly<Record<string, unknown>> | null;
  readonly body: unknown;
  /** Raw body text when the route declares no body schema. */
  readonly bodyText: string | null;
  readonly deadlineMs: number;
}

/** A handler result: a DECLARED status with a body, or a typed problem. */
export type HandlerResult =
  | {
    readonly kind: "response";
    readonly status: number;
    readonly headers?: ReadonlyArray<readonly [string, string]>;
    readonly body?: unknown;
  }
  | {
    readonly kind: "problem";
    readonly problemId: string;
    readonly detail?: string;
    readonly errors?: ReadonlyArray<{
      readonly path: string;
      readonly code: string;
      readonly message: string;
    }>;
  };

/** One handler registration. `statuses` MUST match the pack's declared set. */
export interface BrowserHandlerRegistration {
  readonly handlerKey: string;
  readonly statuses: ReadonlyArray<number>;
  readonly source?: string;
  readonly invoke: (ctx: HandlerContext) => Promise<HandlerResult> | HandlerResult;
}

/**
 * The pack-side expectation for a bundle, derived from the verified
 * pack (function manifest + route declarations). This is what
 * "duplicate or missing handler IDs are rejected before execution" is
 * checked against.
 */
export interface PackHandlerExpectation {
  /** Every handler key the pack's function manifest declares. */
  readonly requiredHandlerKeys: ReadonlyArray<string>;
  /** Declared response statuses per handler key (from its routes). */
  readonly declaredStatuses: Readonly<Record<string, ReadonlyArray<number>>>;
  readonly handlerAbiVersion?: number;
}

// ---------------------------------------------------------------------------
// Registration + validation (fail closed, before any execution)
// ---------------------------------------------------------------------------

export interface HandlerTable {
  readonly abiVersion: number;
  readonly keys: ReadonlyArray<string>;
  invoke(handlerKey: string, ctx: HandlerContext): Promise<HandlerResult>;
}

/**
 * The narrow registration API generated handler modules call. Validates
 * against the pack expectation BEFORE execution:
 * - unknown/missing ABI version → actionable diagnostic;
 * - duplicate handler keys → rejected;
 * - missing pack-declared handlers → rejected (the pack cannot run);
 * - extra handlers the pack does not declare → rejected (no silent
 *   undeclared routes);
 * - any status outside the declared set → rejected at registration.
 */
export function defineBrowserHandlers(
  registrations: ReadonlyArray<BrowserHandlerRegistration>,
  expectation: PackHandlerExpectation,
): HandlerTable {
  const bundleAbi = expectation.handlerAbiVersion ?? HANDLER_ABI_VERSION;
  if (bundleAbi !== HANDLER_ABI_VERSION) {
    throw new HandlerBundleError(
      "UNKNOWN_ABI_VERSION",
      `handler bundle declares ABI ${bundleAbi}; this runtime implements ${HANDLER_ABI_VERSION} — rebuild the bundle with a matching @velqu/browser-runtime`,
    );
  }

  const required = [...expectation.requiredHandlerKeys].sort();
  const byKey = new Map<string, BrowserHandlerRegistration>();
  for (const reg of registrations) {
    if (typeof reg.handlerKey !== "string" || reg.handlerKey.length === 0 || typeof reg.invoke !== "function") {
      throw new HandlerBundleError(
        "INVALID_REGISTRATION",
        `registration ${JSON.stringify(reg.handlerKey)} is not well-formed (need handlerKey + invoke)`,
      );
    }
    if (byKey.has(reg.handlerKey)) {
      throw new HandlerBundleError(
        "DUPLICATE_HANDLER_KEY",
        `handler key ${JSON.stringify(reg.handlerKey)} registered more than once`,
      );
    }
    byKey.set(reg.handlerKey, reg);
  }

  const missing = required.filter((key) => !byKey.has(key));
  if (missing.length > 0) {
    throw new HandlerBundleError(
      "MISSING_HANDLER",
      `bundle is missing ${missing.length} pack-declared handler(s): ${missing.join(", ")}`,
    );
  }
  const extra = [...byKey.keys()].filter((key) => !required.includes(key));
  if (extra.length > 0) {
    throw new HandlerBundleError(
      "INVALID_REGISTRATION",
      `bundle registers handler(s) the pack does not declare: ${extra.join(", ")} (undeclared routes cannot be registered silently)`,
    );
  }

  for (const [key, reg] of byKey) {
    const declared = expectation.declaredStatuses[key] ?? [];
    for (const status of reg.statuses) {
      if (!declared.includes(status)) {
        throw new HandlerBundleError(
          "UNDECLARED_STATUS",
          `handler ${JSON.stringify(key)} registers status ${status}; the pack declares only [${declared.join(", ")}]`,
        );
      }
    }
  }

  const table = new Map(
    [...byKey.entries()].map(([key, reg]) => [key, reg.invoke] as const),
  );
  return {
    abiVersion: HANDLER_ABI_VERSION,
    keys: required,
    async invoke(handlerKey: string, ctx: HandlerContext): Promise<HandlerResult> {
      const invoke = table.get(handlerKey);
      if (!invoke) {
        // Unreachable after validation; kept fail-closed regardless.
        throw new HandlerBundleError(
          "MISSING_HANDLER",
          `handler ${JSON.stringify(handlerKey)} is not registered`,
        );
      }
      return invoke(ctx);
    },
  };
}

// ---------------------------------------------------------------------------
// Deterministic metadata emission (golden fixtures / reproducibility)
// ---------------------------------------------------------------------------

/** Bundle metadata record — byte-stable JSON under normalized input. */
export interface HandlerBundleMetadata {
  readonly handlerAbiVersion: number;
  readonly handlers: ReadonlyArray<{
    readonly handlerKey: string;
    readonly statuses: ReadonlyArray<number>;
    readonly source?: string;
  }>;
}

/**
 * Emit the bundle metadata as deterministic JSON: handlers sorted by
 * key, statuses sorted ascending, keys in a fixed order, 2-space
 * indent, trailing newline. The same input produces byte-identical
 * output (reproducibility fixture).
 */
export function emitHandlerBundleMetadata(
  registrations: ReadonlyArray<BrowserHandlerRegistration>,
  abiVersion: number = HANDLER_ABI_VERSION,
): string {
  const meta: HandlerBundleMetadata = {
    handlerAbiVersion: abiVersion,
    handlers: [...registrations]
      .map((r) => ({
        handlerKey: r.handlerKey,
        statuses: [...r.statuses].sort((a, b) => a - b),
        ...(r.source !== undefined ? { source: r.source } : {}),
      }))
      .sort((a, b) => (a.handlerKey < b.handlerKey ? -1 : a.handlerKey > b.handlerKey ? 1 : 0)),
  };
  return JSON.stringify(meta, null, 2) + "\n";
}

/**
 * Sanitize a source location for runtime errors: keep the
 * project-relative path and line/column, strip any host-absolute
 * prefix or user-directory fragments (no private host paths cross).
 */
export function sanitizeSourceLocation(raw: string): string {
  // Collapse any leading absolute prefixes (…/project/) down to the
  // first segment that looks like a project root (src/, packages/, or
  // a bare relative path). Everything before it is host-private.
  const markers = ["/src/", "/packages/", "/examples/"];
  // Normalize separators first and collapse duplicates (Windows paths,
  // escaped slashes) so marker detection is unambiguous.
  let path = raw.replace(/\\/g, "/").replace(/\/+/g, "/");
  for (const marker of markers) {
    const idx = path.indexOf(marker);
    if (idx > 0) {
      path = path.slice(idx + 1);
      break;
    }
  }
  return path.replace(/^\/+/, "");
}
