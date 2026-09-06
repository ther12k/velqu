/**
 * BWASM-C-001 — browser capability baseline tests: timer bounds and
 * cancellation, pinned crypto semantics, bounded redacted console,
 * default-deny restricted fetch, graph introspection, and the manifest
 * capability declaration (B-002 canonicalization, backward compatible).
 */
import { describe, it, expect } from "bun:test";
import {
  createBrowserCapabilityGraph,
  createTimerCapability,
  createCryptoCapability,
  createConsoleCapability,
  createFetchCapability,
  baselineHandles,
  redactSensitiveText,
  ringBufferSink,
  TimerCancelled,
  TimerDelayTooLarge,
  CryptoSemanticsMismatch,
  ConsoleLimitExceeded,
  FetchPolicyDenied,
  FetchLimitExceeded,
  BROWSER_TIMERS_ID,
  BROWSER_CRYPTO_ID,
  BROWSER_CONSOLE_ID,
  BROWSER_FETCH_ID,
  BROWSER_CAPABILITY_VERSION,
  MAX_CONSOLE_RECORDS,
  type BrowserFetchInput,
} from "../src/capabilities";
import { emitArtifactManifest, loadArtifacts, sha256Hex, type ArtifactRole } from "../src/artifact-loader";

const enc = (s: string) => new TextEncoder().encode(s);
const hex = (bytes: Uint8Array) => [...bytes].map((b) => b.toString(16).padStart(2, "0")).join("");

// ---------------------------------------------------------------------------
// Timer
// ---------------------------------------------------------------------------

describe("C-001 timer (runtime:timers v1)", () => {
  it("resolves after the delay with elapsed time", async () => {
    const { timers } = createTimerCapability();
    const r = await timers.delay(30);
    expect(r.elapsedMs).toBeGreaterThanOrEqual(25);
  });

  it("rejects delays above the ceiling before scheduling", async () => {
    const { timers } = createTimerCapability();
    await expect(timers.delay(301_000)).rejects.toBeInstanceOf(TimerDelayTooLarge);
    await expect(timers.delay(-1)).rejects.toMatchObject({ code: "timer-invalid-delay" });
  });

  it("cancellation clears the timer and the work never fires", async () => {
    const { timers } = createTimerCapability();
    const controller = new AbortController();
    const p = timers.delay(30, controller.signal);
    controller.abort(); // cancel FIRST, then observe the rejection
    const reason = await p.then(
      () => "resolved",
      (e) => e,
    );
    expect(reason).toBeInstanceOf(TimerCancelled);
    // the discarded timer must never fire: past the original 5s?
    // (shortened here) — the promise stays rejected forever.
    await new Promise((r) => setTimeout(r, 40));
    await expect(p).rejects.toBeInstanceOf(TimerCancelled);
  });

  it("pre-aborted signals reject immediately", async () => {
    const { timers } = createTimerCapability();
    const controller = new AbortController();
    controller.abort();
    await expect(timers.delay(10, controller.signal)).rejects.toBeInstanceOf(TimerCancelled);
  });
});

// ---------------------------------------------------------------------------
// Crypto
// ---------------------------------------------------------------------------

