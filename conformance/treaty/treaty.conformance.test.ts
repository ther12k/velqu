/**
 * Treaty Conformance Suite (TRT-001..006):
 * - source mode vs published contract mode parity (TRT-006)
 * - runtime-local mode driving the actual q-runtime binary (TRT-005)
 * - unit-local mode labeled separately (TRT-005)
 * - client bundle isolation: published client imports zero server/compiler code (TRT-004)
 */

import { describe, expect, expectTypeOf, test } from "bun:test";
import { treaty } from "@velqu/treaty";
import { runtimeTreaty, unitTreaty } from "@velqu/testing";
import type { RouteContract } from "@velqu/contract";
import { readFileSync } from "node:fs";

// ---------------------------------------------------------------- published Api type
// M25-006-C: problem responses carry the exact runtime envelope — frozen
// type URI + title literals, the declared status literal — so Treaty error
// narrowing is exact after `if (r.error.status === N)`.
export type ProofPublishedApi = {
  "health.live": RouteContract<"/health/live", "GET", Record<string, never>, Record<string, never>, undefined, { 200: { status: string } }>;
  "hello.get": RouteContract<"/hello/:name", "GET", { name: string }, Record<string, never>, undefined, { 200: { message: string } }>;
  "users.create": RouteContract<"/users", "POST", Record<string, never>, Record<string, never>, { name: string; email: string }, { 201: { id: string; name: string; email: string } }>;
  "users.get": RouteContract<"/users/:id", "GET", { id: string }, Record<string, never>, undefined, {
    200: { id: string; name: string; email: string };
    401: { type: "https://velqu.dev/problems/unauthorized"; title: "Unauthorized"; status: 401; instance: string; detail?: string };
  }>;
  "async.timer": RouteContract<"/async", "GET", Record<string, never>, { ms?: number }, undefined, { 200: { waited: number } }>;
};

const proofContract = {
  "health.live": { path: "/health/live", method: "GET" },
  "hello.get": { path: "/hello/:name", method: "GET" },
  "users.create": { path: "/users", method: "POST" },
  "users.get": { path: "/users/:id", method: "GET" },
  "async.timer": { path: "/async", method: "GET" },
};

describe("Treaty client bundle isolation (TRT-004)", () => {
  test("packages/treaty contains zero imports of server/compiler runtime", () => {
    const treatySrc = readFileSync("packages/treaty/src/index.ts", "utf8");
    expect(treatySrc).not.toContain("@velqu/core");
    expect(treatySrc).not.toContain("@velqu/compiler");
    expect(treatySrc).not.toContain("bun:");
    expect(treatySrc).not.toContain("node:");
    expect(treatySrc).not.toContain("rquickjs");
  });
});

describe("Treaty runtime-local mode (ACTUAL binary over HTTP)", () => {
  test("drives compiled proof pack end-to-end", async () => {
    const rt = await runtimeTreaty<ProofPublishedApi>(
      { packPath: "examples/proof/dist/app.qpack" },
      proofContract,
    );
    expect(rt.__mode).toBe("runtime-local");

    try {
      // 1. health (C0)
      const health = await rt.api["health.live"].get();
      expect(health.error).toBeNull();
      expect(health.data).toEqual({ status: "ok" });

      // 2. hello (C3 path validation)
      const hello = await rt.api["hello.get"]({ name: "Rafi" }).get();
      expect(hello.error).toBeNull();
      expect(hello.data).toEqual({ message: "Hello Rafi" });

      // 3. hello validation failure (422)
      const helloBad = await rt.api["hello.get"]({ name: "x".repeat(61) }).get();
      expect(helloBad.data).toBeNull();
      expect(helloBad.error?.status).toBe(422);

      // 4. users.create (POST 201)
      const created = await rt.api["users.create"]({}).post({ name: "Ada", email: "ada@example.org" });
      expect(created.error).toBeNull();
      expect(created.data?.id).toBe("usr_1");

      // 5. users.get without auth → 401. The policy-provided error flows
      // into the Treaty union: narrowing on status types the problem as
      // the exact unauthorized envelope (M25-006-C)
      const unauth = await rt.api["users.get"]({ id: "usr_1" }).get();
      expect(unauth.data).toBeNull();
      if (unauth.error?.status !== 401) throw new Error("expected 401");
      expect(unauth.error.problem.type).toBe("https://velqu.dev/problems/unauthorized");
      expect(unauth.error.problem.title).toBe("Unauthorized");
      expect(unauth.error.problem.status).toBe(401);
      expect(typeof unauth.error.problem.instance).toBe("string");

      // 6. users.get with auth → 200
      const authed = await rt.api["users.get"]({ id: "usr_1" }).get({
        headers: { authorization: "Bearer q-demo-token" },
      });
      expect(authed.error).toBeNull();
      expect(authed.data?.name).toBe("Ada");

      // 7. async timer (C5 native op)
      const timer = await rt.api["async.timer"].get({ query: { ms: 20 } });
      expect(timer.error).toBeNull();
      expect(timer.data?.waited).toBe(20);
    } finally {
      await rt.close();
    }
  });
});

describe("Treaty unit-local mode (explicitly labeled)", () => {
  test("unit-local adapter is labeled and runs in-process", async () => {
    const unit = unitTreaty({
      routes: {
        "health.live": {
          path: "/health/live",
          method: "GET",
          handle: () => ({ status: "ok" }),
        },
      },
    });
    expect(unit.__mode).toContain("unit-local (NOT runtime conformance)");
    const r = await unit.api["health.live"].get();
    expect(r.data).toEqual({ status: "ok" });
    unit.close();
  });
});
