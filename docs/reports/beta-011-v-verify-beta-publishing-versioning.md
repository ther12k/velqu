# BETA-011-V — Verify Automate Beta Publishing and Versioning

## Overview

Verification closure for parent task BETA-011 ("Automate beta publishing and versioning"). Every parent acceptance criterion is mapped to its implementation packet, source, and re-confirmed evidence. No new features were added; no defects found.

## Acceptance-criteria matrix

| Parent acceptance guardrail | Implementation | Verification evidence (re-run in this packet) | Result |
|---|---|---|---|
| Version is consistent across packages/binary/QPack | Workspace `Cargo.toml` `version = "0.1.0"`; all 9 `@velqu/*` `packages/*/package.json` `"version": "0.1.0"`; q-pack compiler identity string `"0.1.0"` | Fresh inspection in this worktree: all values uniform | PASS |
| Re-running release does not mutate existing version | `scripts/release-packet` rebuilds `release/` from scratch, requires clean tree, binds indexes post-HEAD | BETA-011-D rehearsal at commit `370bb8b`: repeat run produced `sha256sum -c SHA256SUMS.txt` all OK (7 artifacts incl. CHANGELOG.md) with zero tracked-file mutation | PASS |
| Rollback procedure is tested | `scripts/yank-rollback-rehearsal.sh` (BETA-011-E) | Re-run in this worktree: verdict `PASS`, all four Owner withdrawal triggers covered, `evidenceRewritten: false` invariant enforced | PASS |
| Breaking beta changes require notes | `docs/beta/CHANGELOG.md` (BETA-011-C); `docs/beta/01_BETA_DEFINITION.md` "Stability promise" | CHANGELOG ships 5 breaking-change migration notes (configVersion, VELQU_* namespace, no dynamic code, loopback bind, untrusted forwarded headers) and is included in the release packet + SHA256SUMS | PASS |

## Required evidence for parent BETA-011 (re-confirmed in this packet)

- **Dry-run publish**: `./scripts/publish-tag-dryrun.sh` re-run — PASS (`docs/reports/beta-011-b-publish-tag-dry-run.json`); all 9 packages `private: true`; `beta`/`next` tags, never `latest`.
- **Release workflow logs**: BETA-011-D rehearsal — `release packet: commit 370bb8b`, checksums all `OK` (`docs/reports/beta-011-d-github-style-release-packet.md`).
- **Rollback rehearsal**: `./scripts/yank-rollback-rehearsal.sh` re-run — PASS (`docs/reports/beta-011-e-yank-rollback-rehearsal.json`).
- **SemVer prerelease policy**: `./scripts/semver-prerelease-check.sh` re-run — PASS (`docs/reports/beta-011-a-semver-prerelease.json`); `0.1.0-beta.1` is valid SemVer 2.0.0 prerelease per `docs/beta/governance/RELEASE_AUTHORITY.md`.

All rehearsal scripts produce deterministic output; re-running them in this packet left the working tree unchanged.

## Targeted command results

- `cargo test -p q-pack` — pass (100 + 2 + 0 doc)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Publication to npm/GitHub Releases remains Owner-gated; all publishing evidence is dry-run/simulation by design.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
