/**
 * Master benchmark runner (scripts/benchmark).
 * Runs the full reproducible benchmark suite:
 * 1. Build proof and scale packs
 * 2. Bridge microbenchmark (strategy A vs B)
 * 3. Cold-start suite (C0–C5 fresh-process)
 * 4. Route-count scaling suite (25 vs 1,000 routes)
 * 5. Warm throughput & latency suite
 * 6. TypeScript type-system scale suite (100, 500, 1,000 routes)
 * 7. Emits benchmarks/manifest.json with all hashes and environment details.
 */

import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";

const ROOT = import.meta.dir + "/../..";

async function run(name: string, cmd: string[]) {
  console.log(`\n>>> [BENCHMARK] ${name}...`);
  const t0 = performance.now();
  const proc = Bun.spawn(cmd, { cwd: ROOT, stdout: "inherit", stderr: "inherit" });
  const code = await proc.exited;
  if (code !== 0) throw new Error(`${name} failed with exit code ${code}`);
  console.log(`<<< ${name} completed in ${Math.round(performance.now() - t0)}ms`);
}

async function fileHash(path: string): Promise<string> {
  try {
    const bytes = await Bun.file(path).arrayBuffer();
    return createHash("sha256").update(Buffer.from(bytes)).digest("hex");
  } catch {
    return "missing";
  }
}

async function main() {
  console.log("=== Velqu Master Benchmark Suite ===");

  // 1. Build packs
  await run("Build Packs", ["bun", "benchmarks/harness/build-proof-pack.ts", "fixture"]);
  await run("Build Scale Packs (25)", ["bun", "benchmarks/harness/build-proof-pack.ts", "25"]);
  await run("Build Scale Packs (1000)", ["bun", "benchmarks/harness/build-proof-pack.ts", "1000"]);
  await run("Embed Bytecode (25)", ["./target/release/velqu-bytecode", "embed", "--pack", "benchmarks/raw/packs/app-25.qpack", "--out", "benchmarks/raw/packs/app-25-bc.qpack"]);
  await run("Embed Bytecode (1000)", ["./target/release/velqu-bytecode", "embed", "--pack", "benchmarks/raw/packs/app-1000.qpack", "--out", "benchmarks/raw/packs/app-1000-bc.qpack"]);

  // 2. Bridge benchmark
  await run("Bridge Microbenchmark", ["./target/release/q-bridge-bench", "--out-dir", "benchmarks/raw/bridge", "--iters", "2000"]);

  // 3. Cold-start benchmark
  await run("Cold-Start Suite", ["bun", "benchmarks/harness/cold-start.ts", "--samples=60"]);

  // 4. Route-count scaling
  await run("Route-Count Scaling Suite", ["bun", "benchmarks/harness/route-count.ts", "--samples=40"]);

  // 5. Warm load
  await run("Warm-Load Suite", ["bun", "benchmarks/harness/warm.ts"]);

  // 6. TypeScript scale
  await run("TypeScript Scale Suite", ["bun", "benchmarks/type-scale/measure.ts"]);

  // 7. Emit master manifest
  const manifest = {
    format: "velqu-benchmark-manifest-v1",
    generatedAt: new Date().toISOString(),
    environment: {
      platform: "linux",
      kernel: (await Bun.file("/proc/version").text()).trim(),
      cpu: (await Bun.file("/proc/cpuinfo").text()).match(/model name\s+:\s+(.+)\n/)?.[1]?.trim() ?? "unknown",
      bunVersion: Bun.version,
      typescriptVersion: "5.9.3",
      rustcVersion: "1.96.0",
      pinnedEngine: "quickjs-ng 0.15.1 via rquickjs 0.12.2",
    },
    artifacts: {
      qRuntimeRelease: {
        path: "target/release/velqu-runtime",
        sha256: await fileHash("target/release/velqu-runtime"),
      },
      proofPack: {
        path: "examples/proof/dist/app.qpack",
        sha256: await fileHash("examples/proof/dist/app.qpack"),
      },
    },
    evidence: {
      coldStartSummary: "benchmarks/raw/cold-start/summary.json",
      routeCountSummary: "benchmarks/raw/route-count/summary.json",
      warmSummary: "benchmarks/raw/warm/summary.json",
      bridgeSummary: "benchmarks/raw/bridge/bridge-summary.json",
      typeScaleResults: "benchmarks/type-scale/results.json",
    },
  };

  writeFileSync("benchmarks/manifest.json", JSON.stringify(manifest, null, 2));
  console.log("\n=== All Benchmarks Complete: wrote benchmarks/manifest.json ===");
}

await main();
