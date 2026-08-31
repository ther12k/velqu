/**
 * @velqu/cli — pack inspection helper (M4A-002-A).
 *
 * Inspects a compiled QPack artifact (app.qpack) without running any code:
 * extracts format version, runtime identity, route counts, schemas, and
 * linked capability inventory.
 */

import { existsSync, readFileSync } from "node:fs";

export interface PackInspectionReport {
  status: "ok" | "error";
  file: string;
  formatVersion?: number;
  appId?: string;
  contractHash?: string;
  engine?: {
    name: string;
    version: string;
    rquickjs?: string;
    runtimeAbi?: number;
  };
  routesCount?: number;
  schemasCount?: number;
  policiesCount?: number;
  capabilities?: string[];
  bundleSha256?: string;
  error?: string;
}

export function inspectPack(filePath: string): PackInspectionReport {
  if (!existsSync(filePath)) {
    return {
      status: "error",
      file: filePath,
      error: `pack file not found: ${filePath}`,
    };
  }

  try {
    const raw = readFileSync(filePath, "utf8");
    const pack = JSON.parse(raw);

    const formatVersion = Number(pack.formatVersion ?? 1);
    const appId = String(pack.appId ?? "unknown");
    const contractHash = String(pack.contractHash ?? "");
    const engine = pack.engine ?? { name: "unknown", version: "unknown" };
    const routes = Array.isArray(pack.routes) ? pack.routes : [];
    const schemas = Array.isArray(pack.schemaManifest) ? pack.schemaManifest : [];
    const policies = Array.isArray(pack.policyManifest) ? pack.policyManifest : [];
    const capabilities = Array.isArray(pack.capabilities) ? pack.capabilities : [];
    const bundleSha256 = pack.integrity?.bundleSha256 ?? "";

    return {
      status: "ok",
      file: filePath,
      formatVersion,
      appId,
      contractHash,
      engine,
      routesCount: routes.length,
      schemasCount: schemas.length,
      policiesCount: policies.length,
      capabilities,
      bundleSha256,
    };
  } catch (e) {
    return {
      status: "error",
      file: filePath,
      error: `failed to parse pack: ${e instanceof Error ? e.message : String(e)}`,
    };
  }
}
