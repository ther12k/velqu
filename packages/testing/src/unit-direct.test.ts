/**
 * M4A-004-A: unit-local DIRECT generated dispatcher.
 *
 * Proves the direct in-process mode against the loopback unit-local mode:
 * same contract, same Treaty client type, same 2xx/error splitting, same
 * failures — no HTTP anywhere in the direct mode. Also proves the
 * "undeclared status is a contract error" guardrail.
 */
import { describe, it, expect } from "bun:test";
import { unitTreatyDirect, UndeclaredStatusError, unitTreaty, type UnitDirectRoute } from "@velqu/testing";
import { status } from "@velqu/core";

type TestApi = {
  "health.live": {
    path: "/health/live";
    method: "GET";
    params: never;
    query: never;
    body: never;
    headers: never;
    responses: { 200: { status: string } };
  };
  "greetings.get": {
    path: "/greetings/:name";
    method: "GET";
    params: { name: string };
    query: never;
    body: never;
    headers: never;
    responses: { 200: { message: string } };
  };
  "greetings.create": {
    path: "/greetings";
    method: "POST";
    params: never;
    query: never;
    body: { name: string; customGreeting?: string };
    headers: never;
    responses: { 201: { name: string; greeting: string } };
  };
  "gates.check": {
    path: "/gates/:id";
    method: "GET";
    params: { id: string };
    query: never;
    body: never;
    headers: never;
    responses: {
      200: { open: boolean };
      404: { __problem: true; problem: string; status: 404; detail: string };
    };
  };
};

interface GateCtx {
  params: { id: string };
}
interface GreetCtx {
  params: { name: string };
}
interface CreateCtx {
  body: { name: string; customGreeting?: string };
}

const handleLive = () => ({ status: "ok" });
const handleGreet = (ctx: GreetCtx) => ({ message: `Hello, ${ctx.params.name}!` });
const handleCreate = (ctx: CreateCtx) =>
  status(201).value({ name: ctx.body.name, greeting: ctx.body.customGreeting ?? `Welcome, ${ctx.body.name}!` });
const handleGate = (ctx: GateCtx) =>
  ctx.params.id === "main"
    ? { open: true }
    : status(404).problem("not-found", { detail: `gate ${ctx.params.id} unknown` });

const directRoutes = {
  "health.live": { path: "/health/live", method: "GET", responses: { 200: {} }, handle: handleLive },
  "greetings.get": { path: "/greetings/:name", method: "GET", responses: { 200: {} }, handle: handleGreet },
  "greetings.create": { path: "/greetings", method: "POST", responses: { 201: {} }, handle: handleCreate },
  "gates.check": { path: "/gates/:id", method: "GET", responses: { 200: {}, 404: {} }, handle: handleGate },
} as unknown as Record<string, UnitDirectRoute>;

