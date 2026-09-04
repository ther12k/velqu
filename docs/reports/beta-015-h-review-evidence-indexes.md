# BETA-015-H — Review/Evidence Indexes

## Overview

Implements the **review/evidence indexes** deliverable of the beta release packet. Both root index templates (`REVIEW_INDEX.json`, `EVIDENCE_INDEX.json`) were stale M4A-era artifacts; this packet refreshes them to the beta release state:

### REVIEW_INDEX.json (velqu-review-index-v1)
- `milestone`: `BETA-PUBLIC-BETA-RELEASE`.
- Gates: (1) BETA-001..014 parent closure with ledger + Z-report evidence and PR range; (2) BETA-015 self-verifying packet (release-packet rehearsals, bundle verify, SBOM, tool inventory).
- Open items carried honestly: PACK_FORMAT_CURRENT v1 pin, M3-009 owner target, npm publication Owner-gated, license Owner-gated, standing CI disclosure.
- `verification`: verify ALL PASS locally, 434 TS tests.
- Commit/generation fields remain `BOUND_BY_RELEASE_PACKET_*` placeholders — rewritten by `scripts/release-packet` after the clean candidate HEAD is fixed and grep-verified to carry that commit.

### EVIDENCE_INDEX.json (velqu-evidence-index-v1)
- Same milestone/binding posture.
- Benchmarks inventory extended with the ramp crossover/losses artifacts (BETA-003/BETA-014 evidence).
- Reports inventory refreshed to the beta evidence set (BETA-013 soak, BETA-014 benchmark, BETA-015 A–G packet reports, M3-010 soak/chaos/memory/recovery).
- `release` block documents the unified checksum manifest, SBOM script, and npm tarballs composition (BETA-015-D/E/F/G).
- Open items mirror the review index.

## Rehearsal

Executed `./scripts/release-packet` at the clean packet commit: both packet-local indexes were regenerated with the bound commit and grep-verified (`"commit": "<COMMIT>"` present in both) before checksumming — the binding rule works end to end with the refreshed templates.

## Guardrail mapping

- **Checksums verify from release directory** — the indexes are checksummed inside the packet; packet rehearsal all OK.
- **Artifacts map to one source commit** — the binding rule rewrites `commit`/`releaseCommit`/`generatedAt` after HEAD is fixed and the packet grep-verifies them.
- **No stale historical metadata is current** — this deliverable: M4A-era metadata replaced with beta-era content; placeholder fields are machine-bound at packet time, never hand-typed.

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

- Index templates intentionally carry placeholder commit fields; actual values are bound only inside a packet built from a clean tree.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
