/**
 * Ops module unit tests (M4A-009-D): route contracts, readiness, and metrics.
 */
import { describe, it, expect } from "bun:test";
import { readiness, metrics, version, ping, check } from "./routes";

describe("ops module (metrics and readiness M4A-009-D)", () => {
  it("declares ops.readiness GET route with healthy services", () => {
    expect(readiness.id).toBe("ops.readiness");
    expect(readiness.method).toBe("GET");
    expect(readiness.path).toBe("/ops/readiness");

    const res = readiness.handle();
    expect(res.ready).toBe(true);
    expect(res.services.users).toBe(true);
    expect(res.services.items).toBe(true);
  });

  it("declares ops.metrics GET route exposing uptime and counts", () => {
    expect(metrics.id).toBe("ops.metrics");
    expect(metrics.method).toBe("GET");
    expect(metrics.path).toBe("/ops/metrics");

    const res = metrics.handle();
    expect(res.uptimeMs).toBeGreaterThanOrEqual(0);
    expect(res.usersCount).toBe(1);
    expect(res.itemsSampleCount).toBeGreaterThanOrEqual(12);
  });

  it("declares ops.version, ping, and check routes", () => {
    expect(version.handle()).toEqual({
      appId: "proof",
      engine: "quickjs-ng",
      version: "0.1.0-alpha",
    });

    const p = ping.handle();
    expect(p.pong).toBe(true);

    const c = check.handle({ body: { component: "database" } });
    expect(c.healthy).toBe(true);
    expect(c.component).toBe("database");
  });
});
