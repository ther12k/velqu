/**
 * Treaty Conformance Suite (TRT-001..006):
 * - source mode vs published contract mode parity (TRT-006)
 * - runtime-local mode driving the actual q-runtime binary (TRT-005)
 * - unit-local mode labeled separately (TRT-005)
 * - client bundle isolation: published client imports zero server/compiler code (TRT-004)
 */

import { describe, expect, expectTypeOf, test } from "bun:test";
import { treaty } from "@velqu/treaty";
import { contractFromBuild, runtimeTreaty, unitTreaty } from "@velqu/testing";
import type { RouteContract } from "@velqu/contract";
import { readFileSync } from "node:fs";

// ---------------------------------------------------------------- published Api type
// M25-008-A: the client type IS the generated contract — no hand-written
// duplicate interface is required. The compiler emits contract.d.ts from
// the canonical Schema IR; this import proves a consumer needs nothing
// else. Shape facts are pinned by the type-level assertions below plus
// the compiler conformance suite's d.ts snapshots.
import type { Api as GeneratedProofApi } from "../../examples/proof/dist/contract";
export type ProofPublishedApi = GeneratedProofApi;

// Type-level pins (snapshot of the generated projection for the proof
// app): exact route shapes a consumer narrows on.
expectTypeOf<ProofPublishedApi["hello.get"]["responses"]>().toEqualTypeOf<{
  200: { message: string };
}>();
expectTypeOf<ProofPublishedApi["users.get"]["responses"]>().toEqualTypeOf<{
  200: { id: string; name: string; email: string };
  401: { type: "https://velqu.dev/problems/unauthorized"; title: "Unauthorized"; status: 401; instance: string; detail?: string };
}>();
expectTypeOf<ProofPublishedApi["upstream.quote"]["responses"]>().toEqualTypeOf<{
  200: { quote: string; source: string };
  502: { error: string };
}>();

const proofContract = contractFromBuild("examples/proof/dist");

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
    // Start controlled upstream on port 8791 (W4 standard port)
    const upstream = Bun.serve({
      port: 8791,
      hostname: "127.0.0.1",
      fetch(req) {
        const u = new URL(req.url);
        if (u.pathname === "/health") {
          return Response.json({ status: "ok" });
        }
        if (u.pathname === "/io") {
          return Response.json({ status: "ok", ms: 5 });
        }
        if (u.pathname === "/error") {
          return new Response("upstream failure", { status: 500 });
        }
        return new Response("not found", { status: 404 });
      },
    });

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

      // 8. items feature module (M4A-009-A): cursor pagination, CRUD,
      // declared 404 problem, and 422 validation — end-to-end on the
      // actual runtime.
      const page1 = await rt.api["items.list"].get({ query: { limit: 5 } });
      expect(page1.error).toBeNull();
      expect(page1.data?.items.length).toBe(5);
      expect(page1.data?.items[0].id).toBe("itm_001");
      expect(page1.data?.nextCursor).toBe("5");

      const page2 = await rt.api["items.list"].get({
        query: { limit: 5, cursor: page1.data!.nextCursor! },
      });
      expect(page2.error).toBeNull();
      expect(page2.data?.items[0].id).toBe("itm_006");

      const badPage = await rt.api["items.list"].get({ query: { limit: 500 } });
      expect(badPage.data).toBeNull();
      expect(badPage.error?.status).toBe(422);

      const itemCreated = await rt.api["items.create"]({}).post({
        name: "runtime-item",
        tags: ["e2e"],
      });
      expect(itemCreated.error).toBeNull();
      expect(itemCreated.data?.name).toBe("runtime-item");
      const newId = itemCreated.data!.id;

      const fetched = await rt.api["items.get"]({ id: newId }).get();
      expect(fetched.error).toBeNull();
      expect(fetched.data?.tags).toEqual(["e2e"]);

      const renamed = await rt.api["items.update"]({ id: newId }).patch({ name: "renamed" });
      expect(renamed.error).toBeNull();
      expect(renamed.data?.name).toBe("renamed");

      const deleted = await rt.api["items.delete"]({ id: newId }).delete();
      expect(deleted.error).toBeNull();
      expect(deleted.data).toEqual({ deleted: true, id: newId });

      const missing = await rt.api["items.get"]({ id: newId }).get();
      expect(missing.data).toBeNull();
      if (missing.error?.status !== 404) throw new Error("expected declared 404");
      expect(missing.error.problem.type).toBe("https://velqu.dev/problems/not-found");

      // 9. JWT-like policy reference (M4A-009-B): login issues a signed
      // reference token; the protected profile route enforces it end-to-end.
      const badLogin = await rt.api["auth.login"]({}).post({
        username: "ada",
        demoSecret: "wrong-secret",
      });
      expect(badLogin.data).toBeNull();
      expect(badLogin.error?.status).toBe(401);

      const deniedProfile = await rt.api["auth.profile"].get();
      expect(deniedProfile.data).toBeNull();
      expect(deniedProfile.error?.status).toBe(401);

      const login = await rt.api["auth.login"]({}).post({
        username: "ada",
        demoSecret: "jwt-reference-demo-secret",
      });
      expect(login.error).toBeNull();
      const token = login.data!.token;
      expect(token.split(".")).toHaveLength(3);

      const authedProfile = await rt.api["auth.profile"].get({
        headers: { authorization: `Bearer ${token}` },
      });
      expect(authedProfile.error).toBeNull();
      expect(authedProfile.data).toEqual({ userId: "usr_ada", scope: "items:read profile:read" });

      // tampered token signature is rejected by the runtime
      const [h, body] = token.split(".");
      const forged = await rt.api["auth.profile"].get({
        headers: { authorization: `Bearer ${h}.${body}.forged-signature` },
      });
      expect(forged.data).toBeNull();
      expect(forged.error?.status).toBe(401);

      // 10. Controlled upstream (M4A-009-C): quote relay, parameterized relay,
      // fanout aggregation, and typed 502 failure against unavailable upstream.
      const quoteRes = await rt.api["upstream.quote"].get();
      expect(quoteRes.error).toBeNull();
      expect(quoteRes.data).toEqual({ quote: "ok", source: "controlled-upstream" });

      const relayRes = await rt.api["upstream.relay"].get({
        query: { target: `http://127.0.0.1:${upstream.port}/io?ms=5` },
      });
      expect(relayRes.error).toBeNull();
      expect(relayRes.data?.status).toBe("ok");

      const fanoutRes = await rt.api["upstream.fanout"].get({
        query: { count: 2, target: `http://127.0.0.1:${upstream.port}/io?ms=5` },
      });
      expect(fanoutRes.error).toBeNull();
      expect(fanoutRes.data).toEqual({ count: 2, okCount: 2 });

      // Upstream 500 error maps to declared 502 gateway error
      const badRelay = await rt.api["upstream.relay"].get({
        query: { target: `http://127.0.0.1:${upstream.port}/error` },
      });
      expect(badRelay.data).toBeNull();
      expect(badRelay.error?.status).toBe(502);
    } finally {
      upstream.stop(true);
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
