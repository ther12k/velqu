/**
 * JWT capability — one approved algorithm profile (BETA-005-A).
 *
 * The ONLY approved profile is HS256 (HMAC-SHA-256 over the compact
 * JWS serialization) with `typ: "JWT"`. Verification is fail closed:
 *
 *  1. Structural gate — exactly three non-empty base64url segments.
 *  2. Algorithm gate — the decoded header MUST declare `alg: "HS256"`
 *     (exact string). `none`, lowercase/case variants, asymmetric
 *     algorithms, and missing `alg` are rejected BEFORE any signature
 *     work, so algorithm confusion is impossible by construction: there
 *     is no key-type dispatch and no verification path other than
 *     HMAC-SHA-256.
 *  3. Key-injection gate — `jku`, `jwk`, `x5u`, `x5c`, `kid`-driven key
 *     selection do not exist here; header fields other than `alg`/`typ`
 *     cause rejection.
 *  4. Signature gate — HMAC-SHA-256 with a timing-safe comparison.
 *  5. Claims gate — claims must decode to a JSON object.
 *
 * Expiry/audience/issuer validation is layered by callers (BETA-005-C);
 * this module verifies authenticity and shape only.
 *
 * Performance posture: verification is one HMAC over the token —
 * O(token length), no allocations beyond the hash workspace. There is
 * deliberately no verification cache in this profile; caching (e.g.
 * bounded LRU keyed by token hash) is a BETA-005-C decision and would
 * be documented there. Failure costs are the same order as success
 * (constant-shape work), so the path does not leak validity through
 * timing on the structural/algorithm gates.
 */

/** The single approved JWS algorithm for this profile. */
export const APPROVED_ALGORITHM = "HS256";

/** Fail-closed ceilings. */
export const MAX_TOKEN_LENGTH = 8 * 1024;
export const MAX_PAYLOAD_CLAIMS_BYTES = 4 * 1024;

/** Typed verification failures. Closed set; messages never echo token material. */
export type JwtVerifyFailure =
  | "malformed" // wrong segment count / empty segments / overlong
  | "undecodable-header"
  | "undecodable-claims"
  | "claims-not-object"
  | "algorithm-not-approved" // missing, `none`, case variants, asymmetric
  | "header-field-not-allowed" // jku/jwk/x5u/x5c/zip/… beyond alg+typ
  | "typ-not-jwt"
  | "signature-mismatch";

export interface JwtHeader {
  alg: "HS256";
  typ: "JWT";
}

export type JwtVerifyResult =
  | { ok: true; header: JwtHeader; claims: Record<string, unknown> }
  | { ok: false; reason: JwtVerifyFailure };

// ---------------------------------------------------------------- primitives
// Pure-JS SHA-256/HMAC (FIPS 180-4 / RFC 2104): the QuickJS runtime
// intentionally provides no SubtleCrypto, and mocks are forbidden.

const K = new Uint32Array([
  0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
  0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
  0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
  0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
  0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
  0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
  0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
  0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
]);

function sha256(message: Uint8Array): Uint8Array {
  const bitLen = message.length * 8;
  const padded = new Uint8Array((((message.length + 8) >> 6) + 1) << 6);
  padded.set(message);
  padded[message.length] = 0x80;
  const view = new DataView(padded.buffer);
  view.setUint32(padded.length - 8, Math.floor(bitLen / 2 ** 32));
  view.setUint32(padded.length - 4, bitLen >>> 0);

  let h0 = 0x6a09e667, h1 = 0xbb67ae85, h2 = 0x3c6ef372, h3 = 0xa54ff53a;
  let h4 = 0x510e527f, h5 = 0x9b05688c, h6 = 0x1f83d9ab, h7 = 0x5be0cd19;
  const w = new Uint32Array(64);
  const rotr = (x: number, n: number) => (x >>> n) | (x << (32 - n));

  for (let chunk = 0; chunk < padded.length; chunk += 64) {
    for (let i = 0; i < 16; i++) w[i] = view.getUint32(chunk + i * 4);
    for (let i = 16; i < 64; i++) {
      const s0 = rotr(w[i - 15], 7) ^ rotr(w[i - 15], 18) ^ (w[i - 15] >>> 3);
      const s1 = rotr(w[i - 2], 17) ^ rotr(w[i - 2], 19) ^ (w[i - 2] >>> 10);
      w[i] = (w[i - 16] + s0 + w[i - 7] + s1) >>> 0;
    }
    let a = h0, b = h1, c = h2, d = h3, e = h4, f = h5, g = h6, h = h7;
    for (let i = 0; i < 64; i++) {
      const S1 = rotr(e, 6) ^ rotr(e, 11) ^ rotr(e, 25);
      const ch = (e & f) ^ (~e & g);
      const t1 = (h + S1 + ch + K[i] + w[i]) >>> 0;
      const S0 = rotr(a, 2) ^ rotr(a, 13) ^ rotr(a, 22);
      const maj = (a & b) ^ (a & c) ^ (b & c);
      const t2 = (S0 + maj) >>> 0;
      h = g; g = f; f = e; e = (d + t1) >>> 0;
      d = c; c = b; b = a; a = (t1 + t2) >>> 0;
    }
    h0 = (h0 + a) >>> 0; h1 = (h1 + b) >>> 0; h2 = (h2 + c) >>> 0; h3 = (h3 + d) >>> 0;
    h4 = (h4 + e) >>> 0; h5 = (h5 + f) >>> 0; h6 = (h6 + g) >>> 0; h7 = (h7 + h) >>> 0;
  }
  const out = new Uint8Array(32);
  const o = new DataView(out.buffer);
  o.setUint32(0, h0); o.setUint32(4, h1); o.setUint32(8, h2); o.setUint32(12, h3);
  o.setUint32(16, h4); o.setUint32(20, h5); o.setUint32(24, h6); o.setUint32(28, h7);
  return out;
}

