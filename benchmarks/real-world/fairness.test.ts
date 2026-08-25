/**
 * BETA-001-C: fairness audit tests.
 * Identical contracts pass; any contract drift, protocol drift, environment
 * drift, grid mismatch, or retained failure fails the audit loudly.
 */
import { describe, expect, test } from "bun:test";
import { auditFairness, renderFairnessReport, type FairnessFinding } from "./fairness";
import { REAL_WORLD_SUMMARY_FORMAT, type RealWorldSummary } from "./result-schema";

const HASH = "c".repeat(64);

function summary(baseUrl: string, overrides: Partial<RealWorldSummary> = {}): RealWorldSummary {
  return {
    format: REAL_WORLD_SUMMARY_FORMAT,
    generatedAt: "2026-08-25T00:00:00Z",
    baseUrl,
    durationSec: 10,
    concurrencyLevels: [1, 10],
    environment: { bunVersion: "1.4.0", os: "linux", arch: "x64", commit: "abc123" },
    configHashes: { spec: HASH, workloads: HASH, schema: HASH, seed: HASH, versions: HASH },
    cells: [
      { workload: "W1", concurrency: 1, totalRequests: 100, errors: 0, statusMismatches: 0, rps: 100, p50Us: 1, p95Us: 2, p99Us: 3, maxUs: 4 },
      { workload: "W1", concurrency: 10, totalRequests: 200, errors: 0, statusMismatches: 0, rps: 200, p50Us: 1, p95Us: 2, p99Us: 3, maxUs: 4 },
    ],
    raw: "raw.jsonl",
    ...overrides,
  };
}

const failed = (findings: FairnessFinding[], check: string) =>
  findings.find((f) => f.check === check && f.status === "FAIL");

describe("real-world fairness audit", () => {
  test("identical contracts across two candidates pass every check", () => {
    const findings = auditFairness([summary("http://a"), summary("http://b")]);
    expect(findings.filter((f) => f.status === "FAIL")).toEqual([]);
    expect(findings.filter((f) => f.status === "PASS").length).toBeGreaterThan(5);
  });

  test("single-summary run set fails immediately", () => {
    const findings = auditFairness([summary("http://a")]);
    expect(failed(findings, "run-set")).toBeTruthy();
  });

  test("dataset drift (seed hash) fails the audit", () => {
    const b = summary("http://b");
    b.configHashes.seed = "d".repeat(64);
    const findings = auditFairness([summary("http://a"), b]);
    expect(failed(findings, "hash.seed")).toBeTruthy();
    expect(failed(findings, "hash.workloads")).toBeFalsy();
  });

  test("pin drift (versions hash) fails the audit", () => {
    const b = summary("http://b");
    b.configHashes.versions = "e".repeat(64);
    expect(failed(auditFairness([summary("http://a"), b]), "hash.versions")).toBeTruthy();
  });

  test("protocol drift (duration, concurrency) fails the audit", () => {
    const short = summary("http://b", { durationSec: 5 });
    expect(failed(auditFairness([summary("http://a"), short]), "protocol.durationSec")).toBeTruthy();
    const sparse = summary("http://c", { concurrencyLevels: [1] });
    expect(failed(auditFairness([summary("http://a"), sparse]), "protocol.concurrency")).toBeTruthy();
  });

  test("environment class drift fails the audit", () => {
    const arm = summary("http://b");
    arm.environment.arch = "arm64";
    expect(failed(auditFairness([summary("http://a"), arm]), "environment.class")).toBeTruthy();
  });

  test("cell grid mismatch fails parity", () => {
    const partial = summary("http://b");
    partial.cells = partial.cells.slice(0, 1);
    expect(failed(auditFairness([summary("http://a"), partial]), "cells.parity")).toBeTruthy();
  });

  test("retained failures fail the audit and name the candidate and cell", () => {
    const broken = summary("http://b");
    broken.cells[1].errors = 4;
    const findings = auditFairness([summary("http://a"), broken]);
    const f = failed(findings, "failures.retained");
    expect(f).toBeTruthy();
    expect(f!.detail).toContain("http://b");
    expect(f!.detail).toContain("W1@10");
  });

  test("report renders pass/fail verdict deterministically", () => {
    const ok = auditFairness([summary("http://a"), summary("http://b")]);
    const md1 = renderFairnessReport(ok, ["http://a", "http://b"]);
    const md2 = renderFairnessReport(auditFairness([summary("http://a"), summary("http://b")]), ["http://a", "http://b"]);
    expect(md1).toBe(md2);
    expect(md1).toContain("**Fairness audit: PASS**");

    const broken = summary("http://b");
    broken.cells[0].statusMismatches = 7;
    const mdFail = renderFairnessReport(auditFairness([summary("http://a"), broken]), ["http://a", "http://b"]);
    expect(mdFail).toContain("Fairness audit: FAIL (1 finding(s))");
  });
});
