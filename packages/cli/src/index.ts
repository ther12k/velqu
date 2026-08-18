/**
 * @q/cli — the `q` command: build / inspect / contract diff / dev-notes.
 * (Dev server is P1; M2 provides build + inspection + diff per DX-006.)
 */
import { build, contractDiff, CompileError } from "@q/compiler";
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
        console.log(`q build [${profile}]: ${r.routes} routes → ${r.outDir} in ${r.buildMs}ms`);
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
          console.log(
            `${r.method.padEnd(6)} ${r.path.padEnd(22)} ${r.id.padEnd(16)} stage=${r.nativeStage} policy=${r.policy ?? "—"} caps=[${r.capabilities}]`,
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
        console.log("fallbacks:", report.strategies.fallbacks.length ? report.strategies.fallbacks : "none");
        for (const n of report.strategies.notes) console.log(`  ${n}`);
      } else {
        console.error("usage: q inspect <routes|route <id>|capabilities|fallbacks>");
        process.exit(1);
      }
      break;
    }
    case "contract": {
      const sub = rest[0];
      if (sub !== "diff") {
        console.error("usage: q contract diff --against <contract.lock.json>");
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
  q build --project <dir|entry> [--profile serverless] [--out <dir>]
  q inspect routes|route <id>|capabilities|fallbacks [--dist <dir>]
  q contract diff --against <contract.lock.json>`);
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