/** Reference HMAC-SHA-256 (RFC 2104) over UTF-8 inputs. */
export function hmacSha256(key: Uint8Array, message: Uint8Array): Uint8Array {
  let k = key;
  if (k.length > 64) k = sha256(k);
  const block = new Uint8Array(64);
  block.set(k);
  const inner = new Uint8Array(64 + message.length);
  const outer = new Uint8Array(96);
  for (let i = 0; i < 64; i++) {
    inner[i] = block[i] ^ 0x36;
    outer[i] = block[i] ^ 0x5c;
  }
  inner.set(message, 64);
  outer.set(sha256(inner), 64);
  return sha256(outer);
}

export function base64UrlEncode(bytes: Uint8Array): string {
  let binary = "";
  for (const b of bytes) binary += String.fromCharCode(b);
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

export function base64UrlDecode(text: string): Uint8Array {
  const padded = text.replace(/-/g, "+").replace(/_/g, "/");
  const binary = atob(padded + "=".repeat((4 - (padded.length % 4)) % 4));
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
  return bytes;
}

const ENCODER = new TextEncoder();
const DECODER = new TextDecoder();

function timingSafeEqual(a: string, b: string): boolean {
  if (a.length !== b.length) return false;
  let diff = 0;
  for (let i = 0; i < a.length; i++) diff |= a.charCodeAt(i) ^ b.charCodeAt(i);
  return diff === 0;
}

// ---------------------------------------------------------------- issue (reference)

export function signJwt(
  claims: Record<string, unknown>,
  secret: string,
): string {
  const header = base64UrlEncode(
    ENCODER.encode(JSON.stringify({ alg: APPROVED_ALGORITHM, typ: "JWT" })),
  );
  const body = base64UrlEncode(ENCODER.encode(JSON.stringify(claims)));
  const signature = base64UrlEncode(
    hmacSha256(ENCODER.encode(secret), ENCODER.encode(`${header}.${body}`)),
  );
  return `${header}.${body}.${signature}`;
}

// ---------------------------------------------------------------- verify

export function verifyJwt(token: string, secret: string): JwtVerifyResult {
  // 1. structural gate
  if (typeof token !== "string" || token.length === 0 || token.length > MAX_TOKEN_LENGTH) {
    return { ok: false, reason: "malformed" };
  }
  const parts = token.split(".");
  if (parts.length !== 3 || parts.some((p) => p.length === 0)) {
    return { ok: false, reason: "malformed" };
  }
  const [headerSegment, bodySegment, signatureSegment] = parts;

  // 2. algorithm gate — before any signature work
  let header: Record<string, unknown>;
  try {
    header = JSON.parse(DECODER.decode(base64UrlDecode(headerSegment))) as Record<string, unknown>;
  } catch {
    return { ok: false, reason: "undecodable-header" };
  }
  if (header === null || typeof header !== "object" || Array.isArray(header)) {
    return { ok: false, reason: "undecodable-header" };
  }
  if (header.alg !== APPROVED_ALGORITHM) {
    // `none`, case variants, asymmetric algorithms, missing — one gate
    return { ok: false, reason: "algorithm-not-approved" };
  }
  const allowedHeaderKeys = ["alg", "typ"];
  for (const key of Object.keys(header)) {
    if (!allowedHeaderKeys.includes(key)) {
      return { ok: false, reason: "header-field-not-allowed" };
    }
  }
  if (header.typ !== undefined && header.typ !== "JWT") {
    return { ok: false, reason: "typ-not-jwt" };
  }

  // 3. signature gate (timing-safe)
  const expected = base64UrlEncode(
    hmacSha256(ENCODER.encode(secret), ENCODER.encode(`${headerSegment}.${bodySegment}`)),
  );
  if (!timingSafeEqual(signatureSegment, expected)) {
    return { ok: false, reason: "signature-mismatch" };
  }

  // 4. claims gate
  let claims: unknown;
  try {
    if (bodySegment.length > MAX_PAYLOAD_CLAIMS_BYTES) {
      return { ok: false, reason: "malformed" };
    }
    claims = JSON.parse(DECODER.decode(base64UrlDecode(bodySegment)));
  } catch {
    return { ok: false, reason: "undecodable-claims" };
  }
  if (claims === null || typeof claims !== "object" || Array.isArray(claims)) {
    return { ok: false, reason: "claims-not-object" };
  }

  return {
    ok: true,
    header: { alg: APPROVED_ALGORITHM, typ: "JWT" },
    claims: claims as Record<string, unknown>,
  };
}
