/**
 * Claims validation tests (BETA-005-C) — deterministic via the
 * injected clock. Seconds per RFC 7519 NumericDate.
 */
import { describe, expect, test } from "bun:test";
import { signJwt, verifyJwt } from "./index";
import { validateClaims, verifyJwtWithClaims, MAX_CLOCK_SKEW_MS } from "./claims";

const NOW = 1_800_000_000_000; // fixed epoch ms
const S = (ms: number) => Math.floor(ms / 1000);
const SECRET = "claims-test-secret";

function claimsWith(overrides: Record<string, unknown>): Record<string, unknown> {
  return { sub: "usr_1", exp: S(NOW + 3_600_000), ...overrides };
}

describe("expiry checks (BETA-005-C)", () => {
  test("valid exp passes; expired fails typed", () => {
    expect(validateClaims(claimsWith({}), { now: NOW }).ok).toBe(true);
    // 6s past the default 5s skew: expired
    const res = validateClaims(claimsWith({ exp: S(NOW - 6_000) }), { now: NOW });
    expect(res).toEqual({ ok: false, reason: "token-expired" });
  });

  test("missing or non-numeric exp fails typed (no eternal tokens)", () => {
    const { exp: _drop, ...withoutExp } = claimsWith({});
    expect(validateClaims(withoutExp, { now: NOW })).toEqual({
      ok: false,
      reason: "missing-exp",
    });
    expect(
      validateClaims(claimsWith({ exp: "soon" }), { now: NOW }),
    ).toEqual({ ok: false, reason: "exp-not-number" });
    expect(
      validateClaims(claimsWith({ exp: Number.NaN }), { now: NOW }),
    ).toEqual({ ok: false, reason: "exp-not-number" });
  });

  test("clock skew tolerance is bounded and injectable", () => {
    // 4s past expiry: inside the default 5s skew, outside a 1s skew
    const almostExpired = claimsWith({ exp: S(NOW - 4_000) });
    expect(validateClaims(almostExpired, { now: NOW }).ok).toBe(true);
    expect(
      validateClaims(almostExpired, { now: NOW, clockSkewMs: 1_000 }),
    ).toEqual({ ok: false, reason: "token-expired" });
    expect(
      validateClaims(claimsWith({}), {
        now: NOW,
        clockSkewMs: MAX_CLOCK_SKEW_MS + 1,
      }),
    ).toEqual({ ok: false, reason: "invalid-clock-skew" });
  });

  test("nbf: future tokens fail typed; malformed nbf fails typed", () => {
    const future = claimsWith({ nbf: S(NOW + 60_000) });
    expect(validateClaims(future, { now: NOW })).toEqual({
      ok: false,
      reason: "token-not-yet-valid",
    });
    const reached = claimsWith({ nbf: S(NOW - 60_000) });
    expect(validateClaims(reached, { now: NOW }).ok).toBe(true);
    expect(
      validateClaims(claimsWith({ nbf: "later" }), { now: NOW }),
    ).toEqual({ ok: false, reason: "nbf-not-number" });
  });
});

describe("issuer and audience checks (BETA-005-C)", () => {
  test("iss enforced only when expected: match, mismatch, missing", () => {
    const base = { iss: "https://issuer.example" };
    expect(
      validateClaims(claimsWith(base), {
        now: NOW,
        expectedIssuer: "https://issuer.example",
      }).ok,
    ).toBe(true);
    expect(
      validateClaims(claimsWith(base), {
        now: NOW,
        expectedIssuer: "https://other.example",
      }),
    ).toEqual({ ok: false, reason: "issuer-mismatch" });
    expect(
      validateClaims(claimsWith({}), {
        now: NOW,
        expectedIssuer: "https://issuer.example",
      }),
    ).toEqual({ ok: false, reason: "missing-iss" });
    // no expectation: absent iss is fine
    expect(validateClaims(claimsWith({}), { now: NOW }).ok).toBe(true);
  });

  test("aud string and array forms; missing and mismatch fail typed", () => {
    const single = { aud: "api://demo" };
    const multi = { aud: ["api://other", "api://demo"] };
    for (const aud of [single, multi]) {
      expect(
        validateClaims(claimsWith(aud), {
          now: NOW,
          expectedAudience: "api://demo",
        }).ok,
      ).toBe(true);
    }
    expect(
      validateClaims(claimsWith({ aud: "api://other" }), {
        now: NOW,
        expectedAudience: "api://demo",
      }),
    ).toEqual({ ok: false, reason: "audience-mismatch" });
    expect(
      validateClaims(claimsWith({}), {
        now: NOW,
        expectedAudience: "api://demo",
      }),
    ).toEqual({ ok: false, reason: "missing-aud" });
  });
});

describe("composition with the profile gates", () => {
  test("verifyJwtWithClaims: fresh token passes, expired fails typed", () => {
    const expired = signJwt({ sub: "usr_1", exp: S(NOW - 60_000) }, SECRET);
    expect(verifyJwtWithClaims(expired, SECRET, { now: NOW })).toEqual({
      ok: false,
      reason: "token-expired",
    });
    const fresh = signJwt({ sub: "usr_1", exp: S(NOW + 1_000_000) }, SECRET);
    const res = verifyJwtWithClaims(fresh, SECRET, { now: NOW });
    expect(res.ok).toBe(true);
    if (res.ok) expect(res.claims.sub).toBe("usr_1");
  });

  test("signature failures still win when the token is forged", () => {
    const forged = signJwt({ sub: "usr_1", exp: S(NOW + 1_000_000) }, "wrong");
    expect(verifyJwtWithClaims(forged, SECRET, { now: NOW })).toEqual({
      ok: false,
      reason: "signature-mismatch",
    });
  });
});
