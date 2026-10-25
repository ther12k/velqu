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
  /**
   * Whether the binding is reachable through its source module's export
   * surface: an `export` modifier on the declaration, a same-module
   * `export { name }` re-export, or `export default` (#1292). The browser
   * target references handler bindings via namespace imports and fails the
   * build on non-exported ones; the native target is unaffected.
   */
  exported: boolean;
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
  /**
   * Declared policy header name (lowercased at extraction). The route
   * plan's declared-header set is built from it, so the runtime
   * materializes exactly the header the policy check reads (#1401).
   * Defaults to "authorization" — definePolicy's own default.
   */
  header: string;
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

/** Whether a statement-level declaration carries the `export` modifier. */
function hasExportModifier(node: ts.Node): boolean {
  const mods = (node as { modifiers?: readonly ts.Modifier[] }).modifiers;
  return !!mods?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
}

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
      // keep insertion order (properties is a plain map in JSON);
      // propertyOrder carries the SOURCE declaration order explicitly so
      // response encoders can emit declared order (the Rust side stores
      // properties as a sorted BTreeMap for canonical hashing)
      return { kind: "object", properties, propertyOrder: Object.keys(properties), required };
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

/**
 * RUN-009 extension: detect a handler that references no ctx and returns a
 * string literal, foldable ONLY when the route declares a status-200
 * response whose schema is a plain constraint-free string — then the wire
 * semantics (text/plain; charset=utf-8) are known and the constant can be
 * served natively. Anything ambiguous keeps the engine path (fail closed).
 */
function tryStaticStringHandler(
  handleProp: ts.PropertyAssignment | ts.MethodDeclaration,
  file: string,
  responseIr: Record<string, unknown> | undefined,
): string | null {
  if (!responseIr || responseIr.kind !== "string") return null;
  // constraint-free only: constrained constant strings keep the engine path
  // (where the direct text plan validates them cheaply) rather than folding
  // past validation
  for (const k of ["minLength", "maxLength", "pattern", "format"]) {
    if (responseIr[k] !== undefined) return null;
  }
  let body: ts.Node | null = null;
  if (ts.isPropertyAssignment(handleProp)) {
    const init = handleProp.initializer;
    if (ts.isArrowFunction(init) || ts.isFunctionExpression(init)) body = init.body;
  } else if (ts.isMethodDeclaration(handleProp) && handleProp.body) {
    if (handleProp.parameters.length === 0) body = handleProp.body;
  }
  if (!body) return null;
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
  if (ts.isStringLiteral(expr)) return expr.text;
  if (ts.isNoSubstitutionTemplateLiteral(expr)) return expr.text;
  return null;
}

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
  // Names the handler's context binds for the capability object:
  // `ctx` (whole param), or a destructured alias like
  // `({ native })` / `({ native: n })`. Any of these forms grants.
  const aliases = new Set<string>(["ctx.native"]);
  const walkParams = (n: ts.Node): void => {
    if (ts.isParameter(n) && ts.isObjectBindingPattern(n.name)) {
      for (const el of n.name.elements) {
        if (ts.isBindingElement(el)) {
          const bound = el.propertyName?.getText() ?? el.name.getText();
          if (bound === "native") aliases.add(el.name.getText());
        }
      }
    }
    n.forEachChild(walkParams);
  };
  fnNode.forEachChild(walkParams);

  const caps = new Set<string>();
  const walk = (n: ts.Node): void => {
    if (ts.isPropertyAccessExpression(n) && aliases.has(n.expression.getText())) {
      caps.add(n.name.text);
    }
    n.forEachChild(walk);
  };
  fnNode.forEachChild(walk);
  return [...caps];
}

// ---------------------------------------------------------------- route extraction

/** Strip assertion wrappers (`x as const`, parens, `<T>x`, `x!`) for static resolution. */
function unwrapAssertion(e: ts.Expression): ts.Expression {
  let expr = e;
  while (
    expr.kind === ts.SyntaxKind.AsExpression ||
    expr.kind === ts.SyntaxKind.ParenthesizedExpression ||
    expr.kind === ts.SyntaxKind.TypeAssertionExpression ||
    expr.kind === ts.SyntaxKind.NonNullExpression
  ) {
    expr = (expr as unknown as { expression: ts.Expression }).expression;
  }
  return expr;
}

