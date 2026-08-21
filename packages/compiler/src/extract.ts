/**
 * @velqu/compiler — static contract extraction.
 *
 * The compiler NEVER executes application code (COMP-002): it reads the
 * TypeScript AST of route/policy/service declarations, extracts literal
 * metadata, and bundles with Bun.build (which also never evaluates the app).
 * Anything dynamic fails with a source-located diagnostic (PR-004, COMP-006).
 */

import * as ts from "typescript";
import { createHash } from "node:crypto";

export class CompileError extends Error {
  constructor(
    message: string,
    readonly location?: { file: string; line: number; column: number },
    readonly hint?: string,
  ) {
    super(message);
  }
  toString(): string {
    const loc = this.location ? ` (${this.location.file}:${this.location.line}:${this.location.column})` : "";
    const hint = this.hint ? `\n  hint: ${this.hint}` : "";
    return `error${loc}: ${this.message}${hint}`;
  }
}

export interface RouteInfo {
  id: string;
  method: string;
  path: string;
  moduleId: string;
  /** variable name holding the route() result (bundle adapter reference) */
  bindingName: string;
  sourceFile: string;
  policyId: string | null;
  paramsIr: Record<string, unknown> | null;
  queryIr: Record<string, unknown> | null;
  bodyIr: Record<string, unknown> | null;
  responses: Record<string, { strategy: "native" | "js"; problem?: string; ir?: Record<string, unknown> }>;
  /** statically evaluable handler → native liveness (RUN-009) */
  liveness: { status: number; contentType: string; body: string } | null;
  capabilities: string[];
}

export interface PolicyInfo {
  id: string;
  bindingName: string;
  sourceFile: string;
  declaredStatuses: number[];
}

export interface ExtractedApp {
  appId: string;
  entryFile: string;
  routes: RouteInfo[];
  policies: PolicyInfo[];
  modules: string[];
}

// ---------------------------------------------------------------- helpers

const loc = (node: ts.Node, file: string) => ({
  file,
  line: file === "?" ? 1 : (ts.getLineAndCharacterOfPosition as (sf: ts.SourceFile, pos: number) => { line: number; character: number })(
    ts.createSourceFile(file, "", ts.ScriptTarget.ES2022, true),
    0,
  ).line + (node.getSourceFile().getLineAndCharacterOfPosition(node.getStart()).line + 1) - 1,
  column: node.getSourceFile().getLineAndCharacterOfPosition(node.getStart()).character + 1,
});

function nodeLoc(node: ts.Node, file: string): { file: string; line: number; column: number } {
  const sf = node.getSourceFile();
  if (!sf || sf.fileName !== file) {
    // node from a synthetic file — fall back
    return { file, line: 1, column: 1 };
  }
  const { line, character } = sf.getLineAndCharacterOfPosition(node.getStart());
  return { file, line: line + 1, column: character + 1 };
}

const isCallTo = (node: ts.Node, names: string[]): node is ts.CallExpression =>
  ts.isCallExpression(node) &&
  !!node.expression.getText &&
  names.includes(node.expression.getText().replace(/^.*\./, ""));

function literalValue(node: ts.Node, file: string): unknown {
  if (ts.isStringLiteralLike(node)) return node.text;
  if (ts.isNumericLiteral(node)) return Number(node.text);
  if (node.kind === ts.SyntaxKind.TrueKeyword) return true;
  if (node.kind === ts.SyntaxKind.FalseKeyword) return false;
  if (ts.isPrefixUnaryExpression(node) && node.operator === ts.SyntaxKind.MinusToken && ts.isNumericLiteral(node.operand)) {
    return -Number(node.operand.text);
  }
  throw new CompileError(
    `expected a literal value, found '${node.getText()}'`,
    nodeLoc(node, file),
    "route/policy metadata must be statically evaluable",
  );
}

// ---------------------------------------------------------------- schema AST → IR

type Ir = Record<string, unknown>;

