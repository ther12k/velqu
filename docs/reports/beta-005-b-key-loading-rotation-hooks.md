# BETA-005-B — Key Loading/Rotation Hooks (JWT capability)

Status: **ENFORCED** (typed hooks, deterministic tests).

## What was built

`packages/capability-auth-jwt/src/keyring.ts` — `JwtKeyring`:

- **Loading hook**: `JwtKeyring.load(loader)` / `JwtKeyring.fromKeys(keys)`
  — the caller supplies keys (env, file, secret store); the capability
  never fetches secrets itself and never logs them. Loading is
  validated fail-closed: empty sets, duplicate ids, oversized rings
  (> `MAX_KEYRING_KEYS` = 8), and malformed shapes are typed
  `JwtKeyringError` rejections.
- **Rotation hooks**:
  - `rotate(key)` — admit a new signing key; previous keys keep
    verifying (overlap window: tokens issued before rotation stay
    valid). Tokens carry no `kid` (profile gate), so verification tries
    each active key — bounded — and reports which key verified.
    Signing always uses the current key.
  - `retire(id)` — stop verifying a key; retiring the current key
    promotes another and the ring can never empty (typed rejection).
  - `refresh(loader)` — atomic full reload from the loader (push/poll
    rotation); a failed refresh leaves the previous ring untouched.
- **No-secret exposure**: snapshots (`snapshot()`) expose ids only;
  errors carry the typed reason, never key material.

## Security posture

- Verification keeps every A-profile gate per token (algorithm/structure
  rejections are key-independent and fail fast; unknown keys surface as
  the same `signature-mismatch` reason — no key-existence oracle).
- Rotation is caller-driven: no background timers, no implicit network
  fetches, no ambient key discovery.

## Tests (13 new, deterministic)

Loading: defaults/build, empty set, duplicate ids, oversize ring,
malformed shapes. Rotation: overlap verification, retire semantics,
current-key promotion, ceiling rejection, atomic refresh, failed
refresh leaves ring unchanged, per-token algorithm gates still apply,
sign-uses-current-key. Crate total 27 pass (14 profile + 13 keyring).

## Gates

- `bun test packages/capability-auth-jwt` -> 27 pass / 0 fail
- `bun test` -> 411 pass / 0 fail (64 files)
- typecheck / fmt / clippy -> clean
- `./scripts/verify` -> ALL PASS
