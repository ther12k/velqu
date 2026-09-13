/**
 * #1319 / M6-009 — merge-blocking throughput & latency regression gate.
 *
 * Enforces that runtime throughput and latency under warm load do not
 * regress beyond committed, agreed bounds (benchmarks/gate-thresholds.json).
 *
 * Evaluated across canonical benchmark routes against the built proof pack:
 *   - C0 (/health/live): static native liveness (Rust-only, bypasses JS)
 *   - C1 (/js-text): JavaScript-computed plaintext string
 *   - C2 (/js-json): JavaScript-computed small JSON object
 *   - C3 (/hello/Rafi): Parameterized route with schema validation
 *
 * Comparison direction and unit contracts (owner review):
 *   - Throughput (requests/sec, rps): measured >= threshold (higher is better).
 *   - Latency (microseconds, µs): measured <= threshold (lower is better) for
 *     p50, p95, and p99 percentiles.
 *   - Errors: 0 allowed. Missing, empty, or invalid measurements FAIL closed.
 *   - Strict bounds validation: every threshold bound must be a finite positive number.
 *   - Route filter validation: unknown route IDs in --routes fail closed.
 *
 * Usage: bun scripts/throughput-latency-gate.ts [--thresholds <path>] [--duration-secs N] [--concurrency N] [--routes C0,C1...]
 */
