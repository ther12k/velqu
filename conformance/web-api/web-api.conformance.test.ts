/**
 * Web API Standards Conformance Suite (M27-010-A, M27-010-B, M27-010-D).
 *
 * Validates pinned WPT / WinterTC test vectors, records explicit skips/reasons,
 * and verifies unsupported/unadvertised APIs are documented and tracked:
 * 1. WHATWG URL / URLSearchParams
 * 2. WHATWG TextEncoder / TextDecoder (UTF-8)
 * 3. WHATWG AbortController / AbortSignal
 * 4. W3C Web Crypto random subset (getRandomValues / randomUUID)
 */

import { describe, expect, test } from "bun:test";
import { readFileSync } from "fs";
import { join } from "path";

const manifestPath = join(import.meta.dir, "wpt-manifest.json");
const manifest = JSON.parse(readFileSync(manifestPath, "utf-8"));

describe("Web API Standards Conformance (M27-010)", () => {
  describe("Pinned Manifest Structure", () => {
    test("manifest declares all M27 + M28 capability subsets", () => {
      expect(manifest.capabilities.url).toBeDefined();
      expect(manifest.capabilities.text_encoding).toBeDefined();
      expect(manifest.capabilities.abort).toBeDefined();
      expect(manifest.capabilities.crypto).toBeDefined();
      expect(manifest.capabilities.fetch).toBeDefined();
    });

    test("manifest declares valid WinterTC profiles", () => {
      for (const [key, cap] of Object.entries(manifest.capabilities) as [string, any][]) {
        expect(cap.standard).toBeDefined();
        expect(cap.profile).toContain("WinterTC");
        expect(cap.pinnedSubsets.length).toBeGreaterThan(0);
      }
    });

    test("manifest records explicit skips with standard reasons (M27-010-B)", () => {
      for (const [key, cap] of Object.entries(manifest.capabilities) as [string, any][]) {
        expect(cap.explicitSkips).toBeDefined();
        expect(cap.explicitSkips.length).toBeGreaterThan(0);
        for (const skip of cap.explicitSkips) {
          expect(skip.id).toBeDefined();
          expect(skip.name).toBeDefined();
          expect(skip.standardReference).toBeDefined();
          expect(skip.reasonCode).toBeDefined();
          expect(skip.reason.length).toBeGreaterThan(10);
          expect(skip.deferredTo).toBeDefined();
        }
      }
    });
  });

  describe("URL & URLSearchParams Pinned Vectors", () => {
    const urlSubsets = manifest.capabilities.url.pinnedSubsets;

    test("relative URL resolution subset", () => {
      const subset = urlSubsets.find((s: any) => s.id === "wpt-url-resolution");
      for (const tc of subset.cases) {
        const u = new URL(tc.input, tc.base);
        expect(u.href).toBe(tc.expected);
      }
    });

    test("URL normalization subset", () => {
      const subset = urlSubsets.find((s: any) => s.id === "wpt-url-normalization");
      for (const tc of subset.cases) {
        const u = new URL(tc.input);
        if (tc.expectedOrigin !== undefined) expect(u.origin).toBe(tc.expectedOrigin);
        if (tc.expectedPort !== undefined) expect(u.port).toBe(tc.expectedPort);
        if (tc.expectedHost !== undefined) expect(u.host).toBe(tc.expectedHost);
        if (tc.expectedHostname !== undefined) expect(u.hostname).toBe(tc.expectedHostname);
        if (tc.expectedPathname !== undefined) expect(u.pathname).toBe(tc.expectedPathname);
      }
    });

    test("URLSearchParams mutation and querying subset", () => {
      const subset = urlSubsets.find((s: any) => s.id === "wintertc-urlsearchparams");
      for (const tc of subset.cases) {
        const sp = new URLSearchParams(tc.init);
        if (tc.get) expect(sp.get(tc.get.key)).toBe(tc.get.expected);
        if (tc.getAll) expect(sp.getAll(tc.getAll.key)).toEqual(tc.getAll.expected);
        if (tc.has) expect(sp.has(tc.has.key)).toBe(tc.has.expected);
        if (tc.append) {
          sp.append(tc.append.key, tc.append.value);
          expect(sp.toString()).toBe(tc.expectedString);
        }
        if (tc.sort) {
          sp.sort();
          expect(sp.toString()).toBe(tc.expectedString);
        }
        if (tc.delete) {
          sp.delete(tc.delete.key);
          expect(sp.toString()).toBe(tc.expectedString);
        }
      }
    });
  });

  describe("TextEncoder & TextDecoder Pinned Vectors", () => {
    const textSubsets = manifest.capabilities.text_encoding.pinnedSubsets;

    test("TextEncoder UTF-8 encoding subset", () => {
      const subset = textSubsets.find((s: any) => s.id === "wpt-textencoder-utf8");
      const enc = new TextEncoder();
      for (const tc of subset.cases) {
        const bytes = enc.encode(tc.input);
        expect(Array.from(bytes)).toEqual(tc.expectedBytes);
      }
    });

    test("TextDecoder UTF-8 decoding subset", () => {
      const subset = textSubsets.find((s: any) => s.id === "wpt-textdecoder-utf8");
      for (const tc of subset.cases) {
        const dec = new TextDecoder("utf-8", tc.options);
        const bytes = new Uint8Array(tc.bytes);
        if (tc.expectedError) {
          expect(() => dec.decode(bytes)).toThrow();
        } else {
          expect(dec.decode(bytes)).toBe(tc.expected);
        }
      }
    });
  });

  describe("AbortController & AbortSignal Pinned Vectors", () => {
    const abortSubsets = manifest.capabilities.abort.pinnedSubsets;

    test("AbortController basic abort subset", () => {
      const subset = abortSubsets.find((s: any) => s.id === "wpt-abortcontroller-basic");
      for (const tc of subset.cases) {
        const ctrl = new AbortController();
        expect(ctrl.signal.aborted).toBe(false);
        if (tc.customReason) {
          ctrl.abort(tc.customReason);
          expect(ctrl.signal.aborted).toBe(true);
          expect(ctrl.signal.reason).toBe(tc.expectedReason);
        } else {
          ctrl.abort();
          expect(ctrl.signal.aborted).toBe(true);
          expect(ctrl.signal.reason).toBeDefined();
        }
      }
    });

    test("AbortSignal static factories subset", async () => {
      const subset = abortSubsets.find((s: any) => s.id === "wpt-abortsignal-static");
      for (const tc of subset.cases) {
        if (tc.factory === "AbortSignal.abort") {
          const sig = AbortSignal.abort(tc.reason);
          expect(sig.aborted).toBe(true);
          expect(sig.reason).toBe(tc.reason);
        } else if (tc.factory === "AbortSignal.timeout") {
          const sig = AbortSignal.timeout(tc.delayMs);
          expect(sig.aborted).toBe(false);
          await new Promise((r) => setTimeout(r, tc.delayMs + 20));
          expect(sig.aborted).toBe(true);
        }
      }
    });
  });

  describe("Web Crypto Random Pinned Vectors", () => {
    const cryptoSubsets = manifest.capabilities.crypto.pinnedSubsets;

    test("crypto.getRandomValues subset", () => {
      const subset = cryptoSubsets.find((s: any) => s.id === "wpt-crypto-getrandomvalues");
      for (const tc of subset.cases) {
        if (tc.expectedError === "TypeError") {
          if (tc.typedArray === "Float32Array") {
            const arr = new Float32Array(tc.length);
            expect(() => crypto.getRandomValues(arr as any)).toThrow();
          } else if (tc.typedArray === "DataView") {
            const dv = new DataView(new ArrayBuffer(tc.length));
            expect(() => crypto.getRandomValues(dv as any)).toThrow();
          }
        } else if (tc.expectedFilled) {
          if (tc.typedArray === "Uint8Array") {
            const arr = new Uint8Array(tc.length);
            crypto.getRandomValues(arr);
            const nonZero = Array.from(arr).some((b) => b !== 0);
            expect(nonZero).toBe(true);
          } else if (tc.typedArray === "Int32Array") {
            const arr = new Int32Array(tc.length);
            crypto.getRandomValues(arr);
            const nonZero = Array.from(arr).some((b) => b !== 0);
            expect(nonZero).toBe(true);
          }
        }
      }
    });

    test("crypto.randomUUID format and uniqueness subset", () => {
      const subset = cryptoSubsets.find((s: any) => s.id === "wpt-crypto-randomuuid");
      const tc = subset.cases[0];
      const re = new RegExp(tc.pattern);
      const set = new Set<string>();
      for (let i = 0; i < 50; i++) {
        const id = crypto.randomUUID();
        expect(re.test(id)).toBe(true);
        set.add(id);
      }
      expect(set.size).toBe(50);
    });
  });

  describe("Unsupported APIs Enforcement (M27-010-D, M28-001-C)", () => {
    test("unsupported Web API skips are all classified with valid reason codes", () => {
      const validCodes = new Set([
        "BROWSER_ONLY_FEATURE",
        "POSIX_RUNTIME_TARGET",
        "WINTERTC_UTF8_ONLY",
        "STREAMING_DEFERRED",
        "ASYNC_COMBINATOR_DEFERRED",
        "MINIMAL_EVENT_TARGET",
        "UNSUPPORTED_CRYPTO_SUBTLE",
        "SPEC_MANDATED_TYPE_ERROR",
        "NO_WEBSOCKET_BETA",
        "NO_SSE_BETA",
        "NO_STREAMING_REQUEST_BODIES",
        "NO_FORMDATA_MULTIPART",
        "NO_BLOB_FILE_TYPES",
        "NO_SERVICE_WORKERS",
        "NO_XHR",
        "NO_HTTP2_UPSTREAM",
        "NO_CLIENT_CERTS",
        "NO_SOCKS_PROXY",
        "NO_COOKIE_JAR",
        "NO_BR_ZSTD",
      ]);
      for (const cap of Object.values(manifest.capabilities) as any[]) {
        for (const skip of cap.explicitSkips) {
          expect(validCodes.has(skip.reasonCode)).toBe(true);
        }
      }
    });
  });

  describe("Fetch Security Policy Manifest (M28-001-C)", () => {
    test("fetch capability pins an executable security-policy subset", () => {
      const fetch = manifest.capabilities.fetch;
      expect(fetch.pinnedSubsets.length).toBeGreaterThan(0);
      const subset = fetch.pinnedSubsets.find((s: any) => s.id === "fetch-policy-security");
      expect(subset).toBeDefined();
      // The vectors execute in Rust against q_capabilities::fetch_policy,
      // not against Bun globals (engine divergence) — the pointer must be
      // explicit so the report cannot imply a Bun-side run.
      expect(subset.verifiedIn).toBe("crates/q-capabilities/tests/wpt_wintertc_conformance.rs");
      expect(subset.cases.length).toBeGreaterThanOrEqual(23);
      const checks = new Set(subset.cases.map((c: any) => c.check));
      expect(checks.has("scheme")).toBe(true);
      expect(checks.has("address")).toBe(true);
      expect(checks.has("redirect")).toBe(true);
      for (const c of subset.cases) {
        expect(c.expect === "allow" || c.expect === "deny").toBe(true);
      }
      // Both directions present.
      expect(subset.cases.some((c: any) => c.expect === "allow")).toBe(true);
      expect(subset.cases.some((c: any) => c.expect === "deny")).toBe(true);
    });

    test("fetch unsupported features are frozen with deferral targets", () => {
      const fetch = manifest.capabilities.fetch;
      const byId = new Map(fetch.explicitSkips.map((s: any) => [s.id, s]));
      // The headline non-goals from the M28 evidence list.
      expect(byId.has("wpt-fetch-websockets")).toBe(true);
      expect(byId.has("wpt-fetch-sse")).toBe(true);
      for (const skip of fetch.explicitSkips) {
        expect(skip.standardReference).toBeDefined();
        expect(skip.reason.length).toBeGreaterThan(10);
        expect(["OUT_OF_SCOPE", "POST_M28", "POST_BETA", "GA_TRACK"]).toContain(skip.deferredTo);
      }
    });
  });
});
