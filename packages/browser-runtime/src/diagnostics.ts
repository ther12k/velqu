/**
 * BWASM-Q-004 — Browser-WASM observability and developer diagnostics.
 *
 * Provides:
 * - Structured lifecycle events across 12 frozen stages:
 *   load, verify, instantiate, route, validate, invoke, capability,
 *   persist, cache, update, cancel, fail.
 * - Stable, snapshot-tested diagnostic codes (DIAG_*).
 * - Request correlation IDs across Service Worker, runtime, Worker host,
 *   and capabilities.
 * - Bounded developer event stream (ring buffer) preventing log-flood attacks.
 * - Sensitive-data redaction for credentials, authorization, cookies, and tokens.
 * - Source location sanitization and error mapping.
 * - Trace export and Inspector panel adapter for dev preview UI.
 */

import { redactSensitiveText } from "./capabilities";
import { sanitizeSourceLocation } from "./handler-bundle";

// ---------------------------------------------------------------------------
// Lifecycle stages and levels
// ---------------------------------------------------------------------------

export type DiagnosticLifecycleStage =
  | "load"
  | "verify"
  | "instantiate"
  | "route"
  | "validate"
  | "invoke"
  | "capability"
  | "persist"
  | "cache"
  | "update"
  | "cancel"
  | "fail";

export type DiagnosticLevel = "debug" | "info" | "warn" | "error";

const LEVEL_SEVERITY: Record<DiagnosticLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

// ---------------------------------------------------------------------------
// Stable Diagnostic Codes (Step 2 & Acceptance Criteria 1 & 5)
// ---------------------------------------------------------------------------

export const DIAGNOSTIC_CODES = {
  // Integrity & Loader
  LOAD_MANIFEST_OK: "DIAG_LOAD_MANIFEST_OK",
  LOAD_MANIFEST_FAIL: "DIAG_LOAD_MANIFEST_FAIL",
  VERIFY_INTEGRITY_OK: "DIAG_VERIFY_INTEGRITY_OK",
  VERIFY_INTEGRITY_FAIL: "DIAG_VERIFY_INTEGRITY_FAIL",

  // Compatibility
  COMPAT_KERNEL_ABI_OK: "DIAG_COMPAT_KERNEL_ABI_OK",
  COMPAT_KERNEL_ABI_MISMATCH: "DIAG_COMPAT_KERNEL_ABI_MISMATCH",
  COMPAT_HANDLER_ABI_MISMATCH: "DIAG_COMPAT_HANDLER_ABI_MISMATCH",

  // Lifecycle & Kernel
  LIFECYCLE_INSTANTIATING: "DIAG_LIFECYCLE_INSTANTIATING",
  LIFECYCLE_READY: "DIAG_LIFECYCLE_READY",
  LIFECYCLE_DISPOSED: "DIAG_LIFECYCLE_DISPOSED",

  // Routing
  ROUTE_MATCHED: "DIAG_ROUTE_MATCHED",
  ROUTE_NOT_FOUND: "DIAG_ROUTE_NOT_FOUND",
  ROUTE_METHOD_NOT_ALLOWED: "DIAG_ROUTE_METHOD_NOT_ALLOWED",

  // Validation / Schema
  VALIDATE_PASSED: "DIAG_VALIDATE_PASSED",
  VALIDATE_FAILED: "DIAG_VALIDATE_FAILED",

  // Capability
  CAPABILITY_INVOKED: "DIAG_CAPABILITY_INVOKED",
  CAPABILITY_DENIED: "DIAG_CAPABILITY_DENIED",
  CAPABILITY_DEPLOYMENT_REQUIRED: "DIAG_CAPABILITY_DEPLOYMENT_REQUIRED",

  // Invocation / Handler
  INVOKE_START: "DIAG_INVOKE_START",
  INVOKE_SUCCESS: "DIAG_INVOKE_SUCCESS",
  INVOKE_TIMEOUT: "DIAG_INVOKE_TIMEOUT",
  INVOKE_CANCEL: "DIAG_INVOKE_CANCEL",
  INVOKE_FAILED: "DIAG_INVOKE_FAILED",

  // Persistence (KV)
  PERSIST_ACCESS: "DIAG_PERSIST_ACCESS",
  PERSIST_ERROR: "DIAG_PERSIST_ERROR",
  PERSIST_QUOTA_EXCEEDED: "DIAG_PERSIST_QUOTA_EXCEEDED",
  PERSIST_MIGRATION_REQUIRED: "DIAG_PERSIST_MIGRATION_REQUIRED",

  // Cache & Service Worker
  CACHE_HIT: "DIAG_CACHE_HIT",
  CACHE_MISS: "DIAG_CACHE_MISS",
  CACHE_STORED: "DIAG_CACHE_STORED",
  SW_REGISTERED: "DIAG_SW_REGISTERED",
  SW_UPDATE_AVAILABLE: "DIAG_SW_UPDATE_AVAILABLE",
  SW_UPDATE_APPLIED: "DIAG_SW_UPDATE_APPLIED",

  // Failure
  FAIL_INTERNAL: "DIAG_FAIL_INTERNAL",
  FAIL_REDACTED: "DIAG_FAIL_REDACTED",
} as const;

