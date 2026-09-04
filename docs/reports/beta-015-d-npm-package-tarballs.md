# BETA-015-D — npm Package Tarballs

## Overview

Implements the **npm package tarballs** deliverable of the beta release evidence. New script `scripts/npm-package-tarballs.sh`:

- Packs every `@velqu/*` workspace package with `bun pm pack` into `release/npm-tarballs/`.
- Writes and verifies `SHA256SUMS.txt` from inside that directory (`sha256sum -c`, all OK).
- Fails closed if any package fails to pack or if no workspace packages are found.

The packages remain `"private": true` (BETA-010-C): these are shippable tarballs plus checksums only — publication to the registry remains Owner-gated per the BETA-011 posture.

## Rehearsal (this worktree, transcript in the PR body)

```text
$ ./scripts/npm-package-tarballs.sh
pack: @velqu/capability-auth-jwt … pack: @velqu/treaty
npm tarballs: 9 packages -> release/npm-tarballs
velqu-capability-auth-jwt-0.1.0.tgz: OK
velqu-capability-postgres-0.1.0.tgz: OK
velqu-cli-0.1.0.tgz: OK
velqu-compiler-0.1.0.tgz: OK
velqu-contract-0.1.0.tgz: OK
velqu-core-0.1.0.tgz: OK
velqu-schema-0.1.0.tgz: OK
velqu-testing-0.1.0.tgz: OK
velqu-treaty-0.1.0.tgz: OK
NPM-TARBALLS-OK
```

## Artifact inventory (release/npm-tarballs/)

| tarball | bytes | sha256 (first 12) |
|---|---:|---|
| velqu-capability-auth-jwt-0.1.0.tgz | 17,319 | 90942c158636 |
| velqu-capability-postgres-0.1.0.tgz | 5,350 | f3b8230b03b8 |
| velqu-cli-0.1.0.tgz | 36,182 | 81882d7e92cf |
| velqu-compiler-0.1.0.tgz | 38,549 | eb129dd70f7b |
| velqu-contract-0.1.0.tgz | 990 | e206f5966b5e |
| velqu-core-0.1.0.tgz | 3,832 | ed7fc9880d31 |
| velqu-schema-0.1.0.tgz | 5,058 | 1f7e1aa25cc7 |
| velqu-testing-0.1.0.tgz | 8,157 | bf27b7c7c00e |
| velqu-treaty-0.1.0.tgz | 9,384 | 3b1ff0265d40 |

All nine carry version `0.1.0` (uniform with the workspace; the authorized prerelease label `0.1.0-beta.1` is applied at publication time under Owner authority per `docs/beta/governance/RELEASE_AUTHORITY.md`, not silently here). Tarball contents carry the package sources (`package/package.json`, `package/src/*`).

## Guardrail mapping

- **Checksums verify from release directory** — `sha256sum -c SHA256SUMS.txt` all OK inside `release/npm-tarballs/`.
- **Artifacts map to one source commit** — tarballs are packed from the committed working tree; run alongside `scripts/release-packet` from the same clean commit to bind them to `SOURCE-COMMIT.txt`.
- **SBOM identifies dependencies/licenses** — dependency inventory from BETA-009-B scan; license fields remain owner-gated (dedicated BETA-015 SBOM packet).
- **No stale historical metadata is current** — directory is rebuilt from scratch each run.

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

- Tarballs are generated evidence only; registry publication is Owner-gated (dry-run publish and tag policy: BETA-011-B).
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
