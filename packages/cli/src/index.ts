/**
 * @velqu/cli — the `velqu` command: build / inspect / contract diff / dev-notes.
 * (Dev server is P1; M2 provides build + inspection + diff per DX-006.)
 */
import {
  build,
  contractDiff,
  CompileError,
  watchSourceAndContracts,
  extractApp,
  evaluateAppStrategies,
  assertPinnedToolchain,
  PINNED_TOOLCHAIN,
} from "@velqu/compiler";
import { assessPackMigrate } from "./pack-migrate";
import { inspectCapabilities } from "./capability-inspect";
import { inspectPack } from "./pack-inspect";
import { ExitCode, type ExitCodeValue } from "./exit-codes";
import {
  DevServer,
  formatCompileError,
  formatRuntimeError,
  type DevServerOptions,
  type ReloadResult,
  type WorkerInstance,
} from "./dev-server";
import { readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import * as ts from "typescript";

export {
  DevServer,
  ExitCode,
  formatCompileError,
  formatRuntimeError,
  inspectPack,
  type DevServerOptions,
  type ExitCodeValue,
  type ReloadResult,
  type WorkerInstance,
};

async function main() {
  const [cmd, ...rest] = process.argv.slice(2);
  const args = new Map<string, string>();
  for (let i = 0; i < rest.length; i++) {
    if (rest[i].startsWith("--")) args.set(rest[i].slice(2), rest[i + 1] ?? "");
  }
  const project = args.get("project") ?? "examples/proof";

  switch (cmd) {
    case "build": {
      const profile = args.get("profile") ?? "serverless";
      try {
        const r = await build({
          project,
          outDir: args.get("out") ?? undefined,
          updateLock: args.has("update-lock"),
        });
        console.log(`velqu build [${profile}]: ${r.routes} routes → ${r.outDir} in ${r.buildMs}ms`);
        if (r.lockPreserved) console.log("  contract.lock.json: PRESERVED (diff against it; --update-lock to refresh)");
        for (const [k, v] of Object.entries(r.artifactBytes)) console.log(`  ${k}  ${v}B`);
      } catch (e) {
        if (e instanceof CompileError) {
          console.error(e.toString());
          process.exit(ExitCode.GENERAL_ERROR);
        }
        throw e;
      }
      break;
    }
    case "inspect": {
      const what = rest.find((a) => !a.startsWith("--"));

      if (what === "diagnostics") {
        try {
          assertPinnedToolchain({ bun: Bun.version!, typescript: ts.version });
          const entry = resolveEntryPath(project);
          const app = extractApp(entry);
          const { report: strategyReport } = evaluateAppStrategies(app.routes);
          console.log(`diagnostics for ${app.appId} (${entry}):`);
          console.log(`  routes: ${app.routes.length}`);
          console.log(`  policies: ${app.policies.length}`);
          console.log(`  modules: ${app.modules.length}`);
          console.log(`  strategy fallbacks: ${strategyReport.fallbacks.length}`);
          for (const f of strategyReport.fallbacks) {
            console.log(`    - ${f.route} [${f.location}]: ${f.reason} (${f.description})`);
          }
          console.log("  verdict: OK (static contract extraction clean)");
        } catch (e) {
          if (e instanceof CompileError) {
            console.error(e.toString());
            process.exit(ExitCode.GENERAL_ERROR);
          }
          throw e;
        }
        break;
      }

      const dist = args.get("dist") ?? distFor(project);
      const manifestPath = join(dist, "route-manifest.json");
      if (!existsSync(manifestPath)) {
        console.error(`no route manifest at ${manifestPath} — run 'q build' first`);
        process.exit(ExitCode.GENERAL_ERROR);
      }
      const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
      const caps = JSON.parse(readFileSync(join(dist, "capability-manifest.json"), "utf8"));
      if (what === "routes") {
        for (const r of manifest) {
          const val = r.validationStrategy ?? "native";
          const resp = r.responseStrategy ?? "native";
          // M25-007-D: codec choice and bridge-crossing model per route
          const valReason = r.validationFallbackReason ? `(${r.validationFallbackReason})` : "";
          const respReason = r.responseFallbackReason ? `(${r.responseFallbackReason})` : "";
          console.log(
            `${r.method.padEnd(6)} ${r.path.padEnd(22)} ${r.id.padEnd(16)} val=${val}${valReason} resp=${resp}${respReason} codec=${r.validationCodec}/${r.responseCodec} bridge=${r.bridge ?? "single-prevalidated"} stage=${r.nativeStage} policy=${r.policy ?? "—"} caps=[${r.capabilities}]`,
          );
        }
        console.log(`— ${manifest.length} routes`);
      } else if (what === "route") {
        const id = rest[1];
        const r = manifest.find((x: { id: string }) => x.id === id);
        if (!r) {
          console.error(`route '${id}' not found`);
          process.exit(ExitCode.GENERAL_ERROR);
        }
        console.log(JSON.stringify(r, null, 2));
      } else if (what === "capabilities") {
        // M27-002-D: report the pack's hash-verified linked inventory too
        const packPath = join(dist, "app.qpack");
        let pack: Record<string, unknown> = {};
        if (existsSync(packPath)) pack = JSON.parse(readFileSync(packPath, "utf8"));
        for (const line of inspectCapabilities({
          declared: caps.declared,
          perRoute: caps.perRoute,
          nativeOps: caps.nativeOps,
          intrinsicRequirement: caps.intrinsicRequirement,
          reductionImpact: caps.reductionImpact,
          pack,
        })) {
          console.log(line);
        }
      } else if (what === "fallbacks") {
        const report = JSON.parse(readFileSync(join(dist, "build-report.json"), "utf8"));
        const strategies = report.strategies || {};
        const fallbacks = strategies.fallbacks || [];
        if (fallbacks.length === 0) {
          console.log("fallbacks: none (0 routes using fallback; all routes use native strategy)");
          console.log("strategy distribution:");
          console.log(`  native validation: ${manifest.length} routes (100%)`);
          console.log(`  native response: ${manifest.length} routes (100%)`);
          console.log("measured fallback cost threshold: ~20–50 µs bridge crossing + 10–18 KB alloc per fallback");
        } else {
          console.log(`fallbacks: ${fallbacks.length} active`);
          for (const f of fallbacks) {
            console.log(
              `  ${f.route} [${f.location}]: strategy=${f.strategy} reason=${f.reason} overhead=+${f.estimatedOverheadUs}µs (+${f.estimatedAllocBytes}B) — ${f.description}`,
            );
          }
        }
        for (const n of strategies.notes || []) console.log(`  ${n}`);
      } else {
        console.error("usage: velqu inspect <routes|route <id>|capabilities|fallbacks|diagnostics>");
        process.exit(ExitCode.GENERAL_ERROR);
      }
      break;
    }
    case "pack": {
      const sub = rest[0];
      if (sub === "inspect") {
        const file = rest[1] ?? join(distFor(project), "app.qpack");
        const report = inspectPack(file);
        if (report.status === "error") {
          console.error(`pack inspection failed: ${report.error}`);
          process.exit(ExitCode.GENERAL_ERROR);
        }
        console.log(`pack: ${report.file}`);
        console.log(`  appId: ${report.appId}`);
        console.log(`  formatVersion: ${report.formatVersion}`);
        console.log(`  contractHash: ${report.contractHash}`);
        console.log(`  engine: ${report.engine?.name} ${report.engine?.version} (runtimeAbi=${report.engine?.runtimeAbi})`);
        console.log(`  routes: ${report.routesCount}`);
        console.log(`  schemas: ${report.schemasCount}`);
        console.log(`  policies: ${report.policiesCount}`);
        console.log(`  capabilities: [${(report.capabilities ?? []).join(", ")}]`);
        console.log(`  bundleSha256: ${report.bundleSha256}`);
        break;
      } else if (sub === "migrate") {
        const file = rest[1];
        if (!file || !existsSync(file)) {
          console.error(`pack not found: ${file ?? "(none given)"}`);
          process.exit(ExitCode.GENERAL_ERROR);
        }
        const report = assessPackMigrate(() => readFileSync(file, "utf8"));
        if (report.status === "legacy-supported") {
          console.log(`formatVersion ${report.formatVersion} (legacy JSON adapter, supported through M2.6):`);
          for (const line of report.guidance) console.log(`  - ${line}`);
          break;
        }
        console.error(report.message);
        process.exit(ExitCode.UNSUPPORTED_FORMAT);
      } else {
        console.error("usage: velqu pack <inspect [file]|migrate <file>>");
        process.exit(ExitCode.GENERAL_ERROR);
      }
      break;
    }
    case "test": {
      const filter = rest.filter((a) => !a.startsWith("--")).join(" ");
      const testCmd = ["bun", "test"];
      if (filter) testCmd.push(filter);
      const proc = Bun.spawn(testCmd, {
        stdout: "inherit",
        stderr: "inherit",
        env: process.env,
      });
      const exitCode = await proc.exited;
      process.exit(exitCode);
      break;
    }
    case "check": {
      try {
        assertPinnedToolchain({ bun: Bun.version!, typescript: ts.version });
        const entry = resolveEntryPath(project);
        const app = extractApp(entry);
        console.log(`velqu check: ${app.routes.length} routes in ${project} — clean`);
      } catch (e) {
        if (e instanceof CompileError) {
          console.error(e.toString());
          process.exit(ExitCode.GENERAL_ERROR);
        }
        throw e;
      }
      break;
    }
    case "contract": {
      const sub = rest[0];
      if (sub !== "diff") {
        console.error("usage: velqu contract diff --against <contract.lock.json>");
        process.exit(ExitCode.GENERAL_ERROR);
      }
      const dist = args.get("dist") ?? distFor(project);
      const against = args.get("against") ?? join(dist, "contract.lock.json");
      const entries = contractDiff(dist, against);
      if (!entries.length) {
        console.log("contract diff: no changes");
        break;
      }
      let breaking = 0;
      for (const e of entries) {
        console.log(`${e.kind.padEnd(16)} ${e.routeId}: ${e.change}`);
        if (e.kind === "breaking") breaking++;
      }
      if (breaking) process.exit(ExitCode.BREAKING_CONTRACT);
      break;
    }
    case "dev": {
      const port = args.has("port") ? parseInt(args.get("port")!, 10) : 3000;
      const debounceMs = args.has("debounce-ms") ? parseInt(args.get("debounce-ms")!, 10) : 50;
      const profile = args.get("profile") ?? "serverless";
      const server = new DevServer({
        project,
        port,
        debounceMs,
        serviceProfile: profile,
        onLog: (msg) => console.log(msg),
        onReload: (r) => {
          if (r.success) {
            console.log(`[dev:reload] switched to worker gen ${r.generation} in ${r.totalMs}ms (compile ${r.compileMs}ms, init ${r.workerInitMs}ms)`);
          } else {
            console.error(`[dev:reload] reload failed: ${r.error}`);
          }
        },
      });
      await server.start(true);
      break;
    }
    case "watch": {
      const debounceMs = args.has("debounce-ms") ? parseInt(args.get("debounce-ms")!, 10) : 50;
      console.log(`velqu watch: watching ${project} (debounce ${debounceMs}ms)...`);
      const watcher = await watchSourceAndContracts({
        project,
        debounceMs,
        onChange: (events) => {
          for (const ev of events) {
            console.log(`[watch:${ev.kind}] ${ev.action} ${ev.file} (+${ev.latencyMs}ms)`);
          }
        },
        onError: (err) => {
          console.error(`[watch:error] ${err.message}`);
        },
      });
      const discovered = watcher.discover();
      console.log(`  watched sources: ${discovered.sourceFiles.length} files`);
      console.log(`  watched contracts: ${discovered.contractFiles.length} files`);
      console.log(`  watched configs: ${discovered.configFiles.length} files`);
      console.log(`  directories: ${watcher.watchedDirectoryCount()}`);
      break;
    }
    case "help":
    case "--help":
    case "-h":
    default: {
      const isHelp = !cmd || cmd === "help" || cmd === "--help" || cmd === "-h";
      if (!isHelp) {
        console.error(`unknown command: '${cmd}'\n`);
      }
      console.log(`velqu — Unified Velqu CLI
usage:
  velqu dev [--project <dir|entry>] [--port 3000] [--debounce-ms 50]
  velqu build [--project <dir|entry>] [--profile serverless] [--out <dir>]
  velqu inspect routes|route <id>|capabilities|fallbacks|diagnostics [--dist <dir>]
  velqu contract diff --against <contract.lock.json>
  velqu test [filter]
  velqu check [--project <dir|entry>]
  velqu pack inspect <file> | migrate <file>`);
      process.exit(isHelp ? ExitCode.SUCCESS : ExitCode.GENERAL_ERROR);
    }
  }
}

function resolveEntryPath(project: string): string {
  const { statSync, existsSync } = require("node:fs") as typeof import("node:fs");
  const { join } = require("node:path") as typeof import("node:path");
  let st;
  try {
    st = statSync(project);
  } catch {
    throw new Error(`project path not found: ${project}`);
  }
  if (st.isDirectory()) {
    for (const c of ["src/app.ts", "app.ts", "src/index.ts"]) {
      const p = join(project, c);
      if (existsSync(p)) return p;
    }
    throw new Error(`no app entry found in ${project} (looked for src/app.ts, app.ts, src/index.ts)`);
  }
  return project;
}

function distFor(project: string): string {
  // mirrors compiler default: <entry-dir>/../dist
  const { statSync } = require("node:fs") as typeof import("node:fs");
  const { join, dirname } = require("node:path") as typeof import("node:path");
  try {
    const st = statSync(project);
    if (st.isDirectory()) return join(project, "dist");
  } catch {}
  return join(dirname(project), "..", "dist");
}

await main();
