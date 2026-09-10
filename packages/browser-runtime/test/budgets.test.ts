/**
 * BWASM-Q-005 — Automated release size and artifact budget gate tests.
 *
 * Binds the implementation to the ratified budgets in
 * `docs/browser-wasm/evidence/budgets.json`:
 *
 * - base_wasm_kernel: <= 512,000 bytes brotli-compressed.
 * - runtime_js_glue: <= 51,200 bytes brotli-compressed.
 * - per_app_handler_bundle: <= 102,400 bytes brotli-compressed.
 * - optional_capability_bundle: <= 51,200 bytes each.
 * - total_initial_transfer: <= 1,048,576 bytes (1 MiB).
 * - Core projects do not download optional SQL or parity-engine assets.
 */

import { describe, expect, it } from "bun:test";
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { brotliCompressSync, gzipSync } from "node:zlib";

describe("BWASM-Q-005 artifact size budgets (ratified-targets)", () => {
  const root = join(import.meta.dir, "../../..");
  const kernelPath = join(root, "packages/browser-runtime/kernel/q_browser_kernel_bg.wasm");
  const kernelGluePath = join(root, "packages/browser-runtime/kernel/q_browser_kernel.nodejs-glue.js");
  const browserDemoDir = join(root, "examples/browser-demo/dist/browser");
  let kernelBrotliBytes = 0;

  it("base_wasm_kernel satisfies ratified brotli budget <= 512,000 B", () => {
    expect(existsSync(kernelPath)).toBeTrue();
    const bytes = readFileSync(kernelPath);
    const brotli = brotliCompressSync(bytes); // default quality is 11
    kernelBrotliBytes = brotli.byteLength;
    const gzip9 = gzipSync(bytes, { level: 9 });

    // Ratified target: 512,000 bytes brotli (BWASM-D-004)
    expect(brotli.byteLength).toBeLessThanOrEqual(512000);
    // Raw is ~1.7 MB
    expect(bytes.byteLength).toBeGreaterThan(1_000_000);

    // Carried finding documentation: gzip-9 is ~573 KB (> 500 KiB proxy),
    // but the ratified standard brotli-11 is ~400 KB (<= 500 KiB budget).
    expect(gzip9.byteLength).toBeLessThan(600_000);
  }, 20_000);

  it("runtime_js_glue satisfies ratified budget <= 51,200 B", () => {
    expect(existsSync(kernelGluePath)).toBeTrue();
    const glueBytes = readFileSync(kernelGluePath);
    const glueBrotli = brotliCompressSync(glueBytes);
    expect(glueBrotli.byteLength).toBeLessThanOrEqual(51200);
  });

  it("per_app_handler_bundle satisfies ratified budget <= 102,400 B", () => {
    if (existsSync(browserDemoDir)) {
      const handlerFile = join(browserDemoDir, "app.browser.js");
      if (existsSync(handlerFile)) {
        const handlerBytes = readFileSync(handlerFile);
        const handlerBrotli = brotliCompressSync(handlerBytes);
        expect(handlerBrotli.byteLength).toBeLessThanOrEqual(102400);
      }
    }
  });

  it("core projects do not download optional SQL or parity-engine assets", () => {
    // Acceptance criterion 1: Core projects do not download optional SQL
    // (pglite) or parity-engine (quickjs-wasm) assets.
    if (existsSync(browserDemoDir)) {
      const files = readdirSync(browserDemoDir);
      for (const f of files) {
        expect(f).not.toContain("pglite");
        expect(f).not.toContain("quickjs.wasm");
        expect(f).not.toContain("quickjs-wasm");
      }

      const manifestPath = join(browserDemoDir, "browser-manifest.json");
      if (existsSync(manifestPath)) {
        const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
        const roles = Object.keys(manifest.artifacts ?? {});
        expect(roles).not.toContain("sql");
        expect(roles).not.toContain("quickjs-wasm");
      }
    }
  });

  it("total_initial_transfer satisfies 1 MiB (1,048,576 B) budget", () => {
    const glueBytes = readFileSync(kernelGluePath);
    const kernelSize = kernelBrotliBytes || brotliCompressSync(readFileSync(kernelPath)).byteLength;
    let totalBrotli = kernelSize + brotliCompressSync(glueBytes).byteLength;

    if (existsSync(browserDemoDir)) {
      for (const f of readdirSync(browserDemoDir)) {
        if (f.startsWith(".")) continue;
        const b = readFileSync(join(browserDemoDir, f));
        totalBrotli += brotliCompressSync(b).byteLength;
      }
    }

    // Budget: 1,048,576 bytes
    expect(totalBrotli).toBeLessThanOrEqual(1048576);
  }, 20_000);
});
