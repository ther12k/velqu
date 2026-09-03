/**
 * Typed 401/403 problem tests (BETA-005-D) — deterministic; every
 * failure reason maps to the correct status class, type URI, and
 * WWW-Authenticate header.
 */
import { describe, expect, test } from "bun:test";
import { signJwt } from "./index";
import { authenticateBearer, authProblem, requireScope } from "./problems";

const SECRET = "problems-test-secret";
const NOW = 1_800_000_000_000;
const validToken = signJwt(
  { sub: "usr_1", scope: "demo", exp: Math.floor((NOW + 3_600_000) / 1000) },
  SECRET,
);

describe("authProblem mapping (BETA-005-D)", () => {
  test("authentication failures are 401 with WWW-Authenticate and typed URIs", () => {
    for (const reason of [
      "missing-token",
      "malformed",
      "algorithm-not-approved",
      "signature-mismatch",
      "token-expired",
      "issuer-mismatch",
    ] as const) {
      const p = authProblem(reason);
      expect(p.status).toBe(401);
      expect(p.type).toBe(`https://velqu.dev/problems/auth/${reason}`);
      expect(p.wwwAuthenticate).toBe('Bearer error="invalid_token"');
      expect(p.title.length).toBeGreaterThan(0);
    }
  });

  test("authorization failure is 403 insufficient_scope", () => {
    const p = authProblem("insufficient-scope");
    expect(p.status).toBe(403);
    expect(p.type).toBe("https://velqu.dev/problems/auth/insufficient-scope");
    expect(p.wwwAuthenticate).toBe('Bearer error="insufficient_scope"');
  });

  test("unknown reasons collapse to the generic invalid-token 401 (closed set)", () => {
    // @ts-expect-error — deliberate out-of-set probe
    const p = authProblem("mystery");
    expect(p.status).toBe(401);
    expect(p.type).toBe("https://velqu.dev/problems/auth/signature-mismatch");
  });
});

describe("authenticateBearer flow (BETA-005-D)", () => {
  test("valid token returns claims", () => {
    const res = authenticateBearer(`Bearer ${validToken}`, SECRET, { now: NOW });
    expect(res.ok).toBe(true);
    if (res.ok) expect(res.claims.sub).toBe("usr_1");
  });

  test("missing/malformed/expired tokens produce typed 401 problems", () => {
    const missing = authenticateBearer(undefined, SECRET, { now: NOW });
    expect(missing.ok).toBe(false);
    if (!missing.ok) {
      expect(missing.problem.status).toBe(401);
      expect(missing.problem.type).toContain("missing-token");
    }

    const malformed = authenticateBearer("Bearer not-a-token", SECRET, { now: NOW });
    expect(malformed.ok).toBe(false);
    if (!malformed.ok) {
      expect(malformed.problem.status).toBe(401);
      expect(malformed.problem.type).toContain("malformed");
    }

    const expired = signJwt(
      { sub: "usr_1", scope: "demo", exp: Math.floor((NOW - 60_000) / 1000) },
      SECRET,
    );
    const res = authenticateBearer(`Bearer ${expired}`, SECRET, { now: NOW });
    expect(res.ok).toBe(false);
    if (!res.ok) {
      expect(res.problem.status).toBe(401);
      expect(res.problem.type).toContain("token-expired");
      expect(res.problem.wwwAuthenticate).toBe('Bearer error="invalid_token"');
    }
  });

  test("algorithm-confused tokens produce 401 algorithm-not-approved", () => {
    // hand-built alg:none token signed however — rejected at the gate
    const b64 = (o: unknown) =>
      Buffer.from(JSON.stringify(o)).toString("base64url");
    const token = `${b64({ alg: "none", typ: "JWT" })}.${b64({ sub: "x" })}.x`;
    const res = authenticateBearer(`Bearer ${token}`, SECRET, { now: NOW });
    expect(res.ok).toBe(false);
    if (!res.ok) {
      expect(res.problem.status).toBe(401);
      expect(res.problem.type).toContain("algorithm-not-approved");
    }
  });
});

describe("requireScope authorization (BETA-005-D)", () => {
  test("valid token with insufficient scope yields 403, not 401", () => {
    const res = requireScope({ sub: "usr_1", scope: "demo" }, "admin");
    expect(res.ok).toBe(false);
    if (!res.ok) {
      expect(res.problem.status).toBe(403);
      expect(res.problem.type).toContain("insufficient-scope");
      expect(res.problem.detail).toBe("requires scope: admin");
    }
  });

  test("granted scope (single or among many) passes", () => {
    expect(requireScope({ scope: "admin" }, "admin").ok).toBe(true);
    expect(requireScope({ scope: "demo admin" }, "admin").ok).toBe(true);
    expect(requireScope({ scope: "demo" }, "demo").ok).toBe(true);
  });

  test("missing scope claim yields 403", () => {
    const res = requireScope({ sub: "usr_1" }, "demo");
    expect(res.ok).toBe(false);
    if (!res.ok) expect(res.problem.status).toBe(403);
  });
});
