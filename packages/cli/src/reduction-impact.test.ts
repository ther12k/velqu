import { describe, expect, test } from "bun:test";
import {
  reductionImpacts,
  missingApisFor,
} from "../../compiler/src/reduction-impact";
import { inspectCapabilities } from "./capability-inspect";

describe("reduction impact diagnostics (M27-003-C)", () => {
  test("clean bundle: both reductions lose nothing", () => {
    const impacts = reductionImpacts(
      "export function h() { return Promise.resolve(JSON.parse('1')); }",
    );
    expect(impacts).toEqual([
      { profile: "web", missing: [] },
      { profile: "minimal", missing: [] },
    ]);
    expect(missingApisFor("minimal", "")).toEqual([]);
    expect(missingApisFor("full", "")).toEqual([]);
  });

  test("Date usage: web loses Date; minimal loses Date (sorted)", () => {
    const code = "export const t = [Date.now(), new Map()];";
    expect(reductionImpacts(code)).toEqual([
      { profile: "web", missing: ["Date"] },
      { profile: "minimal", missing: ["Date", "Map"] },
    ]);
    expect(missingApisFor("web", code)).toEqual(["Date"]);
    expect(missingApisFor("minimal", code)).toEqual(["Date", "Map"]);
  });

  test("web-only builtin usage: web misses nothing, minimal misses it", () => {
    const code = "export const s = new Set([1]);";
    expect(missingApisFor("web", code)).toEqual([]);
    expect(missingApisFor("minimal", code)).toEqual(["Set"]);
  });

  test("missing sorted deterministically across both boundaries", () => {
    const code = "export const x = [new WeakRef({}), performance, RegExp];";
    expect(missingApisFor("minimal", code)).toEqual([
      "RegExp",
      "WeakRef",
      "performance",
    ]);
  });
});

describe("inspect capabilities reports reduction diagnostics (M27-003-C)", () => {
  const base = { declared: [], perRoute: {}, nativeOps: {}, pack: {} };

  test("requirement and per-profile impact lines render", () => {
    const lines = inspectCapabilities({
      ...base,
      intrinsicRequirement: { requirement: "web" },
      reductionImpact: [
        { profile: "web", missing: [] },
        { profile: "minimal", missing: ["Map"] },
      ],
    }).join("\n");
    expect(lines).toContain("context requirement: web");
    expect(lines).toContain(
      "reduction to 'web': nothing the bundle uses would be lost",
    );
    expect(lines).toContain(
      "reduction to 'minimal': bundle uses dropped builtin(s): Map",
    );
  });

  test("absent diagnostics keep output unchanged", () => {
    const lines = inspectCapabilities(base).join("\n");
    expect(lines).not.toContain("context requirement");
    expect(lines).not.toContain("reduction to");
  });
});
