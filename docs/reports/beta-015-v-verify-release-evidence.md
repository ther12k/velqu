# BETA-015-V — Verify Generate Beta Release Evidence, SBOM, and Checksums

## Overview

Verification closure for parent task **BETA-015** ("Generate beta release evidence, SBOM, and checksums"). Every parent acceptance criterion was mapped to its implementation packet, source, and re-confirmed evidence.

## Acceptance Criteria Matrix

| Parent guardrail | Implementation | Verification evidence | Result |
|---|---|---|---|
| **Checksums verify from release directory** | BETA-015-G | `./scripts/release-packet` at the clean packet commit: single top-level `sha256sum -c SHA256SUMS.txt` verifies the whole packet (20 files: binaries, bundle, ZIP, SBOM, indexes, changelog, known-limitations, 9 npm tarballs) — `CHECKSUMS-OK` | PASS |
| **Artifacts map to one source commit** | BETA-015-A/B/C/E/H | `SOURCE-COMMIT.txt` + `git bundle verify` (bundle HEAD equals the recorded commit) + `git archive` ZIP + grep-verified index bindings + binary digest equal to the manifest-pinned `qRuntimeRelease`; clean-tree requirement blocks drift | PASS |
| **SBOM identifies dependencies/licenses** | BETA-015-F | `scripts/sbom.sh` → CycloneDX 1.5, 277 components, 277/277 license coverage, zero external crates missing licenses, commit-bound; honest posture for owner-gated licenses | PASS |
| **No stale historical metadata is current** | BETA-015-H/I | Both packet indexes refreshed to beta-era content and machine-bound at packet time; known-limitations inventory reflects the current state including all carried open items | PASS |

## Full-packet rehearsal summary (this worktree)

- Source ZIP (BETA-015-A): `git archive` from one bound commit — OK.
- Git bundle (BETA-015-B): `git bundle verify` complete history, HEAD == SOURCE-COMMIT — OK.
- Linux binaries (BETA-015-C): `velqu-runtime` fail-closed, digest == manifest pin — OK.
- npm tarballs (BETA-015-D): 9/9 `@velqu/*` packed + checksummed — OK.
- QPack tools (BETA-015-E): `velqu-bytecode` shipped; full tool inventory verdict PASS — OK.
- SBOM (BETA-015-F): 277 components, license coverage 277/277 — PASS.
- Unified checksums (BETA-015-G): one manifest, 20 files, all OK.
- Indexes (BETA-015-H): regenerated, commit-bound, grep-verified.
- Known limitations (BETA-015-I): shipped in the packet.

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

- Verification closure only; no runtime behavior modified. Publication remains Owner-gated.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
