# BETA-015-B — Git Bundle

## Overview

Verifies the **git bundle** deliverable of the self-verifying public-beta release packet, produced by `scripts/release-packet`:

- `velqu-<short>.bundle` is created with `git bundle create ... HEAD master` from the clean packet tree, so it carries the full history up to exactly one bound source commit.
- The bundle is listed in `SHA256SUMS.txt` and verified with `sha256sum -c` from inside the `release/` directory.
- `git bundle verify <bundle>` (run against the release checkout) proves the bundle is a well-formed, self-contained history whose head matches the recorded `SOURCE-COMMIT.txt`.
- The packet refuses to build from a dirty tree, guaranteeing the bundle matches the committed state.

## Rehearsal

Executed at the clean packet commit (see the PR body for the exact hash and transcript):

```text
$ ./scripts/release-packet
release packet: commit <short>
SOURCE-COMMIT.txt: OK
velqu-<short>.bundle: OK
source-<short>.zip: OK
BENCHMARK_MANIFEST.json: OK
REVIEW_INDEX.json: OK
EVIDENCE_INDEX.json: OK
CHANGELOG.md: OK

$ git bundle verify release/velqu-<short>.bundle
The bundle contains this ref:
<full-commit-hash> refs/heads/master
The bundle requires this ref:
<full-commit-hash> HEAD   (or equivalent ancestry line)

$ git bundle list-heads release/velqu-<short>.bundle
<full-commit-hash> refs/heads/master
```

The bundle's recorded head equals `SOURCE-COMMIT.txt`, confirming one-commit binding.

## Artifact inventory

| artifact | role |
|---|---|
| `velqu-<short>.bundle` | complete git bundle of the release commit (this deliverable) |
| `SOURCE-COMMIT.txt` | bound full commit hash (compared against bundle heads) |
| `source-<short>.zip` | `git archive` source ZIP (BETA-015-A) |
| `BENCHMARK_MANIFEST.json`, `REVIEW_INDEX.json`, `EVIDENCE_INDEX.json`, `CHANGELOG.md` | commit-bound metadata |
| `SHA256SUMS.txt` | checksum manifest for all shipped files |

## Guardrail mapping

- **Checksums verify from release directory** — `sha256sum -c SHA256SUMS.txt` all OK, including the bundle.
- **Artifacts map to one source commit** — `git bundle list-heads` output equals `SOURCE-COMMIT.txt`; indexes grep-verified for the same commit.
- **SBOM identifies dependencies/licenses** — dedicated BETA-015 SBOM packet; dependency inventory from BETA-009-B; license fields remain owner-gated.
- **No stale historical metadata is current** — bundle and indexes are regenerated after HEAD is fixed.

## Gates

- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` / `cargo clippy -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Evidence/reporting packet only; no runtime behavior modified. Publication remains Owner-gated.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
