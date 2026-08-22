/**
 * Projection parity checks (M25-008-B): every published projection derives
 * from the one canonical Schema IR, so statuses, fields, and security must
 * agree across contract.json, the compiled pack, openapi.json, and the
 * generated contract.d.ts. This suite cross-checks the built proof-app
 * artifacts — the same check runs in `scripts/verify` via `bun test`.
 */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";

const DIST = "examples/proof/dist";
const contract = JSON.parse(readFileSync(`${DIST}/contract.json`, "utf8"));
const meta = JSON.parse(readFileSync(`${DIST}/contract.meta.json`, "utf8"));
const openapi = JSON.parse(readFileSync(`${DIST}/openapi.json`, "utf8"));
const pack = JSON.parse(readFileSync(`${DIST}/app.qpack`, "utf8"));
const dts = readFileSync(`${DIST}/contract.d.ts`, "utf8");

/** status members declared for a route in the generated d.ts */
function dtsStatuses(routeId: string): string[] {
  const start = dts.indexOf(`"${routeId}": RouteContract<`);
  if (start < 0) throw new Error(`route ${routeId} missing from contract.d.ts`);
  const end = dts.indexOf("}>;", start);
  const block = dts.slice(start, end);
  return [...block.matchAll(/\n\s+(\d{3}):/g)].map((m) => m[1]);
}

/** OpenAPI spells path parameters `{name}`; the contract spells `:name` */
function openapiPath(path: string): string {
  return path.replace(/:([A-Za-z]+)/g, "{$1}");
}

function opFor(path: string, method: string): Record<string, unknown> {
  const node = (openapi.paths as Record<string, Record<string, Record<string, unknown>>>)[
    openapiPath(path)
  ];
  return node?.[method.toLowerCase()] ?? {};
}

/** property names of an IR object node, sorted */
function irFields(ir: unknown): string[] {
  if (ir == null || typeof ir !== "object") return [];
  const props = (ir as { properties?: Record<string, unknown> }).properties;
  return Object.keys(props ?? {}).sort();
}

describe("projection parity (M25-008-B)", () => {
  test("route identity agrees across contract, pack, and OpenAPI", () => {
    const contractIds = Object.keys(contract.routes).sort();
    const packIds = pack.routes.map((r: { id: string }) => r.id).sort();
    expect(packIds).toEqual(contractIds);

    const openapiIds = Object.values(openapi.paths)
      .flatMap((m: Record<string, { operationId?: string }>) =>
        Object.values(m).map((op) => op.operationId),
      )
      .filter(Boolean)
      .sort();
    expect(openapiIds).toEqual(contractIds);

    // method/path agreement per route
    for (const r of pack.routes as Array<{ id: string; method: string; path: string }>) {
      const c = contract.routes[r.id];
      expect(`${c.method.toUpperCase()} ${c.path}`).toBe(`${r.method} ${r.path}`);
    }
  });

  test("declared statuses agree in all four projections", () => {
    for (const r of pack.routes as Array<{ id: string; responses: Record<string, unknown> }>) {
      const fromContract = Object.keys(contract.routes[r.id].responses).sort();
      const fromPack = Object.keys(r.responses).sort();
      const fromOpenapi = Object.keys(
        (opFor(contract.routes[r.id].path, r.method).responses ?? {}) as Record<string, unknown>,
      ).sort();
      const fromDts = dtsStatuses(r.id).sort();
      expect(fromPack).toEqual(fromContract);
      expect(fromOpenapi).toEqual(fromContract);
      expect(fromDts).toEqual(fromContract);
    }
  });

  test("response/params/query/body fields agree across IR projections", () => {
    for (const r of pack.routes as Array<{ id: string; responses: Record<string, { schema?: string | null }>; params?: { schema?: string | null }; query?: { schema?: string | null }; body?: { schema?: string | null } }>) {
      const c = contract.routes[r.id];
      const packSchemas = pack.schemas as Record<string, unknown>;

      // response fields: contract.json IR == pack schema IR
      for (const [status, decl] of Object.entries(r.responses)) {
        const contractIr = c.responses[status];
        const packIr = decl.schema ? packSchemas[decl.schema] : null;
        expect(irFields(packIr)).toEqual(irFields(contractIr));

        // openapi object schemas expose the same properties
        const op = opFor(c.path, r.method) as {
          responses?: Record<string, { content?: Record<string, { schema?: { properties?: Record<string, unknown> } }> }>;
        };
        const oaSchema = op.responses?.[status]?.content?.["application/json"]?.schema;
        if (oaSchema?.properties) {
          expect(Object.keys(oaSchema.properties).sort()).toEqual(irFields(contractIr));
        }
      }

      // params/query/body field agreement (contract vs pack)
      expect(irFields(packSchemas[r.params?.schema ?? ""] ?? null)).toEqual(irFields(c.params));
      expect(irFields(packSchemas[r.query?.schema ?? ""] ?? null)).toEqual(irFields(c.query));
      expect(irFields(packSchemas[r.body?.schema ?? ""] ?? null)).toEqual(irFields(c.body));
    }
  });

  test("compact contract metadata stays in sync and compact (M25-008-C)", () => {
    // hash binds the metadata to the exact contract
    expect(meta.contractHash).toBe(contract.contractHash);
    expect(meta.appId).toBe(contract.appId);

    // route set + statuses + security agree with the full contract
    expect(Object.keys(meta.routes).sort()).toEqual(Object.keys(contract.routes).sort());
    for (const [id, m] of Object.entries(meta.routes) as Array<[string, { method: string; path: string; statuses: string[]; secured: boolean }]>) {
      const c = contract.routes[id];
      expect(m.method.toUpperCase()).toBe(c.method.toUpperCase());
      expect(m.path).toBe(c.path);
      expect(m.statuses).toEqual(Object.keys(c.responses).sort());
      expect(m.secured).toBe(c.security != null);
    }

    // compactness bound: no schema bodies, and the whole file stays tiny
    const raw = readFileSync(`${DIST}/contract.meta.json`, "utf8");
    expect(raw).not.toContain("properties");
    expect(Buffer.byteLength(raw)).toBeLessThan(4096);
  });

  test("security agrees across contract, pack, and OpenAPI", () => {
    for (const r of pack.routes as Array<{ id: string; security: unknown[] }>) {
      const c = contract.routes[r.id];
      const op = opFor(c.path, r.method) as { security?: unknown[] };
      const securedInContract = c.security != null;
      const securedInPack = (r.security ?? []).length > 0;
      const securedInOpenapi = (op?.security ?? []).length > 0;
      expect(securedInContract).toBe(securedInPack);
      expect(securedInContract).toBe(securedInOpenapi);
    }
  });
});
