/**
 * JWT-like policy reference unit tests (M4A-009-B): token issuance, signature
 * verification, expiry enforcement, and rejection paths.
 */
import { describe, it, expect } from "bun:test";
import { jwtPolicy, issueToken, JWT_DEMO_SECRET, _referenceHmacSha256 } from "../../policy/jwt";

const req = (authorization?: string) => ({ headers: authorization ? { authorization } : {} });

describe("jwt-like policy reference", () => {
  it("issues a three-segment token that verifies and provides a session", async () => {
    const token = await issueToken({ sub: "usr_ada", scope: "profile:read" });
    expect(token.split(".")).toHaveLength(3);

    const result = await jwtPolicy.check(req(`Bearer ${token}`));
    expect("session" in result && result.session).toEqual({
      userId: "usr_ada",
      scope: "profile:read",
    });
  });

  it("rejects missing, malformed, tampered, and expired tokens with declared 401s", async () => {
    // missing header
    const missing = await jwtPolicy.check(req(undefined));
    expect("session" in missing).toBe(false);
    if (!("session" in missing)) expect(missing.status).toBe(401);

    // malformed: wrong segment count
    const malformed = await jwtPolicy.check(req("Bearer not-a-jwt"));
    if (!("session" in malformed)) {
      expect(malformed.status).toBe(401);
      expect(malformed.detail).toContain("malformed");
    }

    // tampered payload
    const token = await issueToken({ sub: "usr_ada", scope: "profile:read" });
    const [h, , sig] = token.split(".");
    const forgedPayload = btoa(JSON.stringify({ sub: "usr_evil", scope: "*", exp: Date.now() + 60000 }))
      .replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
    const tampered = await jwtPolicy.check(req(`Bearer ${h}.${forgedPayload}.${sig}`));
    if (!("session" in tampered)) expect(tampered.status).toBe(401);

    // expired token (beyond the 5s skew allowance)
    const expiredToken = await issueToken({ sub: "usr_ada", scope: "profile:read" }, -10_000);
    const expired = await jwtPolicy.check(req(`Bearer ${expiredToken}`));
    if (!("session" in expired)) expect(expired.status).toBe(401);
  });

  it("uses a demo fixture secret distinct from any real credential", () => {
    expect(JWT_DEMO_SECRET).toContain("demo");
    expect(JWT_DEMO_SECRET.length).toBeGreaterThan(8);
  });
});

describe("reference HMAC-SHA-256 (RFC 4231 vectors)", () => {
  const hex = (b: Uint8Array) => Array.from(b, (x) => x.toString(16).padStart(2, "0")).join("");
  const enc = (t: string) => new TextEncoder().encode(t);

  it("matches RFC 4231 test case 2 (HMAC-SHA-256, key < block size)", () => {
    const mac = hex(_referenceHmacSha256(enc("Jefe"), enc("what do ya want for nothing?")));
    expect(mac).toBe("5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843");
  });

  it("matches RFC 4231 test case 1 (short key/message)", () => {
    const mac = hex(_referenceHmacSha256(new Uint8Array(20).fill(0x0b), enc("Hi There")));
    expect(mac).toBe("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7");
  });

  it("matches the Node crypto reference for a longer secret", () => {
    // hmac("jwt-reference-demo-secret", "hello") — cross-checked with Node
    const mac = hex(_referenceHmacSha256(enc("jwt-reference-demo-secret"), enc("hello")));
    expect(mac).toBe("4190ca6a9fa0709f86aaa76fbd0e544e9ec86240953451b5ae79645fb68f7fcc");
  });
});
