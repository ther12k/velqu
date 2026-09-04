# Installing Velqu (public beta)

Velqu ships two runtime deployment modes plus a container image. Both
modes serve the same verified application pack; pick one:

| mode | artifacts | swap app without touching runtime |
|---|---|---|
| shared (`velqu-runtime`) | runtime binary + `app.qpack` | yes |
| standalone (`velqu-standalone`) | one binary with the pack embedded | no — rebuild the binary |

Beta scope reminder: the supported platform is Linux x86_64 glibc.
Velqu is a public beta — non-SLA, no production-readiness claim, and the
API may change between beta releases (see `docs/beta/01_BETA_DEFINITION.md`).

## Prerequisites (build from source)

Beta distribution is source-based; the `@velqu/*` npm packages are
prepared but not yet published (all are marked `private`).

- Linux x86_64 with glibc (beta target).
- Rust stable (`cargo`) and Bun 1.4 (build/dev tooling only — production
  execution is the Rust binary and its embedded quickjs-ng 0.15.1 engine
  via rquickjs 0.12.2).
- A trusted build: authenticity checks live in your deployment pipeline,
  not in the pack (ADR-0026).

## Step 1 — build the runtime and the pack

```bash
bun install --frozen-lockfile
cargo build --release -p velqu-runtime
bun packages/cli/src/index.ts build --project examples/proof
# → examples/proof/dist/app.qpack  (deterministic bytes)
```

## Step 2 — run shared mode

```bash
./target/release/velqu-runtime --pack examples/proof/dist/app.qpack --port 8080
curl -sf http://127.0.0.1:8080/health/live   # → {"status":"ok"}
curl -sf http://127.0.0.1:8080/hello/beta    # → {"message":"Hello beta"}
```

The process exits non-zero with a structured diagnostic if anything
fails before ready — including a pack built for a different runtime
build ("engine mismatch … SEC-001 exact match"). A pack only runs on
the exact runtime build it was compiled against; upgrading the runtime
means rebuilding/re-shipping the pack.

By default the runtime binds `127.0.0.1` and expects a reverse proxy in
front (`proxyMode: "reverse-proxy"`); a public bind requires switching
to `proxyMode: "direct"` explicitly. Configuration files must declare
`"configVersion": 1`.

## Alternative — standalone single-file deployment

`velqu-standalone` embeds the pack into the binary at build time:

```bash
VELQU_STANDALONE_PACK="$(realpath examples/proof/dist/app.qpack)" \
  cargo build --release -p velqu-runtime --features standalone
./target/release/velqu-standalone --port 8080
curl -sf http://127.0.0.1:8080/health/live
```

The embedded pack is verified at startup exactly like shared mode; the
ready line reports `"mode":"standalone"`. To ship new app code you
rebuild the binary with the new pack.

## Alternative — container image

```bash
docker build -t velqu-runtime .
docker run --rm -d -p 127.0.0.1:8080:3000 \
  -e VELQU_HOST=0.0.0.0 -e VELQU_PROXY_MODE=direct \
  -v "$PWD/examples/proof/dist/app.qpack:/app/app.qpack:ro" \
  velqu-runtime
curl -sf http://127.0.0.1:8080/health/live   # → {"status":"ok"}
```

Inside the container the runtime binds `0.0.0.0` with `proxyMode: "direct"`
(bound-loopback is unreachable through Docker port publishing); the loopback
boundary moves to the host-side publish `127.0.0.1:8080:3000`, so the service
is still only reachable from the container host — public TLS belongs at an
edge reverse proxy. `docker-compose.beta.yml` shows the full posture
(non-root UID 10001, HEALTHCHECK, `stop_grace_period` for graceful drain);
`scripts/container-smoke.sh` rehearses the runtime contract end to end.

## Updating (shared mode)

| change | action |
|---|---|
| app code/routes | rebuild + replace `app.qpack`, restart |
| runtime upgrade | rebuild both artifacts together; fingerprint must match |

## Limits and accuracy notes

Defaults (config-overridable): body 1 MiB, header 32 KiB, URI 8 KiB,
queue 256 concurrent, heap 32 MiB, stack 512 KiB, handler deadline 5 s,
pending ops 1024.

`app.qpack` embeds QuickJS bytecode. Bytecode improves startup and
enables strict verification, but it is not native-machine-code JIT
compilation. No universal performance claim is made; measured,
fixture-specific numbers live under `benchmarks/` with raw samples.
