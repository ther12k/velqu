---
type: Operations Document
title: Rebuilding Velqu Release Artifacts from Source
status: active
date: 2026-09-15
implements: M6-005 (provenance + documented rebuild process)
---

# Rebuild Process (M6-005)

This document describes how to rebuild the release artifacts from a source
commit, which inputs affect output bytes, and which checks the rebuild must
pass. It is the "documented rebuild process" deliverable of M6-005.

**What this document does not claim**: that two independent builders produce
byte-identical *native binaries*. That property is M6-006 (reproducible
builds on independent builders) and is established by separate evidence.
What IS byte-reproducible today, proven every verify run, is the compiled
application artifact set (QPack): `scripts/compare-builds` builds
`examples/proof` twice through fully independent CLI processes from
different working directories and requires byte-identical outputs
(M26-007-D). The release packet checksums whatever artifacts ship with it.

## 1. Toolchain (normative pins)

| Component | Pin | Where |
| --- | --- | --- |
| Rust | 1.96.0 | `rust-toolchain.toml` (rustup honors it automatically) |
| Bun | 1.4.0 | CI setup (`oven-sh/setup-bun@v2`, `bun-version: 1.4.0`) |
| Engine | quickjs-ng 0.15.1 via rquickjs =0.12.2 | `Cargo.lock` (AGENTS.md constraint 1) |
| C compiler | gcc-13 13.3.0 (`13.3.0-6ubuntu2~24.04.1`) | ga-native-repro lane (apt version pin inside a digest-pinned `ubuntu:24.04` builder container) |
| Linker/binutils | GNU ld 2.42 (`binutils 2.42-4ubuntu2.10`) | ga-native-repro lane (same) |
| Lockfiles | `Cargo.lock`, `bun.lock` committed; installs frozen | CI + scripts |

Install: `rustup toolchain install 1.96.0`, `rustup default 1.96.0`, Bun
1.4.0 per vendor instructions.

The C toolchain is a build input, not an afterthought: rquickjs-sys
compiles the vendored QuickJS C sources with `cc`, and different `cc`
versions produce different binary bytes. This is demonstrated, not
hypothetical — the M6-006 capture host (user-space gcc 12.2.0) produces a
different `velqu-runtime` hash than the CI builders (gcc 13.3.0) from the
same source commit and the same rustc. Rebuilds that must match the
independent-builder evidence therefore use the pinned native environment
of the ga-native-repro lane
(`.github/workflows/ga-native-repro.yml`: digest-pinned `ubuntu:24.04`
builder container, gcc-13 and binutils installed at the exact apt versions
recorded in every run's `hashes-*.txt`).

## 2. Rebuild steps (exact)

```bash
# 0. clean checkout at the source commit recorded in the provenance statement
git checkout <commit>
git status --porcelain        # MUST be empty

# 1. dependencies (frozen — never floating)
bun install --frozen-lockfile

# 2. release binaries with the reproducibility path remap
#    (identical to what scripts/verify exports; see §3 parameter table)
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$(pwd)=/velqu-src"
export CFLAGS="${CFLAGS:-} -ffile-prefix-map=$(pwd)=/velqu-src -fdebug-prefix-map=$(pwd)=/velqu-src"
cargo build --release -p velqu-runtime
cargo build --release -p q-bytecode-tool

# 3. application pack (byte-reproducible; this is the compare-builds path)
bun packages/cli/src/index.ts build --project examples/proof

# 4. full verification (fmt, clippy -D warnings, workspace tests, release
#    builds, typecheck, proof build, conformance, OKF, benchmark evidence)
./scripts/verify

# 5. release packet: source archive, git bundle, binaries, SBOM, npm
#    tarballs, provenance, one checksum manifest over everything
./scripts/release-packet            # add --sign / --key for M8-003 signing

# 6. verify the packet and the provenance statement
(cd release && sha256sum -c SHA256SUMS.txt)
python3 scripts/generate-provenance.py --verify release/provenance.json
```

## 3. Parameters that affect output bytes

| Parameter | Value | Why |
| --- | --- | --- |
| `RUSTFLAGS` | `--remap-path-prefix=<repo>=/velqu-src` | rquickjs-sys embeds absolute `OUT_DIR` paths in QuickJS objects; without the remap, binaries differ per checkout path |
| `CFLAGS` | `-ffile-prefix-map=<repo>=/velqu-src -fdebug-prefix-map=<repo>=/velqu-src` | same class of path leakage for C sources |
| profile | `--release` (AGENTS.md: release Rust builds use `--release` only) | |
| lockfiles | frozen (`--frozen-lockfile`, committed locks) | floating resolution would change inputs silently |
| engine/binding | exact (Cargo.lock) | constraint 1; bumps are dedicated packets per ADR-0043 §3 |

The ambient `RUSTFLAGS`/`CFLAGS` of the generating invocation are recorded
in `provenance.json → parameters`.

## 4. Disclosed non-reproducible inputs

Recorded verbatim in every provenance statement
(`scripts/generate-provenance.py`):

- the statement's own `generatedAt` timestamp (metadata about the build,
  not a build output);
- `REVIEW_INDEX.json` / `EVIDENCE_INDEX.json` `generatedAt` fields inside
  the release packet (timestamped metadata, disclosed per file);
- git bundle / source zip creation embed the creating process's timestamps
  and ordering;
- absolute paths are rewritten by the §3 remaps; residue would be caught
  by `scripts/compare-builds` byte-identity on the compiled artifact set.

## 5. Provenance statement

`scripts/generate-provenance.py` (called by `scripts/release-packet`, so
the statement lands inside the checksummed — and, with `--sign`, signed —
packet) emits `release/provenance.json`:

- **source**: 40-hex commit, clean-tree assertion, packet binding;
- **builder**: platform string only; explicitly no hostname/user, and a
  `builderType` that disclaims isolation claims;
- **toolchain**: exact `rustc`/`cargo`/`bun`/`git`/`gpg` version strings,
  the pinned toolchain file verbatim, and the lock-resolved engine/binding
  versions;
- **parameters**: the ambient remap flags and profile;
- **artifacts**: SHA-256 of every file in the packet;
- **claimsNotMade**: SLSA level, hermeticity, independent-builder
  reproducibility (M6-006), signing (M8-003) — listed explicitly so the
  statement cannot be over-read.

Verification: `--verify` re-hashes every listed artifact, checks the
schema, requires all toolchain fields to have been available, and checks
`source.commit` against the packet's `SOURCE-COMMIT.txt`. Any mismatch is
a hard failure.

## 6. Failure handling

- Dirty tree → both `release-packet` and `generate-provenance` refuse.
- Missing binary/stale build → `release-packet` fails closed.
- Hash mismatch on verify → rebuild from §2; if it persists with the same
  toolchain pins and remaps, that is a reproducibility defect to file
  against M6-006 (and it blocks RC per that row's acceptance).
