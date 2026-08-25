/**
 * BETA-001-A: result schema validation tests.
 * Positive fixture plus negatives: missing cells, unordered percentiles,
 * error-count overflow, missing/invalid config hashes, bad format.
 */
import { describe, expect, test } from "bun:test";
import {
  REAL_WORLD_SUMMARY_FORMAT,
  type RealWorldSummary,
  validateRealWorldSummary,
} from "./result-schema";

const HASH = "a".repeat(64);

function validSummary(): RealWorldSummary {
  return {
    format: REAL_WORLD_SUMMARY_FORMAT,
    generatedAt: "2026-08-21T00:00:00Z",
    baseUrl: "http://127.0.0.1:3000",
    durationSec: 10,
    concurrencyLevels: [1, 10],
    environment: { bunVersion: "1.4.0", os: "linux", arch: "x64", commit: "0123456789abcdef" },
    configHashes: { spec: HASH, workloads: HASH, schema: HASH, seed: HASH, versions: HASH },
    cells: [
      {
        workload: "W1",
        concurrency: 1,
        totalRequests: 100,
        errors: 0,
        statusMismatches: 0,
        rps: 1000,
        p50Us: 10,
        p95Us: 20,
        p99Us: 30,
        maxUs: 40,
      },
      {
        workload: "W1",
        concurrency: 10,
        totalRequests: 200,
        errors: 2,
        statusMismatches: 1,
        rps: 2000,
        p50Us: 10,
        p95Us: 20,
        p99Us: 30,
        maxUs: 40,
      },
    ],
    raw: "raw.jsonl",
  };
}

describe("real-world result schema", () => {
  test("valid summary passes with no errors", () => {
    expect(validateRealWorldSummary(validSummary(), ["W1"], [1, 10])).toEqual([]);
  });

  test("missing workload/concurrency cell is reported", () => {
    const s = validSummary();
    expect(validateRealWorldSummary(s, ["W1", "W2"], [1, 10])).toContain("missing cell: W2 c=1");
    expect(validateRealWorldSummary(s, ["W1"], [1, 10, 50])).toContain("missing cell: W1 c=50");
  });

  test("percentile ordering violations are reported", () => {
    const s = validSummary();
    s.cells[0].p50Us = 25;
    const errs = validateRealWorldSummary(s, ["W1"], [1, 10]);
    expect(errs.some((e) => e.includes("p50Us > p95Us"))).toBe(true);
  });

  test("error counts exceeding total requests are reported", () => {
    const s = validSummary();
    s.cells[0].errors = 60;
    s.cells[0].statusMismatches = 60;
    const errs = validateRealWorldSummary(s, ["W1"], [1, 10]);
    expect(errs.some((e) => e.includes("exceed totalRequests"))).toBe(true);
  });

  test("missing or malformed config hashes are reported per key", () => {
    const s = validSummary();
    s.configHashes.seed = "nothex";
    s.configHashes.versions = "short";
    const errs = validateRealWorldSummary(s, ["W1"], [1, 10]);
    expect(errs).toContain("configHashes.seed must be a sha256 hex digest");
    expect(errs).toContain("configHashes.versions must be a sha256 hex digest");
    delete (s.configHashes as Record<string, string>).spec;
    const errs2 = validateRealWorldSummary(s, ["W1"], [1, 10]);
    expect(errs2).toContain("configHashes.spec must be a sha256 hex digest");
  });

  test("wrong format string and missing environment fields are reported", () => {
    const s = validSummary();
    s.format = "something-else";
    s.environment.commit = "";
    const errs = validateRealWorldSummary(s, ["W1"], [1, 10]);
    expect(errs.some((e) => e.startsWith("format must be"))).toBe(true);
    expect(errs).toContain("environment.commit is required");
  });
});
