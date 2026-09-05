# BETA-015-Z — Package Evidence for Generate Beta Release Evidence, SBOM, and Checksums

## Overview

Evidence packaging and handoff for parent task **BETA-015** ("Generate beta
release evidence, SBOM, and checksums"). All ten child packets (A–I, V) were
delivered, merged, and verified; this packet binds them into one evidence
record, re-runs the self-verifying release-packet rehearsal at the packet
commit, and flips the parent ledger entry to PASS.

## BETA-015 packet inventory (all merged)

| Packet | Deliverable | PR | Report |
|---|---|---|---|
| BETA-015-A | Source ZIP (`git archive`, one bound commit) | #1203 | `docs/reports/beta-015-a-source-zip.md` |
| BETA-015-B | Git bundle (full history, `git bundle verify`) | #1204 | `docs/reports/beta-015-b-git-bundle.md` |
| BETA-015-C | Linux x86_64 glibc `velqu-runtime` binary (fail-closed copy) | #1205 | `docs/reports/beta-015-c-linux-binaries.md` |
| BETA-015-D | npm tarballs for all 9 `@velqu/*` packages + checksums | #1206 | `docs/reports/beta-015-d-npm-package-tarballs.md` |
| BETA-015-E | `velqu-bytecode` QPack tool (fail-closed copy) + tool inventory | #1207 | `docs/reports/beta-015-e-qpack-tools.md` |
| BETA-015-F | SBOM (`scripts/sbom.sh`, CycloneDX 1.5, commit-bound) | #1208 | `docs/reports/beta-015-f-sbom.md` |
| BETA-015-G | Single top-level `SHA256SUMS.txt` + verify step in `scripts/release-packet` | #1209 | `docs/reports/beta-015-g-checksums.md` |
| BETA-015-H | `REVIEW_INDEX.json` / `EVIDENCE_INDEX.json` refreshed to beta era | #1210 | `docs/reports/beta-015-h-review-evidence-indexes.md` |
| BETA-015-I | `docs/beta/KNOWN-LIMITATIONS.md` (18 limitations) shipped in packet | #1211 | `docs/reports/beta-015-i-known-limitations.md` |
| BETA-015-V | Parent verification closure (all guardrails re-confirmed) | #1212 | `docs/reports/beta-015-v-verify-release-evidence.md` |

`scripts/release-packet` is the single entry point that composes all of the
above into one self-verifying directory and fails closed if any artifact is
missing.

## Parent acceptance guardrails — final evidence mapping

| Guardrail | Evidence | Result |
|---|---|---|
| Checksums verify from release directory | `./scripts/release-packet` rehearsal at this packet commit: one top-level manifest, `sha256sum -c SHA256SUMS.txt` all OK (`CHECKSUMS-OK` line in transcript below) | PASS |
| Artifacts map to one source commit | `SOURCE-COMMIT.txt` is the clean packet HEAD; bundle HEAD equals it; ZIP generated from it; SBOM `velqu:source-commit` property equals it; binary digest equals the manifest-pinned `qRuntimeRelease` | PASS |
| SBOM identifies dependencies/licenses | `sbom.cdx.json`: CycloneDX 1.5, 277 components, 277/277 license coverage, zero external crates without a license field; owner-gated licenses recorded honestly (`UNLICENSED-BEFORE-OWNER-DECISION` / `NOASSERTION`) | PASS |
| No stale historical metadata is current | `REVIEW_INDEX.json` / `EVIDENCE_INDEX.json` regenerated at packet time and commit-bound; `KNOWN-LIMITATIONS.md` reflects the current beta state | PASS |

## Release-packet rehearsal (this worktree, packet commit)

Command: `./scripts/release-packet` from the clean packet tree (transcript
retained in the PR body of this packet). Outcome:

- Packet composed: source ZIP, git bundle, `velqu-runtime` + `velqu-bytecode`
  binaries, `BENCHMARK_MANIFEST` snapshot, `REVIEW_INDEX.json`,
  `EVIDENCE_INDEX.json`, `CHANGELOG.md`, `KNOWN-LIMITATIONS.md`,
  `sbom.cdx.json`, 9 npm tarballs.
- `sha256sum -c SHA256SUMS.txt` from inside the release directory: all files OK.
- `CHECKSUMS-OK`: every packet file covered by the single top-level manifest.

## Verification transcript (targeted commands, this worktree)

- `cargo test -p q-pack` — pass
- `cargo test -p q-http` — pass
- `cargo test -p q-schema-runtime` — pass
- `bun test` (in `unshare -rn` netns) — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS

## Artifact inventory

- Packet generator: `scripts/release-packet` (composes `scripts/sbom.sh`,
  `scripts/npm-package-tarballs.sh`, index rewrite, checksum manifest).
- Packet contents: `SOURCE-COMMIT.txt`, `velqu-<short>.bundle`,
  `source-<short>.zip`, `velqu-runtime`, `velqu-bytecode`,
  `BENCHMARK_MANIFEST`, `REVIEW_INDEX.json`, `EVIDENCE_INDEX.json`,
  `CHANGELOG.md`, `KNOWN-LIMITATIONS.md`, `sbom.cdx.json`, `npm-tarballs/`
  (9 tarballs), `SHA256SUMS.txt`.
- Status bindings updated in this packet: task record
  `docs/codex-spark-beta/tasks/08_public_beta/BETA-015-Z-…md` (TODO → PASS +
  Result), `docs/codex-spark-beta/STATUS.md` checkbox,
  `docs/codex-spark-beta/indexes/TASK_INDEX.md` row, parent ledger
  `docs/beta/04_TASK_LEDGER.md` (`BETA-015` TODO → PASS).

## Disclosures

- Evidence packaging only; no runtime behavior modified.
- Publication of the packet (GitHub Release, npm, license fields) remains
  Owner-gated; the packet is a local, verifiable artifact set.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps
  at PR creation since roughly #714; local gates/evidence are the acceptance
  basis.