function propKey(p: ts.PropertyAssignment | ts.ShorthandPropertyAssignment | ts.MethodDeclaration, file: string): string {
  const name = p.name;
  if (ts.isIdentifier(name) || ts.isStringLiteral(name) || ts.isNumericLiteral(name)) {
    return name.text;
  }
  throw new CompileError("unsupported property name in schema", nodeLoc(p, file));
}

/** Option bag as ordered [key, initializer] pairs so each key can get its own validation. */
function propsFrom(arg: ts.Expression | undefined, node: ts.Node, file: string): [string, ts.Expression][] {
  if (!arg) return [];
  if (!ts.isObjectLiteralExpression(arg)) {
    throw new CompileError("schema options must be an object literal", nodeLoc(arg, file));
  }
  const out: [string, ts.Expression][] = [];
  for (const p of arg.properties) {
    if (!ts.isPropertyAssignment(p)) {
      throw new CompileError("schema options must use literal properties", nodeLoc(p, file));
    }
    out.push([propKey(p, file), p.initializer]);
  }
  return out;
}

function schemaFromNode(node: ts.Node, file: string): Ir {
  if (!ts.isCallExpression(node)) {
    throw new CompileError(
      `schema must be an s.* call, found '${node.getText().slice(0, 40)}'`,
      nodeLoc(node, file),
      "wrap schemas in s.object/s.string/s.integer/…",
    );
  }
  const callee = node.expression.getText().replace(/^.*\./, ""); // s.string → string
  const args = node.arguments;
  const optsFrom = (arg?: ts.Expression): Ir => {
    if (!arg) return {};
    if (!ts.isObjectLiteralExpression(arg)) {
      throw new CompileError("schema options must be an object literal", nodeLoc(arg, file));
    }
    const out: Ir = {};
    for (const p of arg.properties) {
      if (!ts.isPropertyAssignment(p)) {
        throw new CompileError("schema options must use literal properties", nodeLoc(p, file));
      }
      const k = propKey(p, file);
      if (k === "format") {
        out[k] = literalValue(p.initializer, file);
        continue;
      }
      if (k === "default") {
        out[k] = literalValue(p.initializer, file);
        continue;
      }
      out[k] = literalValue(p.initializer, file);
    }
    return out;
  };

  switch (callee) {
    case "string": {
      const opts = optsFrom(args[0]);
      return strip({ kind: "string", ...opts });
    }
    case "integer":
    case "number": {
      const opts = optsFrom(args[0]);
      return strip({ kind: callee, ...opts });
    }
    case "boolean":
      return { kind: "boolean" };
    case "literal":
      return { kind: "literal", value: literalValue(args[0], file) };
    case "enum": {
      if (!ts.isArrayLiteralExpression(args[0])) {
        throw new CompileError("s.enum expects an array literal", nodeLoc(args[0], file));
      }
      return { kind: "enum", values: args[0].elements.map((e) => literalValue(e, file)) };
    }
    case "optional": {
      const inner = schemaFromNode(args[0], file);
      const opts = optsFrom(args[1]);
      return strip({ kind: "optional", inner, ...opts });
    }
    case "nullable": {
      return { kind: "nullable", inner: schemaFromNode(args[0], file) };
    }
    case "array": {
      const inner = schemaFromNode(args[0], file);
      const opts = optsFrom(args[1]);
      return strip({ kind: "array", items: inner, ...opts });
    }
    case "object": {
      if (!args[0] || !ts.isObjectLiteralExpression(args[0])) {
        throw new CompileError("s.object expects an object literal of schemas", nodeLoc(args[0] ?? node, file));
      }
      const properties: Record<string, Ir> = {};
      const required: string[] = [];
      for (const p of args[0].properties) {
        if (!ts.isPropertyAssignment(p)) {
          throw new CompileError("s.object properties must be literal", nodeLoc(p, file));
        }
        const key = propKey(p, file);
        const ir = schemaFromNode(p.initializer, file);
        properties[key] = ir;
        if (ir.kind !== "optional") required.push(key);
      }
      // keep insertion order (properties is a plain map in JSON)
      return { kind: "object", properties, required };
    }
    case "transform": {
      if (args.length !== 3 || !ts.isStringLiteral(args[2])) throw new CompileError("s.transform requires input, output, and literal name", nodeLoc(node, file));
      const name = args[2].text;
      if (!/^[A-Za-z0-9_.:-]{1,64}$/.test(name)) {
        throw new CompileError("s.transform name must match [A-Za-z0-9_.:-]{1,64}", nodeLoc(args[2], file));
      }
      return { kind: "transform", input: schemaFromNode(args[0], file), output: schemaFromNode(args[1], file), name };
    }
    case "file": {
      const props = propsFrom(args[0], node, file);
      let contentType: string | undefined;
      let maxBytes: number | undefined;
      for (const [k, init] of props) {
        if (k === "contentType") {
          const v = literalValue(init, file);
          if (typeof v !== "string" || v.length === 0 || v.length > 128) throw new CompileError("s.file contentType must be 1..128 characters", nodeLoc(init, file));
          contentType = v;
        } else if (k === "maxBytes") {
          const v = literalValue(init, file);
          if (!Number.isInteger(v) || (v as number) < 1 || (v as number) > 16 * 1024 * 1024) {
            throw new CompileError("s.file maxBytes must be an integer in [1, 16777216]", nodeLoc(init, file));
          }
          maxBytes = v as number;
        } else {
          throw new CompileError(`s.file has no option '${k}'`, nodeLoc(init, file));
        }
      }
      if (maxBytes === undefined) throw new CompileError("s.file requires maxBytes", nodeLoc(node, file));
      // field order matches the Rust SchemaIr declaration so canonical hashes agree
      return { kind: "file", ...(contentType !== undefined ? { contentType } : {}), maxBytes };
    }
    case "problem": {
      const props = propsFrom(args[0], node, file);
      let typeUri: string | undefined;
      let title: string | undefined;
      let status: number | undefined;
      let detail: Ir | undefined;
      for (const [k, init] of props) {
        if (k === "typeUri") {
          const v = literalValue(init, file);
          if (typeof v !== "string" || v.length > 2048) throw new CompileError("s.problem typeUri must be at most 2048 characters", nodeLoc(init, file));
          typeUri = v;
        } else if (k === "title") {
          const v = literalValue(init, file);
          if (typeof v !== "string" || v.length === 0 || v.length > 128) throw new CompileError("s.problem title must be 1..128 characters", nodeLoc(init, file));
          title = v;
        } else if (k === "status") {
          const v = literalValue(init, file);
          if (!Number.isInteger(v) || (v as number) < 400 || (v as number) > 599) {
            throw new CompileError("s.problem status must be an integer in [400, 599]", nodeLoc(init, file));
          }
          status = v as number;
        } else if (k === "detail") {
          detail = schemaFromNode(init, file);
        } else {
          throw new CompileError(`s.problem has no option '${k}'`, nodeLoc(init, file));
        }
      }
      if (title === undefined || status === undefined) {
        throw new CompileError("s.problem requires title and status", nodeLoc(node, file));
      }
      // field order matches the Rust SchemaIr declaration so canonical hashes agree
      return { kind: "problem", ...(typeUri !== undefined ? { typeUri } : {}), title, status, ...(detail !== undefined ? { detail } : {}) };
    }
    case "union": {
      if (!ts.isArrayLiteralExpression(args[0])) {
        throw new CompileError("s.union expects an array literal", nodeLoc(args[0], file));
      }
      if (args[0].elements.length > 4) {
        throw new CompileError("s.union supports at most 4 members", nodeLoc(args[0], file));
      }
      return { kind: "union", members: args[0].elements.map((e) => schemaFromNode(e, file)) };
    }
    case "fallback": {
      if (args.length < 1 || args.length > 2 || !ts.isStringLiteral(args[0])) {
        throw new CompileError("s.fallback requires a literal reason and an optional inner schema", nodeLoc(node, file));
      }
      const reason = args[0].text;
      if (!["unsupported-transform", "unrepresentable", "measured", "explicit"].includes(reason)) {
        throw new CompileError(
          `s.fallback reason '${reason}' is not in the closed vocabulary`,
          nodeLoc(args[0], file),
          "reasons: unsupported-transform | unrepresentable | measured | explicit (docs/specs/unsupported-transformations.md)",
        );
      }
      // field order matches the Rust SchemaIr declaration so canonical hashes agree
      return { kind: "fallback", reason, ...(args[1] ? { inner: schemaFromNode(args[1], file) } : {}) };
    }
    default:
      throw new CompileError(
        `unsupported schema builder '${callee}'`,
        nodeLoc(node, file),
        "Schema IR v2 subset only — unsupported classes and their failure modes: docs/specs/unsupported-transformations.md",
      );
  }
}

