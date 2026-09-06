/**
 * BWASM-R-003 — handler-bundle contract tests: fail-closed negatives,
 * deterministic emission, sanitized locations.
 */
import { describe, it, expect } from "bun:test";
import {
  defineBrowserHandlers,
  emitHandlerBundleMetadata,
  sanitizeSourceLocation,
  HANDLER_ABI_VERSION,
  HandlerBundleError,
} from "../src/handler-bundle";
import type {
  BrowserHandlerRegistration,
  HandlerContext,
  PackHandlerExpectation,
} from "../src/handler-bundle";

const ctx: HandlerContext = {
  routeId: 0,
  handlerKey: "health.live",
  params: null,
  query: null,
  headers: null,
  body: null,
  bodyText: null,
  deadlineMs: 5000,
};

function reg(handlerKey: string, statuses: number[] = [200]): BrowserHandlerRegistration {
  return {
    handlerKey,
    statuses,
    invoke: () => ({ kind: "response" as const, status: statuses[0] ?? 200, body: {} }),
  };
}

const expectation: PackHandlerExpectation = {
  requiredHandlerKeys: ["health.live", "hello.get"],
  declaredStatuses: { "health.live": [200], "hello.get": [200, 404] },
};

describe("R-003 handler-bundle contract", () => {
  it("valid bundles register, invoke, and expose the ABI version", async () => {
    const table = defineBrowserHandlers(
      [reg("health.live"), reg("hello.get", [200, 404])],
      expectation,
    );
    expect(table.abiVersion).toBe(HANDLER_ABI_VERSION);
    expect(table.keys).toEqual(["health.live", "hello.get"]);
    const result = await table.invoke("health.live", ctx);
    expect(result.kind).toBe("response");
  });

  it("unknown ABI versions fail closed with an actionable diagnostic", () => {
    try {
      defineBrowserHandlers([reg("health.live"), reg("hello.get")], {
        ...expectation,
        handlerAbiVersion: 7,
      });
      throw new Error("unreachable");
    } catch (e) {
      const err = e as HandlerBundleError;
      expect(err.code).toBe("UNKNOWN_ABI_VERSION");
      expect(err.message).toContain("ABI 7");
      expect(err.message).toContain("rebuild");
    }
  });

  it("duplicate handler keys are rejected before execution", () => {
    try {
      defineBrowserHandlers(
        [reg("health.live"), reg("health.live"), reg("hello.get")],
        expectation,
      );
      throw new Error("unreachable");
    } catch (e) {
      expect((e as HandlerBundleError).code).toBe("DUPLICATE_HANDLER_KEY");
    }
  });

  it("missing pack-declared handlers are rejected naming every gap", () => {
    try {
      defineBrowserHandlers([reg("health.live")], expectation);
      throw new Error("unreachable");
    } catch (e) {
      const err = e as HandlerBundleError;
      expect(err.code).toBe("MISSING_HANDLER");
      expect(err.message).toContain("hello.get");
    }
  });

  it("handlers the pack does not declare are rejected (no silent routes)", () => {
    try {
      defineBrowserHandlers(
        [reg("health.live"), reg("hello.get"), reg("rogue.handler")],
        expectation,
      );
      throw new Error("unreachable");
    } catch (e) {
      const err = e as HandlerBundleError;
      expect(err.code).toBe("INVALID_REGISTRATION");
      expect(err.message).toContain("rogue.handler");
    }
  });

  it("undeclared statuses are rejected at registration time", () => {
    try {
      defineBrowserHandlers(
        [reg("health.live", [200, 418]), reg("hello.get")],
        expectation,
      );
      throw new Error("unreachable");
    } catch (e) {
      const err = e as HandlerBundleError;
      expect(err.code).toBe("UNDECLARED_STATUS");
      expect(err.message).toContain("418");
    }
  });

  it("malformed registrations are rejected (no invoke function)", () => {
    try {
      defineBrowserHandlers(
        [{ handlerKey: "health.live", statuses: [200], invoke: undefined as never }, reg("hello.get")],
        expectation,
      );
      throw new Error("unreachable");
    } catch (e) {
      expect((e as HandlerBundleError).code).toBe("INVALID_REGISTRATION");
    }
  });
});

describe("R-003 deterministic emission", () => {
  it("metadata is byte-stable across shuffled input (golden fixture)", () => {
    const a = emitHandlerBundleMetadata(
      [reg("health.live", [200]), reg("hello.get", [404, 200])],
    );
    const b = emitHandlerBundleMetadata(
      [reg("hello.get", [200, 404]), reg("health.live", [200])],
    );
    expect(a).toBe(b);
    // Golden fixture (byte-exact: stable field order, sorted handlers
    // and statuses, 2-space indent, trailing newline):
    expect(a).toBe(
      "{\n" +
        '  "handlerAbiVersion": 1,\n' +
        '  "handlers": [\n' +
        "    {\n" +
        '      "handlerKey": "health.live",\n' +
        "      \"statuses\": [\n" +
        "        200\n" +
        "      ]\n" +
        "    },\n" +
        "    {\n" +
        '      "handlerKey": "hello.get",\n' +
        "      \"statuses\": [\n" +
        "        200,\n" +
        "        404\n" +
        "      ]\n" +
        "    }\n" +
        "  ]\n" +
        "}\n",
    );
  });

  it("source fields round-trip deterministically", () => {
    const a = emitHandlerBundleMetadata([
      { ...reg("health.live"), source: "src/modules/health/handlers.ts" },
    ]);
    const b = emitHandlerBundleMetadata([
      { ...reg("health.live"), source: "src/modules/health/handlers.ts" },
    ]);
    expect(a).toBe(b);
    expect(a).toContain('"source": "src/modules/health/handlers.ts"');
  });
});

describe("R-003 source-location sanitization", () => {
  it("strips host-absolute prefixes, keeps project-relative paths", () => {
    expect(
      sanitizeSourceLocation("/home/someuser/secret/project/src/modules/a.ts:12:3"),
    ).toBe("src/modules/a.ts:12:3");
    expect(
      sanitizeSourceLocation("C:\\\\Users\\\\bob\\\\Private\\\\app\\\\src\\\\h.ts:1:1"),
    ).toBe("src/h.ts:1:1");
  });

  it("already-relative paths pass through", () => {
    expect(sanitizeSourceLocation("src/modules/a.ts:5:9")).toBe("src/modules/a.ts:5:9");
  });
});
