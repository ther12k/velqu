/**
 * BETA-001-D: raw-sample retention tests.
 * Archives are deterministic (byte-identical across runs), lossless (row
 * counts and hashes round-trip), and the manifest records both hashes.
 */
import { describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { gunzipSync } from "node:zlib";
import {
  RETENTION_POLICY,
  buildRetentionManifest,
  countRows,
  deterministicGzip,
  verifyArchive,
} from "./retain";
import { mkdirSync, rmSync, writeFileSync, readFileSync } from "node:fs";

const TMP = "/tmp/velqu-retain-test";

function makeRun(rows: string[]): string {
  const dir = `${TMP}/run-${Math.random().toString(36).slice(2)}`;
  mkdirSync(dir, { recursive: true });
  writeFileSync(`${dir}/raw.jsonl`, rows.join("\n") + "\n");
  return dir;
}

const ROWS = [
  JSON.stringify({ workload: "W1", concurrency: 1, latencyUs: 100, status: 200, ok: true, error: null }),
  JSON.stringify({ workload: "W1", concurrency: 10, latencyUs: 250, status: 500, ok: false, error: null }),
  JSON.stringify({ workload: "W4_1ms", concurrency: 1, latencyUs: 1370, status: null, ok: false, error: "conn refused" }),
];

describe("real-world raw-sample retention", () => {
  test("deterministicGzip is byte-identical across runs (pinned mtime)", () => {
    const a = deterministicGzip(Buffer.from("same input"));
    const b = deterministicGzip(Buffer.from("same input"));
    expect(Buffer.compare(a, b)).toBe(0);
  });

  test("archive round-trips losslessly and counts rows", () => {
    const raw = ROWS.join("\n") + "\n";
    const gz = deterministicGzip(Buffer.from(raw));
    expect(gunzipSync(gz).toString("utf8")).toBe(raw);
    expect(countRows(raw)).toBe(3);
    expect(countRows(raw + "\n")).toBe(3); // trailing newline tolerant
  });

  test("verifyArchive accepts the matching raw hash and rejects drift", () => {
    const raw = Buffer.from(ROWS.join("\n") + "\n");
    const sha = createHash("sha256").update(raw).digest("hex");
    expect(verifyArchive(deterministicGzip(raw), sha)).toEqual({ ok: true, rows: 3 });
    expect(verifyArchive(deterministicGzip(Buffer.from("tampered")), sha).ok).toBe(false);
  });

  test("buildRetentionManifest writes archive + manifest with consistent hashes", () => {
    rmSync(TMP, { recursive: true, force: true });
    const dir = makeRun(ROWS);
    const m = buildRetentionManifest(dir);
    expect(m.rows).toBe(3);
    expect(m.policy).toBe(RETENTION_POLICY);
    const gzOnDisk = readFileSync(`${dir}/raw.jsonl.gz`);
    expect(createHash("sha256").update(gzOnDisk).digest("hex")).toBe(m.gzSha256);
    const md = readFileSync(`${dir}/RETENTION.md`, "utf8");
    expect(md).toContain(m.rawSha256);
    expect(md).toContain(m.gzSha256);
    expect(md).toContain("Rows: 3");
    expect(verifyArchive(gzOnDisk, m.rawSha256).ok).toBe(true);
  });

  test("full rebuild from identical rows reproduces identical archive bytes", () => {
    rmSync(TMP, { recursive: true, force: true });
    const d1 = makeRun(ROWS);
    const d2 = makeRun(ROWS);
    const m1 = buildRetentionManifest(d1);
    const m2 = buildRetentionManifest(d2);
    expect(m1.gzSha256).toBe(m2.gzSha256);
    expect(readFileSync(`${d1}/raw.jsonl.gz`)).toEqual(readFileSync(`${d2}/raw.jsonl.gz`));
  });
});
