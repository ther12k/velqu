/**
 * BWASM-R-001 — browser-purity guard. The package's browser-facing
 * graph must contain no Bun.*, node:*, native addon, q-http, native
 * runtime, or testing imports (acceptance criterion 3). Mechanical
 * source scan over the package; the bundler smoke (below, in CI/e2e)
 * covers the compiled form.
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

describe("browser-runtime purity (R-001)", () => {
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
    expect(pkg.name).toBe("@velqu/browser-runtime");
    expect(pkg.browser).toBe("./src/index.ts");
    expect(Object.keys(pkg.exports as object)).toEqual(["."]);
    // No build scripts that could smuggle native artifacts.
    expect(pkg.scripts).toBeUndefined();
  });
});