function arrayLiteralOf(e: ts.Expression): ts.ArrayLiteralExpression | null {
  const expr = unwrapAssertion(e);
  return ts.isArrayLiteralExpression(expr) ? expr : null;
}

/** #1421: spread members are invisible to static extraction — reject them
 *  instead of silently dropping the metadata they carry. */
function rejectSpreads(literal: ts.ObjectLiteralExpression, file: string, what: string): void {
  for (const p of literal.properties) {
    if (ts.isSpreadAssignment(p)) {
      throw new CompileError(
        `${what} object literal does not support spread properties`,
        nodeLoc(p, file),
        "static extraction cannot see spread members — inline every property (PRD §10.2 rejects unsupported dynamic metadata)",
      );
    }
  }
}

/** Parse a defineModule call's static id and raw routes expression. The
 *  routes expression is resolved against the app graph only when a defineApp
 *  is present (see the membership pass in extractApp). */
function moduleDefFromCall(
  call: ts.CallExpression,
  file: string,
  fallbackId?: string,
): { id: string; routesExpr: ts.Expression | undefined } {
  const arg = call.arguments[0];
  if (!arg || !ts.isObjectLiteralExpression(arg)) {
    throw new CompileError("defineModule expects an object literal", nodeLoc(call, file));
  }
  rejectSpreads(arg, file, "defineModule()");
  let id = fallbackId ?? "";
  const idProp = arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, file) === "id");
  if (idProp && ts.isPropertyAssignment(idProp)) {
    id = String(literalValue(idProp.initializer, file));
  }
  const routesProp = arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, file) === "routes");
  return {
    id,
    routesExpr: routesProp && ts.isPropertyAssignment(routesProp) ? routesProp.initializer : undefined,
  };
}