function strip(o: Ir): Ir {
  const out: Ir = {};
  for (const [k, v] of Object.entries(o)) if (v !== undefined) out[k] = v;
  return out;
}

// ---------------------------------------------------------------- static handler eval

/** Detect a handler that references no ctx and returns a literal object/array → native liveness. */
function tryStaticHandler(handleProp: ts.PropertyAssignment | ts.MethodDeclaration, file: string): string | null {
  let body: ts.Node | null = null;
  if (ts.isPropertyAssignment(handleProp)) {
    const init = handleProp.initializer;
    if (ts.isArrowFunction(init) || ts.isFunctionExpression(init)) body = init.body;
  } else if (ts.isMethodDeclaration(handleProp) && handleProp.body) {
    if (handleProp.parameters.length === 0) body = handleProp.body;
  }
  if (!body) return null;
  // no parameters allowed (no ctx usage possible)
  const fn = ts.isPropertyAssignment(handleProp) ? handleProp.initializer : handleProp;
  const params = ts.isArrowFunction(fn) || ts.isFunctionExpression(fn) || ts.isFunctionDeclaration(fn) ? fn.parameters : [];
  if (params.length > 0) return null;
  const unwrap = (n: ts.Node): ts.Node =>
    ts.isParenthesizedExpression(n) ? unwrap(n.expression) : n;
  const rawExpr = ts.isBlock(body)
    ? body.statements.length === 1 && ts.isReturnStatement(body.statements[0])
      ? body.statements[0].expression
      : null
    : body;
  if (!rawExpr) return null;
  const expr = unwrap(rawExpr);
  if (!expr) return null;
  // Only OBJECT literals become native liveness (JSON semantics). String/
  // number returns have unknown content-type → served through the engine.
  if (!ts.isObjectLiteralExpression(expr)) return null;
  try {
    const v = literalValueDeep(expr);
    return JSON.stringify(v);
  } catch {
    return null;
  }
}

