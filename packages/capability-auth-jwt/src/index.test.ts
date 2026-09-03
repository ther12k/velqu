/**
 * JWT capability tests (BETA-005-A): one approved algorithm profile.
 *
 * Deterministic coverage — HMAC vectors live in the proof policy tests
 * (RFC 4231); here the profile gates are proven: algorithm confusion is
 * structurally impossible, invalid tokens fail closed with typed
 * reasons, and the approved profile round-trips.
 */
import { describe, expect, test } from "bun:test";
import {
  APPROVED_ALGORITHM,
  hmacSha256,
  base64UrlEncode,
  base64UrlDecode,
  signJwt,
  verifyJwt,
} from "./index";

const SECRET = "beta-005-a-test-secret";
const HEADER = (alg: unknown, extra: Record<string, unknown> = {}): string =>
  base64UrlEncode(new TextEncoder().encode(JSON.stringify({ alg, typ: "JWT", ...extra })));
const CLAIMS = base64UrlEncode(
  new TextEncoder().encode(JSON.stringify({ sub: "usr_1", scope: "demo" })),
);

function tokenWith(header: string, signatureOver?: string): string {
  const sig = base64UrlEncode(
    hmacSha256(
      new TextEncoder().encode(SECRET),
      new TextEncoder().encode(signatureOver ?? `${header}.${CLAIMS}`),
    ),
  );
  return `${header}.${CLAIMS}.${sig}`;
}

describe("approved profile round-trip (BETA-005-A)", () => {
  test("signJwt -> verifyJwt succeeds and returns header + claims", () => {
    const token = signJwt({ sub: "usr_1", scope: "demo" }, SECRET);
    const res = verifyJwt(token, SECRET);
    expect(res.ok).toBe(true);
    if (res.ok) {
      expect(res.header.alg).toBe("HS256");
      expect(res.header.typ).toBe("JWT");
      expect(res.claims.sub).toBe("usr_1");
    }
  });

  test("HMAC-SHA-256 matches the RFC 4231 test case 2 vector", () => {
    // RFC 4231 TC2: key "Jefe", data "what do ya want for nothing?"
    const mac = base64UrlEncode(
      hmacSha256(
        new TextEncoder().encode("Jefe"),
        new TextEncoder().encode("what do ya want for nothing?"),
      ),
    );
    // digest 5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843
    expect(mac).toBe("W9zBRr9gdU5qBCQmCJV1x1oAPwidJzmDnexYuWTsOEM");
  });
});

describe("algorithm-confusion gates (BETA-005-A)", () => {
  test("alg none is rejected", () => {
    expect(verifyJwt(tokenWith(HEADER("none")), SECRET)).toEqual({
      ok: false,
      reason: "algorithm-not-approved",
    });
  });

  test("lowercase and case-variant alg are rejected", () => {
    for (const alg of ["hs256", "Hs256", "HS256 "]) {
      expect(verifyJwt(tokenWith(HEADER(alg)), SECRET)).toEqual({
        ok: false,
        reason: "algorithm-not-approved",
      });
    }
  });

  test("asymmetric algorithms are rejected even when signed with HMAC", () => {
    for (const alg of ["RS256", "ES256", "PS256"]) {
      expect(verifyJwt(tokenWith(HEADER(alg)), SECRET)).toEqual({
        ok: false,
        reason: "algorithm-not-approved",
      });
    }
  });

  test("missing alg is rejected", () => {
    const header = base64UrlEncode(new TextEncoder().encode(JSON.stringify({ typ: "JWT" })));
    expect(verifyJwt(tokenWith(header), SECRET)).toEqual({
      ok: false,
      reason: "algorithm-not-approved",
    });
  });

  test("non-string alg is rejected", () => {
    expect(verifyJwt(tokenWith(HEADER(42)), SECRET)).toEqual({
      ok: false,
      reason: "algorithm-not-approved",
    });
  });
});

describe("header-field gates (BETA-005-A)", () => {
  test("key-injection header fields are rejected (jku/jwk/x5u)", () => {
    for (const extra of [
      { jku: "https://attacker.example/keys" },
      { jwk: { kty: "oct" } },
      { x5u: "https://attacker.example/cert" },
      { kid: "key-1" },
    ]) {
      expect(verifyJwt(tokenWith(HEADER("HS256", extra)), SECRET)).toEqual({
        ok: false,
        reason: "header-field-not-allowed",
      });
    }
  });

  test("typ other than JWT is rejected", () => {
    const header = base64UrlEncode(
      new TextEncoder().encode(JSON.stringify({ alg: "HS256", typ: "JOSE" })),
    );
    expect(verifyJwt(tokenWith(header), SECRET)).toEqual({
      ok: false,
      reason: "typ-not-jwt",
    });
  });
});

describe("signature and structure gates (BETA-005-A)", () => {
  test("tampered payload fails the signature gate", () => {
    const token = signJwt({ sub: "usr_1", role: "user" }, SECRET);
    const [h, , sig] = token.split(".");
    const forged = base64UrlEncode(
      new TextEncoder().encode(JSON.stringify({ sub: "usr_1", role: "admin" })),
    );
    const res = verifyJwt(`${h}.${forged}.${sig}`, SECRET);
    expect(res).toEqual({ ok: false, reason: "signature-mismatch" });
  });

  test("wrong secret fails closed", () => {
    const token = signJwt({ sub: "usr_1" }, SECRET);
    expect(verifyJwt(token, "another-secret")).toEqual({
      ok: false,
      reason: "signature-mismatch",
    });
  });

  test("segment-count and empty-segment tokens are malformed", () => {
    expect(verifyJwt("", SECRET)).toEqual({ ok: false, reason: "malformed" });
    expect(verifyJwt("a.b.c.d", SECRET)).toEqual({ ok: false, reason: "malformed" });
    expect(verifyJwt("a..c", SECRET)).toEqual({ ok: false, reason: "malformed" });
  });

  test("non-object claims are rejected", () => {
    const body = base64UrlEncode(new TextEncoder().encode("[1,2,3]"));
    const header = HEADER("HS256");
    const sig = base64UrlEncode(
      hmacSha256(
        new TextEncoder().encode(SECRET),
        new TextEncoder().encode(`${header}.${body}`),
      ),
    );
    expect(verifyJwt(`${header}.${body}.${sig}`, SECRET)).toEqual({
      ok: false,
      reason: "claims-not-object",
    });
  });
});

describe("primitives", () => {
  test("base64url round-trips arbitrary bytes", () => {
    const bytes = new Uint8Array([0, 1, 2, 250, 251, 252, 253, 254, 255]);
    expect(base64UrlDecode(base64UrlEncode(bytes))).toEqual(bytes);
  });
});
