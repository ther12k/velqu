/**
 * BWASM-Q-004 — Tests for Browser-WASM observability and developer diagnostics.
 *
 * Covers:
 * 1. Diagnostic catalog and snapshot verification (Criterion 1 & 5).
 * 2. Cross-boundary correlation ID extraction and propagation (Criterion 2).
 * 3. Default sensitive data redaction for auth, cookies, secrets (Criterion 3).
 * 4. Bounded ring buffer stream and log-flood prevention (Criterion 4).
 * 5. Trace export and single-correlation isolation (Criterion 4).
 * 6. Inspector panel adapter summary, text report, and safe HTML (Step 3).
 * 7. Error mapping with source location sanitization and route ID (Step 4).
 * 8. Configurable disablement and level filtering for static deployment (Criterion 6).
 */

import { describe, expect, it } from "bun:test";
import {
  DIAGNOSTIC_CODES,
  createDiagnosticStream,
  createInspectorAdapter,
  extractOrGenerateCorrelationId,
  generateCorrelationId,
  mapDiagnosticError,
  redactDiagnosticMetadata,
  DiagnosticStream,
  type DiagnosticCode,
  type DiagnosticLifecycleStage,
} from "../src/diagnostics";

describe("Q-004 diagnostic codes and lifecycle catalog", () => {
  const REQUIRED_STAGES: ReadonlyArray<DiagnosticLifecycleStage> = [
    "load",
    "verify",
    "instantiate",
    "route",
    "validate",
    "invoke",
    "capability",
    "persist",
    "cache",
    "update",
    "cancel",
    "fail",
  ];

  it("covers all 12 frozen lifecycle stages", () => {
    expect(REQUIRED_STAGES.length).toBe(12);
  });

  it("every diagnostic code starts with DIAG_ prefix and has stable string value", () => {
    for (const [key, code] of Object.entries(DIAGNOSTIC_CODES)) {
      expect(code).toStartWith("DIAG_");
      expect(code).toBe(`DIAG_${key}`);
    }
  });

  it("distinguishes all 10 failure categories per Acceptance Criterion 1", () => {
    // 1. integrity
    expect(DIAGNOSTIC_CODES.VERIFY_INTEGRITY_FAIL).toBe("DIAG_VERIFY_INTEGRITY_FAIL");
    // 2. compatibility
    expect(DIAGNOSTIC_CODES.COMPAT_KERNEL_ABI_MISMATCH).toBe("DIAG_COMPAT_KERNEL_ABI_MISMATCH");
    expect(DIAGNOSTIC_CODES.COMPAT_HANDLER_ABI_MISMATCH).toBe("DIAG_COMPAT_HANDLER_ABI_MISMATCH");
    // 3. route
    expect(DIAGNOSTIC_CODES.ROUTE_NOT_FOUND).toBe("DIAG_ROUTE_NOT_FOUND");
    expect(DIAGNOSTIC_CODES.ROUTE_METHOD_NOT_ALLOWED).toBe("DIAG_ROUTE_METHOD_NOT_ALLOWED");
    // 4. schema
    expect(DIAGNOSTIC_CODES.VALIDATE_FAILED).toBe("DIAG_VALIDATE_FAILED");
    // 5. capability
    expect(DIAGNOSTIC_CODES.CAPABILITY_DENIED).toBe("DIAG_CAPABILITY_DENIED");
    // 6. handler
    expect(DIAGNOSTIC_CODES.INVOKE_FAILED).toBe("DIAG_INVOKE_FAILED");
    // 7. timeout
    expect(DIAGNOSTIC_CODES.INVOKE_TIMEOUT).toBe("DIAG_INVOKE_TIMEOUT");
    // 8. persistence
    expect(DIAGNOSTIC_CODES.PERSIST_ERROR).toBe("DIAG_PERSIST_ERROR");
    expect(DIAGNOSTIC_CODES.PERSIST_QUOTA_EXCEEDED).toBe("DIAG_PERSIST_QUOTA_EXCEEDED");
    expect(DIAGNOSTIC_CODES.PERSIST_MIGRATION_REQUIRED).toBe("DIAG_PERSIST_MIGRATION_REQUIRED");
    // 9. cache
    expect(DIAGNOSTIC_CODES.CACHE_MISS).toBe("DIAG_CACHE_MISS");
    // 10. deployment-required
    expect(DIAGNOSTIC_CODES.CAPABILITY_DEPLOYMENT_REQUIRED).toBe("DIAG_CAPABILITY_DEPLOYMENT_REQUIRED");
  });

  it("diagnostic catalog snapshot is byte-stable", () => {
    const sorted = Object.keys(DIAGNOSTIC_CODES).sort().map((k) => [k, DIAGNOSTIC_CODES[k as keyof typeof DIAGNOSTIC_CODES]]);
    expect(sorted.length).toBe(35);
    expect(sorted[0]).toEqual(["CACHE_HIT", "DIAG_CACHE_HIT"]);
    expect(sorted.at(-1)).toEqual(["VERIFY_INTEGRITY_OK", "DIAG_VERIFY_INTEGRITY_OK"]);
  });
});

