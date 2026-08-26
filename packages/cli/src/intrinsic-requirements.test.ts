import { describe, expect, test } from "bun:test";
import { compileIntrinsicRequirement } from "../../compiler/src/intrinsic-requirements";

function req(code: string) {
  return compileIntrinsicRequirement({ code, sourceMap: null });
}

describe("intrinsic requirement compilation (M27-003-B)", () => {
  test("clean bundle requires minimal", () => {
    const r = req(`const x = JSON.parse("1");
    export function handler(qs) { return Promise.resolve(parseInt(qs)); }`);
    expect(r.requirement).toBe("minimal");
    expect(r.used.dateOrPerformance).toEqual([]);
    expect(r.used.webOnlyBuiltins).toEqual([]);
  });

  test("Map/Set/Proxy/WeakRef/RegExp usage forces web", () => {
    for (const snippet of [
      "new Map([[1,2]])",
      "new Set([1])",
      "new Proxy({}, {})",
      "new WeakRef({})",
      "RegExp('x')",
    ]) {
      const r = req(`export function h() { return ${snippet}; }`);
      expect(r.requirement).toBe("web");
      expect(r.used.webOnlyBuiltins.length).toBeGreaterThan(0);
    }
  });

  test("Date or performance usage forces full", () => {
    for (const snippet of ["Date.now()", "performance.now()"]) {
      const r = req(`export function h() { return ${snippet}; }`);
      expect(r.requirement).toBe("full");
      expect(r.used.dateOrPerformance.length).toBeGreaterThan(0);
    }
  });

  test("full wins over web when both are touched", () => {
    const r = req("export const t = [new Set(), Date];");
    expect(r.requirement).toBe("full");
  });

  test("regex literals are lexically invisible — documented limitation", () => {
    // Only explicit `RegExp` references are detectable; slash literals
    // parse inside the engine, not in source text. Fail direction is
    // loud-at-runtime (never silent), serving keeps its default
    // `full` context until selection is measured (M27-011).
    const r = req("'a b'.replace(/a/, 'b')");
    expect(r.requirement).toBe("minimal");
  });

  test("word boundaries do not over-match identifiers", () => {
    // 'UpdateFoo' must not trigger on 'Date'; 'Mapped' must not hit 'Map'
    const r = req("class UpdateFoo {} const MappedThing = 1; export { UpdateFoo, MappedThing };");
    expect(r.requirement).toBe("minimal");
  });

  test("deterministic for identical inputs", () => {
    const code = "export const a = new Map(); export const d = Date;";
    expect(req(code)).toEqual(req(code));
  });
});
