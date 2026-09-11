/**
 * BWASM-X-001 — QuickJS-NG-in-WASM engine parity spike qualification test.
 *
 * Verifies that the parity spike satisfies all atomic acceptance criteria:
 * 1. Reproducible source and toolchain references documented without masquerading as production.
 * 2. Engine-version mismatch (0.15.1 native vs 0.12.1 wasm) measured and explicitly classified.
 * 3. Payload (+242 KB brotli), startup (1,475x slower), and latency (310x slower) costs compared using raw evidence.
 * 4. Infinite loop/cancellation/recovery behavior analyzed and demonstrated.
 * 5. Scored decision matrix proves an unambiguous NO-GO verdict (9/30 vs 24/30 threshold).
 * 6. NO-GO decision leaves default Worker-based Browser-WASM target clean and unaffected.
 */

import { describe, expect, it } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

describe("BWASM-X-001 QuickJS-NG-in-WASM parity spike qualification", () => {
  const root = join(import.meta.dir, "../../..");
  const evidenceDir = join(root, "docs/browser-wasm/evidence/x001");
  const budgetsPath = join(root, "docs/browser-wasm/evidence/budgets.json");

  it("spike evidence packet is complete and well-formed", () => {
    const requiredFiles = [
      "01-toolchain-inventory.json",
      "01-toolchain-inventory.md",
      "02-comparative-benchmark.json",
      "02-comparative-benchmark.md",
      "03-payload-analysis.md",
      "04-semantic-parity-matrix.md",
      "05-cancellation-and-recovery.md",
      "06-go-no-go-decision.md",
    ];

    for (const file of requiredFiles) {
      const fullPath = join(evidenceDir, file);
      expect(existsSync(fullPath)).toBeTrue();
      const content = readFileSync(fullPath, "utf8");
      expect(content.length).toBeGreaterThan(100);
    }
  });

  it("toolchain inventory accurately classifies the engine version gap and build blockers", () => {
    const inventoryRaw = readFileSync(join(evidenceDir, "01-toolchain-inventory.json"), "utf8");
    const inventory = JSON.parse(inventoryRaw);

    expect(inventory.nativeTarget.pinnedQuickjsNgVersion).toBe("0.15.1");
    expect(inventory.nativeTarget.pinnedRquickjsVersion).toBe("=0.12.2");

    // Target wasm32-unknown-unknown build blockers
    expect(inventory.wasm32TargetAttempt.rquickjsSysStatus).toBe("FAILED");
    expect(inventory.wasm32TargetAttempt.rquickjsSysError).toContain("bindings/wasm32-unknown-unknown.rs");
    expect(inventory.wasm32TargetAttempt.tokioStatus).toBe("FAILED");
    expect(inventory.wasm32TargetAttempt.tokioError).toContain("This wasm target is unsupported by mio");

    // Upstream ecosystem version gap
    expect(inventory.upstreamWasmEcosystem.vendoredQuickjsNgVersion).toBe("0.12.1");
    expect(inventory.upstreamWasmEcosystem.versionSkew.bytecodeCompatibility).toBeFalse();
    expect(inventory.upstreamWasmEcosystem.binaryArtifact.rawBytes).toBeGreaterThan(500_000);
  });

  it("comparative benchmark data demonstrates unacceptable startup and latency regressions", () => {
    const benchRaw = readFileSync(join(evidenceDir, "02-comparative-benchmark.json"), "utf8");
    const bench = JSON.parse(benchRaw);

    // Cold start overhead
    expect(bench.coldStartup.quickjsWasm.totalColdStartMs).toBeGreaterThan(50);
    expect(bench.coldStartup.nativeBrowserJs.totalColdStartMs).toBeLessThan(1);
    expect(bench.coldStartup.overheadFactor).toBeGreaterThan(1000);

    // Request throughput
    expect(bench.requestThroughput.quickjsWasm.meanLatencyUs).toBeGreaterThan(400);
    expect(bench.requestThroughput.nativeBrowserJs.meanLatencyUs).toBeLessThan(10);
    expect(bench.requestThroughput.slowdownFactor).toBeGreaterThan(200);

    // Memory risk
    expect(bench.memoryMetrics.quickjsWasm.wasmInitialLinearMemoryMb).toBeGreaterThanOrEqual(16);
    expect(bench.memoryMetrics.quickjsWasm.disposalAssertionRisk).toBe("list_empty(&rt->gc_obj_list)");
  });

  it("payload analysis proves QuickJS-WASM violates ratified release budgets", () => {
    const budgets = JSON.parse(readFileSync(budgetsPath, "utf8"));
    const glueBudget = budgets.size_budgets.runtime_js_glue.target_max; // 51,200 bytes

    const inventory = JSON.parse(
      readFileSync(join(evidenceDir, "01-toolchain-inventory.json"), "utf8"),
    );
    const qjsWasmBrotli = inventory.upstreamWasmEcosystem.binaryArtifact.brotli11Bytes; // 241,890 bytes

    // QuickJS WASM alone exceeds the entire runtime glue budget by > 4x
    expect(qjsWasmBrotli).toBeGreaterThan(glueBudget * 4);
  });

  it("decision record renders an unambiguous NO-GO verdict satisfying all acceptance criteria", () => {
    const decisionMd = readFileSync(join(evidenceDir, "06-go-no-go-decision.md"), "utf8");

    expect(decisionMd).toContain("VERDICT: UNAMBIGUOUS NO-GO");
    expect(decisionMd).toContain("9 / 30");
    expect(decisionMd).toContain("THRESHOLD NOT MET (Required: >= 24/30)");
    expect(decisionMd).toContain("The ratified **Hybrid Browser-WASM Architecture**");
  });

  it("the NO-GO verdict leaves the production runtime clean of QuickJS WASM dependencies", () => {
    const pkgJson = JSON.parse(
      readFileSync(join(root, "packages/browser-runtime/package.json"), "utf8"),
    );

    const deps = Object.keys(pkgJson.dependencies ?? {});
    const devDeps = Object.keys(pkgJson.devDependencies ?? {});
    const allDeps = [...deps, ...devDeps];

    expect(allDeps.some((d) => d.includes("quickjs"))).toBeFalse();
    expect(allDeps.some((d) => d.includes("@jitl"))).toBeFalse();
  });
});
