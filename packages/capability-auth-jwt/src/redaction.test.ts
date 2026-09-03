/**
 * No-secret-logging tests (BETA-005-E).
 *
 * The enforcement sweep: build every typed failure the capability can
 * produce using a *distinctive* secret, then assert the secret (and the
 * token) never appear in any error, problem document, or snapshot —
 * and that the redaction helpers produce safe markers instead.
 */
import { describe, expect, test } from "bun:test";
import { signJwt } from "./index";
import { JwtKeyring } from "./keyring";
import { authenticateBearer, authProblem, requireScope } from "./problems";
import {
  redactAuthorizationHeader,
  redactToken,
  scrub,
  secretFingerprint,
} from "./redaction";

const SECRET = "SUPER-SECRET-MATERIAL-DO-NOT-LOG-0xF00D";
const NOW = 1_800_000_000_000;
const validToken = signJwt(
  { sub: "usr_1", scope: "demo", exp: Math.floor((NOW + 3_600_000) / 1000) },
  SECRET,
);

/** Collect every string the capability can emit around a failure. */
function emittedStrings(): string[] {
  const out: string[] = [];
  const failures: Array<{ ok: false; reason: string }> = [];
  // profile gate failures
  const probes = [
    "",
    "not-a-token",
    "a.b.c",
    `${Buffer.from(JSON.stringify({ alg: "none" })).toString("base64url")}.x.y`,
  ];
  for (const probe of probes) {
    const res = verifyTokenSafe(probe);
    if (res) failures.push(res);
  }
  for (const f of failures) out.push(JSON.stringify(f), String(f.reason));

  // problems
  for (const reason of ["missing-token", "signature-mismatch", "token-expired"] as const) {
    out.push(JSON.stringify(authProblem(reason)));
  }
  const flow = authenticateBearer(`Bearer ${validToken}`, "wrong-secret", { now: NOW });
  if (!flow.ok) out.push(JSON.stringify(flow.problem));

  // keyring rejections and snapshots
  try {
    JwtKeyring.fromKeys([]);
  } catch (e) {
    out.push(String(e));
  }
  const ring = JwtKeyring.fromKeys([{ id: "k1", secret: SECRET }]);
  out.push(JSON.stringify(ring.snapshot()));
  out.push(JSON.stringify(ring.verify("garbage")));

  // scope failure
  const scope = requireScope({ scope: "demo" }, "admin");
  if (!scope.ok) out.push(JSON.stringify(scope.problem));

  return out;
}

/** verifyJwt wrapped to capture only the typed failure (avoids throwing). */
function verifyTokenSafe(token: string): { ok: false; reason: string } | null {
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  const { verifyJwt } = require("./index") as typeof import("./index");
  const res = verifyJwt(token, SECRET);
  return res.ok ? null : { ok: false, reason: res.reason };
}

describe("no-secret-logging enforcement (BETA-005-E)", () => {
  test("no emitted string contains the secret or the token", () => {
    for (const text of emittedStrings()) {
      expect(text.includes(SECRET)).toBe(false);
      expect(text.includes(validToken)).toBe(false);
    }
  });

  test("typed errors carry only the reason, never material", () => {
    expect(() => JwtKeyring.fromKeys([])).toThrow(
      "jwt keyring: empty-key-set",
    );
    expect(() => JwtKeyring.fromKeys([])).not.toThrow(/secret/i);
  });
});

describe("redaction helpers (BETA-005-E)", () => {
  test("redactToken keeps shapes/sizes, drops material", () => {
    const marker = redactToken(validToken);
    expect(marker).toBe(
      `<jwt redacted; segments=3; bytes=${validToken.length}>`,
    );
    expect(marker).not.toContain(validToken);
    expect(marker).not.toContain(validToken.slice(0, 8));
  });

  test("redactAuthorizationHeader handles absent and bearer forms", () => {
    expect(redactAuthorizationHeader(undefined)).toBe("<authorization absent>");
    const marker = redactAuthorizationHeader(`Bearer ${validToken}`);
    expect(marker.startsWith("bearer ")).toBe(true);
    expect(marker).not.toContain(validToken);
  });

  test("scrub removes every occurrence of the secret from a line", () => {
    const line = `connect failed with ${SECRET} and again ${SECRET}`;
    const clean = scrub(line, [SECRET]);
    expect(clean.includes(SECRET)).toBe(false);
    expect(clean).toContain("<redacted-secret>");
  });

  test("secretFingerprint is stable, short, and non-reversible-shaped", () => {
    const fp1 = secretFingerprint(SECRET);
    const fp2 = secretFingerprint(SECRET);
    expect(fp1).toBe(fp2);
    expect(fp1).toMatch(/^[0-9a-f]{12}$/);
    expect(fp1).not.toContain(SECRET);
    expect(secretFingerprint("different")).not.toBe(fp1);
  });
});