function literalValueDeep(node: ts.Node): unknown {
  if (ts.isObjectLiteralExpression(node)) {
    const out: Record<string, unknown> = {};
    for (const p of node.properties) {
      if (!ts.isPropertyAssignment(p)) throw new Error("non-literal");
      out[propKey(p, "?")] = literalValueDeep(p.initializer);
    }
    return out;
  }
  if (ts.isArrayLiteralExpression(node)) return node.elements.map(literalValueDeep);
  return literalValue(node, "?");
}

// ---------------------------------------------------------------- capability detection

function detectCapabilities(fnNode: ts.Node): string[] {
  const caps = new Set<string>();
  const walk = (n: ts.Node): void => {
    if (ts.isPropertyAccessExpression(n) && n.expression.getText() === "ctx.native") {
      caps.add(n.name.text);
    }
    n.forEachChild(walk);
  };
  fnNode.forEachChild(walk);
  return [...caps];
}

// ---------------------------------------------------------------- route extraction

function routeFromCall(call: ts.CallExpression, file: string, moduleId: string, bindingName: string): RouteInfo {
  const arg = call.arguments[0];
  if (!arg || !ts.isObjectLiteralExpression(arg)) {
    throw new CompileError("route() expects an object literal", nodeLoc(call, file));
  }
  const props = new Map<string, ts.PropertyAssignment | ts.MethodDeclaration>();
  for (const p of arg.properties) {
    if (ts.isPropertyAssignment(p) || ts.isMethodDeclaration(p)) props.set(propKey(p, file), p);
  }
  const need = (k: string): ts.Expression | ts.Node => {
    const p = props.get(k);
    if (!p) throw new CompileError(`route is missing '${k}'`, nodeLoc(arg, file));
    return ts.isPropertyAssignment(p) ? p.initializer : p;
  };

  const idNode = need("id") as ts.Expression;
  const id = String(literalValue(idNode, file));
  const method = String(literalValue(need("method") as ts.Expression, file)).toUpperCase();
  const path = String(literalValue(need("path") as ts.Expression, file));
  if (!/^(GET|POST|PUT|PATCH|DELETE)$/.test(method)) {
    throw new CompileError(`unsupported method '${method}'`, nodeLoc(idNode, file));
  }
  if (!path.startsWith("/")) {
    throw new CompileError(`route path must start with '/': '${path}'`, nodeLoc(idNode, file));
  }

  // params/query/body schemas
  const schemaOf = (k: string): Ir | null => {
    const p = props.get(k);
    if (!p) return null;
    const init = ts.isPropertyAssignment(p) ? p.initializer : p;
    return schemaFromNode(init, file);
  };
  const paramsIr = schemaOf("params");
  const queryIr = schemaOf("query");
  const bodyIr = schemaOf("body");

  // response map: { 200: s.object(...) }
  const respProp = props.get("response");
  if (!respProp) throw new CompileError(`route '${id}' is missing 'response'`, nodeLoc(arg, file));
  const respInit = ts.isPropertyAssignment(respProp) ? respProp.initializer : respProp;
  if (!ts.isObjectLiteralExpression(respInit)) {
    throw new CompileError(`route '${id}' response must be an object literal of status → schema`, nodeLoc(respInit, file));
  }
  const responses: RouteInfo["responses"] = {};
  for (const p of respInit.properties) {
    if (!ts.isPropertyAssignment(p)) {
      throw new CompileError("response entries must be literal", nodeLoc(p, file));
    }
    const statusKey = propKey(p, file);
    if (!/^\d+$/.test(statusKey)) {
      throw new CompileError(`response key '${statusKey}' must be a numeric status`, nodeLoc(p, file));
    }
    responses[statusKey] = {
      strategy: "native", // ADR-0015: native default; engine JS available per-route
      ir: schemaFromNode(p.initializer, file),
    };
  }
  // problem-bearing statuses: policy-declared statuses get problem refs later
  const handleProp = props.get("handle");
  if (!handleProp) throw new CompileError(`route '${id}' is missing 'handle'`, nodeLoc(arg, file));

  if (path === "/health/ready") {
    throw new CompileError(
      `route path '/health/ready' is reserved by the runtime for readiness probes`,
      nodeLoc(arg, file),
      "choose a different path such as '/ready' or '/app-ready'",
    );
  }

  // policy reference: identifier bound to a definePolicy result
  let policyId: string | null = null;
  const policyProp = props.get("policy");
  if (policyProp && ts.isPropertyAssignment(policyProp)) {
    const init = policyProp.initializer;
    if (!ts.isIdentifier(init)) {
      throw new CompileError("route policy must reference a definePolicy(...) variable", nodeLoc(init, file));
    }
    policyId = init.text; // resolved to policy id by the extractor pass
  }

  // native liveness from statically evaluable handler
  const livenessBody = tryStaticHandler(handleProp, file);
  const liveness = livenessBody
    ? { status: 200, contentType: "application/json", body: livenessBody }
    : null;

  // capabilities used inside the handler
  const handleNode = ts.isPropertyAssignment(handleProp) ? handleProp.initializer : handleProp;
  const capabilities = detectCapabilities(handleNode);

  return {
    id,
    method,
    path,
    moduleId,
    bindingName,
    sourceFile: file,
    policyId,
    paramsIr,
    queryIr,
    bodyIr,
    responses,
    liveness,
    capabilities,
  };
}

