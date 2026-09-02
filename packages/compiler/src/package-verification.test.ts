import { describe, it, expect } from "bun:test";
import { build, verifyPublishedPackage } from "./index";
import { mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Published package verification (M4A-005-D)", () => {
  it("accepts a complete package with matching app/hash/version pin", async () => {
    const out = join(tmpdir(), `velqu-package-ok-${Date.now()}`);
    mkdirSync(out, { recursive: true });
    try {
      const result = await build({ project: "examples/proof", outDir: out });
      const manifest = JSON.parse(await Bun.file(join(out, "published-manifest.json")).text());
      const checked = verifyPublishedPackage(join(out, "published-manifest.json"), {
        appId: "proof",
        contractHash: manifest.contractHash,
        formatVersion: 1,
      });
      expect(checked.ok).toBeTrue();
      expect(checked.errors).toEqual([]);
      expect(result.publishedArtifacts["contract.d.ts"]?.bytes).toBeGreaterThan(0);
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  });

  it("reports a diagnosable app/hash pin mismatch", async () => {
    const out = join(tmpdir(), `velqu-package-pin-${Date.now()}`);
    mkdirSync(out, { recursive: true });
    try {
      await build({ project: "examples/proof", outDir: out });
      const checked = verifyPublishedPackage(join(out, "published-manifest.json"), {
        appId: "different-app",
        contractHash: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        formatVersion: 1,
      });
      expect(checked.ok).toBeFalse();
      expect(checked.errors).toContain("appId mismatch (expected different-app, got proof)");
      expect(checked.errors).toContain(
        "contractHash mismatch (expected aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, got cec56f2ef6dd4977205ab389c3178e93)",
      );
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  });
});
