/**
 * Builds velqu benchmark packs from the frozen fixture contract:
 *  - fixture pack (9 routes)  → examples/proof/dist/app.qpack
 *  - route-count packs (N)    → benchmarks/raw/packs/app-N.qpack
 * Deterministic: identical inputs → byte-identical output (COMP-009).
 */
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { featuresOf } from "@velqu/schema";

const sha = (s: string) => createHash("sha256").update(s).digest("hex");

interface Route {
  id: string;
  method: string;
  path: string;
  segments: Array<{ kind: "static" | "param" | "wildcard"; value: string }>;
  handler: string;
  policy?: string;
  params?: { schema: string; coerce: string };
  query?: { schema: string; coerce: string };
  body?: { schema: string; contentType: string; limitBytes: number };
  responses: Record<string, { strategy: "native" | "js"; problem?: string }>;
  nativeLiveness?: { status: number; contentType: string; body: string };
  security?: Array<{ scheme: string; header: string; problemStatus: number }>;
  capabilities?: string[];
}

const seg = (...parts: Array<[string, string]>): Route["segments"] =>
  parts.map(([kind, value]) => ({ kind: kind as "static", value }));

const JSON_HEADERS_200 = { "200": { schema: null, strategy: "js" as const, problem: null } };

function fixtureRoutes(): Route[] {
  return [
    {
      id: "health.live",
      method: "GET",
      path: "/health/live",
      segments: seg(["static", "health"], ["static", "live"]),
      handler: "health.live",
      responses: { ...JSON_HEADERS_200 },
      nativeLiveness: { status: 200, contentType: "application/json", body: '{"status":"ok"}' },
    },
    {
      id: "js.text",
      method: "GET",
      path: "/js-text",
      segments: seg(["static", "js-text"]),
      handler: "js.text",
      responses: { ...JSON_HEADERS_200 },
    },
    {
      id: "js.json",
      method: "GET",
      path: "/js-json",
      segments: seg(["static", "js-json"]),
      handler: "js.json",
      responses: { ...JSON_HEADERS_200 },
    },
    {
      id: "hello.get",
      method: "GET",
      path: "/hello/:name",
      segments: seg(["static", "hello"], ["param", "name"]),
      handler: "hello.get",
      params: { schema: "sch:hello.params", coerce: "path" },
      responses: { "200": { schema: null, strategy: "js", problem: null }, "422": { schema: null, strategy: "js", problem: "validation" } },
    },
    {
      id: "users.create",
      method: "POST",
      path: "/users",
      segments: seg(["static", "users"]),
      handler: "users.create",
      body: { schema: "sch:users.create.body", contentType: "application/json", limitBytes: 65536 },
      responses: { "201": { schema: null, strategy: "js", problem: null }, "422": { schema: null, strategy: "js", problem: "validation" } },
    },
    {
      id: "users.get",
      method: "GET",
      path: "/users/:id",
      segments: seg(["static", "users"], ["param", "id"]),
      handler: "users.get",
      policy: "auth.session",
      params: { schema: "sch:users.get.params", coerce: "path" },
      responses: {
        "200": { schema: null, strategy: "js", problem: null },
        "401": { schema: null, strategy: "js", problem: "unauthorized" },
        "404": { schema: null, strategy: "js", problem: "not-found" },
      },
      security: [{ scheme: "bearer", header: "authorization", problemStatus: 401 }],
    },
    {
      id: "async.timer",
      method: "GET",
      path: "/async",
      segments: seg(["static", "async"]),
      handler: "async.timer",
      query: { schema: "sch:async.query", coerce: "query" },
      responses: { ...JSON_HEADERS_200 },
      capabilities: ["timer"],
    },
    {
      id: "async.cancel",
      method: "GET",
      path: "/cancel",
      segments: seg(["static", "cancel"]),
      handler: "async.cancel",
      query: { schema: "sch:cancel.query", coerce: "query" },
      responses: { ...JSON_HEADERS_200 },
      capabilities: ["timer"],
    },
    {
      id: "throw.redacted",
      method: "GET",
      path: "/throw",
      segments: seg(["static", "throw"]),
      handler: "throw.redacted",
      responses: { ...JSON_HEADERS_200 },
    },
  ];
}

