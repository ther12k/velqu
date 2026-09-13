/**
 * #1319 / M6-009 — cold-start regression gate (functional vs performance
 * separation mandated by the owner, 2026-09-11).
 *
 * The differential suite's 45 s readiness wait is a FUNCTIONAL guard: a
 * server that takes 25 s to boot still passes it. This gate is the
 * PERFORMANCE counterpart: it measures boot-to-ready p50/p95 against
 * `benchmarks/gate-thresholds.json` and fails if startup regresses
 * beyond generous, committed bounds.
 *
 * The bounds are deliberately loose (~80x the measured baseline) so CI
 * runner noise cannot flake the gate; a bound this loose still catches
 * the multi-second/minute startup collapses that functional tests miss.
 * Tightening belongs to the owner benchmark process (constraint 12),
 * not to this gate.
 *
 * Usage: bun scripts/cold-start-gate.ts [--samples N]
 */
import { $ } from "bun";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "..");
const samplesFlag = process.argv.indexOf("--samples");
const SAMPLES =
  samplesFlag !== -1 ? Number(process.argv[samplesFlag + 1]) || 20 : 20;

interface ThresholdsDoc {
  format: string;
  note: string;
  coldStart?: {
    note: string;
    baseline: { date: string; toReadyP50Ms: number; host: string };
    bounds: { toReadyP50Ms: number; toReadyP95Ms: number };
  };
  baseline?: { date: string; toReadyP50Ms: number; host: string };
  bounds?: { toReadyP50Ms: number; toReadyP95Ms: number };
}

const thresholdsDoc: ThresholdsDoc = JSON.parse(
  readFileSync(join(root, "benchmarks", "gate-thresholds.json"), "utf8"),
);

const bounds = thresholdsDoc.coldStart?.bounds ?? thresholdsDoc.bounds;
const baseline = thresholdsDoc.coldStart?.baseline ?? thresholdsDoc.baseline;

if (!bounds || typeof bounds.toReadyP50Ms !== "number" || typeof bounds.toReadyP95Ms !== "number") {
  console.error("cold-start-gate: invalid or missing bounds in benchmarks/gate-thresholds.json");
  process.exit(2);
}

const bin = join(root, "target", "release", "velqu-runtime");
if (!existsSync(bin)) {
  console.error(`cold-start-gate: target/release/velqu-runtime not built at ${bin}`);
  process.exit(2);
}

// Build the proof pack the same way scripts/verify does — in place
// (artifacts are byte-reproducible per M26-007, so this leaves the tree
// unchanged). Building out-of-tree is rejected by the import policy
// (B-003), so a throwaway project copy is not an option.
const pack = join(root, "examples", "proof", "dist", "app.qpack");
if (!existsSync(pack)) {
  try {
    await $`bun packages/cli/src/index.ts build --project examples/proof`.quiet();
  } catch (e) {
    console.error(`cold-start-gate: proof build failed: ${e}`);
    process.exit(2);
  }
}
if (!existsSync(pack)) {
  console.error("cold-start-gate: proof pack missing after build");
  process.exit(2);
}

const toReady: number[] = [];

for (let i = 0; i < SAMPLES; i++) {
  const port = 19400 + ((process.pid + i) % 400);
  const t0 = Bun.nanoseconds();
  const child = Bun.spawn([bin, "--pack", pack, "--port", String(port)], {
    stdout: "ignore",
    stderr: "ignore",
  });
  try {
    const deadline = Date.now() + bounds.toReadyP95Ms + 5_000;
    for (;;) {
      try {
        const res = await fetch(`http://127.0.0.1:${port}/health/live`);
        await res.arrayBuffer();
        break;
      } catch {
        if (Date.now() > deadline) throw new Error(`sample ${i}: not ready within deadline`);
        await Bun.sleep(5);
      }
    }
    toReady.push(Number((Bun.nanoseconds() - t0) / 1e6));
  } finally {
    child.kill();
    await child.exited;
  }
}

const sorted = [...toReady].sort((a, b) => a - b);
const pct = (p: number) =>
  sorted[Math.min(sorted.length - 1, Math.floor(p * sorted.length))]!;
const p50 = pct(0.5);
const p95 = pct(0.95);

const p50Pass = p50 <= bounds.toReadyP50Ms;
const p95Pass = p95 <= bounds.toReadyP95Ms;
const pass = p50Pass && p95Pass;

const report = {
  gate: "cold-start-regression",
  date: new Date().toISOString(),
  candidate: "velqu-runtime",
  samples: SAMPLES,
  baseline: baseline ?? null,
  bounds,
  measured: { toReadyP50Ms: p50, toReadyP95Ms: p95, allMs: toReady },
  pass,
};
console.log(JSON.stringify(report, null, 2));

console.log("\n--- Cold-Start Threshold Evaluations ---");
console.log("Metric | Measured | Threshold | Baseline | Unit | Direction | Status");
console.log("---|---:|---:|---:|---|---|---");
console.log(`boot_to_ready_p50 | ${p50.toFixed(2)} | ${bounds.toReadyP50Ms} | ${baseline?.toReadyP50Ms ?? "N/A"} | ms | <= max | ${p50Pass ? "PASS" : "FAIL"}`);
console.log(`boot_to_ready_p95 | ${p95.toFixed(2)} | ${bounds.toReadyP95Ms} | N/A | ms | <= max | ${p95Pass ? "PASS" : "FAIL"}`);

if (!pass) {
  console.error(`\n✗ cold-start-gate: FAIL — boot-to-ready regression threshold violation:`);
  if (!p50Pass) {
    console.error(`  - p50 ${p50.toFixed(2)} ms exceeds maximum threshold ${bounds.toReadyP50Ms} ms (baseline: ${baseline?.toReadyP50Ms ?? "N/A"} ms)`);
  }
  if (!p95Pass) {
    console.error(`  - p95 ${p95.toFixed(2)} ms exceeds maximum threshold ${bounds.toReadyP95Ms} ms`);
  }
  console.error(`Functional readiness passing no longer implies acceptable startup.`);
  process.exit(1);
}
console.log("\n✓ cold-start-gate: PASS — cold-start performance within approved budgets.");
