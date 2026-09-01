import { describe, it, expect } from "bun:test";
import { build } from "./index";
import { verifyPublishedManifest } from "./published";
import { mkdirSync, rmSync, copyFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

describe("Published compact contract artifacts (M4A-005-A)", () => {
  it("build emits a manifest for the five published artifacts and verifies hashes", async () => {
    const outDir = join(tmpdir(), `velqu-published-${Date.now()}`);
    mkdirSync(outDir, { recursive: true });
    try {
      const result = await build({ project: "examples/proof", outDir });
      expect(Object.keys(result.publishedArtifacts).sort()).toEqual([
        "contract.d.ts",
        "contract.json",
        "contract.lock.json",
        "contract.meta.json",
        "openapi.json",
        "published-manifest.json",
      ].sort());
      const verified = verifyPublishedManifest(join(outDir, "published-manifest.json"));
      expect(verified.ok).toBeTrue();
      expect(verified.errors).toEqual([]);
      expect(verified.manifest?.contractHash).toHaveLength(32);
      expect(verified.manifest?.appId).toBe("proof");
    } finally {
      rmSync(outDir, { recursive: true, force: true });
    }
  });

  it("verifies deterministically across two independent output directories", async () => {
    const left = join(tmpdir(), `velqu-published-left-${Date.now()}`);
    const right = join(tmpdir(), `velqu-published-right-${Date.now()}`);
    mkdirSync(left, { recursive: true });
    mkdirSync(right, { recursive: true });
    try {
      await build({ project: "examples/proof", outDir: left });
      await build({ project: "examples/proof", outDir: right });
      const leftManifest = JSON.parse(await Bun.file(join(left, "published-manifest.json")).text());
      const rightManifest = JSON.parse(await Bun.file(join(right, "published-manifest.json")).text());
      expect(leftManifest).toEqual(rightManifest);
      expect(verifyPublishedManifest(join(left, "published-manifest.json")).ok).toBeTrue();
      expect(verifyPublishedManifest(join(right, "published-manifest.json")).ok).toBeTrue();
    } finally {
      rmSync(left, { recursive: true, force: true });
      rmSync(right, { recursive: true, force: true });
    }
  });

  it("diagnoses a modified published artifact instead of accepting stale metadata", async () => {
    const outDir = join(tmpdir(), `velqu-published-drift-${Date.now()}`);
    mkdirSync(outDir, { recursive: true });
    try {
      await build({ project: "examples/proof", outDir });
      writeFileSync(join(outDir, "contract.d.ts"), "// drift\n");
      const verified = verifyPublishedManifest(join(outDir, "published-manifest.json"));
      expect(verified.ok).toBeFalse();
      expect(verified.errors.some((error) => error.includes("contract.d.ts: sha256 mismatch"))).toBeTrue();
      expect(verified.errors.some((error) => error.includes("byte length mismatch"))).toBeTrue();
    } finally {
      rmSync(outDir, { recursive: true, force: true });
    }
  });
});