export type DiagnosticCode = typeof DIAGNOSTIC_CODES[keyof typeof DIAGNOSTIC_CODES];

// ---------------------------------------------------------------------------
// Event shape
// ---------------------------------------------------------------------------

export interface DiagnosticEvent {
  readonly id: string;
  readonly timestamp: number;
  readonly stage: DiagnosticLifecycleStage;
  readonly code: DiagnosticCode;
  readonly level: DiagnosticLevel;
  readonly correlationId?: string;
  readonly routeId?: string | number;
  readonly sourceLocation?: string;
  readonly detail?: string;
  readonly metadata?: Readonly<Record<string, unknown>>;
}

// ---------------------------------------------------------------------------
// Correlation ID helper (Step 2)
// ---------------------------------------------------------------------------

let correlationCounter = 0;

/**
 * Generate a unique, structured correlation ID for cross-boundary tracing.
 */
export function generateCorrelationId(prefix = "cr"): string {
  const ts = Date.now().toString(36);
  const count = (++correlationCounter).toString(36);
  const rand = Math.random().toString(36).slice(2, 8);
  return `${prefix}_${ts}_${count}_${rand}`;
}

/**
 * Extract an incoming correlation ID from headers (x-correlation-id or x-request-id)
 * or generate a fresh one.
 */
export function extractOrGenerateCorrelationId(
  headers?: Headers | Record<string, string> | null,
): string {
  if (headers) {
    if (typeof (headers as Headers).get === "function") {
      const h = headers as Headers;
      const found = h.get("x-correlation-id") || h.get("x-request-id");
      if (found) return found;
    } else {
      const rec = headers as Record<string, string>;
      const found =
        rec["x-correlation-id"] ||
        rec["x-request-id"] ||
        rec["X-Correlation-Id"] ||
        rec["X-Request-Id"];
      if (found) return found;
    }
  }
  return generateCorrelationId();
}

// ---------------------------------------------------------------------------
// Redaction (Step 5 & Acceptance Criterion 3)
// ---------------------------------------------------------------------------

const SENSITIVE_KEY_PATTERN = /^(authorization|cookie|set-cookie|password|passwd|secret|token|api[_-]?key|bearer|credential|credentials|privkey|private[_-]?key)$/i;
const MAX_METADATA_DEPTH = 3;
const MAX_METADATA_STRING_LEN = 512;

/**
 * Deeply redact sensitive keys, authorization headers, cookies, and tokens
 * from diagnostic metadata payloads.
 */
