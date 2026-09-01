/**
 * M4A-004-B: runtime-local adapter evidence.
 * The tests deliberately start the actual Rust + QuickJS binary, consume its
 * ready identity, use the published contract.json route table, and stop it
 * with a bounded SIGTERM drain.
 */
import { describe, it, expect } from "bun:test";
import { runtimeTreaty, contractFromBuild } from "./index";
import { resolve } from "node:path";

type ProofApi = {
  "health.live": {
    path: "/health/live"; method: "GET"; params: never; query: never; body: never; headers: never;
    responses: { 200: { status: string } };
  };
  "hello.get": {
    path: "/hello/:name"; method: "GET"; params: { name: string }; query: never; body: never; headers: never;
    responses: { 200: { message: string } };
  };
  "users.create": {
    path: "/users"; method: "POST"; params: never; query: never; body: { name: string; email: string }; headers: never;
    responses: { 201: { id: string; name: string; email: string } };
  };
  "users.get": {
    path: "/users/:id"; method: "GET"; params: { id: string }; query: never; body: never; headers: never;
    responses: { 200: { id: string; name: string; email: string }; 401: { type: string; title: string; status: 401; instance: string; detail?: string } };
  };
  "async.timer": {
    path: "/async"; method: "GET"; params: never; query: { ms?: number }; body: never; headers: never;
    responses: { 200: { waited: number } };
  };
};

const dist = resolve("examples/proof/dist");

describe("Runtime-local actual Rust/QuickJS process (M4A-004-B)", () => {
  it("loads the published contract route table rather than a duplicate hand-written table", () => {
    const contract = contractFromBuild(dist);
    expect(Object.keys(contract)).toContain("health.live");
    expect(Object.keys(contract)).toContain("users.get");
    expect(contract["hello.get"]).toEqual({ path: "/hello/:name", method: "GET" });
  });

  it("starts actual runtime, exposes ready identity, serves typed routes, and drains boundedly", async () => {
    const rt = await runtimeTreaty<ProofApi>({ packPath: resolve(dist, "app.qpack"), drainTimeoutMs: 1_000 });
    expect(rt.__mode).toBe("runtime-local");
    expect(rt.ready).not.toBeNull();

    const health = await rt.api.health.live.get();
    expect(health.error).toBeNull();
    expect(health.data).toEqual({ status: "ok" });

    const hello = await rt.api.hello.get({ name: "Runtime" }).get();
    expect(hello.error).toBeNull();
    expect(hello.data).toEqual({ message: "Hello Runtime" });

    const created = await rt.api.users.create.post({ name: "M4B", email: "m4b@example.org" });
    expect(created.error).toBeNull();
    expect(created.data?.id).toBe("usr_1");

    const unauth = await rt.api.users.get({ id: "usr_1" }).get();
    expect(unauth.data).toBeNull();
    expect(unauth.error?.status).toBe(401);

    const timer = await rt.api.async.timer.get({ query: { ms: 5 } });
    expect(timer.error).toBeNull();
    expect(timer.data?.waited).toBe(5);

    const exitCode = await rt.close();
    expect(exitCode).toBe(0);
  });

  it("supports explicit service:N profile and still reports readiness", async () => {
    const rt = await runtimeTreaty<ProofApi>({
      packPath: resolve(dist, "app.qpack"),
      serviceProfile: "service:2",
      drainTimeoutMs: 1_000,
    });
    expect(rt.__mode).toBe("runtime-local");
    expect(rt.ready).not.toBeNull();
    expect(rt.ready?.serviceProfile).toBe("service:2");
    expect(await rt.close()).toBe(0);
  });
});
