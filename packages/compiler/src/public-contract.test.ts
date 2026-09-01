/** M4A-005-C: public hash/version pinning and diagnosable mismatch evidence. */
import { describe, it, expect } from "bun:test";
import { build } from "./index";
import { verifyPublishedManifest } from "./published";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Version and public contract hash (M4A-005-C)", () => {
  it("publishes one stable 128-bit public hash across contract artifacts", async () => {
    const out = join(tmpdir(), `velqu-public-hash-${Date.now()}`);
    mkdirSync(out, { recursive: true });
    try {
      await build({ project: "examples/proof", outDir: out });
      const contract = JSON.parse(await Bun.file(join(out, "contract.json")).text());
      const meta = JSON.parse(await Bun.file(join(out, "contract.meta.json")).text());
      const manifest = JSON.parse(await Bun.file(join(out, "published-manifest.json")).text());
      const dts = await Bun.file(join(out, "contract.d.ts")).text();
      expect(contract.formatVersion).toBe(1);
      expect(meta.formatVersion).toBe(1);
      expect(manifest.formatVersion).toBe(1);
      expect(contract.contractHash).toMatch(/^[a-f0-9]{32}$/);
      expect(meta.contractHash).toBe(contract.contractHash);
      expect(manifest.contractHash).toBe(contract.contractHash);
      expect(dts).toContain(`export const contractHash = "${contract.contractHash}"`);
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  });

  it("diagnoses version and public-hash drift instead of silently accepting it", async () => {
    const out = join(tmpdir(), `velqu-public-hash-drift-${Date.now()}`);
    mkdirSync(out, { recursive: true });
    try {
      await build({ project: "examples/proof", outDir: out });
      const manifestPath = join(out, "published-manifest.json");
      const manifest = JSON.parse(await Bun.file(manifestPath).text());
      manifest.formatVersion = 99;
      manifest.contractHash = "00000000000000000000000000000000";
      writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));
      const verified = verifyPublishedManifest(manifestPath);
      expect(verified.ok).toBeFalse();
      expect(verified.errors).toContain("unsupported published manifest formatVersion: 99 (expected 1)");
      expect(verified.errors.some((error) => error.includes("contractHash mismatch with contract.json"))).toBeTrue();
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  });
});