export function redactDiagnosticMetadata(
  value: unknown,
  depth = 0,
): unknown {
  if (value === null || value === undefined) return value;
  if (typeof value === "string") {
    const redacted = redactSensitiveText(value);
    return redacted.length > MAX_METADATA_STRING_LEN
      ? redacted.slice(0, MAX_METADATA_STRING_LEN) + "...[TRUNCATED]"
      : redacted;
  }
  if (typeof value === "number" || typeof value === "boolean") {
    return value;
  }
  if (depth >= MAX_METADATA_DEPTH) {
    return "[MAX_DEPTH]";
  }
  if (Array.isArray(value)) {
    return value.map((item) => redactDiagnosticMetadata(item, depth + 1));
  }
  if (typeof value === "object") {
    const out: Record<string, unknown> = {};
    for (const [k, v] of Object.entries(value as Record<string, unknown>)) {
      if (SENSITIVE_KEY_PATTERN.test(k)) {
        out[k] = "[REDACTED]";
      } else {
        out[k] = redactDiagnosticMetadata(v, depth + 1);
      }
    }
    return out;
  }
  return String(value);
}

// ---------------------------------------------------------------------------
// Error Mapping (Step 4 & Acceptance Criterion 1)
// ---------------------------------------------------------------------------

export interface MappedErrorDiagnostic {
  readonly stage: DiagnosticLifecycleStage;
  readonly code: DiagnosticCode;
  readonly detail: string;
  readonly sourceLocation?: string;
  readonly routeId?: string | number;
}

/**
 * Maps arbitrary runtime errors or problems back to a stable diagnostic code,
 * lifecycle stage, sanitized source location, and redacted detail.
 */
export function mapDiagnosticError(
  error: unknown,
  context?: { routeId?: string | number; sourceLocation?: string },
): MappedErrorDiagnostic {
  let detail = "unknown error";
  let sourceLocation = context?.sourceLocation;
  let code: DiagnosticCode = DIAGNOSTIC_CODES.FAIL_INTERNAL;
  let stage: DiagnosticLifecycleStage = "fail";

  if (error instanceof Error) {
    detail = error.message;
    // Check for specific error types by message or name
    if (error.name === "AbortError" || /aborted/i.test(error.message)) {
      code = DIAGNOSTIC_CODES.INVOKE_CANCEL;
      stage = "cancel";
    } else if (/timeout|deadline/i.test(error.message)) {
      code = DIAGNOSTIC_CODES.INVOKE_TIMEOUT;
      stage = "invoke";
    } else if (/schema|validation|malformed/i.test(error.message)) {
      code = DIAGNOSTIC_CODES.VALIDATE_FAILED;
      stage = "validate";
    } else if (/capability|deployment-required/i.test(error.message)) {
      code = error.message.includes("deployment-required")
        ? DIAGNOSTIC_CODES.CAPABILITY_DEPLOYMENT_REQUIRED
        : DIAGNOSTIC_CODES.CAPABILITY_DENIED;
      stage = "capability";
    } else if (/quota/i.test(error.message)) {
      code = DIAGNOSTIC_CODES.PERSIST_QUOTA_EXCEEDED;
      stage = "persist";
    } else if (/migration/i.test(error.message)) {
      code = DIAGNOSTIC_CODES.PERSIST_MIGRATION_REQUIRED;
      stage = "persist";
    } else if (/indexeddb|storage/i.test(error.message)) {
      code = DIAGNOSTIC_CODES.PERSIST_ERROR;
      stage = "persist";
    } else if (/integrity|sha256|checksum|mismatch/i.test(error.message)) {
      code = DIAGNOSTIC_CODES.VERIFY_INTEGRITY_FAIL;
      stage = "verify";
    } else if (/abi/i.test(error.message)) {
      code = DIAGNOSTIC_CODES.COMPAT_KERNEL_ABI_MISMATCH;
      stage = "instantiate";
    } else if (/handler/i.test(error.message)) {
      code = DIAGNOSTIC_CODES.INVOKE_FAILED;
      stage = "invoke";
    }

    // Extract source location from stack if not provided
    if (!sourceLocation && error.stack) {
      const lines = error.stack.split("\n");
      for (const line of lines.slice(1)) {
        const match = /(?:at\s+)?(?:.+?\s+\()?(https?:\/\/[^\s)]+|\/[^\s)]+)/.exec(line);
        if (match && match[1]) {
          sourceLocation = sanitizeSourceLocation(match[1]);
          break;
        }
      }
    }
  } else if (typeof error === "object" && error !== null) {
    const obj = error as Record<string, unknown>;
    detail = typeof obj.detail === "string" ? obj.detail : JSON.stringify(obj);
    if (obj.problemId === "validation" || obj.status === 422) {
      code = DIAGNOSTIC_CODES.VALIDATE_FAILED;
      stage = "validate";
    } else if (obj.problemId === "not_found" || obj.status === 404) {
      code = DIAGNOSTIC_CODES.ROUTE_NOT_FOUND;
      stage = "route";
    } else if (obj.problemId === "method_not_allowed" || obj.status === 405) {
      code = DIAGNOSTIC_CODES.ROUTE_METHOD_NOT_ALLOWED;
      stage = "route";
    } else if (obj.problemId === "timeout" || obj.status === 504) {
      code = DIAGNOSTIC_CODES.INVOKE_TIMEOUT;
      stage = "invoke";
    } else if (obj.problemId === "deployment_required") {
      code = DIAGNOSTIC_CODES.CAPABILITY_DEPLOYMENT_REQUIRED;
      stage = "capability";
    }
  } else {
    detail = String(error);
  }

  return {
    stage,
    code,
    detail: redactSensitiveText(detail),
    sourceLocation,
    routeId: context?.routeId,
  };
}

