/**
 * @velqu/compiler — strategy selection based on measured evidence.
 *
 * M25-002-D: compiler strategy selection rules derived from empirical benchmark
 * evidence (M25-002-A/B/C codec & bridge suites).
 *
 * Rules:
 * 1. Native Default: representable Schema IR v2 inputs and structured JSON
 *    responses select "native" strategy.
 *    Evidence: M25-002-A/B/C shows 20–40% lower latency on small/nested
 *    payloads, ~6x lower latency on 16–64 KB payloads, zero bridge crossing
 *    overhead for pre-validated data, and no engine stringify jitter.
 * 2. Explicit Fallback: schemas carrying s_fallback (or unsupported transforms)
 *    select "js" strategy with an explicit reason ("unsupported-transform",
 *    "unrepresentable", "measured", "explicit").
 * 3. Fallback Cost Visibility: every fallback records estimated latency and
 *    allocation overhead based on measured M25-002 evidence, surfaced in
 *    build reports and `velqu inspect fallbacks`.
 * 4. No Single Strategy Forced Globally: decisions are computed per-route and
 *    per-response status deterministically from the schema graph.
 */

import type { RouteInfo } from "./extract";

export type StrategyName = "native" | "js";

export type FallbackReason = "unsupported-transform" | "unrepresentable" | "measured" | "explicit";

export interface FallbackDescriptor {
  route: string;
  location: "params" | "query" | "body" | `response.${string}`;
  strategy: StrategyName;
  reason: FallbackReason;
  estimatedOverheadUs: number;
  estimatedAllocBytes: number;
  description: string;
}

export interface RouteStrategyDecision {
  routeId: string;
  validationStrategy: StrategyName;
  responseStrategies: Record<string, StrategyName>;
  primaryResponseStrategy: StrategyName;
  basis: string;
  fallbacks: FallbackDescriptor[];
}

export interface AppStrategyReport {
  validation: "native" | "hybrid" | "js";
  responses: "native" | "hybrid" | "js";
  fallbacks: FallbackDescriptor[];
  decisions: Array<{
    route: string;
    validationStrategy: StrategyName;
    responseStrategy: StrategyName;
    basis: string;
  }>;
  evidenceSummary: {
    source: string;
    referenceHost: string;
    smallObjectP50Us: { native: number; quickjs: number };
    largeObject64kP50Us: { native: number; quickjs: number };
    array1000P50Us: { native: number; quickjs: number };
  };
  notes: string[];
}

type IrNode = { kind?: string; reason?: string; [k: string]: unknown };

function findFallbackNode(node: unknown): { reason: FallbackReason; kind: string } | null {
  if (!node || typeof node !== "object") return null;
  const ir = node as IrNode;
  if (ir.kind === "fallback" && typeof ir.reason === "string") {
    return { reason: ir.reason as FallbackReason, kind: "fallback" };
  }
  if (ir.kind === "transform") {
    return { reason: "unsupported-transform", kind: "transform" };
  }
  if (ir.kind === "file") {
    return { reason: "unsupported-transform", kind: "file" };
  }
  if (ir.kind === "optional" || ir.kind === "nullable") {
    return findFallbackNode(ir.inner);
  }
  if (ir.kind === "array") {
    return findFallbackNode(ir.items);
  }
  if (ir.kind === "object" && ir.properties && typeof ir.properties === "object") {
    for (const prop of Object.values(ir.properties as Record<string, unknown>)) {
      const found = findFallbackNode(prop);
      if (found) return found;
    }
  }
  if (ir.kind === "union" && Array.isArray(ir.members)) {
    for (const member of ir.members) {
      const found = findFallbackNode(member);
      if (found) return found;
    }
  }
  return null;
}

function estimateFallbackCost(reason: FallbackReason, kind: string): { overheadUs: number; allocBytes: number; desc: string } {
  switch (reason) {
    case "unsupported-transform":
      return {
        overheadUs: 35,
        allocBytes: 10780,
        desc: `${kind} node requires QuickJS engine fallback (no native codec in current runtime)`,
      };
    case "unrepresentable":
      return {
        overheadUs: 30,
        allocBytes: 10500,
        desc: "unrepresentable schema shape requires QuickJS execution",
      };
    case "measured":
      return {
        overheadUs: 400,
        allocBytes: 1894000,
        desc: "measured benchmark evidence selected engine strategy (avoids host parse/projection)",
      };
    case "explicit":
    default:
      return {
        overheadUs: 25,
        allocBytes: 10180,
        desc: "explicit developer fallback marker routes to QuickJS engine",
      };
  }
}

/**
 * Select strategies for a single route deterministically from its schema graph.
 */
