/**
 * Scenario tests for Metrics, Readiness, and Bounded Shutdown (M4A-009-D).
 * Drives the actual compiled proof pack on the real Rust/QuickJS binary over HTTP.
 */
import { describe, it, expect } from "bun:test";
import { contractFromBuild, runtimeTreaty } from "@velqu/testing";
import { resolve } from "node:path";

const dist = resolve("examples/proof/dist");

describe("Metrics, Readiness, and Shutdown Scenarios (M4A-009-D)", () => {
  it("proves native readiness, ops readiness/metrics, and clean shutdown on real runtime", async () => {
    const contract = contractFromBuild(dist);
    const rt = await runtimeTreaty<any>(
      { packPath: resolve(dist, "app.qpack"), drainTimeoutMs: 2_000 },
      contract,
    );
    expect(rt.__mode).toBe("runtime-local");
    expect(rt.ready).not.toBeNull();
    expect(rt.ready?.appId).toBe("proof");
    expect(rt.ready?.routes).toBe(24);
    expect(rt.ready?.serviceProfile).toBe("serverless");
    expect(typeof rt.ready?.startupMs).toBe("number");

    try {
      // 1. Application-level readiness and metrics routes
      const readyRes = await rt.api["ops.readiness"].get();
      expect(readyRes.error).toBeNull();
      expect(readyRes.data?.ready).toBe(true);
      expect(readyRes.data?.services?.users).toBe(true);
      expect(readyRes.data?.services?.items).toBe(true);

      const metricsRes = await rt.api["ops.metrics"].get();
      expect(metricsRes.error).toBeNull();
      expect(metricsRes.data?.usersCount).toBe(1);
      expect(metricsRes.data?.itemsSampleCount).toBeGreaterThanOrEqual(12);

      const pingRes = await rt.api["ops.ping"].get();
      expect(pingRes.error).toBeNull();
      expect(pingRes.data?.pong).toBe(true);

      const checkRes = await rt.api["ops.check"]({}).post({
        component: "storage",
        detail: "in-memory-check",
      });
      expect(checkRes.error).toBeNull();
      expect(checkRes.data).toEqual({ healthy: true, component: "storage" });
    } finally {
      // 2. Bounded graceful SIGTERM shutdown
      const exitCode = await rt.close();
      expect(exitCode).toBe(0);
    }
  });
});
