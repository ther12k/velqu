/**
 * BWASM-B-004 — Service Worker adapter tests: scope guard, request
 * classification, cache plan, fetch-event core behaviors (install,
 * offline, scope escape, passthrough), update decision, bootstrap
 * fallback. FetchEvent/Cache doubles make every behavior deterministic;
 * real-browser lanes are BWASM-Q-002.
 */
import { describe, it, expect } from "bun:test";
import {
  isScopedRequest,
  classifyRequest,
  buildCachePlan,
  cacheNameFor,
  handleFetchEvent,
  bootstrapServiceWorker,
  updateDecision,
  PASSTHROUGH_PATH_PREFIXES,
  type WorkerEnv,
} from "../src/service-worker";
import type { FetchEventLike } from "../src/service-worker";
import { emitArtifactManifest, type BrowserArtifactManifest } from "../src/artifact-loader";

const enc = (s: string) => new TextEncoder().encode(s);

async function makeManifest(): Promise<BrowserArtifactManifest> {
  const packBytes = enc('{"formatVersion":1,"app":"demo"}');
  const { manifest } = await emitArtifactManifest({
    appId: "demo",
    handlerAbiVersion: 1,
    kernelAbiVersion: 1,
    packSha256: await (async () => {
      const d = await crypto.subtle.digest("SHA-256", packBytes as BufferSource);
      return [...new Uint8Array(d)].map((b) => b.toString(16).padStart(2, "0")).join("");
    })(),
    artifacts: {
      pack: packBytes,
      kernelWasm: enc("wasm"),
      handlerBundle: enc("export {};"),
      manifest: enc("{}"),
      contract: enc("{}"),
      schemas: enc("{}"),
      capabilities: enc("{}"),
      sourceMap: enc("{}"),
    },
    urls: {
      pack: "app.qpack",
      kernelWasm: "kernel.wasm",
      handlerBundle: "app.browser.js",
      manifest: "browser-manifest.json",
      contract: "contract.json",
      schemas: "schema-manifest.json",
      capabilities: "capability-manifest.json",
      sourceMap: "app.browser.map.json",
    },
  });
  return manifest;
}

function makeEvent(url: string, init?: { method?: string; mode?: string; accept?: string }): {
  event: FetchEventLike;
  respond: () => Promise<Response>;
} {
  const headers = new Headers(init?.accept ? { accept: init.accept } : {});
  const request = new Request(url, {
    method: init?.method ?? "GET",
    headers,
    ...(init?.mode ? { mode: init.mode } : {}),
  });
  let response: Promise<Response> | Response | null = null;
  const waited: Array<Promise<unknown>> = [];
  const event: FetchEventLike = {
    request,
    respondWith(r) {
      response = r;
    },
    waitUntil(p) {
      waited.push(p);
    },
  };
  return { event, respond: () => (response === null ? Promise.reject(new Error("no respondWith")) : Promise.resolve(response)) };
}

function makeEnv(overrides?: Partial<WorkerEnv>): WorkerEnv {
  return {
    scope: "/app/",
    locationOrigin: "https://app.example",
    manifest: undefined as unknown as BrowserArtifactManifest,
    runtimeFetch: async (request) =>
      new Response(JSON.stringify({ served: "runtime", path: new URL(request.url).pathname }), {
        status: 200,
        headers: [["content-type", "application/json"]],
      }),
    cache: {
      match: async () => undefined,
      put: async () => {},
    },
    baseUrl: "/app/",
    ...overrides,
  };
}

describe("B-004 scope guard (pure)", () => {
  it("same-origin in-scope requests are intercepted", () => {
    expect(isScopedRequest("/app/", "https://app.example/app/x", "https://app.example")).toBeTrue();
  });
  it("scope escape (outside scope prefix) is NOT intercepted", () => {
    expect(isScopedRequest("/app/", "https://app.example/other/x", "https://app.example")).toBeFalse();
  });
  it("cross-origin requests are NOT intercepted (unrelated origins stay untouched)", () => {
    expect(isScopedRequest("/app/", "https://editor.example/app/x", "https://app.example")).toBeFalse();
  });
  it("root scope intercepts same-origin; unparseable URLs never intercept", () => {
    expect(isScopedRequest("", "https://app.example/anything", "https://app.example")).toBeTrue();
    expect(isScopedRequest("", "not a url", "https://app.example")).toBeFalse();
  });
});

describe("B-004 request classification (support matrix)", () => {
  it("navigation via mode or accept header", () => {
    expect(classifyRequest("GET", "/app/x", null, "navigate")).toBe("navigation");
    expect(classifyRequest("GET", "/app/x", "text/html,...", "cors")).toBe("navigation");
  });
  it("asset paths and extensions classify as asset", () => {
    expect(classifyRequest("GET", "/app/assets/logo.svg", null, "cors")).toBe("asset");
    expect(classifyRequest("GET", "/app/kernel.wasm", null, "cors")).toBe("asset");
  });
  it("api is the default for scoped data requests", () => {
    expect(classifyRequest("GET", "/app/greetings/x", "application/json", "cors")).toBe("api");
  });
  it("editor/auth/gateway prefixes are passthrough (never intercepted)", () => {
    for (const prefix of PASSTHROUGH_PATH_PREFIXES) {
      expect(classifyRequest("GET", `${prefix}/x`, "text/html", "navigate")).toBe("passthrough");
    }
  });
  it("forms: POST with form content-type classifies as api (support matrix)", () => {
    expect(classifyRequest("POST", "/app/form", "application/x-www-form-urlencoded", "cors")).toBe("api");
  });
  it("redirects: the SW serves the runtime's redirect Response as-is (status preserved)", async () => {
    const env = makeEnv({
      runtimeFetch: async () => new Response(null, { status: 302, headers: [["location", "/app/next"]] }),
    });
    const { event, respond } = makeEvent("https://app.example/app/old");
    await handleFetchEvent(event, env);
    const response = await respond();
    expect(response.status).toBe(302);
    expect(response.headers.get("location")).toBe("/app/next");
  });
});

