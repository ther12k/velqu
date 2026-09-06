/**
 * BWASM-B-002 — content-addressed browser artifact manifest and loader.
 *
 * Binds one build's artifacts (pack, kernel, handler bundle, schemas,
 * contract, capability declarations, source-map metadata) to a single
 * build ID with SHA-256 digests, sizes, media types, and relative URLs
 * (static hosting under any base path). The loader verifies EVERY
 * artifact's bytes before anything is instantiated or imported —
 * integrity before activation (ADR-0026 semantics; ADR-0037 §4).
 *
 * Browser-pure: hashing uses the standard WebCrypto `crypto.subtle`
 * (browsers, Bun, Node ≥18) — no node:* imports. Fail closed on:
 * tampered, truncated, missing, cross-build (mixed-deployment), and
 * unsupported-version artifacts — each error names the failing
 * artifact without dumping its content.
 */

/** Manifest format version (unsupported versions fail closed). */
export const ARTIFACT_MANIFEST_VERSION = 1;

/** Canonical media types for each artifact role. */
export const MEDIA_TYPES = {
  pack: "application/x-velqu-qpack",
  kernelWasm: "application/wasm",
  handlerBundle: "text/javascript",
  manifest: "application/json",
  contract: "application/json",
  schemas: "application/json",
  capabilities: "application/json",
  sourceMap: "application/json",
} as const;

export type ArtifactRole = keyof typeof MEDIA_TYPES;

export interface ArtifactEntry {
  /** Stable role name (manifest key). */
  readonly role: ArtifactRole;
  /** Deploy-relative URL (POSIX separators; resolved against the base path). */
  readonly url: string;
  readonly sha256: string;
  readonly bytes: number;
  readonly mediaType: (typeof MEDIA_TYPES)[ArtifactRole];
}

export interface BrowserArtifactManifest {
  readonly formatVersion: typeof ARTIFACT_MANIFEST_VERSION;
  /** Content identity of the whole build: sha256 over the canonical manifest. */
  readonly buildId: string;
  readonly target: "browser-wasm";
  readonly handlerAbiVersion: number;
  readonly kernelAbiVersion: number;
  readonly appId: string;
  readonly packSha256: string;
  readonly artifacts: Readonly<Record<ArtifactRole, ArtifactEntry>>;
}

export class ArtifactManifestError extends Error {
  readonly artifact: string;
  readonly reason:
    | "unsupported-version"
    | "missing"
    | "truncated"
    | "tampered"
    | "cross-build";
  constructor(artifact: string, reason: ArtifactManifestError["reason"], detail: string) {
    super(`[@velqu/browser-runtime:artifact:${reason}] ${artifact}: ${detail}`);
    this.name = "ArtifactManifestError";
    this.artifact = artifact;
    this.reason = reason;
  }
}

/** WebCrypto sha256 → hex (browser standard; Bun/Node ≥18 also provide it). */
export async function sha256Hex(bytes: Uint8Array): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", bytes as BufferSource);
  return [...new Uint8Array(digest)].map((b) => b.toString(16).padStart(2, "0")).join("");
}

// ---------------------------------------------------------------------------
// Canonical manifest serialization (golden-vector covered)
// ---------------------------------------------------------------------------

/**
 * Canonical JSON for the manifest: fixed key order, sorted artifact
 * roles, no whitespace — the buildId binds to exactly these bytes
 * (ADR-0023 canonicalization posture).
 */
export function canonicalManifestJson(
  manifest: Omit<BrowserArtifactManifest, "buildId">,
): string {
  const roles = Object.keys(manifest.artifacts).sort() as ArtifactRole[];
  const ordered = {
    formatVersion: manifest.formatVersion,
    target: manifest.target,
    handlerAbiVersion: manifest.handlerAbiVersion,
    kernelAbiVersion: manifest.kernelAbiVersion,
    appId: manifest.appId,
    packSha256: manifest.packSha256,
    artifacts: Object.fromEntries(
      roles.map((role) => {
        const a = manifest.artifacts[role];
        return [role, { mediaType: a.mediaType, sha256: a.sha256, bytes: a.bytes, url: a.url }];
      }),
    ),
  };
  return JSON.stringify(ordered);
}

export async function buildIdOf(
  manifest: Omit<BrowserArtifactManifest, "buildId">,
): Promise<string> {
  return sha256Hex(new TextEncoder().encode(canonicalManifestJson(manifest)));
}

// ---------------------------------------------------------------------------
// Emission (compiler/tooling side): produce the manifest from a set
// ---------------------------------------------------------------------------

export interface EmitInput {
  readonly appId: string;
  readonly handlerAbiVersion: number;
  readonly kernelAbiVersion: number;
  readonly packSha256: string;
  /** role → artifact bytes exactly as deployed. */
  readonly artifacts: Record<ArtifactRole, Uint8Array>;
  /** Deploy-relative URL per role (e.g. "app.qpack", "kernel.wasm"). */
  readonly urls: Record<ArtifactRole, string>;
}

