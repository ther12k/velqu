import { describe, expect, test, afterAll } from "bun:test";
import { join } from "node:path";
import { writeFileSync, readFileSync, unlinkSync, existsSync } from "node:fs";

const root = join(import.meta.dir, "..");
const thresholdsFile = join(root, "benchmarks", "gate-thresholds.json");
const originalDoc = JSON.parse(readFileSync(thresholdsFile, "utf8"));

const tempFiles: string[] = [];

function createTempThresholds(modifier: (doc: any) => void): string {
  const docCopy = JSON.parse(JSON.stringify(originalDoc));
  modifier(docCopy);
  const tempPath = join(root, "benchmarks", `gate-thresholds-test-${Date.now()}-${Math.random().toString(36).slice(2)}.json`);
  writeFileSync(tempPath, JSON.stringify(docCopy, null, 2));
  tempFiles.push(tempPath);
  return tempPath;
}

afterAll(() => {
  for (const f of tempFiles) {
    if (existsSync(f)) {
      try {
        unlinkSync(f);
      } catch {}
    }
  }
});

describe("throughput-latency-gate verification", () => {
  test("clean pass on baseline bounds", async () => {
    const proc = Bun.spawnSync(["bun", "scripts/throughput-latency-gate.ts", "--duration-secs", "1", "--routes", "C0"], {
      cwd: root,
      stdout: "pipe",
      stderr: "pipe",
    });
    const stdout = new TextDecoder().decode(proc.stdout);
    expect(proc.exitCode).toBe(0);
    expect(stdout).toContain("throughput-latency-gate: PASS");
    expect(stdout).toContain("C0");
  });

  test("fail-closed on throughput regression (measured < minRps)", async () => {
    const tempPath = createTempThresholds((doc) => {
      doc.throughputLatency.routes.C0.bounds.minRps = 9999999;
    });

    const proc = Bun.spawnSync(["bun", "scripts/throughput-latency-gate.ts", "--thresholds", tempPath, "--duration-secs", "1", "--routes", "C0"], {
      cwd: root,
      stdout: "pipe",
      stderr: "pipe",
    });
    const stderr = new TextDecoder().decode(proc.stderr);
    expect(proc.exitCode).toBe(1);
    expect(stderr).toContain("FAIL — 1 regression threshold violation(s)");
    expect(stderr).toContain("below minimum threshold 9999999 req/s");
  });

  test("fail-closed on latency regression (measured > maxLatencyUs)", async () => {
    const tempPath = createTempThresholds((doc) => {
      doc.throughputLatency.routes.C0.bounds.maxP95Us = 0.1;
    });

    const proc = Bun.spawnSync(["bun", "scripts/throughput-latency-gate.ts", "--thresholds", tempPath, "--duration-secs", "1", "--routes", "C0"], {
      cwd: root,
      stdout: "pipe",
      stderr: "pipe",
    });
    const stderr = new TextDecoder().decode(proc.stderr);
    expect(proc.exitCode).toBe(1);
    expect(stderr).toContain("FAIL — 1 regression threshold violation(s)");
    expect(stderr).toContain("exceeds maximum threshold 0.1 µs");
  });

  test("fail-closed on unknown route ID in --routes filter (e.g. C0,TYPO)", async () => {
    const proc = Bun.spawnSync(["bun", "scripts/throughput-latency-gate.ts", "--routes", "C0,TYPO"], {
      cwd: root,
      stdout: "pipe",
      stderr: "pipe",
    });
    const stderr = new TextDecoder().decode(proc.stderr);
    expect(proc.exitCode).toBe(2);
    expect(stderr).toContain("unknown route(s) in --routes: TYPO");
  });

  test("fail-closed on invalid non-positive or null bound in threshold file", async () => {
    const tempPath = createTempThresholds((doc) => {
      doc.throughputLatency.routes.C0.bounds.minRps = null;
    });

    const proc = Bun.spawnSync(["bun", "scripts/throughput-latency-gate.ts", "--thresholds", tempPath, "--routes", "C0"], {
      cwd: root,
      stdout: "pipe",
      stderr: "pipe",
    });
    const stderr = new TextDecoder().decode(proc.stderr);
    expect(proc.exitCode).toBe(2);
    expect(stderr).toContain("invalid or non-positive bound 'minRps'");
  });
});
