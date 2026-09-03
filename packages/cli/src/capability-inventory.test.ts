import { describe, expect, test } from "bun:test";
import { capabilityInventoryHash } from "../../compiler/src/emit";

describe("capability inventory hash (M27-002-C)", () => {
  test("mirrors the Rust canonical encoding on pinned vectors", () => {
    // Vectors computed independently (python reference) and pinned in
    // q-capabilities::inventory::canonical_hash_matches_cross_language_vectors.
    // Encoding: u32-le count, then per entry u16-le id-len + utf8 id + u32-le version.
    expect(capabilityInventoryHash([])).toBe(
      "df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119",
    );
    expect(
      capabilityInventoryHash([
        { id: "runtime:abort", version: 1 },
        { id: "runtime:text", version: 2 },
      ]),
    ).toBe("3a1b71efeb688d1d032f863fc32c9742fe9d3f54843c377b41cb2c2b5521f69e");
  });

  test("versions participate in the hash", () => {
    const v1 = capabilityInventoryHash([{ id: "runtime:timers", version: 1 }]);
    const v2 = capabilityInventoryHash([{ id: "runtime:timers", version: 2 }]);
    expect(v1).not.toBe(v2);
  });
});

import { resolveLinkedModules, KNOWN_GRANTS } from "../../compiler/src/emit";
import { inspectCapabilities } from "./capability-inspect";

describe("capability pruning (M27-002-D)", () => {
  test("no grants link zero modules", () => {
    expect(resolveLinkedModules([])).toEqual([]);
    expect(resolveLinkedModules(["timer"])).toEqual([{ id: "runtime:timers", version: 1 }]);
  });

  test("unknown grants fail closed naming the grant", () => {
    expect(() => resolveLinkedModules(["fetch"])).toThrow(/unknown capability 'fetch'/);
    expect(() => resolveLinkedModules(["node:fs"])).toThrow(/unknown capability 'node:fs'/);
  });

  test("grants dedupe to one module entry", () => {
    expect(resolveLinkedModules(["timer", "timer"])).toEqual([
      { id: "runtime:timers", version: 1 },
    ]);
    expect(KNOWN_GRANTS).toEqual(["timer", "postgres"]);
  });

  test("BETA-004-A: postgres grant requires runtime:postgres v1 exactly", () => {
    expect(resolveLinkedModules(["postgres"])).toEqual([
      { id: "runtime:postgres", version: 1 },
    ]);
    // mixing grants dedupes to the exact requirement set
    // output is id-sorted (canonical inventory order), deduped
    expect(resolveLinkedModules(["timer", "postgres", "timer"])).toEqual([
      { id: "runtime:postgres", version: 1 },
      { id: "runtime:timers", version: 1 },
    ]);
    expect(resolveLinkedModules(["postgres", "timer"])).toEqual([
      { id: "runtime:postgres", version: 1 },
      { id: "runtime:timers", version: 1 },
    ]);
  });
});

describe("inspect capabilities accuracy (M27-002-D)", () => {
  const base = { declared: ["timer"], perRoute: {}, nativeOps: {} };

  test("linked line reflects the pack inventory with verified hash", () => {
    const lines = inspectCapabilities({
      ...base,
      pack: {
        capabilityInventory: [{ id: "runtime:timers", version: 1 }],
        capabilityInventorySha256: capabilityInventoryHash([{ id: "runtime:timers", version: 1 }]),
      },
    });
    expect(lines.join("\n")).toContain("linked: runtime:timers@1");
    expect(lines.join("\n")).toContain("inventory sha256:");
  });

  test("empty inventory reports zero linked modules", () => {
    const lines = inspectCapabilities({
      ...base,
      declared: [],
      pack: { capabilityInventory: [], capabilityInventorySha256: capabilityInventoryHash([]) },
    });
    expect(lines.join("\n")).toContain("declared: (none)");
    expect(lines.join("\n")).toContain("(none — zero linked modules)");
  });

  test("hash mismatch fails loud instead of lying", () => {
    expect(() =>
      inspectCapabilities({
        ...base,
        pack: {
          capabilityInventory: [{ id: "runtime:timers", version: 1 }],
          capabilityInventorySha256: "ab".repeat(32),
        },
      }),
    ).toThrow(/hash mismatch/);
  });

  test("unsorted inventory fails loud", () => {
    expect(() =>
      inspectCapabilities({
        ...base,
        pack: {
          capabilityInventory: [
            { id: "runtime:text", version: 2 },
            { id: "runtime:abort", version: 1 },
          ],
          capabilityInventorySha256: capabilityInventoryHash([
            { id: "runtime:text", version: 2 },
            { id: "runtime:abort", version: 1 },
          ]),
        },
      }),
    ).toThrow(/not sorted/);
  });

  test("pre-inventory packs report unknown honestly", () => {
    const lines = inspectCapabilities({ ...base, pack: {} });
    expect(lines.join("\n")).toContain("linked: unknown (pack predates capability inventory)");
  });
});
