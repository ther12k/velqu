import { createHash } from "node:crypto";
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

export interface PublishedArtifactRecord {
  readonly path: string;
  readonly sha256: string;
  readonly bytes: number;
}

export interface PublishedManifest {
  readonly formatVersion: 1;
  readonly appId: string;
  readonly contractHash: string;
  readonly artifacts: Record<string, PublishedArtifactRecord>;
}

export interface PublishedVerification {
  readonly ok: boolean;
  readonly manifest: PublishedManifest | null;
  readonly errors: readonly string[];
}

/**
 * Read and verify a generated published-manifest.json. Every artifact is
 * rehashed from disk; mismatches are reported by artifact name and expected /
 * actual bytes or digest so package/version drift is diagnosable.
 */
export function verifyPublishedManifest(manifestPath: string): PublishedVerification {
  const errors: string[] = [];
  const absoluteManifest = resolve(manifestPath);
  if (!existsSync(absoluteManifest)) {
    return { ok: false, manifest: null, errors: [`published manifest not found: ${absoluteManifest}`] };
  }

  let manifest: PublishedManifest;
  try {
    manifest = JSON.parse(readFileSync(absoluteManifest, "utf8")) as PublishedManifest;
  } catch (error) {
    return {
      ok: false,
      manifest: null,
      errors: [`published manifest is not valid JSON: ${error instanceof Error ? error.message : String(error)}`],
    };
  }

  if (manifest.formatVersion !== 1) {
    errors.push(`unsupported published manifest formatVersion: ${String(manifest.formatVersion)} (expected 1)`);
  }
  if (!manifest.appId) errors.push("published manifest appId is empty");
  if (!/^[a-f0-9]{32}$/.test(manifest.contractHash)) {
    errors.push("published manifest contractHash is not a 128-bit lowercase hex digest");
  }

  const root = dirname(absoluteManifest);
  const contractRecord = manifest.artifacts?.["contract.json"];
  if (contractRecord) {
    const contractPath = join(root, contractRecord.path);
    if (existsSync(contractPath)) {
      try {
        const contract = JSON.parse(readFileSync(contractPath, "utf8")) as { contractHash?: unknown };
        if (contract.contractHash !== manifest.contractHash) {
          errors.push(
            `contractHash mismatch with contract.json (manifest ${String(manifest.contractHash)}, artifact ${String(contract.contractHash)})`,
          );
        }
      } catch {
        errors.push("contract.json: invalid JSON while checking public contract hash");
      }
    }
  }
  for (const [name, record] of Object.entries(manifest.artifacts ?? {})) {
    const artifactPath = join(root, record.path);
    if (!existsSync(artifactPath)) {
      errors.push(`${name}: artifact not found at ${record.path}`);
      continue;
    }
    const bytes = readFileSync(artifactPath);
    const actualBytes = bytes.byteLength;
    const actualHash = createHash("sha256").update(bytes).digest("hex");
    if (actualBytes !== record.bytes) {
      errors.push(`${name}: byte length mismatch (expected ${record.bytes}, got ${actualBytes})`);
    }
    if (actualHash !== record.sha256) {
      errors.push(`${name}: sha256 mismatch (expected ${record.sha256}, got ${actualHash})`);
    }
  }

  return { ok: errors.length === 0, manifest, errors };
}
