# BETA-015-E — QPack Tools

## Overview

Implements the **QPack tools** deliverable of the beta release evidence:

- `scripts/release-packet` now also ships the QPack tooling binary `velqu-bytecode` (q-bytecode-tool: bytecode `embed` and pack migration), fail-closed when not built from this tree, with checksum coverage in `SHA256SUMS.txt` (9 entries).
- The full QPack tool surface is verified by the existing `scripts/qpack-tools-inventory.sh` (BETA-010-D): runtime fingerprint (verdict `compatible`), bytecode embed, CLI `pack inspect` (status `ok`), CLI `pack migrate`, and standalone execution.

## Rehearsal (clean packet commit; transcript in the PR body)

```text
$ ./scripts/release-packet
release packet: commit <short>
SOURCE-COMMIT.txt: OK
velqu-<short>.bundle: OK
source-<short>.zip: OK
velqu-runtime: OK
velqu-bytecode: OK          <-- QPack tooling binary (this deliverable)
BENCHMARK_MANIFEST.json: OK
REVIEW_INDEX.json: OK
EVIDENCE_INDEX.json: OK
CHANGELOG.md: OK

$ ./scripts/qpack-tools-inventory.sh /tmp/b015e-inventory.json
... verifies fingerprint verdict=compatible, bytecode embed, pack inspect ok,
pack migrate ok, standalone mode ...
QPACK-TOOLS-OK (verdict from generated JSON)
```

## QPack tool inventory

| tool | role | verified by |
|---|---|---|
| `velqu-runtime` | runtime binary; `--fingerprint --pack` verifies pack↔runtime identity (SEC-001 exact match) | qpack-tools-inventory #1 |
| `velqu-bytecode` | bytecode embedding (`embed --pack --out`) and pack migration tooling | qpack-tools-inventory #2; shipped in packet (new) |
| `velqu pack inspect --json` | pack structure/routes inspection without executing handlers | qpack-tools-inventory #3 |
| `velqu pack migrate --json` | pack migration between supported representations | qpack-tools-inventory #4 |
| `velqu-standalone` | single-file runtime with embedded pack (identical serving answers) | qpack-tools-inventory #5 |

## Guardrail mapping

- **Checksums verify from release directory** — `sha256sum -c SHA256SUMS.txt` all OK including `velqu-bytecode`.
- **Artifacts map to one source commit** — both binaries are required to be built from this tree and are checksummed next to `SOURCE-COMMIT.txt`.
- **SBOM identifies dependencies/licenses** — dedicated BETA-015 SBOM packet; dependency inventory from BETA-009-B.
- **No stale historical metadata is current** — packet and inventory rebuilt from scratch each run.

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

- `velqu-standalone` is validated by the inventory script but is pack-specific at build time (embedded pack), so it is not shipped as a generic packet artifact.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
