/**
 * npm packaging verification (owner npm-org onboarding packet).
 *
 * Every publishable @velqu/* package must:
 * - build dist/ JS + declarations via scripts/build-packages.ts;
 * - pack a correctly laid-out tarball (files at `package/` root — the
 *   D1 cleanroom defect was archives with entries at the tar root);
 * - declare a non-private, MIT, 0.1.0-beta.1 manifest with public
 *   access and the bun/src + default/dist exports split;
 * - `@velqu/browser-runtime` must ship the vendored, hash-pinned
 *   kernel/ assets (the D3 cleanroom defect was a tarball without a
 *   kernel, which fails `browser-wasm` builds closed);
 * - `bun pm pack` (the publish packer) replaces `workspace:*` deps with
 *   the concrete beta version.
 */
import { describe, it, expect, beforeAll, afterAll } from "bun:test";
import { execSync, spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, readdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..");
// Mirrors ALL_PKGS in scripts/publish-beta.sh (dependency order). The
// sync test below fails if either list drifts — the #1315 defect was a
// package (browser-pglite) present in the publish script but absent
// here, leaving the one unpublished package unqualified.
const PUBLISH_ORDER = [
  "contract",
  "schema",
  "core",
  "treaty",
  "browser-runtime",
  "browser-pglite",
  "compiler",
  "cli",
] as const;
const VERSION = "0.1.0-beta.1";

let packDirs: Record<string, { dir: string; files: string[] }> = {};

beforeAll(() => {
  execSync("bun scripts/build-packages.ts", { cwd: root, stdio: "pipe" });
  for (const pkg of PUBLISH_ORDER) {
    const dir = mkdtempSync(join(tmpdir(), `velqu-pack-${pkg}-`));
    execSync("npm pack --json --pack-destination " + dir, {
      cwd: join(root, "packages", pkg),
      stdio: "pipe",
    });
    const tgz = readdirSync(dir).find((f) => f.endsWith(".tgz"))!;
    const list = execSync(`tar -tzf ${join(dir, tgz)}`, { stdio: "pipe" })
      .toString()
      .trim()
      .split("\n");
    packDirs[pkg] = { dir, files: list };
  }
}, 300_000);

afterAll(() => {
  for (const { dir } of Object.values(packDirs)) rmSync(dir, { recursive: true, force: true });
});

describe("npm packaging readiness", () => {
  it("every publishable manifest is non-private MIT beta with public access", () => {
    for (const pkg of PUBLISH_ORDER) {
      const m = JSON.parse(readFileSync(join(root, "packages", pkg, "package.json"), "utf8"));
      expect(m.private).toBeUndefined();
      expect(m.version).toBe(VERSION);
      expect(m.license).toBe("MIT");
      expect(m.publishConfig.access).toBe("public");
      expect(m.name).toBe(`@velqu/${pkg}`);
      if (pkg === "cli") continue; // bin-only package: no library exports map
      // bun consumers keep fresh TS sources; others get dist JS + types
      expect(m.exports["."].bun).toBe("./src/index.ts");
      expect(m.exports["."].default).toBe("./dist/index.js");
      expect(m.exports["."].types).toBe("./dist/index.d.ts");
    }
  });

  it("tarballs are npm-installable: entries under package/, dist + types present", () => {
    for (const pkg of PUBLISH_ORDER) {
      const { files } = packDirs[pkg];
      // D1 regression: archives must use the npm-standard package/ prefix
      expect(files.every((f) => f === "package/" || f.startsWith("package/"))).toBeTrue();
      expect(files).toContain("package/package.json");
      expect(files).toContain("package/dist/index.js");
      expect(files).toContain("package/dist/index.d.ts");
      expect(files).toContain("package/src/index.ts");
      expect(files).toContain("package/README.md");
    }
  });

  it("browser-runtime ships the vendored hash-pinned kernel assets (D3)", () => {
    const { files } = packDirs["browser-runtime"];
    expect(files).toContain("package/kernel/kernel.json");
    expect(
      files.some((f) => f.startsWith("package/kernel/q_browser_kernel_bg.wasm")),
    ).toBeTrue();
    expect(
      files.some((f) => f.includes("kernel.nodejs-glue.js") || f.includes("kernel.browser-glue")),
    ).toBeTrue();
    const kernelJson = JSON.parse(
      readFileSync(join(root, "packages", "browser-runtime", "kernel", "kernel.json"), "utf8"),
    );
    expect(kernelJson.wasm.sha256).toMatch(/^[0-9a-f]{64}$/);
    expect(kernelJson.glue.sha256).toMatch(/^[0-9a-f]{64}$/);
  });

  it("cli ships a bun-runnable bin and declares its kernel dependency", () => {
    const { files } = packDirs["cli"];
    expect(files).toContain("package/bin/velqu.js");
    const bin = readFileSync(join(root, "packages", "cli", "bin", "velqu.js"), "utf8");
    expect(bin.startsWith("#!/usr/bin/env bun")).toBeTrue();
    const m = JSON.parse(readFileSync(join(root, "packages", "cli", "package.json"), "utf8"));
    expect(m.dependencies["@velqu/browser-runtime"]).toBe("workspace:*");
    expect(m.dependencies["@velqu/compiler"]).toBe("workspace:*");
  });

  it("cross-package workspace deps are declared where imports exist", () => {
    const core = JSON.parse(readFileSync(join(root, "packages", "core", "package.json"), "utf8"));
    expect(core.dependencies["@velqu/schema"]).toBe("workspace:*");
  });

  it("PUBLISH_ORDER stays in sync with scripts/publish-beta.sh ALL_PKGS", () => {
    const script = readFileSync(join(root, "scripts", "publish-beta.sh"), "utf8");
    const m = script.match(/ALL_PKGS=\(([^)]+)\)/);
    expect(m).toBeDefined();
    const scriptOrder = m![1].trim().split(/\s+/);
    expect(scriptOrder.join(" ")).toBe(PUBLISH_ORDER.join(" "));
  });

  it("browser-pglite keeps the PGlite engine external (C-003 packaging invariant)", () => {
    // The engine is a real npm dependency loaded via dynamic import on
    // first open() — never inlined into dist/, never vendored into the
    // tarball. A regressed build (engine bundled) would balloon dist to
    // ~10 MB; the adapter is ~12 KB.
    const manifest = JSON.parse(
      readFileSync(join(root, "packages", "browser-pglite", "package.json"), "utf8"),
    );
    expect(manifest.dependencies["@electric-sql/pglite"]).toBe("0.5.8"); // exact pin
    const dist = readFileSync(join(root, "packages", "browser-pglite", "dist", "index.js"));
    expect(dist.byteLength).toBeLessThan(100_000);
    // no @velqu/* workspace deps: --only browser-pglite is dependency-safe
    expect(Object.keys(manifest.dependencies ?? {}).some((d) => d.startsWith("@velqu/"))).toBeFalse();
    // the tarball ships only src/dist/README — no engine assets, no wasm
    const { files } = packDirs["browser-pglite"];
    expect(files.some((f) => f.endsWith(".wasm"))).toBeFalse();
    expect(files.every((f) => f.startsWith("package/src/") || f.startsWith("package/dist/") || f === "package/README.md" || f === "package/package.json" || f === "package/")).toBeTrue();
  });

  it("bun pm pack replaces workspace:* with the beta version (publish-path parity)", () => {
    const dir = mkdtempSync(join(tmpdir(), "velqu-bunpack-"));
    try {
      const r = spawnSync("bun", ["pm", "pack"], {
        cwd: join(root, "packages", "core"),
        stdio: "pipe",
        env: { ...process.env },
      });
      const tgz = readdirSync(join(root, "packages", "core")).find((f) =>
        f.endsWith(".tgz"),
      );
      expect(tgz).toBeDefined();
      const manifest = execSync(`tar -xzf ${join(root, "packages", "core", tgz!)} -O package/package.json`, {
        cwd: dir,
        stdio: "pipe",
      }).toString();
      const m = JSON.parse(manifest);
      expect(m.version).toBe(VERSION);
      expect(m.dependencies["@velqu/schema"]).toBe(VERSION); // replaced, not workspace:*
      expect(JSON.stringify(m)).not.toContain("workspace:");
      rmSync(join(root, "packages", "core", tgz!));
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  });

  it("dist output imports cleanly under plain Node (non-bun resolver path)", () => {
    // run in a subprocess so the bun-condition src path is not used
    const r = spawnSync(
      "node",
      [
        "--input-type=module",
        "-e",
        `import("${join(root, "packages", "treaty", "dist", "index.js")}").then(m => { if (typeof m.treaty !== "function") throw new Error("bad export"); })`,
      ],
      { stdio: "pipe" },
    );
    expect(r.status).toBe(0);
  });
});