const FIXTURE_BUNDLE = String.raw`
"use strict";
var __users = null;
var __nextUser = 1;
function users() {
  if (__users === null) {
    __users = new Map();
    __users.set("usr_1", { id: "usr_1", name: "Ada", email: "ada@example.org" });
  }
  return __users;
}
function health_live() { return { status: "ok" }; }
function js_text() { return "plain"; }
function js_json() { return { ok: true }; }
function hello_get(ctx) { return { message: "Hello " + ctx.params.name }; }
function users_create(ctx) {
  const id = "usr_" + (__nextUser++);
  const u = { id, name: ctx.body.name, email: ctx.body.email };
  users().set(id, u);
  return { __ok: true, status: 201, value: u };
}
function users_get(ctx) {
  const u = users().get(ctx.params.id);
  if (!u) return { __problem: true, problem: "not-found", status: 404, detail: "user not found" };
  return u;
}
function auth_session(req) {
  const token = req.headers.authorization;
  if (token !== "Bearer q-demo-token") {
    return { __problem: true, problem: "unauthorized", status: 401 };
  }
  return { session: { userId: "usr_1" } };
}
async function async_timer(ctx) {
  const waited = await ctx.native.timer.delay(ctx.query.ms);
  return { waited };
}
async function async_cancel(ctx) {
  const waited = await ctx.native.timer.delay(ctx.query.ms);
  return { cancelled: false, waited };
}
async function throw_redacted() { throw new Error("secret-boom"); }
globalThis.__velquFunctionManifest = [
  ["health.live", 0, health_live],
  ["js.text", 0, js_text],
  ["js.json", 0, js_json],
  ["hello.get", 0, hello_get],
  ["users.create", 0, users_create],
  ["users.get", 0, users_get],
  ["async.timer", 0, async_timer],
  ["async.cancel", 0, async_cancel],
  ["throw.redacted", 0, throw_redacted],
  ["auth.session", 1, auth_session]
];
globalThis.__velquFunctions = globalThis.__velquFunctionManifest.map(function (e) { return e[2]; });
`;

const FIXTURE_SCHEMAS: Record<string, unknown> = {
  "sch:hello.params": {
    kind: "object",
    properties: { name: { kind: "string", minLength: 1, maxLength: 60 } },
    required: ["name"],
  },
  "sch:users.create.body": {
    kind: "object",
    properties: {
      name: { kind: "string", minLength: 1, maxLength: 60 },
      email: { kind: "string", format: "email" },
    },
    required: ["name", "email"],
  },
  "sch:users.get.params": {
    kind: "object",
    properties: { id: { kind: "string", pattern: "^usr_[0-9]+$" } },
    required: ["id"],
  },
  "sch:async.query": {
    kind: "object",
    properties: {
      ms: { kind: "optional", inner: { kind: "integer", minimum: 1, maximum: 1000 }, default: 10 },
    },
    required: [],
  },
  "sch:cancel.query": {
    kind: "object",
    properties: {
      ms: { kind: "optional", inner: { kind: "integer", minimum: 1, maximum: 5000 }, default: 1000 },
    },
    required: [],
  },
};