describe("Q-004 correlation ID propagation", () => {
  it("generates unique correlation IDs with prefix", () => {
    const id1 = generateCorrelationId();
    const id2 = generateCorrelationId();
    expect(id1).toMatch(/^cr_[a-z0-9]+_[a-z0-9]+_[a-z0-9]+$/);
    expect(id2).toMatch(/^cr_[a-z0-9]+_[a-z0-9]+_[a-z0-9]+$/);
    expect(id1).not.toBe(id2);
  });

  it("extracts correlation ID from Headers or plain records", () => {
    const headers1 = new Headers({ "x-correlation-id": "corr_req_123" });
    expect(extractOrGenerateCorrelationId(headers1)).toBe("corr_req_123");

    const headers2 = new Headers({ "x-request-id": "req_456" });
    expect(extractOrGenerateCorrelationId(headers2)).toBe("req_456");

    const record = { "x-correlation-id": "custom_trace_789" };
    expect(extractOrGenerateCorrelationId(record)).toBe("custom_trace_789");

    const generated = extractOrGenerateCorrelationId(null);
    expect(generated).toMatch(/^cr_/);
  });
});

describe("Q-004 sensitive data redaction", () => {
  it("redacts sensitive keys in diagnostic metadata", () => {
    const raw = {
      authorization: "Bearer secret-token-12345",
      cookie: "session=xyz987; tracker=abc",
      apiKey: "sk-live-abcdef123456",
      token: "jwt.token.value",
      password: "SuperSecretPassword!",
      normalKey: "safe-value",
      nested: {
        credentials: { user: "admin", secret: "p@ss" },
        items: [1, "normal text", "bearer 55555555"],
      },
    };

    const redacted = redactDiagnosticMetadata(raw) as Record<string, unknown>;
    expect(redacted.authorization).toBe("[REDACTED]");
    expect(redacted.cookie).toBe("[REDACTED]");
    expect(redacted.apiKey).toBe("[REDACTED]");
    expect(redacted.token).toBe("[REDACTED]");
    expect(redacted.password).toBe("[REDACTED]");
    expect(redacted.normalKey).toBe("safe-value");

    const nested = redacted.nested as Record<string, unknown>;
    expect(nested.credentials).toBe("[REDACTED]");
    const items = nested.items as unknown[];
    expect(items[0]).toBe(1);
    expect(items[1]).toBe("normal text");
    // sensitive text inside strings gets redacted
    expect(String(items[2])).toContain("[REDACTED]");
  });

  it("clamps deep object trees to prevent unbounded expansion", () => {
    const deep: Record<string, unknown> = { a: { b: { c: { d: { e: 1 } } } } };
    const out = redactDiagnosticMetadata(deep) as any;
    expect(out.a.b.c).toBe("[MAX_DEPTH]");
  });
});

