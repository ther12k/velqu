/**
 * @velqu/cli — the `velqu` command: build / inspect / contract diff / dev-notes.
 * (Dev server is P1; M2 provides build + inspection + diff per DX-006.)
 */
import { build, contractDiff, CompileError } from "@velqu/compiler";
import { assessPackMigrate } from "./pack-migrate";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";

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
          process.exit(1);
        }
        throw e;
      }
      break;
    }
    case "inspect": {
      const what = rest.find((a) => !a.startsWith("--"));
      const dist = args.get("dist") ?? distFor(project);
      const manifestPath = join(dist, "route-manifest.json");
      if (!existsSync(manifestPath)) {
        console.error(`no route manifest at ${manifestPath} — run 'q build' first`);
        process.exit(1);
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
          process.exit(1);
        }
        console.log(JSON.stringify(r, null, 2));
      } else if (what === "capabilities") {
        console.log("declared:", caps.declared.join(", ") || "(none)");
        for (const [route, list] of Object.entries(caps.perRoute)) {
          if ((list as string[]).length) console.log(`  ${route}: ${(list as string[]).join(", ")}`);
        }
        console.log("native ops:", JSON.stringify(caps.nativeOps));
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
        console.error("usage: velqu inspect <routes|route <id>|capabilities|fallbacks>");
        process.exit(1);
      }
      break;
    }
    case "pack": {
      // M26-008-B: rebuild/migration guidance for legacy packs.
      const sub = rest[0];
      if (sub !== "migrate") {
        console.error("usage: velqu pack migrate <app.qpack>");
        process.exit(1);
      }
      const file = rest[1];
      if (!file || !existsSync(file)) {
        console.error(`pack not found: ${file ?? "(none given)"}`);
        process.exit(1);
      }
      const report = assessPackMigrate(() => readFileSync(file, "utf8"));
      if (report.status === "legacy-supported") {
        console.log(`formatVersion ${report.formatVersion} (legacy JSON adapter, supported through M2.6):`);
        for (const line of report.guidance) console.log(`  - ${line}`);
        break;
      }
      console.error(report.message);
      process.exit(1);
    }
    case "contract": {
      const sub = rest[0];
      if (sub !== "diff") {
        console.error("usage: velqu contract diff --against <contract.lock.json>");
        process.exit(1);
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
      if (breaking) process.exit(2);
      break;
    }
    default:
      console.log(`q — Velqu build & inspection CLI (M2 scope)
usage:
  velqu build --project <dir|entry> [--profile serverless] [--out <dir>]
  velqu inspect routes|route <id>|capabilities|fallbacks [--dist <dir>]
  velqu contract diff --against <contract.lock.json>
  velqu pack migrate <app.qpack>`);
      process.exit(cmd ? 1 : 0);
  }
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
