/**
 * Scenario tests for Proof Service Treaty Client (M4A-009-E).
 * Validates dot-navigation, apply-parameters, status narrowing, and tree-shaking
 * against real runtime execution.
 */
import { describe, it, expect } from "bun:test";
import { runtimeTreaty, contractFromBuild } from "@velqu/testing";
import { createProofClient, createProofClientSubset, type ProofApi } from "../client";
import { resolve } from "node:path";

const dist = resolve("examples/proof/dist");

describe("Proof Treaty Client (M4A-009-E)", () => {
  it("interacts with live runtime via typed Treaty client", async () => {
    const contract = contractFromBuild(dist);
    const rt = await runtimeTreaty<ProofApi>(
      { packPath: resolve(dist, "app.qpack"), drainTimeoutMs: 1_000 },
      contract,
    );

    try {
      // Connect client to running local runtime port
      const client = createProofClient({
        fetch: async (url, init) => {
          // Forward via real fetch
          return fetch(url, init);
        },
        baseUrl: `http://127.0.0.1:${rt.port}`,
      });

      // 1. Health check via dot-navigation
      const health = await client.health.live.get();
      expect(health.error).toBeNull();
      expect(health.data).toEqual({ status: "ok" });

      // 2. Hello route with path params
      const hello = await client.hello.get({ name: "TreatyTester" }).get();
      expect(hello.error).toBeNull();
      expect(hello.data).toEqual({ message: "Hello TreatyTester" });

      // 3. Items list with query pagination
      const items = await client.items.list.get({ query: { limit: 3 } });
      expect(items.error).toBeNull();
      expect(items.data?.items.length).toBe(3);

      // 4. Ops readiness
      const ops = await client.ops.readiness.get();
      expect(ops.error).toBeNull();
      expect(ops.data?.ready).toBe(true);

      // 5. Tree-shaken client subset
      const subset = createProofClientSubset(["health.live", "ops.ping"], {
        baseUrl: `http://127.0.0.1:${rt.port}`,
      });
      const ping = await subset.ops.ping.get();
      expect(ping.error).toBeNull();
      expect(ping.data?.pong).toBe(true);
    } finally {
      await rt.close();
    }
  });
});
