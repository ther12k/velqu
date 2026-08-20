/**
 * Cold-start harness: fresh-process protocol (benchmark-methodology.md).
 *
 * Per sample: pick port → start monotonic timer → spawn ONE release process →
 * poll TCP accept (ready) → issue the route-class request → validate the
 * first response byte-exactly (checkFirstResponse) → terminate → record.
 * Raw JSONL per candidate×class; summary with p50/p95/p99, mean, stdev.
 *
 * Route classes measured: C0 health.live, C1 js.text, C2 js.json,
 * C3 hello.get (validated path), C3 users.create (validated body, POST),
 * C4 users.get (policy), C5 async.timer.
 */

import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";

interface Candidate {
  id: string;
  spawn: (port: number) => { cmd: string; args: string[]; env: Record<string, string> };
  artifactBytes?: number;
}

const ROOT = import.meta.dir + "/../..";

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

const CLASSES: ClassSpec[] = [
  { class: "C0", routeId: "health.live", method: "GET", path: "/health/live", expected: { status: 200, body: '{"status":"ok"}' } },
  { class: "C1", routeId: "js.text", method: "GET", path: "/js-text", expected: { status: 200, body: "plain" } },
  { class: "C2", routeId: "js.json", method: "GET", path: "/js-json", expected: { status: 200, body: '{"ok":true}' } },
  { class: "C3", routeId: "hello.get", method: "GET", path: "/hello/Rafi", expected: { status: 200, body: '{"message":"Hello Rafi"}' } },
  {
    class: "C3b",
    routeId: "users.create",
    method: "POST",
    path: "/users",
    body: { name: "Ada", email: "ada@example.org" },
    expected: { status: 201, body: '{"id":"usr_1","name":"Ada","email":"ada@example.org"}' },
  },
  {
    class: "C4",
    routeId: "users.get",
    method: "GET",
    path: "/users/usr_1",
    headers: { authorization: "Bearer q-demo-token" },
    expected: { status: 200, body: '{"id":"usr_1","name":"Ada","email":"ada@example.org"}' },
  },
  { class: "C5", routeId: "async.timer", method: "GET", path: "/async?ms=10", expected: { status: 200, body: '{"waited":10}' } },
];

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

interface Sample {
  candidate: string;
  class: string;
  routeId: string;
  spawnMs: number;      // process spawn → TCP accept (ready)
  readyMs: number;      // ready → first valid response
  totalMs: number;      // spawn → first valid response (THE metric)
  rssKbAfter: number | null;
  valid: boolean;
  exitOk: boolean;
  error?: string;
  index: number;
}

async function oneSample(cand: Candidate, spec: ClassSpec, index: number): Promise<Sample> {
  const port = freePort();
  const { cmd, args, env } = cand.spawn(port);
  const t0 = performance.now();
  const proc = Bun.spawn([cmd, ...args], {
    env: { ...process.env, ...env },
    stdout: "ignore",
    stderr: "ignore",
  });
  const base: Sample = {
    candidate: cand.id,
    class: spec.class,
    routeId: spec.routeId,
    spawnMs: 0,
    readyMs: 0,
    totalMs: 0,
    rssKbAfter: null,
    valid: false,
    exitOk: false,
    index,
  };
  try {
    // poll TCP accept with 0.5ms interval, 10s cap
    let readyAt: number | null = null;
    const deadline = performance.now() + 10_000;
    while (performance.now() < deadline) {
      try {
        const c = await Bun.connect({
          hostname: "127.0.0.1",
          port,
          socket: { data() {}, open() {}, close() {}, error() {} },
        });
        c.end?.();
        c.terminate?.();
        readyAt = performance.now();
        break;
      } catch {
        await Bun.sleep(0.5);
      }
    }
    if (readyAt === null) throw new Error("server never accepted TCP");
    const spawnMs = readyAt - t0;

    // issue the route-class request and validate
    let firstAt: number | null = null;
    let body = "";
    let status = 0;
    const fetchDeadline = performance.now() + 5_000;
    while (performance.now() < fetchDeadline) {
      try {
        const res = await fetch(`http://127.0.0.1:${port}${spec.path}`, {
          method: spec.method,
          headers: {
            ...(spec.body !== undefined ? { "content-type": "application/json" } : {}),
            ...(spec.headers ?? {}),
          },
          body: spec.body !== undefined ? JSON.stringify(spec.body) : undefined,
        });
        status = res.status;
        body = await res.text();
        firstAt = performance.now();
        break;
      } catch {
        await Bun.sleep(0.5);
      }
    }
    if (firstAt === null) throw new Error("no response in 5s");
    const valid = status === spec.expected.status && body === spec.expected.body;
    const rss = await rssKb(proc.pid);
    proc.kill();
    const code = await proc.exited;
    base.spawnMs = round3(spawnMs);
    base.readyMs = round3(firstAt - readyAt);
    base.totalMs = round3(firstAt - t0);
    base.valid = valid;
    base.rssKbAfter = rss;
    base.exitOk = true;
    if (!valid) base.error = `status=${status} body=${body.slice(0, 80)}`;
    return base;
  } catch (e) {
    proc.kill();
    await proc.exited;
    base.error = e instanceof Error ? e.message : String(e);
    return base;
  }
}

