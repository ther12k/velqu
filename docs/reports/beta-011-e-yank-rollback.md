# BETA-011-E — Support Yanking/Rollback

## Overview

Rehearsed and validated the release yank/rollback procedure required by the parent guardrail "Rollback procedure is tested", dry-run only (no registry or GitHub release is touched — publication itself is Owner-gated):

- Script: `scripts/yank-rollback-rehearsal.sh`, verdict `PASS`, machine-readable artifact `docs/reports/beta-011-e-yank-rollback-rehearsal.json`.
- Implements exactly the withdrawal governance of `docs/beta/governance/RELEASE_AUTHORITY.md`: the Owner may stop publication, withdraw a release, or request a package yank when **release evidence is incomplete**, **checksums do not match**, **a security issue requires withdrawal**, or **a release violates stated beta limits**.

## Rehearsed rollback actions

1. `npm dist-tag rm @velqu/cli beta --dry-run` — remove the prerelease tag.
2. `npm dist-tag add @velqu/cli@0.1.0-alpha.0 beta --dry-run` — repoint the channel at the last stable prerelease.
3. `npm yank @velqu/cli@0.1.0-beta.1 --dry-run` — yank the affected version.
4. GitHub release withdrawal (`gh release unpublish`/delete) with a withdrawal note.
5. Packet-level withdrawal record schema: `{ withdrawnVersion, reason (one of the four triggers), recordedAt, evidenceRewritten: false }`.

## Invariants verified

- Every rehearsal action is dry-run; no network/registry mutation.
- Withdrawal **appends a record and never rewrites historical evidence** (`evidenceRewritten: false` enforced by the recorded schema invariant).
- Rollback target differs from the withdrawn version (stable channel separation).
- Authority document and packet builder presence checked as part of the script.

## Guardrail mapping

- "Rollback procedure is tested" — the procedure above is scripted, repeatable, and fails non-zero if any invariant check regresses.

## Gates

- `cargo test -p q-pack` — pass
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)
- `./scripts/yank-rollback-rehearsal.sh` — PASS

## Disclosures

- Dry-run only; actual yank/withdrawal requires an authorized published release and an Owner decision.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
