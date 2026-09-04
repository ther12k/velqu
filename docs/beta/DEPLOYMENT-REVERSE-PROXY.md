# Deployment behind a reverse proxy

The beta deployment profile is **reverse-proxy first**. Terminate public
TLS at a trusted edge proxy and keep the Velqu runtime on plain HTTP, bound
to loopback. The runtime defaults to `proxyMode: "reverse-proxy"`; this
mode rejects public binds before ready. `proxyMode: "direct"` is an explicit
operator opt-in for a boundary whose TLS, access control, forwarding-header
and exposure consequences the operator owns. This is a deployment posture,
not a production readiness or availability guarantee.

## Build and run the runtime privately

```bash
bun install --frozen-lockfile
bun packages/cli/src/index.ts build --project examples/proof
cargo build --release -p velqu-runtime
./target/release/velqu-runtime \
  --pack examples/proof/dist/app.qpack \
  --proxy-mode reverse-proxy --host 127.0.0.1 --port 3000
```

The runtime owns routing, readiness, request limits, deadlines, and graceful
shutdown. It does not load certificates or private keys in this beta.

## Example Nginx boundary

The following minimal server terminates TLS at Nginx and forwards only to a
loopback runtime. Replace the certificate paths and hostname for your
controlled environment:

```nginx
server {
    listen 443 ssl;
    server_name api.example.test;

    ssl_certificate     /etc/letsencrypt/live/api.example.test/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.example.test/privkey.pem;

    client_max_body_size 1m;
    proxy_connect_timeout 2s;
    proxy_read_timeout 30s;
    proxy_send_timeout 30s;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-Proto https;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    location = /health/live {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
    }

    location = /health/ready {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
    }
}
```

The proxy must be the only public path to the runtime. Do not expose a plain
HTTP listener bound to `0.0.0.0` as a secure public deployment in the default
`reverse-proxy` mode. Forwarded headers are ordinary request data, never
identity or authorization input (ADR-0034); the runtime does not infer client
identity, scheme, host, or port from them. If the operator explicitly selects
`proxyMode: "direct"`, the operator owns the direct-boundary TLS/access-control
and forwarding-header consequences — the runtime still does not trust those
headers.

The sample's proxy semantics (loopback `proxy_pass`, forwarded headers as
data, native health endpoints reachable through the boundary) were rehearsed
end-to-end with a non-TLS derivation of this block (`listen 8080`, backend on
a private port) before release; the TLS directives themselves are standard
nginx configuration and require a real certificate environment to exercise.

## Forwarded header policy (BETA-008-B)

The runtime treats `X-Forwarded-For`, `X-Forwarded-Proto`,
`X-Forwarded-Host`, `X-Forwarded-Port`, `X-Forwarded-All`, RFC 7239
`Forwarded`, and `Host` as ordinary request data only. They are never used for
client identity, authentication, authorization, scheme reconstruction, or
route selection. A forged forwarding claim therefore cannot authenticate a
request or redirect it to another route. If an application needs identity to
cross the proxy, use a signed application-layer token; proxy header trust is
out of scope.

The runtime records the TCP connection peer from `TcpListener::accept`; it does
not replace that peer with a header claim. Forwarded headers can still be
read when the route explicitly declares them, because readability as data is
not trust. The peer is an internal host value, not an application `ctx` field;
this keeps identity and authorization decisions out of the JS request ABI.

## Health, readiness, and drain

Use `/health/live` for process/listener liveness and `/health/ready` for
traffic admission. Both endpoints accept GET and HEAD, return JSON, and are
served natively before route handler JavaScript. `/health/live` stays 200 while
the process/listener is alive; `/health/ready` returns 200 only while the
engine is healthy and returns 503 with the stable `engine quarantined` problem
when the engine is unavailable. A deployment controller should remove a worker
from the proxy upstream before sending its shutdown signal, then allow
in-flight requests to drain within the configured budget. A readiness failure
is not a license to retry requests blindly; preserve the client's idempotency
and retry policy at the edge.

A safe rollout sequence is:

1. Start the new runtime on a private port and wait for `/health/ready`.
2. Add it to the proxy upstream and verify a real typed route.
3. Stop admitting traffic to the old runtime; readiness is withdrawn before
   the shutdown signal reaches the process.
4. Send SIGTERM and wait for its bounded shutdown report. The runtime flips
   its lock-free drain gate immediately, refuses new dynamic admissions with
   503 + `Retry-After: 1`, and lets in-flight work finish.
5. If the 5-second drain budget expires, in-flight stragglers are force-aborted
   through ownership and reported as `aborted`; the runtime still exits
   deterministically with no pending invocation.
6. Remove the old process only after the drain completes or the deployment
   timeout expires.

## Boundaries and limitations

- The proxy owns public TLS, certificate rotation, edge access logs, request
  size limits, connection limits, and external health checks.
- The runtime owns application routing, schema validation, bounded body/queue
  work, readiness, and shutdown.
- Native runtime TLS, HTTP/2 termination, certificate handling inside Velqu,
  and direct public exposure are not supported promises in this beta.
- Same-process QuickJS executes trusted application code only; it is not a
  hostile-code sandbox.
- `defer` is in-memory best-effort work, never a durable job queue.

## Verify locally

From the repository root:

```bash
bun install --frozen-lockfile
bun packages/cli/src/index.ts build --project examples/proof
cargo test -p velqu-runtime
bun run typecheck
bun run verify
```

The commands verify the runtime and proof artifacts. The Nginx block is a
configuration example; test it with your installed proxy and deployment
configuration before use. This guide does not claim production readiness.