// ---------------------------------------------------------------------------
// Event Stream (Step 3 & Acceptance Criteria 3 & 4)
// ---------------------------------------------------------------------------

export const DEFAULT_DIAGNOSTIC_CAPACITY = 256;
export const MAX_DIAGNOSTIC_CAPACITY = 2048;

export interface DiagnosticStreamOptions {
  /** Ring-buffer capacity (default 256, max 2048). */
  readonly capacity?: number;
  /** Whether logging is active (default true; set false for production/silent). */
  readonly enabled?: boolean;
  /** Minimum level to record (default "info"). */
  readonly minLevel?: DiagnosticLevel;
  /** Whether to redact metadata and detail automatically (default true). */
  readonly redact?: boolean;
}

export interface DiagnosticTraceExport {
  readonly schemaVersion: 1;
  readonly exportedAt: number;
  readonly correlationId?: string;
  readonly totalEvents: number;
  readonly events: ReadonlyArray<DiagnosticEvent>;
  readonly summary: {
    readonly byStage: Record<string, number>;
    readonly byLevel: Record<string, number>;
    readonly failures: number;
  };
}

export class DiagnosticStream {
  private readonly capacity: number;
  private readonly enabled: boolean;
  private readonly minLevel: DiagnosticLevel;
  private readonly redact: boolean;
  private readonly ring: DiagnosticEvent[] = [];
  private readonly listeners = new Set<(event: DiagnosticEvent) => void>();
  private nextEventId = 1;
  private totalRecordedCount = 0;
  private droppedFloodCount = 0;

  constructor(options: DiagnosticStreamOptions = {}) {
    this.capacity = Math.min(
      Math.max(1, options.capacity ?? DEFAULT_DIAGNOSTIC_CAPACITY),
      MAX_DIAGNOSTIC_CAPACITY,
    );
    this.enabled = options.enabled ?? true;
    this.minLevel = options.minLevel ?? "info";
    this.redact = options.redact ?? true;
  }

  get isEnabled(): boolean {
    return this.enabled;
  }