export async function emitArtifactManifest(input: EmitInput): Promise<{
  manifest: BrowserArtifactManifest;
  manifestJson: string;
}> {
  const artifacts = {} as Record<ArtifactRole, ArtifactEntry>;
  for (const role of Object.keys(input.artifacts) as ArtifactRole[]) {
    const bytes = input.artifacts[role];
    artifacts[role] = {
      role,
      url: input.urls[role],
      sha256: await sha256Hex(bytes),
      bytes: bytes.byteLength,
      mediaType: MEDIA_TYPES[role],
    };
  }
  const withoutBuildId = {
    formatVersion: ARTIFACT_MANIFEST_VERSION as typeof ARTIFACT_MANIFEST_VERSION,
    target: "browser-wasm" as const,
    handlerAbiVersion: input.handlerAbiVersion,
    kernelAbiVersion: input.kernelAbiVersion,
    appId: input.appId,
    packSha256: input.packSha256,
    artifacts,
  };
  const manifest: BrowserArtifactManifest = {
    ...withoutBuildId,
    buildId: await buildIdOf(withoutBuildId),
  };
  return {
    manifest,
    manifestJson: JSON.stringify(manifest, null, 2) + "\n",
  };
}

// ---------------------------------------------------------------------------
// Loader (deployment side): verify everything BEFORE activation
// ---------------------------------------------------------------------------

export interface LoadedArtifacts {
  readonly buildId: string;
  readonly manifest: BrowserArtifactManifest;
  /** Verified bytes per role, ready for instantiation/import. */
  readonly bytes: Readonly<Record<ArtifactRole, Uint8Array>>;
}

/** Async artifact reader (fetch in browsers; fs in tooling/tests). */
export type ArtifactReader = (url: string) => Promise<Uint8Array>;

/**
 * Load and verify a deployed artifact set. Every artifact is
 * digest-checked and size-checked against the manifest BEFORE the
 * caller may instantiate WASM or import the handler bundle. The
 * manifest itself is bound to its buildId (canonical-bytes digest), so
 * a swapped manifest is also detected.
 */
export async function loadArtifacts(
  manifestJson: string,
  readArtifact: ArtifactReader,
): Promise<LoadedArtifacts> {
  let manifest: BrowserArtifactManifest;
  try {
    manifest = JSON.parse(manifestJson) as BrowserArtifactManifest;
  } catch {
    throw new ArtifactManifestError(
      "browser-manifest.json",
      "tampered",
      "manifest is not valid JSON",
    );
  }
  if (manifest.formatVersion !== ARTIFACT_MANIFEST_VERSION) {
    throw new ArtifactManifestError(
      "browser-manifest.json",
      "unsupported-version",
      `manifest formatVersion ${String(manifest.formatVersion)} unsupported (loader implements ${ARTIFACT_MANIFEST_VERSION})`,
    );
  }
  if (manifest.target !== "browser-wasm") {
    throw new ArtifactManifestError(
      "browser-manifest.json",
      "unsupported-version",
      `manifest target ${String(manifest.target)} is not browser-wasm`,
    );
  }
  // buildId binding: the manifest binds to its own canonical bytes.
  const { artifacts, buildId, ...rest } = manifest;
  const recomputed = await buildIdOf({ ...rest, artifacts });
  if (recomputed !== buildId) {
    throw new ArtifactManifestError(
      "browser-manifest.json",
      "tampered",
      `manifest buildId mismatch (declared ${buildId.slice(0, 12)}…, canonical ${recomputed.slice(0, 12)}…)`,
    );
  }

  const bytes: Record<ArtifactRole, Uint8Array> = {} as Record<ArtifactRole, Uint8Array>;
  for (const role of Object.keys(manifest.artifacts) as ArtifactRole[]) {
    const entry = manifest.artifacts[role];
    let raw: Uint8Array;
    try {
      raw = await readArtifact(entry.url);
    } catch {
      throw new ArtifactManifestError(role, "missing", `artifact file missing at ${entry.url}`);
    }
    if (raw.byteLength !== entry.bytes) {
      throw new ArtifactManifestError(
        role,
        "truncated",
        `expected ${entry.bytes} bytes, read ${raw.byteLength}`,
      );
    }
    const digest = await sha256Hex(raw);
    if (digest !== entry.sha256) {
      throw new ArtifactManifestError(
        role,
        "tampered",
        `sha256 mismatch (expected ${entry.sha256.slice(0, 12)}…, got ${digest.slice(0, 12)}…)`,
      );
    }
    bytes[role] = raw;
  }

  // Cross-build check: the pack digest must equal the manifest's
  // top-level packSha256 (single-build coherence).
  if (bytes.pack) {
    const packDigest = await sha256Hex(bytes.pack);
    if (packDigest !== manifest.packSha256) {
      throw new ArtifactManifestError(
        "pack",
        "cross-build",
        "pack bytes do not match the manifest's packSha256 (mixed deployment)",
      );
    }
  }

  return { buildId, manifest, bytes };
}

// ---------------------------------------------------------------------------
// Base-path resolution (root or non-root static hosting)
// ---------------------------------------------------------------------------

export function resolveArtifactUrl(baseUrl: string, entry: { readonly url: string }): string {
  const base = baseUrl.replace(/\/+$/, "");
  return `${base}/${entry.url.replace(/^\/+/, "")}`;
}
