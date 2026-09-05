# BETA-016-A — Fresh Linux VM/container

## Overview

Provisions the fresh external Linux environment for the BETA-016
clean-install and tutorial verification: a reproducible Debian stable
x86_64 glibc container containing **only** the documented beta
prerequisites (`docs/beta/QUICKSTART.md` — Bun 1.4.0; repository
lockfile Rust 1.96.0 minimal profile; C toolchain for the quickjs-ng
build) and no Velqu material whatsoever. Later packets (B–F) act as the
external user inside this environment, installing from the release
packet.

## Deliverable

- `scripts/beta-external/Dockerfile` — base image pinned by digest
  `debian:bookworm-slim@sha256:88200866dfff…ea4171`; unprivileged
  `beta` user (UID 1000); Bun 1.4.0 from the official installer; rustup
  with `--profile minimal --default-toolchain 1.96.0` (the repository
  `rust-toolchain.toml` channel); build-essential/pkg-config for the
  runtime build.
- `scripts/beta-external/manifest.sh` — fail-closed environment probe
  (arch, glibc OS, unprivileged user, tool versions, active rustup
  toolchain, freshness: no Velqu material).
- `scripts/beta-external/build-env.sh` — builds the image
  (`velqu-beta-external:0.1.0-beta.1`), prints the manifest, re-runs
  the probe, exits non-zero on any failure.

## External transcript (retained verbatim)

- Host: Linux 7.0.0-30-generic x86_64, docker 29.5.0; source commit
  `6ff2e0c5a68fdd8e8269b067dabbefa7566697b0` (clean tree).
- Image: `velqu-beta-external:0.1.0-beta.1`, image digest
  `sha256:a3df266bf73ee5485b9d0176d07c19015dad98c8a2ded0e7a58c7a6c00b3da58`.
- Base image resolved by pinned digest; build log retained in the
  packet transcript (`/tmp/z16a-transcript.log` at build time,
  regenerable via `scripts/beta-external/build-env.sh`).

## Environment manifest (probe output)

```
os=Debian GNU/Linux 12 (bookworm)
arch=x86_64
kernel=7.0.0-30-generic
user=beta
bun=1.4.0
cargo=cargo 1.96.0 (30a34c682  2026-05-25)
rustc=rustc 1.96.0 (ac68faa20  2026-05-25)
gcc=gcc (Debian 12.2.0-14+deb12u1) 12.2.0
git=git version 2.39.5
bun_install_dir=/opt/bun
rustup_toolchain=1.96.0-x86_64-unknown-linux-gnu (default)
fresh=no-velqu-material
MANIFEST-OK
```

## Issues and resolutions

1. `rustup_toolchain` probe initially printed `unset` — the probe ran
   `rustup show` without the rustup binary on PATH (only cargo/rustc
   were symlinked). Resolution: symlink `rustup` into `/usr/local/bin`
   in the Dockerfile; probe now reports the active pinned toolchain.
2. The manifest probe's bun check (`echo "bun=$(bun --version)" || fail`)
   could not fail closed because a command-substitution failure does not
   propagate to `echo`'s status. Resolution: explicit
   `command -v bun >/dev/null || fail "bun missing"` guard.
3. Both fixes were verified by rebuilding the image and re-running the
   probe (`MANIFEST-OK`, `ENV-BUILD-OK`).

## Gates (this worktree)

- `cargo test -p velqu-runtime` — pass
- `bun test` — 434 pass / 0 fail (67 files, in `unshare -rn` netns)
- `bun run typecheck` — pass

## Disclosures

- Environment provisioning only; no Velqu code paths changed.
- The container pulls Bun from bun.sh and Rust from sh.rustup.rs at
  image-build time; the pinned facts are the base-image digest, the
  bun version argument, and the rustup toolchain argument. A fully
  offline rebuild is a post-beta concern (Owner-gated distribution).
- Standing CI disclosure applies (zero-step verify workflows since
  ~#714); local gates are the acceptance basis.
