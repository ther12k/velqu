# BETA-015-C — Linux Binaries

## Overview

Implements and verifies the **Linux binaries** deliverable of the self-verifying public-beta release packet. `scripts/release-packet` now ships the Linux x86_64 release binary:

- The packet copies `target/release/velqu-runtime` (built from this exact tree with the reproducibility path remap `RUSTFLAGS="--remap-path-prefix=$(pwd)=/velqu-src"`) into the release directory as `velqu-runtime`.
- A missing binary **fails closed** — the packet cannot be built without the release binary, so a packet always carries the Linux x86_64 glibc artifact for its source commit.
- The binary is listed in `SHA256SUMS.txt` and verified with `sha256sum -c` from inside `release/`.

## Rehearsal

Executed at the clean packet commit (transcript in the PR body):

```text
$ ./scripts/release-packet
release packet: commit <short>
SOURCE-COMMIT.txt: OK
velqu-<short>.bundle: OK
source-<short>.zip: OK
velqu-runtime: OK            <-- Linux x86_64 release binary
BENCHMARK_MANIFEST.json: OK
REVIEW_INDEX.json: OK
EVIDENCE_INDEX.json: OK
CHANGELOG.md: OK

$ ./release/velqu-runtime --fingerprint   # binary identity from the packet
$ file release/velqu-runtime              # ELF 64-bit x86-64, dynamically linked (glibc)
```

The shipped binary matches `benchmarks/manifest.json`'s `qRuntimeRelease` SHA-256 (the same artifact tracked by the benchmark evidence validator), tying the packet binary to the commit-bound evidence chain.

## Artifact inventory (updated)

| artifact | role |
|---|---|
| `velqu-runtime` | Linux x86_64 glibc release binary (this deliverable; fail-closed requirement) |
| `SOURCE-COMMIT.txt` / `velqu-<short>.bundle` / `source-<short>.zip` | one-commit binding + full history + source archive |
| `BENCHMARK_MANIFEST.json`, `REVIEW_INDEX.json`, `EVIDENCE_INDEX.json`, `CHANGELOG.md` | commit-bound metadata |
| `SHA256SUMS.txt` | checksum manifest for all shipped files (now 8 entries) |

## Guardrail mapping

- **Checksums verify from release directory** — `sha256sum -c SHA256SUMS.txt` all OK including `velqu-runtime`.
- **Artifacts map to one source commit** — the binary is required to be built from this tree; its hash equals the manifest-pinned `qRuntimeRelease` digest refreshed by this packet's gates.
- **SBOM identifies dependencies/licenses** — dedicated BETA-015 SBOM packet; binary's dynamic dependencies (glibc) recorded in the transcript; license fields remain owner-gated.
- **No stale historical metadata is current** — packet rebuilt from scratch each run; binary and manifest refreshed by this packet's gates.

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

- The beta platform promise is Linux x86_64 glibc only (BETA-010-A); no other-platform binaries are claimed.
- ARM64 build evidence remains conditional (BETA-010-B) and is not shipped in the packet.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
