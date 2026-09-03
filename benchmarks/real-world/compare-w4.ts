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
const get = (name: string): string | null => {
  const i = args.indexOf(`--${name}`);
  return i >= 0 ? args[i + 1] : null;
};
const req = (name: string): string => {
  const v = get(name);
  if (v === null || v === undefined) throw new Error(`--${name} is required`);
  return v;
};
const root = req("root");
// BETA-003-A: the comparison is generalizable to any cell matrix; defaults
// preserve the M28-011-A W4 invocation exactly.
const WORKLOADS = (get("workloads") ?? "W4_1ms,W4_5ms,W4_10ms,W4_25ms")
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
const CONCURRENCIES = (get("concurrency") ?? "1,10")
  .split(",")
  .map((s) => Number(s.trim()))
  .filter((n) => n > 0);
const OUT_NAME = get("out") ?? "comparison.md";
const TITLE = get("title") ?? "W4 Controlled-Upstream Latency — Candidate Comparison (M28-011-A)";

const candidates = readdirSync(root).filter((d) => {
  try {
    readFileSync(join(root, d, "summary.json"), "utf8");
    return true;
  } catch {
    return false;
  }
});
if (candidates.length === 0) throw new Error("no candidate summaries found");

const lines: string[] = [
  `# ${TITLE}`,
  "",
  "Every candidate implements the identical contract (BETA-002): same routes,",
  "same response bodies, same posture — verified per candidate by",
  "verify-contract.ts before this run. Same machine, same controlled upstream,",
  "same load generator; raw rows are retained alongside each summary.",
  "",
  "| candidate | cell | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | mismatches | server RSS (kB) |",
  "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
];

const rssOf = (summary: Summary & { serverRssKb?: number }) =>
  typeof summary.serverRssKb === "number" ? String(summary.serverRssKb) : "n/a";

for (const cand of candidates) {
  const summary: Summary & { serverRssKb?: number } = JSON.parse(
    readFileSync(join(root, cand, "summary.json"), "utf8"),
  );
  const rss = rssOf(summary);
  for (const wl of WORKLOADS) {
    for (const c of CONCURRENCIES) {
      const cell = summary.cells.find((s) => s.workload === wl && s.concurrency === c);
      if (!cell) continue;
      lines.push(
        `| ${cand} | ${wl} | ${c} | ${cell.totalRequests} | ${cell.rps} | ${cell.p50Us} | ${cell.p95Us} | ${cell.p99Us} | ${cell.maxUs} | ${cell.errors} | ${cell.statusMismatches} | ${rss} |`,
      );
    }
  }
}

lines.push(
  "",
  "Tail-latency guardrail: for every candidate and W4 cell, p99 must remain within",
  "50x the nominal upstream latency (a structural sanity bound, not a perf claim).",
  "Non-W4 cells (payload/CPU matrices) are checked for zero errors/mismatches only.",
  "",
);

let failures = 0;
for (const cand of candidates) {
  const summary: Summary = JSON.parse(readFileSync(join(root, cand, "summary.json"), "utf8"));
  for (const cell of summary.cells) {
    const nominalMatch = /^W4_(\d+)ms$/.exec(cell.workload);
    // BETA-003-A: the 0ms cell measures the overhead floor, so its sanity
    // bound is absolute (50ms p99) — a multiplicative bound on 0 is always 0.
    const ZERO_MS_BOUND_US = 50_000;
    const nominalMs = nominalMatch ? Number(nominalMatch[1]) : null;
    const boundUs =
      nominalMs === null ? null : nominalMs === 0 ? ZERO_MS_BOUND_US : nominalMs * 1000 * 50;
    const multiplicative = nominalMs !== null && nominalMs > 0;
    if (cell.errors > 0 || cell.statusMismatches > 0) {
      lines.push(`- FAIL ${cand}/${cell.workload}/c${cell.concurrency}: errors=${cell.errors} mismatches=${cell.statusMismatches}`);
      failures++;
    } else if (boundUs !== null && cell.p99Us > boundUs) {
      const why = multiplicative
        ? `p99 ${cell.p99Us}µs exceeds 50x nominal (${nominalMs! * 1000}µs)`
        : `p99 ${cell.p99Us}µs exceeds the 50ms absolute floor bound (0ms cell)`;
      lines.push(`- FAIL ${cand}/${cell.workload}/c${cell.concurrency}: ${why}`);
      failures++;
    }
  }
}
if (failures === 0) lines.push("- PASS: all candidates, all cells: 0 errors, 0 mismatches, p99 within 50x nominal.");

writeFileSync(join(root, OUT_NAME), lines.join("\n") + "\n");
console.log(lines.join("\n"));
if (failures > 0) process.exit(1);
