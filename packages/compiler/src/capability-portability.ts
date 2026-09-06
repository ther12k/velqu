/**
 * BWASM-C-005 — capability portability registry.
 *
 * Every capability grant carries exactly ONE portability state:
 *
 *   browser             — browser-only (no native counterpart needed)
 *   browser-and-native  — implemented on both targets with pinned parity
 *   simulated           — NEVER a default: only via an explicit
 *                         simulation profile; never silently mocked
 *   deployment-required — works only behind the native Velqu runtime;
 *                         the browser target refuses it at build time
 *                         and the runtime returns the stable
 *                         deployment-required problem before side effects
 *   forbidden           — no Velqu capability exists (payments, email,
 *                         webhooks, cron, durable queues); never mocked,
 *                         never granted
 *
 * Unknown names classify as "forbidden" — unknown classifications fail
 * closed. The same registry drives the compiler's build-time refusal
 * (browser-wasm target), the recorded states in `capability-manifest.json`,
 * the CLI's machine-readable deployment-requirements summary, and the
 * runtime's pre-side-effect policy checks — one classification, four
 * surfaces (acceptance: build, inspect, runtime, and Treaty agree).
 */

export type CapabilityPortability =
  | "browser"
  | "browser-and-native"
  | "simulated"
  | "deployment-required"
  | "forbidden";

export interface CapabilityPortabilityEntry {
  readonly state: CapabilityPortability;
  /** Safe remediation metadata — no secret or provider-private values. */
  readonly remediation: string;
}

/**
 * The portability registry. Grant names are the compiler's grant names
 * (KNOWN_GRANTS + the native capability graph's short names). Reserved
 * production-integration names are recorded as forbidden so app-builder
 * tooling can classify them without executing anything.
 */
export const CAPABILITY_PORTABILITY_REGISTRY: Readonly<Record<string, CapabilityPortabilityEntry>> =
  Object.freeze({
    // native graph + C-001 browser baseline (pinned parity: C-001's crypto
    // subset carries shared vectors; fetch redirect delta is documented)
    timer: { state: "browser-and-native", remediation: "available on both targets" },
    console: { state: "browser-and-native", remediation: "available on both targets" },
    crypto: { state: "browser-and-native", remediation: "pinned subset on browser (SHA-2 digests + random); full native set on native" },
    url: { state: "browser-and-native", remediation: "available on both targets" },
    text: { state: "browser-and-native", remediation: "available on both targets" },
    abort: { state: "browser-and-native", remediation: "available on both targets" },
    // C-004: browser-only preview persistence (never production-durable)
    kv: { state: "browser", remediation: "browser-local preview data; not durable or multi-user" },
    // server-only: requires the native runtime's linked pool (runtime:postgres)
    postgres: {
      state: "deployment-required",
      remediation: "deploy behind the native Velqu runtime with a linked Postgres pool; the browser target refuses this capability",
    },
    // reserved production-integration names: no capability exists — fail closed
    payments: { state: "forbidden", remediation: "no Velqu payments capability exists; integrate via your own server" },
    email: { state: "forbidden", remediation: "no Velqu email capability exists; integrate via your own server" },
    webhooks: { state: "forbidden", remediation: "no Velqu webhooks capability exists; integrate via your own server" },
    cron: { state: "forbidden", remediation: "no Velqu cron capability exists; use external scheduling against a deployed native service" },
    queues: { state: "forbidden", remediation: "no Velqu durable-queue capability exists; defer is bounded in-memory best-effort, never a durable queue" },
  });

/** Unknown names are FORBIDDEN — unknown classifications fail closed. */
export function classifyCapability(grant: string): CapabilityPortabilityEntry {
  const entry = CAPABILITY_PORTABILITY_REGISTRY[grant];
  if (entry) return entry;
  return {
    state: "forbidden",
    remediation: `unknown capability "${grant}" — unknown classifications fail closed (no handle, no simulation, no grant)`,
  };
}

export interface CapabilityPortabilityRouteInput {
  readonly id: string;
  readonly capabilities: ReadonlyArray<string>;
}

export interface CapabilityPortabilityEntryReport {
  readonly capability: string;
  readonly state: CapabilityPortability;
  readonly routes: ReadonlyArray<string>;
  readonly remediation: string;
}

export interface CapabilityPortabilityReport {
  /** True when at least one declared capability cannot run on the target. */
  readonly deploymentRequired: boolean;
  readonly entries: ReadonlyArray<CapabilityPortabilityEntryReport>;
}

/**
 * Machine-readable deployment-requirements summary for CLI and
 * app-builder UI (step 6): per-capability state + routes + remediation,
 * deterministically ordered. `deploymentRequired` is true when any
 * declared capability is deployment-required or forbidden — i.e. the
 * deployment cannot serve that route on the selected target.
 */
export function portabilityReport(
  routes: ReadonlyArray<CapabilityPortabilityRouteInput>,
  options?: { target?: "native" | "browser-wasm" },
): CapabilityPortabilityReport {
  const target = options?.target ?? "browser-wasm";
  const byCapability = new Map<string, { state: CapabilityPortability; routes: Set<string>; remediation: string }>();
  for (const route of routes) {
    for (const grant of route.capabilities) {
      const { state, remediation } = classifyCapability(grant);
      if (!byCapability.has(grant)) {
        byCapability.set(grant, { state, routes: new Set(), remediation });
      }
      byCapability.get(grant)!.routes.add(route.id);
    }
  }
  const nativeTarget = target === "native";
  const entries = [...byCapability.entries()]
    .sort(([a], [b]) => (a < b ? -1 : 1))
    .map(([capability, info]) => ({
      capability,
      state: info.state,
      routes: [...info.routes].sort(),
      remediation: info.remediation,
    }));
  // On the native target deployment-required capabilities are exactly the
  // native runtime's job — only forbidden blocks there.
  const deploymentRequired = entries.some(
    (e) => e.state === "forbidden" || (e.state === "deployment-required" && !nativeTarget),
  );
  return { deploymentRequired, entries };
}
