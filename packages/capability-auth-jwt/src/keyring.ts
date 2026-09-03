/**
 * Key loading and rotation hooks (BETA-005-B).
 *
 * `JwtKeyring` manages the HS256 secrets for the approved JWT profile:
 *
 * - **Loading**: `loadJwtKeyring(loader)` builds a keyring from a
 *   caller-supplied loader (env, file, secret store — the capability
 *   never fetches secrets itself and never logs them). Loading is
 *   validated fail-closed: empty sets, duplicate ids, and oversized
 *   rings are typed rejections before any token is served.
 * - **Rotation**: `rotate(key)` admits a new signing key while the
 *   previous keys keep verifying (overlap window — tokens issued
 *   before rotation stay valid); `retire(id)` removes a verifying key.
 *   `refresh(loader)` atomically replaces the whole set from the loader
 *   (push/poll rotation hook). Every hook is caller-driven; there is no
 *   background timer and no implicit network fetch.
 * - **Verification**: tokens carry no `kid` (header fields beyond
 *   alg/typ are rejected by the profile gate), so verification tries
 *   each active key — bounded by MAX_KEYRING_KEYS — and reports which
 *   key verified. Signing always uses the current key.
 *
 * Secrets never appear in errors or snapshots; `snapshot()` exposes ids
 * only.
 */

import { signJwt, verifyJwt, type JwtVerifyResult } from "./index";

/** Fail-closed ceiling for the number of simultaneously active keys. */
export const MAX_KEYRING_KEYS = 8;

/** One HS256 signing/verifying key. `id` is caller-managed and stable. */
export interface JwtKey {
  id: string;
  secret: string;
}

/** Typed keyring failures. Closed set; never carries secret material. */
export type KeyringError =
  | "empty-key-set"
  | "duplicate-key-id"
  | "too-many-keys"
  | "invalid-key-shape";

export interface KeyringSnapshot {
  currentId: string;
  verifyingIds: string[];
}

export type KeyringVerifyResult =
  | {
      ok: true;
      header: import("./index").JwtHeader;
      claims: Record<string, unknown>;
      /** The id of the key that verified the token. */
      keyId: string;
    }
  | { ok: false; reason: import("./index").JwtVerifyFailure };

/** Async loader shape for the loading/refresh hooks. */
export type JwtKeyLoader = () => Promise<JwtKey[]> | JwtKey[];

export class JwtKeyring {
  private current: JwtKey;
  private verifying: Map<string, JwtKey>;

  private constructor(current: JwtKey, verifying: Map<string, JwtKey>) {
    this.current = current;
    this.verifying = verifying;
  }

  /** Validate a key set (shared by construction and refresh). */
  private static validate(keys: JwtKey[]): Map<string, JwtKey> {
    if (!Array.isArray(keys) || keys.length === 0) {
      throw typed("empty-key-set");
    }
    if (keys.length > MAX_KEYRING_KEYS) {
      throw typed("too-many-keys");
    }
    const map = new Map<string, JwtKey>();
    for (const key of keys) {
      if (
        key === null ||
        typeof key !== "object" ||
        typeof key.id !== "string" ||
        key.id.length === 0 ||
        typeof key.secret !== "string" ||
        key.secret.length === 0
      ) {
        throw typed("invalid-key-shape");
      }
      if (map.has(key.id)) {
        throw typed("duplicate-key-id");
      }
      map.set(key.id, key);
    }
    return map;
  }

  /** Internal: replace the whole set atomically (refresh/rotate paths). */
  private static from(keys: JwtKey[]): JwtKeyring {
    const map = JwtKeyring.validate(keys);
    // construction order is the rotation order: the last key signs
    const id = keys[keys.length - 1].id;
    return new JwtKeyring(map.get(id) as JwtKey, map);
  }

  /** Build a keyring from an initial key array. */
  public static fromKeys(keys: JwtKey[]): JwtKeyring {
    return JwtKeyring.from(keys);
  }

  /** Loading hook: build a keyring from a caller loader. */
  public static async load(loader: JwtKeyLoader): Promise<JwtKeyring> {
    return JwtKeyring.from(await loader());
  }

  /** Rotation hook: atomically replace the whole set from a loader. */
  public async refresh(loader: JwtKeyLoader): Promise<void> {
    const replacement = JwtKeyring.from(await loader());
    this.current = replacement.current;
    this.verifying = replacement.verifying;
  }

  /** Rotation hook: admit a new signing key (old keys keep verifying). */
  public rotate(key: JwtKey): void {
    const candidate = JwtKeyring.validate([key]);
    const merged = new Map(this.verifying);
    if (merged.size >= MAX_KEYRING_KEYS && !merged.has(key.id)) {
      throw typed("too-many-keys");
    }
    const id = [...candidate.keys()][0];
    merged.set(id, candidate.get(id) as JwtKey);
    this.verifying = merged;
    this.current = candidate.get(id) as JwtKey;
  }

  /** Rotation hook: stop verifying with a key (id unknown is a no-op). */
  public retire(id: string): void {
    if (id === this.current.id) {
      // the signing key cannot be retired while it is the only/current one
      if (this.verifying.size <= 1) {
        throw typed("empty-key-set");
      }
      // promote the most recent other key to current
      const others = [...this.verifying.keys()].filter((k) => k !== id);
      const next = this.verifying.get(others[others.length - 1]) as JwtKey;
      this.current = next;
    }
    this.verifying.delete(id);
  }

  /** Current signing key id (snapshots expose ids only). */
  public get currentId(): string {
    return this.current.id;
  }

  /** All active key ids, current last. */
  public snapshot(): KeyringSnapshot {
    return {
      currentId: this.current.id,
      verifyingIds: [...this.verifying.keys()],
    };
  }

  /** Sign with the current key. */
  public sign(claims: Record<string, unknown>): string {
    return signJwt(claims, this.current.secret);
  }

  /**
   * Verify against every active key (rotation overlap). Failure reasons
   * follow the profile gates; an unknown key surfaces as the same
   * `signature-mismatch` reason — no key-existence oracle.
   */
  public verify(token: string): KeyringVerifyResult {
    for (const [id, key] of this.verifying) {
      const res = verifyJwt(token, key.secret);
      if (res.ok) {
        return { ...res, keyId: id };
      }
      // structural/algorithm rejections are key-independent: fail fast
      if (res.reason !== "signature-mismatch") {
        return res;
      }
    }
    return { ok: false, reason: "signature-mismatch" };
  }
}

function typed(reason: KeyringError): Error {
  const error = new Error(`jwt keyring: ${reason}`);
  error.name = "JwtKeyringError";
  (error as Error & { reason: KeyringError }).reason = reason;
  return error;
}

/** Re-exported profile gates for one-stop imports. */
export { verifyJwt, signJwt, APPROVED_ALGORITHM } from "./index";
export type { JwtVerifyResult, JwtVerifyFailure } from "./index";