function routeEntry(r: Route, moduleId: string, routeIdx: number = 0, schemaKeyToId: Map<string, number> = new Map()) {
  const defaultStatus = Object.keys(r.responses).includes("200")
    ? 200
    : Number(Object.keys(r.responses)[0] ?? 200);
  const allowedStatuses = Object.keys(r.responses)
    .map(Number)
    .filter((n) => !isNaN(n))
    .sort((a, b) => a - b);
  const responseStrategy = r.responses["200"]?.strategy ?? (Object.values(r.responses)[0]?.strategy ?? "js");

  const plan = {
    routeId: routeIdx,
    handlerId: routeIdx,
    policyId: r.policy === "auth.session" ? 0 : null,
    policyHandlerId: r.policy === "auth.session" ? 9 : null,
    paramsSchemaId: r.params ? (schemaKeyToId.get(r.params.schema) ?? null) : null,
    querySchemaId: r.query ? (schemaKeyToId.get(r.query.schema) ?? null) : null,
    headersSchemaId: null,
    bodySchemaId: r.body ? (schemaKeyToId.get(r.body.schema) ?? null) : null,
    headerNameIds: [],
    queryNameIds: [],
    cookieNameIds: [],
    defaultStatus,
    allowedStatuses,
    fieldNeeds: {
      params: r.params != null,
      query: r.query != null,
      headers: r.policy != null,
      body: r.body != null,
    },
    responseStrategy,
    // M25-007-A: js plan strategies must carry a fallback reason from the
    // closed vocabulary — pack verification rejects silent fallbacks
    ...(responseStrategy === "js" ? { responseFallbackReason: "explicit" } : {}),
    deadlineMs: 5000,
  };

  return {
    id: r.id,
    moduleId,
    method: r.method,
    path: r.path,
    pathSegments: r.segments,
    handler: r.handler,
    policy: r.policy ?? null,
    params: r.params ? { schema: r.params.schema, coerce: r.params.coerce, contentType: null, limitBytes: 0 } : null,
    query: r.query ? { schema: r.query.schema, coerce: r.query.coerce, contentType: null, limitBytes: 0 } : null,
    body: r.body
      ? { schema: r.body.schema, coerce: null, contentType: r.body.contentType, limitBytes: r.body.limitBytes }
      : null,
    headers: null,
    responses: r.responses,
    validationStrategy: "native",
    nativeLiveness: r.nativeLiveness ?? null,
    security: r.security ?? [],
    capabilities: r.capabilities ?? [],
    deadlineMs: 5000,
    plan,
  };
}

function buildSerializedRouter(routes: Array<{ method: string; pathSegments: Array<{ kind: string; value: string }> }>) {
  const methodMap: Record<string, number> = {
    GET: 0,
    POST: 1,
    PUT: 2,
    PATCH: 3,
    DELETE: 4,
    OPTIONS: 5,
    HEAD: 6,
  };
  const nodes: Array<{
    staticEdges: Array<{ segment: string; targetNode: number }>;
    paramEdge: number | null;
    wildcardEdge: number | null;
    terminal: { methodMask: number; routeByMethod: Array<number | null> } | null;
  }> = [
    { staticEdges: [], paramEdge: null, wildcardEdge: null, terminal: null },
  ];

  for (let rIdx = 0; rIdx < routes.length; rIdx++) {
    const r = routes[rIdx];
    let curr = 0;
    for (const seg of r.pathSegments) {
      if (seg.kind === "static") {
        const existing = nodes[curr].staticEdges.find((e) => e.segment === seg.value);
        if (existing) {
          curr = existing.targetNode;
        } else {
          const next = nodes.length;
          nodes.push({ staticEdges: [], paramEdge: null, wildcardEdge: null, terminal: null });
          nodes[curr].staticEdges.push({ segment: seg.value, targetNode: next });
          curr = next;
        }
      } else if (seg.kind === "param") {
        if (nodes[curr].paramEdge !== null) {
          curr = nodes[curr].paramEdge!;
        } else {
          const next = nodes.length;
          nodes.push({ staticEdges: [], paramEdge: null, wildcardEdge: null, terminal: null });
          nodes[curr].paramEdge = next;
          curr = next;
        }
      } else if (seg.kind === "wildcard") {
        if (nodes[curr].wildcardEdge !== null) {
          curr = nodes[curr].wildcardEdge!;
        } else {
          const next = nodes.length;
          nodes.push({ staticEdges: [], paramEdge: null, wildcardEdge: null, terminal: null });
          nodes[curr].wildcardEdge = next;
          curr = next;
        }
      }
    }
    if (!nodes[curr].terminal) {
      nodes[curr].terminal = {
        methodMask: 0,
        routeByMethod: [null, null, null, null, null, null, null],
      };
    }
    const mIdx = methodMap[r.method.toUpperCase()] ?? 0;
    nodes[curr].terminal!.methodMask |= 1 << mIdx;
    nodes[curr].terminal!.routeByMethod[mIdx] = rIdx;
  }
  return { nodes };
}

