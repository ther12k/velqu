# BETA-011-B — Publish `next` / `beta` Tag

## Overview

Defined, simulated, and validated the npm publication tag policy for prereleases:

- **Target Tags**: `beta` (default public prerelease tag) and `next` (alternative channel).
- **Prohibition of `latest`**: Prerelease versions (`0.1.0-beta.1`) MUST NOT be published under npm's default `latest` tag, ensuring that standard `npm install <pkg>` does not install an unstable prerelease by default.
- **Dry-run Simulation**:
  - Validated via `scripts/publish-tag-dryrun.sh`.
  - Confirms all 9 `@velqu/*` packages remain protected under `"private": true`.
  - Rehearsed command: `npm publish --tag beta --dry-run`.
  - Machine-readable evidence written to `docs/reports/beta-011-b-publish-tag-dry-run.json`.

## Invariants Verified

1. **No accidental `latest` tag**: All distribution tag configuration specifies `--tag beta` explicitly.
2. **Package protection**: All packages remain private; dry-run confirms zero unintended registry modifications.
3. **Non-mutation**: Re-running dry-run publish does not alter package versions or manifests.
4. **Owner Authority**: Consistent with `docs/beta/governance/RELEASE_AUTHORITY.md`.

## Evidence

- `scripts/publish-tag-dryrun.sh` — PASS.
- `docs/reports/beta-011-b-publish-tag-dry-run.json`.
- `docs/beta/governance/RELEASE_AUTHORITY.md`.

## Gates

- `cargo test -p q-pack` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Simulation only; actual network registry upload is gated on Owner decision and repository/license finalization.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
