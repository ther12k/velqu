import { describe, expect, test } from "bun:test";
import { $ } from "bun";
import { join } from "node:path";
import { writeFileSync, readFileSync } from "node:fs";

const root = join(import.meta.dir, "..");
const thresholdsFile = join(root, "benchmarks", "gate-thresholds.json");
const originalThresholds = readFileSync(thresholdsFile, "utf8");

describe("throughput-latency-gate verification", () => {
  test("clean pass on baseline bounds", async () => {
    const proc = Bun.spawnSync(["bun", "scripts/throughput-latency-gate.ts", "--duration-secs", "1", "--routes", "C0"], {
      cwd: root,
      stdout: "pipe",
      stderr: "pipe",
    });
    const stdout = new TextDecoder().decode(proc.stdout);
    const stderr = new TextDecoder().decode(proc.stderr);
    expect(proc.exitCode).toBe(0);
    expect(stdout).toContain("throughput-latency-gate: PASS");
    expect(stdout).toContain("C0");
  });

  test("fail-closed on throughput regression (measured < minRps)", async () => {
    const modified = JSON.parse(originalThresholds);
    // Set impossibly high minRps threshold
    modified.throughputLatency.routes.C0.bounds.minRps = 9999999;
    writeFileSync(thresholdsFile, JSON.stringify(modified, null, 2));

    try {
      const proc = Bun.spawnSync(["bun", "scripts/throughput-latency-gate.ts", "--duration-secs", "1", "--routes", "C0"], {
        cwd: root,
        stdout: "pipe",
        stderr: "pipe",
      });
      const stdout = new TextDecoder().decode(proc.stdout);
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("FAIL — 1 regression threshold violation(s)");
      expect(stderr).toContain("below minimum threshold 9999999 req/s");
    } finally {
      writeFileSync(thresholdsFile, originalThresholds);
    }
  });

  test("fail-closed on latency regression (measured > maxLatencyUs)", async () => {
    const modified = JSON.parse(originalThresholds);
    // Set impossibly low latency threshold (0.1 us)
    modified.throughputLatency.routes.C0.bounds.maxP95Us = 0.1;
    writeFileSync(thresholdsFile, JSON.stringify(modified, null, 2));

    try {
      const proc = Bun.spawnSync(["bun", "scripts/throughput-latency-gate.ts", "--duration-secs", "1", "--routes", "C0"], {
        cwd: root,
        stdout: "pipe",
        stderr: "pipe",
      });
      const stdout = new TextDecoder().decode(proc.stdout);
      const stderr = new TextDecoder().decode(proc.stderr);
      expect(proc.exitCode).toBe(1);
      expect(stderr).toContain("FAIL — 1 regression threshold violation(s)");
      expect(stderr).toContain("exceeds maximum threshold 0.1 µs");
    } finally {
      writeFileSync(thresholdsFile, originalThresholds);
    }
  });

  test("fail-closed on non-existent route filter", async () => {
    const proc = Bun.spawnSync(["bun", "scripts/throughput-latency-gate.ts", "--routes", "NONEXISTENT_ROUTE"], {
      cwd: root,
      stdout: "pipe",
      stderr: "pipe",
    });
    expect(proc.exitCode).toBe(2);
  });
});
