/**
 * Fan-out comparison report generator (M28-011-B). Aggregates per-candidate
 * summaries; structural guardrails: 0 errors, 0 status mismatches, and
 * fan-out wall time must not scale linearly with n (p50 of n=4 must be
 * strictly less than 4x the p50 of n=1 — parallelism proof).
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

const lines: string[] = [
  "# Fan-out Comparison (M28-011-B)",
  "",
  "Each request issues n PARALLEL upstream calls (ms=5) via Promise.all-style",
  "fan-out and aggregates. Parallelism proof: p50(n=4) < 4 x p50(n=1).",
  "",
  "| candidate | n | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | errors | mismatches |",
  "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
];
let failures = 0;
for (const cand of candidates) {
  const summary: Summary = JSON.parse(readFileSync(join(root, cand, "summary.json"), "utf8"));
  const p50 = (n: number, c: number): number | null =>
    summary.cells.find((s) => s.workload === `FANOUT_${n}` && s.concurrency === c)?.p50Us ?? null;
  for (const cell of summary.cells) {
    const n = Number(cell.workload.replace("FANOUT_", ""));
    lines.push(
      `| ${cand} | ${n} | ${cell.concurrency} | ${cell.totalRequests} | ${cell.rps} | ${cell.p50Us} | ${cell.p95Us} | ${cell.p99Us} | ${cell.errors} | ${cell.statusMismatches} |`,
    );
    if (cell.errors > 0 || cell.statusMismatches > 0) {
      lines.push(`- FAIL ${cand}/n=${n}/c${cell.concurrency}: errors=${cell.errors} mismatches=${cell.statusMismatches}`);
      failures++;
    }
  }
  for (const c of [1, 10]) {
    const p1 = p50(1, c);
    const p4 = p50(4, c);
    if (p1 !== null && p4 !== null) {
      if (p4 >= p1 * 4) {
        lines.push(`- FAIL ${cand}/c${c}: p50(n=4)=${p4}µs >= 4x p50(n=1)=${p1 * 4}µs — fan-out is not parallel`);
        failures++;
      } else {
        lines.push(`- OK ${cand}/c${c}: p50(n=4)=${p4}µs < 4x p50(n=1)=${p1 * 4}µs (parallelism proven)`);
      }
    }
  }
}
if (failures === 0) lines.push("- PASS: all candidates: 0 errors, 0 mismatches, fan-out parallelism proven.");
writeFileSync(join(root, "comparison.md"), lines.join("\n") + "\n");
console.log(lines.join("\n"));
if (failures > 0) process.exit(1);
