import { describe, it, expect, beforeEach, afterEach } from "bun:test";
import { ProjectWatcher, watchSourceAndContracts, type WatchEvent } from "./watch";
import { mkdirSync, writeFileSync, rmSync, existsSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

async function waitFor(predicate: () => boolean, timeoutMs = 2000): Promise<void> {
  const start = Date.now();
  while (!predicate()) {
    if (Date.now() - start > timeoutMs) {
      throw new Error("timed out waiting for condition");
    }
    await new Promise((r) => setTimeout(r, 20));
  }
}

describe("ProjectWatcher (M4A-001-A)", () => {
  let tempDir: string;

  beforeEach(() => {
    tempDir = join(tmpdir(), `velqu-watch-test-${Date.now()}-${Math.random().toString(36).slice(2)}`);
    mkdirSync(tempDir, { recursive: true });
    mkdirSync(join(tempDir, "src"), { recursive: true });

    // Seed mock project files:
    writeFileSync(
      join(tempDir, "src", "app.ts"),
      `import { router } from "./routes";\nexport const app = router();\n`,
    );
    writeFileSync(
      join(tempDir, "src", "routes.ts"),
      `export function router() { return { routes: [] }; }\n`,
    );
    writeFileSync(
      join(tempDir, "contract.lock.json"),
      JSON.stringify({ formatVersion: 1, contractHash: "test1234", routes: {} }, null, 2),
    );
    writeFileSync(
      join(tempDir, "tsconfig.json"),
      JSON.stringify({ compilerOptions: { target: "ES2022" } }, null, 2),
    );
  });

  afterEach(() => {
    if (existsSync(tempDir)) {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  it("discovers all source files and contracts statically without code execution", () => {
    const watcher = new ProjectWatcher({ project: tempDir });
    const discovered = watcher.discover();

    expect(discovered.entryFile).toContain("src/app.ts");
    expect(discovered.sourceFiles.some((f) => f.endsWith("src/app.ts"))).toBeTrue();
    expect(discovered.sourceFiles.some((f) => f.endsWith("src/routes.ts"))).toBeTrue();
    expect(discovered.contractFiles.some((f) => f.endsWith("contract.lock.json"))).toBeTrue();
    expect(discovered.configFiles.some((f) => f.endsWith("tsconfig.json"))).toBeTrue();
  });

  it("classifies file paths into source, contract, config, and ignores build artifacts", () => {
    const watcher = new ProjectWatcher({ project: tempDir });

    expect(watcher.classifyFile(join(tempDir, "src", "handler.ts"))).toBe("source");
    expect(watcher.classifyFile(join(tempDir, "src", "utils.js"))).toBe("source");
    expect(watcher.classifyFile(join(tempDir, "contract.lock.json"))).toBe("contract");
    expect(watcher.classifyFile(join(tempDir, "contract.meta.json"))).toBe("contract");
    expect(watcher.classifyFile(join(tempDir, "tsconfig.json"))).toBe("config");
    expect(watcher.classifyFile(join(tempDir, "package.json"))).toBe("config");

    // Ignored paths:
    expect(watcher.classifyFile(join(tempDir, "node_modules", "pkg", "index.ts"))).toBeNull();
    expect(watcher.classifyFile(join(tempDir, ".git", "HEAD"))).toBeNull();
    expect(watcher.classifyFile(join(tempDir, "dist", "app.qpack"))).toBeNull();
  });

  it("detects source file changes and delivers typed event with latency metric", async () => {
    const receivedEvents: WatchEvent[] = [];
    const watcher = await watchSourceAndContracts({
      project: tempDir,
      debounceMs: 30,
      onChange: (events) => {
        receivedEvents.push(...events);
      },
    });

    expect(watcher.isWatching()).toBeTrue();
    expect(watcher.watchedDirectoryCount()).toBeGreaterThanOrEqual(2); // root + src

    // Wait slightly for fs watchers to establish
    await new Promise((r) => setTimeout(r, 60));

    // Modify a source file:
    const routeFile = join(tempDir, "src", "routes.ts");
    writeFileSync(routeFile, `export function router() { return { routes: ["/updated"] }; }\n`);

    // Await debounce + dispatch:
    await waitFor(() => receivedEvents.length > 0);

    expect(receivedEvents.length).toBeGreaterThanOrEqual(1);
    const sourceEv = receivedEvents.find((e) => e.kind === "source" && e.file.endsWith("routes.ts"));
    expect(sourceEv).toBeDefined();
    expect(sourceEv!.kind).toBe("source");
    expect(sourceEv!.latencyMs).toBeGreaterThanOrEqual(0);
    expect(sourceEv!.latencyMs).toBeLessThan(1000); // fast dev feedback

    watcher.close();
    expect(watcher.isWatching()).toBeFalse();
  });

  it("detects contract.lock.json modifications as contract events", async () => {
    const receivedEvents: WatchEvent[] = [];
    const watcher = await watchSourceAndContracts({
      project: tempDir,
      debounceMs: 30,
      onChange: (events) => {
        receivedEvents.push(...events);
      },
    });

    await new Promise((r) => setTimeout(r, 60));

    const lockFile = join(tempDir, "contract.lock.json");
    writeFileSync(
      lockFile,
      JSON.stringify({ formatVersion: 1, contractHash: "updated5678", routes: {} }, null, 2),
    );

    await waitFor(() => receivedEvents.length > 0);

    const contractEv = receivedEvents.find((e) => e.kind === "contract" && e.file.endsWith("contract.lock.json"));
    expect(contractEv).toBeDefined();
    expect(contractEv!.kind).toBe("contract");

    watcher.close();
  });

  it("coalesces rapid burst modifications on the same file into a single debounced event", async () => {
    let callCount = 0;
    const watcher = await watchSourceAndContracts({
      project: tempDir,
      debounceMs: 60,
      onChange: () => {
        callCount++;
      },
    });

    await new Promise((r) => setTimeout(r, 60));

    const routeFile = join(tempDir, "src", "routes.ts");
    // Rapid burst of 5 edits:
    for (let i = 0; i < 5; i++) {
      writeFileSync(routeFile, `export function router() { return { v: ${i} }; }\n`);
      await new Promise((r) => setTimeout(r, 5));
    }

    // Wait for the single debounced event to fire:
    await waitFor(() => callCount > 0);
    await new Promise((r) => setTimeout(r, 80));

    expect(callCount).toBe(1);

    watcher.close();
  });

  it("detects file deletion and reports delete action", async () => {
    const receivedEvents: WatchEvent[] = [];
    const watcher = await watchSourceAndContracts({
      project: tempDir,
      debounceMs: 30,
      onChange: (events) => {
        receivedEvents.push(...events);
      },
    });

    await new Promise((r) => setTimeout(r, 60));

    const extraFile = join(tempDir, "src", "extra.ts");
    writeFileSync(extraFile, `export const extra = 42;\n`);
    await new Promise((r) => setTimeout(r, 60));

    unlinkSync(extraFile);
    await waitFor(() => receivedEvents.some((e) => e.file.endsWith("extra.ts") && e.action === "delete"));

    const delEv = receivedEvents.find((e) => e.file.endsWith("extra.ts") && e.action === "delete");
    expect(delEv).toBeDefined();

    watcher.close();
  });

  it("discovers proof fixture files and project structure accurately", () => {
    const watcher = new ProjectWatcher({ project: "examples/proof" });
    const discovered = watcher.discover();

    expect(discovered.sourceFiles.length).toBeGreaterThanOrEqual(1);
    expect(discovered.entryFile).toContain("examples/proof");
    expect(discovered.sourceFiles.some((f) => f.includes("examples/proof"))).toBeTrue();
  });
});