  record(eventInput: {
    stage: DiagnosticLifecycleStage;
    code: DiagnosticCode;
    level: DiagnosticLevel;
    correlationId?: string;
    routeId?: string | number;
    sourceLocation?: string;
    detail?: string;
    metadata?: Record<string, unknown>;
  }): void {
    if (!this.enabled) return;
    if (LEVEL_SEVERITY[eventInput.level] < LEVEL_SEVERITY[this.minLevel]) return;

    this.totalRecordedCount++;
    const id = `ev_${this.nextEventId++}`;
    const timestamp = Date.now();
    const detail = eventInput.detail
      ? (this.redact ? redactSensitiveText(eventInput.detail) : eventInput.detail)
      : undefined;
    const metadata = eventInput.metadata
      ? ((this.redact
          ? redactDiagnosticMetadata(eventInput.metadata)
          : eventInput.metadata) as Record<string, unknown>)
      : undefined;

    const event: DiagnosticEvent = {
      id,
      timestamp,
      stage: eventInput.stage,
      code: eventInput.code,
      level: eventInput.level,
      ...(eventInput.correlationId ? { correlationId: eventInput.correlationId } : {}),
      ...(eventInput.routeId !== undefined ? { routeId: eventInput.routeId } : {}),
      ...(eventInput.sourceLocation ? { sourceLocation: eventInput.sourceLocation } : {}),
      ...(detail ? { detail } : {}),
      ...(metadata ? { metadata } : {}),
    };

    if (this.ring.length >= this.capacity) {
      this.ring.shift();
      this.droppedFloodCount++;
    }
    this.ring.push(event);

    for (const listener of this.listeners) {
      try {
        listener(event);
      } catch {
        // Observers must not break runtime execution
      }
    }
  }

  events(filter?: {
    stage?: DiagnosticLifecycleStage;
    correlationId?: string;
    minLevel?: DiagnosticLevel;
  }): ReadonlyArray<DiagnosticEvent> {
    return this.ring.filter((ev) => {
      if (filter?.stage && ev.stage !== filter.stage) return false;
      if (filter?.correlationId && ev.correlationId !== filter.correlationId) return false;
      if (filter?.minLevel && LEVEL_SEVERITY[ev.level] < LEVEL_SEVERITY[filter.minLevel]) return false;
      return true;
    });
  }

  subscribe(listener: (event: DiagnosticEvent) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  exportTrace(correlationId?: string): DiagnosticTraceExport {
    const matched = correlationId
      ? this.ring.filter((e) => e.correlationId === correlationId)
      : [...this.ring];

    const byStage: Record<string, number> = {};
    const byLevel: Record<string, number> = {};
    let failures = 0;

    for (const ev of matched) {
      byStage[ev.stage] = (byStage[ev.stage] ?? 0) + 1;
      byLevel[ev.level] = (byLevel[ev.level] ?? 0) + 1;
      if (ev.level === "error" || ev.stage === "fail") failures++;
    }

    return {
      schemaVersion: 1,
      exportedAt: Date.now(),
      ...(correlationId ? { correlationId } : {}),
      totalEvents: matched.length,
      events: matched,
      summary: {
        byStage,
        byLevel,
        failures,
      },
    };
  }

  stats(): { totalRecorded: number; droppedFlood: number; currentCount: number; capacity: number } {
    return {
      totalRecorded: this.totalRecordedCount,
      droppedFlood: this.droppedFloodCount,
      currentCount: this.ring.length,
      capacity: this.capacity,
    };
  }

  clear(): void {
    this.ring.length = 0;
  }
}

export function createDiagnosticStream(options?: DiagnosticStreamOptions): DiagnosticStream {
  return new DiagnosticStream(options);
}

// ---------------------------------------------------------------------------
// Inspector Panel Adapter (Step 3 & Acceptance Criteria 4 & 5)
// ---------------------------------------------------------------------------

export interface InspectorSummary {
  readonly totalEvents: number;
  readonly droppedEvents: number;
  readonly failures: number;
  readonly activeCorrelations: ReadonlyArray<string>;
  readonly stageBreakdown: Readonly<Record<string, number>>;
  readonly levelBreakdown: Readonly<Record<string, number>>;
  readonly recentErrors: ReadonlyArray<DiagnosticEvent>;
}

export class InspectorPanelAdapter {
  constructor(private readonly stream: DiagnosticStream) {}

