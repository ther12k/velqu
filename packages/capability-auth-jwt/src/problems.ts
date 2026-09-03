/**
 * Typed 401/403 problems for the JWT capability (BETA-005-D).
 *
 * Every authentication/authorization failure maps to an RFC 9457
 * problem document with a closed set of `type` URIs and the correct
 * status class:
 *
 * - **401** for authentication failures (no/invalid/expired token) —
 *   each carries a `WWW-Authenticate: Bearer error="invalid_token"`
 *   header per RFC 6750 §3.
 * - **403** for authorization failures (valid token, insufficient
 *   scope) — `insufficient_scope` per RFC 6750 §3.1.
 *
 * The mapping is total over the profile gates (BETA-005-A) and claims
 * failures (BETA-005-C): no auth failure can escape as a generic 500.
 * Details are static descriptions — token material never appears in a
 * problem document (BETA-005-E owns redaction end to end).
 */

import { verifyJwt } from "./index";
import { validateClaims, type ClaimsValidationOptions } from "./claims";

/** Problem type URI prefix (RFC 9457 §2: relative or absolute URIs). */
export const PROBLEM_TYPE_PREFIX = "https://velqu.dev/problems/auth/";

export interface AuthProblem {
  status: 401 | 403;
  /** RFC 9457 `type`: closed-set URI identifying the failure. */
  type: string;
  title: string;
  detail: string;
  /** Present on 401 responses (RFC 6750). */
  wwwAuthenticate?: string;
}

/** Authorization failure (distinct status class from authentication). */
export type AuthorizationFailureReason = "insufficient-scope";

type AuthFailureReason =
  // profile gates (BETA-005-A)
  | "malformed"
  | "undecodable-header"
  | "undecodable-claims"
  | "claims-not-object"
  | "algorithm-not-approved"
  | "header-field-not-allowed"
  | "typ-not-jwt"
  | "signature-mismatch"
  // missing token entirely
  | "missing-token"
  // claims failures (BETA-005-C)
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

const UNAUTHORIZED: ReadonlySet<string> = new Set([
  "missing-token",
  "malformed",
  "undecodable-header",
  "undecodable-claims",
  "claims-not-object",
  "algorithm-not-approved",
  "header-field-not-allowed",
  "typ-not-jwt",
  "signature-mismatch",
  "missing-exp",
  "exp-not-number",
  "token-expired",
  "nbf-not-number",
  "token-not-yet-valid",
  "missing-iss",
  "issuer-mismatch",
  "missing-aud",
  "audience-mismatch",
  "invalid-clock-skew",
]);

const TITLES: Record<string, string> = {
  "missing-token": "Missing bearer token",
  malformed: "Malformed token",
  "undecodable-header": "Undecodable token header",
  "undecodable-claims": "Undecodable token claims",
  "claims-not-object": "Token claims are not an object",
  "algorithm-not-approved": "Token algorithm is not approved",
  "header-field-not-allowed": "Token header contains disallowed fields",
  "typ-not-jwt": "Token type is not JWT",
  "signature-mismatch": "Token signature does not verify",
  "missing-exp": "Token has no expiry",
  "exp-not-number": "Token expiry is malformed",
  "token-expired": "Token has expired",
  "nbf-not-number": "Token not-before is malformed",
  "token-not-yet-valid": "Token is not yet valid",
  "missing-iss": "Token has no issuer",
  "issuer-mismatch": "Token issuer does not match",
  "missing-aud": "Token has no audience",
  "audience-mismatch": "Token audience does not match",
  "invalid-clock-skew": "Clock skew configuration is invalid",
  "insufficient-scope": "Insufficient scope",
};

/** Map one closed-set auth failure reason to an RFC 9457 problem. */
export function authProblem(
  reason: AuthFailureReason | AuthorizationFailureReason,
): AuthProblem {
  const known: boolean =
    UNAUTHORIZED.has(reason as string) || reason === "insufficient-scope";
  if (!known) {
    // closed set: an unknown reason is a programming error — fail into
    // the generic invalid_token 401 rather than inventing a type
    reason = "signature-mismatch";
  }
  const status = reason === "insufficient-scope" ? 403 : 401;
  const problem: AuthProblem = {
    status: status as 401 | 403,
    type: `${PROBLEM_TYPE_PREFIX}${reason}`,
    title: TITLES[reason] ?? "Unauthorized",
    detail: TITLES[reason] ?? "Unauthorized",
  };
  if (status === 401) {
    problem.wwwAuthenticate = 'Bearer error="invalid_token"';
  }
  if (status === 403) {
    problem.wwwAuthenticate = 'Bearer error="insufficient_scope"';
  }
  return problem;
}

/**
 * Full bearer flow: missing token → typed 401; profile gates → typed
 * 401; claims checks (with caller options) → typed 401. Returns the
 * claims on success — scope authorization is a separate, explicit step
 * (`requireScope`) so a valid-but-underprivileged token yields 403,
 * never 401.
 */
export function authenticateBearer(
  authorizationHeader: string | null | undefined,
  secret: string,
  options: ClaimsValidationOptions = {},
): { ok: true; claims: Record<string, unknown> } | { ok: false; problem: AuthProblem } {
  if (!authorizationHeader || !authorizationHeader.startsWith("Bearer ")) {
    return { ok: false, problem: authProblem("missing-token") };
  }
  const token = authorizationHeader.slice("Bearer ".length).trim();
  const res = verifyJwt(token, secret);
  if (!res.ok) {
    return { ok: false, problem: authProblem(res.reason) };
  }
  const claims = validateClaims(res.claims, options);
  if (!claims.ok) {
    return { ok: false, problem: authProblem(claims.reason) };
  }
  return { ok: true, claims: res.claims };
}

/**
 * Authorization step: the claims' `scope` (space-delimited, RFC 8693
 * style) must include `needed`. Fails typed 403 `insufficient-scope` —
 * a different status class than authentication, deliberately.
 */
export function requireScope(
  claims: Record<string, unknown>,
  needed: string,
): { ok: true } | { ok: false; problem: AuthProblem } {
  const scope = claims.scope;
  const granted = typeof scope === "string" ? scope.split(" ").filter(Boolean) : [];
  if (!granted.includes(needed)) {
    const problem = authProblem("insufficient-scope");
    return {
      ok: false,
      problem: { ...problem, detail: `requires scope: ${needed}` },
    };
  }
  return { ok: true };
}