// ---------------------------------------------------------------- module/app extraction

export function extractApp(entryFile: string): ExtractedApp {
  const program = ts.createProgram([entryFile], {
    target: ts.ScriptTarget.ES2022,
    module: ts.ModuleKind.ESNext,
    moduleResolution: ts.ModuleResolutionKind.Bundler,
    strict: false,
    noEmit: true,
    allowImportingTsExtensions: true,
    paths: {
      "@velqu/core": ["packages/core/src/index.ts"],
      "@velqu/schema": ["packages/schema/src/index.ts"],
    },
    baseUrl: ".",
  });

  // unsupported import scan (COMP-006) across all sources
  const unsupportedImports: CompileError[] = [];
  for (const sf of program.getSourceFiles()) {
    if (sf.fileName.includes("node_modules")) continue;
    sf.forEachChild((n) => {
      if (ts.isImportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
        const spec = n.moduleSpecifier.text;
        if (spec.startsWith("node:") || spec.startsWith("bun:") || spec === "fs" || spec === "path" || spec === "http") {
          unsupportedImports.push(
            new CompileError(
              `unsupported import '${spec}'`,
              nodeLoc(n, sf.fileName),
              "Velqu apps run on QuickJS with no Node/Bun APIs (ADR-0003)",
            ),
          );
        }
      }
    });
  }
  if (unsupportedImports.length) throw unsupportedImports[0];

  // walk every (app) source file; find route/definePolicy/defineService/defineApp declarations
  const routes: RouteInfo[] = [];
  const policies: PolicyInfo[] = [];
  const modules: string[] = [];
  let appId = "app";
  const checker = program.getTypeChecker();

  const visit = (n: ts.Node, sf: ts.SourceFile): void => {
    // const X = route({...})
    if (ts.isVariableStatement(n)) {
      for (const decl of n.declarationList.declarations) {
        const init = decl.initializer;
        if (!init || !ts.isIdentifier(decl.name)) continue;
        if (isCallTo(init, ["route"])) {
          const moduleId = sf.fileName.split("/").slice(-2, -1)[0] ?? "app";
          routes.push(routeFromCall(init, sf.fileName, moduleId, decl.name.text));
        } else if (isCallTo(init, ["definePolicy"])) {
          const arg = init.arguments[0];
          const idProp = arg && ts.isObjectLiteralExpression(arg)
            ? arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, sf.fileName) === "id")
            : undefined;
          const declares = arg && ts.isObjectLiteralExpression(arg)
            ? arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, sf.fileName) === "declares")
            : undefined;
          const statuses: number[] = [];
          if (declares && ts.isPropertyAssignment(declares) && ts.isObjectLiteralExpression(declares.initializer)) {
            for (const p of declares.initializer.properties) {
              if (ts.isPropertyAssignment(p)) statuses.push(Number(propKey(p, sf.fileName)));
            }
          }
          policies.push({
            id: idProp && ts.isPropertyAssignment(idProp) ? String(literalValue(idProp.initializer, sf.fileName)) : decl.name.text,
            bindingName: decl.name.text,
            sourceFile: sf.fileName,
            declaredStatuses: statuses,
          });
        } else if (isCallTo(init, ["defineModule"])) {
          const arg = init.arguments[0];
          if (arg && ts.isObjectLiteralExpression(arg)) {
            const idProp = arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, sf.fileName) === "id");
            if (idProp && ts.isPropertyAssignment(idProp)) modules.push(String(literalValue(idProp.initializer, sf.fileName)));
          }
        } else if (isCallTo(init, ["defineApp"])) {
          const arg = init.arguments[0];
          if (arg && ts.isObjectLiteralExpression(arg)) {
            const idProp = arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, sf.fileName) === "id");
            if (idProp && ts.isPropertyAssignment(idProp)) appId = String(literalValue(idProp.initializer, sf.fileName));
          }
        }
      }
    }
    // default export route({...}) — e.g. `export default route({...})`
    if (ts.isExportAssignment(n) && ts.isCallExpression(n.expression) && isCallTo(n.expression, ["route"])) {
      const moduleId = sf.fileName.split("/").slice(-2, -1)[0] ?? "app";
      routes.push(routeFromCall(n.expression, sf.fileName, moduleId, `__default_${sf.fileName.replace(/\W/g, "_")}`));
      const r = routes[routes.length - 1];
      r.bindingName = "__default_export";
    }
    n.forEachChild((c) => visit(c, sf));
  };

  for (const sf of program.getSourceFiles()) {
    if (sf.fileName.includes("node_modules") || !sf.fileName.endsWith(".ts")) continue;
    visit(sf, sf);
  }

  if (routes.length === 0) {
    throw new CompileError(`no route() declarations found from entry ${entryFile}`);
  }

  // resolve policy variable names → policy ids + merge declared statuses into route responses
  const policyByBinding = new Map(policies.map((p) => [p.bindingName, p]));
  for (const r of routes) {
    if (r.policyId) {
      const pol = policyByBinding.get(r.policyId);
      if (!pol) {
        throw new CompileError(
          `route '${r.id}' references unknown policy '${r.policyId}'`,
          { file: r.sourceFile, line: 1, column: 1 },
          "imported policies must be definePolicy(...) variables",
        );
      }
      for (const s of pol.declaredStatuses) {
        if (!r.responses[String(s)]) r.responses[String(s)] = { strategy: "native", problem: problemForStatus(s) };
      }
      r.policyId = pol.id;
    }
    // 404/401 problems only exist where handlers return them; keep minimal set:
    if (r.responses["404"] && !r.responses["404"].problem) r.responses["404"].problem = "not-found";
  }

  // canonical collision detection (COMP-004)
  const seen = new Map<string, string>();
  for (const r of routes) {
    const key = `${r.method} ${canonPath(r.path)}`;
    const prev = seen.get(key);
    if (prev) {
      throw new CompileError(
        `route collision: ${key} is declared by '${prev}' and '${r.id}'`,
        { file: r.sourceFile, line: 1, column: 1 },
        "canonically equivalent routes must not repeat",
      );
    }
    seen.set(key, r.id);
  }
  const seenIds = new Set<string>();
  for (const r of routes) {
    if (seenIds.has(r.id)) {
      throw new CompileError(`duplicate route id '${r.id}'`, { file: r.sourceFile, line: 1, column: 1 });
    }
    seenIds.add(r.id);
  }

  const usedModules = [...new Set(routes.map((r) => r.moduleId))];
  return { appId, entryFile, routes, policies, modules: modules.length ? modules : usedModules };
}

function canonPath(p: string): string {
  return p
    .split("/")
    .filter(Boolean)
    .map((s) => (s.startsWith(":") ? ":param" : s.startsWith("*") ? "*" : s))
    .join("/");
}

function problemForStatus(s: number): string {
  switch (s) {
    case 401: return "unauthorized";
    case 404: return "not-found";
    case 422: return "validation";
    case 405: return "method";
    case 413: return "limit";
    default: return "internal";
  }
}

export const hash = (s: string): string => createHash("sha256").update(s).digest("hex");
