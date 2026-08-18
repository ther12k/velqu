/**
 * Type-system scale measurement (PERF-007): fresh `tsc --noEmit` for
 * N in {100, 500, 1000}; records wall time, peak RSS, declaration size.
 */
import { $ } from "bun";
import * as fs from "node:fs";

const Ns = [100, 500, 1000];
const results: unknown[] = [];

for (const n of Ns) {
  const dir = `benchmarks/type-scale/n${n}`;
  await $`rm -rf ${dir}`.quiet();
  await $`mkdir -p ${dir}`.quiet();
  await $`bun benchmarks/type-scale/gen.ts ${n} ${dir}`.quiet();

  const t0 = performance.now();
  const proc = Bun.spawn(["./node_modules/.bin/tsc", "-p", `${dir}/tsconfig.json`, "--noEmit"], {
    stdout: "pipe",
    stderr: "pipe",
  });
  // sample RSS while running
  let peakRss = 0;
  const sampler = setInterval(() => {
    try {
      const status = fs.readFileSync(`/proc/${proc.pid}/status`, "utf8");
      const m = status.match(/VmHWM:\s+(\d+) kB/);
      if (m) peakRss = Math.max(peakRss, parseInt(m[1], 10) * 1024);
    } catch {
      /* process exited */
    }
  }, 50);
  const err = await new Response(proc.stderr).text();
  await proc.exited;
  clearInterval(sampler);
  const elapsedMs = performance.now() - t0;

  const declBytes = (await Bun.file(`${dir}/api-types.ts`).arrayBuffer()).byteLength;

  // negative check: a deliberately-wrong param type MUST produce a tsc error
  await Bun.write(
    `${dir}/negative.ts`,
    `import type { ProofApi } from "./api-types";\nimport { treaty } from "@velqu/treaty";\n` +
      `const api = treaty<ProofApi>({ baseUrl: "http://x", contract: {} });\n` +
      // @ts-expect-error: id must be number
      `api.res7.get({ id: "not-a-number" });\n`,
  );
  const neg = Bun.spawn(["./node_modules/.bin/tsc", "-p", `${dir}/tsconfig.json`, "--noEmit"], {
    stdout: "pipe",
    stderr: "pipe",
  });
  await neg.exited;

  results.push({
    routes: n,
    freshTscMs: Math.round(elapsedMs),
    peakRssBytes: peakRss,
    declarationBytes: declBytes,
    errors: err ? err.slice(0, 400) : null,
    negativeCaught: neg.exitCode === 0 ? "NOT_CAUGHT" : "CAUGHT",
  });
  console.log(`n=${n}: ${Math.round(elapsedMs)}ms, peakRSS=${(peakRss / 1048576).toFixed(1)}MiB, decl=${declBytes}B, errors=${err ? "YES" : "none"}`);
}

await Bun.write("benchmarks/type-scale/results.json", JSON.stringify({ format: "velqu-type-scale-v1", results }, null, 2));
console.log("wrote benchmarks/type-scale/results.json");
