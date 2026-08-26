import type { BundleResult } from "./emit";

/**
 * M27-003-B: compile the application's intrinsic requirement.
 *
 * Scans the bundled application source (which includes all bundled
 * helpers) for references to the standard builtins each non-full
 * profile drops. Any hit forces a stronger profile — fail-safe
 * direction: we never select a reduction the app cannot survive.
 * Detection is lexical over bundle output, so it is deterministic
 * for the same inputs by construction.
 * Limitation: regex *literals* are invisible to a text scan — only
 * explicit `RegExp` references are caught; misses fail loudly at
 * runtime on a reduced profile (never silently). Serving keeps its
 * configured default `full` context until selection is measured
 * (M27-011); full-profile compatibility is retained by design
 * (M27-003-D).
 *
 * Profile semantics mirror `q-engine-quickjs::ContextProfile`:
 * - full    keeps everything;
 * - web     additionally drops Date and performance;
 * - minimal additionally drops Proxy, Map/Set, WeakRef and RegExp
 *   (keeping Eval, JSON, Promise, TypedArrays — the host bridge).
 */

/** Builtins dropped by `web` relative to `full`. */
const WEB_DROPS = ["Date", "performance"] as const;
/** Builtins dropped by `minimal` relative to `web`. */
const MINIMAL_DROPS = ["Proxy", "Map", "Set", "WeakRef", "RegExp"] as const;

export interface IntrinsicRequirement {
  /** Smallest profile whose kept builtin set covers what the app touches. */
  requirement: "full" | "web" | "minimal";
  /** Which watched builtins appeared in the bundle (per profile boundary). */
  used: { dateOrPerformance: string[]; webOnlyBuiltins: string[] };
}

function findUsages(code: string, names: ReadonlyArray<string>): string[] {
  const hits: string[] = [];
  for (const name of names) {
    // Word-boundary scan; `\b` is sufficient to separate identifiers in
    // minified code and cannot miss usages (only over-approximate via
    // comments/strings — safe direction).
    if (new RegExp(`\\b${name}\\b`).test(code)) hits.push(name);
  }
  return hits;
}

export function compileIntrinsicRequirement(bundle: BundleResult): IntrinsicRequirement {
  const webOnly = findUsages(bundle.code, MINIMAL_DROPS);
  const datePerf = findUsages(bundle.code, WEB_DROPS);
  const requirement: IntrinsicRequirement["requirement"] =
    datePerf.length > 0 ? "full" : webOnly.length > 0 ? "web" : "minimal";
  return {
    requirement,
    used: { dateOrPerformance: datePerf, webOnlyBuiltins: webOnly },
  };
}
