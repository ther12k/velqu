# BETA-010-A — Linux x86_64 glibc Mandatory Working Assumption

## Platform decision

Public beta support is intentionally exact: Linux `x86_64` with a glibc-based
userland, using the release artifact/runtime combinations covered by the beta
evidence process. This packet does not expand support to ARM64, musl, macOS,
Windows, static-libc, or other architectures.

The canonical policy is `docs/beta/governance/PLATFORM_SUPPORT.md`; this report
packages the local platform transcript, release binary inspection, and clean
install/build smoke.

## Install/build transcript

Captured on the acceptance host:

```text
uname: Linux kiranitrov15 7.0.0-30-generic #30~24.04.1-Ubuntu SMP PREEMPT_DYNAMIC Fri Aug  7 13:27:52 UTC 2 2026 x86_64 x86_64 x86_64 GNU/Linux
libc: Ubuntu GLIBC 2.39-0ubuntu8.8 2.39
rustc: 1.96.0 (ac68faa20 2026-05-25)
host: x86_64-unknown-linux-gnu
bun: 1.4.0
```

Commands:

```bash
bun install --frozen-lockfile
cargo build -p q-bytecode-tool
bun packages/cli/src/index.ts build --project examples/proof
RUSTFLAGS="--remap-path-prefix=$(pwd)=/velqu-src" cargo build --release -p velqu-runtime
scripts/proxy-smoke.sh
scripts/container-smoke.sh
```

Result: `PROXY-SMOKE-OK` and `CONTAINER-SMOKE-OK`. The release runtime was
identified as an x86_64 Linux ELF and linked to the host glibc; no source or
compiler artifact is required at runtime beyond the verified QPack.

## Artifact boundary

- Final runtime deployment artifact: `target/release/velqu-runtime` plus
  `examples/proof/dist/app.qpack`.
- The container example keeps Bun/Rust tooling in build stages and only copies
  the release runtime/QPack into the final non-root image.
- `scripts/package` excludes `target` build caches and `node_modules`; package
  inventory must distinguish source archive from platform runtime artifact.

## Unsupported-platform guidance

ARM64 is conditional/CI-only until a separate platform packet and owner
acceptance exist. macOS is development-only best effort; Windows, musl/static
libc, and other OS/architectures have no public beta promise. Do not describe
CI portability signals as published support or infer cloud/scale-to-zero/SLA
behavior from this local transcript.

## Companion evidence

- `docs/beta/governance/PLATFORM_SUPPORT.md`
- `docs/beta/LIMITS-AND-NON-GOALS.md`
- `Dockerfile`, `docker-compose.beta.yml`, `.dockerignore`
- `scripts/container-smoke.sh`, `scripts/proxy-smoke.sh`
- `docs/reports/beta-008-e-container-example.md`
- `.github/workflows/verify.yml` (matrix portability signals; not a support
  expansion)

## Gates

- `cargo test -p q-pack` — pass
- `cargo test -p q-engine-quickjs` — pass
- artifact build + proof pack build — pass
- proxy/container contract smokes — pass
