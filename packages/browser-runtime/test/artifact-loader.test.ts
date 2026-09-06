/**
 * BWASM-B-002 — content-addressed artifact manifest + loader tests:
 * golden canonicalization, tamper/truncation/missing/cross-build/
 * unsupported-version negatives, base-path resolution, buildId binding.
 */
import { describe, it, expect } from "bun:test";
import {
  emitArtifactManifest,
  loadArtifacts,
  canonicalManifestJson,
  buildIdOf,
  resolveArtifactUrl,
  ArtifactManifestError,
  ARTIFACT_MANIFEST_VERSION,
} from "../src/artifact-loader";

const packBytes = new TextEncoder().encode('{"formatVersion":1,"app":"demo"}');
const kernelBytes = new TextEncoder().encode("wasm-bytes-here");
const bundleSource =
  'import { defineBrowserHandlers } from "@velqu/browser-runtime";\nexport const handlers = defineBrowserHandlers([], {});\n';

const enc = (s: string) => new TextEncoder().encode(s);

function makeInput() {
  return {
    appId: "demo",
    handlerAbiVersion: 1,
    kernelAbiVersion: 1,
    packSha256: require("node:crypto")
      .createHash("sha256")
      .update(packBytes)
      .digest("hex") as string,
    artifacts: {
      pack: packBytes,
      kernelWasm: kernelBytes,
      handlerBundle: enc(bundleSource),
      manifest: enc("{}"),
      contract: enc("{}"),
      schemas: enc("{}"),
      capabilities: enc("{}"),
      sourceMap: enc("{}"),
    } as never,
    urls: {
      pack: "app.qpack",
      kernelWasm: "kernel.wasm",
      handlerBundle: "app.browser.js",
      manifest: "browser-manifest.json",
      contract: "contract.json",
      schemas: "schema-manifest.json",
      capabilities: "capability-manifest.json",
      sourceMap: "app.browser.map.json",
    } as never,
  };
}

/** Reader over an in-memory file map keyed by deploy URL. */
function readerFrom(files: Record<string, Uint8Array>) {
  return async (url: string): Promise<Uint8Array> => {
    const hit = files[url];
    if (!hit) throw new Error(`ENOENT ${url}`);
    return hit;
  };
}

async function fullFileMap(input: ReturnType<typeof makeInput>) {
  const { manifest } = await emitArtifactManifest(input);
  const files: Record<string, Uint8Array> = {};
  for (const role of Object.keys(manifest.artifacts) as Array<keyof typeof input.urls>) {
    files[input.urls[role]] = input.artifacts[role];
  }
  return { manifest, files, reader: readerFrom(files) };
}

describe("B-002 manifest canonicalization (golden vectors)", () => {
  it("canonical JSON is fixed-order, no whitespace — golden bytes", async () => {
    const { manifest } = await emitArtifactManifest(makeInput());
    const { buildId, ...rest } = manifest;
    const canonical = canonicalManifestJson(rest);
    expect(
      canonical.startsWith(
        '{"formatVersion":1,"target":"browser-wasm","handlerAbiVersion":1,"kernelAbiVersion":1,"appId":"demo","packSha256":',
      ),
    ).toBeTrue();
    // sorted roles
    expect(canonical.indexOf('"capabilities"')).toBeLessThan(canonical.indexOf('"contract"'));
    expect(canonical.indexOf('"contract"')).toBeLessThan(canonical.indexOf('"handlerBundle"'));
    expect(canonical).not.toContain(" "); // no whitespace
  });

  it("buildId is the sha256 of the canonical bytes and changes on any field", async () => {
    const a = await emitArtifactManifest(makeInput());
    const b = await emitArtifactManifest(makeInput());
    expect(a.manifest.buildId).toBe(b.manifest.buildId);
    expect(a.manifest.buildId).toMatch(/^[0-9a-f]{64}$/);
    const changed = await emitArtifactManifest({ ...makeInput(), kernelAbiVersion: 2 });
    expect(changed.manifest.buildId).not.toBe(a.manifest.buildId);
  });
});

