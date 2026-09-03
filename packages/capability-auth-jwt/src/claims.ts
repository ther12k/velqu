/**
 * Claims validation: expiry / not-before / issuer / audience
 * (BETA-005-C).
 *
 * Layered on the BETA-005-A profile gates: authenticity (signature)
 * is established first; these checks then enforce claim-level
 * freshness and binding. Fail closed with typed reasons:
 *
 * - `exp` is REQUIRED (RFC 7519 NumericDate, seconds). A token without
 *   a numeric `exp` does not pass — eternal tokens do not exist here.
 * - `nbf`, when present, must be numeric and (with clock skew) reached.
 * - `iss`/`aud` are enforced only when the caller configures expected
 *   values; when configured, their absence in the token is itself a
 *   typed failure (a token that omits the claim does not slip past a
 *   configured expectation).
 *
 * The clock is injectable (`now`) so tests are deterministic; the
 * production caller passes nothing and gets `Date.now()`.
 */

import { verifyJwt } from "./index";

export const DEFAULT_CLOCK_SKEW_MS = 5_000;
/** Fail-closed ceiling on the skew allowance. */
export const MAX_CLOCK_SKEW_MS = 60_000;

export interface ClaimsValidationOptions {
  /** Token must carry `iss` equal to this value. */
  expectedIssuer?: string;
  /** Token `aud` (string or array) must include this value. */
  expectedAudience?: string;
  /** Clock-skew allowance in ms (default 5s, ceiling 60s). */
  clockSkewMs?: number;
  /** Injected clock (epoch ms). Defaults to Date.now(). */
  now?: number;
}

export type ClaimsFailure =
  | "missing-exp"
  | "exp-not-number"
  | "token-expired"
  | "nbf-not-number"
  | "token-not-yet-valid"
  | "missing-iss"
  | "issuer-mismatch"
  | "missing-aud"
  | "audience-mismatch"
  | "invalid-clock-skew";

export type ClaimsValidationResult =
  | { ok: true }
  | { ok: false; reason: ClaimsFailure };

export function validateClaims(
  claims: Record<string, unknown>,
  options: ClaimsValidationOptions = {},
): ClaimsValidationResult {
  const skew =
    options.clockSkewMs === undefined
      ? DEFAULT_CLOCK_SKEW_MS
      : options.clockSkewMs;
  if (
    typeof skew !== "number" ||
    !Number.isFinite(skew) ||
    skew < 0 ||
    skew > MAX_CLOCK_SKEW_MS
  ) {
    return { ok: false, reason: "invalid-clock-skew" };
  }
  const now = options.now ?? Date.now();

  // exp: required, numeric (seconds), not expired (skew-tolerant)
  const exp = claims.exp;
  if (exp === undefined) {
    return { ok: false, reason: "missing-exp" };
  }
  if (typeof exp !== "number" || !Number.isFinite(exp)) {
    return { ok: false, reason: "exp-not-number" };
  }
  if (now > exp * 1000 + skew) {
    return { ok: false, reason: "token-expired" };
  }

  // nbf: optional, numeric when present
  const nbf = claims.nbf;
  if (nbf !== undefined) {
    if (typeof nbf !== "number" || !Number.isFinite(nbf)) {
      return { ok: false, reason: "nbf-not-number" };
    }
    if (now < nbf * 1000 - skew) {
      return { ok: false, reason: "token-not-yet-valid" };
    }
  }

  // iss: enforced only when the caller expects one
  if (options.expectedIssuer !== undefined) {
    const iss = claims.iss;
    if (iss === undefined) {
      return { ok: false, reason: "missing-iss" };
    }
    if (iss !== options.expectedIssuer) {
      return { ok: false, reason: "issuer-mismatch" };
    }
  }

  // aud: enforced only when the caller expects one
  if (options.expectedAudience !== undefined) {
    const aud = claims.aud;
    if (aud === undefined) {
      return { ok: false, reason: "missing-aud" };
    }
    const matches =
      (typeof aud === "string" && aud === options.expectedAudience) ||
      (Array.isArray(aud) &&
        (aud as unknown[]).includes(options.expectedAudience));
    if (!matches) {
      return { ok: false, reason: "audience-mismatch" };
    }
  }

  return { ok: true };
}

/**
 * Composition helper: profile verification (signature/algorithm) then
 * claims validation (freshness/binding). Same result shapes as the
 * respective layers, with claims-validation reasons on the claims step.
 */
export function verifyJwtWithClaims(
  token: string,
  secret: string,
  options: ClaimsValidationOptions = {},
): { ok: true; claims: Record<string, unknown> } | { ok: false; reason: string } {
  const res = verifyJwt(token, secret);
  if (!res.ok) return res;
  const claims = validateClaims(res.claims, options);
  if (!claims.ok) return claims;
  return { ok: true, claims: res.claims };
}
