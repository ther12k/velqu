# BETA-015-F — SBOM

## Overview

Implements the **SBOM** deliverable of the beta release packet. New script `scripts/sbom.sh` generates a CycloneDX 1.5 SBOM at `release/sbom.cdx.json`:

- **277 components**: 12 workspace crates + 256 external crates (from `cargo metadata` with versions, purls, and licenses) + 9 shipped `@velqu/*` npm packages (purls, shipped-tarball flag).
- **Commit-bound**: `metadata.properties.velqu:source-commit` carries the full source commit; the serial number derives from it.
- **Deterministic**: components sorted by (type, name, version).
- **License coverage**: all 277 components carry license data — external crates resolve from cargo metadata (MIT/Apache/BSD/ISC/Zlib/Unicode families; zero missing); workspace crates carry the declared `UNLICENSED-BEFORE-OWNER-DECISION` posture; npm packages carry `NOASSERTION` with an explicit `velqu:license-posture: owner-decision-pending` property (license is an owner decision, `docs/open-decisions.md`).
- Fails closed if any **external** crate lacks license data.

## Rehearsal (this worktree)

```json
{
  "format": "CycloneDX 1.5",
  "commit": "0deea8ee…",
  "components": 277,
  "workspaceCrates": 12,
  "externalCrates": 256,
  "npmPackages": 9,
  "externalPackagesMissingLicense": [],
  "verdict": "PASS"
}
```

## Guardrail mapping

- **SBOM identifies dependencies/licenses** — this deliverable: full dependency inventory with per-component license data; vulnerability/advisory scanning availability is disclosed separately (BETA-009-B: no cargo-audit/cargo-deny/osv-scanner in the environment; scanners field carried in the BETA-009-B report).
- **Artifacts map to one source commit** — `velqu:source-commit` property; regenerate alongside `scripts/release-packet` from the same clean commit.
- **Checksums verify from release directory** — the packet's `SHA256SUMS.txt` should be extended with `sbom.cdx.json` at packet assembly time (owner-gated release assembly composes both scripts).
- **No stale historical metadata** — regenerated from scratch per run with a fresh timestamp.

## Gates

- `cargo test -p q-pack` — pass (100+2)
- `cargo test -p q-http` — pass (15)
- `cargo test -p q-schema-runtime` — pass (58)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` / `cargo clippy -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Workspace/npm license selection remains an owner decision; the SBOM records the current posture honestly (`UNLICENSED-BEFORE-OWNER-DECISION` / `NOASSERTION + owner-decision-pending`) instead of asserting a license that has not been chosen.
- Advisory-database scanning was not available in this environment (BETA-009-B disclosure).
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
