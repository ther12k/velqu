/**
 * BWASM-C-005 — runtime fail-closed policy: install and invoke gates run
 * BEFORE any side effect; the deployment-required problem shape is
 * stable, RFC-9457-compatible, and carries no secret values; Treaty
 * decoding round-trips it.
 */
import { describe, it, expect } from "bun:test";
import {
  CapabilityRegistry,
  CapabilityError,
  deploymentRequiredProblem,
  type CapabilityHandle,
  type KernelInstance,
  type KernelModule,
  type KernelPlanRequest,
  type KernelPlanResult,
} from "../src/index";

// kernel double (same shape as the runtime tests): authorizes everything
function kernelDouble(): KernelModule {
  const kernel = class implements KernelInstance {
    plan_request(requestJson: string): string {
      const req = JSON.parse(requestJson) as KernelPlanRequest;
      const result: KernelPlanResult = {
        kind: "invoke",
        abiVersion: 1,
        routeId: 0,
        handlerKey: "h",
        allowedStatuses: [200],
        defaultStatus: 200,
        deadlineMs: 5_000,
      };
      void req;
      return JSON.stringify(result);
    }
    complete_invocation(_completionJson: string): string {
      return JSON.stringify({ kind: "response", status: 200, headers: [], body: {} });
    }
    authorize_capability(_name: string): string {
      // the kernel inventory GRANTS everything: the policy gate under
      // test must refuse BEFORE this even matters
      return JSON.stringify({ authorized: true });
    }
    dispose(): void {}
  };
  return Object.assign(kernel, { kernel_abi_version: () => 1 });
}

function spyHandle(id: string): { handle: CapabilityHandle; state: { calls: number } } {
  const state = { calls: 0 };
  return {
    handle: {
      id,
      version: 1,
      call: async () => {
        state.calls += 1;
        return { done: true };
      },
    },
    state,
  };
}

describe("C-005 runtime policy gate (fail closed before side effects)", () => {
  const runtime = { authorizeCapability: () => ({ authorized: true }) } as never;

  it("install refuses deployment-required and forbidden ids", () => {
    expect(() =>
      new CapabilityRegistry(runtime as never, [spyHandle("runtime:postgres").handle], {
        deploymentRequired: ["runtime:postgres"],
        forbidden: ["payments"],
      }),
    ).toThrow(CapabilityError);
    expect(() =>
      new CapabilityRegistry(runtime as never, [spyHandle("payments").handle], {
        deploymentRequired: ["runtime:postgres"],
        forbidden: ["payments"],
      }),
    ).toThrow(/never simulated/);
  });

  it("install refuses policy-refused ids first; no kernel auth, no side effects", () => {
    // The INSTALL gate runs before anything else: a refused handle never
    // becomes callable, so no kernel authorization happens either.
    let kernelAuthorizeCalls = 0;
    const countingRuntime = {
      authorizeCapability: () => {
        kernelAuthorizeCalls += 1;
        return { authorized: true };
      },
    } as never;
    const spy = spyHandle("runtime:postgres");
    expect(() =>
      new CapabilityRegistry(countingRuntime, [spy.handle], {
        deploymentRequired: ["runtime:postgres"],
      }),
    ).toThrow(/deployment-required on the browser target/);
    expect(kernelAuthorizeCalls).toBe(0);
    expect(spy.state.calls).toBe(0);
  });

  it("without a policy, R-005 order applies unchanged (kernel auth → call)", async () => {
    const countingRuntime = {
      authorizeCapability: () => {
        kernelAuthorizeCalls += 1;
        return { authorized: true };
      },
    } as never;
    let kernelAuthorizeCalls = 0;
    const spy = spyHandle("runtime:timers");
    const registry = new CapabilityRegistry(countingRuntime, [spy.handle]);
    const r = await registry.invokeForRoute("runtime:timers", ["runtime:timers"], {});
    expect(r).toEqual({ done: true });
    expect(kernelAuthorizeCalls).toBe(1);
    expect(spy.state.calls).toBe(1);
  });

  it("no policy: behavior unchanged (R-005 order preserved)", async () => {
    const spy = spyHandle("runtime:timers");
    const registry = new CapabilityRegistry(runtime as never, [spy.handle]);
    const r = await registry.invokeForRoute("runtime:timers", ["runtime:timers"], { ms: 1 });
    expect(r).toEqual({ done: true });
    expect(spy.state.calls).toBe(1);
  });
});

describe("C-005 deployment-required problem shape", () => {
  it("is stable, RFC-9457-compatible, and secret-free", () => {
    const problem = deploymentRequiredProblem({
      capabilityId: "runtime:postgres",
      routeId: "users.list",
      reason: "declared capability is deployment-required on the browser target",
      remediation: "deploy behind the native Velqu runtime with a linked Postgres pool",
    });
    expect(problem.status).toBe(501);
    expect(problem.body).toMatchObject({
      problemId: "deployment-required",
      type: "https://velqu.dev/problems/deployment-required",
      title: "Capability requires native deployment",
      status: 501,
      capabilityId: "runtime:postgres",
      routeId: "users.list",
    });
    const json = JSON.stringify(problem.body);
    expect(json).not.toMatch(/password|secret|api[_-]?key|Bearer /i);
  });

  it("round-trips through a Response body (Treaty decode path)", async () => {
    const problem = deploymentRequiredProblem({
      capabilityId: "runtime:postgres",
      routeId: "users.list",
      reason: "declared capability is deployment-required on the browser target",
      remediation: "deploy behind the native Velqu runtime",
    });
    const response = new Response(JSON.stringify(problem.body), {
      status: problem.status,
      headers: { "content-type": "application/problem+json" },
    });
    expect(response.status).toBe(501);
    expect(response.headers.get("content-type")).toContain("application/problem+json");
    const decoded = (await response.json()) as { problemId: string; capabilityId: string };
    expect(decoded.problemId).toBe("deployment-required");
    expect(decoded.capabilityId).toBe("runtime:postgres");
  });
});
