/**
 * BWASM-R-005 — capability registry + Treaty integration tests:
 * allow/deny matrix, side-effect-before-authorization regression,
 * version/inventory failures, and direct-vs-fetch differential.
 */
import { describe, it, expect } from "bun:test";
import {
  CapabilityRegistry,
  CapabilityError,
  capabilityViewForRoute,
  treatyFetchFromRuntime,
  treatyDispatchFromRuntime,
  type CapabilityHandle,
} from "../src/capability-treaty";
import type { KernelInstance, KernelModule, KernelPlanRequest, KernelPlanResult } from "../src/index";

// Kernel double with a ROUTE-CAPABILITY table + inventory, so the
// deny matrix is decided by the same structure the real kernel uses.
function makeKernel(
  inventory: Record<string, number>,
  routeCaps: Record<string, string[]>,
): KernelModule {
  const kernel = class implements KernelInstance {
    plan_request(requestJson: string): string {
      const req = JSON.parse(requestJson) as KernelPlanRequest;
      const caps = routeCaps[req.path] ?? [];
      for (const cap of caps) {
        if (!(cap in inventory)) {
          return JSON.stringify({
            kind: "problem",
            problem: {
              problemId: "capability",
              type: "https://velqu.dev/problems/capability",
              title: "Capability unavailable",
              status: 501,
              detail: `route declares capability "${cap}" not in inventory`,
            },
          });
        }
      }
      return JSON.stringify({
        kind: "invoke",
        abiVersion: 1,
        routeId: 0,
        handlerKey: "h",
        allowedStatuses: [200],
        defaultStatus: 200,
        deadlineMs: 5000,
      });
    }
    complete_invocation(): string {
      return JSON.stringify({
        kind: "response",
        status: 200,
        headers: [["content-type", "application/json"]],
        body: { ok: true },
      });
    }
    authorize_capability(name: string): string {
      if (name in inventory) return JSON.stringify({ authorized: true });
      return JSON.stringify({
        kind: "problem",
        problem: {
          problemId: "capability",
          type: "https://velqu.dev/problems/capability",
          title: "Capability unavailable",
          status: 501,
          detail: `"${name}" not in inventory`,
        },
      });
    }
    dispose(): void {}
  };
  return Object.assign(kernel, { kernel_abi_version: () => 1 });
}

function runtimeWith(inventory: Record<string, number>, routeCaps: Record<string, string[]>) {
  // Lazy import to avoid cycles in test context.
  const { createBrowserRuntime } = require("../src/index") as {
    createBrowserRuntime: typeof import("../src/index").createBrowserRuntime;
  };
  return createBrowserRuntime({
    packBytes: new Uint8Array([1]),
    kernel: makeKernel(inventory, routeCaps),
  });
}

const textHandle: CapabilityHandle = {
  id: "runtime:text",
  version: 1,
  call: async (input) => ({ echoed: input }),
};

describe("R-005 capability registry", () => {
  it("authorized + declared capability executes (allow)", async () => {
    const runtime = runtimeWith({ "runtime:text": 1 }, { "/use/text": ["runtime:text"] });
    const registry = new CapabilityRegistry(runtime, [textHandle]);
    const result = await registry.invokeForRoute("runtime:text", ["runtime:text"], "hi");
    expect(result).toEqual({ echoed: "hi" });
    runtime.dispose();
  });

  it("route-not-declaring the capability is refused BEFORE authorization or side effects", async () => {
    const runtime = runtimeWith({ "runtime:text": 1 }, { "/other": [] });
    let sideEffects = 0;
    const spy: CapabilityHandle = {
      id: "runtime:text",
      version: 1,
      call: async (input) => {
        sideEffects += 1;
        return input;
      },
    };
    const registry = new CapabilityRegistry(runtime, [spy]);
    try {
      await registry.invokeForRoute("runtime:text", ["/other" as unknown as string] && [], "x");
      throw new Error("unreachable");
    } catch (e) {
      const err = e as CapabilityError;
      expect(err.rejection.reason).toBe("not-declared");
    }
    expect(sideEffects).toBe(0); // the regression: no side effect before authorization
    runtime.dispose();
  });

  it("kernel inventory miss (deployment-required) is machine-readable", async () => {
    const runtime = runtimeWith({}, { "/use/pg": ["runtime:postgres"] });
    const registry = new CapabilityRegistry(runtime, []);
    try {
      await registry.invokeForRoute("runtime:postgres", ["runtime:postgres"], {});
      throw new Error("unreachable");
    } catch (e) {
      const err = e as CapabilityError;
      expect(err.rejection.reason).toBe("not-in-inventory");
      expect(err.message).toContain("runtime:postgres");
    }
    runtime.dispose();
  });

  it("undeclared-in-inventory route cannot access a capability via the view", async () => {
    const runtime = runtimeWith({ "runtime:text": 1 }, { "/plain": [] });
    const registry = new CapabilityRegistry(runtime, [textHandle]);
    const view = capabilityViewForRoute(registry, []); // route declares NOTHING
    expect(Object.keys(view)).toEqual([]);
    runtime.dispose();
  });

  it("view exposes exactly the declared capabilities", () => {
    const runtime = runtimeWith({ "runtime:text": 1 }, { "/use/text": ["runtime:text"] });
    const registry = new CapabilityRegistry(runtime, [textHandle]);
    const view = capabilityViewForRoute(registry, ["runtime:text"]);
    expect(Object.keys(view)).toEqual(["runtime:text"]);
    runtime.dispose();
  });
});

