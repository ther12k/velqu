/**
 * Mixed-outcome comparison report (M28-011-C). Guardrails: every cell has
 * 0 errors and 0 status mismatches (each mode maps to exactly its typed
 * status), and timeout/malformed handling must add bounded, near-flat
 * overhead vs success (p50 <= 2x success p50 + 250ms slack).
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

const MODES = ["MIX_SUCCESS", "MIX_TIMEOUT", "MIX_MALFORMED"];
const lines: string[] = [
  "# Mixed Outcome Comparison (M28-011-C)",
  "",
  "Deterministic upstream outcomes per request: success (200 relay),",
  "timeout (500ms upstream vs 100ms client deadline -> typed 504), malformed",
  "(200 + garbage body -> typed 502). Error handling under load, not error",
  "recovery.",
  "",
  "| candidate | mode | c | requests | rps | p50 (µs) | p95 (µs) | p99 (µs) | errors | mismatches |",
  "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|",
];
let failures = 0;
for (const cand of candidates) {
  const summary: Summary = JSON.parse(readFileSync(join(root, cand, "summary.json"), "utf8"));
  const p50of = (wl: string, c: number): number | null =>
    summary.cells.find((s) => s.workload === wl && s.concurrency === c)?.p50Us ?? null;
  for (const cell of summary.cells) {
    lines.push(
      `| ${cand} | ${cell.workload.replace("MIX_", "")} | ${cell.concurrency} | ${cell.totalRequests} | ${cell.rps} | ${cell.p50Us} | ${cell.p95Us} | ${cell.p99Us} | ${cell.errors} | ${cell.statusMismatches} |`,
    );
    if (cell.errors > 0 || cell.statusMismatches > 0) {
      lines.push(`- FAIL ${cand}/${cell.workload}/c${cell.concurrency}: errors=${cell.errors} mismatches=${cell.statusMismatches} (every mode must map to its exact typed status)`);
      failures++;
    }
  }
  for (const c of [1, 10]) {
    const ok = p50of("MIX_SUCCESS", c);
    const to = p50of("MIX_TIMEOUT", c);
    const bad = p50of("MIX_MALFORMED", c);
    if (ok !== null && to !== null && to > ok * 2 + 250_000) {
      lines.push(`- FAIL ${cand}/c${c}: timeout handling overhead unbounded (p50 ${to}µs vs success ${ok}µs)`);
      failures++;
    }
    if (ok !== null && bad !== null && bad > ok * 2 + 250_000) {
      lines.push(`- FAIL ${cand}/c${c}: malformed handling overhead unbounded (p50 ${bad}µs vs success ${ok}µs)`);
      failures++;
    }
  }
}
if (failures === 0) {
  lines.push("- PASS: all candidates: every mode maps to its exact typed status, 0 errors, 0 mismatches, bounded handling overhead.");
}
writeFileSync(join(root, "comparison.md"), lines.join("\n") + "\n");
console.log(lines.join("\n"));
if (failures > 0) process.exit(1);
