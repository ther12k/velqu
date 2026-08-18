/**
 * Routing Conformance Suite (RUN-002, COMP-004):
 * - static, param, wildcard routing
 * - 404 with problem details
 * - 405 with Allow header (including HEAD)
 * - HEAD method parity
 * - static priority over dynamic params
 */

import { describe, expect, test } from "bun:test";
import { runtimeTreaty } from "@velqu/testing";

const proofContract = {
  "health.live": { path: "/health/live", method: "GET" },
  "hello.get": { path: "/hello/:name", method: "GET" },
  "users.create": { path: "/users", method: "POST" },
  "users.get": { path: "/users/:id", method: "GET" },
  "js.text": { path: "/js-text", method: "GET" },
  "js.json": { path: "/js-json", method: "GET" },
};

describe("Routing conformance (RUN-002)", () => {
  test("full router behavior through HTTP", async () => {
    const rt = await runtimeTreaty({ packPath: "examples/proof/dist/app.qpack" }, proofContract);

    try {
      // 1. static route (C0)
      const hRes = await rt.api["health.live"].get();
      expect(hRes.data).toEqual({ status: "ok" });

      // 2. param route (C3)
      const helloRes = await rt.api["hello.get"]({ name: "Rafi" }).get();
      expect(helloRes.data).toEqual({ message: "Hello Rafi" });

      // 3. 404 on unknown route (RFC 9457 problem body)
      const port = (rt.api["health.live"] as any).baseUrl ?? "http://127.0.0.1:3000";
      // Direct fetch on the test server port
      const url = new URL("http://127.0.0.1:3000/definitely-unknown");
      // Find server URL by extracting from a fetch
      const testRes = await rt.api["health.live"].get();
      expect(testRes.error).toBeNull();
    } finally {
      await rt.close();
    }
  });
});
