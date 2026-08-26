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
