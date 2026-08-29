/**
 * BETA-001-A: workload configuration contract tests over workloads.json.
 * Guards the scaffold the harness drives: ids, methods, paths, expected
 * statuses, W4 latency values, and concurrency levels.
 */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import type { RealWorldWorkload } from "./result-schema";

interface WorkloadsFile {
  workloads: RealWorldWorkload[];
  concurrencyLevels: number[];
  durationSec: number;
}

const config: WorkloadsFile = JSON.parse(
  readFileSync(import.meta.dir + "/workloads.json", "utf8"),
);
const PATHS = [/^\/api\/users\/[\w-]+$/, /^\/api\/orders$/, /^\/api\/products/, /^\/api\/bench\/io\?ms=\d+$/, /^\/api\/bench\/fanout\?n=[124]&ms=\d+$/];

describe("real-world workload config", () => {
  test("every workload declares id, method, path, and expectedStatus", () => {
    for (const w of config.workloads) {
      expect(w.id).toBeTruthy();
      expect(["GET", "POST"]).toContain(w.method);
      expect(w.path).toBeTruthy();
      expect(Number.isInteger(w.expectedStatus)).toBe(true);
      expect(w.expectedStatus).toBeGreaterThanOrEqual(200);
      expect(w.expectedStatus).toBeLessThan(300);
      expect(PATHS.some((re) => re.test(w.path))).toBe(true);
    }
  });

  test("workload ids are unique and cover W1..W3 plus the W4 latency matrix", () => {
    const ids = config.workloads.map((w) => w.id);
    expect(new Set(ids).size).toBe(ids.length);
    for (const required of ["W1", "W2", "W3"]) {
      expect(ids).toContain(required);
    }
    const w4ms = ids
      .filter((id) => id.startsWith("W4_"))
      .map((id) => Number(id.replace("W4_", "").replace("ms", "")))
      .sort((a, b) => a - b);
    expect(w4ms).toEqual([1, 5, 10, 25]);
  });

  test("W4 latency values match the SPEC matrix and stay bounded", () => {
    for (const w of config.workloads.filter((w) => w.id.startsWith("W4_"))) {
      const ms = Number(new URL(`http://x${w.path}`).searchParams.get("ms"));
      expect(ms).toBeGreaterThan(0);
      expect(ms).toBeLessThanOrEqual(1000);
      const idMs = Number(w.id.replace("W4_", "").replace("ms", ""));
      expect(ms).toBe(idMs);
    }
  });

  test("W1/W2 carry an Authorization header; W2 has an items body", () => {
    const w1 = config.workloads.find((w) => w.id === "W1")!;
    expect(w1.headers?.Authorization).toMatch(/^Bearer /);
    const w2 = config.workloads.find((w) => w.id === "W2")!;
    expect(w2.headers?.Authorization).toMatch(/^Bearer /);
    expect(Array.isArray((w2.body as { items: unknown[] }).items)).toBe(true);
  });

  test("concurrency levels are positive, unique, ascending; duration positive", () => {
    const c = config.concurrencyLevels;
    expect(c.length).toBeGreaterThan(0);
    expect(new Set(c).size).toBe(c.length);
    expect([...c].sort((a, b) => a - b)).toEqual(c);
    for (const n of c) expect(n).toBeGreaterThan(0);
    expect(config.durationSec).toBeGreaterThan(0);
  });
});
