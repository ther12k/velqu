/**
 * Real-world benchmark raw-sample retention (BETA-001-D).
 *
 * Policy: every run retains its complete per-request raw JSONL as a
 * deterministic gzip archive committed next to summary.json. The archive is
 * byte-reproducible (gzip mtime pinned to 0, no filename field), so the same
 * raw rows always produce the same archive and sha256. A RETENTION.md
 * manifest records row counts, sizes, and both hashes so parity is checkable
 * without decompressing.
 *
 * Usage:
 *   bun retain.ts --run path/to/runDir
 */

import { createHash } from "node:crypto";
import { readFileSync, statSync, writeFileSync, existsSync } from "node:fs";
import { gunzipSync, gzipSync } from "node:zlib";

export interface RetentionManifest {
  run: string;
  rows: number;
  rawBytes: number;
  gzBytes: number;
  rawSha256: string;
  gzSha256: string;
  policy: string;
}

export const RETENTION_POLICY =
  "raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only";

export function countRows(raw: string): number {
  return raw.split("\n").filter((l) => l.trim().length > 0).length;
}

export function deterministicGzip(data: Buffer): Buffer {
  return gzipSync(data, { mtime: 0, level: 9 });
}

export function verifyArchive(gz: Buffer, expectedRawSha256: string): { ok: boolean; rows: number } {
  const raw = gunzipSync(gz);
  const sha = createHash("sha256").update(raw).digest("hex");
  return { ok: sha === expectedRawSha256, rows: countRows(raw.toString("utf8")) };
}

export function buildRetentionManifest(runDir: string): RetentionManifest {
  const rawPath = `${runDir}/raw.jsonl`;
  if (!existsSync(rawPath)) throw new Error(`retain.ts: ${rawPath} not found`);
  const raw = readFileSync(rawPath);
  const gz = deterministicGzip(raw);
  writeFileSync(`${runDir}/raw.jsonl.gz`, gz);
  const manifest: RetentionManifest = {
    run: runDir,
    rows: countRows(raw.toString("utf8")),
    rawBytes: statSync(rawPath).size,
    gzBytes: gz.length,
    rawSha256: createHash("sha256").update(raw).digest("hex"),
    gzSha256: createHash("sha256").update(gz).digest("hex"),
    policy: RETENTION_POLICY,
  };
  writeFileSync(
    `${runDir}/RETENTION.md`,
    `# Raw-sample retention\n\n` +
      `- Policy: ${manifest.policy}\n` +
      `- Rows: ${manifest.rows}\n` +
      `- raw.jsonl: ${manifest.rawBytes} bytes, sha256 ${manifest.rawSha256}\n` +
      `- raw.jsonl.gz: ${manifest.gzBytes} bytes, sha256 ${manifest.gzSha256}\n` +
      `- Verify: \`gunzip -c raw.jsonl.gz | sha256sum\` must equal the raw.jsonl sha256 above.\n`,
  );
  return manifest;
}

function main() {
  const argv = process.argv.slice(2);
  const i = argv.indexOf("--run");
  if (i < 0 || !argv[i + 1]) throw new Error("--run <dir> is required");
  const manifest = buildRetentionManifest(argv[i + 1]);
  console.log(`retain.ts: ${manifest.rows} rows, ${manifest.gzBytes} gz bytes, sha256 ${manifest.gzSha256.slice(0, 16)}…`);
}

if (import.meta.main) {
  main();
}
