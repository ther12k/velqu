/**
 * BWASM-C-003 — browser-purity guard (R-001 pattern). The package's
 * browser-facing graph must contain no Bun.*, node:*, native addon,
 * q-http, native runtime, or testing imports; the SQL engine stays
 * behind a dynamic import (see lazy.test.ts).
 */
import { describe, it, expect } from "bun:test";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const FORBIDDEN = [
  /from\s+"bun/,
  /require\(["']bun/,
  /from\s+"node:/,
  /require\(["']node:/,
  /from\s+"@velqu\/testing/,
  /from\s+".*q-http/,
  /from\s+".*q-runtime/,
  /process\./,
  /\b__dirname\b/,
  /\b__filename\b/,
];

function* walk(dir: string): Generator<string> {
  for (const entry of readdirSync(dir)) {
    const p = join(dir, entry);
    if (statSync(p).isDirectory()) yield* walk(p);
    else if (p.endsWith(".ts")) yield p;
  }
}

describe("browser-pglite purity (C-003)", () => {
  it("src/ contains no forbidden imports or Node/Bun globals", () => {
    const violations: string[] = [];
    for (const file of walk(join(import.meta.dir, "..", "src"))) {
      const src = readFileSync(file, "utf8");
      for (const pattern of FORBIDDEN) {
        if (pattern.test(src)) violations.push(`${file}: ${pattern}`);
      }
    }
    expect(violations).toEqual([]);
  });

  it("package.json declares browser-only entry points", () => {
    const pkg = JSON.parse(
      readFileSync(join(import.meta.dir, "..", "package.json"), "utf8"),
    ) as Record<string, unknown>;
    expect(pkg.name).toBe("@velqu/browser-pglite");
    // Built browser output for generic resolvers; Bun consumers keep the
    // (browser-only, purity-audited) TypeScript sources.
    expect(pkg.browser).toBe("./dist/index.js");
    const exp = (pkg.exports as Record<string, Record<string, string>>)["."];
    expect(exp.bun).toBe("./src/index.ts");
    expect(exp.default).toBe("./dist/index.js");
    expect(exp.types).toBe("./dist/index.d.ts");
    expect(Object.keys(pkg.exports as object)).toEqual(["."]);
    // No build scripts that could smuggle native artifacts.
    expect(pkg.scripts).toBeUndefined();
    // The SQL engine is a real npm dependency, never inlined by our build
    // (scripts/build-packages.ts keeps @electric-sql/pglite external) and
    // never a peer/host optional: projects without this package never
    // download it (C-003 acceptance).
    expect(pkg.dependencies).toEqual({ "@electric-sql/pglite": "0.5.8" });
  });
});
