# Trusted Proxy Runbook (BETA-008-A)

This runbook is the operational boundary for a Velqu beta deployment behind a
reverse proxy. It is intentionally narrow: public TLS terminates at the proxy;
the Velqu runtime serves plain HTTP on a private listener.

## Required runtime posture

Use the default `proxyMode: "reverse-proxy"` (or pass
`--proxy-mode reverse-proxy`). This mode accepts only loopback binds:
`127.0.0.1`, `::1`, `[::1]`, or `localhost`. A public bind such as
`0.0.0.0` rejects before ready with `startup.rejected` at `config.resolve`.
The guard is a deployment safety check; it does not DNS-resolve operator
names or claim to provide a hostile-code sandbox.

```bash
velqu-runtime \
  --pack examples/proof/dist/app.qpack \
  --proxy-mode reverse-proxy \
  --host 127.0.0.1 --port 3000
```

`proxyMode: "direct"` / `--proxy-mode direct` is an explicit operator opt-in.
Use it only when the operator owns the direct boundary's TLS/access-control,
public exposure, and header consequences. The runtime still treats forwarded
headers as ordinary data; they are never identity or authorization input.

## Proxy boundary checklist

1. Bind the runtime to loopback and firewall the runtime port from external
   clients.
2. Terminate public TLS at the proxy; do not load certificates/private keys in
   Velqu.
3. Set explicit proxy request-size, connect, send, read, and idle timeouts.
4. Forward only the public paths intended for this application.
5. Do not use `X-Forwarded-For`, `X-Forwarded-Proto`, `X-Forwarded-Host`,
   `X-Forwarded-Port`, `X-Forwarded-All`, or RFC 7239 `Forwarded` as runtime
   identity/authentication/authorization inputs. A client that reaches the
   runtime directly can spoof them; the runtime does not trust them.
6. Use signed application-layer tokens when an identity must cross the proxy
   boundary (out of runtime scope).

The runtime routes by method + path only; `Host` never selects a route or
virtual host. The connection peer, not a forwarded header, is the only peer
identity the host can observe.

## Rollout and shutdown

1. Start the new runtime on a private port.
2. Wait for `GET /health/ready` to return 200 and inspect the JSON ready line:
   it must show `config.proxyMode: "reverse-proxy"` and the expected
   `config.proxyModeSource`.
3. Add the private upstream to the proxy and exercise one typed route.
4. Remove the old upstream from proxy admission.
5. Send SIGTERM to the old runtime and wait for its bounded drain report.
6. If the drain budget expires, follow the runtime's forced-abort report; do
   not leave an old public upstream enabled.

## Failure diagnosis

- `PublicBindInReverseProxy` / `reverse-proxy mode requires a loopback bind`:
  fix the bind to loopback, or explicitly choose `proxyMode: "direct"` after
  accepting the operator-owned boundary consequences.
- `unknown proxy mode`:
  use only `reverse-proxy` or `direct`; typos are rejected, never defaulted.
- A route sees a forwarded header:
  it is request data only. Do not treat it as authenticated client identity.
- Runtime is reachable from outside the proxy:
  close the firewall/interface exposure; reverse-proxy mode's bind guard is a
  defense-in-depth check, not a replacement for network policy.

## Container smoke

From the repository root after building the release runtime and proof pack:

```bash
scripts/proxy-smoke.sh
```

The smoke keeps the service private, checks liveness plus a typed route, checks
the ready-line proxy posture, and sends SIGTERM. It ends with
`PROXY-SMOKE-OK` only after deterministic process exit.