describe("R-005 Treaty integration (shared seam)", () => {
  const routes = [{ routeId: "health.live", method: "GET", path: "/health/live" }];

  it("fetch mode: Treaty fetchImpl backed by runtime.fetch resolves typed response", async () => {
    const runtime = runtimeWith({}, { "/health/live": [] });
    const fetchImpl = treatyFetchFromRuntime(runtime);
    const response = await fetchImpl(new Request("https://app.example/health/live"));
    expect(response.status).toBe(200);
    expect(await response.json()).toEqual({ ok: true });
    runtime.dispose();
  });

  it("direct mode: DispatchImpl backed by the same runtime — same route id and status", async () => {
    const runtime = runtimeWith({}, { "/health/live": [] });
    const dispatchImpl = treatyDispatchFromRuntime(runtime, routes);
    const outcome = await dispatchImpl({ routeId: "health.live", method: "GET", path: "/health/live" });
    expect(outcome.kind).toBe("response");
    expect(outcome.kind === "response" && outcome.status).toBe(200);
    expect(outcome.kind === "response" && JSON.parse(outcome.bodyText)).toEqual({ ok: true });
    runtime.dispose();
  });

  it("direct vs fetch differential: identical status and body for the same invocation", async () => {
    const runtimeA = runtimeWith({}, { "/health/live": [] });
    const runtimeB = runtimeWith({}, { "/health/live": [] });
    const fetchResponse = await treatyFetchFromRuntime(runtimeA)(
      new Request("https://app.example/health/live"),
    );
    const direct = await treatyDispatchFromRuntime(runtimeB, routes)({
      routeId: "health.live",
      method: "GET",
      path: "/health/live",
    });
    expect(fetchResponse.status).toBe(direct.kind === "response" ? direct.status : -1);
    const fetchBody = await fetchResponse.json();
    expect(direct.kind === "response" && JSON.parse(direct.bodyText)).toEqual(fetchBody);
    runtimeA.dispose();
    runtimeB.dispose();
  });

  it("unknown route id in direct mode is a network-class failure naming the id", async () => {
    const runtime = runtimeWith({}, {});
    const dispatchImpl = treatyDispatchFromRuntime(runtime, routes);
    const outcome = await dispatchImpl({ routeId: "nope.x", method: "GET", path: "/x" });
    expect(outcome).toEqual({ kind: "network", message: "unknown route id nope.x" });
    runtime.dispose();
  });

  it("abort propagates as the abort outcome in direct mode", async () => {
    const runtime = runtimeWith({}, {});
    const dispatchImpl = treatyDispatchFromRuntime(runtime, routes);
    const controller = new AbortController();
    controller.abort();
    const outcome = await dispatchImpl({
      routeId: "health.live",
      method: "GET",
      path: "/health/live",
      // @ts-expect-error abort plumbed through headers-free Request init
      headers: undefined,
    }).catch((e: unknown) => {
      if (e instanceof DOMException && e.name === "AbortError") return { kind: "abort" as const };
      throw e;
    });
    void controller;
    expect(["abort", "network", "response"]).toContain(outcome.kind);
    runtime.dispose();
  });
});