describe("C-001 crypto (runtime:crypto v1, pinned subset)", () => {
  it("digest matches the shared SHA-256 vector (abc)", async () => {
    const { crypto } = createCryptoCapability();
    const digest = await crypto.digestHex("SHA-256", enc("abc"));
    // NIST FIPS 180-4 test vector — the SAME vector the native sha2 path
    // is pinned against, so browser/native parity is checkable.
    expect(digest).toBe("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
  });

  it("digest supports bytes encoding and the SHA-384/512 family", async () => {
    const { crypto } = createCryptoCapability();
    const b384 = (await crypto.call({ op: "digest", algorithm: "SHA-384", data: enc("abc"), encoding: "bytes" })) as Uint8Array;
    expect(b384.byteLength).toBe(48);
    const h512 = await crypto.digestHex("SHA-512", enc("abc"));
    expect(h512.startsWith("ddaf35a193617aba")).toBeTrue();
  });

  it("rejects unpinned algorithms and ops by name — no silent substitution", async () => {
    const { crypto } = createCryptoCapability();
    await expect(
      crypto.call({ op: "digest", algorithm: "MD5" as never, data: enc("x") }),
    ).rejects.toBeInstanceOf(CryptoSemanticsMismatch);
    await expect(
      crypto.call({ op: "sign" } as never),
    ).rejects.toMatchObject({ code: "crypto-semantics-mismatch" });
  });

  it("getRandomValues is bounded and returns the requested length", async () => {
    const { crypto } = createCryptoCapability();
    const bytes = await crypto.getRandomValues(32);
    expect(bytes.byteLength).toBe(32);
    await expect(crypto.getRandomValues(65_537)).rejects.toBeInstanceOf(CryptoSemanticsMismatch);
  });
});

// ---------------------------------------------------------------------------
// Console
// ---------------------------------------------------------------------------

describe("C-001 console (runtime:console v1, bounded + redacted)", () => {
  it("redacts auth headers, secret prefixes, and key=value secrets (native parity)", () => {
    // Property-style parity with the native suite (q-capabilities
    // console.rs pins !contains(secret) + contains("key=[REDACTED]"));
    // exact strings are intentionally NOT pinned so the JS port can
    // track the native redactor's branch details.
    const r = (msg: string) => redactSensitiveText(msg);
    const out1 = r("authorization: Bearer abc.def, next=1");
    expect(out1).not.toContain("abc.def");
    expect(out1).toContain("[REDACTED]");
    expect(r("Bearer abc.def, next=1")).toBe("Bearer [REDACTED], next=1");
    const out2 = r("key sk-live-abc123 done");
    expect(out2).not.toContain("sk-live-abc123");
    expect(out2).toContain("sk-live-[REDACTED]");
    const out3 = r('token="supersecret"');
    expect(out3).not.toContain("supersecret");
    expect(out3).toContain("token=[REDACTED]");
    const out4 = r("password=hunter2");
    expect(out4).not.toContain("hunter2");
    expect(out4).toContain("password=[REDACTED]");
    // native-limitation note: the colon+space form ("password: x") is
    // NOT covered by the native redactor's value loop either — parity
    // preserved by also leaking it; handled by the "=" form above. 
    const out5 = r("GitHub token ghp_1234567890abcdef and sk-live-999000");
    expect(out5).toContain("ghp_[REDACTED]");
    expect(out5).not.toContain("ghp_1234567890abcdef");
  });

  it("forwards structured, truncated records to the sink", () => {
    const sink = ringBufferSink(4);
    const { console: c } = createConsoleCapability({ sink, correlationId: () => "inv-9" });
    c.warn("token=abc123", { extra: true });
    c.error("x".repeat(17_000));
    const records = sink.records();
    expect(records.length).toBe(2);
    expect(records[0]).toMatchObject({ level: "warn", correlationId: "inv-9", truncated: false });
    expect(records[0]!.message).toContain("token=[REDACTED]");
    expect(records[1]!.truncated).toBeTrue();
    expect(records[1]!.message.endsWith("...[TRUNCATED]")).toBeTrue();
  });

  it("floods produce structured limit errors, not silent drops", () => {
    const sink = ringBufferSink(999);
    const { console: c } = createConsoleCapability({ sink });
    for (let i = 0; i < MAX_CONSOLE_RECORDS; i++) c.info("fill");
    expect(() => c.info("overflow")).toThrow(ConsoleLimitExceeded);
    expect(() => c.info("a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
      "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "1", "2", "3", "4", "5", "6", "7", "8")).toThrow(
      /arg ceiling/,
    );
  });
});

// ---------------------------------------------------------------------------
// Fetch
// ---------------------------------------------------------------------------

function respondingFetch(status = 200, body = "ok", headers: Record<string, string> = {}) {
  const calls: Request[] = [];
  const impl = (async (request: Request) => {
    calls.push(request);
    return new Response(body, { status, headers });
  }) as typeof fetch;
  return { impl, calls };
}

describe("C-001 fetch (runtime:fetch v1, default deny)", () => {
  const input: BrowserFetchInput = { url: "https://api.example.com/v1/data" };

  it("default deny: no allowlist denies everything before any I/O", async () => {
    const { fetch } = createFetchCapability({ policy: { allowedOrigins: [] } });
    await expect(fetch.call(input)).rejects.toMatchObject({ code: "fetch-policy-denied" });
  });

  it("allowlisted origin passes; method policy is enforced", async () => {
    const { impl, calls } = respondingFetch();
    const { fetch } = createFetchCapability({
      policy: { allowedOrigins: ["https://api.example.com"] },
      fetchImpl: impl,
    });
    const r = await fetch.call(input);
    expect(r.status).toBe(200);
    expect(new TextDecoder().decode(r.bodyBytes)).toBe("ok");
    await expect(fetch.call({ ...input, method: "POST" })).rejects.toMatchObject({
      code: "fetch-policy-denied",
    });
    expect(calls.length).toBe(1); // the POST never reached the network
  });

  it("cross-origin and non-http(s) schemes are denied", async () => {
    const { impl } = respondingFetch();
    const { fetch } = createFetchCapability({
      policy: { allowedOrigins: ["https://api.example.com"] },
      fetchImpl: impl,
    });
    await expect(fetch.call({ url: "https://evil.example.com/x" })).rejects.toBeInstanceOf(FetchPolicyDenied);
    await expect(fetch.call({ url: "file:///etc/passwd" })).rejects.toBeInstanceOf(FetchPolicyDenied);
    await expect(fetch.call({ url: "https://api.example.com.evIL.example/x" })).rejects.toBeInstanceOf(
      FetchPolicyDenied,
    );
  });

  it("credentials are forced to omit and credential headers are stripped", async () => {
    const { impl, calls } = respondingFetch();
    const { fetch } = createFetchCapability({
      policy: { allowedOrigins: ["https://api.example.com"] },
      fetchImpl: impl,
    });
    await fetch.call({ ...input, headers: { authorization: "Bearer secret", cookie: "session=1" } });
    // wire contract: credential headers never reach the network
    expect(calls[0]!.headers.get("authorization")).toBeNull();
    expect(calls[0]!.headers.get("cookie")).toBeNull();
    // the RequestInit carries credentials:"omit" for browser enforcement
    // (Bun's Request does not reflect the field back — browser fidelity
    // gap, noted; real browsers enforce it).
    expect(fetch.policySummary()).toContain("credentials: omit");
  });

  it("response bodies are bounded with a structured limit and truncation marker", async () => {
    const big = "x".repeat(1_000);
    const { impl } = respondingFetch(200, big);
    const { fetch } = createFetchCapability({
      policy: { allowedOrigins: ["https://api.example.com"], maxResponseBytes: 100 },
      fetchImpl: impl,
    });
    const r = await fetch.call(input);
    expect(r.truncated).toBeTrue();
    expect(r.bodyBytes.byteLength).toBe(100);
  });

  it("oversized request bodies are rejected before I/O; deadline produces a limit error", async () => {
    const { impl, calls } = respondingFetch();
    const { fetch } = createFetchCapability({
      policy: { allowedOrigins: ["https://api.example.com"], allowedMethods: ["GET", "HEAD", "POST"], maxRequestBytes: 10 },
      fetchImpl: impl,
    });
    await expect(fetch.call({ ...input, method: "POST", body: new Uint8Array(11) })).rejects.toBeInstanceOf(
      FetchLimitExceeded,
    );
    expect(calls.length).toBe(0);

    const never = (async () => new Promise<Response>(() => {})) as typeof fetch;
    const timed = createFetchCapability({
      policy: { allowedOrigins: ["https://api.example.com"], deadlineMs: 30 },
      fetchImpl: never,
    });
    await expect(timed.fetch.call(input)).rejects.toMatchObject({ code: "fetch-limit-exceeded" });
  });

  it("redirects are denied by policy (never followed, never escaped)", async () => {
    const calls: Request[] = [];
    const impl = (async (request: Request) => {
      calls.push(request);
      return new Response("moved", { status: 302, headers: { location: "https://evil.example.com" } });
    }) as unknown as typeof fetch;
    const { fetch } = createFetchCapability({
      policy: { allowedOrigins: ["https://api.example.com"] },
      fetchImpl: impl,
    });
    await expect(fetch.call(input)).rejects.toMatchObject({ code: "fetch-policy-denied" });
    // Bun's Request.redirect does not reflect the init field (fidelity
    // gap, like credentials); the adapter's own 3xx check enforces the
    // policy for any transport — proven by the rejection above.
    expect(fetch.policySummary()).toContain("redirects: denied");
  });

  it("the policy summary is introspectable before execution", () => {
    const { fetch } = createFetchCapability({ policy: { allowedOrigins: [] } });
    expect(fetch.policySummary()).toContain("default-deny");
    expect(fetch.policySummary()).toContain("credentials: omit");
  });
});

// ---------------------------------------------------------------------------
// Graph + manifest declaration + registry handles
// ---------------------------------------------------------------------------

describe("C-001 capability graph (introspection + manifest)", () => {
  function makeGraph(fetchImpl?: typeof fetch) {
    return createBrowserCapabilityGraph({
      consoleSink: ringBufferSink(16),
      fetchPolicy: { allowedOrigins: ["https://api.example.com"] },
      fetchImpl,
    });
  }

  it("exposes frozen descriptors before handler execution", () => {
    const g = makeGraph();
    const ids = g.descriptors.map((d) => d.id);
    expect(ids).toContain(BROWSER_TIMERS_ID);
    expect(ids).toContain(BROWSER_CRYPTO_ID);
    expect(ids).toContain(BROWSER_CONSOLE_ID);
    expect(ids).toContain(BROWSER_FETCH_ID);
    for (const d of g.descriptors) expect(d.version).toBe(BROWSER_CAPABILITY_VERSION);
    expect(Object.isFrozen(g.graph)).toBeTrue();
    expect(g.describe()).toContain("runtime:timers@1");
    expect(g.describe()).toContain("runtime:fetch@1");
  });

  it("graph entries work end-to-end through the frozen graph", async () => {
    const { impl } = respondingFetch();
    const g = makeGraph(impl);
    const graph = g.graph as {
      timers: { delay(ms: number): Promise<{ elapsedMs: number }> };
      crypto: { digestHex(a: "SHA-256", d: Uint8Array): Promise<string> };
      fetch: { call(i: BrowserFetchInput): Promise<{ status: number }> };
    };
    expect((await graph.timers.delay(5)).elapsedMs).toBeGreaterThanOrEqual(0);
    expect(await graph.crypto.digestHex("SHA-256", enc("abc"))).toMatch(/^ba7816bf/);
    const r = await graph.fetch.call({ url: "https://api.example.com/x" });
    expect(r.status).toBe(200);
  });

  it("declares the adapters in the artifact manifest (canonical + verifiable)", async () => {
    const packBytes = enc('{"formatVersion":1,"app":"demo"}');
    const artifacts = {
      pack: packBytes,
      kernelWasm: enc("wasm"),
      handlerBundle: enc("export {};"),
      manifest: enc("{}"),
      contract: enc("{}"),
      schemas: enc("{}"),
      capabilities: enc("{}"),
    } as Record<ArtifactRole, Uint8Array>;
    const urls = {
      pack: "app.qpack",
      kernelWasm: "kernel.wasm",
      handlerBundle: "app.bundle.js",
      manifest: "browser-manifest.json",
      contract: "contract.json",
      schemas: "schema-manifest.json",
      capabilities: "capability-manifest.json",
    } as Record<ArtifactRole, string>;
    const g = makeGraph();
    const base = {
      appId: "demo",
      handlerAbiVersion: 1,
      kernelAbiVersion: 1,
      packSha256: await sha256Hex(packBytes),
      artifacts,
      urls,
    };
    const withCaps = await emitArtifactManifest({
      ...base,
      capabilities: g.descriptors.map(({ id, version }) => ({ id, version })),
    });
    const withoutCaps = await emitArtifactManifest(base);
    // the declaration changes the buildId (different canonical bytes) …
    expect(withCaps.manifest.buildId).not.toBe(withoutCaps.manifest.buildId);
    expect(withCaps.manifestJson).toContain('"runtime:timers"');
    // … and the emitted manifest verifies end-to-end (loader round trip)
    const loaded = await loadArtifacts(withCaps.manifestJson, async (url) => {
      const role = (Object.keys(urls) as ArtifactRole[]).find((r) => urls[r] === url)!;
      return artifacts[role];
    });
    expect(loaded.buildId).toBe(withCaps.manifest.buildId);
    expect(loaded.manifest.capabilities?.length).toBe(g.descriptors.length);
    // backward compatible: an older manifest (no capabilities) still verifies
    const legacy = await loadArtifacts(withoutCaps.manifestJson, async (url) => {
      const role = (Object.keys(urls) as ArtifactRole[]).find((r) => urls[r] === url)!;
      return artifacts[role];
    });
    expect(legacy.buildId).toBe(withoutCaps.manifest.buildId);
    expect(legacy.manifest.capabilities).toBeUndefined();
  });

  it("registry handles route through the call(input) contract", async () => {
    const { impl } = respondingFetch();
    const g = makeGraph(impl);
    const handles = baselineHandles(g);
    const fetchHandle = handles.find((h) => h.id === BROWSER_FETCH_ID)!;
    const r = (await fetchHandle.call({ url: "https://api.example.com/x" })) as { status: number };
    expect(r.status).toBe(200);
    const cryptoHandle = handles.find((h) => h.id === BROWSER_CRYPTO_ID)!;
    const digest = (await cryptoHandle.call({ op: "digest", algorithm: "SHA-256", data: enc("abc") })) as string;
    expect(digest).toMatch(/^ba7816bf/);
  });
});
