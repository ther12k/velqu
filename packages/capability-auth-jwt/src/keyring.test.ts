/**
 * Key loading/rotation hook tests (BETA-005-B).
 *
 * Deterministic coverage: validated loading, rotation overlap,
 * retirement, atomic refresh, snapshot id-only exposure, and the
 * fail-closed rejections. No network, no timers.
 */
import { describe, expect, test } from "bun:test";
import { signJwt, verifyJwt } from "./index";
import { JwtKeyring, MAX_KEYRING_KEYS, type JwtKey } from "./keyring";

const K1: JwtKey = { id: "key-2026-01", secret: "secret-one" };
const K2: JwtKey = { id: "key-2026-02", secret: "secret-two" };

describe("keyring loading hooks (BETA-005-B)", () => {
  test("load builds a ring and reports ids only (no secrets in snapshot)", () => {
    const ring = JwtKeyring.fromKeys([K1, K2]);
    const snap = ring.snapshot();
    expect(snap.currentId).toBe("key-2026-02");
    expect(snap.verifyingIds).toEqual(["key-2026-01", "key-2026-02"]);
    expect(JSON.stringify(snap)).not.toContain("secret-");
  });

  test("empty key set is a typed rejection", async () => {
    await expect(JwtKeyring.load(async () => [])).rejects.toMatchObject({
      name: "JwtKeyringError",
      reason: "empty-key-set",
    });
  });

  test("duplicate ids are a typed rejection", () => {
    expect(() => JwtKeyring.fromKeys([K1, { ...K1 }])).toThrow(
      "jwt keyring: duplicate-key-id",
    );
  });

  test("oversized rings are a typed rejection", () => {
    const keys: JwtKey[] = Array.from({ length: MAX_KEYRING_KEYS + 1 }, (_, i) => ({
      id: `k${i}`,
      secret: `s${i}`,
    }));
    expect(() => JwtKeyring.fromKeys(keys)).toThrow("jwt keyring: too-many-keys");
  });

  test("malformed key shapes are a typed rejection", () => {
    expect(() => JwtKeyring.fromKeys([{ id: "", secret: "s" }])).toThrow();
    expect(() =>
      JwtKeyring.fromKeys([{ id: "k", secret: "" } as unknown as JwtKey]),
    ).toThrow();
  });
});

describe("rotation hooks (BETA-005-B)", () => {
  test("rotate admits a new signing key; old tokens still verify (overlap)", () => {
    const ring = JwtKeyring.fromKeys([K1]);
    const oldToken = ring.sign({ sub: "usr_1" });
    ring.rotate(K2);
    expect(ring.currentId).toBe("key-2026-02");
    // new token signs with the new key...
    const newToken = ring.sign({ sub: "usr_1" });
    // ...and the old token STILL verifies during the overlap window
    const oldRes = ring.verify(oldToken);
    const newRes = ring.verify(newToken);
    expect(oldRes.ok).toBe(true);
    expect(newRes.ok).toBe(true);
    if (oldRes.ok) expect(oldRes.keyId).toBe("key-2026-01");
    if (newRes.ok) expect(newRes.keyId).toBe("key-2026-02");
  });

  test("retire stops verification for the old key; unknown keys give no oracle", () => {
    const ring = JwtKeyring.fromKeys([K1, K2]);
    const oldToken = ring.sign({ sub: "usr_1" }); // signed with current K2
    const overlapToken = JwtKeyring.fromKeys([K1]).sign({ sub: "usr_1" });
    ring.retire("key-2026-01");
    // after retirement the overlap token no longer verifies
    const res = ring.verify(overlapToken);
    expect(res.ok).toBe(false);
    if (!res.ok) expect(res.reason).toBe("signature-mismatch");
    // the current token still verifies
    expect(ring.verify(oldToken).ok).toBe(true);
  });

  test("retiring the current key promotes another and never empties the ring", () => {
    const ring = JwtKeyring.fromKeys([K1, K2]);
    ring.retire("key-2026-02");
    expect(ring.currentId).toBe("key-2026-01");
    expect(() => ring.retire("key-2026-01")).toThrow("empty-key-set");
  });

  test("rotate past the ceiling is a typed rejection", () => {
    const keys: JwtKey[] = Array.from({ length: MAX_KEYRING_KEYS }, (_, i) => ({
      id: `k${i}`,
      secret: `s${i}`,
    }));
    const ring = JwtKeyring.fromKeys(keys);
    expect(() => ring.rotate({ id: "one-more", secret: "s" })).toThrow("too-many-keys");
  });

  test("refresh atomically replaces the set from the loader (rotation hook)", async () => {
    const ring = JwtKeyring.fromKeys([K1]);
    const oldToken = ring.sign({ sub: "usr_1" });
    await ring.refresh(async () => [K2]);
    expect(ring.snapshot().verifyingIds).toEqual(["key-2026-02"]);
    // old key is gone after the atomic refresh: old tokens fail closed
    expect(ring.verify(oldToken).ok).toBe(false);
    expect(ring.verify(ring.sign({ sub: "u" })).ok).toBe(true);
  });

  test("refresh validates too: an empty loader result leaves the ring unchanged", async () => {
    const ring = JwtKeyring.fromKeys([K1]);
    await expect(ring.refresh(async () => [])).rejects.toMatchObject({
      reason: "empty-key-set",
    });
    // unchanged after the failed refresh
    expect(ring.snapshot().verifyingIds).toEqual(["key-2026-01"]);
    expect(ring.verify(ring.sign({ sub: "u" })).ok).toBe(true);
  });
});

describe("profile integration", () => {
  test("keyring verify still enforces the algorithm gates per token", () => {
    const ring = JwtKeyring.fromKeys([K1]);
    const noneHeader = Buffer.from(JSON.stringify({ alg: "none", typ: "JWT" })).toString(
      "base64url",
    );
    const claims = Buffer.from(JSON.stringify({ sub: "usr_1" })).toString("base64url");
    const res = ring.verify(`${noneHeader}.${claims}.x`);
    expect(res.ok).toBe(false);
    if (!res.ok) expect(res.reason).toBe("algorithm-not-approved");
  });

  test("sign uses the current key only (verifyJwt agrees)", () => {
    const ring = JwtKeyring.fromKeys([K1, K2]);
    const token = ring.sign({ sub: "usr_1" });
    expect(verifyJwt(token, K2.secret).ok).toBe(true);
    expect(verifyJwt(token, K1.secret).ok).toBe(false);
  });
});
