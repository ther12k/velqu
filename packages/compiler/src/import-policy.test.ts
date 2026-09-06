/**
 * BWASM-B-003 — import-policy fixture matrix:
 * direct / transitive / aliased-reexport / dynamic forbidden imports;
 * deployment-required classification; browser-safe positives.
 */
import { describe, it, expect, afterEach } from "bun:test";
import { mkdirSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { scanImportPolicy, assertImportPolicyOk, IMPORT_POLICY_VERSION } from "../src/import-policy";

const root = join(import.meta.dir, "..", "..", "..");
let dirs: string[] = [];
afterEach(() => {
  for (const d of dirs.splice(0)) rmSync(d, { recursive: true, force: true });
});

/** Create a synthetic app whose entry imports `entryImports`, with helper modules. */
function fixture(entryImports: string[], helpers: Record<string, string>): string {
  const dir = join(root, `.tmp-b003-${Math.random().toString(36).slice(2)}`);
  dirs.push(dir);
  mkdirSync(join(dir, "src"), { recursive: true });
  writeFileSync(
    join(dir, "src", "app.ts"),
    [
      ...entryImports,
      `import { route } from "@velqu/core";`,
      `import { s } from "@velqu/schema";`,
      ``,
      `export const hello = route({`,
      `  id: "hello.get",`,
      `  method: "GET",`,
      `  path: "/hello/:name",`,
      `  params: s.object({ name: s.string({ maxLength: 60 }) }),`,
      `  response: { 200: s.object({ message: s.string() }) },`,
      `  handle: ({ params }) => ({ message: \`Hello \${params.name}\` }),`,
      `});`,
      ``,
      `export const app = { routes: [hello] };`,
      ``,
    ].join("\n"),
  );
  for (const [name, content] of Object.entries(helpers)) {
    const p = join(dir, "src", name);
    mkdirSync(join(p, ".."), { recursive: true });
    writeFileSync(p, content);
  }
  return join(dir, "src", "app.ts");
}

describe("B-003 import policy — forbidden matrix", () => {
  it("direct node: imports are caught", () => {
    const entry = fixture([`import { readFileSync } from "node:fs";`], {});
    const report = scanImportPolicy(entry);
    expect(report.violations.some((v) => v.specifier === "node:fs")).toBeTrue();
    expect(() => assertImportPolicyOk(report)).toThrow("BWASM-POLICY-NODE-BUILTIN");
  });

  it("transitive forbidden imports through relative helpers are caught, with the import chain", () => {
    const entry = fixture([`import { helper } from "./helper";`], {
      "helper.ts": `export { readFile } from "./deep/fs-wrapper";`,
      "deep/fs-wrapper.ts": `import { readFileSync } from "node:fs";\nexport const readFile = readFileSync;`,
    });
    const report = scanImportPolicy(entry);
    const v = report.violations.find((x) => x.specifier === "node:fs");
    expect(v).toBeDefined();
    expect(v!.chain.length).toBeGreaterThanOrEqual(3); // entry → helper → fs-wrapper
    expect(v!.chain.some((f) => f.includes("fs-wrapper"))).toBeTrue();
    expect(() => assertImportPolicyOk(report)).toThrow("node:fs");
  });

  it("aliased re-exports cannot hide a forbidden import", () => {
    const entry = fixture(
      [`import { write as w } from "./alias";`, `export const write = w;`],
      {
        "alias.ts": `import { writeFileSync as wfs } from "node:fs";\nexport const write = wfs;`,
      },
    );
    const report = scanImportPolicy(entry);
    expect(report.violations.some((v) => v.specifier === "node:fs")).toBeTrue();
  });

  it("bare Node builtins (without node:) are forbidden", () => {
    const entry = fixture([`import { join } from "path";`], {});
    expect(scanImportPolicy(entry).violations.some((v) => v.specifier === "path")).toBeTrue();
  });

  it("dynamic import with an opaque specifier is forbidden (cannot verify)", () => {
    const entry = fixture(
      [`export async function load(name: string) { return import(name); }`],
      {},
    );
    const report = scanImportPolicy(entry);
    const v = report.violations.find((x) => x.code === "BWASM-POLICY-DYNAMIC-IMPORT-OPAQUE");
    expect(v).toBeDefined();
  });

  it("eval and new Function are forbidden dynamic code loading", () => {
    const entry = fixture(
      [`export const a = () => eval("1+1");`, `export const b = () => new Function("return 1")();`],
      {},
    );
    const report = scanImportPolicy(entry);
    expect(report.violations.filter((v) => v.code === "BWASM-POLICY-DYNAMIC-CODE").length).toBe(2);
  });
});

describe("B-003 import policy — deployment-required class", () => {
  it("server-only drivers classify as deployment-required with capability remediation", () => {
    const entry = fixture([`import { pool } from "./db";`], {
      "db.ts": `import pg from "pg";\nexport const pool = pg;`,
    });
    const report = scanImportPolicy(entry);
    // The bare "pg" import from db.ts: policy classifies deployment-required.
    const v = report.violations.find((x) => x.classification === "deployment-required");
    expect(v).toBeDefined();
    expect(v!.remediation).toContain("runtime:postgres");
    expect(v!.docs).toContain("browser-import-policy");
  });
});

describe("B-003 import policy — browser-safe positives", () => {
  it("@velqu/* and relative schema imports produce no violations", () => {
    const entry = fixture([`import { s as schema } from "@velqu/schema";`], {
      "safe.ts": `import { route } from "@velqu/core";\nexport const r = route;`,
    });
    const report = scanImportPolicy(entry);
    expect(report.violations).toEqual([]);
    expect(() => assertImportPolicyOk(report)).not.toThrow();
  });

  it("policy version is explicit", () => {
    expect(IMPORT_POLICY_VERSION).toBe(1);
  });
});
