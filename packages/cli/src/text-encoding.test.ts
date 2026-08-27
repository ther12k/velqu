import { describe, expect, test } from "bun:test";

describe("TextEncoder and TextDecoder baseline conformance (M27-006-A)", () => {
  test("TextEncoder encodes UTF-8 strings into Uint8Array", () => {
    const encoder = new TextEncoder();
    expect(encoder.encoding).toBe("utf-8");
    const bytes = encoder.encode("Hello, 🌍!");
    expect(bytes instanceof Uint8Array).toBe(true);
    expect(bytes.length).toBe(12);
  });

  test("TextEncoder encodeInto writes into preallocated buffer", () => {
    const encoder = new TextEncoder();
    const dest = new Uint8Array(10);
    const result = encoder.encodeInto("abc", dest);
    expect(result.read).toBe(3);
    expect(result.written).toBe(3);
    expect(dest[0]).toBe(97);
    expect(dest[1]).toBe(98);
    expect(dest[2]).toBe(99);
  });

  test("TextDecoder decodes valid UTF-8 byte arrays", () => {
    const decoder = new TextDecoder();
    expect(decoder.encoding).toBe("utf-8");
    const input = new Uint8Array([72, 101, 108, 108, 111]);
    expect(decoder.decode(input)).toBe("Hello");
  });

  test("TextDecoder fatal mode handles errors", () => {
    const fatalDecoder = new TextDecoder("utf-8", { fatal: true });
    expect(fatalDecoder.fatal).toBe(true);
    const nonFatalDecoder = new TextDecoder("utf-8");
    expect(nonFatalDecoder.fatal).toBe(false);
  });

  test("TextDecoder BOM handling options", () => {
    const normal = new TextDecoder("utf-8");
    expect(normal.ignoreBOM).toBe(false);
    const ignoreBOM = new TextDecoder("utf-8", { ignoreBOM: true });
    expect(ignoreBOM.ignoreBOM).toBe(true);
  });

  test("Invalid encoding label throws RangeError", () => {
    expect(() => new TextDecoder("invalid-encoding-label")).toThrow(RangeError);
  });

  describe("Invalid sequence replacement behavior (M27-006-B)", () => {
    test("non-fatal decoder replaces invalid bytes with U+FFFD", () => {
      const decoder = new TextDecoder("utf-8");
      // 0xFF is not a valid UTF-8 start byte
      const invalid = new Uint8Array([0x66, 0x6f, 0x6f, 0xff, 0x62, 0x61, 0x72]);
      expect(decoder.decode(invalid)).toBe("foo\uFFFDbar");
    });

    test("fatal decoder throws TypeError on invalid byte sequences", () => {
      const fatalDecoder = new TextDecoder("utf-8", { fatal: true });
      const invalid = new Uint8Array([0xff, 0xfe]);
      expect(() => fatalDecoder.decode(invalid)).toThrow(TypeError);
    });

    test("truncated multi-byte sequences replaced safely", () => {
      const decoder = new TextDecoder("utf-8");
      const truncated = new Uint8Array([0x61, 0xc2]);
      expect(decoder.decode(truncated)).toBe("a\uFFFD");
    });
  });

  describe("TypedArray ownership and view slicing (M27-006-C)", () => {
    test("decode from ArrayBufferView with non-zero byteOffset", () => {
      const buf = new Uint8Array([0, 0, 72, 101, 108, 108, 111, 0, 0]).buffer;
      const view = new Uint8Array(buf, 2, 5);
      const decoder = new TextDecoder("utf-8");
      expect(decoder.decode(view)).toBe("Hello");
    });

    test("decode directly from raw ArrayBuffer", () => {
      const buf = new Uint8Array([72, 105]).buffer;
      const decoder = new TextDecoder("utf-8");
      expect(decoder.decode(buf)).toBe("Hi");
    });

    test("decode from DataView", () => {
      const buf = new Uint8Array([0, 87, 111, 114, 108, 100]).buffer;
      const dv = new DataView(buf, 1, 5);
      const decoder = new TextDecoder("utf-8");
      expect(decoder.decode(dv)).toBe("World");
    });

    test("encodeInto into a subarray view", () => {
      const encoder = new TextEncoder();
      const dest = new Uint8Array(10);
      const sub = dest.subarray(2, 6);
      const res = encoder.encodeInto("testing", sub);
      // sub has length 4, "testing" is 7 chars; should write 4 bytes into sub (offset 2..6 in dest)
      expect(res.read).toBe(4);
      expect(res.written).toBe(4);
      expect(dest[0]).toBe(0);
      expect(dest[1]).toBe(0);
      expect(dest[2]).toBe(116); // 't'
      expect(dest[5]).toBe(116); // 't'
      expect(dest[6]).toBe(0);
    });

    test("decode with undefined input defaults to empty string", () => {
      const decoder = new TextDecoder();
      expect(decoder.decode()).toBe("");
      expect(decoder.decode(undefined)).toBe("");
    });
  });
});
