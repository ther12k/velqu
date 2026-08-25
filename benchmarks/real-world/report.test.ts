/**
 * BETA-001-A: report generator tests.
 * Rendering is deterministic and failures are retained in the report.
 */
import { describe, expect, test } from "bun:test";
import { renderReport } from "./report";
import { REAL_WORLD_SUMMARY_FORMAT, type RealWorldSummary } from "./result-schema";

const HASH = "b".repeat(64);

function summaryWithFailures(): RealWorldSummary {
  return {
    format: REAL_WORLD_SUMMARY_FORMAT,
    generatedAt: "2026-08-21T12:00:00Z",
    baseUrl: "http://127.0.0.1:8791",
    durationSec: 2,
    concurrencyLevels: [1, 10],
    environment: { bunVersion: "1.4.0", os: "linux", arch: "x64", commit: "feedc0de" },
    configHashes: { spec: HASH, workloads: HASH, schema: HASH, seed: HASH },
    cells: [
      {
        workload: "W4_1ms",
        concurrency: 1,
        totalRequests: 500,
        errors: 0,
        statusMismatches: 0,
        rps: 250,
        p50Us: 1100,
        p95Us: 1300,
        p99Us: 1500,
        maxUs: 2000,
      },
      {
        workload: "W4_1ms",
        concurrency: 10,
        totalRequests: 900,
        errors: 3,
        statusMismatches: 2,
        rps: 450,
        p50Us: 9000,
        p95Us: 11000,
        p99Us: 13000,
        maxUs: 15000,
      },
    ],
    raw: "raw.jsonl",
  };
}

describe("real-world report generator", () => {
  test("renders each workload table with percentile and error columns", () => {
    const md = renderReport(summaryWithFailures());
    expect(md).toContain("# Real-World Benchmark Report");
    expect(md).toContain("## W4_1ms");
    expect(md).toContain("| 1 | 500 | 250 | 1100 | 1300 | 1500 | 2000 | 0 | 0 |");
    expect(md).toContain("| 10 | 900 | 450 | 9000 | 11000 | 13000 | 15000 | 3 | 2 |");
  });

  test("failures are retained in a dedicated section, not dropped", () => {
    const md = renderReport(summaryWithFailures());
    expect(md).toContain("## Retained failures");
    expect(md).toContain("W4_1ms c=10: 3 errors, 2 status mismatches out of 900 requests");
  });

  test("clean run reports no failures explicitly", () => {
    const s = summaryWithFailures();
    for (const c of s.cells) {
      c.errors = 0;
      c.statusMismatches = 0;
    }
    const md = renderReport(s);
    expect(md).toContain("No request errors or status mismatches in this run.");
    expect(md).not.toContain("c=10: 3 errors");
  });

  test("protocol section echoes environment and config hashes deterministically", () => {
    const md1 = renderReport(summaryWithFailures());
    const md2 = renderReport(summaryWithFailures());
    expect(md1).toBe(md2);
    expect(md1).toContain("bun 1.4.0");
    expect(md1).toContain("commit feedc0de");
    expect(md1).toContain(`seed sha256 ${HASH}`);
  });
});
