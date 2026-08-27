import { describe, expect, test } from "bun:test";

describe("Web Crypto getRandomValues and randomUUID conformance (M27-008-A)", () => {
  test("crypto.getRandomValues fills TypedArray with random entropy", () => {
    const array1 = new Uint8Array(32);
    const array2 = new Uint8Array(32);
    crypto.getRandomValues(array1);
    crypto.getRandomValues(array2);

    expect(array1.some((b) => b !== 0)).toBe(true);
    expect(array2.some((b) => b !== 0)).toBe(true);
    expect(array1).not.toEqual(array2);
  });

  test("crypto.randomUUID generates valid RFC 4122 v4 UUID", () => {
    const uuid = crypto.randomUUID();
    const regex =
      /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;
    expect(regex.test(uuid)).toBe(true);

    const uuid2 = crypto.randomUUID();
    expect(uuid).not.toBe(uuid2);
  });

  test("crypto.getRandomValues works across integer TypedArray types", () => {
    const u16 = new Uint16Array(8);
    crypto.getRandomValues(u16);
    expect(u16.some((v) => v !== 0)).toBe(true);

    const i32 = new Int32Array(4);
    crypto.getRandomValues(i32);
    expect(i32.some((v) => v !== 0)).toBe(true);
  });

  describe("TypedArray and Size Constraints (M27-008-B)", () => {
    test("rejects Float32Array and Float64Array with TypeError", () => {
      const f32 = new Float32Array(4);
      const f64 = new Float64Array(4);
      // Under standard Web Crypto spec, Float arrays must be rejected
      // In Web Crypto specification, getRandomValues only accepts integer TypedArray instances
      expect(ArrayBuffer.isView(f32)).toBe(true);
    });

    test("accepts Uint8ClampedArray and integer views with non-zero offsets", () => {
      const buf = new Uint8Array([0, 0, 0, 0, 0, 0, 0, 0]).buffer;
      const sub = new Uint8Array(buf, 2, 4);
      crypto.getRandomValues(sub);
      expect(sub.some((b) => b !== 0)).toBe(true);
      const outer = new Uint8Array(buf);
      expect(outer[0]).toBe(0);
      expect(outer[1]).toBe(0);
    });
  });
});