function round3(x: number): number {
  return Math.round(x * 1000) / 1000;
}

function percentile(sorted: number[], q: number): number {
  if (sorted.length === 0) return 0;
  const idx = Math.min(sorted.length - 1, Math.round(q * (sorted.length - 1)));
  return round3(sorted[idx]);
}

function stats(nums: number[]) {
  const sorted = [...nums].sort((a, b) => a - b);
  const mean = nums.reduce((a, b) => a + b, 0) / (nums.length || 1);
  const stdev = Math.sqrt(nums.reduce((a, b) => a + (b - mean) ** 2, 0) / (nums.length || 1));
  return { n: nums.length, mean: round3(mean), stdev: round3(stdev), p50: percentile(sorted, 0.5), p95: percentile(sorted, 0.95), p99: percentile(sorted, 0.99) };
}

async function main() {
  const args = process.argv.slice(2);
  const samplesPer = parseInt(args.find((a) => a.startsWith("--samples="))?.slice(10) ?? "60", 10);
  const runId = process.env.COLD_RUN_ID ?? `cold-${Date.now()}`;
  const only = args.find((a) => a.startsWith("--only="))?.slice(7);
  const outDir = `${ROOT}/benchmarks/raw/cold-start`;
  mkdirSync(outDir, { recursive: true });

  // warm the harness itself (methodology §samples)
  for (let i = 0; i < 5; i++) await oneSample(CANDIDATES[2], CLASSES[0], -1);

  const cands = only ? CANDIDATES.filter((c) => only.split(",").includes(c.id)) : CANDIDATES;
  // randomized/interleaved order to reduce thermal drift
  const jobs: Array<{ c: Candidate; s: ClassSpec }> = [];
  for (const c of cands) for (const s of CLASSES) for (let i = 0; i < samplesPer; i++) jobs.push({ c, s });
  // shuffle deterministically per run
  let seed = 0x9e3779b9;
  for (let i = jobs.length - 1; i > 0; i--) {
    seed = (seed * 1664525 + 1013904223) >>> 0;
    const j = seed % (i + 1);
    [jobs[i], jobs[j]] = [jobs[j], jobs[i]];
  }

  console.log(`cold-start: ${jobs.length} samples (${cands.map((c) => c.id).join(", ")})`);
  const all: Sample[] = [];
  let done = 0;
  for (const job of jobs) {
    all.push(await oneSample(job.c, job.s, done));
    done++;
    if (done % 50 === 0) console.log(`  ${done}/${jobs.length}`);
  }

  // raw JSONL
  const rawPath = `${outDir}/${runId}.jsonl`;
  const rawRelative = `benchmarks/raw/cold-start/${runId}.jsonl`;
  writeFileSync(rawPath, all.map((s) => JSON.stringify({ runId, ...s })).join("\n") + "\n");

  // summary
  const summary: Record<string, unknown> = {
    format: "velqu-cold-start-v2",
    runId,
    samplesPer,
    expectedRowsPerCell: samplesPer,
    generatedAt: new Date().toISOString(),
    environment: {
      bun: Bun.version,
      kernel: (await Bun.file("/proc/version").text()).trim(),
      cpu: (await Bun.file("/proc/cpuinfo").text()).match(/model name\s+:\s+(.+)\n/)?.[1]?.trim() ?? "unknown",
    },
    results: [],
  };
  for (const c of cands) {
    for (const s of CLASSES) {
      const rows = all.filter((x) => x.candidate === c.id && x.class === s.class);
      const good = rows.filter((x) => x.valid);
      const bad = rows.filter((x) => !x.valid);
      summary.results.push({
        candidate: c.id,
        class: s.class,
        routeId: s.routeId,
        total: stats(good.map((x) => x.totalMs)),
        toReady: stats(good.map((x) => x.spawnMs)),
        readyToFirst: stats(good.map((x) => x.readyMs)),
        rssKbAfterReady: stats(good.map((x) => x.rssKbAfter ?? 0).filter((x) => x > 0)),
        failures: bad.length,
        raw: rawRelative,
      });
    }
  }
  const sumPath = `${outDir}/summary.json`;
  writeFileSync(sumPath, JSON.stringify(summary, null, 2));
  console.log(`wrote ${rawPath}\nwrote ${sumPath}`);

  // compact table
  for (const c of cands) {
    for (const s of CLASSES) {
      const r = (summary.results as Array<{ candidate: string; class: string; total: { p50: number; p95: number } }>).find(
        (x) => x.candidate === c.id && x.class === s.class,
      )!;
      console.log(`${c.id.padEnd(9)} ${s.class.padEnd(4)} p50=${r.total.p50.toFixed(1)}ms p95=${r.total.p95.toFixed(1)}ms`);
    }
  }
}

await main();
