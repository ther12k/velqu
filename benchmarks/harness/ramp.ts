/**
 * First-request-through-steady-state harness (BETA-003-B).
 *
 * Bridges the gap between the cold-start harness (single first request per
 * fresh process) and the warm harness (steady state only): one fresh process
 * per sample, then SEQUENTIAL validated requests from request #0 until a
 * deterministic steady-state criterion is met, recording every request's
 * latency. The per-request series is retained raw; the summary reports the
 * first-request latency, the steady-state latency, and the request index at
 * which steady state began.
 *
 * Steady-state criterion (deterministic, no wall-clock dependence): split
 * the request series into windows of WINDOW requests; a window transition is
 * "flat" when FLOOR * median(window k-1) <= median(window k) <= RATIO *
 * median(window k-1) — neither a >25% regression nor a >20% improvement. The
 * series has "reached steady state" at the start of window k (k >= 2) when
 * both the k and k-1 transitions are flat. A series that is still decaying
 * (or still regressing) at the maxRequests cap is reported as "no onset
 * observed" (honest, never extrapolated).
 *
 * Usage: bun ramp.ts [--reps=3] [--max-requests=400] [--only=velqu,raw-rust]
 * Raw rows: benchmarks/raw/ramp/<runId>.jsonl; summary:
 * benchmarks/raw/ramp/summary.json (+ ramp-report.md).
 */

import { mkdirSync, writeFileSync, existsSync } from "node:fs";

const ROOT = import.meta.dir + "/../..";

// Pinned baseline deps (same policy as run-w4.sh): install from the committed
// lockfile when absent — never resolve unpinned latest versions.
if (!existsSync(`${ROOT}/baselines/elysia2/node_modules`)) {
  const inst = Bun.spawnSync(["bun", "install", "--frozen-lockfile"], {
    cwd: `${ROOT}/baselines/elysia2`,
    stdout: "inherit",
    stderr: "inherit",
  });
  if (inst.exitCode !== 0) {
    console.error("ramp: baselines/elysia2 dependency install failed");
    process.exit(1);
  }
}
const WINDOW = 25;
const RATIO = 1.25;
const FLOOR = 0.8;
const MIN_STEADY_REQUESTS = 50;

interface Candidate {
  id: string;
  spawn: (port: number) => { cmd: string; args: string[]; env: Record<string, string> };
}

const CANDIDATES: Candidate[] = [
  {
    id: "velqu",
    spawn: (port) => ({
      cmd: `${ROOT}/target/release/velqu-runtime`,
      args: ["--pack", process.env.VELQU_PACK ?? `${ROOT}/examples/proof/dist/app.qpack`, "--port", String(port)],
      env: {},
    }),
  },
  {
    id: "raw-rust",
    spawn: (port) => ({
      cmd: `${ROOT}/baselines/raw-rust/target/release/velqu-baseline-raw-rust`,
      args: [],
      env: { PORT: String(port) },
    }),
  },
  {
    id: "raw-bun",
    spawn: (port) => ({
      cmd: "bun",
      args: [`${ROOT}/baselines/raw-bun/server.ts`],
      env: { PORT: String(port) },
    }),
  },
  {
    id: "elysia2",
    spawn: (port) => ({
      cmd: "bun",
      args: [`${ROOT}/baselines/elysia2/server.ts`],
      env: { PORT: String(port) },
    }),
  },
];

interface ClassSpec {
  class: string;
  routeId: string;
  method: "GET" | "POST";
  path: string;
  body?: unknown;
  headers?: Record<string, string>;
  expected: { status: number; body: string };
}

// C0 isolates the native transport floor; C2 adds JS handler + JSON work.
const CLASSES: ClassSpec[] = [
  { class: "C0", routeId: "health.live", method: "GET", path: "/health/live", expected: { status: 200, body: '{"status":"ok"}' } },
  { class: "C2", routeId: "js.json", method: "GET", path: "/js-json", expected: { status: 200, body: '{"ok":true}' } },
];

// ---------- pure, testable helpers ----------

export function round3(x: number): number {
  return Math.round(x * 1000) / 1000;
}

export function percentile(sorted: number[], q: number): number {
  if (sorted.length === 0) return 0;
  const idx = Math.min(sorted.length - 1, Math.round(q * (sorted.length - 1)));
  return round3(sorted[idx]);
}

