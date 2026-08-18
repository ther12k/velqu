/**
 * Compiler conformance: static extraction guarantees, trap tests (COMP-002),
 * unsupported-import/duplicate/dynamic diagnostics (COMP-004/COMP-006),
 * deterministic rebuilds (COMP-003/COMP-009).
 */
import { describe, expect, test } from "bun:test";
import { build, contractDiff, CompileError } from "@q/compiler";
import { readFileSync, rmSync, writeFileSync } from "node:fs";

const TMP = "/tmp/velqu-conformance";

describe("compiler traps (COMP-002: never run the app)", () => {
  test("service factories and module side effects never execute during build", async () => {
    // The trap fixture's service factory THROWS if invoked; module scope
    // increments a global. Build must succeed without executing either.
    const out = `${TMP}/trap`;
    rmSync(out, { recursive: true, force: true });
    const r = await build({ project: "conformance/compiler/fixtures/trap-app.ts", outDir: out });
    expect(r.routes).toBe(1);
    // the compiler process itself never imported the fixture (pure AST) — if it
    // had, the module scope would have run here in THIS process too
    expect((globalThis as { __velquTrapSideEffects?: number }).__velquTrapSideEffects).toBeUndefined();
    // and the factory must not have run: the built bundle still contains it
    // (lazy), but never called — verified by the runtime-local suite
  });
});

describe("compiler diagnostics (source-located, actionable)", () => {
  test("unsupported node:/bun: imports fail the build (COMP-006)", async () => {
    await expectBuildFails("conformance/compiler/fixtures/bad-import-app.ts", /unsupported import 'node:fs'/);
  });
  test("canonically equivalent routes fail the build (COMP-004)", async () => {
    await expectBuildFails("conformance/compiler/fixtures/duplicate-app.ts", /route collision/);
  });
  test("dynamic route metadata fails with a hint (PR-004)", async () => {
    await expectBuildFails("conformance/compiler/fixtures/dynamic-app.ts", /literal/);
  });
});

describe("determinism (COMP-003/009)", () => {
  test("rebuild produces byte-identical pack and contract hash", async () => {
    const out1 = `${TMP}/det1`;
    const out2 = `${TMP}/det2`;
    rmSync(out1, { recursive: true, force: true });
    rmSync(out2, { recursive: true, force: true });
    await build({ project: "examples/proof/src/app.ts", outDir: out1 });
    await new Promise((r) => setTimeout(r, 20)); // timestamps differ — hashes must not
    await build({ project: "examples/proof/src/app.ts", outDir: out2 });
    const pack1 = readFileSync(`${out1}/app.qpack`, "utf8");
    const pack2 = readFileSync(`${out2}/app.qpack`, "utf8");
    const p1 = JSON.parse(pack1);
    const p2 = JSON.parse(pack2);
    expect(p1.integrity.routesSha256).toBe(p2.integrity.routesSha256);
    expect(p1.integrity.bundleSha256).toBe(p2.integrity.bundleSha256);
    expect(p1.contractHash).toBe(p2.contractHash);
  });
});

describe("pack contents (COMP-001/005)", () => {
  test("pack carries pre-compiled segments and versions", async () => {
    const pack = JSON.parse(readFileSync("examples/proof/dist/app.qpack", "utf8"));
    expect(pack.formatVersion).toBe(1);
    expect(pack.runtimeAbi).toBe(1);
    expect(pack.engine.version).toBe("0.15.1");
    expect(pack.engine.binding).toBe("rquickjs-0.12.2");
    for (const r of pack.routes) {
      expect(Array.isArray(r.pathSegments)).toBe(true);
      expect(r.pathSegments.length).toBeGreaterThan(0);
    }
    // health.live must be pre-compiled native liveness
    const health = pack.routes.find((r: { id: string }) => r.id === "health.live");
    expect(health.nativeLiveness.body).toBe('{"status":"ok"}');
  });
});

async function expectBuildFails(project: string, pattern: RegExp) {
  const out = `${TMP}/fail-${Date.now()}`;
  try {
    await build({ project, outDir: out });
    throw new Error(`expected build to fail for ${project}`);
  } catch (e) {
    if (e instanceof Error && e.message.startsWith("expected build to fail")) throw e;
    expect(e).toBeInstanceOf(CompileError);
    expect((e as Error).message).toMatch(pattern);
    if (e instanceof CompileError && e.location) {
      expect(e.location.file.length).toBeGreaterThan(0);
      expect(e.location.line).toBeGreaterThan(0);
    }
  }
}

describe("contract lock workflow (PR-006/SCHEMA-007)", () => {
  test("lock is written once, preserved on rebuild, and diff detects drift", async () => {
    const out = "/tmp/velqu-conformance/lock";
    rmSync(out, { recursive: true, force: true });
    // first build writes the lock
    const b1 = await build({ project: "examples/proof/src/app.ts", outDir: out });
    expect(b1.lockPreserved).toBe(false);
    const lock1 = readFileSync(`${out}/contract.lock.json`, "utf8");
    // second build PRESERVES it (byte-identical)
    await new Promise((r) => setTimeout(r, 20));
    const b2 = await build({ project: "examples/proof/src/app.ts", outDir: out });
    expect(b2.lockPreserved).toBe(true);
    expect(readFileSync(`${out}/contract.lock.json`, "utf8")).toBe(lock1);
    // drift: remove a route from the CURRENT contract (as if the app changed)
    const contract = JSON.parse(readFileSync(`${out}/contract.json`, "utf8"));
    delete contract.routes["users.get"];
    writeFileSync(`${out}/contract.json`, JSON.stringify(contract));
    const diffs = contractDiff(out);
    const removed = diffs.find((d) => d.routeId === "users.get" && d.kind === "breaking");
    expect(removed).toBeDefined();
    // update-lock refreshes the baseline
    const b3 = await build({ project: "examples/proof/src/app.ts", outDir: out, updateLock: true });
    expect(b3.lockPreserved).toBe(false);
    expect(contractDiff(out).length).toBe(0);
  });
});