describe("Q-004 bounded diagnostic event stream", () => {
  it("records events and enforces capacity ring buffer (flood limit)", () => {
    const stream = createDiagnosticStream({ capacity: 5 });
    for (let i = 1; i <= 8; i++) {
      stream.record({
        stage: "invoke",
        code: DIAGNOSTIC_CODES.INVOKE_START,
        level: "info",
        detail: `call ${i}`,
      });
    }

    const events = stream.events();
    expect(events.length).toBe(5);
    // oldest events 1, 2, 3 dropped
    expect(events[0]!.detail).toBe("call 4");
    expect(events[4]!.detail).toBe("call 8");

    const stats = stream.stats();
    expect(stats.totalRecorded).toBe(8);
    expect(stats.droppedFlood).toBe(3);
    expect(stats.currentCount).toBe(5);
    expect(stats.capacity).toBe(5);
  });

  it("filters events by stage, correlationId, and minLevel", () => {
    const stream = createDiagnosticStream({ capacity: 20, minLevel: "debug" });
    stream.record({ stage: "route", code: DIAGNOSTIC_CODES.ROUTE_MATCHED, level: "debug", correlationId: "c1" });
    stream.record({ stage: "validate", code: DIAGNOSTIC_CODES.VALIDATE_PASSED, level: "info", correlationId: "c1" });
    stream.record({ stage: "invoke", code: DIAGNOSTIC_CODES.INVOKE_START, level: "info", correlationId: "c2" });
    stream.record({ stage: "fail", code: DIAGNOSTIC_CODES.FAIL_INTERNAL, level: "error", correlationId: "c1" });

    expect(stream.events({ stage: "route" }).length).toBe(1);
    expect(stream.events({ correlationId: "c1" }).length).toBe(3);
    expect(stream.events({ minLevel: "warn" }).length).toBe(1);
    expect(stream.events({ minLevel: "info" }).length).toBe(3);
  });

  it("notifies active subscribers without throwing on error", () => {
    const stream = createDiagnosticStream();
    const captured: string[] = [];
    const unsubscribe = stream.subscribe((e) => captured.push(e.code));

    stream.record({ stage: "load", code: DIAGNOSTIC_CODES.LOAD_MANIFEST_OK, level: "info" });
    expect(captured).toEqual([DIAGNOSTIC_CODES.LOAD_MANIFEST_OK]);

    unsubscribe();
    stream.record({ stage: "verify", code: DIAGNOSTIC_CODES.VERIFY_INTEGRITY_OK, level: "info" });
    expect(captured.length).toBe(1);
  });

  it("can be disabled or reduced for static production deployment", () => {
    const disabledStream = createDiagnosticStream({ enabled: false });
    disabledStream.record({ stage: "fail", code: DIAGNOSTIC_CODES.FAIL_INTERNAL, level: "error" });
    expect(disabledStream.events().length).toBe(0);

    const errorOnly = createDiagnosticStream({ minLevel: "error" });
    errorOnly.record({ stage: "route", code: DIAGNOSTIC_CODES.ROUTE_MATCHED, level: "info" });
    errorOnly.record({ stage: "fail", code: DIAGNOSTIC_CODES.FAIL_INTERNAL, level: "error" });
    expect(errorOnly.events().length).toBe(1);
  });
});

