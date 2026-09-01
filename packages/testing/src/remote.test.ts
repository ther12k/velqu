/** M4A-004-C: remote Treaty client adapter and mode parity. */
import { describe, it, expect } from "bun:test";
import { remoteTreaty } from "./index";
import { unitTreatyDirect } from "./index";
import { status } from "@velqu/core";

type Api = {
  "hello.get": {
    path: "/hello/:name"; method: "GET"; params: { name: string }; query: never; body: never; headers: never;
    responses: { 200: { message: string }; 404: { code: string } };
  };
  "users.create": {
    path: "/users"; method: "POST"; params: never; query: never; body: { name: string }; headers: never;
    responses: { 201: { name: string }; 422: { code: string } };
  };
};

const contract = {
  "hello.get": { path: "/hello/:name", method: "GET" },
  "users.create": { path: "/users", method: "POST" },
};

function fakeFetch(input: Request | URL | string, init?: RequestInit): Promise<Response> {
  const url = new URL(String(input));
  if (url.pathname === "/hello/Ada" && init?.method === "GET") {
    return Promise.resolve(Response.json({ message: "Hello Ada" }, { status: 200 }));
  }
  if (url.pathname === "/hello/missing" && init?.method === "GET") {
    return Promise.resolve(Response.json({ code: "not-found" }, { status: 404 }));
  }
  if (url.pathname === "/users" && init?.method === "POST") {
    const body = JSON.parse(String(init.body));
    if (body.name === "") return Promise.resolve(Response.json({ code: "invalid" }, { status: 422 }));
    return Promise.resolve(Response.json({ name: body.name }, { status: 201 }));
  }
  return Promise.resolve(new Response("no route", { status: 404 }));
}

describe("Remote Treaty client (M4A-004-C)", () => {
  it("is explicitly labeled remote and uses injected HTTP fetch", async () => {
    const remote = remoteTreaty<Api>({ baseUrl: "https://api.example.test", contract, fetchImpl: fakeFetch });
    expect(remote.__mode).toBe("remote");
    const hello = await remote.api.hello.get({ name: "Ada" }).get();
    expect(hello.error).toBeNull();
    expect(hello.data).toEqual({ message: "Hello Ada" });
  });

  it("preserves typed non-2xx statuses and problem bodies over HTTP", async () => {
    const remote = remoteTreaty<Api>({ baseUrl: "https://api.example.test", contract, fetchImpl: fakeFetch });
    const missing = await remote.api.hello.get({ name: "missing" }).get();
    expect(missing.data).toBeNull();
    expect(missing.error?.status).toBe(404);
    if (missing.error?.status === 404) expect(missing.error.problem).toEqual({ code: "not-found" });

    const invalid = await remote.api.users.create.post({ name: "" });
    expect(invalid.data).toBeNull();
    expect(invalid.error?.status).toBe(422);
  });

  it("supports custom fetch abort and network classification", async () => {
    const aborted = remoteTreaty<Api>({
      baseUrl: "https://api.example.test",
      contract,
      fetchImpl: async (_input, init) => {
        await new Promise((resolve) => setTimeout(resolve, 20));
        if (init?.signal?.aborted) throw new DOMException("aborted", "AbortError");
        return Response.json({ message: "late" });
      },
    });
    const controller = new AbortController();
    const pending = aborted.api.hello.get({ name: "Ada" }).get({ signal: controller.signal });
    controller.abort();
    const abortResult = await pending;
    expect(abortResult.error?.status).toBe(0);
    if (abortResult.error?.status === 0) expect(abortResult.error.kind).toBe("abort");

    const network = remoteTreaty<Api>({
      baseUrl: "https://api.example.test",
      contract,
      fetchImpl: async () => { throw new Error("offline"); },
    });
    const networkResult = await network.api.hello.get({ name: "Ada" }).get();
    expect(networkResult.error?.status).toBe(0);
    if (networkResult.error?.status === 0) expect(networkResult.error.kind).toBe("network");
  });

  it("has parity with direct unit-local mode for equivalent contract results", async () => {
    const direct = unitTreatyDirect<Api>({
      routes: {
        "hello.get": { path: "/hello/:name", method: "GET", responses: { 200: {}, 404: {} }, handle: (ctx: any) => ({ message: `Hello ${ctx.params.name}` }) },
        "users.create": { path: "/users", method: "POST", responses: { 201: {}, 422: {} }, handle: (ctx: any) => ctx.body.name ? status(201).value({ name: ctx.body.name }) : status(422).value({ code: "invalid" }) },
      },
    });
    const remote = remoteTreaty<Api>({ baseUrl: "https://api.example.test", contract, fetchImpl: fakeFetch });
    const directHello = await direct.api.hello.get({ name: "Ada" }).get();
    const remoteHello = await remote.api.hello.get({ name: "Ada" }).get();
    expect(remoteHello).toEqual(directHello);
    const directCreated = await direct.api.users.create.post({ name: "Ada" });
    const remoteCreated = await remote.api.users.create.post({ name: "Ada" });
    expect(remoteCreated).toEqual(directCreated);
    direct.close();
  });
});