describe("B-004 cache plan (content-addressed)", () => {
  it("precache list covers every manifest artifact with hash + resolved URL", async () => {
    const manifest = await makeManifest();
    const plan = buildCachePlan(manifest, "/app");
    expect(plan.length).toBe(Object.keys(manifest.artifacts).length);
    expect(plan.every((e) => e.url.startsWith("/app/") && /^[0-9a-f]{64}$/.test(e.sha256))).toBeTrue();
    expect(cacheNameFor("demo", manifest.buildId)).toBe(`velqu:demo:${manifest.buildId}`);
  });
});

describe("B-004 fetch-event core", () => {
  it("api requests route to the runtime fetch", async () => {
    const manifest = await makeManifest();
    const env = makeEnv({ manifest });
    const { event, respond } = makeEvent("https://app.example/app/greetings/x");
    await handleFetchEvent(event, env);
    const response = await respond();
    expect(response.status).toBe(200);
    const body = (await response.json()) as { served: string };
    expect(body.served).toBe("runtime");
  });

  it("assets serve cache-first from the verified build cache", async () => {
    const manifest = await makeManifest();
    const cached = new Response("cached-svg", { status: 200 });
    const env = makeEnv({
      manifest,
      cache: { match: async (url) => (url.endsWith("logo.svg") ? cached : undefined), put: async () => {} },
    });
    const { event, respond } = makeEvent("https://app.example/app/assets/logo.svg");
    await handleFetchEvent(event, env);
    const response = await respond();
    expect(await response.text()).toBe("cached-svg");
  });

  it("scope-escape requests are NOT intercepted (no respondWith)", async () => {
    const manifest = await makeManifest();
    const env = makeEnv({ manifest });
    const { event, respond } = makeEvent("https://app.example/outside");
    let called = false;
    await handleFetchEvent(
      {
        request: event.request,
        respondWith() {
          called = true;
        },
        waitUntil() {},
      },
      env,
    );
    expect(called).toBeFalse();
    void respond;
  });

  it("cross-origin requests are NOT intercepted", async () => {
    const manifest = await makeManifest();
    const env = makeEnv({ manifest });
    const { event } = makeEvent("https://other.example/app/api");
    let called = false;
    await handleFetchEvent(
      {
        request: event.request,
        respondWith() {
          called = true;
        },
        waitUntil() {},
      },
      env,
    );
    expect(called).toBeFalse();
  });

  it("navigation with dead runtime + cached build yields the offline problem (503)", async () => {
    const manifest = await makeManifest();
    const offline = new Response(JSON.stringify({ contract: true }), { status: 200 });
    const env = makeEnv({
      manifest,
      runtimeFetch: async () => {
        throw new Error("kernel unavailable offline");
      },
      cache: {
        match: async (url) => (url.endsWith("contract.json") ? offline : undefined),
        put: async () => {},
      },
    });
    const { event, respond } = makeEvent("https://app.example/app/", { mode: "navigate", accept: "text/html" });
    await handleFetchEvent(event, env);
    const response = await respond();
    expect(response.status).toBe(503);
    const problem = (await response.json()) as { problemId: string };
    expect(problem.problemId).toBe("offline");
  });

  it("api with dead runtime and no cache surfaces the runtime failure (never a hang)", async () => {
    const manifest = await makeManifest();
    const env = makeEnv({
      manifest,
      runtimeFetch: async () => {
        throw new Error("kernel unavailable");
      },
    });
    const { event, respond } = makeEvent("https://app.example/app/api");
    await handleFetchEvent(event, env);
    await expect(respond()).rejects.toThrow("kernel unavailable");
  });
});

describe("B-004 bootstrap fallback + update decision", () => {
  it("unsupported ServiceWorker environment yields the injected-fetch fallback", async () => {
    const outcome = await bootstrapServiceWorker({ scriptUrl: "/app/sw.js", scope: "/app/" });
    // bun:test global lacks navigator.serviceWorker → deterministic fallback path
    expect(outcome.kind).toBe("injected-fetch-fallback");
    expect(outcome.kind === "injected-fetch-fallback" && outcome.reason).toContain(
      "injected-fetch fallback",
    );
  });

  it("update decision: different buildId applies on next reload; same is no-change", () => {
    expect(updateDecision("aaa", "bbb")).toBe("apply-on-next-reload");
    expect(updateDecision("aaa", "aaa")).toBe("no-change");
  });
});