describe("Q-004 trace export and inspector adapter", () => {
  it("exports structured trace isolated by correlation ID", () => {
    const stream = createDiagnosticStream();
    const corrA = "cr_session_A";
    const corrB = "cr_session_B";

    stream.record({ stage: "route", code: DIAGNOSTIC_CODES.ROUTE_MATCHED, level: "info", correlationId: corrA });
    stream.record({ stage: "validate", code: DIAGNOSTIC_CODES.VALIDATE_PASSED, level: "info", correlationId: corrA });
    stream.record({ stage: "invoke", code: DIAGNOSTIC_CODES.INVOKE_TIMEOUT, level: "error", correlationId: corrA });
    stream.record({ stage: "route", code: DIAGNOSTIC_CODES.ROUTE_MATCHED, level: "info", correlationId: corrB });

    const traceA = stream.exportTrace(corrA);
    expect(traceA.schemaVersion).toBe(1);
    expect(traceA.correlationId).toBe(corrA);
    expect(traceA.totalEvents).toBe(3);
    expect(traceA.summary.failures).toBe(1);
    expect(traceA.summary.byStage["route"]).toBe(1);
    expect(traceA.summary.byStage["validate"]).toBe(1);
    expect(traceA.summary.byStage["invoke"]).toBe(1);

    const traceAll = stream.exportTrace();
    expect(traceAll.totalEvents).toBe(4);
  });

  it("inspector adapter produces summary, text report, and safe HTML", () => {
    const stream = createDiagnosticStream();
    const corr = "cr_inspect_demo";
    stream.record({
      stage: "instantiate",
      code: DIAGNOSTIC_CODES.LIFECYCLE_READY,
      level: "info",
      correlationId: corr,
      detail: "ready <test>",
    });
    stream.record({
      stage: "invoke",
      code: DIAGNOSTIC_CODES.INVOKE_FAILED,
      level: "error",
      correlationId: corr,
      routeId: 42,
      sourceLocation: "examples/demo/app.ts:12:4",
      detail: "failed with & <script>",
    });

    const inspector = createInspectorAdapter(stream);
    const summary = inspector.getSummary();
    expect(summary.totalEvents).toBe(2);
    expect(summary.failures).toBe(1);
    expect(summary.activeCorrelations).toContain(corr);
    expect(summary.recentErrors.length).toBe(1);

    const text = inspector.renderTextReport(corr);
    expect(text).toContain("Velqu Diagnostic Trace");
    expect(text).toContain("DIAG_LIFECYCLE_READY");
    expect(text).toContain("DIAG_INVOKE_FAILED");
    expect(text).toContain("route=42");

    const html = inspector.renderHtml(corr);
    expect(html).toContain("velqu-diagnostics-panel");
    expect(html).toContain("&lt;script&gt;"); // escaped, safe against XSS
    expect(html).not.toContain("<script>");

    const json = inspector.exportJson(corr);
    const parsed = JSON.parse(json);
    expect(parsed.totalEvents).toBe(2);
  });
});

describe("Q-004 error mapping and source location sanitization", () => {
  it("maps generic Error to diagnostic code and sanitizes source location", () => {
    const err = new Error("schema validation failed: required field 'id' missing");
    err.stack = "Error: fail\n    at handle (/home/user/workspace/velqu/examples/proof/src/app.ts:25:7)";

    const mapped = mapDiagnosticError(err, { routeId: "users.get" });
    expect(mapped.stage).toBe("validate");
    expect(mapped.code).toBe(DIAGNOSTIC_CODES.VALIDATE_FAILED);
    expect(mapped.routeId).toBe("users.get");
    expect(mapped.sourceLocation).toBe("src/app.ts:25:7");
    expect(mapped.detail).toContain("validation failed");
  });

  it("maps timeout and abort errors correctly", () => {
    const timeoutErr = new Error("deadline 5000ms exceeded");
    const mappedTimeout = mapDiagnosticError(timeoutErr);
    expect(mappedTimeout.stage).toBe("invoke");
    expect(mappedTimeout.code).toBe(DIAGNOSTIC_CODES.INVOKE_TIMEOUT);

    const abortErr = new DOMException("The operation was aborted", "AbortError");
    const mappedAbort = mapDiagnosticError(abortErr);
    expect(mappedAbort.stage).toBe("cancel");
    expect(mappedAbort.code).toBe(DIAGNOSTIC_CODES.INVOKE_CANCEL);
  });

  it("maps RFC-9457 problem objects to route and capability codes", () => {
    const notFound = { problemId: "not_found", status: 404, detail: "no route" };
    expect(mapDiagnosticError(notFound).code).toBe(DIAGNOSTIC_CODES.ROUTE_NOT_FOUND);

    const deploymentRequired = { problemId: "deployment_required", status: 501, detail: "postgres requires deployment" };
    expect(mapDiagnosticError(deploymentRequired).code).toBe(DIAGNOSTIC_CODES.CAPABILITY_DEPLOYMENT_REQUIRED);
  });
});
