# syntax=docker/dockerfile:1
# benchmarks/multihost — pinned cross-host benchmark environment.
# Descriptive multi-host comparison lane (local / Halotec / Oracle, 1-2 CPU
# containers). The builder base is the SAME digest-pinned ubuntu:24.04 used
# by the ga-native-repro lane, with the same apt pins for the C toolchain,
# so same-arch hosts build byte-identical velqu binaries (M6-006 recipe).
#
# Hosts run:  benchmarks/multihost/run.sh <host-label> [cpus]
# The container runs the fixture checker (fail-closed) for every candidate,
# then the warm harness across ALL candidates with a fixed seed.

FROM ubuntu:24.04@sha256:224a1869083a311ef3f13648a154ba79832fbef6364d31493642ca03082da254
ENV BENCH_BASE_DIGEST=ubuntu:24.04@sha256:224a1869083a311ef3f13648a154ba79832fbef6364d31493642ca03082da254

ARG BUN_VERSION=1.4.0
ARG RUSTC_VERSION=1.96.0
ARG GCC13_PIN=13.3.0-6ubuntu2~24.04.1
ARG BINUTILS_PIN=2.42-4ubuntu2.10
ARG LIBC6_DEV_PIN=2.39-0ubuntu8.9
# 0 = cargo default (all cores); set lower on busy/shared hosts
ARG CARGO_BUILD_JOBS=0
ENV BUN_VERSION=${BUN_VERSION} RUSTC_VERSION=${RUSTC_VERSION}

RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates curl unzip git \
      "gcc-13=${GCC13_PIN}" "binutils=${BINUTILS_PIN}" "libc6-dev=${LIBC6_DEV_PIN}" \
 && apt-get install -y --no-install-recommends gcc \
 && rm -rf /var/lib/apt/lists/*

# Bun (pinned); arch from the build host itself (same-host build/run — works
# on both real docker and podman/buildah, no BuildKit TARGETARCH dependency)
RUN ARCH=$(uname -m | grep -q aarch64 && echo arm64 || echo x64) \
 && curl -fsSL "https://github.com/oven-sh/bun/releases/download/bun-v${BUN_VERSION}/bun-linux-${ARCH}.zip" -o /tmp/bun.zip \
 && unzip -q /tmp/bun.zip -d /tmp/buninstall \
 && mv "/tmp/buninstall/bun-linux-${ARCH}/bun" /usr/local/bin/bun \
 && rm -rf /tmp/bun.zip /tmp/buninstall \
 && bun --version

# Rust (pinned), minimal profile
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --default-toolchain "${RUSTC_VERSION}" --profile minimal --no-modify-path
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /bench
COPY . .

# candidate dependencies (frozen)
RUN cd baselines/elysia2 && bun install --frozen-lockfile \
 && cd ../lugas && bun install --frozen-lockfile \
 && cd /bench && bun install --frozen-lockfile

# proof QPack + release binaries with the canonical reproducibility remaps
RUN export RUSTFLAGS="--remap-path-prefix=/bench=/velqu-src" \
 && export CFLAGS="-ffile-prefix-map=/bench=/velqu-src -fdebug-prefix-map=/bench=/velqu-src" \
 && [ "$CARGO_BUILD_JOBS" = 0 ] || export CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" \
 && bun packages/cli/src/index.ts build --project examples/proof \
 && cargo build --release -p velqu-runtime -p q-bytecode-tool \
 && cd baselines/raw-rust && cargo build --release

ENV WARM_SEED=20260915 WARM_DURATION=10 WARM_REPETITIONS=5 WARM_CONCURRENCY=1,10,50
CMD ["bash", "/bench/benchmarks/multihost/container-run.sh"]
