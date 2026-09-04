# BETA-011-D — Create GitHub-Style Release Packet

## Overview

Extended and rehearsed the self-verifying GitHub-style release packet produced by `scripts/release-packet`:

- The packet binds exactly one source commit: `SOURCE-COMMIT.txt` plus a full `git bundle` and a source ZIP, both prefixed with the short commit.
- Packet contents (all hash-listed in `SHA256SUMS.txt`, verified from inside `release/` via `sha256sum -c`):
  - `SOURCE-COMMIT.txt` — the bound source commit;
  - `velqu-<short>.bundle` — complete Git bundle of the release commit;
  - `source-<short>.zip` — source archive with commit-prefixed prefix path;
  - `BENCHMARK_MANIFEST.json` — canonical benchmark manifest copy;
  - `REVIEW_INDEX.json` / `EVIDENCE_INDEX.json` — generated after HEAD is fixed and grep-verified to carry the exact `commit`/`releaseCommit` binding;
  - `CHANGELOG.md` — **new in this packet**: the beta changelog and migration notes (`docs/beta/CHANGELOG.md`) ship inside the release packet so consumers and the GitHub release body source the same breaking-change notes.

## Script changes (`scripts/release-packet`)

- Copies `docs/beta/CHANGELOG.md` into the packet when present (conditional, so the packet builder still works for historical commits without a changelog).
- Includes `CHANGELOG.md` in `SHA256SUMS.txt` when present, keeping the packet self-verifying.

## Rehearsal

- `./scripts/release-packet` executed at the clean packet commit: `release packet: commit <short>` followed by `sha256sum -c SHA256SUMS.txt` reporting `OK` for every shipped file including `CHANGELOG.md`.
- Index binding check: generated indexes grep-verified against the exact source commit.
- The packet requires a clean working tree and never mutates tracked files; re-running it rebuilds `release/` from scratch without touching the repository version.

## Guardrail mapping

- "Version is consistent across packages/binary/QPack" — packet carries the benchmark manifest and evidence/review indexes bound to one commit.
- "Re-running release does not mutate existing version" — rehearsal repeated with identical input commit produced byte-identical index structure and passed checksums; no tracked file mutated.
- "Breaking beta changes require notes" — `CHANGELOG.md` (BETA-011-C) is now part of the shipped packet.

## Gates

- `cargo test -p q-pack` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/release-packet` — packet built and SHA256SUMS verified

## Disclosures

- Publishing to GitHub Releases remains an Owner-authorized action; this packet prepares the artifact set only.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