function routeFromCall(
  call: ts.CallExpression,
  file: string,
  moduleId: string,
  bindingName: string,
  exported: boolean,
  policyRefs?: Map<RouteInfo, ts.Identifier>,
): RouteInfo {
  const arg = call.arguments[0];
  if (!arg || !ts.isObjectLiteralExpression(arg)) {
    throw new CompileError("route() expects an object literal", nodeLoc(call, file));
  }
  const props = new Map<string, ts.PropertyAssignment | ts.MethodDeclaration>();
  for (const p of arg.properties) {
    if (ts.isSpreadAssignment(p)) {
      throw new CompileError(
        "route() object literal does not support spread properties",
        nodeLoc(p, file),
        "a spread can hide policy/schema metadata from static extraction — inline every property (PRD §10.2 rejects unsupported dynamic metadata)",
      );
    }
    if (ts.isShorthandPropertyAssignment(p)) {
      throw new CompileError(
        `route() object literal does not support shorthand property '${p.name.text}'`,
        nodeLoc(p, file),
        `write '${p.name.text}: ${p.name.text}' so the value expression is statically visible`,
      );
    }
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

  // policy reference: identifier bound to a definePolicy result. The
  // identifier NODE is captured so the extractor pass can resolve it by
  // TypeScript symbol identity — never by binding name (same-name policies
  // in different modules must not collide, #1421).
  let policyId: string | null = null;
  let policyRef: ts.Identifier | null = null;
  const policyProp = props.get("policy");
  if (policyProp && ts.isPropertyAssignment(policyProp)) {
    const init = policyProp.initializer;
    if (!ts.isIdentifier(init)) {
      throw new CompileError("route policy must reference a definePolicy(...) variable", nodeLoc(init, file));
    }
    policyId = init.text; // placeholder; symbol-resolved to a policy id below
    policyRef = init;
  }

  // native liveness from statically evaluable handler — SCHEMA-DRIVEN:
  // the constant folds only when the declared 200 response kind matches the
  // folded expression kind (object → application/json, string →
  // text/plain). A handler whose constant contradicts the declared schema
  // keeps the engine path, where it fails its contract like any dynamic
  // value (the old object-only fold bypassed that check).
  const responseIr200 = responses["200"]?.ir;
  const objectBody = tryStaticHandler(handleProp, file);
  const livenessBody =
    objectBody !== null && responseIr200?.kind === "object" ? objectBody : null;
  let liveness: { status: number; contentType: string; body: string } | null = null;
  if (livenessBody !== null) {
    liveness = { status: 200, contentType: "application/json", body: livenessBody };
  } else {
    // RUN-009 extension: constant string + declared constraint-free
    // s.string() 200 response → native text/plain (wire semantics known)
    const textBody = tryStaticStringHandler(handleProp, file, responses["200"]?.ir);
    if (textBody !== null) {
      liveness = { status: 200, contentType: "text/plain; charset=utf-8", body: textBody };
    }
  }

  // capabilities used inside the handler
  const handleNode = ts.isPropertyAssignment(handleProp) ? handleProp.initializer : handleProp;
  const capabilities = detectCapabilities(handleNode);

  const info: RouteInfo = {
    id,
    method,
    path,
    moduleId,
    bindingName,
    exported,
    sourceFile: file,
    policyId,
    paramsIr,
    queryIr,
    bodyIr,
    responses,
    liveness,
    capabilities,
  };
  if (policyRef && policyRefs) policyRefs.set(info, policyRef);
  return info;
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
  let routes: RouteInfo[] = [];
  const policies: PolicyInfo[] = [];
  const modules: string[] = [];
  // #1292: local names re-exported after their declaration (`export { tick }`)
  const reExportedBindings = new Map<string, Set<string>>();
  let appId = "app";
  const checker = program.getTypeChecker();

  // #1421 static-graph artifacts: routes keyed by their declaring node,
  // array-of-route bindings, module defs keyed by declaration/call node, and
  // defineApp call sites — the inputs for symbol-based policy resolution and
  // app-graph membership below.
  const policyRefs = new Map<RouteInfo, ts.Identifier>();
  const policyByDecl = new Map<ts.Node, PolicyInfo>();
  const routeByDecl = new Map<ts.Node, RouteInfo>();
  const arrayByDecl = new Map<ts.Node, readonly ts.Expression[]>();
  interface ModuleDef {
    id: string;
    routesExpr: ts.Expression | undefined;
    file: string;
  }
  const moduleDefs = new Map<ts.Node, ModuleDef>();
  const appCalls: { call: ts.CallExpression; file: string }[] = [];

  const visit = (n: ts.Node, sf: ts.SourceFile): void => {
    // const X = route({...})
    if (ts.isVariableStatement(n)) {
      for (const decl of n.declarationList.declarations) {
        const init = decl.initializer;
        if (!init || !ts.isIdentifier(decl.name)) continue;
        if (isCallTo(init, ["route"])) {
          const moduleId = sf.fileName.split("/").slice(-2, -1)[0] ?? "app";
          const info = routeFromCall(init, sf.fileName, moduleId, decl.name.text, hasExportModifier(n), policyRefs);
          routes.push(info);
          routeByDecl.set(decl, info);
        } else if (isCallTo(init, ["definePolicy"])) {
          const arg = init.arguments[0];
          if (!arg || !ts.isObjectLiteralExpression(arg)) {
            throw new CompileError("definePolicy expects an object literal", nodeLoc(init, sf.fileName));
          }
          rejectSpreads(arg, sf.fileName, "definePolicy()");
          const idProp = arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, sf.fileName) === "id");
          const declares = arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, sf.fileName) === "declares");
          const headerProp = arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, sf.fileName) === "header");
          const statuses: number[] = [];
          if (declares && ts.isPropertyAssignment(declares) && ts.isObjectLiteralExpression(declares.initializer)) {
            for (const p of declares.initializer.properties) {
              if (ts.isPropertyAssignment(p)) statuses.push(Number(propKey(p, sf.fileName)));
            }
          }
          const policy: PolicyInfo = {
            id: idProp && ts.isPropertyAssignment(idProp) ? String(literalValue(idProp.initializer, sf.fileName)) : decl.name.text,
            bindingName: decl.name.text,
            sourceFile: sf.fileName,
            declaredStatuses: statuses,
            header: headerProp && ts.isPropertyAssignment(headerProp)
              ? String(literalValue(headerProp.initializer, sf.fileName)).toLowerCase()
              : "authorization",
          };
          policies.push(policy);
          policyByDecl.set(decl, policy);
        } else if (isCallTo(init, ["defineModule"])) {
          const def = moduleDefFromCall(init, sf.fileName, decl.name.text);
          moduleDefs.set(decl, { ...def, file: sf.fileName });
          modules.push(def.id);
        } else if (isCallTo(init, ["defineApp"])) {
          const arg = init.arguments[0];
          if (arg && ts.isObjectLiteralExpression(arg)) {
            const idProp = arg.properties.find((p) => ts.isPropertyAssignment(p) && propKey(p, sf.fileName) === "id");
            if (idProp && ts.isPropertyAssignment(idProp)) appId = String(literalValue(idProp.initializer, sf.fileName));
          }
        } else {
          const arr = arrayLiteralOf(init);
          if (arr) arrayByDecl.set(decl, arr.elements);
        }
      }
    }
    // default export route({...}) — e.g. `export default route({...})`
    if (ts.isExportAssignment(n) && ts.isCallExpression(n.expression) && isCallTo(n.expression, ["route"])) {
      const moduleId = sf.fileName.split("/").slice(-2, -1)[0] ?? "app";
      const info = routeFromCall(n.expression, sf.fileName, moduleId, `__default_${sf.fileName.replace(/\W/g, "_")}`, true, policyRefs);
      info.bindingName = "__default_export";
      routes.push(info);
      routeByDecl.set(n, info);
    }
    // default export of an array of routes — `export default [a, b] as const`
    if (ts.isExportAssignment(n) && !(ts.isCallExpression(n.expression) && isCallTo(n.expression, ["route"]))) {
      const arr = arrayLiteralOf(n.expression);
      if (arr) arrayByDecl.set(n, arr.elements);
    }
    // export { tick } / export { tick as tickRoute } — the local binding
    // (propertyName when present, else the specifier name) joins the
    // module's export surface regardless of declaration order.
    if (ts.isExportDeclaration(n) && n.exportClause && ts.isNamedExports(n.exportClause)) {
      let names = reExportedBindings.get(sf.fileName);
      if (!names) reExportedBindings.set(sf.fileName, (names = new Set()));
      for (const el of n.exportClause.elements) {
        names.add((el.propertyName ?? el.name).text);
      }
    }
    // defineModule/defineApp call sites in ANY position — variable form,
    // default export, or inline inside defineApp modules arrays. These feed
    // app-graph membership (#1421); parse errors fail closed here.
    if (ts.isCallExpression(n)) {
      // callee-name check, not isCallTo: isCallTo's type predicate would
      // narrow n to never in the else-branch
      const callee = n.expression.getText().replace(/^.*\./, "");
      if (callee === "defineModule") {
        const parent = n.parent;
        const fallbackId =
          parent && ts.isVariableDeclaration(parent) && ts.isIdentifier(parent.name) ? parent.name.text : undefined;
        const def = { ...moduleDefFromCall(n, sf.fileName, fallbackId), file: sf.fileName };
        moduleDefs.set(n, def);
        if (parent && ts.isVariableDeclaration(parent)) moduleDefs.set(parent, def);
      } else if (callee === "defineApp") {
        const arg = n.arguments[0];
        if (!arg || !ts.isObjectLiteralExpression(arg)) {
          throw new CompileError("defineApp expects an object literal", nodeLoc(n, sf.fileName));
        }
        rejectSpreads(arg, sf.fileName, "defineApp()");
        appCalls.push({ call: n, file: sf.fileName });
      }
    }
    n.forEachChild((c) => visit(c, sf));
  };

  for (const sf of program.getSourceFiles()) {
    if (sf.fileName.includes("node_modules") || !sf.fileName.endsWith(".ts")) continue;
    visit(sf, sf);
  }

  // #1292: fold same-module re-exports into the per-route export flag
  // (the export declaration may appear after the route declaration).
  for (const r of routes) {
    if (!r.exported && reExportedBindings.get(r.sourceFile)?.has(r.bindingName)) {
      r.exported = true;
    }
  }

  if (routes.length === 0) {
    throw new CompileError(`no route() declarations found from entry ${entryFile}`);
  }

  // Resolve policy references by TypeScript SYMBOL identity (#1421): each
  // route captured its policy identifier node; resolve it through the
  // checker (imports and aliases included) and bind the policy declared by
  // that exact symbol. The old binding-NAME map collided when different
  // modules exported same-named policies, silently binding routes to the
  // wrong policy/header/handler.
  const resolveSymbol = (id: ts.Identifier): ts.Symbol | undefined => {
    let sym = checker.getSymbolAtLocation(id);
    if (sym && (sym.flags & ts.SymbolFlags.Alias) !== 0) {
      try {
        const aliased = checker.getAliasedSymbol(sym);
        if (aliased && aliased !== sym) sym = aliased;
      } catch {
        // unresolved import — keep the alias symbol; the declaration scan
        // below finds no policy and fails closed
      }
    }
    return sym;
  };
  const policyForDeclarations = (declarations: readonly ts.Declaration[] | undefined): PolicyInfo | undefined => {
    for (const d of declarations ?? []) {
      if (policyByDecl.has(d)) return policyByDecl.get(d);
      // `export default somePolicy;` — follow the exported identifier
      if (ts.isExportAssignment(d) && ts.isIdentifier(d.expression)) {
        const inner = resolveSymbol(d.expression);
        for (const dd of inner?.declarations ?? []) {
          if (policyByDecl.has(dd)) return policyByDecl.get(dd);
        }
      }
    }
    return undefined;
  };

  // App-graph membership bindings (#1421): a route binding resolves through
  // symbol identity to a route declaration, an exported array of routes, or
  // a default export of either.
  type RouteBinding = { kind: "route"; route: RouteInfo } | { kind: "array"; elements: readonly ts.Expression[] };
  const routeBindingFor = (id: ts.Identifier): RouteBinding | null => {
    for (const d of resolveSymbol(id)?.declarations ?? []) {
      const b = bindingFromDecl(d);
      if (b) return b;
    }
    return null;
  };
  const bindingFromExpr = (e: ts.Expression): RouteBinding | null => {
    const expr = unwrapAssertion(e);
    if (ts.isArrayLiteralExpression(expr)) return { kind: "array", elements: expr.elements };
    if (ts.isIdentifier(expr)) return routeBindingFor(expr);
    return null;
  };
  const bindingFromDecl = (d: ts.Node): RouteBinding | null => {
    if (routeByDecl.has(d)) return { kind: "route", route: routeByDecl.get(d)! };
    if (arrayByDecl.has(d)) return { kind: "array", elements: arrayByDecl.get(d)! };
    if (ts.isExportAssignment(d)) return bindingFromExpr(d.expression);
    if (ts.isVariableDeclaration(d) && d.initializer) return bindingFromExpr(d.initializer);
    return null;
  };

  for (const r of routes) {
    const ref = policyRefs.get(r);
    if (r.policyId && ref) {
      const pol = policyForDeclarations(resolveSymbol(ref)?.declarations);
      if (!pol) {
        throw new CompileError(
          `route '${r.id}' references unknown policy '${ref.text}'`,
          { file: r.sourceFile, line: 1, column: 1 },
          "policies resolve by import symbol — import the definePolicy(...) variable in the route's module (cross-module name matching is not supported)",
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

  // App-graph membership (#1421): when the application declares defineApp,
  // exactly the routes reachable through its module graph are emitted — a
  // route() declaration that no included module references stays OUT of the
  // pack (removing a route from composition removes the endpoint). Without
  // defineApp, every discovered route ships (legacy collect-all behavior,
  // unchanged). Filtering keeps the original discovery order so existing
  // packs stay byte-stable.
  if (appCalls.length > 1) {
    const first = appCalls[0];
    throw new CompileError(
      `multiple defineApp declarations are not supported (found ${appCalls.length})`,
      nodeLoc(first.call, first.file),
    );
  }
  if (appCalls.length === 1) {
    const appCall = appCalls[0];
    const appArg = appCall.call.arguments[0];
    if (!appArg || !ts.isObjectLiteralExpression(appArg)) {
      throw new CompileError("defineApp expects an object literal", nodeLoc(appCall.call, appCall.file));
    }
    const modulesProp = appArg.properties.find(
      (p) => ts.isPropertyAssignment(p) && propKey(p, appCall.file) === "modules",
    );
    if (!modulesProp || !ts.isPropertyAssignment(modulesProp)) {
      throw new CompileError(
        "defineApp requires a static 'modules' property",
        nodeLoc(appArg, appCall.file),
        "list defineModule(...) values or exported module bindings",
      );
    }
    const modulesInit = unwrapAssertion(modulesProp.initializer);
    if (!ts.isArrayLiteralExpression(modulesInit)) {
      throw new CompileError(
        "defineApp 'modules' must be an array literal",
        nodeLoc(modulesProp.initializer, appCall.file),
      );
    }

    const included = new Set<RouteInfo>();
    const includeRoute = (r: RouteInfo): void => {
      included.add(r);
    };
    const resolveRouteElements = (el: ts.Expression, where: string): void => {
      if (ts.isSpreadElement(el)) {
        const target = unwrapAssertion(el.expression);
        if (!ts.isIdentifier(target)) {
          throw new CompileError(
            `module routes spread '...${el.expression.getText()}' is not statically resolvable`,
            nodeLoc(el, appCall.file),
            "spread an exported route or an exported array of routes",
          );
        }
        const binding = routeBindingFor(target);
        if (!binding) {
          throw new CompileError(
            `module routes spread '...${target.text}' does not resolve to an exported route or array of routes`,
            nodeLoc(el, appCall.file),
          );
        }
        if (binding.kind === "route") includeRoute(binding.route);
        else for (const inner of binding.elements) resolveRouteElements(inner, where);
        return;
      }
      const expr = unwrapAssertion(el);
      if (ts.isIdentifier(expr)) {
        const binding = routeBindingFor(expr);
        if (binding?.kind === "route") {
          includeRoute(binding.route);
          return;
        }
        if (binding?.kind === "array") {
          for (const inner of binding.elements) resolveRouteElements(inner, where);
          return;
        }
        throw new CompileError(
          `${where} references '${expr.text}', which is not an exported route or array of routes`,
          nodeLoc(el, appCall.file),
          "routes must be statically referenceable bindings",
        );
      }
      throw new CompileError(
        `${where} contains a dynamic expression ('${el.getText().slice(0, 40)}')`,
        nodeLoc(el, appCall.file),
        "PRD §10.2: unsupported dynamic metadata is rejected — reference route bindings or exported arrays",
      );
    };

    for (const el of modulesInit.elements) {
      if (ts.isSpreadElement(el)) {
        throw new CompileError(
          "defineApp 'modules' does not support spreads",
          nodeLoc(el, appCall.file),
          "list each module explicitly",
        );
      }
      const expr = unwrapAssertion(el);
      let def: ModuleDef | undefined;
      if (ts.isIdentifier(expr)) {
        for (const d of resolveSymbol(expr)?.declarations ?? []) {
          if (moduleDefs.has(d)) {
            def = moduleDefs.get(d)!;
            break;
          }
          if (ts.isExportAssignment(d)) {
            const inner = unwrapAssertion(d.expression);
            if (ts.isCallExpression(inner) && moduleDefs.has(inner)) {
              def = moduleDefs.get(inner)!;
              break;
            }
          }
        }
        if (!def) {
          throw new CompileError(
            `defineApp module reference '${expr.text}' does not resolve to a defineModule(...)`,
            nodeLoc(el, appCall.file),
          );
        }
      } else if (ts.isCallExpression(expr) && isCallTo(expr, ["defineModule"])) {
        def = moduleDefs.get(expr);
        if (!def) {
          throw new CompileError("inline defineModule could not be parsed", nodeLoc(el, appCall.file));
        }
      } else {
        throw new CompileError(
          `defineApp 'modules' contains an unsupported expression ('${el.getText().slice(0, 40)}')`,
          nodeLoc(el, appCall.file),
        );
      }
      if (def.routesExpr === undefined) {
        throw new CompileError(
          `defineModule '${def.id}' is missing a static 'routes' property`,
          nodeLoc(el, appCall.file),
        );
      }
      const moduleRoutes = unwrapAssertion(def.routesExpr);
      if (ts.isArrayLiteralExpression(moduleRoutes)) {
        for (const inner of moduleRoutes.elements) resolveRouteElements(inner, `module '${def.id}' routes`);
      } else {
        resolveRouteElements(moduleRoutes, `module '${def.id}' routes`);
      }
    }
    if (included.size === 0) {
      throw new CompileError("defineApp resolved to zero routes", nodeLoc(appCall.call, appCall.file));
    }
    // keep original discovery order so existing packs stay byte-stable
    routes = routes.filter((r) => included.has(r));
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