function buildPack(routes: Route[], schemas: Record<string, unknown>, bundle: string, appId: string) {
  const sortedSchemaKeys = Object.keys(schemas).sort();
  const schemaKeyToId = new Map<string, number>();
  const schemaManifest: Array<{ id: number; key: string; features: string[]; ir: unknown }> = [];
  for (let i = 0; i < sortedSchemaKeys.length; i++) {
    const k = sortedSchemaKeys[i];
    schemaKeyToId.set(k, i);
    schemaManifest.push({
      id: i,
      key: k,
      features: featuresOf(schemas[k] as Parameters<typeof featuresOf>[0]),
      ir: sortIR(schemas[k]),
    });
  }

  const packRoutes = routes.map((r, i) => routeEntry(r, r.id.split(".")[0], i, schemaKeyToId));

  const functions = [
    ...routes.map((r, i) => ({
      id: i,
      key: r.handler,
      kind: "route-handler",
    })),
    ...(routes.some((r) => r.policy === "auth.session")
      ? [{ id: routes.length, key: "auth.session", kind: "policy-handler" }]
      : []),
  ];
  const policyManifest = routes.some((r) => r.policy === "auth.session")
    ? [{ id: 0, key: "auth.session", handlerId: routes.length }]
    : [];

  const router = buildSerializedRouter(packRoutes);

  // MUST match the Rust canonical form byte-for-byte (q-pack
  // routes_canonical_json, M25-001-C): whole view through sortIR.
  const canonical = JSON.stringify(
    sortIR({
      routes: packRoutes,
      schemas,
      policies: routes.some((r) => r.policy === "auth.session")
        ? { "auth.session": { id: "auth.session", handler: "auth.session", declaredStatuses: [401], provides: "session" } }
        : {},
      capabilities: routes.some((r) => r.capabilities?.includes("timer")) ? ["timer"] : [],
      functions,
      schemaManifest,
      policyManifest,
      router,
    }),
  );

  const projectBinding = (b: { schema: string | null; coerce: string | null; contentType: string | null; limitBytes: number } | null) =>
    b ? { schema: b.schema, coerce: b.coerce, contentType: b.contentType, limitBytes: b.limitBytes } : null;
  const usedSchemas = new Set<string>();
  const publicRoutes = packRoutes.map((r) => {
    const responsesSorted: Record<string, unknown> = {};
    for (const k of Object.keys(r.responses).sort()) {
      const d = r.responses[k];
      if (d.schema) usedSchemas.add(d.schema);
      responsesSorted[k] = { schema: d.schema, problem: d.problem };
    }
    for (const b of [r.params, r.query, r.headers, r.body]) {
      if (b?.schema) usedSchemas.add(b.schema);
    }
    return {
      id: r.id,
      method: r.method,
      path: r.path,
      params: projectBinding(r.params),
      query: projectBinding(r.query),
      headers: projectBinding(r.headers),
      body: projectBinding(r.body),
      responses: responsesSorted,
      security: r.security,
    };
  });
  const schemasPublic: Record<string, unknown> = {};
  for (const k of Object.keys(schemas).sort()) {
    if (usedSchemas.has(k)) schemasPublic[k] = sortIR(schemas[k]);
  }
  const policiesPublic = routes.some((r) => r.policy === "auth.session")
    ? { "auth.session": { declaredStatuses: [401], provides: "session" } }
    : {};
  const publicContractCanonical = JSON.stringify(sortIR([publicRoutes, schemasPublic, policiesPublic]));
  const contractHash = sha(publicContractCanonical).slice(0, 32);

  const pack = {
    formatVersion: 1,
    kind: "velqu.qpack",
    runtimeAbi: 1,
    executionMode: "numeric",
    engine: { name: "quickjs-ng", version: "0.15.1", binding: "rquickjs-0.12.2" },
    schemaIrVersion: 2,
    contractVersion: 1,
    contractHash,
    builtBy: { compiler: "bench-fixture-0.1.0", typescript: Bun.version, bun: Bun.version },
    appId,
    modules: [...new Set(routes.map((r) => r.id.split(".")[0]))],
    entry: "app.js",
    bundle,
    sourceMap: null,
    routes: packRoutes,
    schemas: sortIR(schemas) as Record<string, unknown>,
    policies: routes.some((r) => r.policy === "auth.session")
      ? { "auth.session": { id: "auth.session", handler: "auth.session", declaredStatuses: [401], provides: "session" } }
      : {},
    capabilities: routes.some((r) => r.capabilities?.includes("timer")) ? ["timer"] : [],
    functions,
    schemaManifest,
    policyManifest,
    router,
    integrity: { algorithm: "sha256", bundleSha256: sha(bundle), routesSha256: sha(canonical) },
  };
  return JSON.stringify(pack);
}

