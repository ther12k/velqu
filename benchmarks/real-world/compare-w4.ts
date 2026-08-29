/**
 * W4 comparison report generator (M28-011-A).
 *
 * Reads each candidate's summary.json under the run root and emits a
 * combined per-latency-cell comparison (p50/p95/p99 + error/mismatch
 * counts). Pure aggregation of already-validated summaries — no values
 * are invented here.
 */

import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

interface Cell {
  workload: string;
  concurrency: number;
  totalRequests: number;
  errors: number;
  statusMismatches: number;
  rps: number;
  p50Us: number;
  p95Us: number;
  p99Us: number;
  maxUs: number;
}
interface Summary {
  format: string;
  candidate: string;
  cells: Cell[];
}

const args = process.argv.slice(2);
const get = (name: string): string => {
  const i = args.indexOf(`--${name}`);
  if (i < 0) throw new Error(`--${name} is required`);
  return args[i + 1];
};
const root = get("root");

const candidates = readdirSync(root).filter((d) => {
  try {
    readFileSync(join(root, d, "summary.json"), "utf8");
    return true;
  } catch {
    return false;
  }
});
if (candidates.length === 0) throw new Error("no candidate summaries found");

const WORKLOADS = ["W4_1ms", "W4_5ms", "W4_10ms", "W4_25ms"];
const CONCURRENCIES = [1, 10];

const lines: string[] = [
  "# W4 Controlled-Upstream Latency — Candidate Comparison (M28-011-A)",
  "",
  "Every candidate implements the identical proxy contract: `GET /api/bench/io?ms=N`",
  "relayed through the runtime's native fetch to the controlled upstream",
  "(`GET /io?ms=N`). Same machine, same upstream, same load generator.",
  "",
  "| candidate | cell | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | mismatches |",
  "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
];

for (const cand of candidates) {
  const summary: Summary = JSON.parse(readFileSync(join(root, cand, "summary.json"), "utf8"));
  for (const wl of WORKLOADS) {
    for (const c of CONCURRENCIES) {
      const cell = summary.cells.find((s) => s.workload === wl && s.concurrency === c);
      if (!cell) continue;
      lines.push(
        `| ${cand} | ${wl} | ${c} | ${cell.totalRequests} | ${cell.rps} | ${cell.p50Us} | ${cell.p95Us} | ${cell.p99Us} | ${cell.maxUs} | ${cell.errors} | ${cell.statusMismatches} |`,
      );
    }
  }
}

lines.push(
  "",
  "Tail-latency guardrail: for every candidate and cell, p99 must remain within",
  "50x the nominal upstream latency (a structural sanity bound, not a perf claim).",
  "",
);

let failures = 0;
for (const cand of candidates) {
  const summary: Summary = JSON.parse(readFileSync(join(root, cand, "summary.json"), "utf8"));
  for (const cell of summary.cells) {
    const nominalUs = Number(cell.workload.replace("W4_", "").replace("ms", "")) * 1000;
    if (cell.errors > 0 || cell.statusMismatches > 0) {
      lines.push(`- FAIL ${cand}/${cell.workload}/c${cell.concurrency}: errors=${cell.errors} mismatches=${cell.statusMismatches}`);
      failures++;
    } else if (cell.p99Us > nominalUs * 50) {
      lines.push(`- FAIL ${cand}/${cell.workload}/c${cell.concurrency}: p99 ${cell.p99Us}µs exceeds 50x nominal (${nominalUs}µs)`);
      failures++;
    }
  }
}
if (failures === 0) lines.push("- PASS: all candidates, all cells: 0 errors, 0 mismatches, p99 within 50x nominal.");

writeFileSync(join(root, "comparison.md"), lines.join("\n") + "\n");
console.log(lines.join("\n"));
if (failures > 0) process.exit(1);
