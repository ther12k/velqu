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

interface Thresholds {
  format: string;
  note: string;
  baseline: { date: string; toReadyP50Ms: number; host: string };
  bounds: { toReadyP50Ms: number; toReadyP95Ms: number };
}

const thresholds: Thresholds = JSON.parse(
  readFileSync(join(root, "benchmarks", "gate-thresholds.json"), "utf8"),
);

if (!existsSync(join(root, "target", "release", "velqu-runtime"))) {
  console.error("cold-start-gate: target/release/velqu-runtime not built");
  process.exit(2);
}

// Build the proof pack the same way scripts/verify does — in place
// (artifacts are byte-reproducible per M26-007, so this leaves the tree
// unchanged). Building out-of-tree is rejected by the import policy
// (B-003), so a throwaway project copy is not an option.
const pack = join(root, "examples", "proof", "dist", "app.qpack");
try {
  await $`bun packages/cli/src/index.ts build --project examples/proof`.quiet();
} catch (e) {
  console.error(`cold-start-gate: proof build failed: ${e}`);
  process.exit(2);
}
if (!existsSync(pack)) {
  console.error("cold-start-gate: proof pack missing after build");
  process.exit(2);
}

const bin = join(root, "target", "release", "velqu-runtime");
const toReady: number[] = [];

for (let i = 0; i < SAMPLES; i++) {
  const port = 19400 + ((process.pid + i) % 400);
  const t0 = Bun.nanoseconds();
  const child = Bun.spawn([bin, "--pack", pack, "--port", String(port)], {
    stdout: "ignore",
    stderr: "ignore",
  });
  try {
    const deadline = Date.now() + thresholds.bounds.toReadyP95Ms + 5_000;
    for (;;) {
      try {
        const res = await fetch(`http://127.0.0.1:${port}/greet/ping`);
        await res.arrayBuffer();
        break;
      } catch {
        if (Date.now() > deadline) throw new Error(`sample ${i}: not ready`);
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

const report = {
  gate: "cold-start-regression",
  date: new Date().toISOString(),
  samples: SAMPLES,
  bounds: thresholds.bounds,
  measured: { toReadyP50Ms: p50, toReadyP95Ms: p95, allMs: toReady },
  pass:
    p50 <= thresholds.bounds.toReadyP50Ms &&
    p95 <= thresholds.bounds.toReadyP95Ms,
};
console.log(JSON.stringify(report, null, 2));

if (!report.pass) {
  console.error(
    `cold-start-gate: FAIL — p50 ${p50}ms / p95 ${p95}ms exceeds bounds ` +
      `${thresholds.bounds.toReadyP50Ms}/${thresholds.bounds.toReadyP95Ms}ms. ` +
      `Functional readiness passing no longer implies acceptable startup.`,
  );
  process.exit(1);
}
console.log("cold-start-gate: PASS");
