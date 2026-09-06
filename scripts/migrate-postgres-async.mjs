#!/usr/bin/env node
/**
 * BWASM-C-002 — migration codemod for the async Postgres contract.
 *
 * `runtime:postgres` v1's `sql()` always returns a Promise (BWASM-C-002).
 * Code written against the previously (mis-typed) synchronous surface
 * needs `await` at every call site whose result value is consumed. This
 * tool FINDS those sites and can REWRITE the safe, mechanical patterns:
 *
 *   const r = X.sql(...)          →  const r = await X.sql(...)
 *   return X.sql(...).rows        →  return (await X.sql(...)).rows
 *
 * It never touches call sites already preceded by `await`, `return`
 * (full-promise returns are legal), or inside non-async functions — the
 * latter are reported as manual-fix items (the enclosing function must
 * become async; the compiler's generated wrapper already awaits handler
 * results, so marking the handler async is always safe).
 *
 * Usage:
 *   bun scripts/migrate-postgres-async.mjs <project-dir>            # report
 *   bun scripts/migrate-postgres-async.mjs <project-dir> --write    # rewrite
 *
 * Exit code 0 = no remaining sync-looking sites; 1 = sites remain (or
 * were found without --write); 2 = usage error.
 */
import { readdirSync, readFileSync, writeFileSync, statSync } from "node:fs";
import { join, extname } from "node:path";

const args = process.argv.slice(2);
const write = args.includes("--write");
const root = args.find((a) => !a.startsWith("--"));
if (!root) {
  console.error("usage: bun scripts/migrate-postgres-async.mjs <project-dir> [--write]");
  process.exit(2);
}

function* walk(dir) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) {
      if (name === "node_modules" || name === "dist" || name.startsWith(".")) continue;
      yield* walk(p);
    } else if ([".ts", ".mts", ".cts", ".js", ".mjs"].includes(extname(p))) {
      yield p;
    }
  }
}

const lineOf = (src, idx) => src.slice(0, idx).split("\n").length;
const sites = [];
const rewritten = [];
const manual = [];

// Pass 1: collect sites per file.
const editsByFile = new Map(); // file -> [{ insertAt, line, text }]
for (const file of walk(root)) {
  const src = readFileSync(file, "utf8");
  const re = /\.sql\s*\(/g;
  let m;
  while ((m = re.exec(src)) !== null) {
    const callStart = m.index;
    const line = lineOf(src, callStart);
    const lineStart = src.lastIndexOf("\n", callStart) + 1;
    const lineText = src.slice(lineStart, src.indexOf("\n", callStart) ?? undefined);

    // already awaited (directly or via `return await`)
    if (/(\bawait\s+[\w.$\])]*\.sql\s*\()|(\bawait\s+sql\s*\()/.test(lineText)) continue;
    // Sync-looking = the statement's value is consumed: assignment,
    // property access on the result, or an argument position. A bare
    // fire-and-forget `X.sql(...)` statement is a legal Promise.
    const before = lineText.slice(0, callStart - lineStart);
    const assigns = /(?:const|let|var)\s+[\w{}\[\],\s:<>]+=\s*[\w$.()[\]]*$/.test(before);
    const returnsMember = /return\s+[\w.$()\[\]]*\.sql\s*\(/.test(lineText) &&
      /\.rows|\.affectedRows/.test(lineText);
    const consumedArg = /[(,]\s*[\w.$()\[\]]*\.sql\s*\(/.test(lineText);
    if (!assigns && !returnsMember && !consumedArg) continue;

    // enclosing function async?
    const arrowCtx = src.slice(Math.max(0, callStart - 400), callStart);
    const asyncAhead = /async\s+(?:function|\()|async\s+[\w$]+\s*=>|=>\s*{?\s*$/.test(arrowCtx);
    sites.push({ file, line, text: lineText.trim() });
    if (!asyncAhead) {
      manual.push({ file, line, text: lineText.trim(), reason: "enclosing function is not async — mark it async first" });
      continue;
    }
    // insert `await ` before the receiver expression
    let insertAt = callStart;
    const recv = /([\w$][\w$.[\]]*\.\s*)+sql\s*\($/.exec(src.slice(lineStart, callStart + m[0].length));
    if (recv) insertAt = lineStart + recv.index;
    if (!editsByFile.has(file)) editsByFile.set(file, []);
    editsByFile.get(file).push({ insertAt, line, text: lineText.trim() });
  }
}

// Pass 2: apply edits in reverse offset order (earlier insertions keep
// their offsets; one write per file — no lost edits).
for (const [file, edits] of editsByFile) {
  if (!write) break;
  let src = readFileSync(file, "utf8");
  for (const edit of [...edits].sort((a, b) => b.insertAt - a.insertAt)) {
    src = src.slice(0, edit.insertAt) + "await " + src.slice(edit.insertAt);
    rewritten.push({ file, line: edit.line });
  }
  writeFileSync(file, src);
}
const manualFixes = manual;

console.log(JSON.stringify({
  schemaVersion: 1,
  tool: "migrate-postgres-async",
  contract: "BWASM-C-002: runtime:postgres v1 sql() is Promise-returning",
  scannedRoot: root,
  write,
  syncLookingSites: sites,
  rewritten,
  manualFixes,
  remaining: sites.length - rewritten.length,
}, null, 2));

// Exit 0 only when nothing sync-looking remains (or everything was rewritten).
process.exit(sites.length > 0 && rewritten.length < sites.length ? 1 : 0);
