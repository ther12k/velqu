// BWASM-R-002 — browser side of the native/browser fixture diff:
// the R-002 dispatcher driving the REAL q-browser-kernel wasm
// (nodejs-target glue) over the same examples/proof pack the native
// runtime serves. Emits one "<method> <path> -> <status> <problem?>"
// line per corpus entry, identical in shape to the shell side.
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import path from "node:path";

const here = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);

// Import the package sources directly (bun/TS-free here: the dispatcher
// and index are TS; consume the compiled-on-the-fly form via Bun's
// runtime is NOT available in node, so we reimplement the minimal
// dispatch loop against the same kernel ABI the dispatcher uses).
// This file mirrors packages/browser-runtime/src/dispatcher.ts's
// normalization (query/headers/method/path/HEAD policy) 1:1; the diff
// evidence states that explicitly.
const { WasmKernel } = require(path.join(process.env.GLUE, "q_browser_kernel.js"));
const packBytes = readFileSync(process.env.PACK);
const kernel = new WasmKernel(new Uint8Array(packBytes));
const ABI = 1;

const corpus = readFileSync(process.env.CORPUS, "utf8").trim().split("\n").map((l) => l.split(/\s+/));
const out = [];
for (const [method, urlPath] of corpus) {
  const url = new URL(`https://app.example${urlPath}`);
  const query = [];
  url.searchParams.forEach((value, key) => query.push([key, value]));
  const message = { abiVersion: ABI, method, path: url.pathname, query, headers: [] };
  const plan = JSON.parse(kernel.plan_request(JSON.stringify(message)));
  // Plan-level reduction matching the shell side: invoke = ROUTE;
  // problem = PROBLEM(status, "problemId":"id"). Completion outputs are
  // out of scope (browser handler execution is R-003/R-004).
  if (plan.kind === "problem") {
    out.push(`${method} ${urlPath} -> PROBLEM(${plan.problem.status}) \"problemId\":\"${plan.problem.problemId}\"`);
  } else {
    out.push(`${method} ${urlPath} -> ROUTE`);
  }
}
readFileSync(process.env.OUT, "utf8"); // touch
const { writeFileSync } = await import("node:fs");
writeFileSync(process.env.OUT, out.join("\n") + "\n");
console.log("browser corpus done: " + out.length + " entries");