export function median(nums: number[]): number {
  if (nums.length === 0) return 0;
  const sorted = [...nums].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

/**
 * Index of the first request belonging to the steady phase, or null when the
 * cap was reached without two consecutive non-regressing window transitions.
 */
export function steadyOnsetIndex(latenciesUs: number[], window = WINDOW, ratio = RATIO, floor = FLOOR): number | null {
  const windows: number[] = [];
  for (let i = 0; i + window <= latenciesUs.length; i += window) {
    windows.push(median(latenciesUs.slice(i, i + window)));
  }
  const flat = (cur: number, prev: number): boolean =>
    prev > 0 && cur >= floor * prev && cur <= ratio * prev;
  for (let k = 2; k < windows.length; k++) {
    if (flat(windows[k], windows[k - 1]) && flat(windows[k - 1], windows[k - 2])) {
      return k * window;
    }
  }
  return null;
}

export interface RampAggregates {
  firstRequestUs: number[];
  steadyUs: number[];
  onsetIndices: number[];
  errors: number;
}

/** Phase label for one request, given the (post-run) steady onset. */
export function phaseLabel(index: number, onset: number | null): "first" | "warming" | "steady" {
  if (index === 0) return "first";
  if (onset === null || index < onset) return "warming";
  return "steady";
}

export function aggregateRamp(
  reps: Array<{ latenciesUs: number[]; errors: number }>,
  onset: (latenciesUs: number[]) => number | null = (l) => steadyOnsetIndex(l),
): RampAggregates & { steadyOnsetRequest: number | null; firstRequest: Record<string, number>; steady: Record<string, number> } {
  const firstRequestUs: number[] = [];
  const steadyUs: number[] = [];
  const onsetIndices: number[] = [];
  let errors = 0;
  for (const rep of reps) {
    errors += rep.errors;
    if (rep.latenciesUs.length > 0) firstRequestUs.push(rep.latenciesUs[0]);
    const o = onset(rep.latenciesUs);
    if (o !== null) {
      onsetIndices.push(o);
      steadyUs.push(...rep.latenciesUs.slice(o));
    }
  }
  const sortedFirst = [...firstRequestUs].sort((a, b) => a - b);
  const sortedSteady = [...steadyUs].sort((a, b) => a - b);
  return {
    firstRequestUs,
    steadyUs,
    onsetIndices,
    errors,
    steadyOnsetRequest: onsetIndices.length ? Math.round(median(onsetIndices)) : null,
    firstRequest: {
      n: firstRequestUs.length,
      p50: percentile(sortedFirst, 0.5),
      p95: percentile(sortedFirst, 0.95),
      p99: percentile(sortedFirst, 0.99),
    },
    steady: {
      n: steadyUs.length,
      p50: percentile(sortedSteady, 0.5),
      p95: percentile(sortedSteady, 0.95),
      p99: percentile(sortedSteady, 0.99),
    },
  };
}

// ---------- harness ----------

function freePort(): number {
  // @ts-ignore — Bun exposes a TCP listener
  const l = Bun.listen({ hostname: "127.0.0.1", port: 0, socket: { data() {}, open() {} } });
  const port = l.port;
  l.stop(true);
  return port;
}

async function rssKb(pid: number): Promise<number | null> {
  try {
    const status = await Bun.file(`/proc/${pid}/status`).text();
    const m = status.match(/VmRSS:\s+(\d+) kB/);
    return m ? parseInt(m[1], 10) : null;
  } catch {
    return null;
  }
}

interface RampRow {
  runId: string;
  candidate: string;
  class: string;
  routeId: string;
  rep: number;
  requestIndex: number;
  latencyUs: number;
  phase: "first" | "warming" | "steady";
  valid: boolean;
  error?: string;
}

async function oneRamp(
  cand: Candidate,
  spec: ClassSpec,
  rep: number,
  runId: string,
  maxRequests: number,
): Promise<{ rows: RampRow[]; rssKbAfter: number | null; exitOk: boolean }> {
  const port = freePort();
  const { cmd, args, env } = cand.spawn(port);
  const proc = Bun.spawn([cmd, ...args], {
    env: { ...process.env, ...env },
    stdout: "ignore",
    stderr: "ignore",
  });
  const rows: RampRow[] = [];
  try {
    // poll TCP accept (0.5ms interval, 10s cap)
    let ready = false;
    const readyDeadline = performance.now() + 10_000;
    while (performance.now() < readyDeadline) {
      try {
        const c = await Bun.connect({
          hostname: "127.0.0.1",
          port,
          socket: { data() {}, open() {}, close() {}, error() {} },
        });
        c.end?.();
        c.terminate?.();
        ready = true;
        break;
      } catch {
        await Bun.sleep(0.5);
      }
    }
    if (!ready) throw new Error("server never accepted TCP");

    const latencies: number[] = [];
    let onset: number | null = null;
    for (let i = 0; i < maxRequests; i++) {
      let latencyUs = 0;
      let valid = false;
      let error: string | undefined;
      try {
        const t0 = performance.now();
        const res = await fetch(`http://127.0.0.1:${port}${spec.path}`, {
          method: spec.method,
          headers: {
            ...(spec.body !== undefined ? { "content-type": "application/json" } : {}),
            ...(spec.headers ?? {}),
          },
          body: spec.body !== undefined ? JSON.stringify(spec.body) : undefined,
        });
        const body = await res.text();
        latencyUs = Math.round((performance.now() - t0) * 1000);
        valid = res.status === spec.expected.status && body === spec.expected.body;
        if (!valid) error = `status=${res.status} body=${body.slice(0, 80)}`;
      } catch (e) {
        error = e instanceof Error ? e.message : String(e);
      }
      latencies.push(valid ? latencyUs : Number.MAX_SAFE_INTEGER / 2);
      rows.push({ runId, candidate: cand.id, class: spec.class, routeId: spec.routeId, rep, requestIndex: i, latencyUs, phase: "warming", valid, error });

      // evaluate the criterion on the completed windows; require a minimum
      // steady tail so the "steady" bucket is not a handful of samples
      if ((i + 1) % WINDOW === 0) {
        const onsetNow = steadyOnsetIndex(latencies);
        if (onsetNow !== null && latencies.length - onsetNow >= MIN_STEADY_REQUESTS) {
          onset = onsetNow;
          break;
        }
      }
      if (i + 1 === maxRequests) onset = steadyOnsetIndex(latencies);
    }

    // a series that never met the criterion keeps onset null; if it met it
    // but stopped at the cap early, keep the found onset
    const finalOnset = onset ?? steadyOnsetIndex(latencies);
    for (const row of rows) row.phase = phaseLabel(row.requestIndex, finalOnset);

    const rss = await rssKb(proc.pid);
    proc.kill();
    const code = await proc.exited;
    return { rows, rssKbAfter: rss, exitOk: code !== null };
  } catch (e) {
    proc.kill();
    await proc.exited;
    const message = e instanceof Error ? e.message : String(e);
    if (rows.length === 0) {
      rows.push({ runId, candidate: cand.id, class: spec.class, routeId: spec.routeId, rep, requestIndex: 0, latencyUs: 0, phase: "first", valid: false, error: message });
    } else {
      rows[rows.length - 1].error = rows[rows.length - 1].error ?? message;
    }
    return { rows, rssKbAfter: null, exitOk: false };
  }
}

function renderReport(
  environment: Record<string, unknown>,
  results: Array<Record<string, unknown>>,
): string {
  const lines: string[] = [
    "# First Request Through Steady State (BETA-003-B)",
    "",
    "One fresh process per sample; sequential validated requests from request #0;",
    `steady onset = start of the window after two consecutive flat window`,
    `transitions (window=${WINDOW}, flat = within [${FLOOR}x, ${RATIO}x] of the`,
    `previous window median); series capped at --max-requests.`,
    "",
    `Environment: ${JSON.stringify(environment)}`,
    "",
    "| candidate | class | first p50 (µs) | steady p50 (µs) | first/steady | steady onset (req #) | errors | RSS (kB) |",
    "|---|---|---:|---:|---:|---:|---:|---:|",
  ];
  for (const r of results) {
    const first = r.firstRequest as Record<string, number>;
    const steady = r.steady as Record<string, number>;
    const ratioText =
      steady.n > 0 && steady.p50 > 0 ? (Math.round((first.p50 / steady.p50) * 100) / 100).toFixed(2) : "n/a";
    lines.push(
      `| ${r.candidate} | ${r.class} | ${first.p50} | ${steady.p50} | ${ratioText} | ${r.steadyOnsetRequest ?? "none"} | ${r.errors} | ${r.rssKbAfter ?? "n/a"} |`,
    );
  }
  lines.push(
    "",
    "Errors > 0 or `none` onset are retained findings, never smoothed away.",
    "",
  );
  return lines.join("\n");
}

async function main() {
  const args = process.argv.slice(2);
  const get = (name: string, dflt: string) => args.find((a) => a.startsWith(`--${name}=`))?.slice(name.length + 3) ?? dflt;
  const reps = parseInt(get("reps", "3"), 10);
  const maxRequests = parseInt(get("max-requests", "400"), 10);
  const only = get("only", "");
  const runId = process.env.RAMP_RUN_ID ?? `ramp-${Date.now()}`;
  const outDir = `${ROOT}/benchmarks/raw/ramp`;
  mkdirSync(outDir, { recursive: true });

  const cands = only ? CANDIDATES.filter((c) => only.split(",").includes(c.id)) : CANDIDATES;
  const environment = {
    bun: Bun.version,
    node: await (async () => {
      const p = Bun.spawn(["node", "--version"], { stdout: "pipe", stderr: "ignore" });
      return (await p.exited) === 0 ? (await new Response(p.stdout).text()).trim() : null;
    })(),
    kernel: (await Bun.file("/proc/version").text()).trim(),
    cpu: (await Bun.file("/proc/cpuinfo").text()).match(/model name\s+:\s+(.+)\n/)?.[1]?.trim() ?? "unknown",
    commit: await (async () => {
      const p = Bun.spawn(["git", "rev-parse", "HEAD"], { cwd: ROOT, stdout: "pipe", stderr: "ignore" });
      return (await p.exited) === 0 ? (await new Response(p.stdout).text()).trim() : "unknown";
    })(),
  };

  console.log(`ramp: ${cands.length} candidates x ${CLASSES.length} classes x ${reps} reps (cap ${maxRequests} requests)`);
  const allRows: RampRow[] = [];
  const results: Array<Record<string, unknown>> = [];

  for (const cand of cands) {
    for (const spec of CLASSES) {
      const repResults: Array<{ latenciesUs: number[]; errors: number }> = [];
      let rssKbAfter: number | null = null;
      let exitOk = true;
      for (let rep = 0; rep < reps; rep++) {
        const r = await oneRamp(cand, spec, rep, runId, maxRequests);
        allRows.push(...r.rows);
        repResults.push({
          latenciesUs: r.rows.filter((row) => row.valid).map((row) => row.latencyUs),
          errors: r.rows.filter((row) => !row.valid).length,
        });
        rssKbAfter = r.rssKbAfter ?? rssKbAfter;
        exitOk = exitOk && r.exitOk;
      }
      const agg = aggregateRamp(repResults);
      results.push({
        candidate: cand.id,
        class: spec.class,
        routeId: spec.routeId,
        reps,
        requestsPerRep: repResults.map((r) => r.latenciesUs.length),
        ...agg,
        firstRequestUs: undefined,
        steadyUs: undefined,
        onsetIndices: undefined,
        rssKbAfter,
        exitOk,
      });
      const first = agg.firstRequest;
      console.log(
        `ramp: ${cand.id}/${spec.class}: first p50=${first.p50}µs, steady p50=${agg.steady.p50}µs, onset=${agg.steadyOnsetRequest ?? "none"}, errors=${agg.errors}`,
      );
    }
  }

  writeFileSync(outDir + `/${runId}.jsonl`, allRows.map((r) => JSON.stringify(r)).join("\n") + "\n");
  const summary = {
    format: "velqu-ramp-v1",
    runId,
    window: WINDOW,
    ratio: RATIO,
    floor: FLOOR,
    minSteadyRequests: MIN_STEADY_REQUESTS,
    maxRequests,
    reps,
    generatedAt: new Date().toISOString(),
    environment,
    results,
  };
  writeFileSync(outDir + "/summary.json", JSON.stringify(summary, null, 2) + "\n");
  writeFileSync(outDir + "/ramp-report.md", renderReport(environment, results));
  console.log(`ramp: wrote ${outDir}/summary.json and ramp-report.md`);

  const withErrors = results.filter((r) => (r.errors as number) > 0 || r.steadyOnsetRequest === null);
  if (withErrors.length > 0) {
    console.error(`ramp: FAIL — ${withErrors.length} cell(s) with errors or no steady onset: ${withErrors.map((r) => `${r.candidate}/${r.class}`).join(", ")}`);
    process.exit(1);
  }
}

if (import.meta.main) {
  main();
}
