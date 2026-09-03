/**
 * Real-world benchmark fairness audit (BETA-001-C).
 *
 * A run set is fair only when every candidate faced the identical contract:
 * same workload definitions, same dataset (schema + seed), same version pins,
 * same protocol shape (duration, concurrency levels, environment class), and
 * no candidate hides failures (errors/status mismatches are counted, and any
 * nonzero count is a fairness finding, per SPEC's 0%-error requirement).
 *
 * Usage:
 *   bun fairness.ts --runs dirA,dirB [--out fairness.md]
 * Each dir must contain a summary.json produced by load.ts.
 */

import { readFileSync, writeFileSync } from "node:fs";
import type { RealWorldSummary } from "./result-schema";

const DIR = import.meta.dir;

export interface FairnessFinding {
  check: string;
  status: "PASS" | "FAIL";
  detail: string;
}

const COMPARED_HASHES = ["spec", "workloads", "schema", "seed", "versions", "differences"] as const;

export function auditFairness(summaries: RealWorldSummary[]): FairnessFinding[] {
  const findings: FairnessFinding[] = [];
  const label = (s: RealWorldSummary) => s.baseUrl;
  const names = summaries.map(label);

  if (summaries.length < 2) {
    findings.push({
      check: "run-set",
      status: "FAIL",
      detail: "fairness audit needs at least two candidate summaries",
    });
    return findings;
  }

  // 1. Identical contract hashes across candidates.
  for (const key of COMPARED_HASHES) {
    const values = summaries.map((s) => s.configHashes?.[key]);
    const allPresent = values.every((v) => typeof v === "string" && v.length > 0);
    if (!allPresent) {
      findings.push({ check: `hash.${key}`, status: "FAIL", detail: "missing hash in at least one summary" });
    } else if (new Set(values).size !== 1) {
      findings.push({
        check: `hash.${key}`,
        status: "FAIL",
        detail: `${key} differs across candidates: ${values.join(" vs ")}`,
      });
    } else {
      findings.push({ check: `hash.${key}`, status: "PASS", detail: values[0] });
    }
  }

  // 2. Identical protocol shape.
  const durations = summaries.map((s) => s.durationSec);
  if (new Set(durations).size !== 1) {
    findings.push({ check: "protocol.durationSec", status: "FAIL", detail: durations.join(" vs ") });
  } else {
    findings.push({ check: "protocol.durationSec", status: "PASS", detail: `${durations[0]}s cells` });
  }
  const conc = summaries.map((s) => JSON.stringify(s.concurrencyLevels));
  if (new Set(conc).size !== 1) {
    findings.push({ check: "protocol.concurrency", status: "FAIL", detail: conc.join(" vs ") });
  } else {
    findings.push({ check: "protocol.concurrency", status: "PASS", detail: summaries[0].concurrencyLevels.join("/") });
  }

  // 3. Same host class (os/arch); runtimes intentionally may differ.
  const oses = summaries.map((s) => `${s.environment.os}/${s.environment.arch}`);
  if (new Set(oses).size !== 1) {
    findings.push({ check: "environment.class", status: "FAIL", detail: oses.join(" vs ") });
  } else {
    findings.push({ check: "environment.class", status: "PASS", detail: oses[0] });
  }

  // 4. Cell completeness parity: every candidate covers the same workload x
  //    concurrency grid.
  const grid = (s: RealWorldSummary) =>
    s.cells.map((c) => `${c.workload}@${c.concurrency}`).sort().join(",");
  if (new Set(summaries.map(grid)).size !== 1) {
    findings.push({ check: "cells.parity", status: "FAIL", detail: "workload/concurrency grids differ across candidates" });
  } else {
    findings.push({ check: "cells.parity", status: "PASS", detail: `${summaries[0].cells.length} cells per candidate` });
  }

  // 5. No hidden failures: errors and status mismatches must be zero for the
  //    comparison to be called fair (SPEC: 0% error rate required).
  for (const s of summaries) {
    const bad = s.cells.filter((c) => c.errors > 0 || c.statusMismatches > 0);
    if (bad.length > 0) {
      findings.push({
        check: "failures.retained",
        status: "FAIL",
        detail: `${label(s)}: ${bad.length} cell(s) with errors/mismatches (e.g. ${bad[0].workload}@${bad[0].concurrency}: ${bad[0].errors}err/${bad[0].statusMismatches}mismatch) — retained in raw rows, must be resolved before claiming a fair comparison`,
      });
    }
  }
  if (!findings.some((f) => f.check === "failures.retained" && f.status === "FAIL")) {
    findings.push({ check: "failures.retained", status: "PASS", detail: "zero errors/status mismatches in all cells" });
  }

  return findings;
}

export function renderFairnessReport(findings: FairnessFinding[], runLabels: string[]): string {
  const lines: string[] = [];
  lines.push("# Real-World Benchmark Fairness Audit", "");
  lines.push(`Candidates: ${runLabels.join(" vs ")}`, "");
  lines.push("| Check | Status | Detail |");
  lines.push("|---|---|---|");
  for (const f of findings) {
    lines.push(`| ${f.check} | ${f.status} | ${f.detail} |`);
  }
  lines.push("");
  const failed = findings.filter((f) => f.status === "FAIL").length;
  lines.push(failed === 0 ? "**Fairness audit: PASS**" : `**Fairness audit: FAIL (${failed} finding(s))**`, "");
  return lines.join("\n");
}

function main() {
  const argv = process.argv.slice(2);
  const get = (name: string): string | null => {
    const i = argv.indexOf(`--${name}`);
    return i >= 0 ? argv[i + 1] : null;
  };
  const runs = (get("runs") ?? "").split(",").map((s) => s.trim()).filter(Boolean);
  if (runs.length < 2) throw new Error("--runs needs at least two run directories");
  const summaries = runs.map((r) => JSON.parse(readFileSync(`${r}/summary.json`, "utf8")) as RealWorldSummary);
  const findings = auditFairness(summaries);
  const report = renderFairnessReport(findings, summaries.map((s) => s.baseUrl));
  const out = get("out") ?? `${DIR}/fairness.md`;
  writeFileSync(out, report);
  console.log(report);
  process.exit(findings.some((f) => f.status === "FAIL") ? 1 : 0);
}

if (import.meta.main) {
  main();
}
