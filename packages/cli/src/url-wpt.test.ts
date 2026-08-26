import { describe, expect, test } from "bun:test";

describe("WPT / WinterTC URL and URLSearchParams test suite (M27-005-B)", () => {
  describe("URL resolution and components", () => {
    test("basic resolution against base URL", () => {
      const u = new URL("b/c", "http://example.org/a/");
      expect(u.href).toBe("http://example.org/a/b/c");
      expect(u.pathname).toBe("/a/b/c");
    });

    test("relative dot and double-dot paths", () => {
      const base = "http://example.org/a/b/c/";
      expect(new URL("../d", base).href).toBe("http://example.org/a/b/d");
      expect(new URL("../../d", base).href).toBe("http://example.org/a/d");
      expect(new URL("./d", base).href).toBe("http://example.org/a/b/c/d");
    });

    test("query and hash replacement", () => {
      const base = "http://example.org/path?q=1#h1";
      const u1 = new URL("?q=2", base);
      expect(u1.href).toBe("http://example.org/path?q=2");
      const u2 = new URL("#h2", base);
      expect(u2.href).toBe("http://example.org/path?q=1#h2");
    });

    test("default port omission in href and host", () => {
      expect(new URL("http://example.com:80/").host).toBe("example.com");
      expect(new URL("https://example.com:443/").host).toBe("example.com");
      expect(new URL("http://example.com:8080/").host).toBe("example.com:8080");
    });

    test("canParse validation", () => {
      expect(URL.canParse("https://example.com")).toBe(true);
      expect(URL.canParse("relative", "https://example.com")).toBe(true);
      expect(URL.canParse("")).toBe(false);
      expect(URL.canParse("http://")).toBe(false);
    });
  });

  describe("WinterTC URLSearchParams behavior", () => {
    test("empty and key-only query pairs", () => {
      const sp = new URLSearchParams("a=&b");
      expect(sp.get("a")).toBe("");
      expect(sp.get("b")).toBe("");
      expect(sp.has("a")).toBe(true);
      expect(sp.has("b")).toBe(true);
    });

    test("plus and percent encoding decoding", () => {
      const sp = new URLSearchParams("query=hello+world%26more");
      expect(sp.get("query")).toBe("hello world&more");
    });

    test("append, set, delete, getAll mutations", () => {
      const sp = new URLSearchParams();
      sp.append("tag", "alpha");
      sp.append("tag", "beta");
      expect(sp.getAll("tag")).toEqual(["alpha", "beta"]);

      sp.set("tag", "gamma");
      expect(sp.getAll("tag")).toEqual(["gamma"]);

      sp.delete("tag");
      expect(sp.has("tag")).toBe(false);
    });

    test("sorting parameters", () => {
      const sp = new URLSearchParams("z=1&a=2&m=3&a=1");
      sp.sort();
      expect(sp.toString()).toBe("a=2&a=1&m=3&z=1");
    });

    test("iteration and entries", () => {
      const sp = new URLSearchParams("a=1&b=2");
      expect([...sp.entries()]).toEqual([["a", "1"], ["b", "2"]]);
      expect([...sp.keys()]).toEqual(["a", "b"]);
      expect([...sp.values()]).toEqual(["1", "2"]);
    });
  });

  describe("Host and path encoding behavior (M27-005-C)", () => {
    test("path percent encoding normalizes spaces and special characters", () => {
      const u = new URL("https://example.com/hello world/path#frag");
      expect(u.pathname).toBe("/hello%20world/path");
      expect(u.href).toBe("https://example.com/hello%20world/path#frag");
    });

    test("IDNA host normalization", () => {
      const u = new URL("https://bücher.example/test");
      expect(u.hostname).toBe("xn--bcher-kva.example");
      expect(u.origin).toBe("https://xn--bcher-kva.example");
    });

    test("IPv4 and IPv6 host normalizations", () => {
      const u1 = new URL("http://127.0.0.1:3000/api");
      expect(u1.hostname).toBe("127.0.0.1");

      const u2 = new URL("http://[::1]:8080/api");
      expect(u2.hostname).toBe("[::1]");
      expect(u2.host).toBe("[::1]:8080");
    });
  });
});
