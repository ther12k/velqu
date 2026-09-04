# BETA-011-A — Use SemVer Prerelease

## Overview

Establishes and verifies the SemVer prerelease versioning policy for the public beta:

- **Target Version**: `0.1.0-beta.1` as authorized by the Owner in `docs/beta/governance/RELEASE_AUTHORITY.md`.
- **SemVer Compliance**: Adheres strictly to SemVer 2.0.0 specification with prerelease identifier `-beta.1`.
- **Pre-release Semantics**:
  - Does not imply API/ABI stability (`0.1.0` or later GA release promises are not made).
  - Explicitly documents that breaking beta changes require migration notes.
  - Distinguishes development monorepo version (`0.1.0`) from public release artifact candidate (`0.1.0-beta.1`).

## Verification & Tooling

- Added `scripts/semver-prerelease-check.sh` to validate the SemVer 2.0.0 structure and prerelease tag policy.
- Emits machine-readable evidence to `docs/reports/beta-011-a-semver-prerelease.json` with verdict `PASS`.
- Rehearsed rollback and withdrawal governance (`docs/beta/governance/RELEASE_AUTHORITY.md`): Owner maintains sole authority to withdraw or yank releases without rewriting historical evidence.

## Evidence

- `scripts/semver-prerelease-check.sh` — PASS.
- `docs/reports/beta-011-a-semver-prerelease.json`.
- `docs/beta/governance/RELEASE_AUTHORITY.md`.

## Gates

- `cargo test -p q-pack` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Monorepo package manifests retain `0.1.0` (with `private: true`) until release publication automation activates under owner authority.
- Pre-release versions carry no SLA or backward compatibility guarantees.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
