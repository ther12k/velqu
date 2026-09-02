import { definePolicy, status } from "@velqu/core";

export interface Session {
  userId: string;
  scope: string;
}

// ---------------------------------------------------------------- token codec
// JWT-*like* reference: compact three-segment token with an HMAC-SHA-256
// signature. It follows the JWS compact serialization shape (header.payload
// .signature, base64url) but is a teaching fixture — not a hardened JOSE
// implementation. The QuickJS runtime intentionally provides NO SubtleCrypto
// (M28 scope: crypto is getRandomValues/randomUUID only, and mocks are
// forbidden), so the HMAC-SHA-256 below is a compact pure-JS reference
// implementation over the documented Web primitives.

const ENCODER = new TextEncoder();

// --- pure-JS SHA-256 (FIPS 180-4) over Uint8Array --------------------------
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

function hmacSha256(key: Uint8Array, message: Uint8Array): Uint8Array {
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

function base64UrlEncode(bytes: Uint8Array): string {
  let binary = "";
  for (const b of bytes) binary += String.fromCharCode(b);
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

function base64UrlDecode(text: string): Uint8Array {
  const padded = text.replace(/-/g, "+").replace(/_/g, "/");
  const binary = atob(padded + "=".repeat((4 - (padded.length % 4)) % 4));
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
  return bytes;
}

function sign(payload: string, secret: string): string {
  return base64UrlEncode(hmacSha256(ENCODER.encode(secret), ENCODER.encode(payload)));
}

/** Constant-time-ish string comparison for MAC material. */
function timingSafeEqual(a: string, b: string): boolean {
  if (a.length !== b.length) return false;
  let diff = 0;
  for (let i = 0; i < a.length; i++) diff |= a.charCodeAt(i) ^ b.charCodeAt(i);
  return diff === 0;
}

// ---------------------------------------------------------------- reference tokens

const DEMO_SECRET = "jwt-reference-demo-secret";
const ALLOWED_CLOCK_SKEW_MS = 5_000;

export interface TokenClaims {
  sub: string;
  scope: string;
  exp: number; // epoch ms
}

/** Issue a reference token for the demo login route (educational fixture). */
export async function issueToken(claims: Omit<TokenClaims, "exp">, ttlMs = 3_600_000): Promise<string> {
  const header = base64UrlEncode(ENCODER.encode(JSON.stringify({ alg: "HS256", typ: "JWT" })));
  const body = base64UrlEncode(
    ENCODER.encode(JSON.stringify({ ...claims, exp: Date.now() + ttlMs })),
  );
  const signature = sign(`${header}.${body}`, DEMO_SECRET);
  return `${header}.${body}.${signature}`;
}

// ---------------------------------------------------------------- policy

/**
 * JWT-like bearer policy reference (M4A-009-B): verifies the compact token
 * signature with HMAC-SHA-256, enforces expiry with a small clock-skew
 * allowance, and provides a typed session to the route. Declares typed 401
 * failures; the demo secret and `issueToken` login route are fixtures only —
 * not production authentication guidance.
 */
export const jwtPolicy = definePolicy({
  id: "auth.jwt",
  header: "authorization",
  declares: { 401: "unauthorized" },
  provides: "session",
  check: async (req) => {
    const header = req.headers.authorization ?? "";
    if (!header.startsWith("Bearer ")) {
      return status(401).problem("unauthorized", { detail: "missing bearer token" });
    }
    const parts = header.slice("Bearer ".length).split(".");
    if (parts.length !== 3) {
      return status(401).problem("unauthorized", { detail: "malformed token" });
    }
    const [headerSegment, bodySegment, signatureSegment] = parts;
    const expected = sign(`${headerSegment}.${bodySegment}`, DEMO_SECRET);
    if (!timingSafeEqual(signatureSegment, expected)) {
      return status(401).problem("unauthorized", { detail: "signature mismatch" });
    }
    let claims: TokenClaims;
    try {
      claims = JSON.parse(new TextDecoder().decode(base64UrlDecode(bodySegment))) as TokenClaims;
    } catch {
      return status(401).problem("unauthorized", { detail: "undecodable claims" });
    }
    if (typeof claims.exp !== "number" || Date.now() > claims.exp + ALLOWED_CLOCK_SKEW_MS) {
      return status(401).problem("unauthorized", { detail: "token expired" });
    }
    return {
      session: { userId: claims.sub, scope: claims.scope } satisfies Session,
    };
  },
});

export const JWT_DEMO_SECRET = DEMO_SECRET;
/** Reference MAC (RFC 4231 vectors are pinned by the auth unit tests). */
export const _referenceHmacSha256 = hmacSha256;
export default jwtPolicy;
