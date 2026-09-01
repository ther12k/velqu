/** M4A-005-B: route allowlist and client bundle isolation evidence. */
import { describe, it, expect, expectTypeOf } from "bun:test";
import { treaty, treatyRoutes, type TreatyClient, type AnyRouteContract } from "./index";

type Api = {
  "health.live": { path: "/health/live"; method: "GET"; params: never; query: never; body: never; headers: never; responses: { 200: { status: string } } };
  "hello.get": { path: "/hello/:name"; method: "GET"; params: { name: string }; query: never; body: never; headers: never; responses: { 200: { message: string } } };
  "users.create": { path: "/users"; method: "POST"; params: never; query: never; body: { name: string }; headers: never; responses: { 201: { id: string } } };
};

const contract = {
  "health.live": { path: "/health/live", method: "GET" },
  "hello.get": { path: "/hello/:name", method: "GET" },
  "users.create": { path: "/users", method: "POST" },
};

describe("Tree-shakable published Treaty client (M4A-005-B)", () => {
  it("materializes only the allowlisted route contract at runtime", async () => {
    const seen: string[] = [];
    const client = treatyRoutes<Api>({
      baseUrl: "https://api.example.test",
      contract,
      fetchImpl: async (input) => {
        seen.push(String(input));
        return Response.json({ status: "ok" });
      },
    }, ["health.live"] as const);
    const selectedClient = client as { health: { live: { get: () => Promise<unknown> } } };

    expect(selectedClient.health.live).toBeDefined();
    expect((client as Record<string, unknown>).hello).toBeUndefined();
    expect((client as Record<string, unknown>).users).toBeUndefined();
    await selectedClient.health.live.get();
    expect(seen).toEqual(["https://api.example.test/health/live"]);
  });

  it("preserves exact TreatyClient typing for the selected route subset", () => {
    const selected = treatyRoutes<Api>({ baseUrl: "https://api.example.test", contract }, ["health.live"] as const);
    const selectedTyped = selected as Pick<TreatyClient<Api>, "health">;
    expectTypeOf(selectedTyped).toMatchTypeOf<Pick<TreatyClient<Api>, "health">>();
    expectTypeOf(selectedTyped.health.live.get).toBeFunction();
  });

  it("keeps the published package isolated from server/compiler imports", async () => {
    const source = await Bun.file("packages/treaty/src/index.ts").text();
    expect(source).not.toContain("@velqu/core");
    expect(source).not.toContain("@velqu/compiler");
    expect(source).not.toContain("q-runtime");
  });

  it("defaults to the full client for compatibility when treaty() is used directly", () => {
    const full = treaty<Api>({ baseUrl: "https://api.example.test", contract });
    expect(full.health.live).toBeDefined();
    expect(full.hello).toBeDefined();
    expect(full.users).toBeDefined();
  });
});
