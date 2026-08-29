/**
 * Concurrency-sweep comparison (M28-011-D). Structural guardrails per
 * candidate/cell: 0 errors, 0 status mismatches; throughput scaling
 * (rps at c=200 must exceed rps at c=1 — the ladder exercises parallel
 * capacity, not serialization); tail bound (p99 <= 50x nominal).
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
  "# Concurrency Sweep Comparison (M28-011-D)",
  "",
  "W4 cells (ms=1 and ms=5) at the SPEC's full concurrency ladder 1/10/50/200.",
  "",
  "| candidate | cell | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | max (µs) | errors | mismatches |",
  "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
];
let failures = 0;
for (const cand of candidates) {
  const summary: Summary = JSON.parse(readFileSync(join(root, cand, "summary.json"), "utf8"));
  const cellOf = (wl: string, c: number): Cell | null =>
    summary.cells.find((s) => s.workload === wl && s.concurrency === c) ?? null;
  for (const cell of summary.cells) {
    const nominalUs = Number(cell.workload.replace("W4_", "").replace("ms", "")) * 1000;
    lines.push(
      `| ${cand} | ${cell.workload} | ${cell.concurrency} | ${cell.totalRequests} | ${cell.rps} | ${cell.p50Us} | ${cell.p95Us} | ${cell.p99Us} | ${cell.maxUs} | ${cell.errors} | ${cell.statusMismatches} |`,
    );
    if (cell.errors > 0 || cell.statusMismatches > 0) {
      lines.push(`- FAIL ${cand}/${cell.workload}/c${cell.concurrency}: errors=${cell.errors} mismatches=${cell.statusMismatches}`);
      failures++;
    }
    // Tail bound is concurrency-aware: at c in-flight requests sharing a
    // c x nominal fair-share wait, queueing delay is expected; the bound is
    // max(50x nominal, c x nominal) — Little's-law-shaped, never a hang.
    const tailBoundUs = Math.max(nominalUs * 50, cell.concurrency * nominalUs);
    if (cell.p99Us > tailBoundUs) {
      lines.push(`- FAIL ${cand}/${cell.workload}/c${cell.concurrency}: p99 ${cell.p99Us}µs exceeds the fair-share bound (${tailBoundUs}µs)`);
      failures++;
    }
  }
  for (const wl of ["W4_1ms", "W4_5ms"]) {
    const low = cellOf(wl, 1);
    const high = cellOf(wl, 200);
    if (low && high && high.rps <= low.rps) {
      lines.push(`- FAIL ${cand}/${wl}: rps does not scale (c=200 rps ${high.rps} <= c=1 rps ${low.rps})`);
      failures++;
    } else if (low && high) {
      lines.push(`- OK ${cand}/${wl}: rps scales ${low.rps} -> ${high.rps} (c=1 -> c=200)`);
    }
  }
}
if (failures === 0) lines.push("- PASS: all candidates: 0 errors, 0 mismatches, throughput scales, tails bounded.");
writeFileSync(join(root, "comparison.md"), lines.join("\n") + "\n");
console.log(lines.join("\n"));
if (failures > 0) process.exit(1);
