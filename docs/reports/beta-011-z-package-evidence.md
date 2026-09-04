# BETA-011-Z — Package Evidence: Automate Beta Publishing and Versioning

## Overview

Evidence closure for parent task BETA-011 ("Automate beta publishing and versioning"), flipping the parent row in `docs/beta/04_TASK_LEDGER.md` from TODO to PASS. All five child packets (A–E) plus the V verification packet are PASS with source-backed, re-runnable evidence.

## Packet inventory (parent BETA-011)

| Packet | Deliverable | Canonical evidence | PR |
|---|---|---|---|
| BETA-011-A | SemVer prerelease policy | `scripts/semver-prerelease-check.sh`; `docs/reports/beta-011-a-semver-prerelease{.md,.json}`; `docs/beta/governance/RELEASE_AUTHORITY.md` (0.1.0-beta.1) | #1167 |
| BETA-011-B | `next`/`beta` dist-tag policy | `scripts/publish-tag-dryrun.sh`; `docs/reports/beta-011-b-publish-tag-dry-run.json`; `docs/reports/beta-011-b-publish-next-beta-tag.md` | #1168 |
| BETA-011-C | Changelog + migration notes | `docs/beta/CHANGELOG.md`; `docs/reports/beta-011-c-changelog-migration-notes.md` | #1169 |
| BETA-011-D | GitHub-style release packet | `scripts/release-packet` (ships CHANGELOG.md, SHA256SUMS-verified); `docs/reports/beta-011-d-github-style-release-packet.md` | #1170 |
| BETA-011-E | Yank/rollback rehearsal | `scripts/yank-rollback-rehearsal.sh`; `docs/reports/beta-011-e-yank-rollback-rehearsal.json`; `docs/reports/beta-011-e-yank-rollback.md` | #1171 |
| BETA-011-V | Verification closure | `docs/reports/beta-011-v-verify-beta-publishing-versioning.md` (acceptance matrix) | #1172 |

## Acceptance guardrails → evidence

1. **Version is consistent across packages/binary/QPack** — workspace `Cargo.toml` `version = "0.1.0"`; all 9 `@velqu/*` package.json `"version": "0.1.0"`; q-pack compiler identity string `"0.1.0"`. Verified by fresh inspection in BETA-011-V.
2. **Re-running release does not mutate existing version** — `scripts/release-packet` rehearsal at commit `370bb8b`: clean-tree requirement enforced, repeat run rebuilt `release/` with `sha256sum -c SHA256SUMS.txt` all `OK` (7 artifacts) and zero tracked-file mutation.
3. **Rollback procedure is tested** — `scripts/yank-rollback-rehearsal.sh` verdict PASS: four Owner withdrawal triggers, dist-tag repoint, package yank, GitHub withdrawal, and the `evidenceRewritten: false` record invariant; script exits non-zero on regression.
4. **Breaking beta changes require notes** — `docs/beta/CHANGELOG.md` documents 5 breaking changes with migration guidance and ships inside the release packet, hash-listed in `SHA256SUMS.txt`.

## Re-confirmation in this packet

All three rehearsal scripts re-run in this worktree immediately before the gate battery: `semver-prerelease-check.sh`, `publish-tag-dryrun.sh`, `yank-rollback-rehearsal.sh` — all PASS with deterministic output (working tree unchanged by the re-runs).

## Gate battery (this packet)

- `cargo test -p velqu-runtime` — pass
- `cargo test -p q-pack` — pass (100+2, from BETA-011-V and every child packet)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- npm/GitHub publication remains Owner-gated; all publishing evidence is dry-run/simulation by design (`private: true` guards active on all 9 packages).
- Ledger flip BETA-011 → PASS in `docs/beta/04_TASK_LEDGER.md` accompanies this report.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