import { $ } from "bun";
import { readFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";

const root = join(import.meta.dir, "..");

function argValue(name: string): string | undefined {
  const i = process.argv.indexOf(name);
  return i !== -1 ? process.argv[i + 1] : undefined;
}

const customThresholdsPath = argValue("--thresholds");
const thresholdsPath = customThresholdsPath ? resolve(customThresholdsPath) : join(root, "benchmarks", "gate-thresholds.json");

if (!existsSync(thresholdsPath)) {
  console.error(`throughput-latency-gate: thresholds file missing at ${thresholdsPath}`);
  process.exit(2);
}

interface RouteThresholds {
  path: string;
  description: string;
  baseline: {
    rps: number;
    p50Us: number;
    p95Us: number;
    p99Us: number;
  };
  bounds: {
    minRps: number;
    maxP50Us: number;
    maxP95Us: number;
    maxP99Us: number;
  };
}

interface ThresholdsDoc {
  format: string;
  note: string;
  throughputLatency?: {
    note: string;
    baseline: {
      date: string;
      host: string;
      concurrency: number;
      source: string;
    };
    concurrency: number;
    durationSecs: number;
    routes: Record<string, RouteThresholds>;
  };
}

let doc: ThresholdsDoc;
try {
  doc = JSON.parse(readFileSync(thresholdsPath, "utf8"));
} catch (e) {
  console.error(`throughput-latency-gate: failed to parse thresholds JSON at ${thresholdsPath}: ${e}`);
  process.exit(2);
}

const config = doc.throughputLatency;
if (!config || !config.routes || typeof config.routes !== "object" || Object.keys(config.routes).length === 0) {
  console.error("throughput-latency-gate: no valid throughputLatency.routes configured in thresholds file");
  process.exit(2);
}

// Duration & Concurrency: use JSON configuration as default with optional CLI override
const cliDuration = argValue("--duration-secs");
const cliConcurrency = argValue("--concurrency");
const DURATION_SECS = cliDuration !== undefined ? Number(cliDuration) : config.durationSecs ?? 3;
const CONCURRENCY = cliConcurrency !== undefined ? Number(cliConcurrency) : config.concurrency ?? 10;

if (!Number.isFinite(DURATION_SECS) || DURATION_SECS <= 0) {
  console.error(`throughput-latency-gate: invalid durationSecs: ${DURATION_SECS}. Must be a positive number.`);
  process.exit(2);
}
if (!Number.isInteger(CONCURRENCY) || CONCURRENCY <= 0) {
  console.error(`throughput-latency-gate: invalid concurrency: ${CONCURRENCY}. Must be a positive integer.`);
  process.exit(2);
}

// Strict validation of route bounds: every bound must be finite and > 0
for (const [routeId, spec] of Object.entries(config.routes)) {
  if (!spec || typeof spec !== "object") {
    console.error(`throughput-latency-gate: route ${routeId} specification missing or not an object`);
    process.exit(2);
  }
  if (!spec.path || typeof spec.path !== "string") {
    console.error(`throughput-latency-gate: route ${routeId} missing path string`);
    process.exit(2);
  }
  const b = spec.bounds;
  if (!b || typeof b !== "object") {
    console.error(`throughput-latency-gate: route ${routeId} missing bounds object`);
    process.exit(2);
  }
  const requiredNumericBounds: Array<keyof typeof b> = ["minRps", "maxP50Us", "maxP95Us", "maxP99Us"];
  for (const key of requiredNumericBounds) {
    const val = b[key];
    if (typeof val !== "number" || !Number.isFinite(val) || val <= 0) {
      console.error(`throughput-latency-gate: route ${routeId} has invalid or non-positive bound '${key}': ${val}`);
      process.exit(2);
    }
  }
}

// Route selection: reject unknown routes fail-closed
const knownRouteIds = Object.keys(config.routes);
const cliRoutesRaw = argValue("--routes");
const ROUTES_FILTER = cliRoutesRaw ? cliRoutesRaw.split(",").map((s) => s.trim()).filter(Boolean) : undefined;

let targetRouteIds: string[];
if (ROUTES_FILTER) {
  const unknownRoutes = ROUTES_FILTER.filter((r) => !knownRouteIds.includes(r));
  if (unknownRoutes.length > 0) {
    console.error(
      `throughput-latency-gate: unknown route(s) in --routes: ${unknownRoutes.join(", ")}. Known routes: ${knownRouteIds.join(", ")}`,
    );
    process.exit(2);
  }
  targetRouteIds = ROUTES_FILTER;
} else {
  targetRouteIds = knownRouteIds;
}

if (targetRouteIds.length === 0) {
  console.error("throughput-latency-gate: no target routes selected for benchmark");
  process.exit(2);
}

const bin = join(root, "target", "release", "velqu-runtime");
if (!existsSync(bin)) {
  console.error(`throughput-latency-gate: runtime binary not found at ${bin}. Build with: cargo build --release -p velqu-runtime`);
  process.exit(2);
}

const pack = join(root, "examples", "proof", "dist", "app.qpack");
if (!existsSync(pack)) {
  console.log("throughput-latency-gate: building proof pack...");
  try {
    await $`bun packages/cli/src/index.ts build --project examples/proof`.quiet();
  } catch (e) {
    console.error(`throughput-latency-gate: proof build failed: ${e}`);
    process.exit(2);
  }
}
if (!existsSync(pack)) {
  console.error("throughput-latency-gate: proof pack missing after build");
  process.exit(2);
}

function freePort(): number {
  const l = Bun.listen({ hostname: "127.0.0.1", port: 0, socket: { data() {}, open() {} } });
  const port = l.port;
  l.stop(true);
  return port;
}

interface Measurement {
  routeId: string;
  path: string;
  concurrency: number;
  durationSecs: number;
  totalCompleted: number;
  rps: number;
  p50Us: number;
  p95Us: number;
  p99Us: number;
  errors: number;
}

interface EvaluationResult {
  routeId: string;
  path: string;
  metric: string;
  measured: number;
  threshold: number;
  baseline: number;
  unit: string;
  comparison: ">=" | "<=" | "===";
  passed: boolean;
  reason?: string;
}

async function runBenchmark(): Promise<{ measurements: Measurement[]; evaluations: EvaluationResult[] }> {
  const port = freePort();
  const child = Bun.spawn([bin, "--pack", pack, "--port", String(port), "--log", "off"], {
    stdout: "ignore",
    stderr: "ignore",
  });

  try {
    // Poll readiness
    const deadline = performance.now() + 10_000;
    let ready = false;
    while (performance.now() < deadline) {
      try {
        const res = await fetch(`http://127.0.0.1:${port}/health/live`);
        if (res.status === 200) {
          ready = true;
          break;
        }
      } catch {
        await Bun.sleep(10);
      }
    }
    if (!ready) {
      throw new Error(`velqu-runtime failed to reach readiness within 10s on port ${port}`);
    }

    const measurements: Measurement[] = [];
    const evaluations: EvaluationResult[] = [];

    for (const routeId of targetRouteIds) {
      const spec = config!.routes[routeId]!;
      const url = `http://127.0.0.1:${port}${spec.path}`;

      // Warmup phase (50 requests)
      for (let i = 0; i < 50; i++) {
        try {
          const w = await fetch(url);
          await w.arrayBuffer();
        } catch {}
      }

      // Fixed-duration measurement with concurrent workers
      const latenciesUs: number[] = [];
      let completed = 0;
      let errors = 0;
      const measurementDeadline = performance.now() + DURATION_SECS * 1000;

      const t0 = performance.now();
      const workers = Array.from({ length: CONCURRENCY }, async () => {
        while (performance.now() < measurementDeadline) {
          const reqStart = performance.now();
          try {
            const res = await fetch(url);
            if (res.status === 200) {
              await res.arrayBuffer();
              latenciesUs.push((performance.now() - reqStart) * 1000);
              completed++;
            } else {
              errors++;
            }
          } catch {
            errors++;
          }
        }
      });

      await Promise.all(workers);
      const totalElapsedSec = (performance.now() - t0) / 1000;
      const rps = Math.round(completed / totalElapsedSec);

      latenciesUs.sort((a, b) => a - b);
      const pct = (q: number) => {
        if (latenciesUs.length === 0) return NaN;
        const idx = Math.min(latenciesUs.length - 1, Math.round(q * (latenciesUs.length - 1)));
        return Math.round((latenciesUs[idx] ?? 0) * 10) / 10;
      };

      const m: Measurement = {
        routeId,
        path: spec.path,
        concurrency: CONCURRENCY,
        durationSecs: DURATION_SECS,
        totalCompleted: completed,
        rps,
        p50Us: pct(0.5),
        p95Us: pct(0.95),
        p99Us: pct(0.99),
        errors,
      };
      measurements.push(m);

      // Evaluate error count
      evaluations.push({
        routeId,
        path: spec.path,
        metric: "errors",
        measured: m.errors,
        threshold: 0,
        baseline: 0,
        unit: "count",
        comparison: "===",
        passed: m.errors === 0,
        reason: m.errors > 0 ? `${m.errors} request error(s) observed during measurement` : undefined,
      });

      // Fail-closed: missing / invalid samples
      if (completed === 0 || !Number.isFinite(m.rps) || m.rps <= 0) {
        evaluations.push({
          routeId,
          path: spec.path,
          metric: "samples_validity",
          measured: completed,
          threshold: 1,
          baseline: spec.baseline.rps,
          unit: "count",
          comparison: ">=",
          passed: false,
          reason: `zero requests completed successfully for route ${routeId}`,
        });
      }

      // Evaluate throughput: measured.rps >= bounds.minRps (higher is better)
      const rpsPassed = m.rps >= spec.bounds.minRps;
      evaluations.push({
        routeId,
        path: spec.path,
        metric: "throughput",
        measured: m.rps,
        threshold: spec.bounds.minRps,
        baseline: spec.baseline.rps,
        unit: "req/s",
        comparison: ">=",
        passed: rpsPassed,
        reason: !rpsPassed
          ? `throughput ${m.rps} req/s below minimum threshold ${spec.bounds.minRps} req/s (baseline: ${spec.baseline.rps} req/s)`
          : undefined,
      });

      // Evaluate p50 latency: measured.p50Us <= bounds.maxP50Us (lower is better)
      const p50Passed = Number.isFinite(m.p50Us) && m.p50Us <= spec.bounds.maxP50Us;
      evaluations.push({
        routeId,
        path: spec.path,
        metric: "latency_p50",
        measured: m.p50Us,
        threshold: spec.bounds.maxP50Us,
        baseline: spec.baseline.p50Us,
        unit: "µs",
        comparison: "<=",
        passed: p50Passed,
        reason: !p50Passed
          ? `p50 latency ${m.p50Us} µs exceeds maximum threshold ${spec.bounds.maxP50Us} µs (baseline: ${spec.baseline.p50Us} µs)`
          : undefined,
      });

      // Evaluate p95 latency: measured.p95Us <= bounds.maxP95Us (lower is better)
      const p95Passed = Number.isFinite(m.p95Us) && m.p95Us <= spec.bounds.maxP95Us;
      evaluations.push({
        routeId,
        path: spec.path,
        metric: "latency_p95",
        measured: m.p95Us,
        threshold: spec.bounds.maxP95Us,
        baseline: spec.baseline.p95Us,
        unit: "µs",
        comparison: "<=",
        passed: p95Passed,
        reason: !p95Passed
          ? `p95 latency ${m.p95Us} µs exceeds maximum threshold ${spec.bounds.maxP95Us} µs (baseline: ${spec.baseline.p95Us} µs)`
          : undefined,
      });

      // Evaluate p99 latency: measured.p99Us <= bounds.maxP99Us (lower is better)
      const p99Passed = Number.isFinite(m.p99Us) && m.p99Us <= spec.bounds.maxP99Us;
      evaluations.push({
        routeId,
        path: spec.path,
        metric: "latency_p99",
        measured: m.p99Us,
        threshold: spec.bounds.maxP99Us,
        baseline: spec.baseline.p99Us,
        unit: "µs",
        comparison: "<=",
        passed: p99Passed,
        reason: !p99Passed
          ? `p99 latency ${m.p99Us} µs exceeds maximum threshold ${spec.bounds.maxP99Us} µs (baseline: ${spec.baseline.p99Us} µs)`
          : undefined,
      });
    }

    return { measurements, evaluations };
  } finally {
    child.kill();
    await child.exited;
  }
}

async function main() {
  console.log("=== Performance Gate: Throughput & Latency Regression Check (#1319 / M6-009) ===");
  console.log(`Duration: ${DURATION_SECS}s per route | Concurrency: ${CONCURRENCY} | Target routes: ${targetRouteIds.join(", ")}`);
  console.log(`Thresholds File: ${thresholdsPath}`);
  console.log(`Baseline Source: ${config!.baseline.source} (${config!.baseline.host})\n`);

  let res: { measurements: Measurement[]; evaluations: EvaluationResult[] };
  try {
    res = await runBenchmark();
  } catch (err) {
    console.error(`throughput-latency-gate: execution error: ${err}`);
    process.exit(2);
  }

  const { measurements, evaluations } = res;

  // Print structured measurement summary
  console.log("--- Measured Performance ---");
  console.log("Route | Path | Completed | Throughput | Latency p50 | Latency p95 | Latency p99 | Errors");
  console.log("---|---|---:|---:|---:|---:|---:|---:");
  for (const m of measurements) {
    console.log(
      `${m.routeId} | ${m.path} | ${m.totalCompleted} reqs | ${m.rps} rps | ${m.p50Us.toFixed(1)} µs | ${m.p95Us.toFixed(1)} µs | ${m.p99Us.toFixed(1)} µs | ${m.errors}`,
    );
  }
  console.log("");

  // Print evaluations table
  console.log("--- Threshold Evaluations ---");
  console.log("Route | Metric | Measured | Threshold | Baseline | Unit | Direction | Status");
  console.log("---|---|---:|---:|---:|---|---|---");
  for (const ev of evaluations) {
    const status = ev.passed ? "PASS" : "FAIL";
    const dir = ev.comparison === ">=" ? ">= min" : ev.comparison === "<=" ? "<= max" : "==";
    console.log(
      `${ev.routeId} | ${ev.metric} | ${ev.measured} | ${ev.threshold} | ${ev.baseline} | ${ev.unit} | ${dir} | ${status}`,
    );
  }
  console.log("");

  const failures = evaluations.filter((e) => !e.passed);
  if (failures.length > 0) {
    console.error(`✗ throughput-latency-gate: FAIL — ${failures.length} regression threshold violation(s):`);
    for (const f of failures) {
      console.error(`  - [${f.routeId}] ${f.metric}: ${f.reason}`);
    }
    process.exit(1);
  }

  console.log(`✓ throughput-latency-gate: PASS — all ${evaluations.length} metric checks within approved budgets.`);
  process.exit(0);
}

await main();
