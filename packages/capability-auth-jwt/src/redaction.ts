/**
 * Secret redaction for logging (BETA-005-E).
 *
 * The capability never logs tokens or secrets — these helpers exist for
 * the *caller's* logging path so that safe logging is a one-call
 * affordance instead of a judgment call:
 *
 * - `redactAuthorizationHeader(header)` — replaces the whole header
 *   value with a bounded marker (segment count + byte length only).
 * - `redactToken(token)` — same marker for a bare token.
 * - `scrub(text, secrets)` — defense in depth: removes any occurrence
 *   of the supplied secret material from a line before it is written.
 * - `secretFingerprint(secret)` — a short SHA-256-derived id so logs
 *   can correlate *which* configured key was in play without revealing
 *   it (needs the same secret to reproduce, and cannot be reversed).
 *
 * Markers are constant-shape: they carry lengths and counts (operational
 * signal), never prefixes, suffixes, or partial material (an attacker
 * who can read logs learns nothing reconstructable).
 */

import { hmacSha256 } from "./index";

/** Marker shape: no token/secret material, only shapes and sizes. */
export function redactToken(token: string): string {
  const segments = typeof token === "string" ? token.split(".").length : 0;
  const bytes = typeof token === "string" ? token.length : 0;
  return `<jwt redacted; segments=${segments}; bytes=${bytes}>`;
}

export function redactAuthorizationHeader(header: string | null | undefined): string {
  if (!header) return "<authorization absent>";
  const marker = header.startsWith("Bearer ") ? "bearer " : "";
  return `${marker}<redacted: ${redactToken(header.replace(/^Bearer /, ""))}>`;
}

/** Removes every occurrence of each secret from `text`. */
export function scrub(text: string, secrets: string[]): string {
  let out = text;
  for (const secret of secrets) {
    if (secret.length === 0) continue;
    while (out.includes(secret)) {
      out = out.replace(secret, "<redacted-secret>");
    }
  }
  return out;
}

/**
 * Stable, non-reversible 12-hex-char id for a secret (keyed by a fixed
 * capability context). Same secret → same id; different secrets collide
 * with probability ~2^-48; the id cannot be turned back into material.
 */
export function secretFingerprint(secret: string): string {
  const mac = hmacSha256(
    new TextEncoder().encode("velqu:jwt:fingerprint:v1"),
    new TextEncoder().encode(secret),
  );
  return Array.from(mac.slice(0, 6))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}