describe("B-002 loader fail-closed matrix", () => {
  it("clean deployment loads and exposes verified bytes + buildId", async () => {
    const input = makeInput();
    const { manifest, reader } = await fullFileMap(input);
    const loaded = await loadArtifacts(JSON.stringify(manifest), reader);
    expect(loaded.buildId).toBe(manifest.buildId);
    expect(loaded.bytes.pack).toEqual(packBytes);
  });

  it("tampered artifact bytes fail closed naming the artifact", async () => {
    const input = makeInput();
    const { manifest, files } = await fullFileMap(input);
    // Same-length substitution: tampered, not truncated.
    const tampered = enc(bundleSource.replace("define", "defYne"));
    const tamperedFiles = { ...files, "app.browser.js": tampered };
    try {
      await loadArtifacts(JSON.stringify(manifest), readerFrom(tamperedFiles));
      throw new Error("unreachable");
    } catch (e) {
      const err = e as ArtifactManifestError;
      expect(err.reason).toBe("tampered");
      expect(err.artifact).toBe("handlerBundle");
      expect(err.message).not.toContain("defineBrowserHandlers"); // no content dump
    }
  });

  it("truncated artifacts fail closed with expected vs actual sizes", async () => {
    const input = makeInput();
    const { manifest, files } = await fullFileMap(input);
    const truncatedFiles = { ...files, "kernel.wasm": kernelBytes.slice(0, 4) };
    try {
      await loadArtifacts(JSON.stringify(manifest), readerFrom(truncatedFiles));
      throw new Error("unreachable");
    } catch (e) {
      const err = e as ArtifactManifestError;
      expect(err.reason).toBe("truncated");
      expect(err.artifact).toBe("kernelWasm");
    }
  });

  it("missing artifacts fail closed naming the url", async () => {
    const input = makeInput();
    const { manifest, files } = await fullFileMap(input);
    const missingPack = { ...files };
    delete missingPack["app.qpack"];
    try {
      await loadArtifacts(JSON.stringify(manifest), readerFrom(missingPack));
      throw new Error("unreachable");
    } catch (e) {
      const err = e as ArtifactManifestError;
      expect(err.reason).toBe("missing");
      expect(err.artifact).toBe("pack");
    }
  });

  it("cross-build (mixed deployment) packs fail via packSha256 binding", async () => {
    const { createHash } = require("node:crypto");
    const input = makeInput();
    const { manifest, files } = await fullFileMap(input);
    // Mixed deployment: the deployed pack is from ANOTHER build, and the
    // manifest's per-entry digest was (maliciously or accidentally)
    // updated to match it — the top-level packSha256 binding still
    // catches the incoherence (same length: not truncated).
    const otherPack = enc('{"formatVersion":1,"app":"OTHR"}');
    const forged = JSON.parse(JSON.stringify(manifest)) as {
      artifacts: Record<string, { sha256: string }>;
      packSha256: string;
      buildId: string;
    };
    forged.artifacts.pack.sha256 = createHash("sha256").update(otherPack).digest("hex");
    // Recompute the buildId over the forged manifest: this is the exact
    // case the packSha256 binding exists for — an internally consistent
    // manifest whose top-level pack binding names a DIFFERENT build.
    const { buildIdOf } = await import("../src/artifact-loader");
    const { buildId: _drop, ...rest } = forged;
    void _drop;
    forged.buildId = await buildIdOf(rest);
    const forgedFiles = { ...files, "app.qpack": otherPack };
    try {
      await loadArtifacts(JSON.stringify(forged), readerFrom(forgedFiles));
      throw new Error("unreachable");
    } catch (e) {
      const err = e as ArtifactManifestError;
      expect(err.reason).toBe("cross-build");
      expect(err.artifact).toBe("pack");
    }
  });

  it("unsupported manifest versions fail closed", async () => {
    const input = makeInput();
    const { manifest } = await emitArtifactManifest(input);
    const mutated = JSON.parse(JSON.stringify(manifest)) as { formatVersion: number };
    mutated.formatVersion = 99;
    try {
      await loadArtifacts(JSON.stringify(mutated), async () => new Uint8Array(0));
      throw new Error("unreachable");
    } catch (e) {
      const err = e as ArtifactManifestError;
      expect(err.reason).toBe("unsupported-version");
      expect(err.message).toContain("99");
    }
  });

  it("a swapped manifest (buildId mismatch) fails closed", async () => {
    const a = (await emitArtifactManifest(makeInput())).manifest;
    const b = (await emitArtifactManifest({ ...makeInput(), appId: "other" })).manifest;
    const swapped = { ...a, buildId: b.buildId };
    try {
      await loadArtifacts(JSON.stringify(swapped), async () => new Uint8Array(0));
      throw new Error("unreachable");
    } catch (e) {
      expect((e as ArtifactManifestError).reason).toBe("tampered");
    }
  });
});

describe("B-002 static hosting base paths", () => {
  it("root and non-root base paths resolve to correct artifact URLs", async () => {
    const { manifest } = await emitArtifactManifest(makeInput());
    expect(resolveArtifactUrl("/", manifest.artifacts.pack)).toBe("/app.qpack");
    expect(resolveArtifactUrl("/apps/demo", manifest.artifacts.pack)).toBe(
      "/apps/demo/app.qpack",
    );
    expect(resolveArtifactUrl("/apps/demo/", manifest.artifacts.pack)).toBe(
      "/apps/demo/app.qpack",
    );
  });

  it("manifest version constant is 1 (loader implements exactly this)", () => {
    expect(ARTIFACT_MANIFEST_VERSION).toBe(1);
    expect(buildIdOf).toBeTypeOf("function");
  });
});