  getSummary(): InspectorSummary {
    const events = this.stream.events();
    const stats = this.stream.stats();
    const stageBreakdown: Record<string, number> = {};
    const levelBreakdown: Record<string, number> = {};
    const correlations = new Set<string>();
    const recentErrors: DiagnosticEvent[] = [];

    for (const ev of events) {
      stageBreakdown[ev.stage] = (stageBreakdown[ev.stage] ?? 0) + 1;
      levelBreakdown[ev.level] = (levelBreakdown[ev.level] ?? 0) + 1;
      if (ev.correlationId) correlations.add(ev.correlationId);
      if (ev.level === "error" || ev.stage === "fail") {
        recentErrors.push(ev);
      }
    }

    return {
      totalEvents: stats.totalRecorded,
      droppedEvents: stats.droppedFlood,
      failures: recentErrors.length,
      activeCorrelations: [...correlations].slice(-10),
      stageBreakdown,
      levelBreakdown,
      recentErrors: recentErrors.slice(-5),
    };
  }

  renderTextReport(correlationId?: string): string {
    const trace = this.stream.exportTrace(correlationId);
    const header = correlationId
      ? `=== Velqu Diagnostic Trace: ${correlationId} (${trace.totalEvents} events) ===`
      : `=== Velqu Diagnostic Stream (${trace.totalEvents} events) ===`;

    const lines = [header];
    for (const ev of trace.events) {
      const time = new Date(ev.timestamp).toISOString();
      const lvl = ev.level.toUpperCase().padEnd(5);
      const stage = `[${ev.stage}]`.padEnd(13);
      const corr = ev.correlationId ? ` (${ev.correlationId})` : "";
      const loc = ev.sourceLocation ? ` @ ${ev.sourceLocation}` : "";
      const route = ev.routeId !== undefined ? ` route=${ev.routeId}` : "";
      const detail = ev.detail ? `: ${ev.detail}` : "";
      lines.push(`${time} ${lvl} ${stage} ${ev.code}${corr}${route}${loc}${detail}`);
    }
    lines.push(
      `--- Summary: Failures: ${trace.summary.failures} | Stages: ${JSON.stringify(trace.summary.byStage)} ---`,
    );
    return lines.join("\n");
  }

  renderHtml(correlationId?: string): string {
    const trace = this.stream.exportTrace(correlationId);
    const escape = (s: string): string =>
      s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");

    const rows = trace.events
      .map((ev) => {
        const time = new Date(ev.timestamp).toISOString().slice(11, 23);
        const lvlClass = `velqu-lvl-${ev.level}`;
        const detail = ev.detail ? escape(ev.detail) : "";
        const corr = ev.correlationId ? `<span class="velqu-corr">${escape(ev.correlationId)}</span>` : "";
        return `<tr><td>${time}</td><td class="${lvlClass}">${ev.level.toUpperCase()}</td><td>${ev.stage}</td><td><code>${ev.code}</code></td><td>${corr} ${detail}</td></tr>`;
      })
      .join("\n");

    return `
<div class="velqu-diagnostics-panel">
  <div class="velqu-diag-header">
    <h3>Velqu Browser-WASM Diagnostics</h3>
    <span class="velqu-diag-count">${trace.totalEvents} events (${trace.summary.failures} errors)</span>
  </div>
  <table class="velqu-diag-table">
    <thead>
      <tr><th>Time</th><th>Level</th><th>Stage</th><th>Code</th><th>Detail</th></tr>
    </thead>
    <tbody>
      ${rows || "<tr><td colspan=\"5\">No events recorded</td></tr>"}
    </tbody>
  </table>
</div>`.trim();
  }

  exportJson(correlationId?: string): string {
    return JSON.stringify(this.stream.exportTrace(correlationId), null, 2);
  }
}

export function createInspectorAdapter(stream: DiagnosticStream): InspectorPanelAdapter {
  return new InspectorPanelAdapter(stream);
}
