# BETA-015-G — Checksums

## Overview

Implements the unified **checksums** deliverable of the beta release packet. `scripts/release-packet` now composes the complete release directory and produces **one top-level `SHA256SUMS.txt`** covering every shipped file:

- Base packet artifacts: `SOURCE-COMMIT.txt`, git bundle, source ZIP, `velqu-runtime`, `velqu-bytecode`, `BENCHMARK_MANIFEST.json`, `REVIEW_INDEX.json`, `EVIDENCE_INDEX.json`, `CHANGELOG.md`.
- BETA-015-F SBOM: `sbom.cdx.json` (generated into the packet, commit-bound).
- BETA-015-D npm tarballs: `npm-tarballs/*.tgz` (9 packages, packed into the packet).

The manifest lists every file except `SHA256SUMS.txt` files themselves (the nested `npm-tarballs/SHA256SUMS.txt` still verifies independently inside its directory), is sorted deterministically (`LC_ALL=C sort`), and is verified with `sha256sum -c` from inside `release/`.

## Rehearsal (clean packet commit; transcript in the PR body)

```text
$ ./scripts/release-packet
release packet: commit <short>
<every file>: OK            (sha256sum -c output for all covered files)
CHECKSUMS-OK: 19 files covered
```

19 covered files = 9 packet artifacts + sbom.cdx.json + 9 npm tarballs (SHA256SUMS.txt files themselves are excluded from the manifest; the nested npm-tarballs/SHA256SUMS.txt verifies independently inside its own directory).

## Guardrail mapping

- **Checksums verify from release directory** — one `sha256sum -c SHA256SUMS.txt` verifies the entire packet (binaries, bundle, ZIP, SBOM, indexes, changelog, tarballs).
- **Artifacts map to one source commit** — everything is generated inside the clean-tree packet build bound to `SOURCE-COMMIT.txt`; the SBOM carries `velqu:source-commit` with the same hash.
- **SBOM identifies dependencies/licenses** — `sbom.cdx.json` shipped and checksummed (BETA-015-F).
- **No stale historical metadata is current** — the packet is rebuilt from scratch (`rm -rf release/`) on every run.

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

- The packet requires the release binaries and bun toolchain present (fail-closed); actual publication remains Owner-gated.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