describe("Unit-local DIRECT dispatcher (M4A-004-A)", () => {
  it("is labeled as unit-local direct and NOT runtime conformance", () => {
    const unit = unitTreatyDirect<TestApi>({ routes: directRoutes });
    expect(unit.__mode).toBe("unit-local (direct dispatcher, NOT runtime conformance)");
    unit.close();
  });

  it("drives handlers directly: dot-navigation, apply-form params, POST body", async () => {
    const unit = unitTreatyDirect<TestApi>({ routes: directRoutes });
    try {
      const health = await unit.api.health.live.get();
      expect(health.error).toBeNull();
      expect(health.data).toEqual({ status: "ok" });

      const greeting = await unit.api.greetings.get({ name: "Ada" }).get();
      expect(greeting.error).toBeNull();
      expect(greeting.data?.message).toBe("Hello, Ada!");

      const created = await unit.api.greetings.create.post({
        name: "Grace",
        customGreeting: "Ahoy!",
      });
      expect(created.error).toBeNull();
      expect(created.data?.name).toBe("Grace");
      expect(created.data?.greeting).toBe("Ahoy!");
    } finally {
      unit.close();
    }
  });

  it("splits 2xx data and non-2xx typed errors by declared status (404 problem)", async () => {
    const unit = unitTreatyDirect<TestApi>({ routes: directRoutes });
    try {
      const open = await unit.api.gates.check({ id: "main" }).get();
      expect(open.error).toBeNull();
      expect(open.data).toEqual({ open: true });

      const missing = await unit.api.gates.check({ id: "side" }).get();
      expect(missing.data).toBeNull();
      if (missing.error?.status !== 404) throw new Error("expected declared 404");
      expect(missing.error.problem.status).toBe(404);
      expect(missing.error.problem.detail).toBe("gate side unknown");
    } finally {
      unit.close();
    }
  });

  it("undeclared status is a CONTRACT ERROR (fails loud, names route and declared set)", async () => {
    const unit = unitTreatyDirect({
      routes: {
        "only.created": {
          path: "/only",
          method: "POST",
          responses: { 201: {} }, // 200 NOT declared
          handle: () => ({ oops: true }), // plain return maps to 200
        },
      },
    });
    const api = unit.api as unknown as {
      only: { created: { post: (b: unknown) => Promise<{ data: unknown; error: unknown }> } };
    };
    await expect(api.only.created.post({ x: 1 })).rejects.toThrow(UndeclaredStatusError);
    await expect(api.only.created.post({ x: 1 })).rejects.toThrow(
      /status 200, which the contract never declared \(declared: 201\)/,
    );
    unit.close();
  });

  it("method mismatch fails loud in the direct dispatcher", async () => {
    const unit = unitTreatyDirect({
      routes: {
        "only.get": {
          path: "/only",
          method: "GET",
          responses: { 200: {} },
          handle: () => ({ ok: true }),
        },
      },
    });
    const api = unit.api as unknown as {
      only: { get: { post: (b: unknown) => Promise<{ data: unknown; error: unknown }> } };
    };
    let thrown: unknown = null;
    try {
      await api.only.get.post({});
    } catch (e) {
      thrown = e;
    }
    expect(thrown).toBeInstanceOf(Error);
    expect((thrown as Error).message).toMatch(/method "POST" is not allowed/);
    unit.close();
  });

  it("mode parity: direct dispatcher and loopback unit-local return identical results", async () => {
    const direct = unitTreatyDirect<TestApi>({ routes: directRoutes });
    const loop = unitTreaty<TestApi>({
      routes: {
        "health.live": { path: "/health/live", method: "GET", handle: () => handleLive() },
        "greetings.get": {
          path: "/greetings/:name",
          method: "GET",
          handle: (ctx: unknown) => handleGreet(ctx as GreetCtx),
        },
        "greetings.create": {
          path: "/greetings",
          method: "POST",
          handle: (ctx: unknown) => handleCreate(ctx as CreateCtx),
        },
        "gates.check": {
          path: "/gates/:id",
          method: "GET",
          handle: (ctx: unknown) => handleGate(ctx as GateCtx),
        },
      },
    });

    try {
      // 2xx data parity
      const dHealth = await direct.api.health.live.get();
      const lHealth = await loop.api.health.live.get();
      expect(dHealth).toEqual(lHealth);

      const dCreated = await direct.api.greetings.create.post({ name: "Parity" });
      const lCreated = await loop.api.greetings.create.post({ name: "Parity" });
      expect(dCreated).toEqual(lCreated);
      expect(dCreated.data?.greeting).toBe("Welcome, Parity!");

      const dGreeting = await direct.api.greetings.get({ name: "Parity" }).get();
      const lGreeting = await loop.api.greetings.get({ name: "Parity" }).get();
      expect(dGreeting).toEqual(lGreeting);

      // non-2xx typed error parity (404 problem flows through both modes)
      const dMissing = await direct.api.gates.check({ id: "nope" }).get();
      const lMissing = await loop.api.gates.check({ id: "nope" }).get();
      expect(dMissing.data).toBeNull();
      expect(lMissing.data).toBeNull();
      expect(dMissing.error && "status" in dMissing.error && dMissing.error.status).toBe(404);
      expect(lMissing.error && "status" in lMissing.error && lMissing.error.status).toBe(404);
    } finally {
      direct.close();
      loop.close();
    }
  });
});
