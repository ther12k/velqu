/**
 * Compiler conformance: static extraction guarantees, trap tests (COMP-002),
 * unsupported-import/duplicate/dynamic diagnostics (COMP-004/COMP-006),
 * deterministic rebuilds (COMP-003/COMP-009).
 */
import { describe, expect, test } from "bun:test";
import { build, contractDiff, diffContracts, CompileError } from "@velqu/compiler";
import { existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";

const TMP = "/tmp/velqu-conformance";

describe("compiler traps (COMP-002: never run the app)", () => {
  test("service factories and module side effects never execute during build", async () => {
    // The trap fixture's service factory THROWS if invoked; module scope
    // increments a global. Build must succeed without executing either.
    const out = `${TMP}/trap`;
    rmSync(out, { recursive: true, force: true });
    const r = await build({ project: "conformance/compiler/fixtures/trap-app.ts", outDir: out });
    expect(r.routes).toBe(1);
    // the compiler process itself never imported the fixture (pure AST) — if it
    // had, the module scope would have run here in THIS process too
    expect((globalThis as { __velquTrapSideEffects?: number }).__velquTrapSideEffects).toBeUndefined();
    // and the factory must not have run: the built bundle still contains it
    // (lazy), but never called — verified by the runtime-local suite
  });
});

describe("compiler diagnostics (source-located, actionable)", () => {
  test("unsupported node:/bun: imports fail the build (COMP-006)", async () => {
    await expectBuildFails("conformance/compiler/fixtures/bad-import-app.ts", /unsupported import 'node:fs'/);
  });
  test("canonically equivalent routes fail the build (COMP-004)", async () => {
    await expectBuildFails("conformance/compiler/fixtures/duplicate-app.ts", /route collision/);
  });
  test("dynamic route metadata fails with a hint (PR-004)", async () => {
    await expectBuildFails("conformance/compiler/fixtures/dynamic-app.ts", /literal/);
  });
});

describe("determinism (COMP-003/009)", () => {
  test("rebuild produces byte-identical pack and contract hash", async () => {
    const out1 = `${TMP}/det1`;
    const out2 = `${TMP}/det2`;
    rmSync(out1, { recursive: true, force: true });
    rmSync(out2, { recursive: true, force: true });
    await build({ project: "examples/proof/src/app.ts", outDir: out1 });
    await new Promise((r) => setTimeout(r, 20)); // timestamps differ — hashes must not
    await build({ project: "examples/proof/src/app.ts", outDir: out2 });
    const pack1 = readFileSync(`${out1}/app.qpack`, "utf8");
    const pack2 = readFileSync(`${out2}/app.qpack`, "utf8");
    const p1 = JSON.parse(pack1);
    const p2 = JSON.parse(pack2);
    expect(p1.integrity.routesSha256).toBe(p2.integrity.routesSha256);
    expect(p1.integrity.bundleSha256).toBe(p2.integrity.bundleSha256);
    expect(p1.contractHash).toBe(p2.contractHash);
    // M26-007-A: no wall-clock fields anywhere — EVERY build artifact is
    // byte-identical across clean builds (raw bytes, not parsed JSON).
    const { readdirSync } = await import("node:fs");
    const names = readdirSync(out1).sort();
    expect(names).toEqual(readdirSync(out2).sort());
    for (const name of names) {
      const a = readFileSync(`${out1}/${name}`);
      const b = readFileSync(`${out2}/${name}`);
      expect(a.equals(b), `${name} differs between clean builds`).toBe(true);
    }
  });
});

describe("pack contents (COMP-001/005)", () => {
  test("pack carries pre-compiled segments and versions", async () => {
    const pack = JSON.parse(readFileSync("examples/proof/dist/app.qpack", "utf8"));
    expect(pack.formatVersion).toBe(1);
    expect(pack.runtimeAbi).toBe(1);
    expect(pack.engine.version).toBe("0.15.1");
    expect(pack.engine.binding).toBe("rquickjs-0.12.2");
    for (const r of pack.routes) {
      expect(Array.isArray(r.pathSegments)).toBe(true);
      expect(r.pathSegments.length).toBeGreaterThan(0);
    }
    // health.live must be pre-compiled native liveness
    const health = pack.routes.find((r: { id: string }) => r.id === "health.live");
    expect(health.nativeLiveness.body).toBe('{"status":"ok"}');
  });

  test("compiles declared query fields into dense route IDs", async () => {
    const out = `${TMP}/query-ids`;
    rmSync(out, { recursive: true, force: true });
    await build({ project: "examples/proof/src/app.ts", outDir: out });
    const pack = JSON.parse(readFileSync(`${out}/app.qpack`, "utf8"));
    expect(pack.queryNameTable).toEqual(["ms"]);
    const timer = pack.routes.find((r: { id: string }) => r.id === "async.timer");
    expect(timer.plan.queryNameIds).toEqual([0]);
  });
});

async function expectBuildFails(project: string, pattern: RegExp) {
  const out = `${TMP}/fail-${Date.now()}`;
  try {
    await build({ project, outDir: out });
    throw new Error(`expected build to fail for ${project}`);
  } catch (e) {
    if (e instanceof Error && e.message.startsWith("expected build to fail")) throw e;
    expect(e).toBeInstanceOf(CompileError);
    expect((e as Error).message).toMatch(pattern);
    if (e instanceof CompileError && e.location) {
      expect(e.location.file.length).toBeGreaterThan(0);
      expect(e.location.line).toBeGreaterThan(0);
    }
  }
}

describe("contract lock workflow (PR-006/SCHEMA-007)", () => {
  test("lock is written once, preserved on rebuild, and diff detects drift", async () => {
    const out = "/tmp/velqu-conformance/lock";
    rmSync(out, { recursive: true, force: true });
    // first build writes the lock
    const b1 = await build({ project: "examples/proof/src/app.ts", outDir: out });
    expect(b1.lockPreserved).toBe(false);
    const lock1 = readFileSync(`${out}/contract.lock.json`, "utf8");
    // second build PRESERVES it (byte-identical)
    await new Promise((r) => setTimeout(r, 20));
    const b2 = await build({ project: "examples/proof/src/app.ts", outDir: out });
    expect(b2.lockPreserved).toBe(true);
    expect(readFileSync(`${out}/contract.lock.json`, "utf8")).toBe(lock1);
    // drift: remove a route from the CURRENT contract (as if the app changed)
    const contract = JSON.parse(readFileSync(`${out}/contract.json`, "utf8"));
    delete contract.routes["users.get"];
    writeFileSync(`${out}/contract.json`, JSON.stringify(contract));
    const diffs = contractDiff(out);
    const removed = diffs.find((d) => d.routeId === "users.get" && d.kind === "breaking");
    expect(removed).toBeDefined();
    // update-lock refreshes the baseline
    const b3 = await build({ project: "examples/proof/src/app.ts", outDir: out, updateLock: true });
    expect(b3.lockPreserved).toBe(false);
    expect(contractDiff(out).length).toBe(0);
  });

  test("semantic diff classifies IR v2 constraint changes (M25-008-D)", () => {
    const baseRoute = {
      path: "/items",
      method: "POST",
      body: {
        kind: "object",
        properties: {
          name: { kind: "string" },
          count: { kind: "integer", minimum: 0 },
          tags: { kind: "array", items: { kind: "string" } },
          grade: { kind: "enum", values: ["a", "b", "c"] },
        },
        required: ["name"],
      },
      responses: { "200": { kind: "object", properties: { ok: { kind: "boolean" } } } },
    };
    const lock = { routes: { "items.create": baseRoute } };
    const variant = (body: unknown) => ({
      routes: { "items.create": { ...baseRoute, body } },
    });

    // input maxLength added -> breaking (previously-valid input rejects)
    const d1 = diffContracts(
      variant({
        ...(baseRoute.body as object),
        properties: { ...(baseRoute.body as { properties: object }).properties, name: { kind: "string", maxLength: 8 } },
      }),
      lock,
    );
    expect(d1.some((d) => d.kind === "breaking" && d.change.includes("bounds tightened"))).toBe(true);

    // input minimum lowered (0 → -5) -> compatible (accepts more)
    const d2 = diffContracts(
      variant({
        ...(baseRoute.body as object),
        properties: { ...(baseRoute.body as { properties: object }).properties, count: { kind: "integer", minimum: -5 } },
      }),
      lock,
    );
    expect(d2.some((d) => d.kind === "compatible" && d.change.includes("bounds loosened"))).toBe(true);

    // pattern constraint added -> breaking on input
    const d3 = diffContracts(
      variant({
        ...(baseRoute.body as object),
        properties: { ...(baseRoute.body as { properties: object }).properties, name: { kind: "string", pattern: "^it_" } },
      }),
      lock,
    );
    expect(d3.some((d) => d.kind === "breaking" && d.change.includes("pattern constraint added"))).toBe(true);

    // enum value removed on input -> breaking
    const d4 = diffContracts(
      variant({
        ...(baseRoute.body as object),
        properties: { ...(baseRoute.body as { properties: object }).properties, grade: { kind: "enum", values: ["a", "b"] } },
      }),
      lock,
    );
    expect(d4.some((d) => d.kind === "breaking" && d.change.includes("enum value(s) removed"))).toBe(true);

    // minItems added on input array -> breaking
    const d5 = diffContracts(
      variant({
        ...(baseRoute.body as object),
        properties: { ...(baseRoute.body as { properties: object }).properties, tags: { kind: "array", items: { kind: "string" }, minItems: 1 } },
      }),
      lock,
    );
    expect(d5.some((d) => d.kind === "breaking" && d.change.includes("bounds tightened"))).toBe(true);

    // response bounds tightened -> policy-sensitive (not breaking)
    const respLock = {
      routes: {
        "items.create": {
          ...baseRoute,
          responses: { "200": { kind: "object", properties: { token: { kind: "string" } } } },
        },
      },
    };
    const d6 = diffContracts(
      {
        routes: {
          "items.create": {
            ...baseRoute,
            responses: { "200": { kind: "object", properties: { token: { kind: "string", maxLength: 64 } } } },
          },
        },
      },
      respLock,
    );
    expect(d6.some((d) => d.kind === "policy-sensitive" && d.change.includes("bounds tightened"))).toBe(true);

    // fallback reason change -> policy-sensitive (codec path change)
    const fbLock = {
      routes: {
        "items.create": {
          ...baseRoute,
          body: { kind: "fallback", reason: "explicit", inner: baseRoute.body },
        },
      },
    };
    const d7 = diffContracts(
      {
        routes: {
          "items.create": {
            ...baseRoute,
            body: { kind: "fallback", reason: "measured", inner: baseRoute.body },
          },
        },
      },
      fbLock,
    );
    expect(d7.some((d) => d.kind === "policy-sensitive" && d.change.includes("fallback reason changed"))).toBe(true);
  });

  test("semantic diff detects schema structural changes accurately", () => {
    const lock = {
      routes: {
        "users.create": {
          path: "/users",
          method: "POST",
          body: {
            kind: "object",
            properties: {
              name: { kind: "string" },
              email: { kind: "string" },
            },
            required: ["name", "email"],
          },
          responses: {
            "201": {
              kind: "object",
              properties: { id: { kind: "string" }, name: { kind: "string" } },
              required: ["id", "name"],
            },
          },
        },
      },
    };

    // Case 1: Added required input field -> BREAKING
    const addedRequired = {
      routes: {
        "users.create": {
          ...lock.routes["users.create"],
          body: {
            kind: "object",
            properties: {
              name: { kind: "string" },
              email: { kind: "string" },
              role: { kind: "string" },
            },
            required: ["name", "email", "role"],
          },
        },
      },
    };
    const diff1 = diffContracts(addedRequired, lock);
    expect(diff1.some((d) => d.kind === "breaking" && d.change.includes("required field added"))).toBe(true);

    // Case 2: Added optional input field -> COMPATIBLE
    const addedOptional = {
      routes: {
        "users.create": {
          ...lock.routes["users.create"],
          body: {
            kind: "object",
            properties: {
              name: { kind: "string" },
              email: { kind: "string" },
              nickname: { kind: "string" },
            },
            required: ["name", "email"],
          },
        },
      },
    };
    const diff2 = diffContracts(addedOptional, lock);
    expect(diff2.some((d) => d.kind === "compatible" && d.change.includes("optional field added"))).toBe(true);

    // Case 3: Removed field from response -> BREAKING
    const removedRespField = {
      routes: {
        "users.create": {
          ...lock.routes["users.create"],
          responses: {
            "201": {
              kind: "object",
              properties: { id: { kind: "string" } },
              required: ["id"],
            },
          },
        },
      },
    };
    const diff3 = diffContracts(removedRespField, lock);
    expect(diff3.some((d) => d.kind === "breaking" && d.change.includes("response field removed"))).toBe(true);
  });
});
describe("canonicalization (M25-001-C: order-insensitive hashing)", () => {
  test("option literal field order never changes canonical hashes", async () => {
    const outA = `${TMP}/order-a`;
    const outB = `${TMP}/order-b`;
    rmSync(outA, { recursive: true, force: true });
    rmSync(outB, { recursive: true, force: true });
    await build({ project: "conformance/compiler/fixtures/order-a-app.ts", outDir: outA });
    await build({ project: "conformance/compiler/fixtures/order-b-app.ts", outDir: outB });
    const a = JSON.parse(readFileSync(`${outA}/app.qpack`, "utf8"));
    const b = JSON.parse(readFileSync(`${outB}/app.qpack`, "utf8"));
    // same canonical execution graph and public contract despite reversed
    // option literal order in every schema
    expect(a.integrity.routesSha256).toBe(b.integrity.routesSha256);
    expect(a.contractHash).toBe(b.contractHash);
    // the bundle source text does differ; only canonical forms are stable
    expect(a.integrity.bundleSha256).not.toBe(b.integrity.bundleSha256);
  });
});

describe("strategy selection (M25-002-D: evidence-driven codec selection)", () => {
  test("standard representable routes select native validation and response strategies", async () => {
    const out = `${TMP}/strat-std`;
    rmSync(out, { recursive: true, force: true });
    await build({ project: "examples/proof/src/app.ts", outDir: out });
    const pack = JSON.parse(readFileSync(`${out}/app.qpack`, "utf8"));
    const report = JSON.parse(readFileSync(`${out}/build-report.json`, "utf8"));

    expect(report.strategies.validation).toBe("native");
    expect(report.strategies.responses).toBe("native");
    expect(report.strategies.fallbacks).toEqual([]);
    expect(report.strategies.decisions.length).toBe(pack.routes.length);
    for (const d of report.strategies.decisions) {
      expect(d.validationStrategy).toBe("native");
      expect(d.responseStrategy).toBe("native");
    }
    for (const r of pack.routes) {
      expect(r.validationStrategy).toBe("native");
      expect(r.plan.responseStrategy).toBe("native");
    }
  });

  test("explicit fallback nodes select js strategy and record estimated overhead", async () => {
    const out = `${TMP}/strat-fallback`;
    rmSync(out, { recursive: true, force: true });
    await build({ project: "conformance/compiler/fixtures/fallback-app.ts", outDir: out });
    const pack = JSON.parse(readFileSync(`${out}/app.qpack`, "utf8"));
    const report = JSON.parse(readFileSync(`${out}/build-report.json`, "utf8"));

    expect(report.strategies.validation).toBe("hybrid");
    expect(report.strategies.responses).toBe("hybrid");
    expect(report.strategies.fallbacks.length).toBe(3);

    const bodyFb = report.strategies.fallbacks.find((f: { route: string }) => f.route === "fb.body");
    expect(bodyFb).toBeDefined();
    expect(bodyFb.location).toBe("body");
    expect(bodyFb.reason).toBe("unsupported-transform");
    expect(bodyFb.strategy).toBe("js");
    expect(bodyFb.estimatedOverheadUs).toBeGreaterThan(0);

    const respFb = report.strategies.fallbacks.find((f: { route: string }) => f.route === "fb.resp");
    expect(respFb).toBeDefined();
    expect(respFb.location).toBe("response.200");
    expect(respFb.reason).toBe("measured");
    expect(respFb.strategy).toBe("js");
    expect(respFb.estimatedOverheadUs).toBeGreaterThan(0);

    const queryFb = report.strategies.fallbacks.find((f: { route: string }) => f.route === "fb.query");
    expect(queryFb).toBeDefined();
    expect(queryFb.location).toBe("query");
    expect(queryFb.reason).toBe("explicit");
    expect(queryFb.strategy).toBe("js");
    expect(queryFb.estimatedOverheadUs).toBeGreaterThan(0);

    // Verify pack route plan flags match strategy selection
    const fbBodyRoute = pack.routes.find((r: { id: string }) => r.id === "fb.body");
    expect(fbBodyRoute.validationStrategy).toBe("js");

    const fbRespRoute = pack.routes.find((r: { id: string }) => r.id === "fb.resp");
    expect(fbRespRoute.plan.responseStrategy).toBe("js");

    const stdRoute = pack.routes.find((r: { id: string }) => r.id === "std.get");
    expect(stdRoute.validationStrategy).toBe("native");
    expect(stdRoute.plan.responseStrategy).toBe("native");
  });

  test("fallback reasons are tagged in the RoutePlan (M25-007-A)", async () => {
    const out = `${TMP}/strat-tags`;
    rmSync(out, { recursive: true, force: true });
    await build({ project: "conformance/compiler/fixtures/fallback-app.ts", outDir: out });
    const pack = JSON.parse(readFileSync(`${out}/app.qpack`, "utf8"));

    const byId = (id: string) => pack.routes.find((r: { id: string }) => r.id === id);

    // js validation carries its reason from the closed vocabulary
    const fbBody = byId("fb.body");
    expect(fbBody.plan.validationFallbackReason).toBe("unsupported-transform");
    expect(fbBody.plan.responseFallbackReason).toBeUndefined();

    // js response strategy carries its reason (measured marker)
    const fbResp = byId("fb.resp");
    expect(fbResp.plan.responseFallbackReason).toBe("measured");
    expect(fbResp.plan.validationFallbackReason).toBeUndefined();

    // native routes carry no tags (fallback never activates silently,
    // native never pretends to be fallback)
    const std = byId("std.get");
    expect(std.plan.validationFallbackReason).toBeUndefined();
    expect(std.plan.responseFallbackReason).toBeUndefined();
  });

  test("pack carries the full runtime fingerprint (M26-002-A)", async () => {
    const out = `${TMP}/fingerprint`;
    rmSync(out, { recursive: true, force: true });
    await build({ project: "examples/proof", outDir: out });
    const pack = JSON.parse(readFileSync(`${out}/app.qpack`, "utf8"));

    // engine fingerprint: rquickjs version + runtime build hash
    expect(pack.engine.rquickjs).toBe("0.12.2");
    expect(pack.engine.buildHash).toMatch(/^[0-9a-f]{64}$/);

    // capability hash: sha256 over the sorted, newline-joined names
    const caps = [...pack.capabilities].sort();
    const expected = new Bun.CryptoHasher("sha256").update(caps.join("\n")).digest("hex");
    expect(pack.capabilityHash).toBe(expected);

    // the fingerprint the runtime verifies must agree with this build —
    // proven by the pack actually loading (runtime-local suites) and the
    // identical constants asserted here
    expect(pack.engine.version).toBe("0.15.1");
    expect(pack.engine.binding).toBe("rquickjs-0.12.2");
    expect(pack.runtimeAbi).toBe(1);
  });

  test("route manifest exposes codec choice and bridge crossings (M25-007-D)", async () => {
    const out = `${TMP}/inspect-codecs`;
    rmSync(out, { recursive: true, force: true });
    await build({ project: "conformance/compiler/fixtures/fallback-app.ts", outDir: out });
    const manifest = JSON.parse(readFileSync(`${out}/route-manifest.json`, "utf8"));
    const byId = (id: string) => manifest.find((r: { id: string }) => r.id === id);

    // fallback routes: real strategies (not hardcoded native), reasons,
    // generic codecs, and the lazy bridge model
    const fbBody = byId("fb.body");
    expect(fbBody.validationStrategy).toBe("js");
    expect(fbBody.validationFallbackReason).toBe("unsupported-transform");
    expect(fbBody.validationCodec).toBe("generic-fallback");
    expect(fbBody.bridge).toBe("lazy-per-field");

    const fbResp = byId("fb.resp");
    expect(fbResp.responseStrategy).toBe("js");
    expect(fbResp.responseFallbackReason).toBe("measured");
    expect(fbResp.responseCodec).toBe("engine-stringify");
    expect(fbResp.bridge).toBe("lazy-per-field");

    // native route: direct codecs, single pre-validated crossing, no reasons
    const std = byId("std.get");
    expect(std.validationStrategy).toBe("native");
    expect(std.validationFallbackReason).toBeNull();
    expect(std.validationCodec).toBe("direct-decoder");
    expect(std.responseCodec).toBe("direct-encoder");
    expect(std.bridge).toBe("single-prevalidated");

    // inspect snapshot: the CLI renders the same facts per route
    const proc = Bun.spawnSync([
      "bun",
      "packages/cli/src/index.ts",
      "inspect",
      "routes",
      "--dist",
      out,
    ]);
    const text = new TextDecoder().decode(proc.stdout);
    expect(text).toContain("std.get");
    expect(text).toContain("val=native resp=native codec=direct-decoder/direct-encoder bridge=single-prevalidated");
    expect(text).toContain("fb.body");
    expect(text).toContain("val=js(unsupported-transform)");
    expect(text).toContain("bridge=lazy-per-field");
    expect(text).toContain("fb.resp");
    expect(text).toContain("resp=js(measured)");
    expect(text).toContain("codec=direct-decoder/engine-stringify");
  });

  test("strategy decisions are deterministic across repeated builds", async () => {
    const out1 = `${TMP}/strat-det1`;
    const out2 = `${TMP}/strat-det2`;
    rmSync(out1, { recursive: true, force: true });
    rmSync(out2, { recursive: true, force: true });
    await build({ project: "conformance/compiler/fixtures/fallback-app.ts", outDir: out1 });
    await build({ project: "conformance/compiler/fixtures/fallback-app.ts", outDir: out2 });

    const rep1 = JSON.parse(readFileSync(`${out1}/build-report.json`, "utf8"));
    const rep2 = JSON.parse(readFileSync(`${out2}/build-report.json`, "utf8"));
    expect(rep1.strategies).toEqual(rep2.strategies);
  });
});

describe("problem contracts (M25-006-C: policy errors flow into Treaty unions)", () => {
  test("policy 401 and 404 problems emit exact envelopes in contract.d.ts", async () => {
    const out = `${TMP}/probcon`;
    rmSync(out, { recursive: true, force: true });
    await build({ project: "examples/proof", outDir: out });
    const dts = readFileSync(`${out}/contract.d.ts`, "utf8");

    // policy-provided 401 on users.get: exact literals, status narrows
    const resp = dts.slice(dts.indexOf('"users.get"'));
    expect(resp).toContain('401: { type: "https://velqu.dev/problems/unauthorized"; title: "Unauthorized"; status: 401; instance: string; detail?: string }');
  });

  test("declared 404 emits the exact not-found envelope (d.ts + OpenAPI)", async () => {
    const out = `${TMP}/probapp`;
    rmSync(out, { recursive: true, force: true });
    await build({ project: "conformance/compiler/fixtures/problem-app.ts", outDir: out });
    const dts = readFileSync(`${out}/contract.d.ts`, "utf8");
    expect(dts).toContain('404: { type: "https://velqu.dev/problems/not-found"; title: "Not Found"; status: 404; instance: string; detail?: string }');

    const openapi = JSON.parse(readFileSync(`${out}/openapi.json`, "utf8"));
    const gone404 = openapi.paths["/gone"].get.responses["404"];
    expect(gone404.description).toBe("problem: not-found");
    const schema = gone404.content["application/problem+json"].schema;
    expect(schema.required).toEqual(["type", "title", "status", "instance"]);
    expect(schema.properties.type.enum).toEqual(["https://velqu.dev/problems/not-found"]);
    expect(schema.properties.title.enum).toEqual(["Not Found"]);
    expect(schema.properties.status.enum).toEqual([404]);
  });

  test("policy 401 OpenAPI schema matches the runtime envelope", async () => {
    const out = `${TMP}/probcon`;
    const openapi = JSON.parse(readFileSync(`${out}/openapi.json`, "utf8"));
    const usersGet401 = openapi.paths["/users/{id}"].get.responses["401"];
    expect(usersGet401.description).toBe("problem: unauthorized");
    const schema = usersGet401.content["application/problem+json"].schema;
    expect(schema.required).toEqual(["type", "title", "status", "instance"]);
    expect(schema.properties.type.enum).toEqual(["https://velqu.dev/problems/unauthorized"]);
    expect(schema.properties.title.enum).toEqual(["Unauthorized"]);
    expect(schema.properties.status.enum).toEqual([401]);
  });

  test("PROBLEM_REGISTRY mirrors the runtime registry ids", async () => {
    const { PROBLEM_REGISTRY } = await import("@velqu/compiler");
    expect(Object.keys(PROBLEM_REGISTRY).sort()).toEqual(
      [
        "validation",
        "unauthorized",
        "not-found",
        "method",
        "body",
        "limit",
        "timeout",
        "overload",
        "internal",
      ].sort(),
    );
  });
});

describe("debug source sidecar (ADR-0027, M26-004-C)", () => {
  test("production pack embeds no source map; sidecar carries sources bound to the pack hash", async () => {
    const out = `${TMP}/sidecar`;
    rmSync(out, { recursive: true, force: true });
    await build({ project: "examples/proof/src/app.ts", outDir: out });

    const packRaw = readFileSync(`${out}/app.qpack`, "utf8");
    const pack = JSON.parse(packRaw);
    // production default: the pack ships debug-free
    expect(pack.sourceMap ?? null).toBeNull();

    // the sidecar exists next to the pack and binds to its exact bytes
    const sidecarPath = `${out}/app.qpack.sources.json`;
    expect(existsSync(sidecarPath)).toBe(true);
    const sidecar = JSON.parse(readFileSync(sidecarPath, "utf8"));
    expect(sidecar.formatVersion).toBe(1);
    const { createHash } = await import("node:crypto");
    const want = createHash("sha256").update(packRaw).digest("hex");
    expect(sidecar.packSha256).toBe(want);
    // sources live in the sidecar, not the pack
    expect(typeof sidecar.bundleSource).toBe("string");
    expect(sidecar.bundleSource).toContain("__velquFunctionManifest");
    expect(Array.isArray(sidecar.modules)).toBe(true);
    expect(sidecar.modules.length).toBeGreaterThan(0);
  });
});