/**
 * Sort map-shaped levels to match Rust's BTreeMap-backed serialization:
 * schemas/policies/properties maps sort by key; ordered arrays stay as-is;
 * struct-shaped values keep field order.
 */
/**
 * Rust-canonical sortIR (M25-001-C): every object's keys sorted recursively;
 * arrays keep order. Mirrors q_schema_runtime::canonical_value so the fixture
 * pack's integrity hashes match the runtime byte-for-byte.
 */
function sortIR(v: unknown): unknown {
  if (Array.isArray(v)) return v.map(sortIR);
  if (v && typeof v === "object") {
    const o = v as Record<string, unknown>;
    const out: Record<string, unknown> = {};
    for (const k of Object.keys(o).sort()) out[k] = sortIR(o[k]);
    return out;
  }
  return v;
}

// ---------------------------------------------------------------- route-count packs

function generatedRoutes(n: number): { routes: Route[]; bundle: string; schemas: Record<string, unknown> } {
  const routes: Route[] = [fixtureRoutes()[0]]; // keep health/live for probes
  const fnNames: string[] = ["health_live"];
  const parts: string[] = [
    "\"use strict\";",
    "function health_live() { return { status: \"ok\" }; }",
  ];
  for (let i = 0; i < n; i++) {
    routes.push({
      id: `res${i}.get`,
      method: "GET",
      path: `/res${i}/item/:id`,
      segments: seg(["static", `res${i}`], ["static", "item"], ["param", "id"]),
      handler: `res${i}.get`,
      params: { schema: `sch:res${i}.params`, coerce: "path" },
      responses: { "200": { schema: null, strategy: "js", problem: null }, "422": { schema: null, strategy: "js", problem: "validation" } },
    });
    parts.push(
      `function res${i}_get(ctx) { return { id: ctx.params.id, n: ${n} }; }`,
    );
    fnNames.push(`res${i}_get`);
  }
  const manifestParts: string[] = [
    `["health.live", 0, health_live]`,
    ...Array.from({ length: n }, (_, i) => `["res${i}.get", 0, res${i}_get]`),
  ];
  parts.push(`globalThis.__velquFunctionManifest = [\n  ${manifestParts.join(",\n  ")}\n];`);
  parts.push(`globalThis.__velquFunctions = globalThis.__velquFunctionManifest.map(function (e) { return e[2]; });`);
  const schemas: Record<string, unknown> = {};
  for (let i = 0; i < n; i++) {
    schemas[`sch:res${i}.params`] = {
      kind: "object",
      properties: { id: { kind: "integer", minimum: 1, maximum: n } },
      required: ["id"],
    };
  }
  return { routes, bundle: parts.join("\n"), schemas };
}

// ---------------------------------------------------------------- main

const which = process.argv[2] ?? "fixture";
if (which === "fixture") {
  const pack = buildPack(fixtureRoutes(), FIXTURE_SCHEMAS, FIXTURE_BUNDLE, "proof-fixture");
  mkdirSync("examples/proof/dist", { recursive: true });
  writeFileSync("examples/proof/dist/app.qpack", pack);
  console.log(`fixture pack: examples/proof/dist/app.qpack (${pack.length} bytes)`);
} else {
  const n = parseInt(which, 10);
  if (!Number.isInteger(n) || n <= 0) {
    console.error("usage: bun build-proof-pack.ts [fixture|N]");
    process.exit(1);
  }
  const { routes, bundle, schemas } = generatedRoutes(n);
  const pack = buildPack(routes, schemas, bundle, `scale-${n}`);
  mkdirSync("benchmarks/raw/packs", { recursive: true });
  writeFileSync(`benchmarks/raw/packs/app-${n}.qpack`, pack);
  console.log(`scale pack: benchmarks/raw/packs/app-${n}.qpack (${pack.length} bytes, ${routes.length} routes)`);
}
