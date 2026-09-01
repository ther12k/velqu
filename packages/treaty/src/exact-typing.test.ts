/** M4A-004-D: exact query/header forwarding and response status behavior. */
import { describe, it, expect } from "bun:test";
import { treaty } from "./index";

type Api = {
  "search.list": {
    path: "/search";
    method: "GET";
    params: Record<string, never>;
    query: { q: string; page?: number };
    headers: { authorization: string; "x-trace-id"?: string };
    body: undefined;
    responses: { 200: { items: string[] }; 400: { code: "bad-query" } };
  };
};

describe("Exact Treaty request/response typing (M4A-004-D)", () => {
  it("forwards exact query and declared headers without changing the response union", async () => {
    let seen: { url: string; headers: Headers } | null = null;
    const api = treaty<Api>({
      baseUrl: "https://api.example.test",
      contract: { "search.list": { path: "/search", method: "GET" } },
      fetchImpl: async (input, init) => {
        seen = { url: String(input), headers: new Headers(init?.headers) };
        return Response.json({ items: ["a", "b"] }, { status: 200 });
      },
    });

    const result = await api.search.list.get({
      query: { q: "velqu", page: 2 },
      headers: { authorization: "Bearer test", "x-trace-id": "trace-1" },
    });
    expect(result.error).toBeNull();
    expect(result.data).toEqual({ items: ["a", "b"] });
    expect(seen).not.toBeNull();
    const observed = seen as unknown as { url: string; headers: Headers };
    expect(observed.url).toBe("https://api.example.test/search?q=velqu&page=2");
    expect(observed.headers.get("authorization")).toBe("Bearer test");
    expect(observed.headers.get("x-trace-id")).toBe("trace-1");
  });

  it("preserves a declared non-2xx status and problem body", async () => {
    const api = treaty<Api>({
      baseUrl: "https://api.example.test",
      contract: { "search.list": { path: "/search", method: "GET" } },
      fetchImpl: async () => Response.json({ code: "bad-query" }, { status: 400 }),
    });
    const result = await api.search.list.get({ query: { q: "" }, headers: { authorization: "Bearer test" } });
    expect(result.data).toBeNull();
    expect(result.error?.status).toBe(400);
    if (result.error?.status === 400) {
      expect(result.error.problem).toEqual({ code: "bad-query" });
    }
  });
});
