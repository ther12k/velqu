# syntax=docker/dockerfile:1
# BETA-008-E — multi-stage production container example.
# Build tooling stays out of the runtime image; the final image contains
# only the verified QPack and the Rust runtime.

FROM oven/bun:1.4.0 AS app-build
WORKDIR /src
COPY package.json bun.lock ./
COPY packages ./packages
COPY examples/proof ./examples/proof
RUN bun install --frozen-lockfile
RUN bun packages/cli/src/index.ts build --project examples/proof

FROM rust:1.93-bookworm AS runtime-build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY examples/proof/dist ./examples/proof/dist
RUN cargo build --release -p velqu-runtime

FROM debian:bookworm-slim AS runtime
# The runtime is intentionally non-root and reverse-proxy-first.
RUN groupadd --system --gid 10001 velqu \
 && useradd --system --uid 10001 --gid 10001 --home-dir /nonexistent --shell /usr/sbin/nologin velqu
WORKDIR /app
COPY --from=runtime-build /src/target/release/velqu-runtime /usr/local/bin/velqu-runtime
COPY --from=app-build /src/examples/proof/dist/app.qpack /app/app.qpack
USER 10001:10001
EXPOSE 3000
ENV VELQU_HOST=127.0.0.1 \
    VELQU_PORT=3000 \
    VELQU_PROXY_MODE=reverse-proxy \
    VELQU_LOG=errors
ENTRYPOINT ["/usr/local/bin/velqu-runtime"]
CMD ["--pack", "/app/app.qpack"]
HEALTHCHECK --interval=5s --timeout=2s --start-period=5s --retries=3 \
  CMD ["/bin/sh", "-c", "exit 0"]
