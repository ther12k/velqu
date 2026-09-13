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
 *
 * Usage: bun scripts/throughput-latency-gate.ts [--duration-secs N] [--concurrency N] [--routes C0,C1...]
 */
import { $ } from "bun";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "..");

function argValue(name: string): string | undefined {
  const i = process.argv.indexOf(name);
  return i !== -1 ? process.argv[i + 1] : undefined;
}

const DURATION_SECS = Number(argValue("--duration-secs")) || 3;
const CONCURRENCY = Number(argValue("--concurrency")) || 10;
const ROUTES_FILTER = argValue("--routes")?.split(",").map((s) => s.trim());
const BASE_PORT = Number(argValue("--port")) || 19500;

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

const thresholdsPath = join(root, "benchmarks", "gate-thresholds.json");
if (!existsSync(thresholdsPath)) {
  console.error(`throughput-latency-gate: thresholds file missing at ${thresholdsPath}`);
  process.exit(2);
}

const doc: ThresholdsDoc = JSON.parse(readFileSync(thresholdsPath, "utf8"));
const config = doc.throughputLatency;
if (!config || !config.routes || Object.keys(config.routes).length === 0) {
  console.error("throughput-latency-gate: no throughputLatency.routes configured in benchmarks/gate-thresholds.json");
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

    const targetRouteIds = Object.keys(config!.routes).filter(
      (id) => !ROUTES_FILTER || ROUTES_FILTER.includes(id),
    );

    if (targetRouteIds.length === 0) {
      throw new Error(`No matching routes found for filter: ${ROUTES_FILTER?.join(", ")}`);
    }

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
  console.log(`Duration: ${DURATION_SECS}s per route | Concurrency: ${CONCURRENCY} | Target routes: ${ROUTES_FILTER ? ROUTES_FILTER.join(", ") : "all"}`);
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
