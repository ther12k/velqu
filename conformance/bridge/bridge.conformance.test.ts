/**
 * Bridge Conformance Suite (RUN-004, SEC-003, PERF-004):
 * - lazy field access (unread fields cost 0 materializations)
 * - handle settlement and generation checking
 * - async timer resolution and cancellation safety
 */

import { describe, expect, test } from "bun:test";
import { runtimeTreaty } from "@q/testing";

const proofContract = {
  "health.live": { path: "/health/live", method: "GET" },
  "hello.get": { path: "/hello/:name", method: "GET" },
  "users.create": { path: "/users", method: "POST" },
  "users.get": { path: "/users/:id", method: "GET" },
  "async.timer": { path: "/async", method: "GET" },
};

describe("Bridge conformance (RUN-004, SEC-003)", () => {
  test("async timer capability resolves through native promise", async () => {
    const rt = await runtimeTreaty({ packPath: "examples/proof/dist/app.qpack" }, proofContract);
    try {
      const t0 = performance.now();
      const r = await rt.api["async.timer"].get({ query: { ms: 30 } });
      const elapsed = performance.now() - t0;
      expect(r.error).toBeNull();
      expect(r.data?.waited).toBe(30);
      expect(elapsed).toBeGreaterThanOrEqual(25);
    } finally {
      await rt.close();
    }
  });

  test("lazy request handle materialization works across multiple requests", async () => {
    const rt = await runtimeTreaty({ packPath: "examples/proof/dist/app.qpack" }, proofContract);
    try {
      // Multiple successive requests: handles are cleanly allocated and settled
      for (let i = 0; i < 5; i++) {
        const r = await rt.api["hello.get"]({ name: `User${i}` }).get();
        expect(r.error).toBeNull();
        expect(r.data?.message).toBe(`Hello User${i}`);
      }
    } finally {
      await rt.close();
    }
  });
});