export function selectRouteStrategies(route: RouteInfo): RouteStrategyDecision {
  const fallbacks: FallbackDescriptor[] = [];

  // 1. Input validation strategy
  let validationStrategy: StrategyName = "native";
  let validationBasis = "representable Schema IR v2 with native validator (M25-002: 20–40% lower latency, 0 bridge crossings)";

  const bodyFallback = findFallbackNode(route.bodyIr);
  const queryFallback = findFallbackNode(route.queryIr);
  const paramsFallback = findFallbackNode(route.paramsIr);

  if (bodyFallback) {
    validationStrategy = "js";
    validationBasis = `input body schema contains ${bodyFallback.kind} (${bodyFallback.reason})`;
    const cost = estimateFallbackCost(bodyFallback.reason, bodyFallback.kind);
    fallbacks.push({
      route: route.id,
      location: "body",
      strategy: "js",
      reason: bodyFallback.reason,
      estimatedOverheadUs: cost.overheadUs,
      estimatedAllocBytes: cost.allocBytes,
      description: cost.desc,
    });
  } else if (queryFallback) {
    validationStrategy = "js";
    validationBasis = `query schema contains ${queryFallback.kind} (${queryFallback.reason})`;
    const cost = estimateFallbackCost(queryFallback.reason, queryFallback.kind);
    fallbacks.push({
      route: route.id,
      location: "query",
      strategy: "js",
      reason: queryFallback.reason,
      estimatedOverheadUs: cost.overheadUs,
      estimatedAllocBytes: cost.allocBytes,
      description: cost.desc,
    });
  } else if (paramsFallback) {
    validationStrategy = "js";
    validationBasis = `params schema contains ${paramsFallback.kind} (${paramsFallback.reason})`;
    const cost = estimateFallbackCost(paramsFallback.reason, paramsFallback.kind);
    fallbacks.push({
      route: route.id,
      location: "params",
      strategy: "js",
      reason: paramsFallback.reason,
      estimatedOverheadUs: cost.overheadUs,
      estimatedAllocBytes: cost.allocBytes,
      description: cost.desc,
    });
  }

  // 2. Response serialization strategies per status code
  const responseStrategies: Record<string, StrategyName> = {};
  for (const [status, resp] of Object.entries(route.responses)) {
    if (resp.strategy === "js") {
      responseStrategies[status] = "js";
      continue;
    }
    const respFallback = findFallbackNode(resp.ir);
    if (respFallback) {
      responseStrategies[status] = "js";
      const cost = estimateFallbackCost(respFallback.reason, respFallback.kind);
      fallbacks.push({
        route: route.id,
        location: `response.${status}`,
        strategy: "js",
        reason: respFallback.reason,
        estimatedOverheadUs: cost.overheadUs,
        estimatedAllocBytes: cost.allocBytes,
        description: cost.desc,
      });
    } else {
      responseStrategies[status] = "native";
    }
  }

  const primaryResponseStrategy: StrategyName =
    responseStrategies["200"] ?? (Object.values(responseStrategies)[0] ?? "native");

  return {
    routeId: route.id,
    validationStrategy,
    responseStrategies,
    primaryResponseStrategy,
    basis: validationBasis,
    fallbacks,
  };
}

/**
 * Evaluate strategy decisions across all routes in an app and produce
 * the build report strategy summary.
 */
export function evaluateAppStrategies(routes: RouteInfo[]): {
  decisions: Map<string, RouteStrategyDecision>;
  report: AppStrategyReport;
} {
  const decisions = new Map<string, RouteStrategyDecision>();
  const allFallbacks: FallbackDescriptor[] = [];
  const decisionSummaries: Array<{
    route: string;
    validationStrategy: StrategyName;
    responseStrategy: StrategyName;
    basis: string;
  }> = [];

  let hasNativeVal = false;
  let hasJsVal = false;
  let hasNativeResp = false;
  let hasJsResp = false;

  for (const r of routes) {
    const d = selectRouteStrategies(r);
    decisions.set(r.id, d);
    allFallbacks.push(...d.fallbacks);
    decisionSummaries.push({
      route: r.id,
      validationStrategy: d.validationStrategy,
      responseStrategy: d.primaryResponseStrategy,
      basis: d.basis,
    });

    if (d.validationStrategy === "native") hasNativeVal = true;
    else hasJsVal = true;

    if (d.primaryResponseStrategy === "native") hasNativeResp = true;
    else hasJsResp = true;
  }

  const validationSummary = hasNativeVal && hasJsVal ? "hybrid" : hasNativeVal ? "native" : "js";
  const responseSummary = hasNativeResp && hasJsResp ? "hybrid" : hasNativeResp ? "native" : "js";

  const notes = [
    "validation: native default for representable Schema IR v2 (M25-002 evidence: 20–40% lower latency on small/nested shapes, ~6x lower on 16–64 KB payloads)",
    "responses: native serialization default (ADR-0015, M25-002-C: zero bridge crossings, no JS stringify jitter)",
    "fallback: explicit per-route fallback with visible reason and estimated overhead (SCHEMA-005, ADR-0009)",
  ];

  if (allFallbacks.length > 0) {
    notes.push(
      `active fallbacks: ${allFallbacks.length} location(s) using explicit fallback; inspect with 'velqu inspect fallbacks'`,
    );
  }

  const report: AppStrategyReport = {
    validation: validationSummary,
    responses: responseSummary,
    fallbacks: allFallbacks,
    decisions: decisionSummaries,
    evidenceSummary: {
      source: "M25-002-A/B/C codec benchmarks (run m25-002-c-1787293512)",
      referenceHost: "13th Gen Intel Core i5-13420H, Linux, quickjs-ng 0.15.1",
      smallObjectP50Us: { native: 21.3, quickjs: 30.1 },
      largeObject64kP50Us: { native: 45.5, quickjs: 277.8 },
      array1000P50Us: { native: 3004.1, quickjs: 2635.4 },
    },
    notes,
  };

  return { decisions, report };
}
