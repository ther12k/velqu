# BETA-016-A — fresh external Linux environment for the beta journey.
#
# Reproduces the documented beta prerequisites (docs/beta/INSTALL.md,
# docs/beta/QUICKSTART.md) inside a clean Debian stable container:
# Linux x86_64 glibc, Bun 1.4.0 (dev/package/test tooling), Rust 1.96.0
# (repository lockfile toolchain, minimal profile) with a C toolchain
# for the quickjs-ng build. No Velqu sources, caches, or pre-built
# artifacts are present — later verification packets install everything
# from the release packet as an external user would.
FROM debian:bookworm-slim@sha256:88200866dfff7ea7f5cbcb6ec7c8a701889efe6fe859fe64d6990e4b07ea4171

ARG RUST_TOOLCHAIN=1.96.0
ARG BUN_VERSION=1.4.0

ENV DEBIAN_FRONTEND=noninteractive \
    RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    BUN_INSTALL=/opt/bun

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates curl unzip git xz-utils build-essential pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Bun (dev/package/test tooling only — production execution is the Rust binary).
RUN curl -fsSL https://bun.sh/install | bash -s "bun-v${BUN_VERSION}" \
    && ln -s "${BUN_INSTALL}/bin/bun" /usr/local/bin/bun \
    && ln -s "${BUN_INSTALL}/bin/bunx" /usr/local/bin/bunx

# Rust from the repository lockfile toolchain, minimal profile.
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y \
       --profile minimal --default-toolchain "${RUST_TOOLCHAIN}" --no-modify-path \
    && ln -s "${CARGO_HOME}/bin/cargo" /usr/local/bin/cargo \
    && ln -s "${CARGO_HOME}/bin/rustc" /usr/local/bin/rustc \
    && ln -s "${CARGO_HOME}/bin/rustup" /usr/local/bin/rustup

# The verification packets act as an external user: unprivileged account,
# empty home, no Velqu material anywhere on the filesystem.
RUN useradd --create-home --uid 1000 beta
USER beta
WORKDIR /home/beta

COPY --chown=beta:beta manifest.sh /usr/local/bin/velqu-env-manifest
RUN chmod +x /usr/local/bin/velqu-env-manifest

CMD ["/usr/local/bin/velqu-env-manifest"]
