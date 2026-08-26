import { compileIntrinsicRequirement } from "./intrinsic-requirements";

/**
 * M27-003-C: missing-API diagnostics for context-profile reductions.
 *
 * Given a bundle, compute exactly which standard builtins would be
 * ABSENT under each reduced profile ("web", "minimal"), so builders
 * can see what a reduction would break before choosing it. Derived
 * from the same lexical scan as M27-003-B; same documented
 * limitation (regex literals are invisible to text scanning).
 */

/** All watched builtins and the profile that drops them. */
const DROPPED_BY_PROFILE: Record<"web" | "minimal", ReadonlyArray<string>> = {
  // web drops (relative to full): these are exactly B's boundary set
  web: ["Date", "performance"],
  // minimal additionally drops: B's second boundary
  minimal: ["Proxy", "Map", "Set", "WeakRef", "RegExp"],
};

export interface ReductionImpact {
  profile: "web" | "minimal";
  /**
   * Builtins the app actually touches that this profile drops.
   * Empty = this reduction is safe as far as lexical analysis sees.
   */
  missing: string[];
}

export function reductionImpacts(code: string): ReductionImpact[] {
  const webOnly = compileIntrinsicRequirement({ code, sourceMap: null }).used;
  const touchedWebDrop = webOnly.dateOrPerformance;
  const touchedMinimalOnly = webOnly.webOnlyBuiltins;
  return [
    {
      profile: "web",
      missing: touchedWebDrop,
    },
    {
      profile: "minimal",
      missing: [...touchedMinimalOnly, ...touchedWebDrop].sort(),
    },
  ];
}

export type RequestedProfile = "full" | "web" | "minimal";

/**
 * Which builtin names an application references that a candidate
 * profile does not provide. `full` never misses anything.
 */
export function missingApisFor(
  profile: RequestedProfile,
  code: string,
): string[] {
  if (profile === "full") return [];
  const impacts = reductionImpacts(code);
  const forProfile = impacts.find((i) => i.profile === profile)!;
  return forProfile.missing;
}

export { DROPPED_BY_PROFILE };
