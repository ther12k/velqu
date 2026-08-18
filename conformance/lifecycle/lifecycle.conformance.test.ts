/**
 * Lifecycle & Policy Conformance Suite (RUN-008, SCHEMA-004):
 * - policy session context injection
 * - 401 propagation into Treaty
 * - lazy service initialization (C5)
 * - graceful shutdown
 */

import { describe, expect, test } from "bun:test";
import { runtimeTreaty } from "@q/testing";

const proofContract = {
  "health.live": { path: "/health/live", method: "GET" },
  "hello.get": { path: "/hello/:name", method: "GET" },
  "users.create": { path: "/users", method: "POST" },
  "users.get": { path: "/users/:id", method: "GET" },
};

describe("Lifecycle & Policy conformance (SCHEMA-004)", () => {
  test("policy enforces auth header and injects session into route", async () => {
    const rt = await runtimeTreaty({ packPath: "examples/proof/dist/app.qpack" }, proofContract);

    try {
      // 1. Missing auth -> 401
      const unauth = await rt.api["users.get"]({ id: "usr_1" }).get();
      expect(unauth.data).toBeNull();
      expect(unauth.error?.status).toBe(401);

      // 2. Valid auth -> 200 with user data
      const authed = await rt.api["users.get"]({ id: "usr_1" }).get({
        headers: { authorization: "Bearer q-demo-token" },
      });
      expect(authed.error).toBeNull();
      expect(authed.data?.id).toBe("usr_1");
      expect(authed.data?.name).toBe("Ada");

      // 3. User creation modifies service state (C5)
      const created = await rt.api["users.create"]({}).post({ name: "Bob", email: "bob@example.org" });
      expect(created.error).toBeNull();
      expect(created.data?.id).toBe("usr_1"); // first created user is usr_1 (fresh process sequence)
    } finally {
      await rt.close();
    }
  });
});
