// BWASM-K-006 — drive the ACTUAL wasm-bindgen JS ABI (nodejs target)
// end-to-end outside Rust's test harness: init → plan → complete.
const { WasmKernel, kernel_abi_version } = require("/tmp/k006-glue/q_browser_kernel.js");
const assert = require("node:assert");

// Reconstruct the deterministic test pack the same way the Rust suite
// does (q_pack::minimal_pack_public serialized by the repo toolchain —
// here we reuse the bytes emitted by the evidence generator below).
const packBytes = require("node:fs").readFileSync(process.argv[2]);

assert.strictEqual(kernel_abi_version(), 1, "ABI version export");
const k = new WasmKernel(new Uint8Array(packBytes));

const plan = JSON.parse(k.plan_request(JSON.stringify({
  abiVersion: 1, method: "GET", path: "/health/live",
})));
assert.strictEqual(plan.kind, "invoke");
assert.strictEqual(plan.handlerKey, "health.live");
assert.deepStrictEqual(plan.allowedStatuses, [200]);

const nf = JSON.parse(k.plan_request(JSON.stringify({
  abiVersion: 1, method: "GET", path: "/nope",
})));
assert.strictEqual(nf.problem.problemId, "not-found");
assert.strictEqual(nf.problem.status, 404);

const done = JSON.parse(k.complete_invocation(JSON.stringify({
  abiVersion: 1, routeId: plan.routeId,
  result: { kind: "response", status: 200, headers: [], body: { status: "ok" } },
})));
assert.strictEqual(done.kind, "response");
assert.strictEqual(done.body.status, "ok");

const undeclared = JSON.parse(k.complete_invocation(JSON.stringify({
  abiVersion: 1, routeId: plan.routeId,
  result: { kind: "response", status: 418, headers: [], body: {} },
})));
assert.strictEqual(undeclared.problem.problemId, "internal");
assert.ok(undeclared.problem.detail.includes("undeclared status 418"));

k.dispose();
console.log("JS-ABI-OK: init+plan(200/404)+complete(200/contract-violation) through the real wasm-bindgen surface");
